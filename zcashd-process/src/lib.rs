#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
//! Manage a `zcashd` child process
//!
//! This crate is typically for high-level coordination of zcashd process, such as in integration tests. One example design choice reflecting that is to rely on [anyhow] for errors.

mod procinfo;

pub use self::procinfo::ZcashdProcessInfo;
