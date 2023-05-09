use injective_std::types::injective::insurance::v1beta1::{
    MsgCreateInsuranceFund, MsgCreateInsuranceFundResponse, QueryInsuranceFundRequest,
    QueryInsuranceFundResponse, QueryModuleStateRequest, QueryModuleStateResponse,
};
use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct Insurance<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Insurance<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Insurance<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_insurance_fund: MsgCreateInsuranceFund => MsgCreateInsuranceFundResponse
    }

    fn_query! {
        pub query_module_state ["/injective.insurance.v1beta1.Query/InsuranceModuleState"]: QueryModuleStateRequest => QueryModuleStateResponse
    }

    fn_query! {
        pub query_insurance_fund ["/injective.insurance.v1beta1.Query/InsuranceFund"]: QueryInsuranceFundRequest => QueryInsuranceFundResponse
    }
}

#[cfg(test)]
mod tests {
    use crate::{Account, InjectiveTestApp, Insurance, Module};
    use cosmwasm_std::Coin;
    use injective_std::{
        shim::Duration,
        types::{
            cosmos::base::v1beta1::Coin as TubeCoin,
            injective::insurance::v1beta1::{
                InsuranceFund, MsgCreateInsuranceFund, QueryInsuranceFundRequest,
            },
        },
    };

    #[test]
    fn insurance_integration() {
        let app = InjectiveTestApp::new();

        let insurance = Insurance::new(&app);

        let signer = app
            .init_account(&[
                Coin::new(100_000_000_000_000_000_000_000u128, "inj"),
                Coin::new(100_000_000_000_000_000_000u128, "usdt"),
            ])
            .unwrap();

        insurance
            .create_insurance_fund(
                MsgCreateInsuranceFund {
                    sender: signer.address().to_string(),
                    ticker: "INJ/USDT".to_string(),
                    quote_denom: "usdt".to_string(),
                    oracle_base: "inj".to_string(),
                    oracle_quote: "usdt".to_string(),
                    oracle_type: 2i32,
                    expiry: -1i64,
                    initial_deposit: Some(TubeCoin {
                        amount: "100000000000000000000".to_string(),
                        denom: "usdt".to_string(),
                    }),
                },
                &signer,
            )
            .unwrap();

        let res = insurance
            .query_insurance_fund(&QueryInsuranceFundRequest {
                market_id: "0xc04ba8ebc86a97c57e4385ad264183a156c3afaffc0e4c398cc77120e2b3bab9"
                    .to_string(),
            })
            .unwrap()
            .fund
            .unwrap();

        assert_eq!(
            res,
            InsuranceFund {
                deposit_denom: "usdt".to_string(),
                balance: "100000000000000000000".to_string(),
                insurance_pool_token_denom: "share1".to_string(),
                oracle_base: "inj".to_string(),
                oracle_quote: "usdt".to_string(),
                oracle_type: 2i32,
                expiry: -1i64,
                market_id: "0xc04ba8ebc86a97c57e4385ad264183a156c3afaffc0e4c398cc77120e2b3bab9"
                    .to_string(),
                total_share: "1000000000000000000".to_string(),
                market_ticker: "INJ/USDT".to_string(),
                redemption_notice_period_duration: Some(Duration {
                    seconds: 1209600i64,
                    nanos: 0i32,
                }),
            }
        );
    }
}
