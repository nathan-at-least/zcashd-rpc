use zcashd_process::ZcashdProcess;
use zcashd_rpc_client::ZcashdClient;
use zcashd_rpc_provider::RpcProvider;

#[tokio::test]
async fn get_info() -> anyhow::Result<()> {
    let node = ZcashdProcess::launch_temporary_regtest_node().await?;
    println!("launched: {:#?}", &node);
    let mut client = ZcashdClient::new("http://127.0.0.1:8232")?;
    let info = client.get_info().await?;
    dbg!(info);
    let status = node.kill().await?;
    status.exit_ok()?;
    Ok(())
}
