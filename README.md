# `zcashd-rpc`

A rust crate for consumers _and providers_ of the Zcashd JSON RPC interface.

The key interfaces is [RpcProvider]. Clients can use impls of this trait to automate usage of Zcashd-like systems. 

## Related crates

- `zcashd-rpc-client`: A JSON RPC client implementation which can be used to connect to `zcashd` or services that emulate it. 
- `zcashd-rpc-fuzzer`: An [RpcProvider] impl that returns structurally correct, yet random replies.
