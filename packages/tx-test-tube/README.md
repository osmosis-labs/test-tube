# tx-test-tube

[![tx-test-tube on crates.io](https://img.shields.io/crates/v/tx-test-tube.svg)](https://crates.io/crates/tx-test-tube) [![Docs](https://docs.rs/tx-test-tube/badge.svg)](https://docs.rs/tx-test-tube)

CosmWasm x TX integration testing library that, unlike `cw-multi-test`, it allows you to test your cosmwasm contract against real chain's logic instead of mocks.

## Table of Contents

- [Getting Started](#getting-started)
- [Debugging](#debugging)
- [Using Module Wrapper](#using-module-wrapper)
- [Versioning](#versioning)

## Getting Started

To demonstrate how `tx-test-tube` works, let use simple example contract: [cw-whitelist](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw1-whitelist) from `cw-plus`.

Here is how to setup the test:

```rust
use cosmwasm_std::Coin;
use tx_test_tube::TXTestApp;

// Create new TX appchain instance.
let app = TXTestApp::new();

// Create a new account with initial funds and one without initial funds
use tx_test_tube::runner:app::FEE_DENOM;

let signer = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();

let user   = app.init_account(&[]).unwrap();
```

Now we have the appchain instance and two accounts, let's interact with the chain.
This does not run Docker instance or spawning external process, it just load the appchain's code as a library and creates an in-memory instance.

Note that `init_account` is a convenience function that creates an account with an initial balance.
If you want to create just many accounts, you can use `init_accounts` instead. There are plenty of convenience functions defined which are defined in the package.

```rust
use cosmwasm_std::coins;
use tx_test_tube::TXTestApp;

let app = TXTestApp::new();

let accounts = app
            .init_accounts(&coins(100_000_000_000, FEE_DENOM), 4)
            .unwrap();

let acc1 = accounts.get(0).unwrap();
let acc2 = accounts.get(1).unwrap();
let acc3 = accounts.get(2).unwrap();
let acc4 = accounts.get(3).unwrap();
```

Now if we want to test a cosmwasm contract, we need to

- have a built and optimized wasm file (We will use the cw1_whitelist.wasm contract placed in `test_artifacts` directory)
- store code
- instantiate
- execute or query

```rust
use cosmwasm_std::coins;
use cw1_whitelist::msg::{InstantiateMsg}; // for instantiating cw1_whitelist contract, which is already in a public crate
use tx_test_tube::{Account, Module, TXTestApp, Wasm};

let app = TXTestApp::new();
let accs = app
        .init_accounts(&coins(100_000_000_000, FEE_DENOM), 2)
        .unwrap();

let account1 = &accs.get(0).unwrap();
let account2 = &accs.get(1).unwrap();;
```

To test our smart contract we must first build an optimized wasm file so that we can store it. For this example, as already mentioned, we will use cw1_whitelist, which we already have compiled in the `test_artifacts` directory.
To get more information about this contract you can check [cw-plus](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw1-whitelist).

```rust
// `Wasm` is the module we use to interact with cosmwasm releated logic on the appchain
let wasm = Wasm::new(&app);

// Store compiled wasm code on the appchain and retrieve its code id
let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
let code_id = wasm
            .store_code(&wasm_byte_code, None, &signer)
            .unwrap()
            .data
            .code_id;

// Instantiate contract with initial admin (signer) account defined beforehand and make admin list mutable
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

// Execute the contract to modify admin to user address

wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::UpdateAdmins {
            admins: vec![user.address()],
        },
        &vec![],
        &signer,
    )
    .unwrap();

// Query the contract to verify that the admin has been updated correctly.
let admin_list = wasm
        .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
        .unwrap();

assert_eq!(admin_list.admins, vec![user.address()]);
assert!(admin_list.mutable);

```

## Debugging

In your contract code, if you want to debug, you can use [`deps.api.debug(..)`](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/trait.Api.html#tymethod.debug) which will prints the debug message to stdout. `wasmd` disabled this by default but `TXTestApp` allows stdout emission so that you can debug your smart contract while running tests.

## Using Module Wrapper

In some cases, you might want interact directly with appchain logic to setup the environment or query appchain's state, instead of testing smart contracts.
Module wrappers provide convenient functions to interact with the appchain's module. You can interact with all TX native modules using these wrappers.

Let's try interact with `AssetFT` module, while, at the same time, interacting with the native `Bank` module.

```rust
use cosmwasm_std::Coin;
use tx_test_tube::{Account, Module, TXTestApp, Bank, AssetFT};

let app = TXTestApp::new();

let signer = app
    .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
     .unwrap();
let receiver = app
    .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
    .unwrap();

// Create AssetFT Module Wrapper
let assetft = AssetFT::new(&app);
// Create Bank Module Wrapper
let bank = Bank::new(&app);

// Query the issue fee and assert if the fee is correct
let request_params = assetft.query_params(&QueryParamsRequest {}).unwrap();
assert_eq!(
    request_params.params.unwrap().issue_fee.unwrap(),
    BaseCoin {
        amount: 10000000u128.to_string(),
        denom: FEE_DENOM.to_string(),
    }
);

// Issue a new native asset with the following information
assetft.
    issue(
        MsgIssue {
            issuer: signer.address(),
            symbol: "TEST".to_string(),
            subunit: "utest".to_string(),
            precision: 6,
            initial_amount: "10".to_string(),
            description: "test_description".to_string(),
            features: vec![MINTING as i32],
            burn_rate: "0".to_string(),
            send_commission_rate: "0".to_string(),
            uri: "test_uri".to_string(),
            uri_hash: "test_uri_hash".to_string(),
        },
        &signer,
    )
    .unwrap();

// Query the new asset and verify that the initial_amount is correct.
let denom = format!("{}-{}", "utest", signer.address()).to_lowercase();
let request_balance = assetft
    .query_balance(&QueryBalanceRequest {
        account: signer.address(),
        denom: denom.clone(),
    })
    .unwrap()
assert_eq!(request_balance.balance, "10".to_string());

// Mint additional tokens and verify that the balance has been updated correctly (10 + 990 = 1000)
assetft
    .mint(
        MsgMint {
            sender: signer.address(),
            coin: Some(BaseCoin {
                denom: denom.clone(),
                amount: "990".to_string(),
            }),
            recipient: signer.address(),
        },
        &signer,
    )
    .unwrap()
let request_balance = assetft
    .query_balance(&QueryBalanceRequest {
        account: signer.address(),
        denom: denom.clone(),
    })
    .unwrap()
assert_eq!(request_balance.balance, "1000".to_string());

// Using the bank module, send a transaction to another address and verify that both balances of the AssetFTs have been updated correctly.
bank.send(
    MsgSend {
        from_address: signer.address(),
        to_address: receiver.address(),
        amount: vec![BaseCoin {
            amount: "100".to_string(),
            denom: denom.clone(),
        }],
    },
    &signer,
)
.unwrap()
let request_balance = assetft
    .query_balance(&QueryBalanceRequest {
        account: signer.address(),
        denom: denom.clone(),
    })
    .unwrap()
assert_eq!(request_balance.balance, "900".to_string())
let request_balance = assetft
    .query_balance(&QueryBalanceRequest {
        account: receiver.address(),
        denom: denom.clone(),
    })
    .unwrap()
assert_eq!(request_balance.balance, "100".to_string());
```

## Versioning

The version of tx-test-tube is determined by the versions of its dependencies, TX and test-tube, as well as its own changes. The version is represented in the format A.B.C, where:

- A is the major version of TX,
- B is the minor version of tx-test-tube,
- C is the patch number of tx-test-tube itself.

When a new version of TX is released and contains breaking changes, we will also release breaking changes from test-tube if any and increment the major version of tx-test-tube. This way, it's clear that the new version of tx-test-tube is not backwards-compatible with previous versions.

When adding a new feature to tx-test-tube that is backward-compatible, the minor version number will be incremented.

When fixing bugs or making other changes that are `tx-test-tube` specific and backward-compatible, the patch number will be incremented.

Please review the upgrade guide for upgrading the package, in case of breaking changes.

It is important to note that we track the version of the package independent of the version of dependencies.
