use zcashd_rpc_client::ZcashdClient;
use zcashd_rpc_provider::RpcProvider;

#[tokio::test]
async fn get_info() -> anyhow::Result<()> {
    zcashd_process::with_temporary_regtest_node(get_info_with_node).await
}

async fn get_info_with_node(node_config: zcashd_process::ZcashdConfig) -> anyhow::Result<()> {
    println!(
        "launched regtest node with datadir {:?}",
        node_config.datadir.display()
    );
    let mut client = ZcashdClient::new("http://127.0.0.1:18232")?;
    let info = client.get_info().await?;
    dbg!(info);
    Ok(())
}
