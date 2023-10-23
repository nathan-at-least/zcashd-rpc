//! The messages in [RpcProvider](crate::RpcProvider) requests and responses

pub mod zcash_types;

use crate::zcash_types::{BlockHeight, ConstZero, Timestamp, VersionEncoding, Zat};

/// General status information for a node
///
/// # Wallet Information
///
/// Nodes have three possible states of wallet support:
/// - wallet not enabled/supported (e.g. compiled out of `zcashd` or not implemented in another node): [GetInfo::wallet_info] is `None`
/// - wallet is supported but not "active" (e.g. compiled into `zcashd` but not activated at runtime): [WalletInfo::active_info] is `None`
/// - wallet is supported and active (e.g. supported and active in the node instance): both fields above are present.
///
/// Transcribed from `zcash/src/rpc/misc.cpp` `getinfo`
#[derive(Debug, serde::Deserialize)]
pub struct GetInfo {
    /// The provider version integer
    pub version: VersionEncoding,
    /// The provider build description
    pub build: String,
    /// The provider sub-version description
    pub subversion: String,
    /// The provider's supported protocol version
    pub protocolversion: VersionEncoding,
    /// Wallet status info, if the node supports a wallet
    #[serde(flatten)]
    pub wallet_info: Option<WalletInfo>,
    /// The best-chain blockheight which the node currently knows of
    pub blocks: BlockHeight,
    /// **Deprecated**; always 0
    pub timeoffset: ConstZero,
    /// The number of peer connections
    pub connections: u64,
    /// The proxy connection host:port or empty string
    pub proxy: String,
    /// **To be depecreated**; Whether or not the node is connected to the testnet
    pub testnet: bool,
    /// The fee rate for relaying transactions
    pub relayfee: Zat,
    /// The "statusbar" (e.g. sticky / pinned) errors description
    pub errors: String,
    /// The update time of [GetInfo::errors]
    pub errorstimestamp: Timestamp,
}

/// Status information about a node's wallet
///
/// Transcribed from `zcash/src/rpc/misc.cpp` `getinfo`
#[derive(Debug, serde::Deserialize)]
pub struct WalletInfo {
    /// The wallet engine version
    pub walletversion: VersionEncoding,
    /// The wallet engine version
    pub balance: Zat,
    /// The current fee per kb
    pub paytxfee: Zat,
    /// Wallet fields for an active wallet
    #[serde(flatten)]
    pub active_info: Option<ActiveWalletInfo>,
}

/// Status information about a node's active wallet
///
/// Transcribed from `zcash/src/rpc/misc.cpp` `getinfo`
#[derive(Debug, serde::Deserialize)]
pub struct ActiveWalletInfo {
    /// The timestamp of the oldest key in the keypool
    pub keypoololdest: Timestamp,
    /// The number of keys in the keypool
    pub keypoolsize: u64,
}
