#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
#![doc = include_str!("../../README.md")]

// use async_trait::async_trait;
use zcashd_rpc_messages::GetInfo;
use async_trait::async_trait;

/// A trait for providers of a zcashd-compatible RPC interface
#[async_trait]
pub trait RpcProvider {
    /// A provider-specific error type:
    type Error;

    /// Get the general status information about a node
    async fn get_info(&mut self) -> Result<GetInfo, Self::Error>;
}
