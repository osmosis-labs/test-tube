use injective_std::types::injective::exchange::v1beta1::{
    MsgInstantSpotMarketLaunch, MsgInstantSpotMarketLaunchResponse, QuerySpotMarketsRequest,
    QuerySpotMarketsResponse,
};

use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct Exchange<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Exchange<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Exchange<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub instant_spot_market_launch: MsgInstantSpotMarketLaunch => MsgInstantSpotMarketLaunchResponse
    }

    fn_query! {
        pub query_spot_markets ["/injective.tokenfactory.v1beta1.Query/DenomsFromCreator"]: QuerySpotMarketsRequest => QuerySpotMarketsResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use injective_std::types::injective::exchange::v1beta1::{
        MsgInstantSpotMarketLaunch, QuerySpotMarketsRequest,
    };

    use crate::{Account, Exchange, InjectiveTestApp};
    use test_tube::Module;

    #[test]
    fn exchange_integration() {
        let app = InjectiveTestApp::new();
        let signer = app
            .init_account(&[
                Coin::new(10_000_000_000_000_000_000_000u128, "inj"),
                Coin::new(100_000_000_000_000_000_000u128, "usdt"),
            ])
            .unwrap();
        let exchange = Exchange::new(&app);

        exchange
            .instant_spot_market_launch(
                MsgInstantSpotMarketLaunch {
                    sender: signer.address(),
                    ticker: "INJ/USDT".to_owned(),
                    base_denom: "inj".to_owned(),
                    quote_denom: "usdt".to_owned(),
                    min_price_tick_size: "10000".to_owned(),
                    min_quantity_tick_size: "100000".to_owned(),
                },
                &signer,
            )
            .unwrap()
            .data;

        let spot_markets = exchange
            .query_spot_markets(&QuerySpotMarketsRequest {
                status: "".to_string(),
            })
            .unwrap();

        println!("{:?}", spot_markets);
        assert_eq!(1, 2);
    }
}
