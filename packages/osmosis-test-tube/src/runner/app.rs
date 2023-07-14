use cosmrs::Any;

use cosmwasm_std::{Coin, Timestamp};

use prost::Message;
use serde::de::DeserializeOwned;
use test_tube::account::SigningAccount;

use test_tube::runner::result::{RunnerExecuteResult, RunnerResult};
use test_tube::runner::Runner;
use test_tube::BaseApp;

const FEE_DENOM: &str = "uosmo";
const OSMO_ADDRESS_PREFIX: &str = "osmo";
const CHAIN_ID: &str = "osmosis-1";
const DEFAULT_GAS_ADJUSTMENT: f64 = 1.2;

#[derive(Debug, PartialEq)]
pub struct OsmosisTestApp {
    inner: BaseApp,
}

impl Default for OsmosisTestApp {
    fn default() -> Self {
        OsmosisTestApp::new()
    }
}

impl OsmosisTestApp {
    pub fn new() -> Self {
        Self {
            inner: BaseApp::new(
                FEE_DENOM,
                CHAIN_ID,
                OSMO_ADDRESS_PREFIX,
                DEFAULT_GAS_ADJUSTMENT,
            ),
        }
    }

    /// Get the current block time as a timestamp
    pub fn get_block_timestamp(&self) -> Timestamp {
        self.inner.get_block_timestamp()
    }

    /// Get the current block time in nanoseconds
    pub fn get_block_time_nanos(&self) -> i64 {
        self.inner.get_block_time_nanos()
    }

    /// Get the current block time in seconds
    pub fn get_block_time_seconds(&self) -> i64 {
        self.inner.get_block_time_nanos() / 1_000_000_000i64
    }

    /// Get the current block height
    pub fn get_block_height(&self) -> i64 {
        self.inner.get_block_height()
    }

    /// Get the first validator address
    pub fn get_first_validator_address(&self) -> RunnerResult<String> {
        self.inner.get_first_validator_address()
    }

    /// Get the first validator signing account
    pub fn get_first_validator_signing_account(&self) -> RunnerResult<SigningAccount> {
        self.inner.get_first_validator_signing_account()
    }

    /// Increase the time of the blockchain by the given number of seconds.
    pub fn increase_time(&self, seconds: u64) {
        self.inner.increase_time(seconds)
    }

    /// Initialize account with initial balance of any coins.
    /// This function mints new coins and send to newly created account
    pub fn init_account(&self, coins: &[Coin]) -> RunnerResult<SigningAccount> {
        self.inner.init_account(coins)
    }
    /// Convinience function to create multiple accounts with the same
    /// Initial coins balance
    pub fn init_accounts(&self, coins: &[Coin], count: u64) -> RunnerResult<Vec<SigningAccount>> {
        self.inner.init_accounts(coins, count)
    }

    /// Simulate transaction execution and return gas info
    pub fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> RunnerResult<cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        self.inner.simulate_tx(msgs, signer)
    }

    /// Set parameter set for a given subspace.
    pub fn set_param_set(&self, subspace: &str, pset: impl Into<Any>) -> RunnerResult<()> {
        self.inner.set_param_set(subspace, pset)
    }

    /// Get parameter set for a given subspace.
    pub fn get_param_set<P: Message + Default>(
        &self,
        subspace: &str,
        type_url: &str,
    ) -> RunnerResult<P> {
        self.inner.get_param_set(subspace, type_url)
    }
}

impl<'a> Runner<'a> for OsmosisTestApp {
    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.inner.execute_multiple(msgs, signer)
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + DeserializeOwned + Default,
    {
        self.inner.query(path, q)
    }

    fn execute_multiple_raw<R>(
        &self,
        msgs: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        R: prost::Message + Default,
    {
        self.inner.execute_multiple_raw(msgs, signer)
    }
}

#[cfg(test)]
mod tests {
    use osmosis_std::types::cosmos::bank::v1beta1::QueryAllBalancesResponse;
    use prost::Message;
    use std::option::Option::None;

    use cosmrs::Any;
    use cosmwasm_std::{attr, coins, Coin};
    use osmosis_std::types::cosmos::bank::v1beta1::QueryAllBalancesRequest;

    use osmosis_std::types::osmosis::gamm::v1beta1::QueryTotalSharesRequest;
    use osmosis_std::types::osmosis::lockup::{
        self, MsgForceUnlock, MsgForceUnlockResponse, MsgLockTokens, MsgLockTokensResponse,
    };
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgCreateDenomResponse, QueryParamsRequest, QueryParamsResponse,
    };

    use crate::module::Gamm;
    use crate::module::Wasm;
    use crate::runner::app::OsmosisTestApp;
    use crate::Bank;
    use test_tube::account::{Account, FeeSetting};
    use test_tube::module::Module;
    use test_tube::ExecuteResponse;
    use test_tube::{runner::*, RunnerError};

    #[test]
    fn test_init_accounts() {
        let app = OsmosisTestApp::default();
        let accounts = app
            .init_accounts(&coins(100_000_000_000, "uosmo"), 3)
            .unwrap();

        assert!(accounts.get(0).is_some());
        assert!(accounts.get(1).is_some());
        assert!(accounts.get(2).is_some());
        assert!(accounts.get(3).is_none());
    }

    #[test]
    fn test_get_and_set_block_timestamp() {
        let app = OsmosisTestApp::default();

        let block_time_nanos = app.get_block_time_nanos();
        let block_time_seconds = app.get_block_time_seconds();

        app.increase_time(10u64);

        assert_eq!(
            app.get_block_time_nanos(),
            block_time_nanos + 10_000_000_000
        );
        assert_eq!(app.get_block_time_seconds(), block_time_seconds + 10);
    }

    #[test]
    fn test_get_block_height() {
        let app = OsmosisTestApp::default();

        assert_eq!(app.get_block_height(), 1i64);

        app.increase_time(10u64);

        assert_eq!(app.get_block_height(), 2i64);
    }

    #[test]
    fn test_execute() {
        let app = OsmosisTestApp::default();

        let acc = app
            .init_account(&coins(100_000_000_000_000, "uosmo"))
            .unwrap();
        let addr = acc.address();

        let msg = MsgCreateDenom {
            sender: acc.address(),
            subdenom: "newdenom".to_string(),
        };

        let res: ExecuteResponse<MsgCreateDenomResponse> =
            app.execute(msg, MsgCreateDenom::TYPE_URL, &acc).unwrap();

        let create_denom_attrs = &res
            .events
            .iter()
            .find(|e| e.ty == "create_denom")
            .unwrap()
            .attributes;

        assert_eq!(
            create_denom_attrs,
            &vec![
                attr("creator", &addr),
                attr(
                    "new_token_denom",
                    format!("factory/{}/{}", &addr, "newdenom")
                )
            ]
        );

        // execute on more time to excercise account sequence
        let msg = MsgCreateDenom {
            sender: acc.address(),
            subdenom: "newerdenom".to_string(),
        };

        let res: ExecuteResponse<MsgCreateDenomResponse> =
            app.execute(msg, MsgCreateDenom::TYPE_URL, &acc).unwrap();

        let create_denom_attrs = &res
            .events
            .iter()
            .find(|e| e.ty == "create_denom")
            .unwrap()
            .attributes;

        // TODO: make assertion based on string representation
        assert_eq!(
            create_denom_attrs,
            &vec![
                attr("creator", &addr),
                attr(
                    "new_token_denom",
                    format!("factory/{}/{}", &addr, "newerdenom")
                )
            ]
        );
    }

    #[test]
    fn test_query() {
        let app = OsmosisTestApp::default();

        let denom_creation_fee = app
            .query::<QueryParamsRequest, QueryParamsResponse>(
                "/osmosis.tokenfactory.v1beta1.Query/Params",
                &QueryParamsRequest {},
            )
            .unwrap()
            .params
            .unwrap()
            .denom_creation_fee;

        // fee is no longer set
        assert_eq!(denom_creation_fee, [])
    }

    #[test]
    fn test_multiple_as_module() {
        let app = OsmosisTestApp::default();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, "uatom"),
                Coin::new(1_000_000_000_000, "uosmo"),
            ])
            .unwrap();

        let gamm = Gamm::new(&app);

        let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
        let pool_id = gamm
            .create_basic_pool(&pool_liquidity, &alice)
            .unwrap()
            .data
            .pool_id;

        let pool = gamm.query_pool(pool_id).unwrap();

        assert_eq!(
            pool_liquidity
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<osmosis_std::types::cosmos::base::v1beta1::Coin>>(),
            pool.pool_assets
                .into_iter()
                .map(|a| a.token.unwrap())
                .collect::<Vec<osmosis_std::types::cosmos::base::v1beta1::Coin>>(),
        );

        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, &alice)
            .unwrap()
            .data
            .code_id;

        assert_eq!(code_id, 1);
    }

    #[test]
    fn test_wasm_execute_and_query() {
        use cw1_whitelist::msg::*;

        let app = OsmosisTestApp::default();
        let accs = app
            .init_accounts(
                &[
                    Coin::new(1_000_000_000_000, "uatom"),
                    Coin::new(1_000_000_000_000, "uosmo"),
                ],
                2,
            )
            .unwrap();
        let admin = &accs[0];
        let new_admin = &accs[1];

        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, admin)
            .unwrap()
            .data
            .code_id;
        assert_eq!(code_id, 1);

        // initialize admins and check if the state is correct
        let init_admins = vec![admin.address()];
        let contract_addr = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    admins: init_admins.clone(),
                    mutable: true,
                },
                Some(&admin.address()),
                None,
                &[],
                admin,
            )
            .unwrap()
            .data
            .address;
        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();
        assert_eq!(admin_list.admins, init_admins);
        assert!(admin_list.mutable);

        // update admin and check again
        let new_admins = vec![new_admin.address()];
        wasm.execute::<ExecuteMsg>(
            &contract_addr,
            &ExecuteMsg::UpdateAdmins {
                admins: new_admins.clone(),
            },
            &[],
            admin,
        )
        .unwrap();

        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();

        assert_eq!(admin_list.admins, new_admins);
        assert!(admin_list.mutable);
    }

    #[test]
    fn test_custom_fee() {
        let app = OsmosisTestApp::default();
        let initial_balance = 1_000_000_000_000;
        let alice = app.init_account(&coins(initial_balance, "uosmo")).unwrap();
        let bob = app.init_account(&coins(initial_balance, "uosmo")).unwrap();

        let amount = Coin::new(1_000_000, "uosmo");
        let gas_limit = 100_000_000;

        // use FeeSetting::Auto by default, so should not equal newly custom fee setting
        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let res = wasm.store_code(&wasm_byte_code, None, &alice).unwrap();

        assert_ne!(res.gas_info.gas_wanted, gas_limit);

        //update fee setting
        let bob = bob.with_fee_setting(FeeSetting::Custom {
            amount: amount.clone(),
            gas_limit,
        });
        let res = wasm.store_code(&wasm_byte_code, None, &bob).unwrap();

        let bob_balance = Bank::new(&app)
            .query_all_balances(&QueryAllBalancesRequest {
                address: bob.address(),
                pagination: None,
            })
            .unwrap()
            .balances
            .into_iter()
            .find(|c| c.denom == "uosmo")
            .unwrap()
            .amount
            .parse::<u128>()
            .unwrap();

        assert_eq!(res.gas_info.gas_wanted, gas_limit);
        assert_eq!(bob_balance, initial_balance - amount.amount.u128());
    }

    #[test]
    fn test_param_set() {
        let app = OsmosisTestApp::default();

        let whitelisted_users = app
            .init_accounts(&coins(1_000_000_000_000, "uosmo"), 2)
            .unwrap();

        let in_pset = lockup::Params {
            force_unlock_allowed_addresses: whitelisted_users
                .into_iter()
                .map(|a| a.address())
                .collect(),
        };

        app.set_param_set(
            "lockup",
            osmosis_std::shim::Any {
                type_url: lockup::Params::TYPE_URL.to_string(),
                value: in_pset.encode_to_vec(),
            },
        )
        .unwrap();

        let out_pset: lockup::Params = app
            .get_param_set("lockup", lockup::Params::TYPE_URL)
            .unwrap();

        assert_eq!(in_pset, out_pset);
    }

    #[test]
    fn test_set_param_set() {
        let app = OsmosisTestApp::default();

        let balances = vec![
            Coin::new(1_000_000_000_000, "uosmo"),
            Coin::new(1_000_000_000_000, "uion"),
        ];
        let whitelisted_user = app.init_account(&balances).unwrap();

        // create pool
        let gamm = Gamm::new(&app);
        let pool_id = gamm
            .create_basic_pool(
                &[Coin::new(1_000_000, "uosmo"), Coin::new(1_000_000, "uion")],
                &whitelisted_user,
            )
            .unwrap()
            .data
            .pool_id;

        // query shares
        let shares = app
            .query::<QueryTotalSharesRequest, QueryAllBalancesResponse>(
                "/osmosis.gamm.v1beta1.Query/TotalShares",
                &QueryTotalSharesRequest { pool_id },
            )
            .unwrap()
            .balances;

        // lock all shares
        app.execute::<_, MsgLockTokensResponse>(
            MsgLockTokens {
                owner: whitelisted_user.address(),
                duration: Some(osmosis_std::shim::Duration {
                    seconds: 1000000000,
                    nanos: 0,
                }),
                coins: shares,
            },
            MsgLockTokens::TYPE_URL,
            &whitelisted_user,
        )
        .unwrap();

        // try to unlock
        let err = app
            .execute::<_, MsgForceUnlockResponse>(
                MsgForceUnlock {
                    owner: whitelisted_user.address(),
                    id: pool_id,
                    coins: vec![], // all
                },
                MsgForceUnlock::TYPE_URL,
                &whitelisted_user,
            )
            .unwrap_err();

        // should fail
        assert_eq!(err,  RunnerError::ExecuteError {
            msg: format!("failed to execute message; message index: 0: Sender ({}) not allowed to force unlock: unauthorized", whitelisted_user.address()),
        });

        // add whitelisted user to param set
        app.set_param_set(
            "lockup",
            Any {
                type_url: lockup::Params::TYPE_URL.to_string(),
                value: lockup::Params {
                    force_unlock_allowed_addresses: vec![whitelisted_user.address()],
                }
                .encode_to_vec(),
            },
        )
        .unwrap();

        // unlock again after adding whitelisted user
        let res = app
            .execute::<_, MsgForceUnlockResponse>(
                MsgForceUnlock {
                    owner: whitelisted_user.address(),
                    id: pool_id,
                    coins: vec![], // all
                },
                MsgForceUnlock::TYPE_URL,
                &whitelisted_user,
            )
            .unwrap();

        // should succeed
        assert!(res.data.success);
    }
}
