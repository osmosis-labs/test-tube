use injective_std::types::cosmos::gov::v1beta1::MsgSubmitProposalResponse;
use injective_std::types::injective::oracle::v1beta1::{
    GrantPriceFeederPrivilegeProposal, QueryModuleStateRequest, QueryModuleStateResponse,
};
use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct Oracle<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Oracle<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Oracle<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub grant_price_feeder_proposal: GrantPriceFeederPrivilegeProposal => GrantPriceFeederPrivilegeProposal
    }

    fn_query! {
        pub query_module_state ["/injective.oracle.v1beta1.Query/OracleModuleState"]: QueryModuleStateRequest => QueryModuleStateResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use injective_std::{
        shim::Any,
        types::{
            cosmos::{
                base::v1beta1::Coin as TubeCoin,
                gov::v1beta1::{MsgSubmitProposal, MsgVote, QueryProposalRequest},
            },
            injective::oracle,
            injective::oracle::v1beta1::GrantPriceFeederPrivilegeProposal,
        },
    };
    use prost::Message;
    use std::str::FromStr;

    use crate::{Account, Gov, InjectiveTestApp, Module, Oracle};

    #[test]
    fn oracle_integration() {
        let app = InjectiveTestApp::new();
        let signer = app
            .init_account(&[
                Coin::new(10_000_000_000_000_000_000_000u128, "inj"),
                Coin::new(100_000_000_000_000_000_000u128, "usdt"),
            ])
            .unwrap();
        let gov = Gov::new(&app);

        let mut buf = vec![];
        oracle::v1beta1::GrantPriceFeederPrivilegeProposal::encode(
            &GrantPriceFeederPrivilegeProposal {
                title: "test-proposal".to_string(),
                description: "test-proposal".to_string(),
                base: "inj".to_string(),
                quote: "usdt".to_string(),
                relayers: vec![signer.address().to_string()],
            },
            &mut buf,
        )
        .unwrap();

        let res = gov
            .submit_proposal(
                MsgSubmitProposal {
                    content: Some(Any {
                        type_url: "/injective.oracle.v1beta1.GrantPriceFeederPrivilegeProposal"
                            .to_string(),
                        value: buf,
                    }),
                    initial_deposit: vec![TubeCoin {
                        amount: "100000000000000000000".to_string(),
                        denom: "inj".to_string(),
                    }],
                    proposer: signer.address().to_string(),
                    is_expedited: false,
                },
                &signer,
            )
            .unwrap();

        let proposal_id = res
            .events
            .iter()
            .find(|e| e.ty == "submit_proposal")
            .unwrap()
            .attributes[0]
            .value
            .clone();

        let proposal = gov
            .query_proposal(&QueryProposalRequest {
                proposal_id: u64::from_str(&proposal_id).unwrap(),
            })
            .unwrap();

        // println!("{:?}", proposal);

        let vote = gov
            .vote(
                MsgVote {
                    proposal_id: u64::from_str(&proposal_id).unwrap(),
                    voter: signer.address().to_string(),
                    option: 1i32,
                },
                &signer,
            )
            .unwrap();

        println!("{:?}", vote);
        println!("{:?}", app.get_first_validator_address());

        let proposal = gov
            .query_proposal(&QueryProposalRequest {
                proposal_id: u64::from_str(&proposal_id).unwrap(),
            })
            .unwrap();

        // println!("{:?}", proposal.proposal.unwrap());

        assert_eq!(1, 2);
    }
}
