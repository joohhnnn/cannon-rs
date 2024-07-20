//! This module contains the various traits used in this crate.

use anyhow::Result;
use preimage_oracle::Hint;

/// A [PreimageOracle] is a trait describing the functionality of a preimage
/// server.
pub trait PreimageOracle {
    /// Insert the given preimage into the oracle.
    ///
    /// ### Takes
    /// - `value`: The preimage to insert.
    fn hint(&mut self, value: impl Hint) -> Result<()>;

    /// Fetch the preimage for the given key.
    ///
    /// ### Takes
    /// - `key`: The keccak digest to fetch the preimage for.
    ///
    /// ### Returns
    /// - `Ok(Some(preimage))`: The preimage for the given key.
    /// - `Ok(None)`: The preimage for the given key does not exist.
    /// - `Err(_)`: An error occurred while fetching the preimage.
    fn get(&mut self, key: [u8; 32]) -> Result<Vec<u8>>;
}
