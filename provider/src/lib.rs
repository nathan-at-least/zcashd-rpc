#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

//! Provides [RpcProvider], a trait representing a `zcashd`-compatible RPC service

// use async_trait::async_trait;
use async_trait::async_trait;
use zcashd_rpc_messages::GetInfo;

/// A trait for providers of a zcashd-compatible RPC interface
#[async_trait]
pub trait RpcProvider {
    /// A provider-specific error type:
    type Error;

    /// Get the general status information about a node
    async fn get_info(&mut self) -> Result<GetInfo, Self::Error>;
}
