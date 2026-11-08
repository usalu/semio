//! 📋️ Ordered vector originals retain their existing backing through paid return and cancellation turns.
use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{RetainedWorkOutcome,ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{ManuallyDrop,size_of};
fn refuse(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,reason)}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}

/// 🧳️ Keeps the original vector allocation and every unreturned native payload in the same cursor.
pub struct OriginalVectorCursor<T:RetireOwned>{original:ManuallyDrop<Option<std::vec::IntoIter<T>>>,pending:ManuallyDrop<Option<T>>,active:ManuallyDrop<Option<ControlledRetirement<T>>>,backing_bytes:usize,closing:bool}
impl<T:RetireOwned> OriginalVectorCursor<T>{
 /// 📏️ Quotes only the metadata transfer into the original iterator, with no backing birth.
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 /// 🎟️ Retains the same vector and backing pointer on every admission refusal.
 pub fn admit(original:Vec<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,Vec<T>)>{if !T::controlled_retirement_supported(){return RetainedWorkOutcome{result:Err((refuse("original vector payload requires native controlled retirement"),original)),work:0}}let d=Self::constructor_demand();if !permits(d,grant){return RetainedWorkOutcome{result:Err((refuse("original vector metadata transfer exceeds caller grant"),original)),work:0}}let Some(backing_bytes)=original.capacity().checked_mul(size_of::<T>())else{return RetainedWorkOutcome{result:Err((refuse("original vector backing layout overflow"),original)),work:0}};let cursor=Self{original:ManuallyDrop::new(Some(original.into_iter())),pending:ManuallyDrop::new(None),active:ManuallyDrop::new(None),backing_bytes,closing:false};RetainedWorkOutcome{result:Ok((cursor,RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
 /// 👁️ Borrows the unchanged remaining original payloads without constructing a projection.
 pub fn remaining(&self)->&[T]{self.original.as_ref().map_or(&[],std::vec::IntoIter::as_slice)}
 /// 🧲️ Quotes the native payload transfer and iterator-position update before returning an item.
 pub fn next_take_demand(&self)->RetirementDemand{if self.closing||self.active.is_some()||self.remaining().is_empty(){return Default::default();}RetirementDemand{depth:1,..Default::default()}}
 /// 🤝️ Returns the exact next native owner only under the unchanged independent caller grant.
 pub fn take_front(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>{if self.closing||self.active.is_some()||self.remaining().is_empty(){return RetainedWorkOutcome{result:Ok(None),work:0}}let d=self.next_take_demand();if !permits(d,grant){return RetainedWorkOutcome{result:Ok(None),work:0}}let Some(original)=self.original.as_mut()else{return RetainedWorkOutcome{result:Err(ValueError::invariant("original vector remains resident").into()),work:0}};let Some(value)=original.next()else{return RetainedWorkOutcome{result:Err(ValueError::invariant("original vector has its quoted next row").into()),work:0}};RetainedWorkOutcome{result:Ok(Some((value,RetainedCloneProgress{copied_items:1,..Default::default()}))),work:1}}
 /// 🛑️ Switches to original payload closure while retaining the same vector allocation.
 pub fn begin_close_demand(&self)->RetirementDemand{crate::retained_close_start_demand(self.closing)}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 pub fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{crate::start_retained_close(&mut self.closing,effect)}
 /// 👁️ Reports whether the original close election has already been retained.
 pub fn close_started(&self)->bool{self.closing}
 /// 🔜️ Quotes the actual next child, native payload handoff, or final backing release.
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(active)=self.active.as_ref(){if active.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}let mut d=active.next_demand(body)?;d.depth=d.depth.checked_add(1).ok_or_else(||refuse("original vector child depth overflow"))?;return Ok(d);}
  if self.pending.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  let Some(original)=self.original.as_ref()else{return Ok(Default::default());};
  if original.len()>0{return Ok(RetirementDemand{depth:1,..Default::default()});}
  Ok(RetirementDemand{release_bytes:self.backing_bytes,depth:1,..Default::default()})
 }
 /// ⛽️ Quotes the selected native row, retained child, or backing release.
 pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty()||!self.closing{return Ok(0)}if let Some(active)=self.active.as_ref(){if !active.terminal_is_empty(){return crate::ErasedSnapshotRetirement::next_computational_work(active,body)}}Ok(1)}
 /// 🧹️ Performs one preadmitted child or separately funded final backing release.
 pub fn step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{self.step_with(effect,|active,effect|active.step(effect))}
 /// 🪢️ Delegates one selected child to its actual original control without a parent settlement.
 pub fn step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll{let grant=effect.grant();if !self.requires_original_control(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"vector has no selected original child").into()),work:0})}self.step_with(effect,|active,effect|crate::ErasedSnapshotRetirement::close_step_controlled(active,effect,control))}
 /// 🧷️ Delegates the selected child through the original retirement loan.
 pub fn step_original(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();if !self.requires_original_control(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"vector has no selected original child").into()),work:0})}self.step_with(effect,|active,effect|crate::ErasedSnapshotRetirement::close_original_step(active,original,effect))}
 fn requires_original_control(&self)->bool{self.closing&&self.active.as_ref().is_some_and(|active|!active.terminal_is_empty()&&crate::ErasedSnapshotRetirement::requires_original_control(active))}
 fn step_with(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut ControlledRetirement<T>,&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll)->crate::OriginalRetirementPoll{let grant=effect.grant();
  let mut work=0;
  let result=(||->Result<RetainedCloneStep,crate::retirement_contract::RetirementPollExit>{
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.next_computational_work(grant.maximum_copy_bytes)?>effect.maximum_work_units(){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let d=self.next_demand(grant.maximum_copy_bytes)?;
   if !permits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let p=if let Some(active)=self.active.as_mut(){
    if active.terminal_is_empty(){*self.active=None;work=1;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
    else{let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let mut child=effect.nested(child);let turn=crate::retirement_contract::RetirementPollExit::settled(invoke(active,&mut child))?;work=turn.work;turn.result?.progress()}
   }else if let Some(original)=self.pending.take(){
    work=1;match ControlledRetirement::new(original){Ok(active)=>{*self.active=Some(active)},Err((error,original))=>{*self.pending=Some(original);return Err(error.into())}}
    RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}
   }else if !self.remaining().is_empty(){
    *self.pending=self.original.as_mut().ok_or_else(||ValueError::invariant("original absent"))?.next();work=1;
    RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}
   }else{
    self.original.take();self.backing_bytes=0;work=1;
    RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}
   };
   if !p.fits(grant){return Err(refuse("original vector child receipt exceeds caller grant").with_retained_progress(p).into())}
   Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();
  crate::retirement_contract::RetirementPollExit::finish(result,work)
 }
 /// ✅️ Requires both payload and physical original backing custody to be empty.
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.pending.is_none()&&self.active.is_none()&&self.backing_bytes==0}
}
impl<T:RetireOwned> Drop for OriginalVectorCursor<T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original vector abandoned its native backing custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.active);}}}}
impl<T:RetireOwned> RetireOwned for OriginalVectorCursor<T>{type RetirementCursor=OriginalVectorRetirementSeed<T>;fn retirement_seed(self)->Self::RetirementCursor{OriginalVectorRetirementSeed{owner:self}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalVectorRetirementSeed<T>>())}fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}}
impl<T:RetireOwned> RetirementCursor for OriginalVectorCursor<T>{
  fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{OriginalVectorCursor::next_computational_work(self,body)}
 fn requires_original_control(&self)->bool{OriginalVectorCursor::requires_original_control(self)}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(self.step_controlled(effect,control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(self.step_original(original,effect))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{OriginalVectorCursor::next_demand(self,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{super::erased_cursor_turn(self.step(effect))}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[doc(hidden)]
pub struct OriginalVectorRetirementSeed<T:RetireOwned>{owner:OriginalVectorCursor<T>}
impl<T:RetireOwned> RetirementCursor for OriginalVectorRetirementSeed<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if self.owner.closing{self.owner.next_computational_work(body)}else{Ok(1)}}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.owner.closing{self.owner.next_demand(body)}else{Ok(crate::retained_close_start_demand(false))}}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if !self.owner.closing{if effect.maximum_work_units()==0{return RetirementStep::BudgetExhausted}let turn=crate::start_retained_close(&mut self.owner.closing,effect);return match turn.result{Ok(progress)=>RetirementStep::Progress(progress,turn.work),Err(error)=>RetirementStep::Failure(error.into(),turn.work)}}RetirementCursor::close_step(&mut self.owner,effect)}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn requires_original_control(&self)->bool{self.owner.requires_original_control()}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{RetirementCursor::close_original_step(&mut self.owner,original,effect)}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{RetirementCursor::close_step_controlled(&mut self.owner,effect,control)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
