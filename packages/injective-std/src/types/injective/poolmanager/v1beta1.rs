use osmosis_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SwapAmountInRoute")]
pub struct SwapAmountInRoute {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_out_denom: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.SwapAmountOutRoute")]
pub struct SwapAmountOutRoute {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub token_in_denom: ::prost::alloc::string::String,
}
/// ModuleRouter defines a route encapsulating pool type.
/// It is used as the value of a mapping from pool id to the pool type,
/// allowing the pool manager to know which module to route swaps to given the
/// pool id.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ModuleRoute")]
pub struct ModuleRoute {
    /// pool_type specifies the type of the pool
    #[prost(enumeration = "PoolType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_type: i32,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
/// PoolType is an enumeration of all supported pool types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum PoolType {
    /// Balancer is the standard xy=k curve. Its pool model is defined in x/gamm.
    Balancer = 0,
    /// Stableswap is the Solidly cfmm stable swap curve. Its pool model is defined
    /// in x/gamm.
    Stableswap = 1,
}
impl PoolType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PoolType::Balancer => "Balancer",
            PoolType::Stableswap => "Stableswap",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Balancer" => Some(Self::Balancer),
            "Stableswap" => Some(Self::Stableswap),
            _ => None,
        }
    }
}
/// Params holds parameters for the poolmanager module
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub pool_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the poolmanager module's genesis state.
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.GenesisState")]
pub struct GenesisState {
    /// the next_pool_id
    #[prost(uint64, tag = "1")]
    #[serde(alias = "next_poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_pool_id: u64,
    /// params is the container of poolmanager parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
    /// pool_routes is the container of the mappings from pool id to pool type.
    #[prost(message, repeated, tag = "3")]
    pub pool_routes: ::prost::alloc::vec::Vec<ModuleRoute>,
}
/// ===================== MsgSwapExactAmountIn
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountIn")]
pub struct MsgSwapExactAmountIn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    #[prost(message, optional, tag = "3")]
    pub token_in: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub token_out_min_amount: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountInResponse")]
pub struct MsgSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// ===================== MsgSwapExactAmountOut
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountOut")]
pub struct MsgSwapExactAmountOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "3")]
    pub token_in_max_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountOutResponse")]
pub struct MsgSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// =============================== Params
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ParamsRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/Params",
    response_type = ParamsResponse
)]
pub struct ParamsRequest {}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.ParamsResponse")]
pub struct ParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// =============================== EstimateSwapExactAmountIn
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountInRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
    response_type = EstimateSwapExactAmountInResponse
)]
pub struct EstimateSwapExactAmountInRequest {
    /// TODO: CHANGE THIS TO RESERVED IN A PATCH RELEASE
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountInResponse")]
pub struct EstimateSwapExactAmountInResponse {
    #[prost(string, tag = "1")]
    pub token_out_amount: ::prost::alloc::string::String,
}
/// =============================== EstimateSwapExactAmountOut
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountOutRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
    response_type = EstimateSwapExactAmountOutResponse
)]
pub struct EstimateSwapExactAmountOutRequest {
    /// TODO: CHANGE THIS TO RESERVED IN A PATCH RELEASE
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "poolID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
    #[prost(string, tag = "4")]
    pub token_out: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.EstimateSwapExactAmountOutResponse")]
pub struct EstimateSwapExactAmountOutResponse {
    #[prost(string, tag = "1")]
    pub token_in_amount: ::prost::alloc::string::String,
}
/// =============================== NumPools
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.NumPoolsRequest")]
#[proto_query(
    path = "/osmosis.poolmanager.v1beta1.Query/NumPools",
    response_type = NumPoolsResponse
)]
pub struct NumPoolsRequest {}
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
#[proto_message(type_url = "/osmosis.poolmanager.v1beta1.NumPoolsResponse")]
pub struct NumPoolsResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_pools: u64,
}
pub struct PoolmanagerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PoolmanagerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<ParamsResponse, cosmwasm_std::StdError> {
        ParamsRequest {}.query(self.querier)
    }
    pub fn estimate_swap_exact_amount_in(
        &self,
        sender: ::prost::alloc::string::String,
        pool_id: u64,
        token_in: ::prost::alloc::string::String,
        routes: ::prost::alloc::vec::Vec<SwapAmountInRoute>,
    ) -> Result<EstimateSwapExactAmountInResponse, cosmwasm_std::StdError> {
        EstimateSwapExactAmountInRequest {
            sender,
            pool_id,
            token_in,
            routes,
        }
        .query(self.querier)
    }
    pub fn estimate_swap_exact_amount_out(
        &self,
        sender: ::prost::alloc::string::String,
        pool_id: u64,
        routes: ::prost::alloc::vec::Vec<SwapAmountOutRoute>,
        token_out: ::prost::alloc::string::String,
    ) -> Result<EstimateSwapExactAmountOutResponse, cosmwasm_std::StdError> {
        EstimateSwapExactAmountOutRequest {
            sender,
            pool_id,
            routes,
            token_out,
        }
        .query(self.querier)
    }
    pub fn num_pools(&self) -> Result<NumPoolsResponse, cosmwasm_std::StdError> {
        NumPoolsRequest {}.query(self.querier)
    }
}
