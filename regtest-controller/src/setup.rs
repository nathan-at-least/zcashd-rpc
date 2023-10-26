use crate::ZcashdConfig;
use anyhow::Context;
use indoc::indoc;
use std::path::Path;
use tokio::process::Child;

pub(crate) async fn setup() -> anyhow::Result<(Child, ZcashdConfig)> {
    let datadir =
        tempfile::TempDir::with_prefix(format!("{}_regtest_node.", env!("CARGO_PKG_NAME")))?
            .into_path();

    write_config(&datadir).await?;
    let (child, command_description) = launch_child(&datadir)?;
    // TODO: block on rpc service and also include cookie in `ZcashdConfig`.

    let config = ZcashdConfig {
        datadir,
        command_description,
    };

    Ok((child, config))
}

async fn write_config(datadir: &Path) -> std::io::Result<()> {
    tokio::fs::write(
        datadir.join("zcash.conf"),
        indoc! { r#"
            regtest=1
            server=1
        "# },
    )
    .await
}

fn launch_child(datadir: &Path) -> anyhow::Result<(Child, String)> {
    use tokio::process::Command;

    let mut cmd = Command::new("zcashd");
    cmd.arg(format!("-datadir={}", datadir.display()));
    cmd.current_dir(datadir);

    let command_description = format!("command: {:?}", cmd.as_std());

    let child = cmd.spawn().context(command_description.clone())?;

    Ok((child, command_description))
}
