use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct Exchange<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Exchange<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Exchange<'a, R>
where
    R: Runner<'a>,
{
    // fn_execute! {
    //     pub create_denom: MsgCreateDenom ["/injective.tokenfactory.v1beta1.MsgCreateDenom"] => MsgCreateDenomResponse
    // }

    // fn_query! {
    //     pub query_denoms_from_creator ["/injective.tokenfactory.v1beta1.Query/DenomsFromCreator"]: QueryDenomsFromCreatorRequest => QueryDenomsFromCreatorResponse
    // }
}
