use tx_wasm_sdk::types::cosmos::staking::v1beta1::{
    MsgCreateValidator, MsgCreateValidatorResponse, MsgDelegate, MsgDelegateResponse,
    MsgUndelegate, MsgUndelegateResponse, QueryDelegationRequest, QueryDelegationResponse,
    QueryDelegatorDelegationsRequest, QueryDelegatorDelegationsResponse,
    QueryDelegatorUnbondingDelegationsRequest, QueryDelegatorUnbondingDelegationsResponse,
    QueryParamsRequest, QueryParamsResponse, QueryUnbondingDelegationRequest,
    QueryUnbondingDelegationResponse, QueryValidatorsRequest, QueryValidatorsResponse,
};
use test_tube_tx::module::Module;
use test_tube_tx::runner::Runner;
use test_tube_tx::{fn_execute, fn_query};

pub struct Staking<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Staking<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Staking<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub delegate: MsgDelegate["/cosmos.staking.v1beta1.MsgDelegate"] => MsgDelegateResponse
    }

    fn_execute! {
        pub undelegate: MsgUndelegate["/cosmos.staking.v1beta1.MsgUndelegate"] => MsgUndelegateResponse
    }

    fn_execute! {
        pub create_validator: MsgCreateValidator["/cosmos.staking.v1beta1.MsgCreateValidator"] => MsgCreateValidatorResponse
    }

    fn_query! {
        pub query_params ["/cosmos.staking.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_validators ["/cosmos.staking.v1beta1.Query/Validators"]: QueryValidatorsRequest => QueryValidatorsResponse
    }

    fn_query! {
        pub query_delegation ["/cosmos.staking.v1beta1.Query/Delegation"]: QueryDelegationRequest => QueryDelegationResponse
    }

    fn_query! {
        pub query_unbonding_delegation ["/cosmos.staking.v1beta1.Query/UnbondingDelegation"]: QueryUnbondingDelegationRequest => QueryUnbondingDelegationResponse
    }

    fn_query! {
        pub query_delegations ["/cosmos.staking.v1beta1.Query/DelegatorDelegations"]: QueryDelegatorDelegationsRequest => QueryDelegatorDelegationsResponse
    }

    fn_query! {
        pub query_unbonding_delegations ["/cosmos.staking.v1beta1.Query/DelegatorUnbondingDelegations"]: QueryDelegatorUnbondingDelegationsRequest => QueryDelegatorUnbondingDelegationsResponse
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::app::FEE_DENOM;
    use crate::{TXTestApp, Staking};
    use bech32::{Bech32, Hrp};
    use tx_wasm_sdk::shim::Any;
    use tx_wasm_sdk::types::cosmos::base::v1beta1::Coin as BaseCoin;
    use tx_wasm_sdk::types::cosmos::staking::v1beta1::{
        BondStatus, CommissionRates, Description, MsgCreateValidator, QueryValidatorsRequest,
    };
    use cosmrs::proto;
    use cosmrs::tx::MessageExt;
    use cosmwasm_std::Coin;
    use ring::{
        rand,
        signature::{self, KeyPair},
    };
    use test_tube_tx::{Account, Module};

    fn get_validator_address(address: &str) -> String {
        let (_, data) = bech32::decode(address).expect("failed to decode");
        let val_hrp = Hrp::parse("corevaloper").unwrap();
        bech32::encode::<Bech32>(val_hrp, &data).expect("failed to encode string")
    }

    fn get_ed25519_pubkey() -> Any {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let public_key = key_pair.public_key();
        Any {
            type_url: "/cosmos.crypto.ed25519.PubKey".to_string(),
            value: proto::cosmos::crypto::ed25519::PubKey {
                key: public_key.as_ref().to_vec(),
            }
            .to_bytes()
            .unwrap(),
        }
    }

    #[test]
    fn staking_integration() {
        let app = TXTestApp::new();
        let signer = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();

        let staking = Staking::new(&app);
        // Check that we currently have 1 active validator
        let response = staking
            .query_validators(&QueryValidatorsRequest {
                status: String::from(BondStatus::Bonded.as_str_name()),
                pagination: None,
            })
            .unwrap();

        assert_eq!(response.validators.len(), 1);

        // Let's create a second validator and check that it's active
        staking
            .create_validator(
                #[allow(deprecated)]
                MsgCreateValidator {
                    description: Some(Description {
                        moniker: "moniker".to_string(),
                        identity: "".to_string(),
                        website: "".to_string(),
                        security_contact: "".to_string(),
                        details: "".to_string(),
                    }),
                    commission: Some(CommissionRates {
                        rate: "1".to_string(),
                        max_rate: "5".to_string(),
                        max_change_rate: "1".to_string(),
                    }),
                    min_self_delegation: "20000000000".to_string(),
                    delegator_address: signer.address().to_string(),
                    validator_address: get_validator_address(signer.address().to_string().as_str()),
                    pubkey: Some(get_ed25519_pubkey()),
                    value: Some(BaseCoin {
                        denom: FEE_DENOM.to_string(),
                        amount: "20000000000".to_string(),
                    }),
                },
                &signer,
            )
            .unwrap();

        let response = staking
            .query_validators(&QueryValidatorsRequest {
                status: String::from(BondStatus::Bonded.as_str_name()),
                pagination: None,
            })
            .unwrap();

        assert_eq!(response.validators.len(), 2);
    }
}
