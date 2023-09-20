use coreum_wasm_sdk::types::cosmwasm::wasm::v1::{
    AccessConfig, MsgExecuteContract, MsgExecuteContractResponse, MsgInstantiateContract,
    MsgInstantiateContractResponse, MsgStoreCode, MsgStoreCodeResponse,
    QuerySmartContractStateRequest, QuerySmartContractStateResponse,
};
use cosmwasm_std::Coin;
use serde::{de::DeserializeOwned, Serialize};

use test_tube_coreum::{
    runner::Runner, Account, DecodeError, EncodeError, RunnerError, RunnerExecuteResult,
    RunnerResult, SigningAccount,
};

pub struct Wasm<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> super::Module<'a, R> for Wasm<'a, R> {
    fn new(runner: &'a R) -> Self {
        Wasm { runner }
    }
}

impl<'a, R> Wasm<'a, R>
where
    R: Runner<'a>,
{
    pub fn store_code(
        &self,
        wasm_byte_code: &[u8],
        instantiate_permission: Option<AccessConfig>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgStoreCodeResponse> {
        self.runner.execute(
            MsgStoreCode {
                sender: signer.address(),
                wasm_byte_code: wasm_byte_code.to_vec(),
                instantiate_permission,
            },
            "/cosmwasm.wasm.v1.MsgStoreCode",
            signer,
        )
    }

    pub fn instantiate<M>(
        &self,
        code_id: u64,
        msg: &M,
        admin: Option<&str>,
        label: Option<&str>,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgInstantiateContractResponse>
    where
        M: ?Sized + Serialize,
    {
        self.runner.execute(
            MsgInstantiateContract {
                sender: signer.address(),
                admin: admin.unwrap_or_default().to_string(),
                code_id,
                label: label.unwrap_or(" ").to_string(), // empty string causes panic
                msg: serde_json::to_vec(msg).map_err(EncodeError::JsonEncodeError)?,
                funds: funds
                    .iter()
                    .map(|c| coreum_wasm_sdk::types::cosmos::base::v1beta1::Coin {
                        denom: c.denom.parse().unwrap(),
                        amount: format!("{}", c.amount.u128()),
                    })
                    .collect(),
            },
            "/cosmwasm.wasm.v1.MsgInstantiateContract",
            signer,
        )
    }

    pub fn execute<M>(
        &self,
        contract: &str,
        msg: &M,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse>
    where
        M: ?Sized + Serialize,
    {
        self.runner.execute(
            MsgExecuteContract {
                sender: signer.address(),
                msg: serde_json::to_vec(msg).map_err(EncodeError::JsonEncodeError)?,
                funds: funds
                    .iter()
                    .map(|c| coreum_wasm_sdk::types::cosmos::base::v1beta1::Coin {
                        denom: c.denom.parse().unwrap(),
                        amount: format!("{}", c.amount.u128()),
                    })
                    .collect(),
                contract: contract.to_owned(),
            },
            "/cosmwasm.wasm.v1.MsgExecuteContract",
            signer,
        )
    }

    pub fn query<M, Res>(&self, contract: &str, msg: &M) -> RunnerResult<Res>
    where
        M: ?Sized + Serialize,
        Res: ?Sized + DeserializeOwned,
    {
        let res = self
            .runner
            .query::<QuerySmartContractStateRequest, QuerySmartContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/SmartContractState",
                &QuerySmartContractStateRequest {
                    address: contract.to_owned(),
                    query_data: serde_json::to_vec(msg).map_err(EncodeError::JsonEncodeError)?,
                },
            )?;

        serde_json::from_slice(&res.data)
            .map_err(DecodeError::JsonDecodeError)
            .map_err(RunnerError::DecodeError)
    }
}

#[cfg(test)]
mod tests {
    use crate::{runner::app::FEE_DENOM, CoreumTestApp, Wasm};
    use crate::{Account, Module};
    use cosmwasm_std::Coin;
    use cw1_whitelist::msg::{AdminListResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

    #[test]
    fn wasm_integration() {
        let app = CoreumTestApp::new();
        let signer = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();

        let user = app.init_account(&[]).unwrap();

        let wasm = Wasm::new(&app);

        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();

        let code_id = wasm
            .store_code(&wasm_byte_code, None, &signer)
            .unwrap()
            .data
            .code_id;

        assert_eq!(code_id, 1);

        let contract_addr = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    admins: vec![signer.address()],
                    mutable: true,
                },
                None,
                "label".into(),
                &[],
                &signer,
            )
            .unwrap()
            .data
            .address;

        assert!(!contract_addr.is_empty());

        wasm.execute::<ExecuteMsg>(
            &contract_addr,
            &ExecuteMsg::UpdateAdmins {
                admins: vec![user.address()],
            },
            &vec![],
            &signer,
        )
        .unwrap();

        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();

        assert_eq!(admin_list.admins, vec![user.address()]);
        assert!(admin_list.mutable);
    }
}
