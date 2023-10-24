#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! Manage a `zcashd` child process
//!
//! This crate is typically for high-level coordination of zcashd process, such as in integration tests. One example design choice reflecting that is to rely on [anyhow] for errors.

use anyhow::Context;
use std::process::ExitStatus;
use tempfile::TempDir;
use tokio::process::Child;

/// Represents the status of a `zcashd` process and its configuration
#[derive(Debug)]
pub struct ZcashdProcessInfo {
    /// The purpose of this field is merely to delegate to `TempDir::drop` when `Self` drops, so it is not quite "unused". The unused warning could be considered a false-positive for this case.
    #[allow(unused)]
    datadir: TempDir,
    command_description: String,
    child: Child,
}

impl ZcashdProcessInfo {
    /// Launch a new temporary regtest node
    ///
    /// The `-datadir` is a temporary directory which is deleted when `self` is dropped.
    pub fn launch_temporary_regtest_node() -> anyhow::Result<Self> {
        use tokio::process::Command;

        let datadir =
            tempfile::TempDir::with_prefix(format!("{}_regtest_node.", env!("CARGO_PKG_NAME")))?;

        let mut cmd = Command::new("zcashd");
        cmd.arg("-regtest");
        cmd.arg(format!("-datadir={}", datadir.as_ref().display()));
        cmd.current_dir(&datadir);

        let command_description = format!("command: {:?}", &cmd);

        let child = cmd.spawn().context(command_description.clone())?;

        Ok(ZcashdProcessInfo {
            datadir,
            command_description,
            child,
        })
    }

    /// Kill the child process and wait for the exit status
    pub async fn kill(self) -> anyhow::Result<ExitStatus> {
        kill_without_error_context(self.child)
            .await
            .context(self.command_description)
    }
}

async fn kill_without_error_context(mut child: Child) -> anyhow::Result<ExitStatus> {
    child.start_kill()?;
    let status = child.wait().await?;
    Ok(status)
}
