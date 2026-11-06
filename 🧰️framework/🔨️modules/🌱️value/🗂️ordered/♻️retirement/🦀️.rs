//! ♻️ Defining ordered and shared owners compose their actual payload retirement under independent authorities.

use super::{OrderedMap, Retirement, RetirementStep, SharedOwner,UpdateCursor,LookupCursor};
use crate::{ValueError, retained_clone::{RetainedCloneGrant,RetainedCloneProgress,admit_retained_clone_close}, retirement::{RetireOwned,RetirementCursor,RetirementStep as TypedStep,controlled::ControlledRetirement}};
use std::mem::ManuallyDrop;

trait RootOwner<V>:Send {
    fn needs_start(&self)->bool{false}
    fn start_paid(&mut self,_:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{crate::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
    fn payload_ready(&self)->bool;
    fn work_demand(&self)->u64{u64::from(!self.is_empty())}
    fn last_turn_work(&self)->u64;
    fn next_work_demand(&self)->u64;
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>;
    fn is_empty(&self)->bool;
    fn copy_demand(&self)->usize;
    fn release_demand(&self)->Result<usize,ValueError>;
    fn depth_demand(&self)->usize;
    fn retained_depth(&self)->usize;
    fn restore_value(&mut self,value:V);
}

impl<V:RetireOwned+Sync> RootOwner<V> for Retirement<V> {
    fn payload_ready(&self)->bool{self.payload.is_some()&&self.key_bytes.as_ref().is_none_or(Vec::is_empty)}
    fn last_turn_work(&self)->u64{Retirement::last_turn_work(self)}
    fn next_work_demand(&self)->u64{Retirement::next_work_demand(self)}
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{self.advance(grant)}
    fn is_empty(&self)->bool{self.terminal_is_empty()}
    fn copy_demand(&self)->usize{self.next_copy_byte_demand()}
    fn release_demand(&self)->Result<usize,ValueError>{self.next_close_byte_demand()}
    fn depth_demand(&self)->usize{self.next_depth_demand()}
    fn retained_depth(&self)->usize{self.retained_depth()}
    fn restore_value(&mut self,value:V){assert!(self.payload.is_none(),"ordered pending payload custody is exclusive");*self.payload=Some(value);}
}

impl<V:RetireOwned+Sync> RootOwner<V> for UpdateCursor<V> {
    fn needs_start(&self)->bool{!self.state.closing}
    fn start_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{crate::retained_clone::start_retained_close(&mut self.state.closing,effect)}
    fn payload_ready(&self)->bool{self.state.retirement.payload.is_some()&&self.state.retirement.key_bytes.as_ref().is_none_or(Vec::is_empty)}
    fn last_turn_work(&self)->u64{self.last_close_work()}
    fn next_work_demand(&self)->u64{self.next_close_work_demand()}
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{self.close_step(grant)}
    fn is_empty(&self)->bool{self.terminal_is_empty()}
    fn copy_demand(&self)->usize{self.next_close_copy_byte_demand()}
    fn release_demand(&self)->Result<usize,ValueError>{self.next_close_byte_demand()}
    fn depth_demand(&self)->usize{self.next_close_depth_demand()}
    fn retained_depth(&self)->usize{self.state.retained_depth()+self.state.retirement.retained_depth()}
    fn restore_value(&mut self,value:V){assert!(self.state.retirement.payload.is_none(),"ordered update pending payload custody is exclusive");*self.state.retirement.payload=Some(value);}
}
impl<V:RetireOwned+Sync> RootOwner<V> for LookupCursor<V> {
    fn needs_start(&self)->bool{!self.state.closing}
    fn start_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneProgress,ValueError>{crate::retained_clone::start_retained_close(&mut self.state.closing,effect)}
    fn payload_ready(&self)->bool{self.state.retirement.payload.is_some()&&self.state.retirement.key_bytes.as_ref().is_none_or(Vec::is_empty)}
    fn last_turn_work(&self)->u64{self.last_close_work()}
    fn next_work_demand(&self)->u64{u64::from(!self.terminal_is_empty())}
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{self.close_step(grant)}
    fn is_empty(&self)->bool{self.terminal_is_empty()}
    fn copy_demand(&self)->usize{self.next_close_copy_byte_demand()}
    fn release_demand(&self)->Result<usize,ValueError>{self.next_close_byte_demand()}
    fn depth_demand(&self)->usize{self.next_close_depth_demand()}
    fn retained_depth(&self)->usize{self.state.retained_depth()+self.state.retirement.retained_depth()}
    fn restore_value(&mut self,value:V){assert!(self.state.retirement.payload.is_none(),"ordered lookup pending payload custody is exclusive");*self.state.retirement.payload=Some(value);}
}

#[doc(hidden)]
pub struct SharedRoot<V> {last_work:u64,owner:SharedOwner<V>,value:ManuallyDrop<Option<V>>}
impl<V:RetireOwned+Sync> RootOwner<V> for SharedRoot<V> {
    fn payload_ready(&self)->bool{self.value.is_some()}
    fn last_turn_work(&self)->u64{self.last_work}
    fn next_work_demand(&self)->u64{u64::from(!self.is_empty())}
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{
        self.last_work=0;if self.is_empty(){return RetirementStep::Complete;}
        if grant.maximum_items==0{return RetirementStep::Blocked;}
        if grant.maximum_depth==0{return RetirementStep::Failure(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"shared payload requires admitted depth"));}
        if let Some(value)=self.value.take(){self.last_work=1;return RetirementStep::OwnedValue(value);}
        let turn=self.owner.release_step(grant);self.last_work=turn.work;match turn.result{
            Err(error)=>RetirementStep::Failure(error),
            Ok(step) if step.progress.copied_items==0=>RetirementStep::Blocked,
            Ok(step)=>{*self.value=step.value;RetirementStep::Progress {released_items:1,released_bytes:step.progress.released_bytes}}
        }
    }
    fn is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.value.is_none()}
    fn copy_demand(&self)->usize{0}
    fn release_demand(&self)->Result<usize,ValueError>{Ok(self.owner.next_release_byte_demand())}
    fn depth_demand(&self)->usize{usize::from(!self.is_empty())}
    fn retained_depth(&self)->usize{usize::from(!self.owner.terminal_is_empty())}
    fn restore_value(&mut self,value:V){assert!(self.value.is_none(),"shared pending payload custody is exclusive");*self.value=Some(value);}
}
impl<V> Drop for SharedRoot<V> {
    fn drop(&mut self){assert!(std::thread::panicking()||(self.owner.terminal_is_empty()&&self.value.is_none()),"shared typed payload abandoned before terminal custody");if self.value.is_none(){unsafe{ManuallyDrop::drop(&mut self.value);}}}
}

#[doc(hidden)]
pub struct TypedOwner<V:RetireOwned+Sync,R:RootOwner<V>> {root:R,payload:ManuallyDrop<Option<ControlledRetirement<V>>>}
impl<V:RetireOwned+Sync,R:RootOwner<V>> TypedOwner<V,R> {
    fn new(root:R)->Self{Self{root,payload:ManuallyDrop::new(None)}}
    fn step_with(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut ControlledRetirement<V>,&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome)->TypedStep{if self.terminal_is_empty(){return TypedStep::Complete(0)}let grant=effect.grant();let work=match self.next_computational_work(grant.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return TypedStep::Failure(error.into(),0)};if work>effect.maximum_work_units(){return TypedStep::BudgetExhausted}let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return TypedStep::Failure(error.into(),0)};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return TypedStep::BudgetExhausted}if self.root.needs_start(){let turn=self.root.start_paid(grant);return match turn.result{Ok(progress)=>TypedStep::Progress(progress,turn.work),Err(error)=>TypedStep::Failure(error.into(),turn.work)}}
        if self.terminal_is_empty(){return TypedStep::Complete(0)}
        if grant.maximum_items==0{return TypedStep::BudgetExhausted}
        match self.next_depth_demand(){Err(error)=>return TypedStep::Failure(error.into(),0),Ok(depth)if depth>grant.maximum_depth=>return TypedStep::Failure(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered typed retirement exceeds admitted depth").into(),0),_=>{}}
        let retained_depth=self.root.retained_depth();
        if let Some(payload)=self.payload.as_mut(){
            if payload.terminal_is_empty(){self.payload.take();return TypedStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1)}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-retained_depth,..grant};
            let turn=invoke(payload,&mut effect.reborrow(child,effect.maximum_work_units()).expect("selected ordered child effect"));let result=turn.result;let work=turn.work;
            let result=result.and_then(|step|admit_retained_clone_close(child,step,payload.terminal_is_empty(),"ordered typed payload retirement").map_err(Into::into));
            return match result{Err(error)=>TypedStep::Failure(error,work),Ok(step)=>TypedStep::Progress(step.progress(),work)}
        }
        let step=self.root.step(grant);let work=self.root.last_turn_work();
        match step{
            RetirementStep::Blocked=>TypedStep::BudgetExhausted,
            RetirementStep::Failure(error)=>TypedStep::Failure(error.into(),work),
            RetirementStep::Progress{released_items,released_bytes}=>TypedStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()},work),
            RetirementStep::ProcessedBytes(_)=>TypedStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},work),
            RetirementStep::OwnedValue(value)=>match ControlledRetirement::new(value){
                Ok(owner)=>{*self.payload=Some(owner);TypedStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},work)},
                Err((error,value))=>{self.root.restore_value(value);TypedStep::Failure(error.into(),work)}
            },
            RetirementStep::Complete=>TypedStep::Complete(work)
        }
    }

}
impl<V:RetireOwned+Sync,R:RootOwner<V>> RetirementCursor for TypedOwner<V,R> {
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{if self.root.needs_start(){return Ok(crate::retained_clone::retained_close_start_demand(false))}if let Some(owner)=self.payload.as_ref(){if owner.terminal_is_empty(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<V>>>(),depth:1,..Default::default()})}let mut d=owner.next_demand(body)?;d.depth=d.depth.checked_add(self.root.retained_depth()).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered nested retirement depth overflow"))?;Ok(d)}else{Ok(crate::RetirementDemand{copy_bytes:self.root.copy_demand().checked_add(if self.root.payload_ready(){size_of::<Option<V>>()+size_of::<Option<ControlledRetirement<V>>>()}else{0}).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"ordered typed payload headers overflow"))?,release_bytes:self.root.release_demand()?,depth:self.root.depth_demand(),..Default::default()})}}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_demand(body)?;if self.root.needs_start(){return Ok(1)}self.payload.as_ref().map_or(Ok(self.root.work_demand()),|payload|if payload.terminal_is_empty(){Ok(1)}else{payload.next_computational_work(body)})}
    fn requires_original_control(&self)->bool{!self.root.needs_start()&&self.payload.as_ref().is_some_and(|owner|crate::ErasedSnapshotRetirement::requires_original_control(owner))}
    fn requires_original_birth(&self)->bool{!self.root.needs_start()&&self.payload.as_ref().is_some_and(|owner|crate::ErasedSnapshotRetirement::requires_original_birth(owner))}
    fn close_original_birth_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->TypedStep{let depth=self.root.retained_depth();if grant.maximum_depth<depth{return TypedStep::BudgetExhausted}let Some(owner)=self.payload.as_mut()else{return TypedStep::Failure(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"ordered owner has no selected original birth").into(),0)};semio_framework_value_retirement::retirement::erased_cursor_turn(crate::ErasedSnapshotRetirement::close_original_birth_step(owner,original,RetainedCloneGrant{maximum_depth:grant.maximum_depth-depth,..grant}))}
    fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->TypedStep{self.step_with(effect,|owner,effect|crate::ErasedSnapshotRetirement::close_step_controlled(owner,effect,control))}
    fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->TypedStep{self.step_with(effect,|owner,effect|crate::ErasedSnapshotRetirement::close_original_step(owner,original,effect))}
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->TypedStep{self.step_with(effect,|owner,effect|{let result=owner.step(effect);crate::RetainedWorkOutcome{result,work:owner.last_turn_work()}})}
    fn terminal_is_empty(&self)->bool{self.root.is_empty()&&self.payload.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.payload.as_ref().map_or_else(||Ok(self.root.copy_demand()),ControlledRetirement::next_copy_byte_demand)}
    fn next_close_byte_demand(&self)->Option<usize>{self.payload.as_ref().map_or_else(||self.root.release_demand().ok(),|owner|owner.next_release_byte_demand().ok())}
    fn next_birth_bytes(&self,maximum_bytes:usize)->Option<usize>{self.payload.as_ref().map_or(Some(0),|owner|owner.next_capacity_byte_demand(maximum_bytes).ok())}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.payload.as_ref().map_or(Ok(self.root.depth_demand()),|owner|self.root.retained_depth().checked_add(owner.next_depth_demand()?).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered nested retirement depth overflow")))}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<V:RetireOwned+Sync,R:RootOwner<V>> Drop for TypedOwner<V,R> {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"ordered typed retirement abandoned before terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.payload);}}}
}

impl<V:RetireOwned+Sync> RetireOwned for OrderedMap<V> {
    type RetirementCursor=TypedOwner<V,Retirement<V>>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(self.retire())}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,Retirement<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl<V:RetireOwned+Sync> RetireOwned for SharedOwner<V> {
    type RetirementCursor=TypedOwner<V,SharedRoot<V>>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(SharedRoot{last_work:0,owner:self,value:ManuallyDrop::new(None)})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,SharedRoot<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl RetireOwned for super::OrderedSet {
    type RetirementCursor=TypedOwner<(),Retirement<()>>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(self.retire())}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<(),Retirement<()>>>())}
    fn controlled_retirement_supported()->bool{true}
}
impl<V:RetireOwned+Sync> RetireOwned for Retirement<V> {
    type RetirementCursor=TypedOwner<V,Retirement<V>>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,Retirement<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl<V:RetireOwned+Sync> RetireOwned for UpdateCursor<V> {
    type RetirementCursor=TypedOwner<V,UpdateCursor<V>>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,UpdateCursor<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl<V:RetireOwned+Sync> RetireOwned for LookupCursor<V> {
    type RetirementCursor=TypedOwner<V,LookupCursor<V>>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,LookupCursor<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}

#[cfg(test)]
#[path="../🎟️work/♻️retirement/🧪️tests/🦀️.rs"]
mod actual_work_tests;
