//! 📋️ Ordered vector originals retain their existing backing through paid return and cancellation turns.
use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{ManuallyDrop,size_of};
fn refuse(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,reason)}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}

/// 🧳️ Keeps the original vector allocation and every unreturned native payload in the same cursor.
pub struct OriginalVectorCursor<T:RetireOwned>{original:ManuallyDrop<Option<std::vec::IntoIter<T>>>,pending:ManuallyDrop<Option<T>>,active:ManuallyDrop<Option<ControlledRetirement<T>>>,backing_bytes:usize,closing:bool}
impl<T:RetireOwned> OriginalVectorCursor<T>{
 /// 📏️ Quotes only the metadata transfer into the original iterator, with no backing birth.
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Vec<T>>()+size_of::<std::vec::IntoIter<T>>(),depth:1,..Default::default()}}
 /// 🎟️ Retains the same vector and backing pointer on every admission refusal.
 pub fn admit(original:Vec<T>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,Vec<T>)>{
  if !T::controlled_retirement_supported(){return Err((refuse("original vector payload requires native controlled retirement"),original));}
  let d=Self::constructor_demand();if !permits(d,grant){return Err((refuse("original vector metadata transfer exceeds caller grant"),original));}
  let Some(backing_bytes)=original.capacity().checked_mul(size_of::<T>())else{return Err((refuse("original vector backing layout overflow"),original));};
  Ok((Self{original:ManuallyDrop::new(Some(original.into_iter())),pending:ManuallyDrop::new(None),active:ManuallyDrop::new(None),backing_bytes,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 /// 👁️ Borrows the unchanged remaining original payloads without constructing a projection.
 pub fn remaining(&self)->&[T]{self.original.as_ref().map_or(&[],std::vec::IntoIter::as_slice)}
 /// 📏️ Quotes the native payload transfer and iterator-position update before returning an item.
 pub fn next_take_demand(&self)->RetirementDemand{if self.closing||self.active.is_some()||self.remaining().is_empty(){return Default::default();}RetirementDemand{copy_bytes:size_of::<T>()+size_of::<usize>(),depth:1,..Default::default()}}
 /// 🤝️ Returns the exact next native owner only under the unchanged independent caller grant.
 pub fn take_front(&mut self,grant:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{if self.closing||self.active.is_some()||self.remaining().is_empty(){return Ok(None);}let d=self.next_take_demand();if !permits(d,grant){return Ok(None);}Ok(self.original.as_mut().unwrap().next().map(|value|(value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 /// 🛑️ Switches to original payload closure while retaining the same vector allocation.
 pub fn begin_close(&mut self){self.closing=true;}
 /// 📏️ Quotes the actual next child, native payload handoff, or final backing release.
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
  if let Some(active)=self.active.as_ref(){if active.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()});}let body=active.next_copy_byte_demand()?;return Ok(RetirementDemand{copy_bytes:body,capacity_bytes:active.next_capacity_byte_demand(body)?,release_bytes:active.next_release_byte_demand()?,depth:active.next_depth_demand()?.checked_add(1).ok_or_else(||refuse("original vector child depth overflow"))?});}
  if self.pending.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<T>()+size_of::<ControlledRetirement<T>>(),depth:1,..Default::default()});}
  let Some(original)=self.original.as_ref()else{return Ok(Default::default());};
  if original.len()>0{return Ok(RetirementDemand{copy_bytes:size_of::<T>()+size_of::<usize>(),depth:1,..Default::default()});}
  Ok(RetirementDemand{copy_bytes:size_of::<Option<std::vec::IntoIter<T>>>(),release_bytes:self.backing_bytes,depth:1,..Default::default()})
 }
 /// 🧹️ Performs one preadmitted child or separately funded final backing release.
 pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let d=self.next_demand()?;if !permits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let p=if let Some(active)=self.active.as_mut(){if active.terminal_is_empty(){*self.active=None;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}else{active.step(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant})?.progress()}}
  else if let Some(original)=self.pending.take(){match ControlledRetirement::new(original){Ok(active)=>*self.active=Some(active),Err((error,original))=>{*self.pending=Some(original);return Err(error);}}RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
  else if !self.remaining().is_empty(){*self.pending=self.original.as_mut().unwrap().next();RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
  else{self.original.take();self.backing_bytes=0;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}};
  if !p.fits(grant){return Err(refuse("original vector child receipt exceeds caller grant"));}
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 /// ✅️ Requires both payload and physical original backing custody to be empty.
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.pending.is_none()&&self.active.is_none()&&self.backing_bytes==0}
}
impl<T:RetireOwned> Drop for OriginalVectorCursor<T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original vector abandoned its native backing custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.active);}}}}
impl<T:RetireOwned> RetireOwned for OriginalVectorCursor<T>{fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}}
impl<T:RetireOwned> RetirementCursor for OriginalVectorCursor<T>{
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{self.begin_close();match self.step(g){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p))=>RetirementStep::Progress(p)}}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand()?.copy_bytes)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{self.next_demand().ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand().ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand()?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
