//! 🧫️ Test-only cumulative allocation requests with inclusive, stack-owned observation scopes.
use std::{alloc::{GlobalAlloc, Layout}, cell::Cell, ptr::null_mut};
use semio_framework_trace::HeapWitness;

struct Scope { parent: *mut Scope, requested: usize, released: usize, overflow: bool }
std::thread_local! { static ACTIVE: Cell<*mut Scope> = const { Cell::new(null_mut()) }; }
struct Restore(*mut Scope);
impl Drop for Restore {
    fn drop(&mut self) { let _ = ACTIVE.try_with(|active| active.set(self.0)); }
}
fn record(bytes: usize) {
    let _ = ACTIVE.try_with(|active| {
        let mut scope = active.get();
        while !scope.is_null() {
            unsafe {
                match (*scope).requested.checked_add(bytes) { Some(total) => (*scope).requested = total, None => (*scope).overflow = true }
                scope = (*scope).parent;
            }
        }
    });
}
fn release(bytes: usize) {
    let _ = ACTIVE.try_with(|active| {
        let mut scope = active.get();
        while !scope.is_null() {
            unsafe {
                match (*scope).released.checked_add(bytes) { Some(total) => (*scope).released = total, None => (*scope).overflow = true }
                scope = (*scope).parent;
            }
        }
    });
}

/// 🧮️ Preserves cumulative requested-byte measurement while reusing the first-party [`HeapWitness`].
pub(crate) struct RequestedAllocator;
unsafe impl GlobalAlloc for RequestedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { record(layout.size()); unsafe { HeapWitness.alloc(layout) } }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { record(layout.size()); unsafe { HeapWitness.alloc_zeroed(layout) } }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 { record(size); let result = unsafe { HeapWitness.realloc(pointer, layout, size) }; if !result.is_null() { release(layout.size()); } result }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) { release(layout.size()); unsafe { HeapWitness.dealloc(pointer, layout) } }
}

/// 📏️ Measures this thread's requests, includes nested scopes, and restores the parent during unwind.
pub(crate) fn observe<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    let (result, requested, _) = observe_backing(operation);
    (result, requested)
}

/// 👁️ Reads the active request counter without allocating or changing the observation scope.
pub(crate) fn observed_requested_bytes() -> Option<usize> {
    ACTIVE.with(|active| {
        let scope = active.get();
        if scope.is_null() { None } else { Some(unsafe { (*scope).requested }) }
    })
}

/// ♻️ Measures actual request and released layout bytes with the same inclusive observation stack.
pub(crate) fn observe_backing<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    let parent = ACTIVE.with(Cell::get);
    let mut scope = Scope { parent, requested: 0, released: 0, overflow: false };
    let restore = Restore(parent);
    ACTIVE.with(|active| active.set(&mut scope));
    let result = operation();
    drop(restore);
    assert!(!scope.overflow, "requested allocation observation overflow");
    (result, scope.requested, scope.released)
}

#[path = "🧪️tests/🦀️.rs"]
mod tests;
