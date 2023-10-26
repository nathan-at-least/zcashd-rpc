use crate::ZcashdConfig;
use anyhow::Context;
use indoc::indoc;
use tokio::process::Child;

pub(crate) async fn setup() -> anyhow::Result<(Child, ZcashdConfig)> {
    use tokio::process::Command;

    let datadir =
        tempfile::TempDir::with_prefix(format!("{}_regtest_node.", env!("CARGO_PKG_NAME")))?
            .into_path();

    tokio::fs::write(
        datadir.join("zcash.conf"),
        indoc! { r#"
            regtest=1
            server=1
        "# },
    )
    .await?;

    let mut cmd = Command::new("zcashd");
    cmd.arg(format!("-datadir={}", datadir.display()));
    cmd.current_dir(&datadir);

    let command_description = format!("command: {:?}", cmd.as_std());

    let child = cmd.spawn().context(command_description.clone())?;

    let config = ZcashdConfig {
        datadir,
        command_description,
    };

    Ok((child, config))
}
