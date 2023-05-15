use injective_std::types::injective::exchange::v1beta1::{
    MsgCreateDerivativeLimitOrder, MsgCreateDerivativeLimitOrderResponse, MsgCreateSpotLimitOrder,
    MsgCreateSpotLimitOrderResponse, MsgInstantPerpetualMarketLaunch,
    MsgInstantPerpetualMarketLaunchResponse, MsgInstantSpotMarketLaunch,
    MsgInstantSpotMarketLaunchResponse, MsgPrivilegedExecuteContract,
    MsgPrivilegedExecuteContractResponse, QueryDerivativeMarketsRequest,
    QueryDerivativeMarketsResponse, QueryDerivativeMidPriceAndTobRequest,
    QueryDerivativeMidPriceAndTobResponse, QuerySpotMarketsRequest, QuerySpotMarketsResponse,
    QuerySpotMidPriceAndTobRequest, QuerySpotMidPriceAndTobResponse,
    QuerySubaccountDepositsRequest, QuerySubaccountDepositsResponse,
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

    fn_execute! {
        pub create_spot_limit_order: MsgCreateSpotLimitOrder => MsgCreateSpotLimitOrderResponse
    }

    fn_execute! {
        pub create_deriviative_limit_order: MsgCreateDerivativeLimitOrder => MsgCreateDerivativeLimitOrderResponse
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
        pub query_spot_mid_price_and_tob ["/injective.exchange.v1beta1.Query/SpotMidPriceAndTOB"]: QuerySpotMidPriceAndTobRequest => QuerySpotMidPriceAndTobResponse
    }

    fn_query! {
        pub query_derivative_mid_price_and_tob ["/injective.exchange.v1beta1.Query/DerivativeMidPriceAndTOB"]: QueryDerivativeMidPriceAndTobRequest => QueryDerivativeMidPriceAndTobResponse
    }

    fn_query! {
        pub query_derivative_markets ["/injective.exchange.v1beta1.Query/DerivativeMarkets"]: QueryDerivativeMarketsRequest => QueryDerivativeMarketsResponse
    }

    fn_query! {
        pub query_subaccount_deposits ["/injective.exchange.v1beta1.Query/SubaccountDeposits"]: QuerySubaccountDepositsRequest => QuerySubaccountDepositsResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin};
    use injective_cosmwasm::get_default_subaccount_id_for_checked_address;
    use injective_std::types::injective::exchange::v1beta1::{
        MarketStatus, MsgCreateSpotLimitOrder, MsgInstantSpotMarketLaunch, OrderInfo,
        QuerySpotMarketsRequest, QuerySpotMarketsResponse, QuerySpotMidPriceAndTobRequest,
        QuerySpotMidPriceAndTobResponse, SpotMarket, SpotOrder,
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
        let trader = app
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
            .unwrap_err();

        app.increase_time(1u64);

        let spot_markets = exchange
            .query_spot_markets(&QuerySpotMarketsRequest {
                status: "Active".to_owned(),
            })
            .unwrap();

        let expected_response = QuerySpotMarketsResponse {
            markets: vec![SpotMarket {
                ticker: "INJ/USDT".to_string(),
                base_denom: "inj".to_string(),
                quote_denom: "usdt".to_string(),
                maker_fee_rate: "-100000000000000".to_string(),
                taker_fee_rate: "1000000000000000".to_string(),
                relayer_fee_share_rate: "400000000000000000".to_string(),
                market_id: "0xd5a22be807011d5e42d5b77da3f417e22676efae494109cd01c242ad46630115"
                    .to_string(),
                status: MarketStatus::Active.into(),
                min_price_tick_size: "10000".to_string(),
                min_quantity_tick_size: "100000".to_string(),
            }],
        };
        assert_eq!(spot_markets, expected_response);

        let spot_mid_price_and_tob = exchange
            .query_spot_mid_price_and_tob(&QuerySpotMidPriceAndTobRequest {
                market_id: "0xd5a22be807011d5e42d5b77da3f417e22676efae494109cd01c242ad46630115"
                    .to_string(),
            })
            .unwrap();

        let expected_response = QuerySpotMidPriceAndTobResponse {
            mid_price: "".to_string(),
            best_buy_price: "".to_string(),
            best_sell_price: "".to_string(),
        };
        assert_eq!(spot_mid_price_and_tob, expected_response);

        exchange
            .create_spot_limit_order(
                MsgCreateSpotLimitOrder {
                    sender: signer.address(),
                    order: Some(SpotOrder {
                        market_id:
                            "0xd5a22be807011d5e42d5b77da3f417e22676efae494109cd01c242ad46630115"
                                .to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: get_default_subaccount_id_for_checked_address(
                                &Addr::unchecked(signer.address()),
                            )
                            .as_str()
                            .to_string(),
                            fee_recipient: signer.address(),
                            price: "1000000000000000000".to_string(),
                            quantity: "10000000000000000000".to_string(),
                        }),
                        order_type: 1i32,
                        trigger_price: "".to_string(),
                    }),
                },
                &signer,
            )
            .unwrap();

        exchange
            .create_spot_limit_order(
                MsgCreateSpotLimitOrder {
                    sender: trader.address(),
                    order: Some(SpotOrder {
                        market_id:
                            "0xd5a22be807011d5e42d5b77da3f417e22676efae494109cd01c242ad46630115"
                                .to_string(),
                        order_info: Some(OrderInfo {
                            subaccount_id: get_default_subaccount_id_for_checked_address(
                                &Addr::unchecked(trader.address()),
                            )
                            .as_str()
                            .to_string(),
                            fee_recipient: trader.address(),
                            price: "2000000000000000000".to_string(),
                            quantity: "10000000000000000000".to_string(),
                        }),
                        order_type: 2i32,
                        trigger_price: "".to_string(),
                    }),
                },
                &trader,
            )
            .unwrap();

        let spot_mid_price_and_tob = exchange
            .query_spot_mid_price_and_tob(&QuerySpotMidPriceAndTobRequest {
                market_id: "0xd5a22be807011d5e42d5b77da3f417e22676efae494109cd01c242ad46630115"
                    .to_string(),
            })
            .unwrap();

        let expected_response = QuerySpotMidPriceAndTobResponse {
            mid_price: "1500000000000000000".to_string(),
            best_buy_price: "1000000000000000000".to_string(),
            best_sell_price: "2000000000000000000".to_string(),
        };
        assert_eq!(spot_mid_price_and_tob, expected_response);
    }
}
