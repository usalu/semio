//! ⏱️ Physical allocator witness: counts the bytes a closure births and frees on its thread through the one test allocator of the binary that mounts it.

pub fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    let (result, observation) = semio_framework_trace::observe_heap_allocations_on_this_thread(operation);
    (result, observation.requested_bytes, observation.released_bytes)
}
