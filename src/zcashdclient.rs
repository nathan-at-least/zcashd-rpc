use crate::messages::GetInfo;
use crate::RpcProvider;
use async_trait::async_trait;
use jsonrpc::client::Client as JsonRpcClient;
use jsonrpc::http::simple_http::Error as JsonRpcError;

/// A `zcashd` client which implements [RpcProvider]
pub struct ZcashdClient {
    #[allow(dead_code)]
    client: JsonRpcClient,
}

impl ZcashdClient {
    /// Construct a new client
    pub fn new(url: &str, user: String, password: String) -> Result<Self, JsonRpcError> {
        let client = JsonRpcClient::simple_http(url, Some(user), Some(password))?;
        Ok(ZcashdClient { client })
    }
}

/// An error during method execution
pub struct ZcashdClientError {}

#[async_trait]
impl RpcProvider for ZcashdClient {
    type Error = ZcashdClientError;

    async fn get_info(&mut self) -> Result<GetInfo, Self::Error> {
        todo!()
    }
}
