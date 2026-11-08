//! 🚚️ Admitted collection edits move original headers through their actual iterator and clone only authored additions.
use semio_framework_value_original_vector::OriginalVectorCursor;
use crate::{IndexedEditPlan,IndexedEditKind,IndexedOutputCursor,IndexedOutputWork,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedFieldCursor,RetainedCloneClose,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,retained_clone_close_demands,settle_child_close,take_child_output,admit_retained_clone_progress}};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum PendingKind{Original,Removed,Added}
/// 🧳️ The unpublished target lends its original vector once; every survivor retains its actual payload allocation.
pub struct IndexedCollectionCommit<T:RetainedClone>{original:ManuallyDrop<Option<OriginalVectorCursor<T>>>,output:ManuallyDrop<Option<Vec<T>>>,pending:ManuallyDrop<Option<T>>,pending_kind:PendingKind,field:RetainedFieldCursor<T>,retirement:RetainedCloneClose,cursor:IndexedOutputCursor,phase:u8,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"indexed collection transfer lost its admitted original ownership")}
impl<T:RetainedClone> Default for IndexedCollectionCommit<T>{fn default()->Self{Self::new()}}
impl<T:RetainedClone> IndexedCollectionCommit<T>{
 pub fn new()->Self{Self{original:ManuallyDrop::new(None),output:ManuallyDrop::new(None),pending:ManuallyDrop::new(None),pending_kind:PendingKind::Original,field:Default::default(),retirement:Default::default(),cursor:Default::default(),phase:0,closing:false}}
 fn fixed()->usize{0}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(Self::fixed()).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(Self::fixed()),maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
 fn receipt(mut progress:RetainedCloneProgress)->RetainedCloneProgress{progress.copied_bytes+=Self::fixed();progress}
 fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:Self::fixed(),depth:1,..Default::default()}}
 pub fn complete(&self)->bool{self.phase==6}
 pub fn next_added_ordinal(&self,plan:&IndexedEditPlan)->Result<Option<usize>,ValueError>{if self.phase==3{match self.cursor.next(plan)?{IndexedOutputWork::Added{ordinal}=>Ok(Some(ordinal)),_=>Err(invalid())}}else{Ok(None)}}
 pub fn demand(&self,plan:&IndexedEditPlan,post:&Vec<T>,added:Option<RetainedCloneRef<'_,T>>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||!plan.ready()||!T::controlled_retirement_supported(){return Err(invalid());}
  if self.phase<2&&post.len()!=plan.shape().base_len||self.phase>=2&&self.phase<6&&(!post.is_empty()||post.capacity()!=0){return Err(invalid());}
  match self.phase{
   0=>{let mut demand=Self::scalar();if !plan.entries(IndexedEditKind::Removed)?.is_empty()||!plan.entries(IndexedEditKind::Added)?.is_empty(){demand.capacity_bytes=plan.output_len()?.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed output directory extent overflow"))?;}Ok(demand)},
   1=>Self::nested(OriginalVectorCursor::<T>::constructor_demand()),
   2=>match self.cursor.next(plan)?{IndexedOutputWork::Original{..}|IndexedOutputWork::Removed{..}=>Self::nested(self.original.as_ref().ok_or_else(invalid)?.next_take_demand()),_=>Ok(Self::scalar())},
   3 if self.field.output_ready()=>Self::nested(self.field.take_demand()?),
   3=>Self::nested(self.field.advance_demands(added.ok_or_else(invalid)?,body,maximum_work_units)?),
   4 if self.pending_kind==PendingKind::Removed=>Self::nested(self.retirement.next_owner_demands::<T>(self.pending.is_some(),&None,body.saturating_sub(Self::fixed()))?),
   4=>Ok(RetirementDemand{copy_bytes:size_of::<T>(),..Self::scalar()}),
   5 if self.field.terminal_is_empty()=>Ok(Self::scalar()),
   5=>Self::nested(if self.field.close_started(){retained_clone_close_demands::<T>(&self.field,body.saturating_sub(Self::fixed()))?}else{self.field.begin_close_demand()?}),
   6=>Ok(Self::scalar()),
   _=>Err(invalid()),
  }
 }
 pub fn advance(&mut self,plan:&IndexedEditPlan,post:&mut Vec<T>,added:Option<RetainedCloneRef<'_,T>>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(||->Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
  if self.complete(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.demand(plan,post,added,body,maximum_work_units)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed(),..Default::default()};
  match self.phase{
   0=>{if plan.entries(IndexedEditKind::Removed)?.is_empty()&&plan.entries(IndexedEditKind::Added)?.is_empty(){self.phase=6;work+=1;return Ok(RetainedCloneStep::Complete(progress));}work+=1;let output=crate::allocate_empty_vec::<T>(plan.output_len()?,effect)?;progress.retained_capacity_bytes=output.capacity()*size_of::<T>();*self.output=Some(output);self.phase=1;},
   1=>{let turn=OriginalVectorCursor::admit(std::mem::take(post),Self::child(grant));work+=turn.work;match turn.result{Ok((original,p))=>{*self.original=Some(original);self.phase=2;return Ok(RetainedCloneStep::Progress(Self::receipt(p)));},Err((error,original))=>{*post=original;return Err(error.into());}}},
   2=>match self.cursor.next(plan)?{
    IndexedOutputWork::Original{..}|IndexedOutputWork::Removed{..}=>{self.pending_kind=if matches!(self.cursor.next(plan)?,IndexedOutputWork::Removed{..}){PendingKind::Removed}else{PendingKind::Original};let turn=self.original.as_mut().ok_or_else(invalid)?.take_front(Self::child(grant));work+=turn.work;let(value,p)=turn.result?.ok_or_else(invalid)?;*self.pending=Some(value);self.phase=4;return Ok(RetainedCloneStep::Progress(Self::receipt(p)));},
    IndexedOutputWork::Added{..}=>{self.phase=3;work+=1;},
    IndexedOutputWork::Complete=>{if !self.original.as_ref().ok_or_else(invalid)?.remaining().is_empty()||self.output.as_ref().ok_or_else(invalid)?.len()!=plan.output_len()?{return Err(invalid().into());}*post=self.output.take().ok_or_else(invalid)?;self.phase=6;work+=1;return Ok(RetainedCloneStep::Complete(progress));},
   },
   3=>{if self.field.output_ready(){let child=Self::child(grant);let Some(p)=take_child_output(&mut self.field,&mut *self.pending,child,effect)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};work+=1;self.pending_kind=PendingKind::Added;self.phase=4;let p=admit_retained_clone_progress(child,p,"indexed authored addition take")?;return Ok(RetainedCloneStep::Progress(Self::receipt(p)));}else{return {let turn=self.field.advance(added.ok_or_else(invalid)?,body,maximum_work_units,&mut effect.nested_within(Self::child(grant),maximum_work_units));work+=turn.work;turn.result}.map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}},
   4 if self.pending_kind==PendingKind::Removed=>{if !self.retirement.is_empty(){return {let turn=settle_child_close(self.retirement.step_granted(&mut effect.nested(Self::child(grant))));work+=turn.work;turn.result}.map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}if let Some(step)={let turn=self.retirement.begin_granted(&mut self.pending,&mut effect.nested(Self::child(grant)));work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.cursor.advance(plan)?;self.phase=2;work+=1;},
   4=>{self.output.as_mut().ok_or_else(invalid)?.push(self.pending.take().ok_or_else(invalid)?);work+=1;progress.copied_bytes=size_of::<T>();self.cursor.advance(plan)?;self.phase=if self.pending_kind==PendingKind::Added{5}else{2};},
   5=>{if self.field.terminal_is_empty(){self.field=Default::default();self.phase=2;work+=1;}else if !self.field.close_started(){let turn=self.field.begin_close_paid(&mut effect.nested(Self::child(grant)));work+=turn.work;return turn.result.map(|p|RetainedCloneStep::Progress(Self::receipt(p))).map_err(Into::into);}else{return {let turn=settle_child_close(self.field.close_step(&mut effect.nested(Self::child(grant))));work+=turn.work;turn.result}.map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}},
   6=>return Ok(RetainedCloneStep::Complete(Default::default())),
   _=>return Err(invalid().into()),
  }
  Ok(RetainedCloneStep::Progress(progress))
 })();crate::finish(effect,work,result)}

 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.field.terminal_is_empty(){return Self::nested(if self.field.close_started(){retained_clone_close_demands::<T>(&self.field,body.saturating_sub(Self::fixed()))?}else{self.field.begin_close_demand()?});}
  if !self.retirement.is_empty()||self.pending.is_some(){return Self::nested(self.retirement.next_owner_demands::<T>(self.pending.is_some(),&None,body.saturating_sub(Self::fixed()))?);}
  if self.output.is_some(){return Self::nested(self.retirement.next_owner_demands::<Vec<T>>(true,&None,body.saturating_sub(Self::fixed()))?);}
  if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(Self::scalar())}else{Self::nested(original.next_demand(body.saturating_sub(Self::fixed()))?)};}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(||->Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
  if !self.closing{return Err(invalid().into());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let child=Self::child(grant);
  if !self.field.terminal_is_empty(){if !self.field.close_started(){let turn=self.field.begin_close_paid(&mut effect.nested(child));work+=turn.work;return turn.result.map(|p|RetainedCloneStep::Progress(Self::receipt(p))).map_err(Into::into);}return {let turn=settle_child_close(self.field.close_step(&mut effect.nested(child)));work+=turn.work;turn.result}.map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if !self.retirement.is_empty(){return {let turn=settle_child_close(self.retirement.step_granted(&mut effect.nested(child)));work+=turn.work;turn.result}.map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if let Some(step)={let turn=self.retirement.begin_granted(&mut self.pending,&mut effect.nested(child));work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if let Some(step)={let turn=self.retirement.begin_granted(&mut self.output,&mut effect.nested(child));work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if let Some(original)=self.original.as_mut(){if original.terminal_is_empty(){self.original.take();work+=1;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed(),..Default::default()}));}if !original.close_started(){let turn=original.begin_close_paid(&mut effect.nested(child));work+=turn.work;return turn.result.map(|p|RetainedCloneStep::Progress(Self::receipt(p))).map_err(Into::into);}return {let turn=settle_child_close(original.step(&mut effect.nested(child)));work+=turn.work;turn.result}.map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 })();crate::finish(effect,work,result)}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.field.terminal_is_empty()&&self.retirement.is_empty()&&self.pending.is_none()&&self.output.is_none()&&self.original.is_none()}
}
impl<T:RetainedClone> Drop for IndexedCollectionCommit<T>{fn drop(&mut self){let empty=self.original.is_none()&&self.output.is_none()&&self.pending.is_none()&&self.retirement.is_empty();assert!(std::thread::panicking()||empty,"indexed collection commit requires original child-first closure");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.pending);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
