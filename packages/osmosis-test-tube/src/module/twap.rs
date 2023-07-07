use osmosis_std::types::osmosis::twap::v1beta1;
use test_tube::{fn_query, Module};

use test_tube::runner::Runner;

pub struct Twap<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Twap<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Twap<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    // ========== Queries ==========

    fn_query! {
        pub query_arithmetic_twap ["/osmosis.twap.v1beta1.Query/ArithmeticTwap"]: v1beta1::ArithmeticTwapRequest => v1beta1::ArithmeticTwapResponse
    }
    fn_query! {
        pub query_arithmetic_twap_to_now ["/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow"]: v1beta1::ArithmeticTwapToNowRequest => v1beta1::ArithmeticTwapToNowResponse
    }

    fn_query! {
        pub query_geometric_twap ["/osmosis.twap.v1beta1.Query/GeometricTwap"]: v1beta1::GeometricTwapRequest => v1beta1::GeometricTwapResponse
    }
    fn_query! {
        pub query_geometric_twap_to_now ["/osmosis.twap.v1beta1.Query/GeometricTwapToNow"]: v1beta1::GeometricTwapToNowRequest => v1beta1::GeometricTwapToNowResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use osmosis_std::shim::Timestamp;
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as SdkCoin,
        osmosis::{
            poolmanager::v1beta1::{MsgSwapExactAmountIn, SpotPriceRequest, SwapAmountInRoute},
            tokenfactory::v1beta1::{MsgCreateDenom, MsgMint},
            twap::v1beta1,
        },
    };
    use test_tube::Account;

    use crate::{Gamm, Module, OsmosisTestApp, PoolManager, TokenFactory, Twap};

    #[test]
    fn test_twap() {
        let app = OsmosisTestApp::new();
        let signer = app
            .init_account(&[Coin::new(1_000_000_000_000_000, "uosmo")])
            .unwrap()
            .with_fee_setting(test_tube::account::FeeSetting::Auto {
                gas_price: Coin::new(25, "uosmo"),
                gas_adjustment: 1.2,
            });

        let tokenfactory = TokenFactory::new(&app);
        let pool_manager = PoolManager::new(&app);
        let gamm = Gamm::new(&app);
        let twap = Twap::new(&app);

        // create denom
        let denom0 = tokenfactory
            .create_denom(
                MsgCreateDenom {
                    sender: signer.address(),
                    subdenom: "denomzero".to_string(),
                },
                &signer,
            )
            .unwrap()
            .data
            .new_token_denom;

        let denom1 = tokenfactory
            .create_denom(
                MsgCreateDenom {
                    sender: signer.address(),
                    subdenom: "denomone".to_string(),
                },
                &signer,
            )
            .unwrap()
            .data
            .new_token_denom;

        // mint denom
        tokenfactory
            .mint(
                MsgMint {
                    sender: signer.address(),
                    amount: Some(Coin::new(10_000_000_000_000, &denom0).into()),
                    mint_to_address: signer.address(),
                },
                &signer,
            )
            .unwrap();

        tokenfactory
            .mint(
                MsgMint {
                    sender: signer.address(),
                    amount: Some(Coin::new(100_000_000_000, &denom1).into()),
                    mint_to_address: signer.address(),
                },
                &signer,
            )
            .unwrap();

        // create pool 1
        gamm.create_basic_pool(
            &[
                Coin::new(1_000_000_000, denom0.clone()),
                Coin::new(2_000_000_000, denom1.clone()),
            ],
            &signer,
        )
        .unwrap();

        let res = pool_manager
            .query_spot_price(&SpotPriceRequest {
                pool_id: 1,
                base_asset_denom: denom0.clone(),
                quote_asset_denom: denom1.clone(),
            })
            .unwrap();
        assert_eq!(res.spot_price, "2.000000000000000000");

        pool_manager
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: signer.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: 1,
                        token_out_denom: denom1.clone(),
                    }],
                    token_in: Some(SdkCoin {
                        amount: "1_000_000_000".to_string(),
                        denom: denom0.clone(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &signer,
            )
            .unwrap();

        let res = pool_manager
            .query_spot_price(&SpotPriceRequest {
                pool_id: 1,
                base_asset_denom: denom0.clone(),
                quote_asset_denom: denom1.clone(),
            })
            .unwrap();
        assert_eq!(res.spot_price, "0.502512560000000000");

        let timestamp = app.get_block_timestamp();

        app.increase_time(10u64);

        let res = twap
            .query_arithmetic_twap(&v1beta1::ArithmeticTwapRequest {
                pool_id: 1,
                base_asset: denom0.clone(),
                quote_asset: denom1.clone(),
                start_time: Some(Timestamp {
                    seconds: timestamp.seconds() as i64,
                    nanos: timestamp.subsec_nanos() as i32,
                }),
                end_time: None,
            })
            .unwrap();
        assert_eq!(res.arithmetic_twap, "502512560000000000");

        let res = twap
            .query_arithmetic_twap_to_now(&v1beta1::ArithmeticTwapToNowRequest {
                pool_id: 1,
                base_asset: denom0.clone(),
                quote_asset: denom1.clone(),
                start_time: Some(Timestamp {
                    seconds: timestamp.seconds() as i64,
                    nanos: timestamp.subsec_nanos() as i32,
                }),
            })
            .unwrap();
        assert_eq!(res.arithmetic_twap, "502512560000000000");

        let res = twap
            .query_geometric_twap(&v1beta1::GeometricTwapRequest {
                pool_id: 1,
                base_asset: denom0.clone(),
                quote_asset: denom1.clone(),
                start_time: Some(Timestamp {
                    seconds: timestamp.seconds() as i64,
                    nanos: timestamp.subsec_nanos() as i32,
                }),
                end_time: None,
            })
            .unwrap();
        assert_eq!(res.geometric_twap, "502512560000000000");

        let res = twap
            .query_geometric_twap_to_now(&v1beta1::GeometricTwapToNowRequest {
                pool_id: 1,
                base_asset: denom0,
                quote_asset: denom1,
                start_time: Some(Timestamp {
                    seconds: timestamp.seconds() as i64,
                    nanos: timestamp.subsec_nanos() as i32,
                }),
            })
            .unwrap();
        assert_eq!(res.geometric_twap, "502512560000000000");
    }
}
