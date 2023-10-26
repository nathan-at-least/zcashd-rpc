#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! Manage a `zcashd` child process
//!
//! This crate is typically for high-level coordination of zcashd process, such as in integration tests. One example design choice reflecting that is to rely on [anyhow] for errors.

mod config;

pub use self::config::ZcashdConfig;

use anyhow::Context;
use anyhow_std::process::ExitStatus;
use std::future::Future;
use tokio::process::Child;

/// Create a temporary regtest `zcashd` process and pass it to `f`, then cleanup the node
pub async fn with_temporary_node<F, Fut>(f: F) -> anyhow::Result<()>
where
    F: FnOnce(ZcashdConfig) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let (child, config) = setup().await?;
    let res = f(config.clone())
        .await
        .context(format!("regtest data dir: {:?}", config.datadir.display()))
        .context(format!("regtest node pid: {:?}", child.id()))
        .context(format!(
            "regtest node command: {}",
            &config.command_description
        ))
        .context("-while executing with temporary zcashd regtest node".to_string());
    teardown(child, config).await?;
    res
}

async fn setup() -> anyhow::Result<(Child, ZcashdConfig)> {
    use tokio::process::Command;

    let datadir =
        tempfile::TempDir::with_prefix(format!("{}_regtest_node.", env!("CARGO_PKG_NAME")))?
            .into_path();

    tokio::fs::write(datadir.join("zcash.conf"), "").await?;

    let mut cmd = Command::new("zcashd");
    cmd.arg(format!("-datadir={}", datadir.display()));
    cmd.arg("-regtest");
    cmd.arg("-server");
    cmd.current_dir(&datadir);

    let command_description = format!("command: {:?}", cmd.as_std());

    let child = cmd.spawn().context(command_description.clone())?;

    let config = ZcashdConfig {
        datadir,
        command_description,
    };

    Ok((child, config))
}

async fn teardown(child: Child, config: ZcashdConfig) -> anyhow::Result<()> {
    let status = kill_without_error_context(child, config.command_description.clone())
        .await
        .context(config.command_description)?;

    if status.success() {
        tokio::fs::remove_dir_all(config.datadir).await?;
    }

    Ok(())
}

async fn kill_without_error_context(
    mut child: Child,
    command_description: String,
) -> anyhow::Result<ExitStatus> {
    child.start_kill()?;
    let status = child.wait().await?;
    Ok(ExitStatus::from((status, command_description)))
}
