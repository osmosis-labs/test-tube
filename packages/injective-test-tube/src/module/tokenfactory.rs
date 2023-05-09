use injective_std::types::injective::tokenfactory::v1beta1::{
    MsgBurn, MsgBurnResponse, MsgChangeAdmin, MsgChangeAdminResponse, MsgCreateDenom,
    MsgCreateDenomResponse, MsgMint, MsgMintResponse, MsgSetDenomMetadata,
    MsgSetDenomMetadataResponse, QueryDenomAuthorityMetadataRequest,
    QueryDenomAuthorityMetadataResponse, QueryDenomsFromCreatorRequest,
    QueryDenomsFromCreatorResponse, QueryParamsRequest, QueryParamsResponse,
};

use test_tube::module::Module;
use test_tube::runner::Runner;
use test_tube::{fn_execute, fn_query};

pub struct TokenFactory<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for TokenFactory<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> TokenFactory<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_denom: MsgCreateDenom ["/injective.tokenfactory.v1beta1.MsgCreateDenom"] => MsgCreateDenomResponse
    }

    // NOTE: mint and burn are not supported until we create rust types for injective with the relevant proto as they
    // diverge from the Osmosis types, and the msg.rs in injective-cosmwasm does not suffice
    fn_execute! {
        pub mint: MsgMint ["/injective.tokenfactory.v1beta1.MsgMint"]  => MsgMintResponse
    }

    fn_execute! {
        pub burn: MsgBurn ["/injective.tokenfactory.v1beta1.MsgBurn"] => MsgBurnResponse
    }

    fn_execute! {
        pub change_admin: MsgChangeAdmin ["/injective.tokenfactory.v1beta1.MsgChangeAdmin"]  => MsgChangeAdminResponse
    }

    fn_execute! {
        pub set_denom_metadata: MsgSetDenomMetadata  ["/injective.tokenfactory.v1beta1.MsgSetDenomMetadata"]  => MsgSetDenomMetadataResponse
    }

    fn_query! {
        pub query_params ["/injective.tokenfactory.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_denom_authority_metadata ["/injective.tokenfactory.v1beta1.Query/DenomAuthorityMetadata"]: QueryDenomAuthorityMetadataRequest => QueryDenomAuthorityMetadataResponse
    }

    fn_query! {
        pub query_denoms_from_creator ["/injective.tokenfactory.v1beta1.Query/DenomsFromCreator"]: QueryDenomsFromCreatorRequest => QueryDenomsFromCreatorResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use injective_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
    use injective_std::types::injective::tokenfactory::v1beta1::{
        MsgBurn, MsgCreateDenom, MsgMint, QueryDenomsFromCreatorRequest,
    };

    use crate::{Account, Bank, InjectiveTestApp, TokenFactory};
    use test_tube::Module;

    #[test]
    fn tokenfactory_integration() {
        let app = InjectiveTestApp::new();
        let signer = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, "inj")])
            .unwrap();
        let tokenfactory = TokenFactory::new(&app);
        let bank = Bank::new(&app);

        // create denom
        let subdenom = "udenom";
        let denom = tokenfactory
            .create_denom(
                MsgCreateDenom {
                    sender: signer.address(),
                    subdenom: subdenom.to_owned(),
                },
                &signer,
            )
            .unwrap()
            .data
            .new_token_denom;

        assert_eq!(format!("factory/{}/{}", signer.address(), subdenom), denom);

        // denom from creator
        let denoms = tokenfactory
            .query_denoms_from_creator(&QueryDenomsFromCreatorRequest {
                creator: signer.address(),
            })
            .unwrap()
            .denoms;

        assert_eq!(denoms, [denom.clone()]);

        // TODO mint new denom
        // mint
        let coin: injective_std::types::cosmos::base::v1beta1::Coin =
            Coin::new(1000000000, denom.clone()).into();
        tokenfactory
            .mint(
                MsgMint {
                    sender: signer.address(),
                    amount: Some(coin.clone()),
                },
                &signer,
            )
            .unwrap();

        let balance = bank
            .query_balance(&QueryBalanceRequest {
                address: signer.address(),
                denom: denom.clone(),
            })
            .unwrap()
            .balance
            .unwrap();

        assert_eq!(coin.amount, balance.amount);
        assert_eq!(coin.denom, balance.denom);

        // burn
        tokenfactory
            .burn(
                MsgBurn {
                    sender: signer.address(),
                    amount: Some(coin.clone()),
                },
                &signer,
            )
            .unwrap();

        let balance = bank
            .query_balance(&QueryBalanceRequest {
                address: signer.address(),
                denom: denom.clone(),
            })
            .unwrap()
            .balance
            .unwrap();

        assert_eq!("0", balance.amount);
        assert_eq!(coin.denom, balance.denom);
    }
}
