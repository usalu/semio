//! 📦️ Original mutation values retain their installed issuer through independently admitted cleanup.
use crate::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FactoryAuthority,FactoryRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::{mem::ManuallyDrop,sync::Arc};

/// 🧳️ A send-only original remains owned when its exact payload issuer refuses admission.
pub struct FactoryOwnedRetirement<T:Send+'static> {
    original:ManuallyDrop<Option<T>>,
    unique:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    closing_factory:ManuallyDrop<Option<FactoryAuthority>>,
}
impl<T:Send+'static> FactoryOwnedRetirement<T> {
    pub const fn constructor_capacity_bytes()->usize {size_of::<Self>()}
    /// 🎟️ Grants the complete frame before taking its original or cloning its installed issuer.
    pub fn admit_original(original:&mut Option<T>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->Result<Option<(Box<Self>,RetainedCloneProgress)>,ValueError> {
        if original.is_none(){return Ok(None);}
        let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"owned factory frame requires one admitted item"))}else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"owned factory frame requires admitted depth"))}else if grant.maximum_capacity_bytes<Self::constructor_capacity_bytes(){Some((ValueRefusalKind::OwnershipLimit,"owned factory frame exceeds admitted capacity"))}else{None};
        if let Some((kind,message))=refusal{return Err(ValueError::literal(kind,message));}
        let layout=std::alloc::Layout::new::<Self>();
        let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<Self>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"owned factory frame allocation failed"));};
        let owner=unsafe{pointer.as_ptr().write(Self{original:ManuallyDrop::new(original.take()),unique:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(Arc::clone(factory))),closing_factory:ManuallyDrop::new(None)});Box::from_raw(pointer.as_ptr())};
        Ok(Some((owner,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})))
    }
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        if let Some(original)=self.original.as_ref(){return Ok(RetirementDemand{capacity_bytes:self.factory.as_ref().expect("original value retains installed issuer").retirement_birth_bytes(original),depth:2,..Default::default()});}
        let mut demand=if let Some(unique)=self.unique.as_ref(){crate::factory_ticket_demands(unique,copy)?}else if self.factory.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}else if let Some(factory)=self.closing_factory.as_ref(){factory.demands(copy)?}else{return Ok(Default::default());};
        demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"owned factory nested retirement depth overflow"))?;
        Ok(demand)
    }
}
impl<T:Send+'static> ErasedSnapshotRetirement for FactoryOwnedRetirement<T> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if self.original.is_some(){
            let value=self.original.take().unwrap();
            match self.factory.as_ref().unwrap().retire_owned(value,child){
                Ok((owner,progress))=>{*self.unique=Some(owner);if !progress.fits(child)||progress.retained_capacity_bytes!=demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"owned factory payload constructor changed its admitted receipt"));}return Ok(RetainedCloneStep::Progress(progress));},
                Err((error,value))=>{*self.original=Some(value);return Err(error);},
            }
        }
        if self.unique.is_some(){return crate::close_factory_ticket(&mut self.unique,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.factory.is_some(){
            let factory:Arc<dyn FactoryRetirement>=self.factory.take().unwrap();*self.closing_factory=Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));
        }
        let factory=self.closing_factory.as_mut().unwrap();let step=factory.step(child)?;let step=admit_retained_clone_close(child,step,factory.terminal_is_empty(),"original owned mutation factory capability")?;
        if factory.terminal_is_empty(){drop(self.closing_factory.take());}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn terminal_is_empty(&self)->bool {self.original.is_none()&&self.unique.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.depth)}
}
impl<T:Send+'static> Drop for FactoryOwnedRetirement<T> {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned factory retirement abandoned original payload or issuer custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.unique);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.closing_factory);}}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
