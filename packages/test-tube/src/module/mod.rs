use crate::runner::Runner;

pub mod bank;
pub mod wasm;

pub use bank::Bank;
pub use wasm::Wasm;

#[macro_use]
pub mod macros;

pub trait Module<'a, R: Runner<'a>> {
    fn new(runner: &'a R) -> Self;
}
