//! This mod provides Zcash types used in `zcashd` RPC messages
//!
//! Ideally all of these types would be:
//! - newtypes which enforce value contraints appropriate to the type
//! - re-exports of crates like `zcash_primitives`
//!
//! For expediency many are currently type aliases to aide in distinguishing bare types.

/// An integer-precision amount of ZEC in units of Zatoshi
pub type Zat = u64;

/// An f64-precision amount of ZEC in units of ZEC
pub type Zec = f64;

/// A protocol version integer encoding
pub type VersionEncoding = u64;

/// A block height
pub type BlockHeight = u64;

/// A constant zero value/field
pub type ConstZero = u64;

/// A time
///
/// BUG: Document the time units
pub type Timestamp = i64;
