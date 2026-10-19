//! 🚪️ The original shared tick payload helper and its original source fixture execute at the public package boundary.
#[global_allocator]
static ORIGINAL_TICK_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[path="../../🦀️.rs"]
mod original_payload;
