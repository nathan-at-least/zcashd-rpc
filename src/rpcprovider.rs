// use async_trait::async_trait;
use crate::messages::get_info;
use tower::Service;

/// FIXME
pub trait RpcProvider: Service<get_info::Request, Response = get_info::Response> {}
