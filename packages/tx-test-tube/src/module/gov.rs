use tx_wasm_sdk::types::cosmos::gov::v1::{
    MsgSubmitProposal, MsgSubmitProposalResponse, MsgVote, MsgVoteResponse, QueryParamsRequest,
    QueryParamsResponse, QueryProposalRequest, QueryProposalResponse,
};

use test_tube_tx::module::Module;
use test_tube_tx::runner::Runner;
use test_tube_tx::{fn_execute, fn_query};

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
        pub submit_proposal: MsgSubmitProposal["/cosmos.gov.v1.MsgSubmitProposal"] => MsgSubmitProposalResponse
    }

    fn_execute! {
        pub vote: MsgVote["/cosmos.gov.v1.MsgVote"] => MsgVoteResponse
    }

    fn_query! {
        pub query_proposal ["/cosmos.gov.v1.Query/Proposal"]: QueryProposalRequest => QueryProposalResponse
    }

    fn_query! {
        pub query_params ["/cosmos.gov.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }
}

#[cfg(test)]
mod tests {
    use crate::{runner::app::FEE_DENOM, TXTestApp};
    use crate::{Account, Gov, Module};
    use tx_wasm_sdk::shim::Any;
    use tx_wasm_sdk::types::cosmos::auth::v1beta1::{
        ModuleAccount, QueryModuleAccountByNameRequest, QueryModuleAccountByNameResponse,
    };
    use tx_wasm_sdk::types::cosmos::bank::v1beta1::MsgSend;
    use tx_wasm_sdk::types::cosmos::base::v1beta1::Coin as BaseCoin;
    use tx_wasm_sdk::types::cosmos::gov::v1::{
        MsgSubmitProposal, QueryParamsRequest, QueryProposalRequest,
    };
    use cosmwasm_std::Coin;
    use test_tube_tx::Runner;

    #[test]
    fn test_submit_and_query_proposal() {
        let app = TXTestApp::default();
        let gov = Gov::new(&app);

        let proposer = app
            .init_account(&[cosmwasm_std::Coin::new(1000000000000000000u128, FEE_DENOM)])
            .unwrap();

        let receiver = app.init_account(&[Coin::new(1u128, FEE_DENOM)]).unwrap();

        let params = gov
            .query_params(&QueryParamsRequest {
                params_type: "deposit".to_string(),
            })
            .unwrap();

        let min_deposit = params
            .params
            .expect("deposit params must exist")
            .min_deposit;

        let module_account_query = QueryModuleAccountByNameRequest {
            name: "gov".to_string(),
        };
        let module_account_res = app
            .query::<QueryModuleAccountByNameRequest, QueryModuleAccountByNameResponse>(
                "/cosmos.auth.v1beta1.Query/ModuleAccountByName",
                &module_account_query,
            )
            .unwrap();

        let module_account: ModuleAccount =
            prost::Message::decode(module_account_res.account.unwrap().value.as_slice()).unwrap();

        let send_msg = MsgSend {
            from_address: module_account.base_account.unwrap().address,
            to_address: receiver.address(),
            amount: vec![BaseCoin {
                amount: "1".to_string(),
                denom: FEE_DENOM.to_string(),
            }],
        };

        let submit_proposal_res = gov
            .submit_proposal(
                MsgSubmitProposal {
                    messages: vec![Any {
                        type_url: send_msg.to_any().type_url,
                        value: send_msg.to_any().value.to_vec(),
                    }],
                    initial_deposit: min_deposit,
                    proposer: proposer.address(),
                    metadata: "".to_string(),
                    title: "proposal title".to_string(),
                    summary: "proposal summary".to_string(),
                    expedited: false,
                },
                &proposer,
            )
            .unwrap();

        assert_eq!(submit_proposal_res.data.proposal_id, 1);

        let query_proposal_res = gov
            .query_proposal(&QueryProposalRequest { proposal_id: 1 })
            .unwrap();

        assert_eq!(
            query_proposal_res.proposal.unwrap().messages.first(),
            Some(&Any {
                type_url: send_msg.to_any().type_url,
                value: send_msg.to_any().value.to_vec()
            })
        );
    }
}
