mod errors;

use std::fmt;

pub use self::errors::{CallError, NewError};

pub(crate) struct Client {
    client: reqwest::Client,
    endpoint: reqwest::Url,
    authheader: String,
    idgen: std::ops::Range<u64>,
}

impl Client {
    pub(crate) fn new(endpoint: &str, cookie: &str) -> Result<Self, NewError> {
        let client = reqwest::Client::new();
        let endpoint = endpoint.parse()?;
        let authheader = format!("Basic {}", base64_encode(cookie));
        let idgen = 0..u64::MAX;

        Ok(Client {
            client,
            endpoint,
            authheader,
            idgen,
        })
    }

    pub(crate) fn endpoint(&self) -> &reqwest::Url {
        &self.endpoint
    }

    pub(crate) async fn call<P, R>(&mut self, method: &str, params: P) -> Result<R, CallError>
    where
        P: serde::Serialize,
        R: serde::Serialize + serde::de::DeserializeOwned,
    {
        let id = self.idgen.next().expect("u64 request id overflow!");

        let response = self
            .client
            .post(self.endpoint.clone())
            .header("Authorization", &self.authheader)
            .json(&RequestEnvelope { method, params, id })
            .send()
            .await?;

        let status = response.status();
        if status.is_success() || status.is_server_error() {
            use self::errors::JsonRpcInvalidReason::*;

            let response: Response<R> = response.json().await?;

            if response.id != id {
                Err(UnexpectedId {
                    expected: id,
                    found: response.id,
                })?
            } else {
                response.into_result()
            }
        } else {
            let status = status.to_string();
            let body = response.text().await?;
            Err(CallError::HttpError { status, body })
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

#[derive(Debug, serde::Serialize)]
struct RequestEnvelope<'a, P> {
    method: &'a str,
    params: P,
    id: u64,
}

#[derive(Debug, serde::Deserialize)]
struct Response<T> {
    id: u64,
    result: Option<T>,
    error: Option<self::errors::RpcError>,
}

impl<T> Response<T>
where
    T: serde::Serialize,
{
    fn into_result(self) -> Result<T, CallError> {
        use self::errors::JsonRpcInvalidReason::{BothErrorAndResult, NeitherErrorNorResult};

        match (self.result, self.error) {
            (Some(res), None) => Ok(res),
            (None, Some(err)) => Err(err)?,
            (None, None) => Err(NeitherErrorNorResult)?,
            (Some(res), Some(err)) => Err(BothErrorAndResult {
                error: err,
                result: serde_json::to_string(&res)
                    .unwrap_or_else(|e| format!("[internal json serialization failure: {e}]")),
            })?,
        }
    }
}

fn base64_encode(s: &str) -> String {
    use base64::Engine;

    base64::engine::general_purpose::URL_SAFE.encode(s)
}
