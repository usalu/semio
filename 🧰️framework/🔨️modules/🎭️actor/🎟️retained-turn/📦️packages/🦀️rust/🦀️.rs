//! 🎟️ The original retained turn implementation is the canonical native package owner.
#![allow(async_fn_in_trait)]
#[path="../../🦀️.rs"]
pub mod retained_turn;
pub use retained_turn::*;

#[cfg(test)]
#[global_allocator]
static ACTOR_TEST_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
