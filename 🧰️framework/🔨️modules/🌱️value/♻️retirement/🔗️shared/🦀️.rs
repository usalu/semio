//! 🔗️ Shared immutable owners retain Arc backing custody until aliases and weak leases close.

use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{sync::Arc,mem::ManuallyDrop};

#[path="🏭️factory/🦀️.rs"]
pub mod factory;
pub use factory::FactorySharedRetirement;

pub struct SharedControlledRetirement<T:RetireOwned+Sync> {source:ManuallyDrop<Option<Arc<T>>>,owned:ManuallyDrop<Option<ControlledRetirement<T>>>,lease_only:bool}
impl<T:RetireOwned+Sync> SharedControlledRetirement<T> {
    pub fn new(source:Arc<T>)->Self {Self::with_custody(source,false)}
    pub fn lease(source:Arc<T>)->Self {Self::with_custody(source,true)}
    fn with_custody(source:Arc<T>,lease_only:bool)->Self {Self {source:ManuallyDrop::new(Some(source)),owned:ManuallyDrop::new(None),lease_only}}
    pub fn terminal_is_empty(&self)->bool {self.source.is_none()&&self.owned.is_none()}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.owned.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.owned.as_ref().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(copy))}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {if let Some(source)=self.source.as_ref(){return Ok(if (self.lease_only||Arc::strong_count(source)==1)&&Arc::weak_count(source)==0{arc_bytes::<T>()}else{0});}self.owned.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {self.owned.as_ref().map_or(Ok(usize::from(self.source.is_some())),ControlledRetirement::next_depth_demand)}
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"shared retirement exceeds admitted depth"));}
        if let Some(source)=self.source.as_mut(){
            if self.lease_only {
                if Arc::weak_count(source)!=0||grant.maximum_release_bytes<arc_bytes::<T>(){return Ok(RetainedCloneStep::Progress(empty));}
                let value=Arc::into_inner(self.source.take().unwrap());
                let released_bytes=if let Some(value)=value{*self.owned=Some(ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("admitted shared payload refused: {error}")));arc_bytes::<T>()}else{0};
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes,..empty}));
            }
            if grant.maximum_release_bytes<arc_bytes::<T>()||Arc::get_mut(source).is_none(){return Ok(RetainedCloneStep::Progress(empty));}
            let source=self.source.take().unwrap();
            match Arc::try_unwrap(source){Ok(value)=>{*self.owned=Some(ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("admitted shared payload refused: {error}")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:arc_bytes::<T>(),..empty}));},Err(source)=>{*self.source=Some(source);return Ok(RetainedCloneStep::Progress(empty));}}
        }
        let owned=self.owned.as_mut().unwrap();
        if owned.terminal_is_empty(){*self.owned=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));}
        owned.step(grant)
    }
}

impl<T:RetireOwned+Sync> crate::ErasedSnapshotRetirement for SharedControlledRetirement<T> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {self.step(grant)}
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Self::next_copy_byte_demand(self)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Self::next_capacity_byte_demand(self,copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Self::next_release_byte_demand(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
}

/// 📏️ Borrows the original shared-owner cursor frame before moving any lease.
pub const fn shared_retirement_birth_bytes<T:RetireOwned+Sync>()->usize {size_of::<SharedControlledRetirement<T>>()}

/// 🎟️ Admits one concrete shared frame and returns the same original lease on refusal.
pub fn admit_shared_retirement<T:RetireOwned+Sync>(source:Arc<T>,grant:RetainedCloneGrant,lease_only:bool)->Result<(Box<dyn crate::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<T>)> {
    if !T::controlled_retirement_supported(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared payload has no controlled retirement authority"),source));}
    if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"shared frame requires one admitted item"),source));}
    if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"shared frame requires admitted depth"),source));}
    let layout=std::alloc::Layout::new::<SharedControlledRetirement<T>>();
    if layout.size()>grant.maximum_capacity_bytes{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"shared frame exceeds admitted capacity"),source));}
    let Some(pointer)=std::ptr::NonNull::new(unsafe {std::alloc::alloc(layout)}.cast::<SharedControlledRetirement<T>>())else{return Err((ValueError::literal(ValueRefusalKind::AllocationFailed,"shared frame allocation failed"),source));};
    let owner=unsafe {pointer.as_ptr().write(SharedControlledRetirement::with_custody(source,lease_only));Box::from_raw(pointer.as_ptr())};
    Ok((owner,RetainedCloneProgress {copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()}))
}
impl<T:RetireOwned+Sync> Drop for SharedControlledRetirement<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared retirement abandoned actual Arc allocation ownership");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.owned);}}}}
impl<T:RetireOwned+Sync> RetirementCursor for SharedControlledRetirement<T> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {match self.step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}}
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.next_copy_byte_demand()}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned+Sync> RetireOwned for Arc<T> {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(SharedControlledRetirement::new(self))}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<SharedControlledRetirement<T>>())}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
}
impl<T:RetireOwned+Sync> RetireOwned for SharedControlledRetirement<T> {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
/// 📏️ Exact original shared backing demand used by the admitted shared retirement owners.
pub fn shared_retirement_allocation_bytes<T>()->usize {std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).expect("shared Arc allocation layout").0.pad_to_align().size()}
pub(crate) fn arc_bytes<T>()->usize {shared_retirement_allocation_bytes::<T>()}

#[cfg(test)]
#[path="🔐️unique/🧪️tests/🦀️.rs"]
mod shared_unique_tests;
