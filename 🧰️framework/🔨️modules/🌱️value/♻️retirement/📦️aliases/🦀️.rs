//! ♻️ Original refold alias batches retain their native snapshot roots until exact admission.
use semio_framework_value::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::{ManuallyDrop,size_of,size_of_val},sync::Arc};
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

pub struct OriginalAliasBatch<P>{
    original:ManuallyDrop<Option<Vec<Arc<P>>>>,
    pending:ManuallyDrop<Option<Arc<P>>>,
    active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}
impl<P> OriginalAliasBatch<P>{
    pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()}}
    pub fn admit_alias_original(original:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        if original.is_none(){return Ok(None);}
        let demand=Self::constructor_demand();
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original single alias requires its exact inline copy and depth grant"));}
        Ok(Some((Self{original:ManuallyDrop::new(None),pending:ManuallyDrop::new(original.take()),active:ManuallyDrop::new(None)},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }
    pub fn admit_original(original:&mut Option<Vec<Arc<P>>>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        if original.is_none(){return Ok(None);}
        let demand=Self::constructor_demand();
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original alias batch requires its exact inline copy and depth grant"));}
        Ok(Some((Self{original:ManuallyDrop::new(original.take()),pending:ManuallyDrop::new(None),active:ManuallyDrop::new(None)},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }
    pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.pending.is_none()&&self.active.is_none()}
    pub fn next_demand(&self,alias_birth:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.active.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()});}
            let copy=owner.next_copy_byte_demand()?;
            return Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||refusal("original alias child depth overflow"))?});
        }
        if self.pending.is_some(){return Ok(RetirementDemand{capacity_bytes:alias_birth,depth:2,..Default::default()});}
        if let Some(original)=self.original.as_ref(){
            return Ok(if original.is_empty(){RetirementDemand{copy_bytes:size_of::<Vec<Arc<P>>>(),release_bytes:original.capacity().checked_mul(size_of::<Arc<P>>()).ok_or_else(||refusal("original alias backing byte overflow"))?,depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:2*size_of::<Arc<P>>(),depth:1,..Default::default()}});
        }
        Ok(Default::default())
    }
    pub fn advance<F>(&mut self,alias_birth:usize,grant:RetainedCloneGrant,admit:F)->Result<RetainedCloneStep,ValueError>
    where F:FnOnce(&mut Option<Arc<P>>,RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.next_demand(alias_birth)?;
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(owner)=self.active.as_mut(){
            if !owner.terminal_is_empty(){
                let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
                let step=owner.close_step(child)?;
                if !step.progress().fits(child)||matches!(step,RetainedCloneStep::Complete(_))&&!owner.terminal_is_empty(){return Err(refusal("original alias child violated its admitted receipt"));}
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            drop(self.active.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}));
        }
        if self.pending.is_some(){
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            let (owner,receipt)=admit(&mut self.pending,child)?.ok_or_else(||refusal("original alias issuer returned no retained frame"))?;
            *self.active=Some(owner);
            if self.pending.is_some()||!receipt.fits(child){return Err(refusal("original alias issuer violated its admitted custody"));}
            return Ok(RetainedCloneStep::Progress(receipt));
        }
        let original=self.original.as_mut().ok_or_else(||refusal("original alias batch lost its backing"))?;
        if !original.is_empty(){
            *self.pending=original.pop();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
        }
        drop(self.original.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
    }
}
impl<P> Drop for OriginalAliasBatch<P>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original tool alias batch requires exact terminal cleanup");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
