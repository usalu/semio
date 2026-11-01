//! 🪢️ Original vectors stream into one funded output directory without cloning payloads.
use crate::{RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,list::{PagedList,PagedListError},retirement::{RetireOwned,original_vector::OriginalVectorCursor},retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original vector concatenation lost its declared ownership frontier")}
fn mapped<T>(result:Result<T,PagedListError>)->Result<T,ValueError>{result.map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))}
/// 🧳️ Retains every original allocation across separately paid publication and close turns.
pub struct OriginalVectorConcat<T:RetireOwned>{
 chunks:ManuallyDrop<PagedList<OriginalVectorCursor<T>,{usize::MAX}>>,
 pending:ManuallyDrop<Option<OriginalVectorCursor<T>>>,returned:ManuallyDrop<Option<T>>,
 output:ManuallyDrop<Option<Vec<T>>>,retirement:RetainedCloneClose,total:usize,index:usize,phase:u8,closing:bool,
}
impl<T:RetireOwned> Default for OriginalVectorConcat<T>{fn default()->Self{Self::new()}}
impl<T:RetireOwned> OriginalVectorConcat<T>{
 pub fn new()->Self{Self{chunks:ManuallyDrop::new(PagedList::new()),pending:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),total:0,index:0,phase:0,closing:false}}
 fn scalar()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant}}
 fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
 fn moved()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
 /// 📏️ Quotes the actual page reservation, input handoff or new retained cursor row.
 pub fn push_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=0{return Err(invalid())}
  if self.pending.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<OriginalVectorCursor<T>>(),depth:mapped(self.chunks.next_push_depth_demand())?.max(1),..Default::default()})}
  if !self.chunks.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:mapped(self.chunks.next_reserve_copy_byte_demand())?,capacity_bytes:mapped(self.chunks.next_allocation_bytes())?,depth:mapped(self.chunks.next_reserve_depth_demand())?.max(1),..Default::default()})}
  Self::nested(OriginalVectorCursor::<T>::constructor_demand())
 }
 /// 🤝️ Keeps the same input on refusal and stages its admitted owner before publication.
 pub fn push(&mut self,input:&mut Vec<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<(bool,RetainedCloneProgress)>{
  let mut work=0;let result=(||{
   let d=self.push_demand()?;if !retained_clone_grant_admits(grant,d){return Ok((false,Default::default()))}
   if let Some(original)=self.pending.take(){self.chunks.push_reserved(original).unwrap_or_else(|_|unreachable!("paid chunk row remains exclusively reserved"));work=1;return Ok((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
   if !self.chunks.has_reserved_slot(){return match self.chunks.reserve_one(grant.maximum_capacity_bytes){Ok(p)=>{work=p.work;Ok((false,RetainedCloneProgress{copied_items:usize::from(p.progressed),copied_bytes:p.placed_bytes,retained_capacity_bytes:p.allocated_bytes,released_bytes:p.released_allocation_bytes}))},Err(e)=>{work=e.work;Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:e.allocated_bytes,..Default::default()}))}}}
   let total=self.total.checked_add(input.len()).ok_or_else(invalid)?;
   let turn=OriginalVectorCursor::admit(std::mem::take(input),Self::child(grant));work=turn.work;
   match turn.result{Ok((original,p))=>{*self.pending=Some(original);self.total=total;Ok((false,p))},Err((e,original))=>{*input=original;Err(e)}}
  })();RetainedWorkOutcome{result,work}
 }
 /// 🌱️ Quotes one actual output backing after all original cardinalities are retained.
 pub fn finish_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=0||self.pending.is_some(){return Err(invalid())}Ok(RetirementDemand{capacity_bytes:self.total.checked_mul(size_of::<T>()).ok_or_else(invalid)?,..Self::scalar()})}
 pub fn finish(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;let result=(||{let d=self.finish_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let mut output=Vec::new();work=1;output.try_reserve_exact(self.total).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original concatenated output reservation refused"))?;let bytes=output.capacity().checked_mul(size_of::<T>()).ok_or_else(invalid)?;*self.output=Some(output);self.phase=4;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{retained_capacity_bytes:bytes,..Self::moved()}))})();RetainedWorkOutcome{result,work}
 }
 /// 🎒️ Quotes original row return separately from its new output-directory slot.
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||!matches!(self.phase,1|4){return Err(invalid())}if self.phase==4{return Ok(Self::scalar())}
  if self.returned.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<T>(),..Self::scalar()})}
  let Some(chunk)=self.chunks.get(self.index)else{return Ok(Self::scalar())};
  let mut d=if !chunk.remaining().is_empty(){Self::nested(chunk.next_take_demand())?}else if chunk.terminal_is_empty()||!chunk.close_started(){Self::scalar()}else{Self::nested(chunk.next_demand(body)?)?};d.depth=d.depth.max(mapped(self.chunks.next_get_depth_demand(self.index))?);Ok(d)
 }
 pub fn advance(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;let result=(||{
   let d=self.demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.phase==4{self.phase=1;work=1;return Ok(RetainedCloneStep::Progress(Self::moved()))}
   if self.returned.is_some(){let output=self.output.as_mut().ok_or_else(invalid)?;if output.len()==output.capacity(){return Err(invalid())}output.push(self.returned.take().ok_or_else(||ValueError::invariant("returned absent"))?);work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:size_of::<T>(),..Self::moved()}))}
   let Some(chunk)=self.chunks.get_mut(self.index)else{self.phase=2;work=1;return Ok(RetainedCloneStep::Complete(Self::moved()))};
   if !chunk.remaining().is_empty(){let turn=chunk.take_front(Self::child(grant));work=turn.work;let(value,p)=turn.result?.ok_or_else(invalid)?;*self.returned=Some(value);return Ok(RetainedCloneStep::Progress(p))}
   if !chunk.terminal_is_empty(){if !chunk.close_started(){chunk.begin_close();work=1;return Ok(RetainedCloneStep::Progress(Self::moved()))}let turn=chunk.step(Self::child(grant));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}
   self.index+=1;work=1;Ok(RetainedCloneStep::Progress(Self::moved()))
  })();RetainedWorkOutcome{result,work}
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.output.is_some()}
 /// 📤️ Quotes the fixed output header handoff with no payload copy.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(Self::scalar())}
 pub fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Vec<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None)}let output=self.output.take().ok_or_else(invalid)?;self.phase=3;work=1;Ok(Some((output,Self::moved())))})();RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Quotes the live pending owner before scaffolds and unpublished output.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}
  if !self.retirement.is_empty(){return Self::nested(self.retirement.demands(body)?)}
  if let Some(chunk)=self.pending.as_ref(){return if chunk.terminal_is_empty()||!chunk.close_started(){Ok(Self::scalar())}else{Self::nested(chunk.next_demand(body)?)}}
  if self.returned.is_some(){return Self::nested(self.retirement.next_owner_demands::<T>(true,&None,body)?)}
  if let Some(chunk)=self.chunks.last(){let mut d=if chunk.terminal_is_empty(){RetirementDemand{depth:mapped(self.chunks.next_pop_depth_demand())?.max(1),..Default::default()}}else if !chunk.close_started(){Self::scalar()}else{Self::nested(chunk.next_demand(body)?)?};d.depth=d.depth.max(mapped(self.chunks.next_get_depth_demand(self.chunks.len()-1))?);return Ok(d)}
  if self.chunks.allocated_bytes()!=0{return Ok(RetirementDemand{release_bytes:mapped(self.chunks.next_release_allocation_bytes())?,depth:mapped(self.chunks.next_release_depth_demand())?.max(1),..Default::default()})}
  if self.output.is_some(){return Self::nested(self.retirement.next_owner_demands::<Vec<T>>(true,&None,body)?)}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let mut work=0;let result=(||{
   if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if !self.retirement.is_empty(){let turn=self.retirement.step_granted(Self::child(grant));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}
   if let Some(chunk)=self.pending.as_mut(){if chunk.terminal_is_empty(){*self.pending=None;work=1;return Ok(RetainedCloneStep::Progress(Self::moved()))}if !chunk.close_started(){chunk.begin_close();work=1;return Ok(RetainedCloneStep::Progress(Self::moved()))}let turn=chunk.step(Self::child(grant));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}
   if self.returned.is_some(){let turn=self.retirement.begin_granted(&mut self.returned,Self::child(grant));work=turn.work;return turn.result?.ok_or_else(invalid).map(|s|RetainedCloneStep::Progress(s.progress()))}
   if let Some(chunk)=self.chunks.last_mut(){if chunk.terminal_is_empty(){self.chunks.pop();work=1;return Ok(RetainedCloneStep::Progress(Self::moved()))}if !chunk.close_started(){chunk.begin_close();work=1;return Ok(RetainedCloneStep::Progress(Self::moved()))}let turn=chunk.step(Self::child(grant));work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}
   if self.chunks.allocated_bytes()!=0{let p=mapped(self.chunks.release_empty_page(grant.maximum_release_bytes))?;work=p.work;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(p.progressed),copied_bytes:p.placed_bytes,retained_capacity_bytes:p.allocated_bytes,released_bytes:p.released_allocation_bytes}))}
   let turn=self.retirement.begin_granted(&mut self.output,Self::child(grant));work=turn.work;turn.result?.ok_or_else(invalid).map(|s|RetainedCloneStep::Progress(s.progress()))
  })();RetainedWorkOutcome{result,work}
 }
 fn empty(&self)->bool{self.chunks.terminal_is_empty()&&self.pending.is_none()&&self.returned.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.empty()}
}
impl<T:RetireOwned> Drop for OriginalVectorConcat<T>{fn drop(&mut self){let empty=self.empty();assert!(std::thread::panicking()||empty,"original vector concatenation requires paid child and backing closure");if empty{unsafe{ManuallyDrop::drop(&mut self.chunks);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.returned);ManuallyDrop::drop(&mut self.output);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
