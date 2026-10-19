//! 🪢️ Original vectors stream into one funded output directory without cloning payloads.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,list::{PagedList,PagedListError},retirement::{RetireOwned,original_vector::OriginalVectorCursor},retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original vector concatenation lost its declared ownership frontier")}
fn mapped<T>(result:Result<T,PagedListError>)->Result<T,ValueError>{result.map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))}
/// 🧳️ Each input keeps its actual iterator backing until returned rows and child closure settle.
pub struct OriginalVectorConcat<T:RetireOwned>{
 chunks:ManuallyDrop<PagedList<OriginalVectorCursor<T>,{usize::MAX}>>,
 output:ManuallyDrop<Option<Vec<T>>>,retirement:RetainedCloneClose,total:usize,index:usize,phase:u8,closing:bool,
}
impl<T:RetireOwned> Default for OriginalVectorConcat<T>{fn default()->Self{Self::new()}}
impl<T:RetireOwned> OriginalVectorConcat<T>{
 pub fn new()->Self{Self{chunks:ManuallyDrop::new(PagedList::new()),output:ManuallyDrop::new(None),retirement:Default::default(),total:0,index:0,phase:0,closing:false}}
 fn fixed()->usize{size_of::<usize>()*8+size_of::<u8>()*2+size_of::<bool>()*2+size_of::<Vec<T>>()*2}
 fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:Self::fixed(),depth:1,..Default::default()}}
 fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed(),maximum_depth:grant.maximum_depth-1,..grant}}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(Self::fixed()).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn receipt(mut progress:RetainedCloneProgress)->RetainedCloneProgress{progress.copied_bytes+=Self::fixed();progress}
 /// 📏️ A chunk reserves metadata pages before adopting any original vector.
 pub fn push_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=0{return Err(invalid());}
  if !self.chunks.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:Self::fixed()+mapped(self.chunks.next_reserve_copy_byte_demand())?,capacity_bytes:mapped(self.chunks.next_allocation_bytes())?,depth:mapped(self.chunks.next_reserve_depth_demand())?.max(2),..Default::default()});}
  let mut demand=Self::nested(OriginalVectorCursor::<T>::constructor_demand())?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<OriginalVectorCursor<T>>()).ok_or_else(invalid)?;demand.depth=demand.depth.max(mapped(self.chunks.next_push_depth_demand())?);Ok(demand)
 }
 /// 🤝️ An unfunded turn leaves the caller's exact vector and payloads untouched.
 pub fn push(&mut self,input:&mut Vec<T>,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
  let demand=self.push_demand()?;if !retained_clone_grant_admits(grant,demand){return Ok((false,Default::default()));}
  if !self.chunks.has_reserved_slot(){let progress=self.chunks.reserve_one_funded(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed(),..grant}).map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;return Ok((false,Self::receipt(progress)));}
  let total=self.total.checked_add(input.len()).ok_or_else(invalid)?;
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed()-size_of::<OriginalVectorCursor<T>>(),maximum_depth:grant.maximum_depth-1,..grant};
  let(original,mut progress)=match OriginalVectorCursor::admit(std::mem::take(input),child){Ok(result)=>result,Err((error,original))=>{*input=original;return Err(error);}};
  self.chunks.push_reserved(original).unwrap_or_else(|_|unreachable!("paid original chunk slot remains exclusively owned"));self.total=total;progress.copied_bytes+=Self::fixed()+size_of::<OriginalVectorCursor<T>>();Ok((true,progress))
 }
 /// 🌱️ One output backing is born after all original chunk cardinalities are known.
 pub fn finish_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=0{return Err(invalid());}Ok(RetirementDemand{capacity_bytes:self.total.checked_mul(size_of::<T>()).ok_or_else(invalid)?,..Self::scalar()})}
 pub fn finish(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.finish_demand()?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut output=Vec::new();output.try_reserve_exact(self.total).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original concatenated output reservation refused"))?;
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:output.capacity().checked_mul(size_of::<T>()).ok_or_else(invalid)?,..Default::default()};*self.output=Some(output);self.phase=1;Ok(RetainedCloneStep::Progress(progress))
 }
 /// 📏️ Each turn prices the actual ranked chunk and one original payload header.
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=1{return Err(invalid());}
  let Some(chunk)=self.chunks.get(self.index)else{return Ok(Self::scalar());};
  let mut demand=if !chunk.remaining().is_empty(){let mut demand=chunk.next_take_demand();demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<T>()).ok_or_else(invalid)?;Self::nested(demand)?}else if !chunk.terminal_is_empty(){Self::nested(chunk.next_demand(body.saturating_sub(Self::fixed()))?)?}else{Self::scalar()};
  demand.depth=demand.depth.max(mapped(self.chunks.next_get_depth_demand(self.index))?);Ok(demand)
 }
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let Some(chunk)=self.chunks.get_mut(self.index)else{self.phase=2;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));};
  if !chunk.remaining().is_empty(){let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed()-size_of::<T>(),maximum_depth:grant.maximum_depth-1,..grant};let(value,mut progress)=chunk.take_front(child)?.ok_or_else(invalid)?;self.output.as_mut().ok_or_else(invalid)?.push(value);progress.copied_bytes+=Self::fixed()+size_of::<T>();return Ok(RetainedCloneStep::Progress(progress));}
  if !chunk.terminal_is_empty(){chunk.begin_close();return chunk.step(Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  self.index+=1;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.output.is_some()}
 /// 📤️ Transfers the original ordered output once without copying any payload allocation.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid());}Ok(Self::scalar())}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<T>,RetainedCloneProgress)>,ValueError>{let demand=self.take_demand()?;if !retained_clone_grant_admits(grant,demand){return Ok(None);}self.phase=3;Ok(Some((self.output.take().ok_or_else(invalid)?,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Original children close before page scaffolds and any unpublished output.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}
  if let Some(chunk)=self.chunks.last(){let mut demand=if chunk.terminal_is_empty(){RetirementDemand{copy_bytes:Self::fixed()+size_of::<OriginalVectorCursor<T>>(),depth:mapped(self.chunks.next_pop_depth_demand())?.max(1),..Default::default()}}else{Self::nested(chunk.next_demand(body.saturating_sub(Self::fixed()))?)?};demand.depth=demand.depth.max(mapped(self.chunks.next_get_depth_demand(self.chunks.len()-1))?);return Ok(demand);}
  if self.chunks.allocated_bytes()!=0{return Ok(RetirementDemand{copy_bytes:Self::fixed(),release_bytes:mapped(self.chunks.next_release_allocation_bytes())?,depth:mapped(self.chunks.next_release_depth_demand())?.max(1),..Default::default()});}
  if self.output.is_some()||!self.retirement.is_empty(){return Self::nested(self.retirement.next_owner_demands::<Vec<T>>(self.output.is_some(),&None,body.saturating_sub(Self::fixed()))?);}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(chunk)=self.chunks.last_mut(){if !chunk.terminal_is_empty(){chunk.begin_close();return chunk.step(Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.chunks.pop();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if self.chunks.allocated_bytes()!=0{let progress=mapped(self.chunks.release_empty_page(grant.maximum_release_bytes))?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:progress.released_allocation_bytes,..Default::default()}));}
  let step=if !self.retirement.is_empty(){self.retirement.step_granted(Self::child(grant))?}else{self.retirement.begin_granted(&mut self.output,Self::child(grant))?.ok_or_else(invalid)?};Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))
 }
 fn empty(&self)->bool{self.chunks.terminal_is_empty()&&self.output.is_none()&&self.retirement.is_empty()}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.empty()}
}
impl<T:RetireOwned> Drop for OriginalVectorConcat<T>{fn drop(&mut self){let empty=self.empty();assert!(std::thread::panicking()||empty,"original vector concatenation requires paid child and backing closure");if empty{unsafe{ManuallyDrop::drop(&mut self.chunks);ManuallyDrop::drop(&mut self.output);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
