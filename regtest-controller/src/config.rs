use std::path::PathBuf;

/// Represents the configuration of a `zcashd` process and its configuration
#[derive(Clone, Debug)]
pub struct ZcashdConfig {
    /// The `-datadir` for `zcashd`
    pub datadir: PathBuf,
    /// A description of the process command
    pub command_description: String,
    /// The JSON-RPC auth cookie
    pub rpc_cookie: String,
    /// The URL of the JSON-RPC endpoint
    pub rpc_endpoint: String,
}
