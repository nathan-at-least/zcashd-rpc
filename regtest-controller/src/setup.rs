use crate::ZcashdConfig;
use anyhow::Context;
use indoc::indoc;
use std::path::Path;
use tokio::fs::File;
use tokio::process::Child;
use tokio::time::{sleep, Duration};

const POLL_INTERVAL: Duration = Duration::from_millis(127);

pub(crate) async fn setup() -> anyhow::Result<(Child, ZcashdConfig)> {
    let datadir =
        tempfile::TempDir::with_prefix(format!("{}_regtest_node.", env!("CARGO_PKG_NAME")))?
            .into_path();

    write_config(&datadir).await?;
    let (child, command_description, rpc_endpoint) = launch_child(&datadir)?;
    wait_for_rpc_service(&datadir).await?;
    let rpc_cookie = read_cookie(&datadir).await?;

    let config = ZcashdConfig {
        datadir,
        command_description,
        rpc_cookie,
        rpc_endpoint,
    };

    Ok((child, config))
}

async fn write_config(datadir: &Path) -> std::io::Result<()> {
    tokio::fs::write(
        datadir.join("zcash.conf"),
        indoc! { r#"
            regtest=1
            server=1
            debug=http
        "# },
    )
    .await
}

fn launch_child(datadir: &Path) -> anyhow::Result<(Child, String, String)> {
    use tokio::process::Command;

    let mut cmd = Command::new("zcashd");
    cmd.arg(format!("-datadir={}", datadir.display()));
    cmd.current_dir(datadir);

    let command_description = format!("command: {:?}", cmd.as_std());

    let child = cmd.spawn().context(command_description.clone())?;

    // BUG: Parse the endpoint from config or log output.
    Ok((
        child,
        command_description,
        "http://127.0.0.1:18232".to_string(),
    ))
}

async fn wait_for_rpc_service(datadir: &Path) -> std::io::Result<()> {
    use tokio::io::{self, AsyncBufReadExt, BufReader};

    let logpath = datadir.join("regtest").join("debug.log");
    let f = open_file_once_present(&logpath).await?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => {
                // Reached EOF, waiting for more data
                sleep(POLL_INTERVAL).await;
            }
            Ok(_) => {
                if line.contains("Binding RPC on address") {
                    return Ok(());
                }
                line.clear();
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Would block, wait before trying again
                sleep(POLL_INTERVAL).await;
            }
            Err(e) => {
                // An actual error occurred
                return Err(e);
            }
        }
    }
}

async fn open_file_once_present(path: &Path) -> std::io::Result<File> {
    use tokio::io::ErrorKind::NotFound;

    loop {
        match File::open(path).await {
            Err(ref e) if e.kind() == NotFound => {
                sleep(POLL_INTERVAL).await;
            }
            other => {
                return other;
            }
        }
    }
}

async fn read_cookie(datadir: &Path) -> anyhow::Result<String> {
    let cookie = tokio::fs::read_to_string(datadir.join("regtest").join(".cookie")).await?;
    Ok(cookie)
}
