//! 🧯️ Compact final-owner lease close: the sealed cursor and the lease cursor retire inline, one phase at a time, without a paged frontier or close-call frame.
use super::*;
use retirement::{RetirementCursor,RetirementStep,controlled::{borrowed_child_step,RetainedOwnerGate}};
use shared_issuance::SealedShared;
pub(crate) type SealedCursor=shared_issuance::Cursor<RetainedCloneLeaseOwner>;
pub(crate) type LeaseCursor=Box<dyn RetirementCursor>;
/// 🪪️ Holds only the phase that is alive: the counted lease, its sealed close, its lease close, or nothing.
pub(crate) enum BindingState{Live(SealedShared<RetainedCloneLeaseOwner>),Sealed(SealedCursor),Lease(RetainedOwnerGate<LeaseCursor>),Closed}
/// 🧭️ Selects how the lease close is driven: a borrowed reply (fresh or recovered) or a direct original loan.
pub(crate) enum Drive{Reply{recover:bool},Loan}
fn poll(result:Result<RetainedCloneStep,OriginalRetirementFailure>,work:u128)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}
fn invariant(message:&'static str)->OriginalRetirementFailure{ValueError::invariant(message).into()}
impl BindingState{
 pub(crate) fn is_live(&self)->bool{matches!(self,Self::Live(_))}
 pub(crate) fn is_closed(&self)->bool{matches!(self,Self::Closed)}
 pub(crate) fn transition_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<SealedShared<RetainedCloneLeaseOwner>>()+size_of::<Self>(),depth:1,..Default::default()}}
 pub(crate) fn closed_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<Option<RetainedCloneBinding>>(),depth:1,..Default::default()}}
 fn pop_demand(cursor:&LeaseCursor)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<usize>()+size_of::<LeaseCursor>(),release_bytes:cursor.terminal_release_bytes().ok_or_else(||ValueError::invariant("retirement owner has no controlled terminal release authority"))?,depth:1,..Default::default()})}
 pub(crate) fn phase_demand(&self,body:usize)->Result<(RetirementDemand,u64),ValueError>{
  match self{
   Self::Sealed(cursor)=>{if let Some(quote)=cursor.unwrapped().and_then(Self::cold_quote){return Ok(quote)}Ok((cursor.next_demand(body)?,cursor.next_computational_work(body)?))},
   Self::Lease(gate)=>{let cursor=gate.try_lock().map_err(|_|super::close_query_busy())?;if cursor.terminal_is_empty(){Ok((Self::pop_demand(&cursor)?,1))}else{Ok((cursor.next_demand(body)?,cursor.next_computational_work(body)?))}},
   Self::Live(_)|Self::Closed=>Err(ValueError::invariant("compact close has no selected phase")),
  }
 }
 fn cold_quote(owner:&RetainedCloneLeaseOwner)->Option<(RetirementDemand,u64)>{
  let payload=owner.payload.as_ref()?;let(bytes,work)=payload.cold_close_quote()?;
  Some((RetirementDemand{copy_bytes:size_of::<Option<Box<dyn ErasedSnapshotRetirement>>>(),release_bytes:size_of_val(&**payload).checked_add(bytes)?,depth:1,..Default::default()},work.checked_add(1)?))
 }
 pub(crate) fn requires_original_birth(&self)->bool{matches!(self,Self::Lease(gate) if gate.try_lock().map_or(true,|cursor|cursor.requires_original_birth()))}
 pub(crate) fn requires_original_control(&self)->bool{matches!(self,Self::Lease(gate) if gate.try_lock().map_or(true,|cursor|cursor.requires_original_control()))}
 fn mapped(step:RetirementStep,grant:RetainedCloneGrant)->Result<(RetainedCloneStep,u128),OriginalRetirementFailure>{
  let work=step.work();
  match step{
   RetirementStep::Progress(progress,_)if progress.fits(grant)=>Ok((RetainedCloneStep::Progress(progress),work)),
   RetirementStep::Progress(..)=>Err(invariant("compact lease close receipt exceeded its grant")),
   RetirementStep::BudgetExhausted=>Ok((RetainedCloneStep::Progress(Default::default()),work)),
   RetirementStep::Failure(error,_)=>Err(error),
   _=>Err(invariant("compact lease close produced a structural child")),
  }
 }
 pub(crate) fn step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,drive:Drive,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let recover=matches!(drive,Drive::Reply{recover:true});let empty=||Ok(RetainedCloneStep::Progress(Default::default()));
  let(demand,work)=if recover{(Default::default(),0)}else{match self.phase_demand(body){Ok(quote)=>quote,Err(error)=>return poll(Err(error.into()),0)}};
  if !recover&&(work>effect.maximum_work_units()||grant.maximum_items==0||!retained_clone::retained_clone_grant_admits(grant,demand)){return poll(empty(),0)}
  match self{
   Self::Sealed(cursor)=>{
    if cursor.unwrapped().and_then(Self::cold_quote).is_some(){
     match effect.performed(work){Ok(true)=>{},Ok(false)=>return poll(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original binding close exceeded its work authority").into()),work.into()),Err(error)=>return poll(Err(error.into()),work.into())}
     let mut owner=cursor.take_unwrapped().expect("cold payload checked");
     let mut payload=owner.payload.take().expect("cold payload checked");payload.cold_close().expect("cold payload quoted");drop(payload);drop(owner);
     *self=Self::Closed;
     return poll(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})),work.into())
    }
    match cursor.close_step(body,effect){
    RetirementStep::ChildProgress(child,progress,work)=>{*self=Self::Lease(RetainedOwnerGate::new(child));poll(Ok(RetainedCloneStep::Progress(progress)),work)},
    step=>match Self::mapped(step,grant){Ok((step,work))=>poll(Ok(step),work),Err(error)=>poll(Err(error),0)},
    }
   },
   Self::Lease(gate)=>{
    let cursor=gate.get_mut();
    if cursor.terminal_is_empty(){
     match effect.performed(1){Ok(true)=>{},Ok(false)=>return poll(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original binding close exceeded its work authority").into()),1),Err(error)=>return poll(Err(error.into()),1)}
     *self=Self::Closed;return poll(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})),1)
    }
    let boundary=effect.performed_boundary();
    let child_grant=if recover{grant}else{RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes,..grant}};
    let maximum=if recover{match effect.remaining_work_units(){Ok(units)=>units,Err(error)=>return poll(Err(error.into()),0)}}else{work};
    let mut child=match effect.reborrow(child_grant,maximum){Ok(child)=>child,Err(error)=>return poll(Err(error.into()),0)};
    let step=match drive{
     Drive::Reply{recover}=>match borrowed_child_step(cursor.as_mut(),body,&mut child,&mut None,original.take(),recover){Ok(Some(step))=>step,Ok(None)=>return RetainedWorkPoll::Pending{work:effect.performed_since(boundary).unwrap_or(0)},Err(error)=>return poll(Err(error.into()),0)},
     Drive::Loan=>match original.take(){Some(original)=>cursor.as_mut().close_original_step(original,body,&mut child),None=>return poll(Err(invariant("compact lease close lost its original loan")),0)},
    };
    drop(child);
    match step{
     RetirementStep::Pending(work)=>RetainedWorkPoll::Pending{work},
     RetirementStep::Complete(work)=>{
      if !cursor.terminal_is_empty(){return poll(Err(invariant("compact lease close completed with retained custody")),work)}
      poll(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work)
     },
     step=>match Self::mapped(step,grant){Ok((step,work))=>poll(Ok(step),work),Err(error)=>poll(Err(error),0)},
    }
   },
   Self::Live(_)|Self::Closed=>poll(Err(invariant("compact close has no selected phase")),0),
  }
 }
 pub(crate) fn birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  let Self::Lease(gate)=self else{return poll(Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original binding has no selected child birth").into()),0)};
  let cursor=gate.get_mut();
  if !cursor.requires_original_birth(){return poll(Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"compact lease close has no selected original birth").into()),0)}
  let unfunded=|original:&mut OriginalRetirementCloseLoan<'_>|if original.failure_storage_birth().terminal_is_empty(){poll(Ok(RetainedCloneStep::Progress(Default::default())),0)}else{RetainedWorkPoll::Pending{work:0}};
  if grant.maximum_items==0{return unfunded(original)}
  let demand=match cursor.next_demand(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return poll(Err(error.into()),0)};
  if grant.maximum_depth<demand.depth{return poll(Err(ValueError::literal(ValueRefusalKind::DepthLimit,"controlled birth exceeds its admitted depth").into()),0)}
  if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes{return unfunded(original)}
  let child_grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand.capacity_bytes,..grant};
  match cursor.close_original_birth_step(original,child_grant){
   RetirementStep::Pending(work)=>RetainedWorkPoll::Pending{work},
   step=>match Self::mapped(step,child_grant){Ok((step,work))=>poll(Ok(step),work),Err(error)=>poll(Err(error),0)},
  }
 }
}
