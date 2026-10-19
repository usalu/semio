//! ⏱️ Physical allocator witness: counts the bytes a closure births and frees on its thread, the global allocator of the test binary that mounts it.

use std::alloc::{GlobalAlloc, Layout};
use std::cell::Cell;

thread_local! {
    static OBSERVATION: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
}

struct PhysicalAllocator;

unsafe impl GlobalAlloc for PhysicalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { semio_framework_trace::HeapWitness.alloc(layout) };
        if !pointer.is_null() {
            let _ = OBSERVATION.try_with(|state| if let Some((births, frees)) = state.get() { state.set(Some((births + layout.size(), frees))) });
        }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let _ = OBSERVATION.try_with(|state| if let Some((births, frees)) = state.get() { state.set(Some((births, frees + layout.size()))) });
        unsafe { semio_framework_trace::HeapWitness.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: PhysicalAllocator = PhysicalAllocator;

pub fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    OBSERVATION.with(|state| { assert!(state.get().is_none()); state.set(Some((0, 0))); });
    let result = operation();
    let (births, frees) = OBSERVATION.with(|state| state.replace(None).unwrap());
    (result, births, frees)
}
