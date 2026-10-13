//! 🔭️ Test-only allocation observer shared by every `🧊️3d` crate's test binary: the one `#[global_allocator]` of that
//! binary, counting the system heap a closure reserves and releases (`observe_tessellation_system`) plus the exact
//! sample and mesh pointers a tessellation retirement law tracks. Mounted under `#[cfg(test)]` by each crate root, never
//! compiled into a library.

pub(crate) struct TessellationAllocationObserver;
std::thread_local! {
    static SYSTEM_OBSERVING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static SYSTEM_CAPACITY: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static SYSTEM_RELEASE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
pub(crate) fn observe_tessellation_system<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    SYSTEM_CAPACITY.with(|value| value.set(0));
    SYSTEM_RELEASE.with(|value| value.set(0));
    SYSTEM_OBSERVING.with(|value| value.set(true));
    let result = operation();
    SYSTEM_OBSERVING.with(|value| value.set(false));
    (result, (SYSTEM_CAPACITY.with(std::cell::Cell::get), SYSTEM_RELEASE.with(std::cell::Cell::get)))
}
pub(crate) static OBSERVED_SAMPLE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static OBSERVED_MESH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static SAMPLE_RELEASES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static MESH_RELEASES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static OBSERVED_RELEASE_BYTES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[global_allocator]
static TESSELLATION_ALLOCATOR: TessellationAllocationObserver = TessellationAllocationObserver;
unsafe impl std::alloc::GlobalAlloc for TessellationAllocationObserver {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let pointer = unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) };
        if !pointer.is_null() && SYSTEM_OBSERVING.with(std::cell::Cell::get) { SYSTEM_CAPACITY.with(|value| value.set(value.get() + layout.size())); }
        pointer
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: std::alloc::Layout) {
        use std::sync::atomic::Ordering::SeqCst;
        if SYSTEM_OBSERVING.with(std::cell::Cell::get) { SYSTEM_RELEASE.with(|value| value.set(value.get() + layout.size())); }
        if OBSERVED_SAMPLE.compare_exchange(pointer as usize,0,SeqCst,SeqCst).is_ok() { SAMPLE_RELEASES.fetch_add(1,SeqCst);OBSERVED_RELEASE_BYTES.fetch_add(layout.size(),SeqCst); }
        if OBSERVED_MESH.compare_exchange(pointer as usize,0,SeqCst,SeqCst).is_ok() { MESH_RELEASES.fetch_add(1,SeqCst);OBSERVED_RELEASE_BYTES.fetch_add(layout.size(),SeqCst); }
        unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout); }
    }
}
