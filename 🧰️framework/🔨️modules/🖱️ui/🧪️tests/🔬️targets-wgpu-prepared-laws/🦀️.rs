//! 🧪️ Companion white-box laws of `semio-framework-ui-prepared`.

#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]

#[path = "../🔬️targets-wgpu-prepared-unit/🦀️.rs"]
mod unit;

#[global_allocator]
static ALLOCATOR: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(operation);
    assert!(!heap.overflowed);
    (result, heap.requested_bytes, heap.released_bytes)
}
