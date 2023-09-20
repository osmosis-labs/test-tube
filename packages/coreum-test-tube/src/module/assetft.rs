use coreum_wasm_sdk::types::coreum::asset::ft::v1::{
    EmptyResponse, MsgBurn, MsgFreeze, MsgGloballyFreeze, MsgGloballyUnfreeze, MsgIssue, MsgMint,
    MsgSetWhitelistedLimit, MsgUnfreeze, MsgUpgradeTokenV1, QueryBalanceRequest,
    QueryBalanceResponse, QueryFrozenBalanceRequest, QueryFrozenBalanceResponse,
    QueryFrozenBalancesRequest, QueryFrozenBalancesResponse, QueryParamsRequest,
    QueryParamsResponse, QueryTokenRequest, QueryTokenResponse, QueryTokenUpgradeStatusesRequest,
    QueryTokenUpgradeStatusesResponse, QueryTokensRequest, QueryTokensResponse,
    QueryWhitelistedBalanceRequest, QueryWhitelistedBalanceResponse,
    QueryWhitelistedBalancesRequest, QueryWhitelistedBalancesResponse,
};
use test_tube_coreum::{fn_execute, fn_query, Module};

use test_tube_coreum::runner::Runner;

pub struct AssetFT<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for AssetFT<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> AssetFT<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! { pub issue: MsgIssue => EmptyResponse }

    fn_execute! { pub mint: MsgMint => EmptyResponse }

    fn_execute! { pub burn: MsgBurn => EmptyResponse }

    fn_execute! { pub freeze: MsgFreeze => EmptyResponse }

    fn_execute! { pub unfreeze: MsgUnfreeze => EmptyResponse }

    fn_execute! { pub globally_freeze: MsgGloballyFreeze => EmptyResponse }

    fn_execute! { pub globally_unfreeze: MsgGloballyUnfreeze => EmptyResponse }

    fn_execute! { pub set_whitelisted_limit: MsgSetWhitelistedLimit => EmptyResponse }

    fn_execute! { pub upgrade_token_v1: MsgUpgradeTokenV1 => EmptyResponse }

    fn_query! {
        pub query_params ["/coreum.asset.ft.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_token ["/coreum.asset.ft.v1.Query/Token"]: QueryTokenRequest => QueryTokenResponse
    }

    fn_query! {
        pub query_token_upgrade_statuses ["/coreum.asset.ft.v1.Query/TokenUpgradeStatuses"]: QueryTokenUpgradeStatusesRequest => QueryTokenUpgradeStatusesResponse
    }

    fn_query! {
        pub query_tokens ["/coreum.asset.ft.v1.Query/Tokens"]: QueryTokensRequest => QueryTokensResponse
    }

    fn_query! {
        pub query_balance ["/coreum.asset.ft.v1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_frozen_balances ["/coreum.asset.ft.v1.Query/FrozenBalances"]: QueryFrozenBalancesRequest => QueryFrozenBalancesResponse
    }

    fn_query! {
        pub query_frozen_balance ["/coreum.asset.ft.v1.Query/FrozenBalance"]: QueryFrozenBalanceRequest => QueryFrozenBalanceResponse
    }

    fn_query! {
        pub query_whitelisted_balances ["/coreum.asset.ft.v1.Query/WhitelistedBalances"]: QueryWhitelistedBalancesRequest => QueryWhitelistedBalancesResponse
    }

    fn_query! {
        pub query_whitelisted_balance ["/coreum.asset.ft.v1.Query/WhitelistedBalance"]: QueryWhitelistedBalanceRequest => QueryWhitelistedBalanceResponse
    }
}

#[cfg(test)]
mod tests {
    use coreum_wasm_sdk::assetft::MINTING;
    use coreum_wasm_sdk::types::coreum::asset::ft::v1::{
        MsgIssue, MsgMint, QueryBalanceRequest, QueryParamsRequest,
    };
    use coreum_wasm_sdk::types::cosmos::bank::v1beta1::MsgSend;
    use coreum_wasm_sdk::types::cosmos::base::v1beta1::Coin as BaseCoin;
    use cosmwasm_std::Coin;

    use crate::runner::app::FEE_DENOM;
    use crate::{Account, AssetFT, Bank, CoreumTestApp, Module};

    #[test]
    fn asset_ft_testing() {
        let app = CoreumTestApp::new();

        let signer = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();
        let receiver = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();

        let assetft = AssetFT::new(&app);
        let bank = Bank::new(&app);
        let request_params = assetft.query_params(&QueryParamsRequest {}).unwrap();

        assert_eq!(
            request_params.params.unwrap().issue_fee.unwrap(),
            BaseCoin {
                amount: 10000000u128.to_string(),
                denom: FEE_DENOM.to_string(),
            }
        );

        assetft
            .issue(
                MsgIssue {
                    issuer: signer.address(),
                    symbol: "TEST".to_string(),
                    subunit: "utest".to_string(),
                    precision: 6,
                    initial_amount: "10".to_string(),
                    description: "test_description".to_string(),
                    features: vec![MINTING as i32],
                    burn_rate: "0".to_string(),
                    send_commission_rate: "0".to_string(),
                },
                &signer,
            )
            .unwrap();

        let denom = format!("{}-{}", "utest", signer.address()).to_lowercase();
        let request_balance = assetft
            .query_balance(&QueryBalanceRequest {
                account: signer.address(),
                denom: denom.clone(),
            })
            .unwrap();

        assert_eq!(request_balance.balance, "10".to_string());

        assetft
            .mint(
                MsgMint {
                    sender: signer.address(),
                    coin: Some(BaseCoin {
                        denom: denom.clone(),
                        amount: "990".to_string(),
                    }),
                },
                &signer,
            )
            .unwrap();

        let request_balance = assetft
            .query_balance(&QueryBalanceRequest {
                account: signer.address(),
                denom: denom.clone(),
            })
            .unwrap();

        assert_eq!(request_balance.balance, "1000".to_string());

        bank.send(
            MsgSend {
                from_address: signer.address(),
                to_address: receiver.address(),
                amount: vec![BaseCoin {
                    amount: "100".to_string(),
                    denom: denom.clone(),
                }],
            },
            &signer,
        )
        .unwrap();

        let request_balance = assetft
            .query_balance(&QueryBalanceRequest {
                account: signer.address(),
                denom: denom.clone(),
            })
            .unwrap();

        assert_eq!(request_balance.balance, "900".to_string());

        let request_balance = assetft
            .query_balance(&QueryBalanceRequest {
                account: receiver.address(),
                denom: denom.clone(),
            })
            .unwrap();

        assert_eq!(request_balance.balance, "100".to_string());
    }
}
