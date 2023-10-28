use zcashd_regtest_controller::ZcashdConfig;
use zcashd_rpc_client::ZcashdClient;

#[tokio::test]
async fn get_info() -> anyhow::Result<()> {
    zcashd_regtest_controller::with_temporary_node(get_info_with_node).await
}

async fn get_info_with_node(node_config: ZcashdConfig) -> anyhow::Result<()> {
    println!("launched regtest node: {:#?}", &node_config);
    let mut client = ZcashdClient::new(&node_config.rpc_endpoint, &node_config.rpc_cookie)?;
    let info = client.wait_for_startup().await?;

    // Show the full info for test failures:
    dbg!(&info);

    // Verify a field which should rarely ever change:
    assert_eq!(info.protocolversion, 170100);
    // Verify all of the deterministic fields:
    assert!(info.wallet_info.is_none());
    assert_eq!(info.blocks, 0);
    assert_eq!(info.timeoffset, 0);
    assert_eq!(info.connections, 0);
    assert_eq!(info.proxy, "");
    assert_eq!(info.testnet, false);
    assert_eq!(info.relayfee, 1e-6);
    assert_eq!(info.errors, "");
    // The other fields are likely to change (e.g. timestamps, node version, …)

    Ok(())
}
