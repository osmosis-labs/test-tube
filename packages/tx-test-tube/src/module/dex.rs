use tx_wasm_sdk::types::coreum::dex::v1::{
    EmptyResponse, MsgCancelOrder, MsgCancelOrdersByDenom, MsgPlaceOrder,
    QueryAccountDenomOrdersCountRequest, QueryAccountDenomOrdersCountResponse,
    QueryOrderBookOrdersRequest, QueryOrderBookOrdersResponse, QueryOrderBookParamsRequest,
    QueryOrderBookParamsResponse, QueryOrderBooksRequest, QueryOrderBooksResponse,
    QueryOrderRequest, QueryOrderResponse, QueryOrdersRequest, QueryOrdersResponse,
    QueryParamsRequest, QueryParamsResponse,
};
use test_tube_tx::{fn_execute, fn_query, Module};

use test_tube_tx::runner::Runner;

pub struct Dex<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Dex<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Dex<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! { pub place_order: MsgPlaceOrder => EmptyResponse }

    fn_execute! { pub cancel_order: MsgCancelOrder => EmptyResponse }

    fn_execute! { pub cancel_orders_by_denom: MsgCancelOrdersByDenom => EmptyResponse }

    fn_query! {
        pub query_params ["/coreum.dex.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_order_book_params ["/coreum.dex.v1.Query/OrderBookParams"]: QueryOrderBookParamsRequest => QueryOrderBookParamsResponse
    }

    fn_query! {
        pub query_account_denom_orders_count ["/coreum.dex.v1.Query/AccountDenomOrdersCount"]: QueryAccountDenomOrdersCountRequest => QueryAccountDenomOrdersCountResponse
    }

    fn_query! {
        pub query_order_book_orders ["/coreum.dex.v1.Query/OrderBookOrders"]: QueryOrderBookOrdersRequest => QueryOrderBookOrdersResponse
    }

    fn_query! {
        pub query_order_books ["/coreum.dex.v1.Query/OrderBooks"]: QueryOrderBooksRequest => QueryOrderBooksResponse
    }

    fn_query! {
        pub query_order ["/coreum.dex.v1.Query/Order"]: QueryOrderRequest => QueryOrderResponse
    }

    fn_query! {
        pub query_orders ["/coreum.dex.v1.Query/Orders"]: QueryOrdersRequest => QueryOrdersResponse
    }
}

#[cfg(test)]
mod tests {
    use tx_wasm_sdk::types::coreum::asset::ft::v1::{DexSettings, Feature, MsgIssue};
    use tx_wasm_sdk::types::coreum::dex::v1::{
        MsgCancelOrder, MsgCancelOrdersByDenom, MsgPlaceOrder, OrderType,
        QueryAccountDenomOrdersCountRequest, QueryOrderBookOrdersRequest,
        QueryOrderBookParamsRequest, QueryOrderBooksRequest, QueryOrderRequest, QueryOrdersRequest,
        QueryParamsRequest, Side, TimeInForce,
    };
    // use tx_wasm_sdk::types::cosmos::bank::v1beta1::MsgSend;
    use tx_wasm_sdk::types::cosmos::base::v1beta1::Coin as BaseCoin;
    use cosmwasm_std::Coin;

    use crate::runner::app::FEE_DENOM;
    use crate::{Account, AssetFT, TXTestApp, Dex, Module};

    #[test]
    fn dex_testing() {
        let app = TXTestApp::new();

        let acc1 = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();
        app.init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();

        let assetft = AssetFT::new(&app);
        let dex = Dex::new(&app);

        let request_params = dex.query_params(&QueryParamsRequest {}).unwrap();
        let params = request_params.params.unwrap();
        assert_eq!(
            params.default_unified_ref_amount,
            "1000000000000000000000000"
        );
        assert_eq!(params.price_tick_exponent, -6);
        assert_eq!(params.max_orders_per_denom, 100);
        assert_eq!(
            params.order_reserve.unwrap(),
            BaseCoin {
                amount: 10000000u128.to_string(),
                denom: FEE_DENOM.to_string(),
            }
        );

        assetft
            .issue(
                MsgIssue {
                    issuer: acc1.address(),
                    symbol: "TKN1".to_string(),
                    subunit: "tkn1".to_string(),
                    precision: 5,
                    initial_amount: "1000000".to_string(),
                    description: "dex token 1".to_string(),
                    features: vec![
                        Feature::DexOrderCancellation as i32,
                        Feature::DexUnifiedRefAmountChange as i32,
                    ],
                    burn_rate: "0".to_string(),
                    send_commission_rate: "0".to_string(),
                    uri: "token_1_uri".to_string(),
                    uri_hash: "token_1_uri_hash".to_string(),
                    extension_settings: None,
                    dex_settings: Some(DexSettings {
                        unified_ref_amount: params.default_unified_ref_amount.to_string(),
                        whitelisted_denoms: vec![],
                    }),
                },
                &acc1,
            )
            .unwrap();

        let denom1 = format!("{}-{}", "tkn1", acc1.address()).to_lowercase();

        assetft
            .issue(
                MsgIssue {
                    issuer: acc1.address(),
                    symbol: "TKN2".to_string(),
                    subunit: "tkn2".to_string(),
                    precision: 5,
                    initial_amount: "1000000".to_string(),
                    description: "dex token 2".to_string(),
                    features: vec![
                        Feature::DexOrderCancellation as i32,
                        Feature::DexUnifiedRefAmountChange as i32,
                    ],
                    burn_rate: "0".to_string(),
                    send_commission_rate: "0".to_string(),
                    uri: "token_2_uri".to_string(),
                    uri_hash: "token_2_uri_hash".to_string(),
                    extension_settings: None,
                    dex_settings: Some(DexSettings {
                        unified_ref_amount: params.default_unified_ref_amount.to_string(),
                        whitelisted_denoms: vec![],
                    }),
                },
                &acc1,
            )
            .unwrap();

        let denom2 = format!("{}-{}", "tkn2", acc1.address()).to_lowercase();

        let msg_place_order = MsgPlaceOrder {
            sender: acc1.address(),
            r#type: OrderType::Limit as i32,
            id: "id".to_string(),
            base_denom: denom1.clone(),
            quote_denom: denom2.clone(),
            price: "1e-1".to_string(),
            quantity: "10000".to_string(),
            side: Side::Sell as i32,
            good_til: None,
            time_in_force: TimeInForce::Gtc as i32,
        };

        dex.place_order(msg_place_order.clone(), &acc1).unwrap();

        let request_order_book_params = dex
            .query_order_book_params(&QueryOrderBookParamsRequest {
                base_denom: denom1.clone(),
                quote_denom: denom2.clone(),
            })
            .unwrap();

        assert_eq!(request_order_book_params.price_tick, "1e-6");
        assert_eq!(request_order_book_params.quantity_step, "10000");
        assert_eq!(
            request_order_book_params.base_denom_unified_ref_amount,
            "1000000000000000000000000"
        );
        assert_eq!(
            request_order_book_params.quote_denom_unified_ref_amount,
            "1000000000000000000000000"
        );

        let request_order_books = dex
            .query_order_books(&QueryOrderBooksRequest { pagination: None })
            .unwrap();

        assert_eq!(request_order_books.order_books.len(), 2);

        dex.cancel_order(
            MsgCancelOrder {
                sender: acc1.address().to_string(),
                id: "id".to_string(),
            },
            &acc1,
        )
            .unwrap();

        let mut msg_place_order = msg_place_order.clone();
        msg_place_order.id = "id2".to_string();
        dex.place_order(msg_place_order, &acc1).unwrap();

        let request_order_book_orders = dex
            .query_order_book_orders(&QueryOrderBookOrdersRequest {
                base_denom: denom1.clone(),
                quote_denom: denom2.clone(),
                side: Side::Sell as i32,
                pagination: None,
            })
            .unwrap();

        assert_eq!(request_order_book_orders.orders.len(), 1);
        assert_eq!(request_order_book_orders.orders[0].id, "id2");
        assert_eq!(request_order_book_orders.orders[0].price, "1e-1");

        let request_order = dex
            .query_order(&QueryOrderRequest {
                creator: acc1.address().to_string(),
                id: "id2".to_string(),
            })
            .unwrap();

        let order = request_order.order.unwrap();
        assert_eq!(order.id, "id2");
        assert_eq!(order.price, "1e-1");

        let request_orders = dex
            .query_orders(&QueryOrdersRequest {
                creator: acc1.address().to_string(),
                pagination: None,
            })
            .unwrap();

        assert_eq!(request_orders.orders.len(), 1);
        assert_eq!(request_orders.orders[0].id, "id2");
        assert_eq!(request_orders.orders[0].price, "1e-1");

        let request_account_denom_orders_count = dex
            .query_account_denom_orders_count(&QueryAccountDenomOrdersCountRequest {
                account: acc1.address().to_string(),
                denom: denom1.clone(),
            })
            .unwrap();

        assert_eq!(request_account_denom_orders_count.count, 1);

        dex.cancel_orders_by_denom(
            MsgCancelOrdersByDenom {
                sender: acc1.address().to_string(),
                account: acc1.address().to_string(),
                denom: denom1.clone(),
            },
            &acc1,
        )
            .unwrap();
    }
}