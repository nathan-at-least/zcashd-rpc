# zcashd-rpc

A rust crate for consumers _and providers_ of the Zcashd JSON RPC interface.

The key interfaces is [RpcProvider]. Clients can use impls of this trait to automate usage of Zcashd-like systems. 

Because this is a trait, there can be multiple providers, including the straightfoward [ZcashdClient] which connects to Zcashd (or something emulating it) over tcp/ip. Other providers are possible such as "mock" implementations for testing, implementations for other nodes, middleware services, etc…
