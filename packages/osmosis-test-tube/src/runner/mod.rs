pub mod app;

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::{Bank, Wasm};

    use super::app::OsmosisTestApp;
    use cosmwasm_std::{to_binary, BankMsg, Coin, CosmosMsg, Empty, Event, WasmMsg};
    use cw1_whitelist::msg::{ExecuteMsg, InstantiateMsg};
    use osmosis_std::types::cosmos::bank::v1beta1::{MsgSendResponse, QueryBalanceRequest};
    use osmosis_std::types::cosmwasm::wasm::v1::{
        MsgExecuteContractResponse, MsgInstantiateContractResponse,
    };
    use osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::{
        MsgCreateBalancerPool, MsgCreateBalancerPoolResponse,
    };
    use osmosis_std::types::osmosis::gamm::v1beta1::PoolParams;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{PoolRequest, PoolResponse};
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgCreateDenomResponse,
    };
    use test_tube::account::Account;
    use test_tube::runner::error::RunnerError::{ExecuteError, QueryError};
    use test_tube::runner::result::RawResult;
    use test_tube::runner::Runner;
    use test_tube::{Module, RunnerExecuteResult};

    #[derive(::prost::Message)]
    struct AdhocRandomQueryRequest {
        #[prost(uint64, tag = "1")]
        id: u64,
    }

    #[derive(::prost::Message, serde::Deserialize)]
    struct AdhocRandomQueryResponse {
        #[prost(string, tag = "1")]
        msg: String,
    }

    #[test]
    fn test_query_error_no_route() {
        let app = OsmosisTestApp::default();
        let res = app.query::<AdhocRandomQueryRequest, AdhocRandomQueryResponse>(
            "/osmosis.random.v1beta1.Query/AdhocRandom",
            &AdhocRandomQueryRequest { id: 1 },
        );

        let err = res.unwrap_err();
        assert_eq!(
            err,
            QueryError {
                msg: "No route found for `/osmosis.random.v1beta1.Query/AdhocRandom`".to_string()
            }
        );
    }

    #[test]
    fn test_query_error_failed_query() {
        let app = OsmosisTestApp::default();
        let res = app.query::<PoolRequest, PoolResponse>(
            "/osmosis.poolmanager.v1beta1.Query/Pool",
            &PoolRequest { pool_id: 1 },
        );

        let err = res.unwrap_err();
        assert_eq!(
            err,
            QueryError {
                msg: "rpc error: code = Internal desc = failed to find route for pool id (1)"
                    .to_string()
            }
        );
    }

    #[test]
    fn test_execute_error() {
        let app = OsmosisTestApp::default();
        let signer = app.init_account(&[]).unwrap();
        let res: RunnerExecuteResult<MsgCreateBalancerPoolResponse> = app.execute(
            MsgCreateBalancerPool {
                sender: signer.address(),
                pool_params: Some(PoolParams {
                    swap_fee: "10000000000000000".to_string(),
                    exit_fee: "10000000000000000".to_string(),
                    smooth_weight_change_params: None,
                }),
                pool_assets: vec![],
                future_pool_governor: "".to_string(),
            },
            MsgCreateBalancerPool::TYPE_URL,
            &signer,
        );

        let err = res.unwrap_err();
        assert_eq!(
            err,
            ExecuteError {
                msg: String::from("pool should have at least 2 assets, as they must be swapping between at least two assets")
            }
        )
    }

    #[test]
    fn test_raw_result_ptr_with_0_bytes_in_content_should_not_error() {
        let base64_string = base64::encode(vec![vec![0u8], vec![0u8]].concat());
        let res = unsafe {
            RawResult::from_ptr(
                CString::new(base64_string.as_bytes().to_vec())
                    .unwrap()
                    .into_raw(),
            )
        }
        .unwrap()
        .into_result()
        .unwrap();

        assert_eq!(res, vec![0u8]);
    }

    #[test]
    fn test_execute_cosmos_msgs() {
        let app = OsmosisTestApp::new();
        let signer = app
            .init_account(&[Coin::new(1000000000000, "uosmo")])
            .unwrap();

        let bank = Bank::new(&app);

        // BankMsg::Send
        let to = app.init_account(&[]).unwrap();
        let coin = Coin::new(100, "uosmo");
        let send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: to.address(),
            amount: vec![coin],
        });
        app.execute_cosmos_msgs::<MsgSendResponse>(&[send_msg], &signer)
            .unwrap();
        let balance = bank
            .query_balance(&QueryBalanceRequest {
                address: to.address(),
                denom: "uosmo".to_string(),
            })
            .unwrap()
            .balance;
        assert_eq!(balance.clone().unwrap().amount, "100".to_string());
        assert_eq!(balance.unwrap().denom, "uosmo".to_string());

        // WasmMsg, first upload a contract
        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &signer)
            .unwrap()
            .data
            .code_id;
        assert_eq!(code_id, 1);

        // Wasm::Instantiate
        let instantiate_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id,
            msg: to_binary(&InstantiateMsg {
                admins: vec![signer.address()],
                mutable: true,
            })
            .unwrap(),
            funds: vec![],
            label: "test".to_string(),
            admin: None,
        });
        let init_res = app
            .execute_cosmos_msgs::<MsgInstantiateContractResponse>(&[instantiate_msg], &signer)
            .unwrap();
        let contract_address = init_res.data.address;
        assert_ne!(contract_address, "".to_string());

        // Wasm::Execute
        let execute_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_address.clone(),
            msg: to_binary(&ExecuteMsg::<Empty>::Freeze {}).unwrap(),
            funds: vec![],
        });
        let execute_res = app
            .execute_cosmos_msgs::<MsgExecuteContractResponse>(&[execute_msg], &signer)
            .unwrap();
        let events = execute_res.events;

        let wasm_events: Vec<Event> = events.into_iter().filter(|x| x.ty == "wasm").collect();
        for event in wasm_events.iter() {
            assert_eq!(event.attributes[0].key, "_contract_address");
            assert_eq!(event.attributes[0].value, contract_address);
            assert_eq!(event.attributes[1].key, "action");
            assert_eq!(event.attributes[1].value, "freeze");
        }

        // Stargate
        let denom = "test".to_string();
        let create_denom_msg: CosmosMsg = MsgCreateDenom {
            sender: signer.address(),
            subdenom: denom.clone(),
        }
        .into();
        let create_denom_res = app
            .execute_cosmos_msgs::<MsgCreateDenomResponse>(&[create_denom_msg], &signer)
            .unwrap();
        assert_eq!(
            create_denom_res.data.new_token_denom,
            format!("factory/{}/{}", signer.address(), denom)
        );
    }
}
