use crate::messages::GetInfo;
use crate::{jsonrpc, RpcProvider};
use async_trait::async_trait;

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
