//! 🚚️ Admitted collection edits move original headers through their actual iterator and clone only authored additions.
use semio_framework_value_original_vector::OriginalVectorCursor;
use semio_framework_value_control_contract::{OriginalEffectInvocationIdentity,OriginalEffectReply};
use crate::{IndexedEditPlan,IndexedEditKind,IndexedOutputCursor,IndexedOutputWork,RetirementDemand,ValueError,ValueRefusalKind,indexed_edit::{CloseKey,Done,Reply,invocation_identity},retained_clone::{RetainedClone,RetainedCloneCursor,RetainedFieldCursor,RetainedCloneClose,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,retained_clone_close_demands,forward_original_value_reply}};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum PendingKind{Original,Removed,Added}
type AdvanceKey=(OriginalEffectInvocationIdentity,u8,usize,u64);
macro_rules! pay{($effect:expr,$reply:expr,$what:expr)=>{if !$effect.performed(1)?{return Ok($reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,$what).into())))}};}
/// 🧳️ The unpublished target lends its original vector once; every survivor retains its actual payload allocation.
pub struct IndexedCollectionCommit<T:RetainedClone>{original:ManuallyDrop<Option<OriginalVectorCursor<T>>>,output:ManuallyDrop<Option<Vec<T>>>,pending:ManuallyDrop<Option<T>>,pending_kind:PendingKind,field:RetainedFieldCursor<T>,retirement:RetainedCloneClose,cursor:IndexedOutputCursor,phase:u8,closing:bool,advance_route:Option<AdvanceKey>,close_route:Option<CloseKey>,copy_reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"indexed collection transfer lost its admitted original ownership")}
impl<T:RetainedClone> Default for IndexedCollectionCommit<T>{fn default()->Self{Self::new()}}
impl<T:RetainedClone> IndexedCollectionCommit<T>{
 pub fn new()->Self{Self{original:ManuallyDrop::new(None),output:ManuallyDrop::new(None),pending:ManuallyDrop::new(None),pending_kind:PendingKind::Original,field:Default::default(),retirement:Default::default(),cursor:Default::default(),phase:0,closing:false,advance_route:None,close_route:None,copy_reply:ManuallyDrop::new(None)}}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
 fn scalar()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 pub fn complete(&self)->bool{self.phase==6}
 pub fn next_added_ordinal(&self,plan:&IndexedEditPlan)->Result<Option<usize>,ValueError>{if self.phase==3{match self.cursor.next(plan)?{IndexedOutputWork::Added{ordinal}=>Ok(Some(ordinal)),_=>Err(invalid())}}else{Ok(None)}}
 pub fn demand(&self,plan:&IndexedEditPlan,post:&Option<Vec<T>>,added:Option<RetainedCloneRef<'_,T>>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||!plan.ready()||!T::controlled_retirement_supported(){return Err(invalid());}
  if self.phase<2&&!post.as_ref().is_some_and(|rows|rows.len()==plan.shape().base_len)||self.phase>=2&&self.phase<6&&post.is_some(){return Err(invalid());}
  match self.phase{
   0=>{let mut demand=Self::scalar();if !plan.entries(IndexedEditKind::Removed)?.is_empty()||!plan.entries(IndexedEditKind::Added)?.is_empty(){demand.capacity_bytes=plan.output_len()?.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed output directory extent overflow"))?;}Ok(demand)},
   1=>Self::nested(OriginalVectorCursor::<T>::constructor_demand()),
   2=>match self.cursor.next(plan)?{IndexedOutputWork::Original{..}|IndexedOutputWork::Removed{..}=>Self::nested(self.original.as_ref().ok_or_else(invalid)?.next_take_demand()),_=>Ok(Self::scalar())},
   3 if self.field.output_ready()=>Self::nested(self.field.take_demand()?),
   3=>Self::nested(self.field.advance_demands(added.ok_or_else(invalid)?,body,maximum_work_units)?),
   4 if self.pending_kind==PendingKind::Removed=>Self::nested(self.retirement.next_owner_demands::<T>(self.pending.is_some(),&None,body)?),
   4=>Ok(RetirementDemand{copy_bytes:size_of::<T>(),..Self::scalar()}),
   5 if self.field.terminal_is_empty()=>Ok(Self::scalar()),
   5=>Self::nested(if self.field.close_started(){retained_clone_close_demands::<T>(&self.field,body)?}else{self.field.begin_close_demand()?}),
   6=>Ok(Self::scalar()),
   _=>Err(invalid()),
  }
 }
 /// 📥️ Records one admitted commit action directly into the caller's original reply.
 pub fn advance_into<'r>(&mut self,plan:&IndexedEditPlan,post:&mut Option<Vec<T>>,added:Option<RetainedCloneRef<'_,T>>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(plan,post,added,body,maximum_work_units,effect,reply,false)}
 /// 🔁️ Recovers only the unfinished child invocation of the same admitted commit action.
 pub fn recover_advance_into<'r>(&mut self,plan:&IndexedEditPlan,post:&mut Option<Vec<T>>,added:Option<RetainedCloneRef<'_,T>>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(plan,post,added,body,maximum_work_units,effect,reply,true)}
 fn advance_reply<'r>(&mut self,plan:&IndexedEditPlan,post:&mut Option<Vec<T>>,added:Option<RetainedCloneRef<'_,T>>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let identity=invocation_identity(effect,recover)?;let maximum=maximum_work_units.min(effect.maximum_work_units());
  if self.close_route.is_some()||self.closing{return Err(invalid())}
  if recover{let key=self.advance_route.ok_or_else(invalid)?;if key.0!=identity||key.2!=body||key.3!=maximum{return Err(ValueError::invariant("indexed collection commit recovery changed its invocation, body or Work"))}return self.advance_invoke(key.1,true,key,added,body,maximum,effect,reply)}
  if self.advance_route.is_some()||self.copy_reply.is_some(){return Err(ValueError::invariant("indexed collection commit requires recovery"))}
  if self.complete(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let grant=effect.grant();let demand=self.demand(plan,post,added,body,maximum)?;if !retained_clone_grant_admits(grant,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};let child=Self::child(grant);
  match self.phase{
   0=>{
    pay!(effect,reply,"indexed collection commit exceeded its original work");
    if plan.entries(IndexedEditKind::Removed)?.is_empty()&&plan.entries(IndexedEditKind::Added)?.is_empty(){self.phase=6;return Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress))))}
    let output=match crate::allocate_empty_vec::<T>(plan.output_len()?,effect){Ok(output)=>output,Err(error)=>return Ok(reply.complete(Err(error.into())))};progress.retained_capacity_bytes=output.capacity()*size_of::<T>();*self.output=Some(output);self.phase=1;
   },
   1=>{
    let inner=OriginalEffectReply::borrow(&mut self.copy_reply)?;let done=OriginalVectorCursor::admit_into(post,&mut self.original,&mut effect.nested(child),inner)?;drop(done);
    let answer=self.copy_reply.take().ok_or_else(||ValueError::invariant("original vector admission left its reply pending"))?;if answer.is_ok()&&self.original.is_some(){self.phase=2}
    return Ok(reply.complete(answer.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)));
   },
   2=>match self.cursor.next(plan)?{
    IndexedOutputWork::Original{..}|IndexedOutputWork::Removed{..}=>{
     self.pending_kind=if matches!(self.cursor.next(plan)?,IndexedOutputWork::Removed{..}){PendingKind::Removed}else{PendingKind::Original};
     let inner=OriginalEffectReply::borrow(&mut self.copy_reply)?;let done=self.original.as_mut().ok_or_else(invalid)?.take_front_into(&mut self.pending,&mut effect.nested(child),inner)?;drop(done);
     let answer=self.copy_reply.take().ok_or_else(||ValueError::invariant("original vector row transfer left its reply pending"))?;if answer.is_ok()&&self.pending.is_some(){self.phase=4}
     return Ok(reply.complete(answer.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)));
    },
    IndexedOutputWork::Added{..}=>{pay!(effect,reply,"indexed collection commit exceeded its original work");self.phase=3;},
    IndexedOutputWork::Complete=>{
     if !self.original.as_ref().ok_or_else(invalid)?.remaining().is_empty()||self.output.as_ref().ok_or_else(invalid)?.len()!=plan.output_len()?{return Err(invalid())}
     pay!(effect,reply,"indexed collection commit exceeded its original work");*post=self.output.take();self.phase=6;return Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress))));
    },
   },
   3 if self.field.output_ready()=>{let key=(identity,4,body,maximum);self.advance_route=Some(key);return self.advance_invoke(4,false,key,added,body,maximum,effect,reply)},
   3=>{let key=(identity,3,body,maximum);self.advance_route=Some(key);return self.advance_invoke(3,false,key,added,body,maximum,effect,reply)},
   4 if self.pending_kind==PendingKind::Removed=>{
    if !self.retirement.is_empty(){let key=(identity,5,body,maximum);self.advance_route=Some(key);return self.advance_invoke(5,false,key,added,body,maximum,effect,reply)}
    match self.retirement.begin_granted(&mut self.pending,&mut effect.nested(child)).result{Ok(Some(step))=>return Ok(reply.complete(Ok(RetainedCloneStep::Progress(step.progress())))),Ok(None)=>{},Err(error)=>return Ok(reply.complete(Err(error.into())))}
    pay!(effect,reply,"indexed collection commit exceeded its original work");self.cursor.advance(plan)?;self.phase=2;
   },
   4=>{pay!(effect,reply,"indexed collection commit exceeded its original work");self.output.as_mut().ok_or_else(invalid)?.push(self.pending.take().ok_or_else(invalid)?);progress.copied_bytes=size_of::<T>();self.cursor.advance(plan)?;self.phase=if self.pending_kind==PendingKind::Added{5}else{2};},
   5=>{
    if self.field.terminal_is_empty(){pay!(effect,reply,"indexed collection commit exceeded its original work");self.field=Default::default();self.phase=2;}
    else if !self.field.close_started(){let turn=self.field.begin_close_paid(&mut effect.nested(child));return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
    else{let key=(identity,6,body,maximum);self.advance_route=Some(key);return self.advance_invoke(6,false,key,added,body,maximum,effect,reply)}
   },
   6=>return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default())))),
   _=>return Err(invalid()),
  }
  Ok(reply.complete(Ok(RetainedCloneStep::Progress(progress))))
 }
 fn advance_invoke<'r>(&mut self,route:u8,recover:bool,key:AdvanceKey,added:Option<RetainedCloneRef<'_,T>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{
  let child=Self::child(effect.grant());let mut nested=effect.nested_within(child,maximum);let mut scope=reply.progress().scope(&mut self.advance_route,key)?;
  if route==4{
   if self.copy_reply.is_none(){let inner=OriginalEffectReply::borrow(&mut self.copy_reply)?;let done=if recover{self.field.recover_take(&mut self.pending,&mut nested,inner)}else{self.field.take(&mut self.pending,&mut nested,inner)}?;drop(done)}
   if let Some(Ok(step))=self.copy_reply.as_ref(){if !step.progress().fits(child){*self.copy_reply=Some(Err(ValueError::invariant("indexed authored addition take exceeded its original grant")))}else if self.pending.is_some(){self.pending_kind=PendingKind::Added;self.phase=4}}
   let done=forward_original_value_reply(&mut self.copy_reply,scope.borrow()?)?;drop(done);return Ok(scope.forwarded())
  }
  let inner=scope.borrow()?;
  let done=match route{
   3=>{let source=added.ok_or_else(invalid)?;if recover{self.field.recover_advance_into(source,body,maximum,&mut nested,inner)}else{self.field.advance_into(source,body,maximum,&mut nested,inner)}},
   5=>crate::indexed_edit::CloseReply::close_reply(&mut self.retirement,None,body,&mut nested,inner,recover),
   6=>if recover{self.field.recover_close_into(body,&mut nested,inner)}else{self.field.close_into(body,&mut nested,inner)},
   _=>Err(ValueError::invariant("indexed collection commit route changed")),
  }?;drop(done);Ok(scope.forwarded())
 }

 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.field.terminal_is_empty(){return Self::nested(if self.field.close_started(){retained_clone_close_demands::<T>(&self.field,body)?}else{self.field.begin_close_demand()?});}
  if !self.retirement.is_empty()||self.pending.is_some(){return Self::nested(self.retirement.next_owner_demands::<T>(self.pending.is_some(),&None,body)?);}
  if self.output.is_some(){return Self::nested(self.retirement.next_owner_demands::<Vec<T>>(true,&None,body)?);}
  if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(Self::scalar())}else{Self::nested(original.next_demand(body)?)};}
  Ok(Default::default())
 }
 /// 📬️ Records one ordinary close directly into the caller's reply.
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply_commit(body,effect,reply,false)}
 /// 🔁️ Recovers only the unfinished child close of the same invocation.
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.close_reply_commit(body,effect,reply,true)}
 fn close_reply_commit<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if !self.closing{return Err(invalid());}
  if self.advance_route.is_some()||self.copy_reply.is_some(){return Err(ValueError::invariant("indexed collection commit advance must settle before close"))}
  let identity=invocation_identity(effect,recover)?;
  if recover{let key=self.close_route.ok_or_else(invalid)?;if key.0!=identity||key.3.is_some(){return Err(ValueError::invariant("indexed collection commit close recovery changed its invocation"))}return self.close_invoke(key.1,true,key,key.2,effect,reply)}
  if self.close_route.is_some(){return Err(ValueError::invariant("indexed collection commit close requires recovery"))}
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let grant=effect.grant();let demand=self.close_demand(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let child=Self::child(grant);
  if !self.field.terminal_is_empty(){
   if !self.field.close_started(){let turn=self.field.begin_close_paid(&mut effect.nested(child));return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
   let key=(identity,1,body,None);self.close_route=Some(key);return self.close_invoke(1,false,key,body,effect,reply)
  }
  if !self.retirement.is_empty(){let key=(identity,2,body,None);self.close_route=Some(key);return self.close_invoke(2,false,key,body,effect,reply)}
  for slot in [0,1]{let turn=if slot==0{self.retirement.begin_granted(&mut self.pending,&mut effect.nested(child))}else{self.retirement.begin_granted(&mut self.output,&mut effect.nested(child))};match turn.result{Ok(Some(step))=>return Ok(reply.complete(Ok(RetainedCloneStep::Progress(step.progress())))),Ok(None)=>{},Err(error)=>return Ok(reply.complete(Err(error.into())))}}
  if let Some(original)=self.original.as_mut(){
   if original.terminal_is_empty(){pay!(effect,reply,"indexed collection commit exceeded its original work");self.original.take();return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))))}
   if !original.close_started(){let turn=original.begin_close_paid(&mut effect.nested(child));return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
   let key=(identity,3,body,None);self.close_route=Some(key);return self.close_invoke(3,false,key,body,effect,reply)
  }
  Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))
 }
 fn close_invoke<'r>(&mut self,route:u8,recover:bool,key:CloseKey,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{
  let child=Self::child(effect.grant());let mut nested=effect.nested(child);let mut scope=reply.progress().scope(&mut self.close_route,key)?;let inner=scope.borrow()?;
  let done=match route{
   1=>if recover{self.field.recover_close_into(body,&mut nested,inner)}else{self.field.close_into(body,&mut nested,inner)},
   2=>crate::indexed_edit::CloseReply::close_reply(&mut self.retirement,None,body,&mut nested,inner,recover),
   3=>{let original=self.original.as_mut().ok_or_else(invalid)?;if recover{original.recover_close_into(body,&mut nested,inner)}else{original.close_into(body,&mut nested,inner)}},
   _=>Err(ValueError::invariant("indexed collection commit close route changed")),
  }?;drop(done);Ok(scope.forwarded())
 }

 pub fn terminal_is_empty(&self)->bool{self.closing&&self.field.terminal_is_empty()&&self.retirement.is_empty()&&self.pending.is_none()&&self.output.is_none()&&self.original.is_none()&&self.advance_route.is_none()&&self.close_route.is_none()&&self.copy_reply.is_none()}
}
#[cfg(feature="testkit")]
impl<T:RetainedClone> IndexedCollectionCommit<T>{
 /// 🔬️ Observes the unchanged remaining original rows.
 pub fn remaining(&self)->&[T]{self.original.as_ref().map_or(&[],OriginalVectorCursor::remaining)}
}
impl<T:RetainedClone> crate::indexed_edit::CloseReply for IndexedCollectionCommit<T>{fn close_reply<'r>(&mut self,_:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.close_reply_commit(body,effect,reply,recover)}}
impl<T:RetainedClone> Drop for IndexedCollectionCommit<T>{fn drop(&mut self){let empty=self.original.is_none()&&self.output.is_none()&&self.pending.is_none()&&self.retirement.is_empty()&&self.advance_route.is_none()&&self.close_route.is_none()&&self.copy_reply.is_none();assert!(std::thread::panicking()||empty,"indexed collection commit requires original child-first closure");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.copy_reply);}}}}
