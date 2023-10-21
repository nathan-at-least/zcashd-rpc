# zcashd-rpc

A rust crate for consumers _and providers_ of the Zcashd JSON RPC interface.

The key interfaces is [RpcProvider]. Clients can use impls of this trait to automate usage of Zcashd-like systems. 
