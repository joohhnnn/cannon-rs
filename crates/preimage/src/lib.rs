#![doc = include_str!("../README.md")]
#![allow(dead_code, unused_variables)]

mod oracle;
// pub use oracle::{OracleClient, OracleServer};

mod traits;
pub use traits::{Hint, Hinter, Key, Oracle};

mod types;
pub use types::{Keccak256Key, KeyType, LocalIndexKey, PreimageGetter};