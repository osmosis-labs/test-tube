use tx_wasm_sdk::types::cosmos::nft::v1beta1::{
    MsgSend, MsgSendResponse, QueryBalanceRequest, QueryBalanceResponse, QueryClassRequest,
    QueryClassResponse, QueryClassesRequest, QueryClassesResponse, QueryNfTsRequest,
    QueryNfTsResponse, QueryNftRequest, QueryNftResponse, QueryOwnerRequest, QueryOwnerResponse,
    QuerySupplyRequest, QuerySupplyResponse,
};
use test_tube_tx::{fn_execute, fn_query, Module};

use test_tube_tx::runner::Runner;

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
        pub query_balance ["/cosmos.nft.v1beta1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_owner ["/cosmos.nft.v1beta1.Query/Owner"]: QueryOwnerRequest => QueryOwnerResponse
    }

    fn_query! {
        pub query_supply ["/cosmos.nft.v1beta1.Query/Supply"]: QuerySupplyRequest => QuerySupplyResponse
    }

    fn_query! {
        pub query_nfts ["/cosmos.nft.v1beta1.Query/NFTs"]: QueryNfTsRequest => QueryNfTsResponse
    }

    fn_query! {
        pub query_nft["/cosmos.nft.v1beta1.Query/NFT"]: QueryNftRequest => QueryNftResponse
    }

    fn_query! {
        pub query_class ["/cosmos.nft.v1beta1.Query/Class"]: QueryClassRequest => QueryClassResponse
    }

    fn_query! {
        pub query_classes ["/cosmos.nft.v1beta1.Query/Classes"]: QueryClassesRequest => QueryClassesResponse
    }
}
