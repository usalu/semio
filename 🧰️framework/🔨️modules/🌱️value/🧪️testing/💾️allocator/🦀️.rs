//! 💾️ Original physical allocation receipt observer shared by true native owner suites.
//!
//! 🧷️ This crate owns the one `#[global_allocator]` of every test binary that links it, including through the job `testkit` feature. A test binary MUST NOT declare its own global allocator and MUST measure through [`semio_framework_trace::observe_heap_allocations_on_this_thread`], [`semio_framework_trace_memory_testing::observe`], [`observe_retirement_allocations`] and the refusal scopes of this crate. Production targets never depend on it: it is only reachable through `role = "test"` crates, optional `testkit` features and `[dev-dependencies]`.

#[derive(Clone,Copy)]
enum Refusal{One(usize),All(usize),Sized{size:usize,attempts:usize}}
thread_local!{static REFUSAL:std::cell::Cell<Option<Refusal>>=const{std::cell::Cell::new(None)};}
fn refuse_selected_allocation(requested:Option<usize>)->bool{REFUSAL.try_with(|state|{let(next,refused)=match state.get(){Some(Refusal::One(0))=>(Refusal::One(1),true),Some(Refusal::All(attempts))=>(Refusal::All(attempts+1),true),Some(Refusal::Sized{size,attempts})if requested==Some(size)=>(Refusal::Sized{size,attempts:attempts+1},true),_=>return false};state.set(Some(next));refused}).unwrap_or(false)}
fn refusing<T>(initial:Refusal,operation:impl FnOnce()->T)->(T,usize){
 struct Guard;impl Drop for Guard{fn drop(&mut self){let _=REFUSAL.try_with(|state|state.set(None));}}
 REFUSAL.with(|state|{assert!(state.get().is_none(),"allocation refusal scope already active");state.set(Some(initial));});let guard=Guard;let result=operation();
 let attempts=match REFUSAL.with(|state|state.replace(None)).unwrap(){Refusal::One(attempts)|Refusal::All(attempts)|Refusal::Sized{attempts,..}=>attempts};drop(guard);(result,attempts)
}
/// 🚫️ Refuses exactly one allocation request on this thread so raw fallible native admission owns its real failure.
pub fn refuse_one_allocation_on_this_thread<T>(operation:impl FnOnce()->T)->(T,usize){refusing(Refusal::One(0),operation)}
/// 🚫️ Refuses every allocation request on this thread while the operation runs, so allocation-free retirement proves its own zero attempts.
pub fn refuse_all_allocations_on_this_thread<T>(operation:impl FnOnce()->T)->(T,usize){refusing(Refusal::All(0),operation)}
/// 🎯️ Refuses every fresh allocation of exactly `size` bytes on this thread while the operation runs, so one owner's own backing fails while its surroundings succeed.
pub fn refuse_allocations_of_size_on_this_thread<T>(size:usize,operation:impl FnOnce()->T)->(T,usize){refusing(Refusal::Sized{size,attempts:0},operation)}
/// 🪝️ Process-wide hooks that see every request layout and every released pointer; they must gate themselves on their own thread-local scope and must not allocate.
pub fn install_allocation_tap(requested:fn(std::alloc::Layout),released:fn(*mut u8,std::alloc::Layout)){let _=ALLOCATION_TAP.set((requested,released));}
static ALLOCATION_TAP:std::sync::OnceLock<(fn(std::alloc::Layout),fn(*mut u8,std::alloc::Layout))>=std::sync::OnceLock::new();
fn tap_requested(layout:std::alloc::Layout){if let Some((requested,_))=ALLOCATION_TAP.get(){requested(layout)}}
fn tap_released(pointer:*mut u8,layout:std::alloc::Layout){if let Some((_,released))=ALLOCATION_TAP.get(){released(pointer,layout)}}

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
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{tap_requested(layout);if refuse_selected_allocation(Some(layout.size())){return std::ptr::null_mut();}if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(layout.size(),std::sync::atomic::Ordering::Relaxed);}let pointer=unsafe{std::alloc::GlobalAlloc::alloc(&semio_framework_trace_memory_testing::RequestedAllocator,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{tap_requested(layout);if refuse_selected_allocation(Some(layout.size())){return std::ptr::null_mut();}if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(layout.size(),std::sync::atomic::Ordering::Relaxed);}let pointer=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&semio_framework_trace_memory_testing::RequestedAllocator,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{if let Ok(grown)=std::alloc::Layout::from_size_align(size,layout.align()){tap_requested(grown);}if refuse_selected_allocation(None){return std::ptr::null_mut();}if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(size,std::sync::atomic::Ordering::Relaxed);}let grown=unsafe{std::alloc::GlobalAlloc::realloc(&semio_framework_trace_memory_testing::RequestedAllocator,pointer,layout,size)};if !grown.is_null(){retirement_allocation_event(size,layout.size());}grown}
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){tap_released(pointer,layout);retirement_allocation_event(0,layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&semio_framework_trace_memory_testing::RequestedAllocator,pointer,layout)}}
}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
