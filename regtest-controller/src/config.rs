use std::path::PathBuf;

/// Represents the configuration of a `zcashd` process and its configuration
#[derive(Clone, Debug)]
pub struct ZcashdConfig {
    /// The `-datadir` for `zcashd`
    pub datadir: PathBuf,
    /// A description of the process command
    pub command_description: String,
}

impl ZcashdConfig {
    /// Read the RPC cookie
    pub async fn read_cookie(&self) -> anyhow::Result<String> {
        let cookie =
            tokio::fs::read_to_string(self.datadir.join("regtest").join(".cookie")).await?;
        Ok(cookie)
    }
}
