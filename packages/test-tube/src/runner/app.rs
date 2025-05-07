use std::ffi::CString;

use base64::Engine as _;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::proto::tendermint::abci::ResponseFinalizeBlock;
use cosmrs::tx::{Fee, SignerInfo};
use cosmrs::{tx, Any};
use cosmwasm_std::{Coin, Timestamp};
use prost::Message;

use crate::account::{Account, FeeSetting, SigningAccount};
use crate::bindings::{
    AccountNumber, AccountSequence, CleanUp, Commit, FinalizeBlock, GetBlockHeight, GetBlockTime,
    GetParamSet, GetValidatorAddress, GetValidatorPrivateKey, IncreaseTime, InitAccount,
    InitTestEnv, Query, SetParamSet, Simulate,
};
use crate::redefine_as_go_string;
use crate::runner::error::{DecodeError, EncodeError, RunnerError};
use crate::runner::result::RawResult;
use crate::runner::result::{RunnerExecuteResult, RunnerResult};
use crate::runner::Runner;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

pub const OSMOSIS_MIN_GAS_PRICE: u128 = 2_500;

#[derive(Debug, PartialEq)]
pub struct BaseApp {
    id: u64,
    fee_denom: String,
    chain_id: String,
    address_prefix: String,
    default_gas_adjustment: f64,
}

impl BaseApp {
    pub fn new(
        fee_denom: &str,
        chain_id: &str,
        address_prefix: &str,
        default_gas_adjustment: f64,
    ) -> Self {
        let id = unsafe { InitTestEnv() };
        BaseApp {
            id,
            fee_denom: fee_denom.to_string(),
            chain_id: chain_id.to_string(),
            address_prefix: address_prefix.to_string(),
            default_gas_adjustment,
        }
    }

    /// Increase the time of the blockchain by the given number of seconds.
    pub fn increase_time(&self, seconds: u64) {
        unsafe {
            IncreaseTime(self.id, seconds.try_into().unwrap());
        }
    }

    /// Get the first validator address
    pub fn get_first_validator_address(&self) -> RunnerResult<String> {
        let addr = unsafe {
            let addr = GetValidatorAddress(self.id, 0);
            CString::from_raw(addr)
        }
        .to_str()
        .map_err(DecodeError::Utf8Error)?
        .to_string();

        Ok(addr)
    }

    /// Get the first validator signing account
    pub fn get_first_validator_signing_account(&self) -> RunnerResult<SigningAccount> {
        let base64_priv = unsafe {
            let val_priv = GetValidatorPrivateKey(self.id, 0);
            CString::from_raw(val_priv)
        }
        .to_str()
        .map_err(DecodeError::Utf8Error)?
        .to_string();

        let secp256k1_priv = BASE64_STANDARD
            .decode(base64_priv)
            .map_err(DecodeError::Base64DecodeError)?;
        let signging_key = SigningKey::from_slice(&secp256k1_priv).map_err(|e| {
            let msg = e.to_string();
            DecodeError::SigningKeyDecodeError { msg }
        })?;

        Ok(SigningAccount::new(
            self.address_prefix.clone(),
            signging_key,
            FeeSetting::Auto {
                gas_price: Coin::new(OSMOSIS_MIN_GAS_PRICE, self.fee_denom.clone()),
                gas_adjustment: self.default_gas_adjustment,
            },
        ))
    }

    pub fn get_chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn get_account_sequence(&self, address: &str) -> u64 {
        redefine_as_go_string!(address);
        unsafe { AccountSequence(self.id, address) }
    }

    pub fn get_account_number(&self, address: &str) -> u64 {
        redefine_as_go_string!(address);
        unsafe { AccountNumber(self.id, address) }
    }

    /// Get the current block time
    pub fn get_block_timestamp(&self) -> Timestamp {
        let result = unsafe { GetBlockTime(self.id) };

        Timestamp::from_nanos(result as u64)
    }

    /// Get the current block time
    pub fn get_block_time_nanos(&self) -> i64 {
        unsafe { GetBlockTime(self.id) }
    }

    /// Get the current block height
    pub fn get_block_height(&self) -> i64 {
        unsafe { GetBlockHeight(self.id) }
    }
    /// Initialize account with initial balance of any coins.
    /// This function mints new coins and send to newly created account
    pub fn init_account(&self, coins: &[Coin]) -> RunnerResult<SigningAccount> {
        let mut coins = coins.to_vec();

        // invalid coins if denom are unsorted
        coins.sort_by(|a, b| a.denom.cmp(&b.denom));

        let coins_json = serde_json::to_string(&coins).map_err(EncodeError::JsonEncodeError)?;
        redefine_as_go_string!(coins_json);

        let base64_priv = unsafe {
            let addr = InitAccount(self.id, coins_json);
            CString::from_raw(addr)
        }
        .to_str()
        .map_err(DecodeError::Utf8Error)?
        .to_string();

        let secp256k1_priv = BASE64_STANDARD
            .decode(base64_priv)
            .map_err(DecodeError::Base64DecodeError)?;
        let signging_key = SigningKey::from_slice(&secp256k1_priv).map_err(|e| {
            let msg = e.to_string();
            DecodeError::SigningKeyDecodeError { msg }
        })?;

        Ok(SigningAccount::new(
            self.address_prefix.clone(),
            signging_key,
            FeeSetting::Auto {
                gas_price: Coin::new(OSMOSIS_MIN_GAS_PRICE, self.fee_denom.clone()),
                gas_adjustment: self.default_gas_adjustment,
            },
        ))
    }
    /// Convenience function to create multiple accounts with the same
    /// Initial coins balance
    pub fn init_accounts(&self, coins: &[Coin], count: u64) -> RunnerResult<Vec<SigningAccount>> {
        (0..count).map(|_| self.init_account(coins)).collect()
    }

    fn create_signed_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
        fee: Fee,
    ) -> RunnerResult<Vec<u8>>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        let tx_body = tx::Body::new(msgs, "", 0u32);
        let addr = signer.address();

        let seq = self.get_account_sequence(&addr);
        let account_number = self.get_account_number(&addr);

        let signer_info = SignerInfo::single_direct(Some(signer.public_key()), seq);

        let chain_id = self
            .chain_id
            .parse()
            .expect("parse const str of chain id should never fail");

        let auth_info = signer_info.auth_info(fee);
        let sign_doc = tx::SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)
            .map_err(EncodeError::from_proto_error_report)?;

        let tx_raw = sign_doc
            .sign(signer.signing_key())
            .map_err(EncodeError::from_proto_error_report)?;

        tx_raw
            .to_bytes()
            .map_err(EncodeError::from_proto_error_report)
            .map_err(RunnerError::EncodeError)
    }

    pub fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> RunnerResult<cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        self.simulate_tx_bytes(&self.create_signed_tx(
            msgs,
            signer,
            self.default_simulation_fee(),
        )?)
    }

    pub fn default_simulation_fee(&self) -> Fee {
        Fee::from_amount_and_gas(
            cosmrs::Coin {
                denom: self.fee_denom.parse().unwrap(),
                amount: OSMOSIS_MIN_GAS_PRICE,
            },
            0u64,
        )
    }

    pub fn simulate_tx_bytes(
        &self,
        tx_bytes: &[u8],
    ) -> RunnerResult<cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo> {
        let base64_tx_bytes = BASE64_STANDARD.encode(tx_bytes);
        redefine_as_go_string!(base64_tx_bytes);

        unsafe {
            let res = Simulate(self.id, base64_tx_bytes);
            let res = RawResult::from_non_null_ptr(res).into_result()?;

            cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo::decode(res.as_slice())
                .map_err(DecodeError::ProtoDecodeError)
                .map_err(RunnerError::DecodeError)
        }
    }

    pub fn calculate_fee(&self, tx_bytes: &[u8], fee_payer: &SigningAccount) -> RunnerResult<Fee> {
        match &fee_payer.fee_setting() {
            FeeSetting::Auto {
                gas_price,
                gas_adjustment,
            } => {
                let gas_info = self.simulate_tx_bytes(tx_bytes)?;
                let gas_limit = ((gas_info.gas_used as f64) * (gas_adjustment)).ceil() as u64;

                let amount = cosmrs::Coin {
                    denom: self.fee_denom.parse().unwrap(),
                    amount: (((gas_limit as f64) * (gas_price.amount.u128() as f64)).ceil() as u64)
                        .into(),
                };

                Ok(Fee::from_amount_and_gas(amount, gas_limit))
            }
            FeeSetting::Custom { amount, gas_limit } => Ok(Fee::from_amount_and_gas(
                cosmrs::Coin {
                    denom: amount.denom.parse().unwrap(),
                    amount: amount.amount.to_string().parse().unwrap(),
                },
                *gas_limit,
            )),
        }
    }

    /// Directly trigger sudo entrypoint on a given contract.
    ///
    /// # Caution
    ///
    /// This function bypasses standard state changes and processes within the chain logic that might occur in normal situation,
    /// It is primarily intended for internal system logic where necessary state adjustments are handled.
    /// Use only with full understanding of the function's impact on system state and testing validity.
    /// Improper use may result in misleading test outcomes, including false positives or negatives.
    #[cfg(feature = "wasm-sudo")]
    pub fn wasm_sudo<M>(&self, contract_address: &str, sudo_msg: M) -> RunnerResult<Vec<u8>>
    where
        M: serde::Serialize,
    {
        let msg_string = serde_json::to_string(&sudo_msg).map_err(EncodeError::JsonEncodeError)?;
        redefine_as_go_string!(msg_string);
        redefine_as_go_string!(contract_address);

        unsafe {
            let res = crate::bindings::WasmSudo(self.id, contract_address, msg_string);
            RawResult::from_non_null_ptr(res).into_result()
        }
    }

    /// Set parameter set for a given subspace.
    pub fn set_param_set(&self, subspace: &str, pset: impl Into<Any>) -> RunnerResult<()> {
        unsafe {
            // BeginBlock(self.id);
            let pset = Message::encode_to_vec(&pset.into());
            let pset = BASE64_STANDARD.encode(pset);
            redefine_as_go_string!(pset);
            redefine_as_go_string!(subspace);
            let res = SetParamSet(self.id, subspace, pset);

            // EndBlock(self.id);

            // returns empty bytes if success
            RawResult::from_non_null_ptr(res).into_result()?;
            Ok(())
        }
    }

    /// Get parameter set for a given subspace.
    pub fn get_param_set<P: Message + Default>(
        &self,
        subspace: &str,
        type_url: &str,
    ) -> RunnerResult<P> {
        unsafe {
            redefine_as_go_string!(subspace);
            redefine_as_go_string!(type_url);
            let pset = GetParamSet(self.id, subspace, type_url);
            let pset = RawResult::from_non_null_ptr(pset).into_result()?;
            let pset = P::decode(pset.as_slice()).map_err(DecodeError::ProtoDecodeError)?;
            Ok(pset)
        }
    }
}

/// Cleanup the test environment when the app is dropped.
impl Drop for BaseApp {
    fn drop(&mut self) {
        unsafe {
            CleanUp(self.id);
        }
    }
}

impl<'a> Runner<'a> for BaseApp {
    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        let msgs = msgs
            .iter()
            .map(|(msg, type_url)| {
                let mut buf = Vec::new();
                M::encode(msg, &mut buf).map_err(EncodeError::ProtoEncodeError)?;

                Ok(cosmrs::Any {
                    type_url: type_url.to_string(),
                    value: buf,
                })
            })
            .collect::<Result<Vec<cosmrs::Any>, RunnerError>>()?;

        self.execute_multiple_raw(msgs, signer)
    }

    fn execute_multiple_raw<R>(
        &self,
        msgs: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        R: ::prost::Message + Default,
    {
        let tx_sim_fee =
            self.create_signed_tx(msgs.clone(), signer, self.default_simulation_fee())?;
        let fee = self.calculate_fee(&tx_sim_fee, signer)?;

        let tx = self.create_signed_tx(msgs.clone(), signer, fee)?;
        let res = self.execute_tx(&tx)?;

        res.try_into()
    }

    fn execute_tx(&self, tx_bytes: &[u8]) -> RunnerResult<ResponseFinalizeBlock> {
        unsafe {
            let base64_tx = BASE64_STANDARD.encode(tx_bytes);
            redefine_as_go_string!(base64_tx);

            let res = FinalizeBlock(self.id, base64_tx);
            let res = RawResult::from_non_null_ptr(res).into_result()?;

            RawResult::from_non_null_ptr(Commit(self.id)).into_result()?;

            let res = ResponseFinalizeBlock::decode(res.as_slice())
                .map_err(DecodeError::ProtoDecodeError)?;

            let tx_result = res.tx_results.get(0).cloned().expect("tx_result not found");

            if !tx_result.codespace.is_empty() {
                return Err(RunnerError::ExecuteError {
                    msg: tx_result.log.clone(),
                });
            }

            Ok(res)
        }
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default,
    {
        let mut buf = Vec::new();

        Q::encode(q, &mut buf).map_err(EncodeError::ProtoEncodeError)?;

        let base64_query_msg_bytes = BASE64_STANDARD.encode(buf);
        redefine_as_go_string!(path);
        redefine_as_go_string!(base64_query_msg_bytes);

        unsafe {
            let res = Query(self.id, path, base64_query_msg_bytes);
            let res = RawResult::from_non_null_ptr(res).into_result()?;
            R::decode(res.as_slice())
                .map_err(DecodeError::ProtoDecodeError)
                .map_err(RunnerError::DecodeError)
        }
    }
}
