//! 🚚️ Admitted collection edits move original headers through their actual iterator and clone only authored additions.
use crate::{IndexedEditPlan,IndexedEditKind,IndexedOutputCursor,IndexedOutputWork,RetirementDemand,ValueError,ValueRefusalKind,retirement::original_vector::OriginalVectorCursor,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedFieldCursor,RetainedCloneClose,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,retained_clone_close_demands}};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum PendingKind{Original,Removed,Added}
/// 🧳️ The unpublished target lends its original vector once; every survivor retains its actual payload allocation.
pub struct IndexedCollectionCommit<T:RetainedClone>{original:ManuallyDrop<Option<OriginalVectorCursor<T>>>,output:ManuallyDrop<Option<Vec<T>>>,pending:ManuallyDrop<Option<T>>,pending_kind:PendingKind,field:RetainedFieldCursor<T>,retirement:RetainedCloneClose,cursor:IndexedOutputCursor,phase:u8,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"indexed collection transfer lost its admitted original ownership")}
impl<T:RetainedClone> Default for IndexedCollectionCommit<T>{fn default()->Self{Self::new()}}
impl<T:RetainedClone> IndexedCollectionCommit<T>{
 pub fn new()->Self{Self{original:ManuallyDrop::new(None),output:ManuallyDrop::new(None),pending:ManuallyDrop::new(None),pending_kind:PendingKind::Original,field:Default::default(),retirement:Default::default(),cursor:Default::default(),phase:0,closing:false}}
 fn fixed()->usize{size_of::<Self>()*2+size_of::<T>()*2+size_of::<Vec<T>>()*4}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(Self::fixed()).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed(),maximum_depth:grant.maximum_depth-1,..grant}}
 fn receipt(mut progress:RetainedCloneProgress)->RetainedCloneProgress{progress.copied_bytes+=Self::fixed();progress}
 fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:Self::fixed(),depth:1,..Default::default()}}
 pub fn complete(&self)->bool{self.phase==6}
 pub fn next_added_ordinal(&self,plan:&IndexedEditPlan)->Result<Option<usize>,ValueError>{if self.phase==3{match self.cursor.next(plan)?{IndexedOutputWork::Added{ordinal}=>Ok(Some(ordinal)),_=>Err(invalid())}}else{Ok(None)}}
 pub fn demand(&self,plan:&IndexedEditPlan,post:&Vec<T>,added:Option<RetainedCloneRef<'_,T>>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||!plan.ready()||!T::controlled_retirement_supported(){return Err(invalid());}
  if self.phase<2&&post.len()!=plan.shape().base_len||self.phase>=2&&self.phase<6&&(!post.is_empty()||post.capacity()!=0){return Err(invalid());}
  match self.phase{
   0=>{let mut demand=Self::scalar();if !plan.entries(IndexedEditKind::Removed)?.is_empty()||!plan.entries(IndexedEditKind::Added)?.is_empty(){demand.capacity_bytes=plan.output_len()?.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed output directory extent overflow"))?;}Ok(demand)},
   1=>Self::nested(OriginalVectorCursor::<T>::constructor_demand()),
   2=>match self.cursor.next(plan)?{IndexedOutputWork::Original{..}|IndexedOutputWork::Removed{..}=>Self::nested(self.original.as_ref().ok_or_else(invalid)?.next_take_demand()),_=>Ok(Self::scalar())},
   3 if self.field.output_ready()=>Ok(Self::scalar()),
   3=>Self::nested(self.field.advance_demands(added.ok_or_else(invalid)?,body)?),
   4 if self.pending_kind==PendingKind::Removed=>Self::nested(self.retirement.next_owner_demands::<T>(self.pending.is_some(),&None,body.saturating_sub(Self::fixed()))?),
   4=>Ok(Self::scalar()),
   5 if self.field.terminal_is_empty()=>Ok(Self::scalar()),
   5=>Self::nested(retained_clone_close_demands::<T>(&self.field,body.saturating_sub(Self::fixed()))?),
   6=>Ok(Self::scalar()),
   _=>Err(invalid()),
  }
 }
 pub fn advance(&mut self,plan:&IndexedEditPlan,post:&mut Vec<T>,added:Option<RetainedCloneRef<'_,T>>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.demand(plan,post,added,body)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed(),..Default::default()};
  match self.phase{
   0=>{if plan.entries(IndexedEditKind::Removed)?.is_empty()&&plan.entries(IndexedEditKind::Added)?.is_empty(){self.phase=6;return Ok(RetainedCloneStep::Complete(progress));}let mut output=Vec::new();output.try_reserve_exact(plan.output_len()?).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"indexed output directory reservation refused"))?;progress.retained_capacity_bytes=output.capacity()*size_of::<T>();*self.output=Some(output);self.phase=1;},
   1=>{match OriginalVectorCursor::admit(std::mem::take(post),Self::child(grant)){Ok((original,p))=>{*self.original=Some(original);self.phase=2;return Ok(RetainedCloneStep::Progress(Self::receipt(p)));},Err((error,original))=>{*post=original;return Err(error);}}},
   2=>match self.cursor.next(plan)?{
    IndexedOutputWork::Original{..}|IndexedOutputWork::Removed{..}=>{self.pending_kind=if matches!(self.cursor.next(plan)?,IndexedOutputWork::Removed{..}){PendingKind::Removed}else{PendingKind::Original};let(value,p)=self.original.as_mut().ok_or_else(invalid)?.take_front(Self::child(grant))?.ok_or_else(invalid)?;*self.pending=Some(value);self.phase=4;return Ok(RetainedCloneStep::Progress(Self::receipt(p)));},
    IndexedOutputWork::Added{..}=>self.phase=3,
    IndexedOutputWork::Complete=>{if !self.original.as_ref().ok_or_else(invalid)?.remaining().is_empty()||self.output.as_ref().ok_or_else(invalid)?.len()!=plan.output_len()?{return Err(invalid());}*post=self.output.take().ok_or_else(invalid)?;self.phase=6;return Ok(RetainedCloneStep::Complete(progress));},
   },
   3=>{if self.field.output_ready(){*self.pending=Some(self.field.take().ok_or_else(invalid)?);self.pending_kind=PendingKind::Added;self.phase=4;}else{return self.field.advance(added.ok_or_else(invalid)?,body,Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}},
   4 if self.pending_kind==PendingKind::Removed=>{if !self.retirement.is_empty(){return self.retirement.step_granted(Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}if let Some(step)=self.retirement.begin_granted(&mut self.pending,Self::child(grant))?{return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.cursor.advance(plan)?;self.phase=2;},
   4=>{self.output.as_mut().ok_or_else(invalid)?.push(self.pending.take().ok_or_else(invalid)?);self.cursor.advance(plan)?;self.phase=if self.pending_kind==PendingKind::Added{self.field.begin_close();5}else{2};},
   5=>{if self.field.terminal_is_empty(){self.field=Default::default();self.phase=2;}else{return self.field.close_step(Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}},
   6=>return Ok(RetainedCloneStep::Complete(Default::default())),
   _=>return Err(invalid()),
  }
  Ok(RetainedCloneStep::Progress(progress))
 }
 pub fn begin_close(&mut self){self.closing=true;self.field.begin_close();if let Some(original)=self.original.as_mut(){original.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if !self.field.terminal_is_empty(){return Self::nested(retained_clone_close_demands::<T>(&self.field,body.saturating_sub(Self::fixed()))?);}
  if !self.retirement.is_empty()||self.pending.is_some(){return Self::nested(self.retirement.next_owner_demands::<T>(self.pending.is_some(),&None,body.saturating_sub(Self::fixed()))?);}
  if self.output.is_some(){return Self::nested(self.retirement.next_owner_demands::<Vec<T>>(true,&None,body.saturating_sub(Self::fixed()))?);}
  if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(Self::scalar())}else{Self::nested(original.next_demand(body.saturating_sub(Self::fixed()))?)};}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let child=Self::child(grant);
  if !self.field.terminal_is_empty(){return self.field.close_step(child).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(child).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if let Some(step)=self.retirement.begin_granted(&mut self.pending,child)?{return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if let Some(step)=self.retirement.begin_granted(&mut self.output,child)?{return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  if let Some(original)=self.original.as_mut(){if original.terminal_is_empty(){self.original.take();return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed(),..Default::default()}));}return original.step(child).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.field.terminal_is_empty()&&self.retirement.is_empty()&&self.pending.is_none()&&self.output.is_none()&&self.original.is_none()}
}
impl<T:RetainedClone> Drop for IndexedCollectionCommit<T>{fn drop(&mut self){let empty=self.original.is_none()&&self.output.is_none()&&self.pending.is_none()&&self.retirement.is_empty();assert!(std::thread::panicking()||empty,"indexed collection commit requires original child-first closure");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.pending);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
