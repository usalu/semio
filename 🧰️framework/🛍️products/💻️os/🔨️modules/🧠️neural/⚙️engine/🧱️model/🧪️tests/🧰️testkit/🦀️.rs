//! 🧰️ Test kit of the neural engine producers: the observing system allocator and the nested ownership observation scope, registered as the global allocator by each test unit that measures heap custody.

#[derive(Clone,Copy,Default)]
pub struct ShellObservation { pub recording:bool,pub count:usize,pub addresses:[usize;8],pub sizes:[usize;8],pub released:usize }
thread_local! {pub static SHELL_OBSERVATION:std::cell::Cell<ShellObservation>=const {std::cell::Cell::new(ShellObservation {recording:false,count:0,addresses:[0;8],sizes:[0;8],released:0})};}
thread_local! {static OWNERSHIP_OBSERVATION:std::cell::Cell<Option<(usize,usize)>>=const {std::cell::Cell::new(None)};}
struct OwnershipObservationScope {parent:Option<(usize,usize)>,active:bool}
impl OwnershipObservationScope {
    fn finish(&mut self)->(usize,usize) {
        let result=OWNERSHIP_OBSERVATION.with(|state|{let result=state.replace(None).unwrap();state.set(self.parent.map(|(allocated,released)|(allocated+result.0,released+result.1)));result});
        self.active=false;result
    }
}
impl Drop for OwnershipObservationScope {fn drop(&mut self){if self.active {self.finish();}}}
pub fn observe_ownership<T>(operation:impl FnOnce()->T)->(T,usize,usize) {
    let parent=OWNERSHIP_OBSERVATION.with(|state|state.replace(Some((0,0))));
    let mut scope=OwnershipObservationScope {parent,active:true};
    let value=operation();
    let (allocated,released)=scope.finish();
    (value,allocated,released)
}
pub struct ObservedSystem;
unsafe impl std::alloc::GlobalAlloc for ObservedSystem {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8 {
        let pointer=unsafe {std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};
        if !pointer.is_null(){let _=SHELL_OBSERVATION.try_with(|state|{let mut observed=state.get();if observed.recording&&observed.count<observed.addresses.len(){let index=observed.count;observed.addresses[index]=pointer as usize;observed.sizes[index]=layout.size();observed.count+=1;state.set(observed);}});}
        if !pointer.is_null(){let _=OWNERSHIP_OBSERVATION.try_with(|state|{if let Some((allocated,released))=state.get(){state.set(Some((allocated+layout.size(),released)));}});}
        pointer
    }
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout) {
        let _=OWNERSHIP_OBSERVATION.try_with(|state|{if let Some((allocated,released))=state.get(){state.set(Some((allocated,released+layout.size())));}});
        let _=SHELL_OBSERVATION.try_with(|state|{let mut observed=state.get();if let Some(index)=observed.addresses.iter().position(|address|*address==pointer as usize){observed.addresses[index]=0;observed.released+=layout.size();state.set(observed);}});
        unsafe {std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)};
    }
}
