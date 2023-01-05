use crate::runner::Runner;

#[macro_use]
pub mod macros;

pub trait Module<'a, R: Runner<'a>> {
    fn new(runner: &'a R) -> Self;
}
