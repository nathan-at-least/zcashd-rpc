use anyhow::Context;
use std::process::ExitStatus;
use tempdir::TempDir;
use tokio::process::Child;

/// Represents the status of a `zcashd` process and its configuration
#[derive(Debug)]
pub struct ZcashdProcessInfo {
    datadir: TempDir,
    command: String,
    child: Child,
}

impl ZcashdProcessInfo {
    /// Launch a new temporary regtest node
    ///
    /// The `-datadir` is a temporary directory which is deleted when `self` is dropped.
    pub fn launch_temporary_regtest_node() -> anyhow::Result<Self> {
        use tokio::process::Command;

        let datadir = tempdir::TempDir::new(&format!("{}_regtest_node.", env!("CARGO_PKG_NAME")))?;

        let mut cmd = Command::new("zcashd");
        cmd.arg("-regtest");
        cmd.arg(format!("-datadir={}", datadir.as_ref().display()));
        cmd.current_dir(&datadir);

        let command = format!("command: {:?}", &cmd);

        let child = cmd.spawn().context(&command)?;

        Ok(ZcashdProcessInfo {
            datadir,
            command,
            child,
        })
    }

    /// Kill the child process and wait for the exit status
    pub async fn kill(self) -> anyhow::Result<ExitStatus> {
        kill_without_error_context(self.child)
            .await
            .context(self.command)
    }
}

async fn kill_without_error_context(mut child: Child) -> anyhow::Result<ExitStatus> {
    child.start_kill()?;
    let status = child.wait().await?;
    Ok(status)
}
