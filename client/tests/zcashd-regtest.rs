use zcashd_regtest_controller::ZcashdConfig;
use zcashd_rpc_client::ZcashdClient;

#[ignore] // This test is not yet functional.
#[tokio::test]
async fn get_info() -> anyhow::Result<()> {
    zcashd_regtest_controller::with_temporary_node(get_info_with_node).await
}

async fn get_info_with_node(node_config: ZcashdConfig) -> anyhow::Result<()> {
    println!("launched regtest node: {:#?}", &node_config);
    let mut client = ZcashdClient::new(&node_config.rpc_endpoint, &node_config.rpc_cookie)?;
    let info = client.wait_for_startup().await?;
    dbg!(info);
    Ok(())
}
