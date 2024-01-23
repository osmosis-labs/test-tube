#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RandomDataResponse, SudoMsg};
use crate::state::RANDOM_DATA;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    return Ok(Response::default());
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRandomData { key } => {
            let value = RANDOM_DATA.load(deps.storage, key.as_str())?;
            to_json_binary(&RandomDataResponse { key, value })
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::SetRandomData { key, value } => {
            let attrs = vec![
                ("action", "set_random_data"),
                ("key", key.as_str()),
                ("value", value.as_str()),
            ];

            RANDOM_DATA.save(deps.storage, key.as_str(), &value)?;

            Ok(Response::default()
                .add_attributes(attrs)
                .set_data(format!("{key}={value}").as_bytes()))
        }
    }
}

#[cfg(test)]
mod tests {}
