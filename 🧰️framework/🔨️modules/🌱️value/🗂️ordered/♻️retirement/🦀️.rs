//! ♻️ Defining ordered and shared owners compose their actual payload retirement under independent authorities.

use super::{OrderedMap, Retirement, RetirementStep, SharedOwner,UpdateCursor,LookupCursor};
use crate::{ValueError, retained_clone::{RetainedCloneGrant,RetainedCloneProgress,admit_retained_clone_close}, retirement::{RetireOwned,RetirementCursor,RetirementStep as TypedStep,controlled::ControlledRetirement}};
use std::mem::ManuallyDrop;

trait RootOwner<V>:Send {
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>;
    fn is_empty(&self)->bool;
    fn copy_demand(&self)->usize;
    fn release_demand(&self)->Result<usize,ValueError>;
    fn depth_demand(&self)->usize;
    fn retained_depth(&self)->usize;
    fn restore_value(&mut self,value:V);
}

impl<V:RetireOwned+Sync> RootOwner<V> for Retirement<V> {
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{self.advance(grant)}
    fn is_empty(&self)->bool{self.terminal_is_empty()}
    fn copy_demand(&self)->usize{self.next_copy_byte_demand()}
    fn release_demand(&self)->Result<usize,ValueError>{self.next_close_byte_demand()}
    fn depth_demand(&self)->usize{self.next_depth_demand()}
    fn retained_depth(&self)->usize{self.retained_depth()}
    fn restore_value(&mut self,value:V){assert!(self.payload.is_none(),"ordered pending payload custody is exclusive");*self.payload=Some(value);}
}

impl<V:RetireOwned+Sync> RootOwner<V> for UpdateCursor<V> {
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{self.close_step(grant)}
    fn is_empty(&self)->bool{self.terminal_is_empty()}
    fn copy_demand(&self)->usize{self.next_close_copy_byte_demand()}
    fn release_demand(&self)->Result<usize,ValueError>{self.next_close_byte_demand()}
    fn depth_demand(&self)->usize{self.next_close_depth_demand()}
    fn retained_depth(&self)->usize{self.state.retained_depth()+self.state.retirement.retained_depth()}
    fn restore_value(&mut self,value:V){assert!(self.state.retirement.payload.is_none(),"ordered update pending payload custody is exclusive");*self.state.retirement.payload=Some(value);}
}
impl<V:RetireOwned+Sync> RootOwner<V> for LookupCursor<V> {
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{self.close_step(grant)}
    fn is_empty(&self)->bool{self.terminal_is_empty()}
    fn copy_demand(&self)->usize{self.next_close_copy_byte_demand()}
    fn release_demand(&self)->Result<usize,ValueError>{self.next_close_byte_demand()}
    fn depth_demand(&self)->usize{self.next_close_depth_demand()}
    fn retained_depth(&self)->usize{self.state.retained_depth()+self.state.retirement.retained_depth()}
    fn restore_value(&mut self,value:V){assert!(self.state.retirement.payload.is_none(),"ordered lookup pending payload custody is exclusive");*self.state.retirement.payload=Some(value);}
}

struct SharedRoot<V> {owner:SharedOwner<V>,value:ManuallyDrop<Option<V>>}
impl<V:RetireOwned+Sync> RootOwner<V> for SharedRoot<V> {
    fn step(&mut self,grant:RetainedCloneGrant)->RetirementStep<V>{
        if self.is_empty(){return RetirementStep::Complete;}
        if grant.maximum_items==0{return RetirementStep::Blocked;}
        if grant.maximum_depth==0{return RetirementStep::Failure(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"shared payload requires admitted depth"));}
        if let Some(value)=self.value.take(){return RetirementStep::OwnedValue(value);}
        match self.owner.release_step(grant){
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

struct TypedOwner<V:RetireOwned+Sync,R:RootOwner<V>> {root:R,payload:ManuallyDrop<Option<ControlledRetirement<V>>>}
impl<V:RetireOwned+Sync,R:RootOwner<V>> TypedOwner<V,R> {
    fn new(root:R)->Self{Self{root,payload:ManuallyDrop::new(None)}}
}
impl<V:RetireOwned+Sync,R:RootOwner<V>> RetirementCursor for TypedOwner<V,R> {
    fn close_step(&mut self,grant:RetainedCloneGrant)->TypedStep {
        if self.terminal_is_empty(){return TypedStep::Complete;}
        if grant.maximum_items==0{return TypedStep::BudgetExhausted;}
        match self.next_depth_demand(){Err(error)=>return TypedStep::Failure(error),Ok(depth) if depth>grant.maximum_depth=>return TypedStep::Failure(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered typed retirement exceeds admitted depth")),_=>{}}
        let retained_depth=self.root.retained_depth();
        if let Some(payload)=self.payload.as_mut(){
            if payload.terminal_is_empty(){self.payload.take();return TypedStep::Advanced;}
            let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-retained_depth,..grant};
            return match payload.step(child_grant).and_then(|step|admit_retained_clone_close(child_grant,step,payload.terminal_is_empty(),"ordered typed payload retirement")){
                Err(error)=>TypedStep::Failure(error),
                Ok(step)=>TypedStep::Progress(step.progress()),
            };
        }
        match self.root.step(grant){
            RetirementStep::Blocked=>TypedStep::BudgetExhausted,
            RetirementStep::Failure(error)=>TypedStep::Failure(error),
            RetirementStep::Progress {released_items,released_bytes}=>TypedStep::Progress(RetainedCloneProgress {copied_items:released_items,released_bytes,..RetainedCloneProgress::default()}),
            RetirementStep::ProcessedBytes(bytes)=>TypedStep::ProcessedBytes(bytes),
            RetirementStep::OwnedValue(value)=>match ControlledRetirement::new(value){Ok(owner)=>{*self.payload=Some(owner);TypedStep::Advanced},Err((error,value))=>{self.root.restore_value(value);TypedStep::Failure(error)}},
            RetirementStep::Complete=>TypedStep::Complete,
        }
    }
    fn terminal_is_empty(&self)->bool{self.root.is_empty()&&self.payload.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError> {self.payload.as_ref().map_or_else(||Ok(self.root.copy_demand()),ControlledRetirement::next_copy_byte_demand)}
    fn next_close_byte_demand(&self)->Option<usize>{self.payload.as_ref().map_or_else(||self.root.release_demand().ok(),|owner|owner.next_release_byte_demand().ok())}
    fn next_birth_bytes(&self,maximum_bytes:usize)->Option<usize>{self.payload.as_ref().map_or(Some(0),|owner|owner.next_capacity_byte_demand(maximum_bytes).ok())}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.payload.as_ref().map_or(Ok(self.root.depth_demand()),|owner|self.root.retained_depth().checked_add(owner.next_depth_demand()?).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered nested retirement depth overflow")))}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl<V:RetireOwned+Sync,R:RootOwner<V>> Drop for TypedOwner<V,R> {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"ordered typed retirement abandoned before terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.payload);}}}
}

impl<V:RetireOwned+Sync> RetireOwned for OrderedMap<V> {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(TypedOwner::<V,Retirement<V>>::new(self.retire()))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl<V:RetireOwned+Sync> RetireOwned for SharedOwner<V> {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(TypedOwner::<V,SharedRoot<V>>::new(SharedRoot {owner:self,value:ManuallyDrop::new(None)}))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,SharedRoot<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl RetireOwned for super::OrderedSet {
    fn retirement(self)->Box<dyn RetirementCursor>{self.retire().retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<(),Retirement<()>>>())}
    fn controlled_retirement_supported()->bool{true}
}
impl<V:RetireOwned+Sync> RetireOwned for Retirement<V> {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(TypedOwner::<V,Retirement<V>>::new(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl<V:RetireOwned+Sync> RetireOwned for UpdateCursor<V> {
    fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(TypedOwner::<V,UpdateCursor<V>>::new(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,UpdateCursor<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
impl<V:RetireOwned+Sync> RetireOwned for LookupCursor<V> {
    fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(TypedOwner::<V,LookupCursor<V>>::new(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,LookupCursor<V>>>())}
    fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
}
