use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCollectFees, MsgCollectFeesResponse, MsgCollectIncentives, MsgCollectIncentivesResponse,
    MsgCreateConcentratedPool, MsgCreateConcentratedPoolResponse, MsgCreateIncentive,
    MsgCreateIncentiveResponse, MsgCreatePosition, MsgCreatePositionResponse, MsgWithdrawPosition,
    MsgWithdrawPositionResponse, QueryClaimableFeesRequest, QueryClaimableFeesResponse,
    QueryLiquidityDepthsForRangeRequest, QueryLiquidityDepthsForRangeResponse, QueryParamsRequest,
    QueryParamsResponse, QueryPoolsRequest, QueryPoolsResponse, QueryPositionByIdRequest,
    QueryPositionByIdResponse, QueryTotalLiquidityForRangeRequest,
    QueryTotalLiquidityForRangeResponse, QueryUserPositionsRequest, QueryUserPositionsResponse,
};
use test_tube::{fn_execute, fn_query, Module};

use test_tube::runner::Runner;

pub struct ConcentratedLiquidity<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for ConcentratedLiquidity<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> ConcentratedLiquidity<'a, R>
where
    R: Runner<'a>,
{
    // ========== Messages ==========

    // create concentrated pool
    fn_execute! { pub create_concentrated_pool: MsgCreateConcentratedPool => MsgCreateConcentratedPoolResponse }

    // create position
    fn_execute! { pub create_position: MsgCreatePosition => MsgCreatePositionResponse }

    // withdraw position
    fn_execute! { pub withdraw_position: MsgWithdrawPosition => MsgWithdrawPositionResponse }

    // collect fees
    fn_execute! { pub collected_fees: MsgCollectFees => MsgCollectFeesResponse }

    // collect incentives
    fn_execute! { pub collect_incentives: MsgCollectIncentives => MsgCollectIncentivesResponse }

    // create incentive
    fn_execute! { pub create_incentive: MsgCreateIncentive => MsgCreateIncentiveResponse }

    // ========== Queries ==========

    // query pools
    fn_query! {
        pub query_pools ["/osmosis.concentratedliquidity.v1beta1.Query/Pools"]: QueryPoolsRequest => QueryPoolsResponse
    }

    // query params
    fn_query! {
        pub query_params ["/osmosis.concentratedliquidity.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    // query liquidity_depths_for_range
    fn_query! {
        pub query_liquidity_depths_for_range ["/osmosis.concentratedliquidity.v1beta1.Query/LiquidityDepthsForRange"]: QueryLiquidityDepthsForRangeRequest => QueryLiquidityDepthsForRangeResponse
    }

    // query user_positions
    fn_query! {
        pub query_user_positions ["/osmosis.concentratedliquidity.v1beta1.Query/UserPositions"]: QueryUserPositionsRequest => QueryUserPositionsResponse
    }

    // query total_liquidity_for_range
    fn_query! {
        pub query_total_liquidity_for_range ["/osmosis.concentratedliquidity.v1beta1.Query/TotalLiquidityForRange"]: QueryTotalLiquidityForRangeRequest => QueryTotalLiquidityForRangeResponse
    }

    // query claimable_fees
    fn_query! {
        pub query_claimable_fees ["/osmosis.concentratedliquidity.v1beta1.Query/ClaimableFees"]: QueryClaimableFeesRequest => QueryClaimableFeesResponse
    }

    // query position_by_id
    fn_query! {
        pub query_position_by_id ["/osmosis.concentratedliquidity.v1beta1.Query/PositionById"]: QueryPositionByIdRequest => QueryPositionByIdResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use osmosis_std::types::cosmos::base::v1beta1;
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint};
    use test_tube::Account;

    use crate::{OsmosisTestApp, TokenFactory};

    use super::*;

    #[test]
    fn test_concentrated_liquidity() {
        let app = OsmosisTestApp::new();
        let signer = app
            .init_account(&[Coin::new(10_000_000_000, "uosmo")])
            .unwrap()
            .with_fee_setting(test_tube::account::FeeSetting::Auto {
                gas_price: Coin::new(25, "uosmo"),
                gas_adjustment: 1.2,
            });

        let tokenfactory = TokenFactory::new(&app);
        let concentrated_liquidity = ConcentratedLiquidity::new(&app);

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
                    amount: Some(Coin::new(100_000_000_000, &denom0).into()),
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

        // create pool
        let pool_id = concentrated_liquidity
            .create_concentrated_pool(
                MsgCreateConcentratedPool {
                    sender: signer.address(),
                    denom0: denom0.clone(),
                    denom1: denom1.clone(),
                    tick_spacing: 1,
                    precision_factor_at_price_one: "-10".to_string(),
                    swap_fee: "0".to_string(),
                },
                &signer,
            )
            .unwrap()
            .data
            .pool_id;

        assert_eq!(pool_id, 1);

        let position_id = concentrated_liquidity
            .create_position(
                MsgCreatePosition {
                    pool_id,
                    sender: signer.address(),
                    lower_tick: 0,
                    upper_tick: 100,
                    token_desired0: Some(v1beta1::Coin {
                        denom: denom0,
                        amount: "10000000000".to_string(),
                    }),
                    token_desired1: Some(v1beta1::Coin {
                        denom: denom1,
                        amount: "10000000000".to_string(),
                    }),
                    token_min_amount0: "1".to_string(),
                    token_min_amount1: "1".to_string(),
                    freeze_duration: None,
                },
                &signer,
            )
            .unwrap()
            .data
            .position_id;

        assert_eq!(position_id, 1);
    }
}
