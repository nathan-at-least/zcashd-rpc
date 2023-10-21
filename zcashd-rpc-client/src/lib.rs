#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! A `zcashd` JSON-RPC client impl of [RpcProvider]

mod jsonrpc;

use async_trait::async_trait;
use zcashd_rpc::messages::GetInfo;
use zcashd_rpc::RpcProvider;

/// A `zcashd` client which implements [RpcProvider]
pub struct ZcashdClient {
    jsonclient: jsonrpc::Client,
}

impl ZcashdClient {
    /// Construct a new client
    pub fn new(endpoint: &str) -> Result<Self, jsonrpc::NewError> {
        Ok(ZcashdClient {
            jsonclient: jsonrpc::Client::new(endpoint)?,
        })
    }
}

#[async_trait]
impl RpcProvider for ZcashdClient {
    type Error = jsonrpc::CallError;

    async fn get_info(&mut self) -> Result<GetInfo, Self::Error> {
        self.jsonclient.call("getinfo", ()).await
    }
}
