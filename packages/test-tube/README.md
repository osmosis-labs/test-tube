# test-tube-core

[![test-tube-tx on crates.io](https://img.shields.io/crates/v/test-tube-tx.svg)](https://crates.io/crates/test-tube-tx) [![Docs](https://docs.rs/test-tube-tx/badge.svg)](https://docs.rs/test-tube-tx)

`test-tube` is a generic library for building testing environments for [CosmWasm](https://cosmwasm.com/) smart contracts. It allows you to test your smart contract logic against the actual Cosmos SDK chain's logic, which is written in Go, using Rust. This eliminates the need to write Go code or learn Go in order to test your smart contracts against the Cosmos SDK.

`test-tube-tx` is a modification of `test-tube` adjusted for the TX Blockchain.
