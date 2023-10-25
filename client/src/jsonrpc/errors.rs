use serde::Deserialize;
use thiserror::Error;

pub type NewError = <reqwest::Url as std::str::FromStr>::Err;

#[derive(Debug, Error, derive_more::From)]
pub enum CallError {
    #[error("http client")]
    Reqwest(reqwest::Error),
    #[error("http protocol")]
    HttpError { status: String, body: String },
    #[error("jsonrpc protocol: {0}")]
    JsonRpcInvalid(JsonRpcInvalidReason),
    #[error("jsonrpc server error: {0}")]
    JsonRpcError(RpcError),
}

#[derive(Debug, Error)]
pub enum JsonRpcInvalidReason {
    #[error("unknown version: {0:?}")]
    UnknownVersion(String),
    #[error("unexpected id {found:?}, expecting {expected:?}")]
    UnexpectedId { expected: u64, found: u64 },
}

#[derive(Debug, Error, Deserialize)]
#[error("code {code}: {message} {data:#?}")]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
