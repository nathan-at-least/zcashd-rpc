# `zcashd-rpc-*` crates

A suite of rust crates for working with `zcashd`'s RPC interface or related clients and tools.

## `RpcProvider`

The cornerstone trait is `RpcProvider` found in `zcashd-rpc-provider` which is an `async-trait` that maps one-to-one to `zcashd` RPC method calls and responses.
