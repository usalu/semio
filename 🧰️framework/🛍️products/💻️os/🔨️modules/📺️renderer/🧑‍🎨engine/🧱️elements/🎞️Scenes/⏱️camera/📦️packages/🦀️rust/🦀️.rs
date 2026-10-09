//! ⏱️ Owns the original camera storage module and its complete physical corpus in place.
#[path = "../../🦀️.rs"]
mod storage;
pub use storage::*;

#[cfg(test)]
#[global_allocator]
static CAMERA_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
