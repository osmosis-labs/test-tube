use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    AllPoolsRequest, AllPoolsResponse, EstimateSinglePoolSwapExactAmountInRequest,
    EstimateSinglePoolSwapExactAmountOutRequest, EstimateSwapExactAmountInRequest,
    EstimateSwapExactAmountInResponse, EstimateSwapExactAmountOutRequest,
    EstimateSwapExactAmountOutResponse, MsgSplitRouteSwapExactAmountIn,
    MsgSplitRouteSwapExactAmountInResponse, MsgSplitRouteSwapExactAmountOut,
    MsgSplitRouteSwapExactAmountOutResponse, MsgSwapExactAmountIn, MsgSwapExactAmountInResponse,
    MsgSwapExactAmountOut, MsgSwapExactAmountOutResponse, NumPoolsRequest, NumPoolsResponse,
    PoolRequest, PoolResponse, SpotPriceRequest, SpotPriceResponse, TotalPoolLiquidityRequest,
    TotalPoolLiquidityResponse,
};
use test_tube::{fn_execute, fn_query, Module};

use test_tube::runner::Runner;

pub struct PoolManager<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for PoolManager<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> PoolManager<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    // swap exact amount in
    fn_execute! { pub swap_exact_amount_in: MsgSwapExactAmountIn => MsgSwapExactAmountInResponse }

    // split route swap exact amount in
    fn_execute! { pub split_route_swap_exact_amount_in: MsgSplitRouteSwapExactAmountIn => MsgSplitRouteSwapExactAmountInResponse }

    // swap exact amount out
    fn_execute! { pub swap_exact_amount_out: MsgSwapExactAmountOut => MsgSwapExactAmountOutResponse }

    // split route swap exact amount out
    fn_execute! { pub split_route_swap_exact_amount_out: MsgSplitRouteSwapExactAmountOut => MsgSplitRouteSwapExactAmountOutResponse }

    // ========== Queries ==========

    // estimate swap exact amount in
    fn_query! {
        pub query_swap_exact_amount_in ["/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn"]: EstimateSwapExactAmountInRequest => EstimateSwapExactAmountInResponse
    }

    // estimate single pool swap exact amount in
    fn_query! {
        pub query_single_pool_swap_exact_amount_in ["/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountIn"]: EstimateSinglePoolSwapExactAmountInRequest => EstimateSwapExactAmountInResponse
    }

    // estimate swap exact amount out
    fn_query! {
        pub query_swap_exact_amount_out ["/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut"]: EstimateSwapExactAmountOutRequest => EstimateSwapExactAmountOutResponse
    }

    // estimate single pool swap exact amount out
    fn_query! {
        pub query_single_pool_swap_exact_amount_out ["/osmosis.poolmanager.v1beta1.Query/EstimateSinglePoolSwapExactAmountOut"]: EstimateSinglePoolSwapExactAmountOutRequest => EstimateSwapExactAmountOutResponse
    }

    // query num pools
    fn_query! {
        pub query_num_pools ["/osmosis.poolmanager.v1beta1.Query/NumPools"]: NumPoolsRequest => NumPoolsResponse
    }

    // query all pools
    fn_query! {
        pub query_all_pools ["/osmosis.poolmanager.v1beta1.Query/AllPools"]: AllPoolsRequest => AllPoolsResponse
    }

    // query pool
    fn_query! {
        pub query_pool ["/osmosis.poolmanager.v1beta1.Query/Pool"]: PoolRequest => PoolResponse
    }

    // query spot price
    fn_query! {
        pub query_spot_price ["/osmosis.poolmanager.v1beta1.Query/SpotPrice"]: SpotPriceRequest => SpotPriceResponse
    }

    // query total liquidity
    fn_query! {
        pub query_total_liquidity ["/osmosis.poolmanager.v1beta1.Query/TotalPoolLiquidity"]: TotalPoolLiquidityRequest => TotalPoolLiquidityResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Coin, Decimal, Uint128};
    use osmosis_std::types::{
        cosmos::base::v1beta1::Coin as SdkCoin,
        osmosis::{
            concentratedliquidity::v1beta1::{
                CreateConcentratedLiquidityPoolsProposal, MsgCreatePosition, PoolRecord,
            },
            poolmanager::v1beta1::{
                AllPoolsRequest, EstimateSwapExactAmountInRequest, MsgSwapExactAmountIn,
                SpotPriceRequest, SwapAmountInRoute, TotalPoolLiquidityRequest,
            },
            tokenfactory::v1beta1::{MsgCreateDenom, MsgMint},
        },
    };
    use test_tube::Account;

    use crate::{
        ConcentratedLiquidity, Gamm, GovWithAppAccess, Module, OsmosisTestApp, PoolManager,
        TokenFactory,
    };
    use std::str::FromStr;

    #[test]
    fn test_pool_manager() {
        let app = OsmosisTestApp::new();
        let signer = app
            .init_account(&[Coin::new(1_000_000_000_000_000, "uosmo")])
            .unwrap()
            .with_fee_setting(test_tube::account::FeeSetting::Auto {
                gas_price: Coin::new(25, "uosmo"),
                gas_adjustment: 1.2,
            });

        let tokenfactory = TokenFactory::new(&app);
        let concentrated_liquidity = ConcentratedLiquidity::new(&app);
        let pool_manager = PoolManager::new(&app);
        let gamm = Gamm::new(&app);
        let gov = GovWithAppAccess::new(&app);

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

        // create pool 2
        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "Create concentrated uosmo:usdc pool".to_string(),
                description: "Create concentrated uosmo:usdc pool, so that we can trade it"
                    .to_string(),
                pool_records: vec![PoolRecord {
                    denom0: denom0.clone(),
                    denom1: "uosmo".to_string(),
                    tick_spacing: 100,
                    spread_factor: "0".to_string(),
                }],
            },
            signer.address(),
            false,
            &signer,
        )
        .unwrap();

        // query all pools
        let res = pool_manager.query_all_pools(&AllPoolsRequest {}).unwrap();
        assert_eq!(res.pools.len(), 2);

        let res = pool_manager
            .query_spot_price(&SpotPriceRequest {
                pool_id: 1,
                base_asset_denom: denom0.clone(),
                quote_asset_denom: denom1,
            })
            .unwrap();

        assert_eq!(
            Decimal::from_str(&res.spot_price).unwrap(),
            Decimal::from_atomics(Uint128::from(2u128), 0).unwrap()
        );

        // provide liquidity into pool 2
        concentrated_liquidity
            .create_position(
                MsgCreatePosition {
                    pool_id: 2,
                    sender: signer.address(),
                    lower_tick: -9900i64,
                    upper_tick: 10000i64,
                    tokens_provided: vec![
                        SdkCoin {
                            amount: "1_000_000_000_000".to_string(),
                            denom: denom0.clone(),
                        },
                        SdkCoin {
                            amount: "1_000_000_000_000".to_string(),
                            //  25_753_988_892
                            denom: "uosmo".to_string(),
                        },
                    ],
                    token_min_amount0: "0".to_string(),
                    token_min_amount1: "0".to_string(),
                },
                &signer,
            )
            .unwrap();

        let res = pool_manager
            .query_spot_price(&SpotPriceRequest {
                pool_id: 2,
                base_asset_denom: denom0.clone(),
                quote_asset_denom: "uosmo".to_string(),
            })
            .unwrap();
        assert_eq!(res.spot_price, "1.000000000000000000");

        let res = pool_manager
            .query_total_liquidity(&TotalPoolLiquidityRequest { pool_id: 2 })
            .unwrap();
        assert_eq!(
            res.liquidity,
            vec![
                SdkCoin {
                    amount: "1000000000000".to_string(),
                    denom: denom0.clone(),
                },
                SdkCoin {
                    amount: "99766582669".to_string(),
                    denom: "uosmo".to_string(),
                },
            ]
        );

        // swap amount in
        let trader = app
            .init_account(&[
                Coin::new(1_000_000_000, denom0.clone()),
                Coin::new(1_000_000_000, "uosmo".to_string()),
            ])
            .unwrap();

        let res = pool_manager
            .query_swap_exact_amount_in(&EstimateSwapExactAmountInRequest {
                pool_id: 2,
                routes: vec![SwapAmountInRoute {
                    pool_id: 2,
                    token_out_denom: "uosmo".to_string(),
                }],
                token_in: format!("{}{}", "1000000", denom0),
            })
            .unwrap();
        let expected_token_out = res.token_out_amount;

        let res = pool_manager
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: trader.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: 2,
                        token_out_denom: "uosmo".to_string(),
                    }],
                    token_in: Some(SdkCoin {
                        amount: "1_000_000".to_string(),
                        denom: denom0,
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &trader,
            )
            .unwrap();
        let token_out = res.data.token_out_amount;

        assert_eq!(expected_token_out, token_out);
    }
}
