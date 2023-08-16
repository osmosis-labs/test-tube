use std::ffi::CString;

use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::proto::tendermint::abci::{RequestDeliverTx, ResponseDeliverTx};
use cosmrs::tx::{Fee, SignerInfo};
use cosmrs::{tx, Any};
use cosmwasm_std::{Coin, Timestamp};
use prost::Message;

use crate::account::{Account, FeeSetting, SigningAccount};
use crate::bindings::{
    AccountNumber, AccountSequence, BeginBlock, CleanUp, EndBlock, Execute, GetBlockHeight,
    GetBlockTime, GetParamSet, GetValidatorAddress, GetValidatorPrivateKey, IncreaseTime,
    InitAccount, InitTestEnv, Query, SetParamSet, Simulate,
};
use crate::redefine_as_go_string;
use crate::runner::error::{DecodeError, EncodeError, RunnerError};
use crate::runner::result::RawResult;
use crate::runner::result::{RunnerExecuteResult, RunnerResult};
use crate::runner::Runner;

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

        let secp256k1_priv = base64::decode(base64_priv).map_err(DecodeError::Base64DecodeError)?;
        let signging_key = SigningKey::from_bytes(&secp256k1_priv).map_err(|e| {
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
            BeginBlock(self.id);
            let addr = InitAccount(self.id, coins_json);
            EndBlock(self.id);
            CString::from_raw(addr)
        }
        .to_str()
        .map_err(DecodeError::Utf8Error)?
        .to_string();

        let secp256k1_priv = base64::decode(base64_priv).map_err(DecodeError::Base64DecodeError)?;
        let signging_key = SigningKey::from_bytes(&secp256k1_priv).map_err(|e| {
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
    /// Convinience function to create multiple accounts with the same
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
        redefine_as_go_string!(addr);

        let seq = unsafe { AccountSequence(self.id, addr) };

        let account_number = unsafe { AccountNumber(self.id, addr) };
        let signer_info = SignerInfo::single_direct(Some(signer.public_key()), seq);
        let auth_info = signer_info.auth_info(fee);
        let sign_doc = tx::SignDoc::new(
            &tx_body,
            &auth_info,
            &(self
                .chain_id
                .parse()
                .expect("parse const str of chain id should never fail")),
            account_number,
        )
        .map_err(|e| match e.downcast::<prost::EncodeError>() {
            Ok(encode_err) => EncodeError::ProtoEncodeError(encode_err),
            Err(e) => panic!("expect `prost::EncodeError` but got {:?}", e),
        })?;

        let tx_raw = sign_doc.sign(signer.signing_key()).unwrap();

        tx_raw
            .to_bytes()
            .map_err(|e| match e.downcast::<prost::EncodeError>() {
                Ok(encode_err) => EncodeError::ProtoEncodeError(encode_err),
                Err(e) => panic!("expect `prost::EncodeError` but got {:?}", e),
            })
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
        let zero_fee = Fee::from_amount_and_gas(
            cosmrs::Coin {
                denom: self.fee_denom.parse().unwrap(),
                amount: OSMOSIS_MIN_GAS_PRICE,
            },
            0u64,
        );

        let tx = self.create_signed_tx(msgs, signer, zero_fee)?;
        let base64_tx_bytes = base64::encode(tx);
        redefine_as_go_string!(base64_tx_bytes);

        unsafe {
            let res = Simulate(self.id, base64_tx_bytes);
            let res = RawResult::from_non_null_ptr(res).into_result()?;

            cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo::decode(res.as_slice())
                .map_err(DecodeError::ProtoDecodeError)
                .map_err(RunnerError::DecodeError)
        }
    }
    fn estimate_fee<I>(&self, msgs: I, signer: &SigningAccount) -> RunnerResult<Fee>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        match &signer.fee_setting() {
            FeeSetting::Auto {
                gas_price,
                gas_adjustment,
            } => {
                let gas_info = self.simulate_tx(msgs, signer)?;
                let gas_limit = ((gas_info.gas_used as f64) * (gas_adjustment)).ceil() as u64;

                let amount = cosmrs::Coin {
                    denom: self.fee_denom.parse().unwrap(),
                    amount: (((gas_limit as f64) * (gas_price.amount.u128() as f64)).ceil() as u64)
                        .into(),
                };

                Ok(Fee::from_amount_and_gas(amount, gas_limit))
            }
            FeeSetting::Custom { .. } => {
                panic!("estimate fee is a private function and should never be called when fee_setting is Custom");
            }
        }
    }

    /// Ensure that all execution that happens in `execution` happens in a block
    /// and end block properly, no matter it suceeds or fails.
    unsafe fn run_block<T, E>(&self, execution: impl Fn() -> Result<T, E>) -> Result<T, E> {
        unsafe { BeginBlock(self.id) };
        match execution() {
            ok @ Ok(_) => {
                unsafe { EndBlock(self.id) };
                ok
            }
            err @ Err(_) => {
                unsafe { EndBlock(self.id) };
                err
            }
        }
    }

    /// Set parameter set for a given subspace.
    pub fn set_param_set(&self, subspace: &str, pset: impl Into<Any>) -> RunnerResult<()> {
        unsafe {
            BeginBlock(self.id);
            let pset = Message::encode_to_vec(&pset.into());
            let pset = base64::encode(pset);
            redefine_as_go_string!(pset);
            redefine_as_go_string!(subspace);
            let res = SetParamSet(self.id, subspace, pset);

            EndBlock(self.id);

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
        unsafe {
            self.run_block(|| {
                let fee = match &signer.fee_setting() {
                    FeeSetting::Auto { .. } => self.estimate_fee(msgs.clone(), signer)?,
                    FeeSetting::Custom { amount, gas_limit } => Fee::from_amount_and_gas(
                        cosmrs::Coin {
                            denom: amount.denom.parse().unwrap(),
                            amount: amount.amount.to_string().parse().unwrap(),
                        },
                        *gas_limit,
                    ),
                };

                let tx = self.create_signed_tx(msgs.clone(), signer, fee)?;

                let mut buf = Vec::new();
                RequestDeliverTx::encode(&RequestDeliverTx { tx }, &mut buf)
                    .map_err(EncodeError::ProtoEncodeError)?;

                let base64_req = base64::encode(buf);
                redefine_as_go_string!(base64_req);

                let res = Execute(self.id, base64_req);
                let res = RawResult::from_non_null_ptr(res).into_result()?;

                ResponseDeliverTx::decode(res.as_slice())
                    .map_err(DecodeError::ProtoDecodeError)?
                    .try_into()
            })
        }
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + Default,
    {
        let mut buf = Vec::new();

        Q::encode(q, &mut buf).map_err(EncodeError::ProtoEncodeError)?;

        let base64_query_msg_bytes = base64::encode(buf);
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
