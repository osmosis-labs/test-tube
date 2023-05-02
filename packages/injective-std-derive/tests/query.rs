use cosmwasm_std::{Empty, QueryRequest};
use injective_std_derive::CosmwasmExt;

#[derive(
    Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/injective.tokenfactory.v1beta1.QueryDenomsFromCreatorRequest")]
#[proto_query(
    path = "/injective.tokenfactory.v1beta1.Query/DenomsFromCreator",
    response_type = QueryDenomsFromCreatorResponse
)]
pub struct QueryDenomsFromCreatorRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/injective.tokenfactory.v1beta1.QueryDenomsFromCreatorResponse")]
pub struct QueryDenomsFromCreatorResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

fn main() {
    let _: QueryRequest<Empty> = QueryDenomsFromCreatorRequest {
        creator: "inj1sr9zm2pq3xrru7l7gz632t2rqs9caet9xulwvapcqagq9pytkcgqwfc3nk".to_string(),
    }
    .into();
}
