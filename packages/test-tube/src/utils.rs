use cosmrs::proto::{
    cosmos::bank::v1beta1::MsgSend,
    cosmwasm::wasm::v1::{
        MsgClearAdmin, MsgExecuteContract, MsgInstantiateContract, MsgMigrateContract,
        MsgUpdateAdmin,
    },
};
use cosmwasm_std::{BankMsg, Coin, WasmMsg};
use prost::Message;

use crate::{Account, EncodeError, RunnerError, SigningAccount};

pub fn coins_to_proto(coins: &[Coin]) -> Vec<cosmrs::proto::cosmos::base::v1beta1::Coin> {
    let mut coins = coins.to_vec();
    coins.sort_by(|a, b| a.denom.cmp(&b.denom));
    coins
        .iter()
        .map(|c| cosmrs::proto::cosmos::base::v1beta1::Coin {
            denom: c.denom.parse().unwrap(),
            amount: format!("{}", c.amount.u128()),
        })
        .collect()
}

pub fn proto_coin_to_coin(proto_coin: &cosmrs::proto::cosmos::base::v1beta1::Coin) -> Coin {
    Coin {
        denom: proto_coin.denom.clone(),
        amount: proto_coin.amount.parse().unwrap(),
    }
}

pub fn proto_coins_to_coins(coins: &[cosmrs::proto::cosmos::base::v1beta1::Coin]) -> Vec<Coin> {
    coins.iter().map(proto_coin_to_coin).collect()
}

pub fn msg_to_any<T: Message>(type_url: &str, msg: &T) -> Result<cosmrs::Any, RunnerError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)
        .map_err(EncodeError::ProtoEncodeError)?;

    Ok(cosmrs::Any {
        type_url: type_url.to_owned(),
        value: buf,
    })
}

pub fn bank_msg_to_any(msg: &BankMsg, signer: &SigningAccount) -> Result<cosmrs::Any, RunnerError> {
    match msg {
        BankMsg::Send { to_address, amount } => {
            let type_url = "/cosmos.bank.v1beta1.MsgSend";
            let msg = MsgSend {
                from_address: signer.address(),
                to_address: to_address.to_string(),
                amount: coins_to_proto(amount),
            };
            msg_to_any(type_url, &msg)
        }
        _ => {
            todo!() // TODO: Can't find BurnMsg...?
        }
    }
}

pub fn wasm_msg_to_any(msg: &WasmMsg, signer: &SigningAccount) -> Result<cosmrs::Any, RunnerError> {
    match msg {
        WasmMsg::Execute {
            contract_addr,
            msg,
            funds,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgExecuteContract",
            &MsgExecuteContract {
                contract: contract_addr.clone(),
                funds: coins_to_proto(funds),
                sender: signer.address(),
                msg: msg.to_vec(),
            },
        ),
        WasmMsg::Instantiate {
            admin,
            code_id,
            msg,
            funds,
            label,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgInstantiateContract",
            &MsgInstantiateContract {
                sender: signer.address(),
                admin: admin.clone().unwrap_or_default(),
                code_id: *code_id,
                label: label.clone(),
                msg: msg.to_vec(),
                funds: coins_to_proto(funds),
            },
        ),
        WasmMsg::Migrate {
            contract_addr,
            new_code_id,
            msg,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgMigrateContract",
            &MsgMigrateContract {
                contract: contract_addr.clone(),
                sender: signer.address(),
                code_id: *new_code_id,
                msg: msg.to_vec(),
            },
        ),
        WasmMsg::UpdateAdmin {
            contract_addr,
            admin,
        } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgUpdateAdmin",
            &MsgUpdateAdmin {
                contract: contract_addr.clone(),
                sender: signer.address(),
                new_admin: admin.clone(),
            },
        ),
        WasmMsg::ClearAdmin { contract_addr } => msg_to_any(
            "/cosmwasm.wasm.v1.MsgClearAdmin",
            &MsgClearAdmin {
                contract: contract_addr.clone(),
                sender: signer.address(),
            },
        ),
        _ => Err(RunnerError::ExecuteError {
            msg: "Unsupported WasmMsg".to_string(),
        }),
    }
}
