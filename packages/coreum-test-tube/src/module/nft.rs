use coreum_wasm_sdk::types::coreum::nft::v1beta1::{
    MsgSend, MsgSendResponse, QueryBalanceRequest, QueryBalanceResponse, QueryClassRequest,
    QueryClassResponse, QueryClassesRequest, QueryClassesResponse, QueryNfTsRequest,
    QueryNfTsResponse, QueryNftRequest, QueryNftResponse, QueryOwnerRequest, QueryOwnerResponse,
    QuerySupplyRequest, QuerySupplyResponse,
};
use test_tube_coreum::{fn_execute, fn_query, Module};

use test_tube_coreum::runner::Runner;

pub struct NFT<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for NFT<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> NFT<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! { pub send: MsgSend => MsgSendResponse }

    fn_query! {
        pub query_balance ["/coreum.nft.v1beta1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_owner ["/coreum.nft.v1beta1.Query/Owner"]: QueryOwnerRequest => QueryOwnerResponse
    }

    fn_query! {
        pub query_supply ["/coreum.nft.v1beta1.Query/Supply"]: QuerySupplyRequest => QuerySupplyResponse
    }

    fn_query! {
        pub query_nfts ["/coreum.nft.v1beta1.Query/NFTs"]: QueryNfTsRequest => QueryNfTsResponse
    }

    fn_query! {
        pub query_nft["/coreum.nft.v1beta1.Query/NFT"]: QueryNftRequest => QueryNftResponse
    }

    fn_query! {
        pub query_class ["/coreum.nft.v1beta1.Query/Class"]: QueryClassRequest => QueryClassResponse
    }

    fn_query! {
        pub query_classes ["/coreum.nft.v1beta1.Query/Classes"]: QueryClassesRequest => QueryClassesResponse
    }
}
