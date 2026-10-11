//! 🔎️ The one global allocator every host unit test observes allocation through.

use std::cell::Cell;

thread_local! {
    static BYTES: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
    static CALLS: Cell<Option<usize>> = const { Cell::new(None) };
}

struct ObservedAllocator;
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let pointer = unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) };
        let _ = CALLS.try_with(|calls| if let Some(count) = calls.get() { calls.set(Some(count + 1)); });
        if !pointer.is_null() { let _ = BYTES.try_with(|state| if let Some((born, freed)) = state.get() { state.set(Some((born + layout.size(), freed))); }); }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: std::alloc::Layout) {
        let _ = BYTES.try_with(|state| if let Some((born, freed)) = state.get() { state.set(Some((born, freed + layout.size()))); });
        unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System, pointer, layout) }
    }
}
#[global_allocator]
static ALLOCATOR: ObservedAllocator = ObservedAllocator;

/// 🔢️ Counts allocation calls on this thread until [`allocations_end`].
pub(crate) fn allocations_start() { CALLS.with(|calls| calls.set(Some(0))); }

/// 🔢️ Stops counting and returns the allocation calls observed since [`allocations_start`].
pub(crate) fn allocations_end() -> usize { CALLS.with(|calls| calls.replace(None)).unwrap_or(0) }

/// 📏️ Runs `body` and returns its result with the bytes this thread allocated and freed meanwhile.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn measured<T>(body: impl FnOnce() -> T) -> (T, usize, usize) {
    BYTES.with(|state| { assert!(state.get().is_none()); state.set(Some((0, 0))); });
    let result = body();
    let (born, freed) = BYTES.with(|state| state.replace(None).unwrap());
    (result, born, freed)
}
