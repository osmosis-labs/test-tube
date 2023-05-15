use osmosis_std_derive::CosmwasmExt;
/// Module is the config object of the authz module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.authz.module.v1.Module")]
pub struct Module {}
