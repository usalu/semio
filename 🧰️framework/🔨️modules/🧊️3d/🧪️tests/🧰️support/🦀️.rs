//! 🔭️ Test-only allocation observation shared by every `🧊️3d` crate's test binary: it counts the system heap a closure reserves and releases (`observe_tessellation_system`) plus the exact sample and mesh pointers a tessellation retirement law tracks through the pointer tap of the one testing allocator. Mounted under `#[cfg(test)]` by each crate root, never compiled into a library.

use semio_framework_value_testing_allocator::install_allocation_tap;

/// 🧮️ Weighs the system heap a closure requests and releases on this thread through the one testing allocator.
pub fn observe_tessellation_system<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    watch_released_pointers();
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(operation);
    (result, (heap.requested_bytes, heap.released_bytes))
}
/// 🪝️ Installs the released-pointer tap that feeds the observed sample and mesh counters; idempotent.
pub fn watch_released_pointers() {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| install_allocation_tap(|_| {}, released_pointer));
}
pub static OBSERVED_SAMPLE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub static OBSERVED_MESH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub static SAMPLE_RELEASES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub static MESH_RELEASES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub static OBSERVED_RELEASE_BYTES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
fn released_pointer(pointer: *mut u8, layout: std::alloc::Layout) {
    use std::sync::atomic::Ordering::SeqCst;
    if OBSERVED_SAMPLE.compare_exchange(pointer as usize,0,SeqCst,SeqCst).is_ok() { SAMPLE_RELEASES.fetch_add(1,SeqCst);OBSERVED_RELEASE_BYTES.fetch_add(layout.size(),SeqCst); }
    if OBSERVED_MESH.compare_exchange(pointer as usize,0,SeqCst,SeqCst).is_ok() { MESH_RELEASES.fetch_add(1,SeqCst);OBSERVED_RELEASE_BYTES.fetch_add(layout.size(),SeqCst); }
}
