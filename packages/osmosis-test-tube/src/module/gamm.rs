use cosmwasm_std::{Coin, StdResult};
use osmosis_std::types::osmosis::gamm;
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::{
    MsgCreateStableswapPool, MsgCreateStableswapPoolResponse,
};

use osmosis_std::types::osmosis::gamm::{
    poolmodels::balancer::v1beta1::{MsgCreateBalancerPool, MsgCreateBalancerPoolResponse},
    v1beta1::{PoolAsset, PoolParams},
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::{PoolRequest, PoolResponse};

use prost::Message;
use test_tube::{fn_execute, fn_query};

use test_tube::module::Module;
use test_tube::runner::error::{DecodeError, RunnerError};
use test_tube::runner::result::{RunnerExecuteResult, RunnerResult};
use test_tube::{
    account::{Account, SigningAccount},
    runner::Runner,
};

pub struct Gamm<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Gamm<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Gamm<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_balancer_pool: MsgCreateBalancerPool => MsgCreateBalancerPoolResponse
    }

    fn_execute! {
        pub create_stable_swap_pool: MsgCreateStableswapPool => MsgCreateStableswapPoolResponse
    }

    fn_query! {
        _query_pool ["/osmosis.poolmanager.v1beta1.Query/Pool"]: PoolRequest => PoolResponse
    }

    pub fn create_basic_pool(
        &self,
        initial_liquidity: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgCreateBalancerPoolResponse> {
        self.create_balancer_pool(
            MsgCreateBalancerPool {
                sender: signer.address(),
                pool_params: Some(PoolParams {
                    swap_fee: "10000000000000000".to_string(),
                    exit_fee: "0".to_string(),
                    smooth_weight_change_params: None,
                }),
                pool_assets: initial_liquidity
                    .iter()
                    .map(|c| PoolAsset {
                        token: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                            denom: c.denom.to_owned(),
                            amount: format!("{}", c.amount),
                        }),
                        weight: "1000000".to_string(),
                    })
                    .collect(),
                future_pool_governor: "".to_string(),
            },
            signer,
        )
    }

    pub fn query_pool(&self, pool_id: u64) -> RunnerResult<gamm::v1beta1::Pool> {
        let res = self._query_pool(&PoolRequest { pool_id })?;
        gamm::v1beta1::Pool::decode(res.pool.unwrap().value.as_slice())
            .map_err(DecodeError::ProtoDecodeError)
            .map_err(RunnerError::DecodeError)
    }

    pub fn query_pool_reserves(&self, pool_id: u64) -> RunnerResult<Vec<Coin>> {
        let pool = self.query_pool(pool_id)?;

        let result = pool
            .pool_assets
            .into_iter()
            .filter_map(|asset| asset.token.map(|coin| coin.try_into()))
            .collect::<StdResult<Vec<Coin>>>()
            .map_err(|e| RunnerError::GenericError(e.to_string()))?;
        Ok(result)
    }
}
