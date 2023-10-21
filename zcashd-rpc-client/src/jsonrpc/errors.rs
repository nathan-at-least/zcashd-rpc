use serde::Deserialize;

pub type NewError = <reqwest::Url as std::str::FromStr>::Err;

#[derive(Debug, derive_more::From)]
pub enum CallError {
    Reqwest(reqwest::Error),
    HttpError { status: String, body: String },
    JsonRpcInvalid(JsonRpcInvalidReason),
    JsonRpcError(RpcError),
}

#[derive(Debug)]
pub enum JsonRpcInvalidReason {
    UnknownVersion(String),
    UnexpectedId { expected: u64, found: u64 },
}

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
