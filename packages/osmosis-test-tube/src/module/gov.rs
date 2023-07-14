use cosmrs::tx::MessageExt;
use osmosis_std::shim::Any;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::cosmos::gov::v1beta1::{
    MsgSubmitProposal, MsgSubmitProposalResponse, MsgVote, MsgVoteResponse, QueryParamsRequest,
    QueryParamsResponse, QueryProposalRequest, QueryProposalResponse, VoteOption,
};
use test_tube::{fn_execute, fn_query, Account, RunnerError, RunnerExecuteResult, SigningAccount};

use test_tube::module::Module;
use test_tube::runner::Runner;

use crate::OsmosisTestApp;

pub struct Gov<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Gov<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Gov<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub submit_proposal: MsgSubmitProposal["/cosmos.gov.v1beta1.MsgSubmitProposal"] => MsgSubmitProposalResponse
    }

    fn_execute! {
        pub vote: MsgVote["/cosmos.gov.v1beta1.MsgVote"] => MsgVoteResponse
    }

    fn_query! {
        pub query_proposal ["/cosmos.gov.v1beta1.Query/Proposal"]: QueryProposalRequest => QueryProposalResponse
    }

    fn_query! {
        pub query_params ["/cosmos.gov.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    pub fn submit_executable_proposal<M: prost::Message>(
        &self,
        msg_type_url: String,
        msg: M,
        initial_deposit: Vec<cosmwasm_std::Coin>,
        proposer: String,
        is_expedited: bool,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgSubmitProposalResponse> {
        self.submit_proposal(
            MsgSubmitProposal {
                content: Some(Any {
                    type_url: msg_type_url,
                    value: msg
                        .to_bytes()
                        .map_err(|e| RunnerError::EncodeError(e.into()))?,
                }),
                initial_deposit: initial_deposit
                    .into_iter()
                    .map(|coin| Coin {
                        denom: coin.denom,
                        amount: coin.amount.to_string(),
                    })
                    .collect(),
                proposer,
                is_expedited,
            },
            signer,
        )
    }
}

/// Extension for Gov module
/// It has ability to access to `OsmosisTestApp` which is more specific than `Runner`
pub struct GovWithAppAccess<'a> {
    gov: Gov<'a, OsmosisTestApp>,
    app: &'a OsmosisTestApp,
}

impl<'a> GovWithAppAccess<'a> {
    pub fn new(app: &'a OsmosisTestApp) -> Self {
        Self {
            gov: Gov::new(app),
            app,
        }
    }

    pub fn to_gov(&self) -> &Gov<'a, OsmosisTestApp> {
        &self.gov
    }

    pub fn propose_and_execute<M: prost::Message>(
        &self,
        msg_type_url: String,
        msg: M,
        proposer: String,
        is_expedited: bool,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgSubmitProposalResponse> {
        // query deposit params
        let params = self.gov.query_params(&QueryParamsRequest {
            params_type: "deposit".to_string(),
        })?;

        let min_deposit = params
            .deposit_params
            .expect("deposit params must exist")
            .min_deposit;

        // submit proposal
        let submit_proposal_res = self.gov.submit_proposal(
            MsgSubmitProposal {
                content: Some(Any {
                    type_url: msg_type_url,
                    value: msg
                        .to_bytes()
                        .map_err(|e| RunnerError::EncodeError(e.into()))?,
                }),
                initial_deposit: min_deposit,
                proposer,
                is_expedited,
            },
            signer,
        )?;

        let proposal_id = submit_proposal_res.data.proposal_id;

        // get validator to vote yes for proposal
        let val = self.app.get_first_validator_signing_account()?;
        self.gov
            .vote(
                MsgVote {
                    proposal_id,
                    voter: val.address(),
                    option: VoteOption::Yes.into(),
                },
                &val,
            )
            .unwrap();

        // query params
        let params = self.gov.query_params(&QueryParamsRequest {
            params_type: "voting".to_string(),
        })?;

        // get voting period
        let voting_period = params
            .voting_params
            .expect("voting params must exist")
            .voting_period
            .expect("voting period must exist");

        // increase time to pass voting period
        self.app.increase_time(voting_period.seconds as u64 + 1);

        Ok(submit_proposal_res)
    }
}

#[cfg(test)]
mod tests {
    use osmosis_std::types::{
        cosmwasm::wasm::v1::{QueryCodeRequest, QueryCodeResponse, StoreCodeProposal},
        osmosis::cosmwasmpool::v1beta1::UploadCosmWasmPoolCodeAndWhiteListProposal,
    };
    use test_tube::Account;

    use super::*;
    use crate::OsmosisTestApp;

    #[test]
    fn test_passing_and_execute_proposal() {
        let app = OsmosisTestApp::default();
        let gov = GovWithAppAccess::new(&app);

        let proposer = app
            .init_account(&[cosmwasm_std::Coin::new(1000000000000000000, "uosmo")])
            .unwrap();

        // query code id 1 should error since it has not been stored
        let err = app
            .query::<_, QueryCodeResponse>(
                "/cosmwasm.wasm.v1.Query/Code",
                &QueryCodeRequest { code_id: 1 },
            )
            .unwrap_err();

        assert_eq!(
            err,
            RunnerError::QueryError {
                msg: "not found".to_string()
            }
        );

        // store code through proposal
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let res = gov
            .propose_and_execute(
                StoreCodeProposal::TYPE_URL.to_string(),
                StoreCodeProposal {
                    title: String::from("test"),
                    description: String::from("test"),
                    run_as: proposer.address(),
                    wasm_byte_code: wasm_byte_code.clone(),
                    instantiate_permission: None,
                    unpin_code: false,
                    source: String::new(),
                    builder: String::new(),
                    code_hash: Vec::new(),
                },
                proposer.address(),
                false,
                &proposer,
            )
            .unwrap();

        assert_eq!(res.data.proposal_id, 1);

        // query code id 1 should find the code after proposal is executed
        let QueryCodeResponse { code_info, data } = app
            .query(
                "/cosmwasm.wasm.v1.Query/Code",
                &QueryCodeRequest { code_id: 1 },
            )
            .unwrap();

        assert_eq!(code_info.unwrap().creator, proposer.address());
        assert_eq!(data, wasm_byte_code);
    }

    #[test]
    fn test_cosmwasmpool_proposal() {
        let app = OsmosisTestApp::default();
        let gov = GovWithAppAccess::new(&app);

        let proposer = app
            .init_account(&[cosmwasm_std::Coin::new(1000000000000000000, "uosmo")])
            .unwrap();

        // upload cosmwasm pool code and whitelist through proposal
        let wasm_byte_code = std::fs::read("./test_artifacts/transmuter.wasm").unwrap();
        let res = gov
            .propose_and_execute(
                UploadCosmWasmPoolCodeAndWhiteListProposal::TYPE_URL.to_string(),
                UploadCosmWasmPoolCodeAndWhiteListProposal {
                    title: String::from("test"),
                    description: String::from("test"),
                    wasm_byte_code,
                },
                proposer.address(),
                false,
                &proposer,
            )
            .unwrap();

        assert_eq!(res.data.proposal_id, 1);
    }
}
