//! 🪪️ Erased publication views retain genuine typed factory ownership through full closure.
use super::{ReadLease,ReadLeaseId,ReadOwnershipRegistry};
use crate::{ErasedSnapshotRetirement,FactoryAuthority,FactoryRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::{any::Any,mem::ManuallyDrop,sync::Arc};

trait Publication:FactoryRetirement+Any {
    fn as_any(&self)->&dyn Any;
    fn publish(&self,generation:u64,revision:[u8;32])->bool;
    fn matches(&self,generation:u64,revision:[u8;32])->bool;
    fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError>;
    fn occupied_count(&self)->Result<usize,ValueError>;
    fn has_returned(&self)->bool;
}
impl<T:RetireOwned+Sync> Publication for ReadOwnershipRegistry<T> {
    fn as_any(&self)->&dyn Any {self}
    fn publish(&self,generation:u64,revision:[u8;32])->bool {self.publish_authority(generation,revision)}
    fn matches(&self,generation:u64,revision:[u8;32])->bool {self.authority_matches(generation,revision)}
    fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError> {self.contains(id)}
    fn occupied_count(&self)->Result<usize,ValueError> {self.occupied_count()}
    fn has_returned(&self)->bool {self.has_returned()}
}

/// 🪪️ A sealed erased view and its original typed factory capability close as one managed owner.
pub struct ReadAuthority {view:ManuallyDrop<Option<Arc<dyn Publication>>>,owner:FactoryAuthority}
impl ReadAuthority {
    pub fn terminal_is_empty(&self)->bool {self.view.is_none()&&self.owner.terminal_is_empty()}
    pub fn constructor_capacity_bytes<T:RetireOwned+Sync>()->usize {ReadOwnershipRegistry::<T>::constructor_capacity_bytes()}
    pub fn admit<T:RetireOwned+Sync>(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError> {
        let(root,p)=ReadOwnershipRegistry::<T>::admit(grant)?;let view:Arc<dyn Publication>=root;let factory:Arc<dyn FactoryRetirement>=view.clone();
        Ok((Self {view:ManuallyDrop::new(Some(view)),owner:FactoryAuthority::new(factory)},p))
    }
    pub fn admit_clone(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError> {
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"read authority clone requires an admitted item"));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"read authority clone requires admitted depth"));}
        let view=self.view.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"read authority cannot clone after closure began"))?.clone();let factory:Arc<dyn FactoryRetirement>=view.clone();
        Ok((Self {view:ManuallyDrop::new(Some(view)),owner:FactoryAuthority::new(factory)},RetainedCloneProgress {copied_items:1,..Default::default()}))
    }
    pub fn publish_authority(&self,generation:u64,revision:[u8;32])->bool {self.view.as_ref().is_some_and(|view|view.publish(generation,revision))}
    pub fn authority_matches(&self,generation:u64,revision:[u8;32])->bool {self.view.as_ref().is_some_and(|view|view.matches(generation,revision))}
    pub fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError> {self.view.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"read authority is closing"))?.contains(id)}
    pub fn occupied_count(&self)->Result<usize,ValueError> {self.view.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"read authority is closing"))?.occupied_count()}
    pub fn has_returned(&self)->bool {self.view.as_ref().is_some_and(|view|view.has_returned())}
    /// 🪪️ The sealed publication family proves the concrete Arc payload before reconstructing its typed alias.
    pub fn issue<T:RetireOwned+Sync>(&self,root:Arc<T>,grant:RetainedCloneGrant)->Result<(ReadLease<T>,RetainedCloneProgress),(ValueError,Arc<T>)> {
        let Some(view)=self.view.as_ref().filter(|view|view.as_any().is::<ReadOwnershipRegistry<T>>())else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"read authority has a different concrete root type or is closing"),root));};
        let pointer=Arc::into_raw(Arc::clone(view))as*const ReadOwnershipRegistry<T>;
        let registry=unsafe{Arc::from_raw(pointer)};
        registry.try_issue(root,grant)
    }
    pub fn take_returned<T:RetireOwned+Sync>(&self,grant:RetainedCloneGrant)->Result<(Option<Arc<T>>,RetainedCloneProgress),ValueError> {
        self.view.as_ref().and_then(|view|view.as_any().downcast_ref::<ReadOwnershipRegistry<T>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"read authority maintenance has a different concrete root type or is closing"))?.take_returned(grant)
    }
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {if self.view.is_some(){Ok(RetirementDemand {depth:1,..Default::default()})}else{self.owner.demands(copy)}}
}
impl ErasedSnapshotRetirement for ReadAuthority {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.view.is_some(){if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"read view closure requires admitted depth"));}drop(self.view.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        self.owner.step(grant)
    }
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.depth)}
}
impl RetirementCursor for ReadAuthority {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {match ErasedSnapshotRetirement::close_step(self,grant){Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool {ErasedSnapshotRetirement::terminal_is_empty(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError> {ErasedSnapshotRetirement::next_depth_demand(self)}
    fn allows_admitted_narrow_work(&self)->bool {true}
    fn terminal_release_bytes(&self)->Option<usize> {ErasedSnapshotRetirement::terminal_is_empty(self).then_some(size_of::<Self>())}
}
impl RetireOwned for ReadAuthority {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool {true}
}
impl Drop for ReadAuthority {fn drop(&mut self){assert!(std::thread::panicking()||ErasedSnapshotRetirement::terminal_is_empty(self),"read authority must close its original typed root and factory frame");if self.view.is_none(){unsafe{ManuallyDrop::drop(&mut self.view);}}}}
