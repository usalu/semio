struct ObservedAllocator;
thread_local! { static RETIREMENT_ALLOCATION_EVENTS: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) }; }
fn retirement_allocation_event(requested: usize, released: usize) { let _ = RETIREMENT_ALLOCATION_EVENTS.try_with(|events| { if let Some((allocated, freed)) = events.get() { events.set(Some((allocated + requested, freed + released))); } }); }
pub(crate) fn observe_retirement_allocations<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    RETIREMENT_ALLOCATION_EVENTS.with(|events| { assert!(events.replace(Some((0, 0))).is_none()); });
    let result = operation();
    let events = RETIREMENT_ALLOCATION_EVENTS.with(|events| events.replace(None).unwrap());
    (result, events)
}
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{let pointer=unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{let pointer=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{let grown=unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)};if !grown.is_null(){retirement_allocation_event(size,layout.size());}grown}
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){retirement_allocation_event(0,layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;
