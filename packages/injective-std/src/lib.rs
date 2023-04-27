#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

mod serde;
pub mod shim;
pub mod types;

// pub use shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};
