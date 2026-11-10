//! 💾️ Original physical allocation receipt observer shared by true native owner suites.

thread_local!{static REFUSED_ALLOCATION:std::cell::Cell<Option<usize>>=const{std::cell::Cell::new(None)};}
fn refuse_selected_allocation()->bool{REFUSED_ALLOCATION.try_with(|state|match state.get(){Some(0)=>{state.set(Some(1));true},_=>false}).unwrap_or(false)}
/// 🚫️ Refuses exactly one allocation request on this thread so raw fallible native admission owns its real failure.
pub fn refuse_one_allocation_on_this_thread<T>(operation:impl FnOnce()->T)->(T,usize){
 struct Guard;impl Drop for Guard{fn drop(&mut self){let _=REFUSED_ALLOCATION.try_with(|state|state.set(None));}}
 REFUSED_ALLOCATION.with(|state|{assert!(state.replace(None).is_none());state.set(Some(0));});let guard=Guard;let result=operation();let attempts=REFUSED_ALLOCATION.with(|state|state.replace(None).unwrap());drop(guard);(result,attempts)
}

struct ObservedAllocator;
thread_local! { static RETIREMENT_ALLOCATION_EVENTS: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) }; }
fn retirement_allocation_event(requested: usize, released: usize) { let _ = RETIREMENT_ALLOCATION_EVENTS.try_with(|events| { if let Some((allocated, freed)) = events.get() { events.set(Some((allocated + requested, freed + released))); } }); }
pub fn observe_retirement_allocations<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    struct Guard;
    impl Drop for Guard{fn drop(&mut self){let _=RETIREMENT_ALLOCATION_EVENTS.try_with(|events|events.set(None));}}
    RETIREMENT_ALLOCATION_EVENTS.with(|events|{assert!(events.get().is_none());events.set(Some((0,0)));});
    let guard=Guard;
    let result=operation();
    let events=RETIREMENT_ALLOCATION_EVENTS.with(|events|events.replace(None).unwrap());
    drop(guard);
    (result,events)
}
/// 📏️ Reads the requested and released bytes the open retirement scope of this thread has recorded so far; nested observers measure by difference.
pub fn retirement_allocations_so_far()->Option<(usize,usize)>{RETIREMENT_ALLOCATION_EVENTS.try_with(|events|events.get()).ok().flatten()}
pub static OBSERVE_ALLOCATIONS:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
pub static LARGEST_ALLOCATION:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{if refuse_selected_allocation(){return std::ptr::null_mut();}if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(layout.size(),std::sync::atomic::Ordering::Relaxed);}let pointer=unsafe{std::alloc::GlobalAlloc::alloc(&semio_framework_trace_memory_testing::RequestedAllocator,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{if refuse_selected_allocation(){return std::ptr::null_mut();}if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(layout.size(),std::sync::atomic::Ordering::Relaxed);}let pointer=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&semio_framework_trace_memory_testing::RequestedAllocator,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{if refuse_selected_allocation(){return std::ptr::null_mut();}if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(size,std::sync::atomic::Ordering::Relaxed);}let grown=unsafe{std::alloc::GlobalAlloc::realloc(&semio_framework_trace_memory_testing::RequestedAllocator,pointer,layout,size)};if !grown.is_null(){retirement_allocation_event(size,layout.size());}grown}
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){retirement_allocation_event(0,layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&semio_framework_trace_memory_testing::RequestedAllocator,pointer,layout)}}
}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
