# RELEASE PROCESS

This repository contains `test-tube` – the core and `osmosis-test-tube` – the osmosis bindings.

## Release `test-tube`

As `test-tube` is the core, we can simply release it by:
- Run `cargo test` to make sure that everything works
- Bump `test-tube` version in `packages/test-tube/Cargo.toml`
- Release new version of `test-tube` by running `cargo publish -p test-tube`
- Create github release with `test-tube-v<major>.<minor>.<patch>` as tag and title

## Release `osmosis-test-tube`

Releasing `osmosis-test-tube` is a bit more complicated as it depends on `test-tube` and `osmosis`.
- Run `SKIP_GIT_UPDATE=1 ./scripts/update-osmosis-test-tube.sh v<major>.<minor>.<patch>` using the version of `osmosis` you want to release with. This will update `replace` directives in `packages/osmosis-test-tube/libosmosistesttube/go.mod` to be compatible with the designated version of `osmosis`.
- in `go.mod` file, make sure that the line `github.com/osmosis-labs/osmosis/v<major> v<major>.<minor>.<patch>` is present and points to the correct version of `osmosis` and run `go mod tidy`
- With updated version, replace all existing import that use the previous `github.com/osmosis-labs/osmosis/v<major>` with the new one
- [Release `osmosis-std`](https://github.com/osmosis-labs/osmosis-rust/blob/main/RELEASE.md) with new osmosis version and update `osmosis-std` version in `packages/osmosis-test-tube/Cargo.toml`
- Check if there is any update on `test-tube` since lastest release of `osmosis-test-tube`.
    - Can check via `git diff --stat osmosis-test-tube-v<major>.<minor>.<patch> -- packages/test-tube/`
    - If there is any [release `test-tube`](#release-test-tube) first
    - Update `test-tube` version in `packages/osmosis-test-tube/Cargo.toml`
- Run `cargo test` to make sure that everything works
> *__Note__: if doc test fail, try `cargo clean` and run test again.*
- Bump `osmosis-test-tube` version in `packages/osmosis-test-tube/Cargo.toml`
- Release new version of `osmosis-test-tube` by running `cargo publish -p osmosis-test-tube`
- Create github release with `osmosis-test-tube-v<major>.<minor>.<patch>` as tag and title

