//! 🔗️ Original Store read guards retain exact generation and registry custody through funded closure.
use super::{SnapshotRead,SnapshotReadLeaseRefusal,snapshot_registry_alias_demands,snapshot_registry_alias_close_step,artifact_retirement_box_demands,artifact_retirement_box_close_step,bounded_outcome,outcome_retirement_step};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedWorkOutcome,ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,arc_custody::{SharedControlledRetirement,admit_shared_retirement}}};
use std::mem::ManuallyDrop;

struct SnapshotReadRetirement<T:RetireOwned+Sync> {
    read:ManuallyDrop<Option<SnapshotRead<T>>>,
    alias:ManuallyDrop<Option<SharedControlledRetirement<T>>>,
    registry:ManuallyDrop<Option<crate::SnapshotReadRegistryAliasRetirement>>,
    active_returned:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}
impl<T:RetireOwned+Sync> SnapshotReadRetirement<T> {
    fn new(read:SnapshotRead<T>)->Self {Self{read:ManuallyDrop::new(Some(read)),alias:ManuallyDrop::new(None),registry:ManuallyDrop::new(None),active_returned:ManuallyDrop::new(None)}}
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let nested=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained Store read depth overflow"))?;Ok(demand)};
        if self.read.as_ref().is_some_and(|read|read.owner.is_some()){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(alias)=self.alias.as_ref(){return if alias.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{nested(RetirementDemand{copy_bytes:alias.next_copy_byte_demand()?,capacity_bytes:alias.next_capacity_byte_demand(copy)?,release_bytes:alias.next_release_byte_demand()?,depth:alias.next_depth_demand()?})};}
        if self.read.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(active)=self.active_returned.as_ref(){return nested(artifact_retirement_box_demands(active,copy)?);}
        if let Some(registry)=self.registry.as_ref(){if registry.strong_count()==1&&registry.has_returned(){return nested(registry.returned_admission_demands::<T>(|_|RetirementDemand{capacity_bytes:size_of::<SharedControlledRetirement<T>>(),depth:1,..Default::default()}).map_err(SnapshotReadLeaseRefusal::into_value_error)?);}}
        snapshot_registry_alias_demands(&self.registry)
    }
    fn step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
        let refuse=|error:ValueError|RetainedWorkOutcome{result:Err(error),work:0};
        let moved=|copied_items:usize|RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items,..Default::default()})),work:1};
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
        if grant.maximum_items==0{return bounded_outcome(Ok(RetainedCloneStep::Progress(Default::default())));}
        let demand=match self.demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return refuse(error)};
        if grant.maximum_depth<demand.depth{return refuse(ValueError::literal(ValueRefusalKind::DepthLimit,"Store read closure exceeds admitted depth"));}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return bounded_outcome(Ok(RetainedCloneStep::Progress(Default::default())));}
        if let Some(read)=self.read.as_mut(){if let Some(owner)=read.owner.take(){*self.alias=Some(SharedControlledRetirement::lease(owner));return moved(1);}}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if let Some(alias)=self.alias.as_mut(){
            if alias.terminal_is_empty(){self.alias.take();return moved(1);}
            let result=alias.step(child).map(|step|RetainedCloneStep::Progress(step.progress()));
            return RetainedWorkOutcome{result,work:alias.last_turn_work()};
        }
        if let Some(mut read)=self.read.take(){if let Some(mut lease)=read.lease.take(){lease.return_now();*self.registry=Some(crate::SnapshotReadRegistryAliasRetirement::new(lease.registry));}return moved(1);}
        if self.active_returned.is_some(){return artifact_retirement_box_close_step(&mut self.active_returned,child);}
        if let Some(registry)=self.registry.as_ref(){if registry.strong_count()==1&&registry.has_returned(){
            let turn=registry.try_admit_one_returned::<T,_>(child,|root,grant|admit_shared_retirement(root,grant,true));
            let result=match turn.result{Ok((owner,receipt))=>{*self.active_returned=owner;if !receipt.fits(child){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained read constructor exceeded its original grant"))}else{Ok(RetainedCloneStep::Progress(receipt))}},Err(SnapshotReadLeaseRefusal::Busy)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(reason)=>Err(reason.into_value_error())};
            return RetainedWorkOutcome{result,work:turn.work};
        }}
        bounded_outcome(snapshot_registry_alias_close_step(&mut self.registry,grant))
    }
}
impl<T:RetireOwned+Sync> RetirementCursor for SnapshotReadRetirement<T> {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.demands(body) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{outcome_retirement_step(self.step(grant))}
    fn terminal_is_empty(&self)->bool{self.read.is_none()&&self.alias.is_none()&&self.registry.is_none()&&self.active_returned.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned+Sync> RetireOwned for SnapshotRead<T> {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SnapshotReadRetirement::new(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SnapshotReadRetirement<T>>())}
    fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
impl<T:RetireOwned+Sync> Drop for SnapshotReadRetirement<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Store read custody requires full granted closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.read);ManuallyDrop::drop(&mut self.alias);ManuallyDrop::drop(&mut self.registry);ManuallyDrop::drop(&mut self.active_returned);}}}}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
