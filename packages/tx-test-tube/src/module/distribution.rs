use tx_wasm_sdk::types::cosmos::distribution::v1beta1::{
    QueryDelegationRewardsRequest, QueryDelegationRewardsResponse,
    QueryDelegationTotalRewardsRequest, QueryDelegationTotalRewardsResponse,
};
use test_tube_tx::{fn_query, Module, Runner};

pub struct Distribution<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Distribution<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Distribution<'a, R>
where
    R: Runner<'a>,
{
    fn_query! {
        pub query_delegation_rewards ["/cosmos.distribution.v1beta1.Query/DelegationRewards"]: QueryDelegationRewardsRequest => QueryDelegationRewardsResponse
    }

    fn_query! {
        pub query_delegation_total_rewards ["/cosmos.distribution.v1beta1.Query/DelegationTotalRewards"]: QueryDelegationTotalRewardsRequest => QueryDelegationTotalRewardsResponse
    }
}
