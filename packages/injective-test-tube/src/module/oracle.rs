use injective_std::types::injective::oracle::v1beta1::{
    MsgRelayPriceFeedPrice, MsgRelayPriceFeedPriceResponse, QueryModuleStateRequest,
    QueryModuleStateResponse, QueryOraclePriceRequest, QueryOraclePriceResponse,
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
        pub relay_price_feed: MsgRelayPriceFeedPrice => MsgRelayPriceFeedPriceResponse
    }

    fn_query! {
        pub query_module_state ["/injective.oracle.v1beta1.Query/OracleModuleState"]: QueryModuleStateRequest => QueryModuleStateResponse
    }

    fn_query! {
        pub query_oracle_price ["/injective.oracle.v1beta1.Query/OraclePrice"]: QueryOraclePriceRequest => QueryOraclePriceResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use injective_std::{
        shim::Any,
        types::{
            cosmos::{
                bank::v1beta1::MsgSend,
                base::v1beta1::Coin as TubeCoin,
                gov::v1beta1::{MsgSubmitProposal, MsgVote, QueryProposalRequest},
            },
            injective::oracle,
            injective::oracle::v1beta1::{
                GrantPriceFeederPrivilegeProposal, MsgRelayPriceFeedPrice,
            },
        },
    };
    use prost::Message;
    use std::str::FromStr;

    use crate::{Account, Bank, Gov, InjectiveTestApp, Module, Oracle};

    #[test]
    fn oracle_integration() {
        let app = InjectiveTestApp::new();

        let gov = Gov::new(&app);
        let bank = Bank::new(&app);
        let oracle = Oracle::new(&app);

        let signer = app
            .init_account(&[
                Coin::new(100_000_000_000_000_000_000_000u128, "inj"),
                Coin::new(100_000_000_000_000_000_000u128, "usdt"),
            ])
            .unwrap();

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

        let validator = app
            .get_first_validator_signing_account("inj".to_string(), 1.2f64)
            .unwrap();

        // fund the validator account
        bank.send(
            MsgSend {
                from_address: signer.address().to_string(),
                to_address: validator.address().to_string(),
                amount: vec![TubeCoin {
                    amount: "1000000000000000000000".to_string(),
                    denom: "inj".to_string(),
                }],
            },
            &signer,
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
                    proposer: validator.address().to_string(),
                },
                &validator,
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
            .unwrap()
            .proposal
            .unwrap();

        gov.vote(
            MsgVote {
                proposal_id: u64::from_str(&proposal_id).unwrap(),
                voter: validator.address().to_string(),
                option: 1i32,
            },
            &validator,
        )
        .unwrap();

        // NOTE: increase the block time in order to move past the voting period
        app.increase_time(10u64);

        let proposal = gov
            .query_proposal(&QueryProposalRequest {
                proposal_id: u64::from_str(&proposal_id).unwrap(),
            })
            .unwrap()
            .proposal
            .unwrap();
        let expected_price = "120000".to_string();

        oracle
            .relay_price_feed(
                MsgRelayPriceFeedPrice {
                    sender: signer.address().to_string(),
                    base: vec!["inj".to_string()],
                    quote: vec!["usdt".to_string()],
                    price: vec![expected_price.clone()],
                },
                &signer,
            )
            .unwrap();

        let price = oracle
            .query_oracle_price(&oracle::v1beta1::QueryOraclePriceRequest {
                oracle_type: 2i32,
                base: "inj".to_string(),
                quote: "usdt".to_string(),
            })
            .unwrap()
            .price_pair_state
            .unwrap()
            .pair_price;

        assert_eq!(price, expected_price);
    }
}
