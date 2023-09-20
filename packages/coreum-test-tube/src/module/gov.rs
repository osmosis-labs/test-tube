use coreum_wasm_sdk::types::cosmos::gov::v1beta1::{
    MsgSubmitProposal, MsgSubmitProposalResponse, MsgVote, MsgVoteResponse, QueryParamsRequest,
    QueryParamsResponse, QueryProposalRequest, QueryProposalResponse,
};

use test_tube_coreum::{fn_execute, fn_query};
use test_tube_coreum::module::Module;
use test_tube_coreum::runner::Runner;

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
}

#[cfg(test)]
mod tests {
    use crate::{runner::app::FEE_DENOM, CoreumTestApp};
    use crate::{Account, Module, Gov, RunnerError};
    use coreum_wasm_sdk::types::cosmos::gov::v1beta1::{QueryParamsRequest, MsgSubmitProposal, QueryProposalRequest};
    use coreum_wasm_sdk::{shim::Any, types::cosmos::gov::v1beta1::TextProposal};
    use cosmrs::tx::MessageExt;

    #[test]
    fn test_submit_and_query_proposal() {
        let app = CoreumTestApp::default();
        let gov = Gov::new(&app);

        let proposer = app
            .init_account(&[cosmwasm_std::Coin::new(1000000000000000000, FEE_DENOM)])
            .unwrap();

        let params = gov
            .query_params(&QueryParamsRequest {
                params_type: "deposit".to_string(),
            })
            .unwrap();

        let min_deposit = params
            .deposit_params
            .expect("deposit params must exist")
            .min_deposit;

        let submit_proposal_res = gov
            .submit_proposal(
                MsgSubmitProposal {
                    content: Some(Any {
                        type_url: TextProposal::TYPE_URL.to_string(),
                        value: TextProposal {
                            title: "Title".to_string(),
                            description: "Description".to_string(),
                        }
                        .to_bytes()
                        .map_err(|e| RunnerError::EncodeError(e.into()))
                        .unwrap(),
                    }),
                    initial_deposit: min_deposit,
                    proposer: proposer.address(),
                },
                &proposer,
            )
            .unwrap();

        assert_eq!(submit_proposal_res.data.proposal_id, 1);

        let query_proposal_res = gov
            .query_proposal(&QueryProposalRequest { proposal_id: 1 })
            .unwrap();

        assert_eq!(
            query_proposal_res.proposal.unwrap().content,
            Some(Any {
                type_url: TextProposal::TYPE_URL.to_string(),
                value: TextProposal {
                    title: "Title".to_string(),
                    description: "Description".to_string(),
                }
                .to_bytes()
                .map_err(|e| RunnerError::EncodeError(e.into()))
                .unwrap(),
            })
        );
    }
}
