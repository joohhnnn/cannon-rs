//! Contains the memory implementation for the Cannon FPVM.

pub(crate) mod page;
pub use page::CachedPage;

mod map_memory;
pub use map_memory::{Memory, MemoryReader};
