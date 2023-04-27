use osmosis_std_derive::CosmwasmExt;

/// SpotMarket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/injective.exchange.v1beta1.SpotMarket")]
pub struct SpotMarket {
    #[prost(string, tag = "1")]
    pub ticker: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub relater_fee_share_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    #[serde(alias = "MarketStatus")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_status: u64,
    #[prost(string, tag = "9")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}

/// MsgInstantSpotMarketLaunch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/injective.exchange.v1beta1.MsgInstantSpotMarketLaunch")]
pub struct MsgInstantSpotMarketLaunch {
    /// Can be empty for no admin, or a valid address
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub quote_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
}

/// MsgInstantSpotMarketLaunchResponse is the return value of MsgInstantSpotMarketLaunch\
/// apparently this is empty
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/injective.exchange.v1beta1.MsgInstantSpotMarketLaunchResponse")]
pub struct MsgInstantSpotMarketLaunchResponse {}

/// QuerySpotMarketsRequest defines the request structure for the
/// DenomsFromCreator gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/injective.exchange.v1beta1.QuerySpotMarketsRequest")]
#[proto_query(
    path = "/injective.exchange.v1beta1.Query/SpotMarkets",
    response_type = QuerySpotMarketsResponse
)]
pub struct QuerySpotMarketsRequest {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
/// QuerySpotMarketsRequest defines the response structure for the
/// SpotMarkets gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/injective.exchange.v1beta1.QuerySpotMarketsResponse")]
pub struct QuerySpotMarketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<SpotMarket>,
}

pub struct ExchangeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ExchangeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn query_spot_markets(
        &self,
        status: ::prost::alloc::string::String,
    ) -> Result<QuerySpotMarketsResponse, cosmwasm_std::StdError> {
        QuerySpotMarketsRequest { status }.query(self.querier)
    }
}
