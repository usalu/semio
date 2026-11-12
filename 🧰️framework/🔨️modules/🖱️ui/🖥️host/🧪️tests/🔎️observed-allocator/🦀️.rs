//! 🔎️ Every host unit test observes allocation through the one testing allocator.

use std::cell::Cell;

thread_local! {
    static CALLS: Cell<Option<usize>> = const { Cell::new(None) };
}

fn count_call(_layout: std::alloc::Layout) {
    let _ = CALLS.try_with(|calls| if let Some(count) = calls.get() { calls.set(Some(count + 1)); });
}

/// 🔢️ Counts allocation calls on this thread until [`allocations_end`].
pub(crate) fn allocations_start() {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| semio_framework_value_testing_allocator::install_allocation_tap(count_call, |_, _| {}));
    CALLS.with(|calls| calls.set(Some(0)));
}

/// ⏹️ Stops counting and returns the allocation calls observed since [`allocations_start`].
pub(crate) fn allocations_end() -> usize { CALLS.with(|calls| calls.replace(None)).unwrap_or(0) }

/// 📏️ Runs `body` and returns its result with the bytes this thread allocated and freed meanwhile.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn measured<T>(body: impl FnOnce() -> T) -> (T, usize, usize) {
    let (result, (born, freed)) = semio_framework_value_testing_allocator::observe_retirement_allocations(body);
    (result, born, freed)
}
