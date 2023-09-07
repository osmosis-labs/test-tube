use crate::runner::Runner;

pub mod bank;
pub mod wasm;

#[cfg(feature = "bank")]
pub use bank::Bank;

#[cfg(feature = "wasm")]
pub use wasm::Wasm;

#[macro_use]
pub mod macros;

pub trait Module<'a, R: Runner<'a>> {
    fn new(runner: &'a R) -> Self;
}
