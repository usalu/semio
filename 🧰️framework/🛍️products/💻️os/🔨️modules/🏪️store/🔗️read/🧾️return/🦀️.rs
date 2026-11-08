//! 🧾️ Returned snapshot witnesses preserve original registry custody through funded closure.
use super::{SnapshotReadReturn,SnapshotReadLeaseRegistry,snapshot_registry_frame_bytes,pay_pure_work,original_outcome_retirement_step};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedWorkOutcome,RetainedWorkPoll,RetainedAllocationEffect,OriginalRetirementPoll,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::{mem::ManuallyDrop,sync::Arc};

impl super::ErasedSnapshotRead {
 /// 🫴️ Returns the exact original read only after its independent authority and registry admit custody.
 pub fn try_return_to_registry_witness(mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<(SnapshotReadReturn,RetainedCloneProgress),(ValueError,Self)>{
  let grant=effect.grant();let copied_bytes=size_of::<SnapshotReadReturn>()+size_of::<u64>()+size_of::<usize>();
  let denied=if grant.maximum_items==0||effect.maximum_work_units()==0||grant.maximum_copy_bytes<copied_bytes{Some((ValueRefusalKind::WorkLimit,"original read return requires its complete witness and publication metadata"))}else if grant.maximum_depth<2{Some((ValueRefusalKind::DepthLimit,"original read return requires its registry depth"))}else{None};
  if let Some((kind,message))=denied{return Err((ValueError::literal(kind,message),self))}
  let Some(lease)=self.lease.as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"original read return lacks its exact lease"),self))};
  let registry=lease.registry.clone();
  let locked=match registry.state.try_lock(){Ok(locked)=>locked,Err(_)=>return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original read return registry is busy"),self))};
  let index=lease.index;let generation=lease.generation;let slot=index as usize;
  if self.owner.is_none()||slot>=super::SNAPSHOT_READ_LEASE_CAPACITY||locked.occupied[slot/64]&(1<<(slot%64))==0||registry.lease_generations[slot].load(std::sync::atomic::Ordering::Acquire)!=generation{drop(locked);return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"original read return lost its registry generation"),self))}
  let original=unsafe{locked.slots[slot].assume_init_ref()};
  if original.generation!=generation||self.owner.as_ref().is_none_or(|owner|!Arc::ptr_eq(&original.owner,owner)){drop(locked);return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"original read return requires its exact registry slot and source pointer"),self))}
  let mut paid=0;if let Err(error)=pay_pure_work(effect,&mut paid){drop(locked);return Err((error,self))}
  registry.returned.fetch_add(1,std::sync::atomic::Ordering::AcqRel);
  if registry.lease_generations[slot].compare_exchange(generation,generation|(1u64<<63),std::sync::atomic::Ordering::AcqRel,std::sync::atomic::Ordering::Acquire).is_err(){registry.returned.fetch_sub(1,std::sync::atomic::Ordering::AcqRel);drop(locked);return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"original read return lost its exact return mark"),self))}
  drop(self.owner.take());drop(self.lease.take());drop(locked);
  Ok((SnapshotReadReturn{registry,index,generation},RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))
 }
}

#[doc(hidden)]
pub struct SnapshotReadReturnRetirement {
 registry:ManuallyDrop<Option<crate::SnapshotReadRegistryHandle>>,
 last_registry:ManuallyDrop<Option<SnapshotReadLeaseRegistry>>,
 index:u16,
 generation:u64,
}
impl SnapshotReadReturnRetirement {
 fn new(value:SnapshotReadReturn)->Self {Self{registry:ManuallyDrop::new(Some(value.registry)),last_registry:ManuallyDrop::new(None),index:value.index,generation:value.generation}}
 fn demands(&self)->Result<RetirementDemand,ValueError>{
  if self.registry.is_some(){return Ok(RetirementDemand{release_bytes:snapshot_registry_frame_bytes()?,depth:1,..Default::default()});}
  if let Some(owner)=self.last_registry.as_ref(){let mut demand=owner.owned_backing_demands()?;demand.depth=demand.depth.max(1);return Ok(demand);}
  Ok(Default::default())
 }
 fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let grant=effect.grant();let mut work=0;
  let result=(||{
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
   if grant.maximum_items==0||effect.maximum_work_units()==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
   let demand=self.demands()?;
   if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"returned read witness exceeds admitted depth"));}
   if grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
   if let Some(registry)=self.registry.as_ref(){
    if registry.strong_count()==1&&registry.contains(self.index,self.generation){return Ok(RetainedCloneStep::Progress(Default::default()));}
    pay_pure_work(effect,&mut work)?;
    let Some(owned)=self.registry.take()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original returned read registry"))};
    let last=owned.into_inner();
    let released=if let Some(registry)=last{*self.last_registry=Some(registry);demand.release_bytes}else{0};
    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}));
   }
   let Some(owner)=self.last_registry.as_mut()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"last original registry custody"))};
   if !owner.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"last returned read registry still requires its store-owned typed retirement pump"));}
   if demand.release_bytes!=0{let turn=owner.close_owned_backing_step(effect);work+=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()));}
   pay_pure_work(effect,&mut work)?;
   self.last_registry.take();
   Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
  })();
  RetainedWorkOutcome{result,work}
 }
}
impl RetireOwned for SnapshotReadReturn {
 type RetirementCursor=SnapshotReadReturnRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{SnapshotReadReturnRetirement::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SnapshotReadReturnRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for SnapshotReadReturnRetirement {
    fn next_demand(&self, _body: usize) -> Result<RetirementDemand, ValueError> { self.demands() }
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{let turn=self.step(effect);match turn.result{Err(error)=>RetirementStep::Failure(error.into(),turn.work),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work)}}
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}if self.registry.is_none()&&self.demands()?.release_bytes!=0{if let Some(owner)=self.last_registry.as_ref(){return owner.owned_backing_work_demand()}}Ok(1)}
 fn terminal_is_empty(&self)->bool{self.registry.is_none()&&self.last_registry.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,_copy:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands().ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for SnapshotReadReturnRetirement {
 fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"returned read witness abandoned original registry custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.registry);ManuallyDrop::drop(&mut self.last_registry);}}}
}
/// 🗳️ An erased original read hands its registry slot back as an exact witness before that witness retires through its own typed cursor.
#[doc(hidden)]
pub struct ErasedSnapshotReadRetirement {
 read:ManuallyDrop<Option<super::ErasedSnapshotRead>>,
 witness:ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<SnapshotReadReturn>>>,
}
impl ErasedSnapshotReadRetirement {
 fn new(read:super::ErasedSnapshotRead)->Self {Self{read:ManuallyDrop::new(Some(read)),witness:ManuallyDrop::new(None)}}
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.read.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<SnapshotReadReturn>()+size_of::<u64>()+size_of::<usize>(),depth:2,..Default::default()});}
  let Some(owner)=self.witness.as_ref()else{return Ok(Default::default())};
  let depth=owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"erased read witness depth overflow"))?;
  Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth})
 }
 fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();
  let done=|result:Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,work:u128|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
  let refuse=|error:ValueError|done(Err(error.into()),0);
  if self.terminal_is_empty(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0);}
  if grant.maximum_items==0||effect.maximum_work_units()==0{return done(Ok(RetainedCloneStep::Progress(Default::default())),0);}
  let demand=match self.demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return refuse(error)};
  if grant.maximum_depth<demand.depth{return refuse(ValueError::literal(ValueRefusalKind::DepthLimit,"erased read closure exceeds admitted depth"));}
  if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return done(Ok(RetainedCloneStep::Progress(Default::default())),0);}
  if let Some(read)=self.read.take(){
   let before=effect.performed_work();
   return match read.try_return_to_registry_witness(effect){
    Ok((witness,progress))=>{let work=effect.performed_work().saturating_sub(before);match semio_framework_value::retirement::controlled::ControlledRetirement::new(witness){Ok(witness)=>{*self.witness=Some(witness);done(Ok(RetainedCloneStep::Progress(progress)),work)},Err((error,_))=>done(Err(error.into()),work)}},
    Err((error,read))=>{*self.read=Some(read);refuse(error)},
   };
  }
  let Some(owner)=self.witness.as_mut()else{return refuse(ValueError::literal(ValueRefusalKind::InvariantViolated,"erased read witness remains until terminal"))};
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  let turn=match owner.step(&mut effect.nested(child)){RetainedWorkPoll::Pending{work}=>return RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>turn};
  let step=match turn.result{Ok(step)=>step,Err(error)=>return done(Err(error),turn.work)};
  if owner.terminal_is_empty(){self.witness.take();return done(Ok(RetainedCloneStep::Complete(step.progress())),turn.work);}
  done(Ok(RetainedCloneStep::Progress(step.progress())),turn.work)
 }
}
impl RetireOwned for super::ErasedSnapshotRead {
 type RetirementCursor=ErasedSnapshotReadRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{ErasedSnapshotReadRetirement::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<ErasedSnapshotReadRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for ErasedSnapshotReadRetirement {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.demands(body) }
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{match self.step(effect){RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),RetainedWorkPoll::Complete(turn)=>original_outcome_retirement_step(turn)}}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.read.is_some(){return Ok(1)}self.witness.as_ref().map_or(Ok(0),|owner|owner.next_computational_work(body))}
 fn terminal_is_empty(&self)->bool{self.read.is_none()&&self.witness.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for ErasedSnapshotReadRetirement {
 fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"erased read custody requires full granted closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.read);ManuallyDrop::drop(&mut self.witness);}}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;