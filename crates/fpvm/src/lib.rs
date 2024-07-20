#![doc = include_str!("../README.md")]

mod mips;
pub use mips::InstrumentedState;

mod memory;
pub use memory::{CachedPage, Memory, MemoryReader};

mod utils;
pub use utils::{
    patch::{load_elf, patch_go, patch_stack, MultiReader},
    ser,
};

mod types;
pub use types::{
    state_hash, Address, Fd, Gindex, Page, PageIndex, State, StateWitness, StepWitness, VMStatus,
    STATE_WITNESS_SIZE,
};

#[cfg(any(feature = "test-utils", test))]
pub mod test_utils;

#[cfg(feature = "evm")]
pub mod evm;
