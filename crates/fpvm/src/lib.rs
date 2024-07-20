// #![doc = include_str!("../README.md")]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod memory;
pub use memory::{CachedPage, Memory, MemoryReader};

mod traits;
pub use self::traits::PreimageOracle;

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

mod mips;
pub use mips::InstrumentedState;

pub mod test_utils;

#[cfg(feature = "evm")]
pub mod evm;
