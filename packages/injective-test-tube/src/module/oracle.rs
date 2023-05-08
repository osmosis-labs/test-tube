use injective_std::types::injective::oracle::v1beta1::{
    MsgInstantPerpetualMarketLaunch, MsgInstantPerpetualMarketLaunchResponse,
    MsgInstantSpotMarketLaunch, MsgInstantSpotMarketLaunchResponse, MsgPrivilegedExecuteContract,
    MsgPrivilegedExecuteContractResponse, QueryDerivativeMarketsRequest,
    QueryDerivativeMarketsResponse, QuerySpotMarketsRequest, QuerySpotMarketsResponse,
    QuerySubaccountDepositsRequest, QuerySubaccountDepositsResponse,
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
        pub instant_spot_market_launch: MsgInstantSpotMarketLaunch => MsgInstantSpotMarketLaunchResponse
    }

    fn_execute! {
        pub instant_perpetual_market_launch: MsgInstantPerpetualMarketLaunch => MsgInstantPerpetualMarketLaunchResponse
    }

    fn_execute! {
        pub privileged_execute_contract: MsgPrivilegedExecuteContract => MsgPrivilegedExecuteContractResponse
    }

    fn_query! {
        pub query_spot_markets ["/injective.exchange.v1beta1.Query/SpotMarkets"]: QuerySpotMarketsRequest => QuerySpotMarketsResponse
    }

    fn_query! {
        pub query_derivative_markets ["/injective.exchange.v1beta1.Query/DerivativeMarkets"]: QueryDerivativeMarketsRequest => QueryDerivativeMarketsResponse
    }

    fn_query! {
        pub query_subaccount_deposits ["/injective.exchange.v1beta1.Query/SubaccountDeposits"]: QuerySubaccountDepositsRequest => QuerySubaccountDepositsResponse
    }
}
