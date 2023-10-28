#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! Manage a `zcashd` child process
//!
//! This crate is typically for high-level coordination of zcashd process, such as in integration tests. One example design choice reflecting that is to rely on [anyhow] for errors.

mod config;
mod setup;
mod teardown;

pub use self::config::ZcashdConfig;

use anyhow::Context;
use std::future::Future;

/// Create a temporary regtest `zcashd` process and pass it to `f`, then cleanup the node
pub async fn with_temporary_node<F, Fut>(f: F) -> anyhow::Result<()>
where
    F: FnOnce(ZcashdConfig) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let (child, config) = setup::setup().await?;

    let res = f(config.clone())
        .await
        .context(format!("regtest data dir: {:?}", config.datadir.display()))
        .context(format!("regtest node pid: {:?}", child.id()))
        .context(format!(
            "regtest node command: {}",
            &config.command_description
        ))
        .context("-while executing with temporary zcashd regtest node".to_string());

    teardown::teardown(child, config).await?;
    res
}
