#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! Manage a `zcashd` child process
//!
//! This crate is typically for high-level coordination of zcashd process, such as in integration tests. One example design choice reflecting that is to rely on [anyhow] for errors.

use anyhow::Context;
use anyhow_std::process::ExitStatus;
use std::path::PathBuf;
use tokio::process::Child;

/// Represents the status of a `zcashd` process and its configuration
#[derive(Debug)]
pub struct ZcashdProcess {
    datadir: PathBuf,
    command_description: String,
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
        cmd.arg("-regtest");
        cmd.arg(format!("-datadir={}", datadir.display()));
        cmd.current_dir(&datadir);

        let command_description = format!("command: {:?}", cmd.as_std());

        let child = cmd.spawn().context(command_description.clone())?;

        Ok(ZcashdProcess {
            datadir,
            command_description,
            child,
        })
    }

    /// Kill the child process and wait for the exit status
    ///
    /// If this succeeds, and the [ExitStatus::success] then delete the datadir.
    pub async fn kill(self) -> anyhow::Result<ExitStatus> {
        let status = kill_without_error_context(self.child, self.command_description.clone())
            .await
            .context(self.command_description)?;

        if status.success() {
            tokio::fs::remove_dir_all(self.datadir).await?;
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
