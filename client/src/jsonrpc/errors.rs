use serde::Deserialize;
use thiserror::Error;

pub type NewError = <reqwest::Url as std::str::FromStr>::Err;

#[derive(Debug, Error, derive_more::From)]
pub enum CallError {
    #[error("http protocol error: {0}")]
    Reqwest(reqwest::Error),
    #[error("http server error: [{status}] {body}")]
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
    #[error("neither error nor result fields in reponse")]
    NeitherErrorNorResult,
    #[error("both error & result present: error {error:?} and result {result:?}")]
    BothErrorAndResult { error: RpcError, result: String },
}

#[derive(Debug, Error, Deserialize)]
#[error("code {code}: {message} {data:#?}")]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
