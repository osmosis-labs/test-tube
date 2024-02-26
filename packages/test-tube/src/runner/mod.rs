use cosmwasm_std::CosmosMsg;
use serde::de::DeserializeOwned;

use crate::account::SigningAccount;
use crate::runner::result::{RunnerExecuteResult, RunnerResult};
use crate::utils::{bank_msg_to_any, wasm_msg_to_any};
use crate::RunnerError;

pub mod app;
pub mod error;
pub mod result;

pub trait Runner<'a> {
    fn execute<M, R>(
        &self,
        msg: M,
        type_url: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.execute_custom_tx(msg, type_url, "", 0, vec![], vec![], signer)
    }

    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.execute_multiple_custom_tx(msgs, "", 0, vec![], vec![], signer)
    }

    fn execute_multiple_raw<R>(
        &self,
        msgs: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        R: ::prost::Message + Default,
    {
        self.execute_multiple_raw_custom_tx(msgs, "", 0, vec![], vec![], signer)
    }

    fn execute_custom_tx<M, R>(
        &self,
        msg: M,
        type_url: &str,
        memo: &str,
        timeout_height: u32,
        extension_options: Vec<cosmrs::Any>,
        non_critical_extension_options: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.execute_multiple_custom_tx(
            &[(msg, type_url)],
            memo,
            timeout_height,
            extension_options,
            non_critical_extension_options,
            signer,
        )
    }

    fn execute_multiple_custom_tx<M, R>(
        &self,
        msgs: &[(M, &str)],
        memo: &str,
        timeout_height: u32,
        extension_options: Vec<cosmrs::Any>,
        non_critical_extension_options: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default;

    fn execute_multiple_raw_custom_tx<R>(
        &self,
        msgs: Vec<cosmrs::Any>,
        memo: &str,
        timeout_height: u32,
        extension_options: Vec<cosmrs::Any>,
        non_critical_extension_options: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        R: ::prost::Message + Default;

    fn execute_cosmos_msgs<S>(
        &self,
        msgs: &[CosmosMsg],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<S>
    where
        S: ::prost::Message + Default,
    {
        self.execute_cosmos_msgs_custom_tx(msgs, "", 0, vec![], vec![], signer)
    }

    fn execute_cosmos_msgs_custom_tx<S>(
        &self,
        msgs: &[CosmosMsg],
        memo: &str,
        timeout_height: u32,
        extension_options: Vec<cosmrs::Any>,
        non_critical_extension_options: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<S>
    where
        S: ::prost::Message + Default,
    {
        let msgs = msgs
            .iter()
            .map(|msg| match msg {
                CosmosMsg::Bank(msg) => bank_msg_to_any(msg, signer),
                CosmosMsg::Stargate { type_url, value } => Ok(cosmrs::Any {
                    type_url: type_url.clone(),
                    value: value.0.clone(),
                }),
                CosmosMsg::Wasm(msg) => wasm_msg_to_any(msg, signer),
                _ => todo!("unsupported cosmos msg variant"),
            })
            .collect::<Result<Vec<_>, RunnerError>>()?;

        self.execute_multiple_raw_custom_tx(
            msgs,
            memo,
            timeout_height,
            extension_options,
            non_critical_extension_options,
            signer,
        )
    }

    fn query<Q, R>(&self, path: &str, query: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + DeserializeOwned + Default;
}
