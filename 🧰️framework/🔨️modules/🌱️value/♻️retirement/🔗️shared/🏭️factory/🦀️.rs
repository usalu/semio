//! 🔐️ Shared artifact values retain original payload and factory authority through admitted handoff.
use crate::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryRetirement,FactoryAuthority,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::{mem::ManuallyDrop,sync::Arc};

/// 🧳️ An inline pending value preserves custody when its exact owned factory refuses admission.
pub struct FactorySharedRetirement<T:Send+Sync+'static> {
    alias:ManuallyDrop<Option<Arc<T>>>,
    pending:ManuallyDrop<Option<T>>,
    unique:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    closing_factory:ManuallyDrop<Option<FactoryAuthority>>,
}
impl<T:Send+Sync+'static> FactorySharedRetirement<T> {
    pub fn new(alias:Arc<T>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->Self {Self {alias:ManuallyDrop::new(Some(alias)),pending:ManuallyDrop::new(None),unique:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(factory)),closing_factory:ManuallyDrop::new(None)}}
    pub const fn constructor_capacity_bytes()->usize {size_of::<Self>()}
    pub fn admit(alias:Arc<T>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<T>,Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)> {
        let refusal=if grant.maximum_items==0 {Some((ValueRefusalKind::WorkLimit,"shared factory frame requires one admitted item"))}else if grant.maximum_depth==0 {Some((ValueRefusalKind::DepthLimit,"shared factory frame requires admitted depth"))}else if grant.maximum_capacity_bytes<Self::constructor_capacity_bytes() {Some((ValueRefusalKind::OwnershipLimit,"shared factory frame exceeds admitted capacity"))}else{None};
        if let Some((kind,message))=refusal {return Err((ValueError::literal(kind,message),alias,factory));}
        let layout=std::alloc::Layout::new::<Self>();
        let Some(pointer)=std::ptr::NonNull::new(unsafe {std::alloc::alloc(layout)}.cast::<Self>())else{return Err((ValueError::literal(ValueRefusalKind::AllocationFailed,"shared factory frame allocation failed"),alias,factory));};
        let owner=unsafe {pointer.as_ptr().write(Self::new(alias,factory));Box::from_raw(pointer.as_ptr())};
        Ok((owner,RetainedCloneProgress {copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()}))
    }
    /// 🎟️ Admits a borrowed original alias before taking it or cloning its installed factory capability.
    pub fn admit_original(original:&mut Option<Arc<T>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
        if original.is_none(){return Ok(None);}
        let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"shared factory transfer requires one admitted item"))}else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"shared factory transfer requires admitted depth"))}else if grant.maximum_capacity_bytes<Self::constructor_capacity_bytes(){Some((ValueRefusalKind::OwnershipLimit,"shared factory transfer exceeds admitted frame capacity"))}else{None};
        if let Some((kind,message))=refusal{return Err(ValueError::literal(kind,message));}
        match Self::admit(original.take().expect("observed original snapshot alias"),Arc::clone(factory),grant){Ok(admitted)=>Ok(Some(admitted)),Err((error,alias,capability))=>{*original=Some(alias);drop(capability);Err(error)}}
    }
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        if self.alias.is_some(){return Ok(RetirementDemand {release_bytes:super::arc_bytes::<T>(),depth:1,..Default::default()});}
        if let Some(value)=self.pending.as_ref(){return Ok(RetirementDemand {capacity_bytes:self.factory.as_ref().expect("pending value retains original factory").retirement_birth_bytes(value),depth:2,..Default::default()});}
        if let Some(unique)=self.unique.as_ref(){let mut demand=crate::factory_ticket_demands(unique,copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"shared factory payload depth overflow"))?;return Ok(demand);}
        if self.factory.is_some(){return Ok(RetirementDemand {depth:1,..Default::default()});}
        if let Some(factory)=self.closing_factory.as_ref(){let mut demand=factory.demands(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"shared factory capability depth overflow"))?;return Ok(demand);}
        Ok(Default::default())
    }
}
impl<T:Send+Sync+'static> ErasedSnapshotRetirement for FactorySharedRetirement<T> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        if let Some(alias)=self.alias.as_ref(){
            if Arc::weak_count(alias)!=0{return Ok(RetainedCloneStep::Progress(empty));}
            let unique=Arc::into_inner(self.alias.take().unwrap());let released_bytes=if unique.is_some(){super::arc_bytes::<T>()}else{0};
            *self.pending=unique;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes,..empty}));
        }
        let child=RetainedCloneGrant {maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if self.pending.is_some(){
            let value=self.pending.take().unwrap();
            match self.factory.as_ref().unwrap().retire_owned(value,child){
                Ok((owner,progress))=>{*self.unique=Some(owner);if !progress.fits(child)||progress.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"shared factory payload constructor changed its admitted receipt"));}return Ok(RetainedCloneStep::Progress(progress));},
                Err((error,value))=>{*self.pending=Some(value);return Err(error);},
            }
        }
        if self.unique.is_some(){return crate::close_factory_ticket(&mut self.unique,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.factory.is_some(){
            let factory:Arc<dyn FactoryRetirement>=self.factory.take().unwrap();*self.closing_factory=Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
        }
        let factory=self.closing_factory.as_mut().unwrap();let step=factory.step(child)?;let step=admit_retained_clone_close(child,step,factory.terminal_is_empty(),"shared snapshot factory capability")?;
        if factory.terminal_is_empty(){drop(self.closing_factory.take());}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn terminal_is_empty(&self)->bool {self.alias.is_none()&&self.pending.is_none()&&self.unique.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.depth)}
}
impl<T:Send+Sync+'static> Drop for FactorySharedRetirement<T> {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"shared factory retirement abandoned original snapshot or capability custody");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.alias);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.unique);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.closing_factory);}}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
