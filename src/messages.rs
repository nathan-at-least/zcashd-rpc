//! The messages in [RpcProvider](crate::RpcProvider) requests and responses

/// General status information for a node
#[derive(Debug, serde::Deserialize)]
pub struct GetInfo {}
