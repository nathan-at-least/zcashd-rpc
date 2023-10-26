use crate::ZcashdConfig;
use anyhow::Context;
use anyhow_std::process::ExitStatus;
use tokio::process::Child;

pub(crate) async fn teardown(child: Child, config: ZcashdConfig) -> anyhow::Result<()> {
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
