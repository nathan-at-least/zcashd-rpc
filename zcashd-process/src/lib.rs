#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! Manage a `zcashd` child process
//!
//! This crate is typically for high-level coordination of zcashd process, such as in integration tests. One example design choice reflecting that is to rely on [anyhow] for errors.

use anyhow::Context;
use anyhow_std::process::ExitStatus;
use std::future::Future;
use std::path::PathBuf;
use tokio::process::Child;

/// Create a temporary regtest `zcashd` process and pass it to `f`, then cleanup the node
pub async fn with_temporary_regtest_node<F, Fut>(f: F) -> anyhow::Result<()>
where
    F: FnOnce(ZcashdConfig) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let node = ZcashdProcess::launch_temporary_regtest_node().await?;
    let node_desc = format!("{:#?}", &node);
    let res = f(node.config.clone()).await.context(node_desc);

    println!("killing zcashd…");
    let status = node.kill().await?;
    println!("zcashd exit: {:?}", &status);
    status.exit_ok()?;
    res
}

/// Represents the configuration of a `zcashd` process and its configuration
#[derive(Clone, Debug)]
pub struct ZcashdConfig {
    /// The `-datadir` for `zcashd`
    pub datadir: PathBuf,
    /// A description of the process command
    pub command_description: String,
}

/// Represents the status of a `zcashd` process and its configuration
#[derive(Debug)]
pub struct ZcashdProcess {
    config: ZcashdConfig,
    child: Child,
}

impl ZcashdProcess {
    /// Launch a new temporary regtest node
    ///
    /// The `-datadir` is a temporary directory which is deleted when `self` is dropped.
    pub async fn launch_temporary_regtest_node() -> anyhow::Result<Self> {
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

        Ok(ZcashdProcess {
            config: ZcashdConfig {
                datadir,
                command_description,
            },
            child,
        })
    }

    /// Kill the child process and wait for the exit status
    ///
    /// If this succeeds, and the [ExitStatus::success](std::process::ExitStatus::success) then delete the datadir.
    pub async fn kill(self) -> anyhow::Result<ExitStatus> {
        let status =
            kill_without_error_context(self.child, self.config.command_description.clone())
                .await
                .context(self.config.command_description)?;

        if status.success() {
            tokio::fs::remove_dir_all(self.config.datadir).await?;
        }

        Ok(status)
    }
}

async fn kill_without_error_context(
    mut child: Child,
    command_description: String,
) -> anyhow::Result<ExitStatus> {
    child.start_kill()?;
    let status = child.wait().await?;
    Ok(ExitStatus::from((status, command_description)))
}

impl ZcashdConfig {
    /// Read the RPC cookie
    pub async fn read_cookie(&self) -> anyhow::Result<String> {
        let cookie =
            tokio::fs::read_to_string(self.datadir.join("regtest").join(".cookie")).await?;
        Ok(cookie)
    }
}
