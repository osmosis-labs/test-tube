use osmosis_std_derive::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.Params")]
pub struct Params {
    #[prost(string, tag = "1")]
    pub pyth_contract: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.OracleInfo")]
pub struct OracleInfo {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration = "OracleType", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub oracle_type: i32,
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub scale_factor: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.ChainlinkPriceState")]
pub struct ChainlinkPriceState {
    #[prost(string, tag = "1")]
    #[serde(alias = "feedID")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub answer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: u64,
    #[prost(message, optional, tag = "4")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.BandPriceState")]
pub struct BandPriceState {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub rate: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub resolve_time: u64,
    #[prost(uint64, tag = "4")]
    #[serde(alias = "requestID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub request_id: u64,
    #[prost(message, optional, tag = "5")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PriceFeedState")]
pub struct PriceFeedState {
    #[prost(string, tag = "1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub quote: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub price_state: ::core::option::Option<PriceState>,
    #[prost(string, repeated, tag = "4")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.ProviderInfo")]
pub struct ProviderInfo {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.ProviderState")]
pub struct ProviderState {
    #[prost(message, optional, tag = "1")]
    pub provider_info: ::core::option::Option<ProviderInfo>,
    #[prost(message, repeated, tag = "2")]
    pub provider_price_states: ::prost::alloc::vec::Vec<ProviderPriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.ProviderPriceState")]
pub struct ProviderPriceState {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PriceFeedInfo")]
pub struct PriceFeedInfo {
    #[prost(string, tag = "1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub quote: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PriceFeedPrice")]
pub struct PriceFeedPrice {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.CoinbasePriceState")]
pub struct CoinbasePriceState {
    /// kind should always be "prices"
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// timestamp of the when the price was signed by coinbase
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: u64,
    /// the symbol of the price, e.g. BTC
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
    /// the value of the price scaled by 1e6
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub value: u64,
    /// the price state
    #[prost(message, optional, tag = "5")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PriceState")]
pub struct PriceState {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PythPriceState")]
pub struct PythPriceState {
    #[prost(string, tag = "1")]
    #[serde(alias = "priceID")]
    pub price_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ema_price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ema_conf: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub conf: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub publish_time: u64,
    #[prost(message, optional, tag = "6")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.BandOracleRequest")]
pub struct BandOracleRequest {
    /// Unique Identifier for band ibc oracle request
    #[prost(uint64, tag = "1")]
    #[serde(alias = "requestID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub request_id: u64,
    /// OracleScriptID is the unique identifier of the oracle script to be executed.
    #[prost(int64, tag = "2")]
    #[serde(alias = "oracle_scriptID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub oracle_script_id: i64,
    /// Symbols is the list of symbols to prepare in the calldata
    #[prost(string, repeated, tag = "3")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// AskCount is the number of validators that are requested to respond to this
    /// oracle request. Higher value means more security, at a higher gas cost.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ask_count: u64,
    /// MinCount is the minimum number of validators necessary for the request to
    /// proceed to the execution phase. Higher value means more security, at the
    /// cost of liveness.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_count: u64,
    /// FeeLimit is the maximum tokens that will be paid to all data source providers.
    #[prost(message, repeated, tag = "6")]
    pub fee_limit: ::prost::alloc::vec::Vec<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    /// PrepareGas is amount of gas to pay to prepare raw requests
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub prepare_gas: u64,
    /// ExecuteGas is amount of gas to reserve for executing
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub execute_gas: u64,
    /// MinSourceCount is the minimum number of data sources that must be used by each validator
    #[prost(uint64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_source_count: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.BandIBCParams")]
pub struct BandIbcParams {
    /// true if Band IBC should be enabled
    #[prost(bool, tag = "1")]
    pub band_ibc_enabled: bool,
    /// block request interval to send Band IBC prices
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ibc_request_interval: i64,
    /// band IBC source channel
    #[prost(string, tag = "3")]
    pub ibc_source_channel: ::prost::alloc::string::String,
    /// band IBC version
    #[prost(string, tag = "4")]
    pub ibc_version: ::prost::alloc::string::String,
    /// band IBC portID
    #[prost(string, tag = "5")]
    #[serde(alias = "ibc_portID")]
    pub ibc_port_id: ::prost::alloc::string::String,
    ///   legacy oracle scheme ids
    #[prost(int64, repeated, tag = "6")]
    #[serde(alias = "legacy_oracleIDs")]
    pub legacy_oracle_ids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.SymbolPriceTimestamp")]
pub struct SymbolPriceTimestamp {
    #[prost(enumeration = "OracleType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub oracle: i32,
    #[prost(string, tag = "2")]
    #[serde(alias = "symbolID")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.LastPriceTimestamps")]
pub struct LastPriceTimestamps {
    #[prost(message, repeated, tag = "1")]
    pub last_price_timestamps: ::prost::alloc::vec::Vec<SymbolPriceTimestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PriceRecords")]
pub struct PriceRecords {
    #[prost(enumeration = "OracleType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub oracle: i32,
    #[prost(string, tag = "2")]
    #[serde(alias = "symbolID")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub latest_price_records: ::prost::alloc::vec::Vec<PriceRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PriceRecord")]
pub struct PriceRecord {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: i64,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
}
/// MetadataStatistics refers to the metadata summary statistics of the historical sample considered
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MetadataStatistics")]
pub struct MetadataStatistics {
    /// GroupCount refers to the number of groups used. Equals RecordsSampleSize if no grouping is used
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_count: u32,
    /// RecordsSampleSize refers to the total number of records used.
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub records_sample_size: u32,
    /// Mean refers to the arithmetic mean
    /// For trades, the mean is the VWAP computed over the grouped trade records ∑ (price * quantity) / ∑ quantity
    /// For oracle prices, the mean is computed over the price records ∑ (price) / prices_count
    #[prost(string, tag = "3")]
    pub mean: ::prost::alloc::string::String,
    /// TWAP refers to the time-weighted average price which equals ∑ (price_i * ∆t_i) / ∑ ∆t_i where ∆t_i = t_i - t_{i-1}
    #[prost(string, tag = "4")]
    pub twap: ::prost::alloc::string::String,
    /// FirstTimestamp is the timestamp of the oldest record considered
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub first_timestamp: i64,
    /// LastTimestamp is the timestamp of the youngest record considered
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_timestamp: i64,
    /// MinPrice refers to the smallest individual raw price considered
    #[prost(string, tag = "7")]
    pub min_price: ::prost::alloc::string::String,
    /// MaxPrice refers to the largest individual raw price considered
    #[prost(string, tag = "8")]
    pub max_price: ::prost::alloc::string::String,
    /// MedianPrice refers to the median individual raw price considered
    #[prost(string, tag = "9")]
    pub median_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PriceAttestation")]
pub struct PriceAttestation {
    #[prost(string, tag = "1")]
    #[serde(alias = "priceID")]
    pub price_id: ::prost::alloc::string::String,
    /// MaxPrice refers to the largest individual raw price considered
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub price: i64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub conf: u64,
    #[prost(int32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub expo: i32,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ema_price: i64,
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ema_conf: u64,
    #[prost(int32, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ema_expo: i32,
    #[prost(int64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub publish_time: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum OracleType {
    Unspecified = 0,
    Band = 1,
    PriceFeed = 2,
    Coinbase = 3,
    Chainlink = 4,
    Razor = 5,
    Dia = 6,
    Api3 = 7,
    Uma = 8,
    Pyth = 9,
    BandIbc = 10,
    Provider = 11,
}
impl OracleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OracleType::Unspecified => "Unspecified",
            OracleType::Band => "Band",
            OracleType::PriceFeed => "PriceFeed",
            OracleType::Coinbase => "Coinbase",
            OracleType::Chainlink => "Chainlink",
            OracleType::Razor => "Razor",
            OracleType::Dia => "Dia",
            OracleType::Api3 => "API3",
            OracleType::Uma => "Uma",
            OracleType::Pyth => "Pyth",
            OracleType::BandIbc => "BandIBC",
            OracleType::Provider => "Provider",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Band" => Some(Self::Band),
            "PriceFeed" => Some(Self::PriceFeed),
            "Coinbase" => Some(Self::Coinbase),
            "Chainlink" => Some(Self::Chainlink),
            "Razor" => Some(Self::Razor),
            "Dia" => Some(Self::Dia),
            "API3" => Some(Self::Api3),
            "Uma" => Some(Self::Uma),
            "Pyth" => Some(Self::Pyth),
            "BandIBC" => Some(Self::BandIbc),
            "Provider" => Some(Self::Provider),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.SetChainlinkPriceEvent")]
pub struct SetChainlinkPriceEvent {
    #[prost(string, tag = "1")]
    #[serde(alias = "feedID")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub answer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: u64,
}
/// Event type upon set ref
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.SetBandPriceEvent")]
pub struct SetBandPriceEvent {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub resolve_time: u64,
    #[prost(uint64, tag = "5")]
    #[serde(alias = "requestID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub request_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.SetBandIBCPriceEvent")]
pub struct SetBandIbcPriceEvent {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub prices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub resolve_time: u64,
    #[prost(uint64, tag = "5")]
    #[serde(alias = "requestID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub request_id: u64,
    #[prost(int64, tag = "6")]
    #[serde(alias = "clientID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub client_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.EventBandIBCAckSuccess")]
pub struct EventBandIbcAckSuccess {
    #[prost(string, tag = "1")]
    pub ack_result: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(alias = "clientID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub client_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.EventBandIBCAckError")]
pub struct EventBandIbcAckError {
    #[prost(string, tag = "1")]
    pub ack_error: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(alias = "clientID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub client_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.EventBandIBCResponseTimeout")]
pub struct EventBandIbcResponseTimeout {
    #[prost(int64, tag = "1")]
    #[serde(alias = "clientID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub client_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.SetPriceFeedPriceEvent")]
pub struct SetPriceFeedPriceEvent {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote: ::prost::alloc::string::String,
    /// price defines the price of the oracle base and quote
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.SetProviderPriceEvent")]
pub struct SetProviderPriceEvent {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.SetCoinbasePriceEvent")]
pub struct SetCoinbasePriceEvent {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.EventSetPythPrices")]
pub struct EventSetPythPrices {
    #[prost(message, repeated, tag = "1")]
    pub prices: ::prost::alloc::vec::Vec<PythPriceState>,
}
/// GenesisState defines the oracle module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of related to oracle.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, repeated, tag = "2")]
    pub band_relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub band_price_states: ::prost::alloc::vec::Vec<BandPriceState>,
    #[prost(message, repeated, tag = "4")]
    pub price_feed_price_states: ::prost::alloc::vec::Vec<PriceFeedState>,
    #[prost(message, repeated, tag = "5")]
    pub coinbase_price_states: ::prost::alloc::vec::Vec<CoinbasePriceState>,
    #[prost(message, repeated, tag = "6")]
    pub band_ibc_price_states: ::prost::alloc::vec::Vec<BandPriceState>,
    #[prost(message, repeated, tag = "7")]
    pub band_ibc_oracle_requests: ::prost::alloc::vec::Vec<BandOracleRequest>,
    #[prost(message, optional, tag = "8")]
    pub band_ibc_params: ::core::option::Option<BandIbcParams>,
    #[prost(uint64, tag = "9")]
    #[serde(alias = "band_ibc_latest_clientID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub band_ibc_latest_client_id: u64,
    #[prost(message, repeated, tag = "10")]
    pub calldata_records: ::prost::alloc::vec::Vec<CalldataRecord>,
    #[prost(uint64, tag = "11")]
    #[serde(alias = "band_ibc_latest_requestID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub band_ibc_latest_request_id: u64,
    #[prost(message, repeated, tag = "12")]
    pub chainlink_price_states: ::prost::alloc::vec::Vec<ChainlinkPriceState>,
    #[prost(message, repeated, tag = "13")]
    pub historical_price_records: ::prost::alloc::vec::Vec<PriceRecords>,
    #[prost(message, repeated, tag = "14")]
    pub provider_states: ::prost::alloc::vec::Vec<ProviderState>,
    #[prost(message, repeated, tag = "15")]
    pub pyth_price_states: ::prost::alloc::vec::Vec<PythPriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.CalldataRecord")]
pub struct CalldataRecord {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "clientID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub client_id: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub calldata: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.GrantBandOraclePrivilegeProposal")]
pub struct GrantBandOraclePrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.RevokeBandOraclePrivilegeProposal"
)]
pub struct RevokeBandOraclePrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.GrantPriceFeederPrivilegeProposal"
)]
pub struct GrantPriceFeederPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub quote: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.GrantProviderPrivilegeProposal")]
pub struct GrantProviderPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.RevokeProviderPrivilegeProposal")]
pub struct RevokeProviderPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.RevokePriceFeederPrivilegeProposal"
)]
pub struct RevokePriceFeederPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub quote: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.AuthorizeBandOracleRequestProposal"
)]
pub struct AuthorizeBandOracleRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<BandOracleRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.UpdateBandOracleRequestProposal")]
pub struct UpdateBandOracleRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "3")]
    #[serde(alias = "delete_requestIDs")]
    pub delete_request_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "4")]
    pub update_oracle_request: ::core::option::Option<BandOracleRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.EnableBandIBCProposal")]
pub struct EnableBandIbcProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub band_ibc_params: ::core::option::Option<BandIbcParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryPythPriceRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/PythPrice",
    response_type = QueryPythPriceResponse
)]
pub struct QueryPythPriceRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "priceID")]
    pub price_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryPythPriceResponse")]
pub struct QueryPythPriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price_state: ::core::option::Option<PythPriceState>,
}
/// QueryOracleParamsRequest is the request type for the Query/OracleParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryOracleParamsResponse is the response type for the Query/OracleParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryBandRelayersRequest is the request type for the Query/BandRelayers RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryBandRelayersRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/BandRelayers",
    response_type = QueryBandRelayersResponse
)]
pub struct QueryBandRelayersRequest {}
/// QueryBandRelayersResponse is the response type for the Query/BandRelayers RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryBandRelayersResponse")]
pub struct QueryBandRelayersResponse {
    #[prost(string, repeated, tag = "1")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryBandPriceStatesRequest is the request type for the Query/BandPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryBandPriceStatesRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/BandPriceStates",
    response_type = QueryBandPriceStatesResponse
)]
pub struct QueryBandPriceStatesRequest {}
/// QueryBandPriceStatesResponse is the response type for the Query/BandPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryBandPriceStatesResponse")]
pub struct QueryBandPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<BandPriceState>,
}
/// QueryBandIBCPriceStatesRequest is the request type for the Query/BandIBCPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryBandIBCPriceStatesRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/BandIBCPriceStates",
    response_type = QueryBandIbcPriceStatesResponse
)]
pub struct QueryBandIbcPriceStatesRequest {}
/// QueryBandIBCPriceStatesResponse is the response type for the Query/BandIBCPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryBandIBCPriceStatesResponse")]
pub struct QueryBandIbcPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<BandPriceState>,
}
/// QueryPriceFeedPriceStatesRequest is the request type for the Query/PriceFeedPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryPriceFeedPriceStatesRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/PriceFeedPriceStates",
    response_type = QueryPriceFeedPriceStatesResponse
)]
pub struct QueryPriceFeedPriceStatesRequest {}
/// QueryPriceFeedPriceStatesResponse is the response type for the Query/PriceFeedPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.QueryPriceFeedPriceStatesResponse"
)]
pub struct QueryPriceFeedPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<PriceFeedState>,
}
/// QueryCoinbasePriceStatesRequest is the request type for the Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryCoinbasePriceStatesRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/CoinbasePriceStates",
    response_type = QueryCoinbasePriceStatesResponse
)]
pub struct QueryCoinbasePriceStatesRequest {}
/// QueryCoinbasePriceStatesResponse is the response type for the Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryCoinbasePriceStatesResponse")]
pub struct QueryCoinbasePriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<CoinbasePriceState>,
}
/// QueryPythPriceStatesRequest is the request type for the Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryPythPriceStatesRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/PythPriceStates",
    response_type = QueryPythPriceStatesResponse
)]
pub struct QueryPythPriceStatesRequest {}
/// QueryPythPriceStatesResponse is the response type for the Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryPythPriceStatesResponse")]
pub struct QueryPythPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<PythPriceState>,
}
/// QueryProviderPriceStateRequest is the request type for the Query/ProviderPriceState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryProviderPriceStateRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/ProviderPriceState",
    response_type = QueryProviderPriceStateResponse
)]
pub struct QueryProviderPriceStateRequest {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
}
/// QueryProviderPriceStatesResponse is the response type for the Query/ProviderPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryProviderPriceStateResponse")]
pub struct QueryProviderPriceStateResponse {
    #[prost(message, optional, tag = "1")]
    pub price_state: ::core::option::Option<PriceState>,
}
/// QueryModuleStateRequest is the request type for the Query/OracleModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryModuleStateRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/OracleModuleState",
    response_type = QueryModuleStateResponse
)]
pub struct QueryModuleStateRequest {}
/// QueryModuleStateResponse is the response type for the Query/OracleModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryModuleStateResponse")]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.QueryHistoricalPriceRecordsRequest"
)]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/HistoricalPriceRecords",
    response_type = QueryHistoricalPriceRecordsResponse
)]
pub struct QueryHistoricalPriceRecordsRequest {
    #[prost(enumeration = "OracleType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub oracle: i32,
    #[prost(string, tag = "2")]
    #[serde(alias = "symbolID")]
    pub symbol_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.QueryHistoricalPriceRecordsResponse"
)]
pub struct QueryHistoricalPriceRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_records: ::prost::alloc::vec::Vec<PriceRecords>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.OracleHistoryOptions")]
pub struct OracleHistoryOptions {
    /// MaxAge restricts the oracle price records oldest age in seconds from the current block time to consider.
    /// A value of 0 means use all the records present on the chain.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_age: u64,
    /// If IncludeRawHistory is true, the raw underlying data used for the computation is included in the response
    #[prost(bool, tag = "2")]
    pub include_raw_history: bool,
    /// If IncludeMetadata is true, metadata on the computation is included in the response
    #[prost(bool, tag = "3")]
    pub include_metadata: bool,
}
/// QueryOracleVolatilityRequest is the request type for Query/OracleVolatility RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryOracleVolatilityRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/OracleVolatility",
    response_type = QueryOracleVolatilityResponse
)]
pub struct QueryOracleVolatilityRequest {
    #[prost(message, optional, tag = "1")]
    pub base_info: ::core::option::Option<OracleInfo>,
    #[prost(message, optional, tag = "2")]
    pub quote_info: ::core::option::Option<OracleInfo>,
    #[prost(message, optional, tag = "3")]
    pub oracle_history_options: ::core::option::Option<OracleHistoryOptions>,
}
/// QueryOracleVolatilityResponse is the response type for Query/OracleVolatility RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryOracleVolatilityResponse")]
pub struct QueryOracleVolatilityResponse {
    #[prost(string, tag = "1")]
    pub volatility: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub history_metadata: ::core::option::Option<MetadataStatistics>,
    #[prost(message, repeated, tag = "3")]
    pub raw_history: ::prost::alloc::vec::Vec<PriceRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryOracleProvidersInfoRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/OracleProvidersInfo",
    response_type = QueryOracleProvidersInfoResponse
)]
pub struct QueryOracleProvidersInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryOracleProvidersInfoResponse")]
pub struct QueryOracleProvidersInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub providers: ::prost::alloc::vec::Vec<ProviderInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryOracleProviderPricesRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/OracleProviderPrices",
    response_type = QueryOracleProviderPricesResponse
)]
pub struct QueryOracleProviderPricesRequest {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/injective.oracle.v1beta1.QueryOracleProviderPricesResponse"
)]
pub struct QueryOracleProviderPricesResponse {
    #[prost(message, repeated, tag = "1")]
    pub provider_state: ::prost::alloc::vec::Vec<ProviderState>,
}
/// QueryOraclePriceRequest is the request type for the Query/OraclePrice RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryOraclePriceRequest")]
#[proto_query(
    path = "/injective.oracle.v1beta1.Query/OraclePrice",
    response_type = QueryOraclePriceResponse
)]
pub struct QueryOraclePriceRequest {
    #[prost(enumeration = "OracleType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub oracle_type: i32,
    #[prost(string, tag = "2")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.PricePairState")]
pub struct PricePairState {
    #[prost(string, tag = "1")]
    pub pair_price: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub base_cumulative_price: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub quote_cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub base_timestamp: i64,
    #[prost(int64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub quote_timestamp: i64,
}
/// QueryOraclePriceResponse is the response type for the Query/OraclePrice RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.QueryOraclePriceResponse")]
pub struct QueryOraclePriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price_pair_state: ::core::option::Option<PricePairState>,
}
/// MsgRelayProviderPrice defines a SDK message for setting a price through the provider oracle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayProviderPrices")]
pub struct MsgRelayProviderPrices {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub prices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayProviderPricesResponse")]
pub struct MsgRelayProviderPricesResponse {}
/// MsgRelayPriceFeedPrice defines a SDK message for setting a price through the pricefeed oracle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayPriceFeedPrice")]
pub struct MsgRelayPriceFeedPrice {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub base: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub quote: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// price defines the price of the oracle base and quote
    #[prost(string, repeated, tag = "4")]
    pub price: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayPriceFeedPriceResponse")]
pub struct MsgRelayPriceFeedPriceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayBandRates")]
pub struct MsgRelayBandRates {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, repeated, tag = "3")]
    pub rates: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "4")]
    pub resolve_times: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "5")]
    pub request_i_ds: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayBandRatesResponse")]
pub struct MsgRelayBandRatesResponse {}
/// MsgRelayCoinbaseMessages defines a SDK message for relaying price messages from Coinbase API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayCoinbaseMessages")]
pub struct MsgRelayCoinbaseMessages {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayCoinbaseMessagesResponse")]
pub struct MsgRelayCoinbaseMessagesResponse {}
/// MsgRequestBandIBCRates defines a SDK message for requesting data from BandChain using IBC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRequestBandIBCRates")]
pub struct MsgRequestBandIbcRates {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "requestID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub request_id: u64,
}
/// MsgRequestDataResponse defines the Msg/RequestBandIBCRates response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRequestBandIBCRatesResponse")]
pub struct MsgRequestBandIbcRatesResponse {}
/// MsgRelayPythPrices defines a SDK message for updating Pyth prices
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayPythPrices")]
pub struct MsgRelayPythPrices {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub price_attestations: ::prost::alloc::vec::Vec<PriceAttestation>,
}
/// MsgRelayPythPricesResponse defines the Msg/RelayPythPrices response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/injective.oracle.v1beta1.MsgRelayPythPricesResponse")]
pub struct MsgRelayPythPricesResponse {}
pub struct OracleQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> OracleQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn band_relayers(
        &self,
    ) -> Result<QueryBandRelayersResponse, cosmwasm_std::StdError> {
        QueryBandRelayersRequest {}.query(self.querier)
    }
    pub fn band_price_states(
        &self,
    ) -> Result<QueryBandPriceStatesResponse, cosmwasm_std::StdError> {
        QueryBandPriceStatesRequest {}.query(self.querier)
    }
    pub fn band_ibc_price_states(
        &self,
    ) -> Result<QueryBandIbcPriceStatesResponse, cosmwasm_std::StdError> {
        QueryBandIbcPriceStatesRequest {}.query(self.querier)
    }
    pub fn price_feed_price_states(
        &self,
    ) -> Result<QueryPriceFeedPriceStatesResponse, cosmwasm_std::StdError> {
        QueryPriceFeedPriceStatesRequest {
        }
            .query(self.querier)
    }
    pub fn coinbase_price_states(
        &self,
    ) -> Result<QueryCoinbasePriceStatesResponse, cosmwasm_std::StdError> {
        QueryCoinbasePriceStatesRequest {}.query(self.querier)
    }
    pub fn pyth_price_states(
        &self,
    ) -> Result<QueryPythPriceStatesResponse, cosmwasm_std::StdError> {
        QueryPythPriceStatesRequest {}.query(self.querier)
    }
    pub fn provider_price_state(
        &self,
        provider: ::prost::alloc::string::String,
        symbol: ::prost::alloc::string::String,
    ) -> Result<QueryProviderPriceStateResponse, cosmwasm_std::StdError> {
        QueryProviderPriceStateRequest {
            provider,
            symbol,
        }
            .query(self.querier)
    }
    pub fn oracle_module_state(
        &self,
    ) -> Result<QueryModuleStateResponse, cosmwasm_std::StdError> {
        QueryModuleStateRequest {}.query(self.querier)
    }
    pub fn historical_price_records(
        &self,
        oracle: i32,
        symbol_id: ::prost::alloc::string::String,
    ) -> Result<QueryHistoricalPriceRecordsResponse, cosmwasm_std::StdError> {
        QueryHistoricalPriceRecordsRequest {
            oracle,
            symbol_id,
        }
            .query(self.querier)
    }
    pub fn oracle_volatility(
        &self,
        base_info: ::core::option::Option<OracleInfo>,
        quote_info: ::core::option::Option<OracleInfo>,
        oracle_history_options: ::core::option::Option<OracleHistoryOptions>,
    ) -> Result<QueryOracleVolatilityResponse, cosmwasm_std::StdError> {
        QueryOracleVolatilityRequest {
            base_info,
            quote_info,
            oracle_history_options,
        }
            .query(self.querier)
    }
    pub fn oracle_providers_info(
        &self,
    ) -> Result<QueryOracleProvidersInfoResponse, cosmwasm_std::StdError> {
        QueryOracleProvidersInfoRequest {}.query(self.querier)
    }
    pub fn oracle_provider_prices(
        &self,
        provider: ::prost::alloc::string::String,
    ) -> Result<QueryOracleProviderPricesResponse, cosmwasm_std::StdError> {
        QueryOracleProviderPricesRequest {
            provider,
        }
            .query(self.querier)
    }
    pub fn oracle_price(
        &self,
        oracle_type: i32,
        base: ::prost::alloc::string::String,
        quote: ::prost::alloc::string::String,
    ) -> Result<QueryOraclePriceResponse, cosmwasm_std::StdError> {
        QueryOraclePriceRequest {
            oracle_type,
            base,
            quote,
        }
            .query(self.querier)
    }
    pub fn pyth_price(
        &self,
        price_id: ::prost::alloc::string::String,
    ) -> Result<QueryPythPriceResponse, cosmwasm_std::StdError> {
        QueryPythPriceRequest { price_id }.query(self.querier)
    }
}
