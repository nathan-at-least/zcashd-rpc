#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! A `zcashd` JSON-RPC client impl of [RpcProvider]

mod jsonrpc;

use async_trait::async_trait;
use std::fmt;
use zcashd_rpc_messages::GetInfo;
use zcashd_rpc_provider::RpcProvider;

/// A `zcashd` client which implements [RpcProvider]
pub struct ZcashdClient {
    jsonclient: jsonrpc::Client,
}

impl ZcashdClient {
    /// Construct a new client
    pub fn new(endpoint: &str, cookie: &str) -> Result<Self, jsonrpc::NewError> {
        Ok(ZcashdClient {
            jsonclient: jsonrpc::Client::new(endpoint, cookie)?,
        })
    }

    /// Poll `self.get_info` and as long this produces a known startup-message error, wait and retry
    ///
    /// Other errors are propagated
    pub async fn wait_for_startup(&mut self) -> Result<GetInfo, jsonrpc::CallError> {
        use tokio::time::{sleep, Duration};

        const POLL_INTERVAL: Duration = Duration::from_millis(113);
        const RPC_IN_WARMUP: i64 = -28; // Ref: `zcash/src/rpc/protocol.h`

        loop {
            match self.get_info().await {
                Err(jsonrpc::CallError::JsonRpcError(e)) if e.code == RPC_IN_WARMUP => {
                    sleep(POLL_INTERVAL).await;
                }
                other => {
                    return other;
                }
            }
        }
    }
}

#[async_trait]
impl RpcProvider for ZcashdClient {
    type Error = jsonrpc::CallError;

    async fn get_info(&mut self) -> Result<GetInfo, Self::Error> {
        self.jsonclient.call("getinfo", ()).await
    }
}

impl fmt::Debug for ZcashdClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typename = std::any::type_name::<Self>();
        let endpoint = &self.jsonclient.endpoint();
        write!(f, "{typename}[{endpoint}]")
    }
}
