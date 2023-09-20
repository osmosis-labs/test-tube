use coreum_wasm_sdk::types::coreum::asset::nft::v1::{
    EmptyResponse, MsgAddToWhitelist, MsgBurn, MsgFreeze, MsgIssueClass, MsgMint,
    MsgRemoveFromWhitelist, MsgUnfreeze, QueryBurntNftRequest, QueryBurntNftResponse,
    QueryClassRequest, QueryClassResponse, QueryClassesRequest, QueryClassesResponse,
    QueryFrozenRequest, QueryFrozenResponse, QueryParamsRequest, QueryParamsResponse,
    QueryWhitelistedAccountsForNftRequest, QueryWhitelistedAccountsForNftResponse,
    QueryWhitelistedRequest, QueryWhitelistedResponse,
};
use test_tube_coreum::{fn_execute, fn_query, Module};

use test_tube_coreum::runner::Runner;

pub struct AssetNFT<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for AssetNFT<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> AssetNFT<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! { pub issue: MsgIssueClass => EmptyResponse }

    fn_execute! { pub mint: MsgMint => EmptyResponse }

    fn_execute! { pub burn: MsgBurn => EmptyResponse }

    fn_execute! { pub freeze: MsgFreeze => EmptyResponse }

    fn_execute! { pub unfreeze: MsgUnfreeze => EmptyResponse }

    fn_execute! { pub add_to_whitelist: MsgAddToWhitelist => EmptyResponse }

    fn_execute! { pub remove_from_whitelist: MsgRemoveFromWhitelist => EmptyResponse }

    fn_query! {
        pub query_params ["/coreum.asset.nft.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_class ["/coreum.asset.nft.v1.Query/Class"]: QueryClassRequest => QueryClassResponse
    }

    fn_query! {
        pub query_classes ["/coreum.asset.nft.v1.Query/Classes"]: QueryClassesRequest => QueryClassesResponse
    }

    fn_query! {
        pub query_frozen ["/coreum.asset.nft.v1.Query/Frozen"]: QueryFrozenRequest => QueryFrozenResponse
    }

    fn_query! {
        pub query_whitelisted ["/coreum.asset.nft.v1.Query/Whitelisted"]: QueryWhitelistedRequest => QueryWhitelistedResponse
    }

    fn_query! {
        pub query_whitelisted_accounts_for_nft ["/coreum.asset.nft.v1.Query/WhitelistedAccountsForNFT"]: QueryWhitelistedAccountsForNftRequest => QueryWhitelistedAccountsForNftResponse
    }

    fn_query! {
        pub query_burnt_nft ["/coreum.asset.nft.v1.Query/BurntNFT"]: QueryBurntNftRequest => QueryBurntNftResponse
    }

    fn_query! {
        pub query_burnt_nfts_in_class ["/coreum.asset.nft.v1.Query/BurntNFTsInClass"]: QueryBurntNftRequest => QueryBurntNftResponse
    }
}

#[cfg(test)]
mod tests {
    use coreum_wasm_sdk::{
        assetnft::BURNING,
        types::coreum::{
            asset::nft::v1::{MsgIssueClass, MsgMint, QueryParamsRequest},
            nft::v1beta1::{MsgSend, QueryOwnerRequest},
        },
    };

    use coreum_wasm_sdk::types::cosmos::base::v1beta1::Coin as BaseCoin;
    use cosmwasm_std::Coin;

    use test_tube_coreum::{Account, Module};

    use crate::{runner::app::FEE_DENOM, AssetNFT, CoreumTestApp, NFT};

    #[test]
    fn asset_nft_testing() {
        let app = CoreumTestApp::new();

        let signer = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();
        let receiver = app.init_account(&[]).unwrap();

        let assetnft = AssetNFT::new(&app);
        let nft = NFT::new(&app);

        let request_params = assetnft.query_params(&QueryParamsRequest {}).unwrap();

        assert_eq!(
            request_params.params.unwrap().mint_fee.unwrap(),
            BaseCoin {
                amount: 0.to_string(),
                denom: FEE_DENOM.to_string(),
            }
        );

        assetnft
            .issue(
                MsgIssueClass {
                    issuer: signer.address(),
                    symbol: "TEST".to_string(),
                    name: "TEST_NAME".to_string(),
                    description: "test_description".to_string(),
                    uri: "".to_string(),
                    uri_hash: "".to_string(),
                    data: None,
                    features: vec![BURNING as i32],
                    royalty_rate: "0".to_string(),
                },
                &signer,
            )
            .unwrap();

        let class_id = format!("{}-{}", "TEST", signer.address()).to_lowercase();

        assetnft
            .mint(
                MsgMint {
                    sender: signer.address(),
                    class_id: class_id.clone(),
                    id: "test1".to_string(),
                    uri: "".to_string(),
                    uri_hash: "".to_string(),
                    data: None,
                },
                &signer,
            )
            .unwrap();

        nft.send(
            MsgSend {
                class_id: class_id.clone(),
                id: "test1".to_string(),
                sender: signer.address(),
                receiver: receiver.address(),
            },
            &signer,
        )
        .unwrap();

        let query_owner_response = nft
            .query_owner(&QueryOwnerRequest {
                class_id,
                id: "test1".to_string(),
            })
            .unwrap();

        assert_eq!(query_owner_response.owner, receiver.address());
    }
}
