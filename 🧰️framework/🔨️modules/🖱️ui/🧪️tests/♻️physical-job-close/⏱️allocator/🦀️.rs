//! ⏱️ Physical allocator witness: counts the bytes a closure births and frees on its thread through the one test allocator of the binary that mounts it, nesting inside every fixture turn that observes its own Source.

pub fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    let (result, (born, freed)) = semio_framework_value_testing_allocator::observe_retirement_allocations(operation);
    (result, born, freed)
}
