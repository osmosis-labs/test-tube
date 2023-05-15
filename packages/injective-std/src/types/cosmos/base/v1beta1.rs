use osmosis_std_derive::CosmwasmExt;
/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an Int which implements the custom method
/// signatures required by gogoproto.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.v1beta1.CoinCoin")]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// DecCoin defines a token with a denomination and a decimal amount.
///
/// NOTE: The amount field is an Dec which implements the custom method
/// signatures required by gogoproto.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.v1beta1.DecCoinDecCoin")]
pub struct DecCoin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// IntProto defines a Protobuf wrapper around an Int object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.v1beta1.IntProtoIntProto")]
pub struct IntProto {
    #[prost(string, tag = "1")]
    pub int: ::prost::alloc::string::String,
}
/// DecProto defines a Protobuf wrapper around a Dec object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.v1beta1.DecProtoDecProto")]
pub struct DecProto {
    #[prost(string, tag = "1")]
    pub dec: ::prost::alloc::string::String,
}
