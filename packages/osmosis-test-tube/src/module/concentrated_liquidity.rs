use osmosis_std::types::osmosis::concentratedliquidity::poolmodel::concentrated::v1beta1::{
    MsgCreateConcentratedPool, MsgCreateConcentratedPoolResponse,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ClaimableSpreadRewardsRequest, ClaimableSpreadRewardsResponse, LiquidityNetInDirectionRequest,
    LiquidityNetInDirectionResponse, LiquidityPerTickRangeRequest, LiquidityPerTickRangeResponse,
    MsgCollectIncentives, MsgCollectIncentivesResponse, MsgCollectSpreadRewards,
    MsgCollectSpreadRewardsResponse, MsgCreatePosition, MsgCreatePositionResponse,
    MsgWithdrawPosition, MsgWithdrawPositionResponse, ParamsRequest, ParamsResponse, PoolsRequest,
    PoolsResponse, PositionByIdRequest, PositionByIdResponse, UserPositionsRequest,
    UserPositionsResponse,
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

    // collect spread rewards
    fn_execute! { pub collected_spread_rewards: MsgCollectSpreadRewards => MsgCollectSpreadRewardsResponse }

    // collect incentives
    fn_execute! { pub collect_incentives: MsgCollectIncentives => MsgCollectIncentivesResponse }

    // ========== Queries ==========

    // query pools
    fn_query! {
        pub query_pools ["/osmosis.concentratedliquidity.v1beta1.Query/Pools"]: PoolsRequest => PoolsResponse
    }

    // query params
    fn_query! {
        pub query_params ["/osmosis.concentratedliquidity.v1beta1.Query/Params"]: ParamsRequest => ParamsResponse
    }

    // query liquidity_net_in_direction
    fn_query! {
        pub query_liquidity_depths_for_range ["/osmosis.concentratedliquidity.v1beta1.Query/LiquidityNetInDirection"]: LiquidityNetInDirectionRequest => LiquidityNetInDirectionResponse
    }

    // query user_positions
    fn_query! {
        pub query_user_positions ["/osmosis.concentratedliquidity.v1beta1.Query/UserPositions"]: UserPositionsRequest => UserPositionsResponse
    }

    // query liquidity_net_in_direction
    fn_query! {
        pub query_liquidity_net_in_direction ["/osmosis.concentratedliquidity.v1beta1.Query/LiquidityNetInDirection"]: LiquidityNetInDirectionRequest => LiquidityNetInDirectionResponse
    }

    // query liquidity_per_tick_range
    fn_query! {
        pub query_liquidity_per_tick_range ["/osmosis.concentratedliquidity.v1beta1.Query/LiquidityPerTickRange"]: LiquidityPerTickRangeRequest => LiquidityPerTickRangeResponse
    }

    // query claimable_fees
    fn_query! {
        pub query_claimable_fees ["/osmosis.concentratedliquidity.v1beta1.Query/ClaimableSpreadRewards"]: ClaimableSpreadRewardsRequest => ClaimableSpreadRewardsResponse
    }

    // query position_by_id
    fn_query! {
        pub query_position_by_id ["/osmosis.concentratedliquidity.v1beta1.Query/PositionById"]: PositionByIdRequest => PositionByIdResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use osmosis_std::types::cosmos::base::v1beta1;
    use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord,
    };
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint};
    use prost::Message;
    use test_tube::Account;

    use crate::{GovWithAppAccess, OsmosisTestApp, TokenFactory};

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

        gov.propose_and_execute(
            CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
            CreateConcentratedLiquidityPoolsProposal {
                title: "Create concentrated uosmo:usdc pool".to_string(),
                description: "Create concentrated uosmo:usdc pool, so that we can trade it"
                    .to_string(),
                pool_records: vec![PoolRecord {
                    denom0: denom0.clone(),
                    denom1: "uosmo".to_string(),
                    tick_spacing: 1,
                    spread_factor: "0".to_string(),
                }],
            },
            signer.address(),
            false,
            &signer,
        )
        .unwrap();

        let pools = concentrated_liquidity
            .query_pools(&PoolsRequest { pagination: None })
            .unwrap();

        let pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

        assert_eq!(pool.id, 1);

        let pool_id = pool.id;

        let position_id = concentrated_liquidity
            .create_position(
                MsgCreatePosition {
                    pool_id,
                    sender: signer.address(),
                    lower_tick: 0,
                    upper_tick: 100,
                    tokens_provided: vec![
                        v1beta1::Coin {
                            denom: denom0,
                            amount: "10000000000".to_string(),
                        },
                        v1beta1::Coin {
                            denom: "uosmo".to_string(),
                            amount: "10000000000".to_string(),
                        },
                    ],
                    token_min_amount0: "0".to_string(),
                    token_min_amount1: "0".to_string(),
                },
                &signer,
            )
            .unwrap()
            .data
            .position_id;

        assert_eq!(position_id, 1);
    }
}
