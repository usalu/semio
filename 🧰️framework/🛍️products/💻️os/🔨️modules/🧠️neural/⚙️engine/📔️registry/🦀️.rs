//! 📔️ Exact shared registry ownership and incremental retirement.

use super::{Operator,Registry,ValueRetirement,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement,RetirementStep};
use semio_framework_value::retained_clone::admit_retained_clone_close;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};
use std::alloc::{AllocError,Allocator,Global,Layout};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize,Ordering};

//#region 🔗️SharedOwnership
struct Shell {address:usize,layout:Layout}
struct FinalRootSlot {registry:OnceLock<ManuallyDrop<Registry>>,shell:OnceLock<Shell>}
#[derive(Default)]
struct ShellAllocator(AtomicUsize);
impl Clone for ShellAllocator {fn clone(&self)->Self {Self(AtomicUsize::new(self.0.load(Ordering::Acquire)))}}
unsafe impl Allocator for ShellAllocator {
    fn allocate(&self,layout:Layout)->Result<NonNull<[u8]>,AllocError> {let allocation=Global.allocate(layout)?;self.0.store(layout.size(),Ordering::Release);Ok(allocation)}
    unsafe fn deallocate(&self,pointer:NonNull<u8>,layout:Layout) {unsafe {Global.deallocate(pointer,layout)};}
}
type FinalRoot=Arc<FinalRootSlot,ShellAllocator>;
/// 🧷️ The calling SharedRegistry reader keeps this original final-root slot alive through its allocation handoff.
#[derive(Clone,Copy)]
struct RegistryAllocator {final_root:usize}
unsafe impl Allocator for RegistryAllocator {
    fn allocate(&self,layout:Layout)->Result<NonNull<[u8]>,AllocError> {Global.allocate(layout)}
    unsafe fn deallocate(&self,pointer:NonNull<u8>,layout:Layout) {let root=unsafe {&*(self.final_root as *const FinalRootSlot)};assert!(root.shell.set(Shell {address:pointer.as_ptr()as usize,layout}).is_ok(),"one original registry allocation shell");}
}

/// 🪪️ Identifies one original registry root while its exact source readers remain alive.
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub struct RegistryIdentity(usize);

/// 🔗️ Every registry reader participates in the exact last-reader handoff; raw Arc roots never escape.
#[derive(Clone)]
pub struct SharedRegistry { root: Option<Arc<Registry,RegistryAllocator>>, final_root: FinalRoot }
impl SharedRegistry {
    pub fn into_retirement(self)->RegistryLeaseRetirement {RegistryLeaseRetirement {source:Some(self)}}
    /// 🪪️ Borrows execution identity from this original source lease without allocating or hashing metadata.
    pub fn owner_identity(&self)->RegistryIdentity {RegistryIdentity(Arc::as_ptr(self.root.as_ref().expect("open registry reader")) as usize)}
    /// 🎟️ Creates readers and their unique retirement authority before any shared alias can escape.
    pub fn new(registry: Registry) -> (Self, RegistryRetirement) {
        let final_root=Arc::new_in(FinalRootSlot {registry:OnceLock::new(),shell:OnceLock::new()},ShellAllocator::default());
        let retirement=RegistryRetirement {final_root:ManuallyDrop::new(Some(Arc::clone(&final_root))),registry:None};
        let root=Arc::new_in(registry,RegistryAllocator {final_root:Arc::as_ptr(&final_root)as usize});
        (Self { root: Some(root), final_root }, retirement)
    }
}
impl Deref for SharedRegistry { type Target = Registry; fn deref(&self) -> &Registry { self.root.as_deref().expect("open registry reader") } }
impl AsRef<Registry> for SharedRegistry { fn as_ref(&self) -> &Registry { self } }
impl Drop for SharedRegistry {
    fn drop(&mut self) {
        if let Some(root) = self.root.take().and_then(Arc::into_inner) {
            assert!(self.final_root.registry.set(ManuallyDrop::new(root)).is_ok(), "registry final root is handed off exactly once");
        }
    }
}
//#endregion 🔗️SharedOwnership

//#region 🧹️RegistryRetirement
/// 🧹️ Preserves the original registry payload and both allocator-recorded shells until funded closure.
#[must_use="registry retirement must reach terminal-empty"]
pub struct RegistryRetirement {final_root:ManuallyDrop<Option<FinalRoot>>,registry:Option<ControlledRetirement<Registry>>}
impl RegistryRetirement {
    pub fn terminal_is_empty(&self)->bool {self.final_root.is_none()&&self.registry.is_none()}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(0)}else{self.registry.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand)}}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(0)}else{self.registry.as_ref().map_or(Ok(0),|value|value.next_capacity_byte_demand(copy))}}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        if let Some(root)=self.final_root.as_ref() {
            if Arc::strong_count(root)!=1 {return Ok(0);}
            if let Some(shell)=root.shell.get(){return Ok(shell.layout.size());}
            if root.registry.get().is_some(){return Ok(0);}
            return Ok(Arc::allocator(root).0.load(Ordering::Acquire));
        }
        self.registry.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)
    }
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(1)}else{self.registry.as_ref().map_or(Ok(0),ControlledRetirement::next_depth_demand)}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"registry retained ownership exceeds admitted depth"));}
        if let Some(root)=self.final_root.as_mut() {
            let Some(slot)=Arc::get_mut(root) else {return Ok(RetainedCloneStep::Progress(empty));};
            if let Some(shell)=slot.shell.get() {
                if shell.layout.size()>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
                let shell=slot.shell.take().unwrap();
                unsafe {Global.deallocate(NonNull::new_unchecked(shell.address as *mut u8),shell.layout)};
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:shell.layout.size(),..empty}));
            }
            if let Some(value)=slot.registry.take() {
                self.registry=Some(ControlledRetirement::new(ManuallyDrop::into_inner(value)).unwrap_or_else(|(error,_)|panic!("original registry typed ownership refused: {error}")));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
            }
            if self.registry.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registry final-root payload is missing"));}
            let bytes=Arc::allocator(root).0.load(Ordering::Acquire);
            if bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            drop(self.final_root.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
        }
        let value=self.registry.as_mut().unwrap();
        let step=value.step(grant)?;
        let progress=admit_retained_clone_close(grant,step,value.terminal_is_empty(),"original registry fields")?.progress();
        if value.terminal_is_empty(){self.registry=None;}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
impl Drop for RegistryRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"registry must finish explicit retirement before drop");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.final_root);}}}}

/// 🔗️ A reader transfers only its actual lease; the original external authority keeps every final shell.
#[must_use="registry lease must reach terminal-empty"]
pub struct RegistryLeaseRetirement {source:Option<SharedRegistry>}
impl RegistryLeaseRetirement {
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    pub fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError> {Ok(0)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(usize::from(self.source.is_some()))}
    pub fn terminal_is_empty(&self)->bool {self.source.is_none()}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        let Some(source)=self.source.as_ref() else {return Ok(RetainedCloneStep::Complete(empty));};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"registry reader handoff requires admitted depth"));}
        if Arc::strong_count(&source.final_root)<2{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"registry reader has no retained external shell authority"));}
        drop(self.source.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..empty}))
    }
}
impl Drop for RegistryLeaseRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"registry source lease abandoned before explicit handoff");}}
impl RetireOwned for SharedRegistry {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(self.into_retirement())}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(std::mem::size_of::<RegistryLeaseRetirement>())}
    fn controlled_retirement_supported()->bool {true}
}

struct OperatorRetirement {operator:Option<Box<dyn Operator>>,values:ValueRetirement}
impl OperatorRetirement {
    fn terminal_is_empty(&self)->bool {self.operator.is_none()&&self.values.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){self.values.next_copy_byte_demand()}else{self.operator.as_ref().map_or(Ok(0),|value|value.next_retire_copy_byte_demand())}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){self.values.next_capacity_byte_demand(copy)}else{self.operator.as_ref().map_or(Ok(0),|value|value.next_retire_capacity_byte_demand(copy))}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        if !self.values.terminal_is_empty(){return self.values.next_release_byte_demand();}
        self.operator.as_ref().map_or(Ok(0),|value|if value.retirement_is_empty(){Ok(std::mem::size_of_val(value.as_ref()))}else{value.next_retire_release_byte_demand()})
    }
    fn next_depth_demand(&self)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){self.values.next_depth_demand()}else{self.operator.as_ref().map_or(Ok(0),|value|if value.retirement_is_empty(){Ok(1)}else{value.next_retire_depth_demand()})}}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if !self.values.terminal_is_empty(){return self.values.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"operator retained ownership exceeds admitted depth"));}
        let value=self.operator.as_mut().unwrap();
        if value.retirement_is_empty() {
            let bytes=std::mem::size_of_val(value.as_ref());
            if bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            drop(self.operator.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
        }
        let step=value.retire_step(grant,&mut self.values)?;
        Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant,step,value.retirement_is_empty(),"original operator fields")?.progress()))
    }
}
impl Drop for OperatorRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"operator abandoned before physical shell closure");}}
impl RetireOwned for Box<dyn Operator> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(OperatorRetirement {operator:Some(self),values:ValueRetirement::default()})}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(std::mem::size_of::<OperatorRetirement>())}
    fn controlled_retirement_supported()->bool {true}
}
macro_rules! original_retirement_cursor {
    ($type:ty) => {impl RetirementCursor for $type {
        fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {match Self::close_step(self,grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}}
        fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
        fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
        fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
        fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
        fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
        fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
    }};
}
original_retirement_cursor!(RegistryLeaseRetirement);
original_retirement_cursor!(OperatorRetirement);
//#endregion 🧹️RegistryRetirement

#[cfg(test)]
#[path = "🧪️tests/📔️registry/🦀️.rs"]
pub(crate) mod tests;
