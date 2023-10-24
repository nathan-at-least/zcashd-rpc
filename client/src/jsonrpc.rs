mod errors;

use std::fmt;

pub use self::errors::{CallError, NewError};

pub(crate) struct Client {
    client: reqwest::Client,
    endpoint: reqwest::Url,
    idgen: std::ops::Range<u64>,
}

impl Client {
    pub(crate) fn new(endpoint: &str) -> Result<Self, NewError> {
        let client = reqwest::Client::new();
        let endpoint = endpoint.parse()?;
        let idgen = 0..u64::MAX;

        Ok(Client {
            client,
            endpoint,
            idgen,
        })
    }

    pub(crate) fn endpoint(&self) -> &reqwest::Url {
        &self.endpoint
    }

    pub(crate) async fn call<P, R>(&mut self, method: &str, params: P) -> Result<R, CallError>
    where
        P: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let id = self.idgen.next().expect("u64 request id overflow!");

        let response = self
            .client
            .post(self.endpoint.clone())
            .json(&RequestEnvelope {
                method,
                params,
                id,
                jsonrpc: JSON_RPC_VERSION,
            })
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let status = status.to_string();
            let body = response.text().await?;
            return Err(CallError::HttpError { status, body });
        }

        let response: Response<R> = response.json().await?;

        {
            use self::errors::JsonRpcInvalidReason::*;
            use ResultResponse::*;

            if response.jsonrpc != JSON_RPC_VERSION {
                Err(UnknownVersion(response.jsonrpc))?
            } else if response.id != id {
                Err(UnexpectedId {
                    expected: id,
                    found: response.id,
                })?
            } else {
                match response.result {
                    Result { result } => Ok(result),
                    Error { error } => Err(error)?,
                }
            }
        }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typename = std::any::type_name::<Self>();
        let endpoint = &self.endpoint;
        write!(f, "{typename}[{endpoint}]")
    }
}

const JSON_RPC_VERSION: &str = "2.0";

#[derive(Debug, serde::Serialize)]
struct RequestEnvelope<'a, P> {
    method: &'a str,
    params: P,
    id: u64,
    jsonrpc: &'static str,
}

#[derive(Debug, serde::Deserialize)]
struct Response<T> {
    jsonrpc: String,
    id: u64,
    #[serde(flatten)]
    result: ResultResponse<T>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum ResultResponse<T> {
    Result { result: T },
    Error { error: self::errors::RpcError },
}
