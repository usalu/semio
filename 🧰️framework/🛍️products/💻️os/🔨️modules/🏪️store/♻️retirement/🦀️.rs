//! 🏪️ Store-specific retirement implementations over the neutral owner contract.
#[path = "📦️backing/🦀️.rs"]
pub mod resident_backing;
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence};
use std::mem::ManuallyDrop;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
