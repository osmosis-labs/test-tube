use osmosis_std_derive::CosmwasmExt;
/// NOTE: The following type is not implemented due to current limitations of code generator
/// which currently has issue with tendermint_proto.
/// This will be fixed in the upcoming release.
#[allow(dead_code)]
struct HistoricalInfo {}
/// CommissionRates defines the initial commission rates to be used for creating
/// a validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.CommissionRates")]
pub struct CommissionRates {
    /// rate is the commission rate charged to delegators, as a fraction.
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
    /// max_rate defines the maximum commission rate which validator can ever charge, as a fraction.
    #[prost(string, tag = "2")]
    pub max_rate: ::prost::alloc::string::String,
    /// max_change_rate defines the maximum daily increase of the validator commission, as a fraction.
    #[prost(string, tag = "3")]
    pub max_change_rate: ::prost::alloc::string::String,
}
/// Commission defines commission parameters for a given validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.Commission")]
pub struct Commission {
    /// commission_rates defines the initial commission rates to be used for creating a validator.
    #[prost(message, optional, tag = "1")]
    pub commission_rates: ::core::option::Option<CommissionRates>,
    /// update_time is the last time the commission rate was changed.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// Description defines a validator description.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.Description")]
pub struct Description {
    /// moniker defines a human-readable name for the validator.
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    /// website defines an optional website link.
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    /// security_contact defines an optional email for security contact.
    #[prost(string, tag = "4")]
    pub security_contact: ::prost::alloc::string::String,
    /// details define other optional details.
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
}
/// Validator defines a validator, together with the total amount of the
/// Validator's bond shares and their exchange rate to coins. Slashing results in
/// a decrease in the exchange rate, allowing correct calculation of future
/// undelegations without iterating over delegators. When coins are delegated to
/// this validator, the validator is credited with a delegation whose number of
/// bond shares is based on the amount of coins delegated divided by the current
/// exchange rate. Voting power can be calculated as total bonded shares
/// multiplied by exchange rate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.Validator")]
pub struct Validator {
    /// operator_address defines the address of the validator's operator; bech encoded in JSON.
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    /// consensus_pubkey is the consensus public key of the validator, as a Protobuf Any.
    #[prost(message, optional, tag = "2")]
    pub consensus_pubkey: ::core::option::Option<crate::shim::Any>,
    /// jailed defined whether the validator has been jailed from bonded status or not.
    #[prost(bool, tag = "3")]
    pub jailed: bool,
    /// status is the validator status (bonded/unbonding/unbonded).
    #[prost(enumeration = "BondStatus", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// tokens define the delegated tokens (incl. self-delegation).
    #[prost(string, tag = "5")]
    pub tokens: ::prost::alloc::string::String,
    /// delegator_shares defines total shares issued to a validator's delegators.
    #[prost(string, tag = "6")]
    pub delegator_shares: ::prost::alloc::string::String,
    /// description defines the description terms for the validator.
    #[prost(message, optional, tag = "7")]
    pub description: ::core::option::Option<Description>,
    /// unbonding_height defines, if unbonding, the height at which this validator has begun unbonding.
    #[prost(int64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_height: i64,
    /// unbonding_time defines, if unbonding, the min time for the validator to complete unbonding.
    #[prost(message, optional, tag = "9")]
    pub unbonding_time: ::core::option::Option<crate::shim::Timestamp>,
    /// commission defines the commission parameters.
    #[prost(message, optional, tag = "10")]
    pub commission: ::core::option::Option<Commission>,
    /// min_self_delegation is the validator's self declared minimum self delegation.
    ///
    /// Since: cosmos-sdk 0.46
    #[prost(string, tag = "11")]
    pub min_self_delegation: ::prost::alloc::string::String,
    /// strictly positive if this validator's unbonding has been stopped by external modules
    #[prost(int64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_on_hold_ref_count: i64,
    /// list of unbonding ids, each uniquely identifing an unbonding of this validator
    #[prost(uint64, repeated, tag = "13")]
    #[serde(alias = "unbondingIDs")]
    pub unbonding_ids: ::prost::alloc::vec::Vec<u64>,
}
/// ValAddresses defines a repeated set of validator addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.ValAddresses")]
pub struct ValAddresses {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DVPair is struct that just has a delegator-validator pair with no other data.
/// It is intended to be used as a marshalable pointer. For example, a DVPair can
/// be used to construct the key to getting an UnbondingDelegation from state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVPair")]
pub struct DvPair {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// DVPairs defines an array of DVPair objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVPairs")]
pub struct DvPairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<DvPair>,
}
/// DVVTriplet is struct that just has a delegator-validator-validator triplet
/// with no other data. It is intended to be used as a marshalable pointer. For
/// example, a DVVTriplet can be used to construct the key to getting a
/// Redelegation from state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVVTriplet")]
pub struct DvvTriplet {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
}
/// DVVTriplets defines an array of DVVTriplet objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVVTriplets")]
pub struct DvvTriplets {
    #[prost(message, repeated, tag = "1")]
    pub triplets: ::prost::alloc::vec::Vec<DvvTriplet>,
}
/// Delegation represents the bond with tokens held by an account. It is
/// owned by one delegator, and is associated with the voting power of one
/// validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.Delegation")]
pub struct Delegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the bech32-encoded address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// shares define the delegation shares received.
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
}
/// UnbondingDelegation stores all of a single delegator's unbonding bonds
/// for a single validator in an time-ordered list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.UnbondingDelegation")]
pub struct UnbondingDelegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the bech32-encoded address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// entries are the unbonding delegation entries.
    ///
    /// unbonding delegation entries
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<UnbondingDelegationEntry>,
}
/// UnbondingDelegationEntry defines an unbonding object with relevant metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.UnbondingDelegationEntry")]
pub struct UnbondingDelegationEntry {
    /// creation_height is the height which the unbonding took place.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: i64,
    /// completion_time is the unix time for unbonding completion.
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
    /// initial_balance defines the tokens initially scheduled to receive at completion.
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    /// balance defines the tokens to receive at completion.
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
    /// Incrementing id that uniquely identifies this entry
    #[prost(uint64, tag = "5")]
    #[serde(alias = "unbondingID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_id: u64,
    /// Strictly positive if this entry's unbonding has been stopped by external modules
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_on_hold_ref_count: i64,
}
/// RedelegationEntry defines a redelegation object with relevant metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.RedelegationEntry")]
pub struct RedelegationEntry {
    /// creation_height  defines the height which the redelegation took place.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: i64,
    /// completion_time defines the unix time for redelegation completion.
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
    /// initial_balance defines the initial balance when redelegation started.
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    /// shares_dst is the amount of destination-validator shares created by redelegation.
    #[prost(string, tag = "4")]
    pub shares_dst: ::prost::alloc::string::String,
    /// Incrementing id that uniquely identifies this entry
    #[prost(uint64, tag = "5")]
    #[serde(alias = "unbondingID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_id: u64,
    /// Strictly positive if this entry's unbonding has been stopped by external modules
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_on_hold_ref_count: i64,
}
/// Redelegation contains the list of a particular delegator's redelegating bonds
/// from a particular source validator to a particular destination validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.Redelegation")]
pub struct Redelegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_src_address is the validator redelegation source operator address.
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    /// validator_dst_address is the validator redelegation destination operator address.
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
    /// entries are the redelegation entries.
    ///
    /// redelegation entries
    #[prost(message, repeated, tag = "4")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntry>,
}
/// Params defines the parameters for the x/staking module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.Params")]
pub struct Params {
    /// unbonding_time is the time duration of unbonding.
    #[prost(message, optional, tag = "1")]
    pub unbonding_time: ::core::option::Option<crate::shim::Duration>,
    /// max_validators is the maximum number of validators.
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_validators: u32,
    /// max_entries is the max entries for either unbonding delegation or redelegation (per pair/trio).
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_entries: u32,
    /// historical_entries is the number of historical entries to persist.
    #[prost(uint32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub historical_entries: u32,
    /// bond_denom defines the bondable coin denomination.
    #[prost(string, tag = "5")]
    pub bond_denom: ::prost::alloc::string::String,
    /// min_commission_rate is the chain-wide minimum commission rate that a validator can charge their delegators
    #[prost(string, tag = "6")]
    pub min_commission_rate: ::prost::alloc::string::String,
}
/// DelegationResponse is equivalent to Delegation except that it contains a
/// balance in addition to shares which is more suitable for client responses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.DelegationResponse")]
pub struct DelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub delegation: ::core::option::Option<Delegation>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// RedelegationEntryResponse is equivalent to a RedelegationEntry except that it
/// contains a balance in addition to shares which is more suitable for client
/// responses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.RedelegationEntryResponse")]
pub struct RedelegationEntryResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation_entry: ::core::option::Option<RedelegationEntry>,
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
}
/// RedelegationResponse is equivalent to a Redelegation except that its entries
/// contain a balance in addition to shares which is more suitable for client
/// responses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.RedelegationResponse")]
pub struct RedelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation: ::core::option::Option<Redelegation>,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntryResponse>,
}
/// Pool is used for tracking bonded and not-bonded token supply of the bond
/// denomination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.Pool")]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub not_bonded_tokens: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bonded_tokens: ::prost::alloc::string::String,
}
/// NOTE: The following type is not implemented due to current limitations of code generator
/// which currently has issue with tendermint_proto.
/// This will be fixed in the upcoming release.
#[allow(dead_code)]
struct ValidatorUpdates {}
/// BondStatus is the status of a validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum BondStatus {
    /// UNSPECIFIED defines an invalid validator status.
    Unspecified = 0,
    /// UNBONDED defines a validator that is not bonded.
    Unbonded = 1,
    /// UNBONDING defines a validator that is unbonding.
    Unbonding = 2,
    /// BONDED defines a validator that is bonded.
    Bonded = 3,
}
impl BondStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BondStatus::Unspecified => "BOND_STATUS_UNSPECIFIED",
            BondStatus::Unbonded => "BOND_STATUS_UNBONDED",
            BondStatus::Unbonding => "BOND_STATUS_UNBONDING",
            BondStatus::Bonded => "BOND_STATUS_BONDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BOND_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BOND_STATUS_UNBONDED" => Some(Self::Unbonded),
            "BOND_STATUS_UNBONDING" => Some(Self::Unbonding),
            "BOND_STATUS_BONDED" => Some(Self::Bonded),
            _ => None,
        }
    }
}
/// Infraction indicates the infraction a validator commited.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Infraction {
    /// UNSPECIFIED defines an empty infraction.
    Unspecified = 0,
    /// DOUBLE_SIGN defines a validator that double-signs a block.
    DoubleSign = 1,
    /// DOWNTIME defines a validator that missed signing too many blocks.
    Downtime = 2,
}
impl Infraction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Infraction::Unspecified => "INFRACTION_UNSPECIFIED",
            Infraction::DoubleSign => "INFRACTION_DOUBLE_SIGN",
            Infraction::Downtime => "INFRACTION_DOWNTIME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INFRACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "INFRACTION_DOUBLE_SIGN" => Some(Self::DoubleSign),
            "INFRACTION_DOWNTIME" => Some(Self::Downtime),
            _ => None,
        }
    }
}
/// GenesisState defines the staking module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of related to deposit.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// last_total_power tracks the total amounts of bonded tokens recorded during
    /// the previous end block.
    #[prost(bytes = "vec", tag = "2")]
    pub last_total_power: ::prost::alloc::vec::Vec<u8>,
    /// last_validator_powers is a special index that provides a historical list
    /// of the last-block's bonded validators.
    #[prost(message, repeated, tag = "3")]
    pub last_validator_powers: ::prost::alloc::vec::Vec<LastValidatorPower>,
    /// delegations defines the validator set at genesis.
    #[prost(message, repeated, tag = "4")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// delegations defines the delegations active at genesis.
    #[prost(message, repeated, tag = "5")]
    pub delegations: ::prost::alloc::vec::Vec<Delegation>,
    /// unbonding_delegations defines the unbonding delegations active at genesis.
    #[prost(message, repeated, tag = "6")]
    pub unbonding_delegations: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// redelegations defines the redelegations active at genesis.
    #[prost(message, repeated, tag = "7")]
    pub redelegations: ::prost::alloc::vec::Vec<Redelegation>,
    #[prost(bool, tag = "8")]
    pub exported: bool,
}
/// LastValidatorPower required for validator set update logic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.LastValidatorPower")]
pub struct LastValidatorPower {
    /// address is the address of the validator.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// power defines the power of the validator.
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
}
/// MsgCreateValidator defines a SDK message for creating a new validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgCreateValidator")]
pub struct MsgCreateValidator {
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<Description>,
    #[prost(message, optional, tag = "2")]
    pub commission: ::core::option::Option<CommissionRates>,
    #[prost(string, tag = "3")]
    pub min_self_delegation: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub pubkey: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "7")]
    pub value: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgCreateValidatorResponse defines the Msg/CreateValidator response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgCreateValidatorResponse")]
pub struct MsgCreateValidatorResponse {}
/// MsgEditValidator defines a SDK message for editing an existing validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgEditValidator")]
pub struct MsgEditValidator {
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<Description>,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// We pass a reference to the new commission rate and min self delegation as
    /// it's not mandatory to update. If not updated, the deserialized rate will be
    /// zero with no way to distinguish if an update was intended.
    /// REF: #2373
    #[prost(string, tag = "3")]
    pub commission_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub min_self_delegation: ::prost::alloc::string::String,
}
/// MsgEditValidatorResponse defines the Msg/EditValidator response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgEditValidatorResponse")]
pub struct MsgEditValidatorResponse {}
/// MsgDelegate defines a SDK message for performing a delegation of coins
/// from a delegator to a validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgDelegate")]
pub struct MsgDelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgDelegateResponse defines the Msg/Delegate response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgDelegateResponse")]
pub struct MsgDelegateResponse {}
/// MsgBeginRedelegate defines a SDK message for performing a redelegation
/// of coins from a delegator and source validator to a destination validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgBeginRedelegate")]
pub struct MsgBeginRedelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgBeginRedelegateResponse defines the Msg/BeginRedelegate response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgBeginRedelegateResponse")]
pub struct MsgBeginRedelegateResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgUndelegate defines a SDK message for performing an undelegation from a
/// delegate and a validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUndelegate")]
pub struct MsgUndelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgUndelegateResponse defines the Msg/Undelegate response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUndelegateResponse")]
pub struct MsgUndelegateResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgCancelUnbondingDelegation defines the SDK message for performing a cancel unbonding delegation for delegator
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgCancelUnbondingDelegation")]
pub struct MsgCancelUnbondingDelegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// amount is always less than or equal to unbonding delegation entry balance
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
    /// creation_height is the height which the unbonding took place.
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: i64,
}
/// MsgCancelUnbondingDelegationResponse
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/cosmos.staking.v1beta1.MsgCancelUnbondingDelegationResponse"
)]
pub struct MsgCancelUnbondingDelegationResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/staking parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
