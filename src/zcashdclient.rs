use crate::messages::get_info;
use crate::RpcProvider;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

/// A `zcashd` client which implements [RpcProvider]
pub struct ZcashdClient {}

/// An error during method execution
pub struct ZcashdClientError {}

impl RpcProvider for ZcashdClient {}

impl Service<get_info::Request> for ZcashdClient {
    type Response = get_info::Response;
    type Error = ZcashdClientError;
    // FIXME: Replace with a concrete type?
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, _req: get_info::Request) -> Self::Future {
        todo!()
    }
}
