//! 🪢️ Original vectors stream into one funded output directory without cloning payloads.
use semio_framework_value_original_vector::OriginalVectorCursor;
use crate::{RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,list::{PagedList,PagedListError,PagedListReservationOutcome,PagedListReservationFailure,PagedListReleaseOutcome,PagedListReleaseFailure},retirement::RetireOwned,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_control_contract::{OriginalEffectReply as Reply,OriginalEffectReplyComplete as Complete,RetainedAllocationEffect as Effect,retained_clone_grant_admits};
use semio_framework_value_retirement::{OriginalRetirementFailure as Failure,OriginalRetirementCloseLoan as Loan,OriginalRetirementPoll,retirement::controlled::ControlledRetirement,ErasedSnapshotRetirement};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::invariant("original vector concatenation lost its declared ownership frontier")}
fn mapped<T>(result:Result<T,PagedListError>)->Result<T,ValueError>{result.map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))}
fn reservation_error(failure:PagedListReservationFailure)->ValueError{match failure{PagedListReservationFailure::Work(error)=>error,PagedListReservationFailure::Allocation{error,..}=>ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()})}}
fn release_error(failure:PagedListReleaseFailure)->ValueError{match failure{PagedListReleaseFailure::Work(error)=>error,PagedListReleaseFailure::Release(error)=>ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason)}}
/// 🧳️ Retains every original allocation across separately paid publication and close turns.
pub struct OriginalVectorConcat<T:RetireOwned>{
 chunks:ManuallyDrop<PagedList<OriginalVectorCursor<T>,{usize::MAX}>>,
 pending:ManuallyDrop<Option<OriginalVectorCursor<T>>>,returned:ManuallyDrop<Option<T>>,
 output:ManuallyDrop<Option<Vec<T>>>,
 row_close:ManuallyDrop<Option<ControlledRetirement<T>>>,output_close:ManuallyDrop<Option<ControlledRetirement<Vec<T>>>>,
 total:usize,index:usize,phase:u8,closing:bool,
}
impl<T:RetireOwned> Default for OriginalVectorConcat<T>{fn default()->Self{Self::new()}}
impl<T:RetireOwned> OriginalVectorConcat<T>{
 pub fn new()->Self{Self{chunks:ManuallyDrop::new(PagedList::new()),pending:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),output:ManuallyDrop::new(None),row_close:ManuallyDrop::new(None),output_close:ManuallyDrop::new(None),total:0,index:0,phase:0,closing:false}}
 fn header(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child<'a>(effect:&'a mut Effect<'_>)->Result<Effect<'a>,ValueError>{let grant=effect.grant();let maximum=effect.remaining_work_units()?;effect.reborrow(RetainedCloneGrant{maximum_depth:grant.maximum_depth.checked_sub(1).ok_or_else(invalid)?,..grant},maximum)}
 fn funded(effect:&mut Effect<'_>,demand:RetirementDemand,work:u64)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),demand)&&effect.remaining_work_units()?>=work)}
 fn pay(effect:&mut Effect<'_>,work:u64)->Result<(),ValueError>{if effect.performed(work)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original concatenation exceeded its Work"))}}
 fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
 /// 📏️ Quotes the actual page reservation, original admission or retained cursor publication.
 pub fn push_demand(&self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=0{return Err(invalid())}
  if self.pending.is_some(){let mut d=mapped(self.chunks.push_reserved_source_demand())?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<usize>()).ok_or_else(invalid)?;return Ok(d)}
  if !self.chunks.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:mapped(self.chunks.next_reserve_copy_byte_demand())?,capacity_bytes:mapped(self.chunks.next_allocation_bytes())?,depth:mapped(self.chunks.next_reserve_depth_demand())?.max(1),..Default::default()})}
  Self::nested(OriginalVectorCursor::<T>::constructor_demand())
 }
 pub fn push_work_demand(&self)->Result<u64,ValueError>{self.push_demand()?;if self.pending.is_some(){mapped(self.chunks.push_reserved_source_work_demand())}else if !self.chunks.has_reserved_slot(){mapped(self.chunks.reserve_one_work_demand())}else{Ok(1)}}
 /// 🤝️ Moves the input only into resident custody and completes after its directory publication.
 pub fn push_into<'r>(&mut self,input:&mut Option<Vec<T>>,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Complete<'r,RetainedCloneStep,ValueError>,ValueError>{
  let d=self.push_demand()?;let work=self.push_work_demand()?;
  if !Self::funded(effect,d,work)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if let Some(cursor)=self.pending.as_ref(){let total=self.total.checked_add(cursor.remaining().len()).ok_or_else(invalid)?;Self::pay(effect,work)?;let turn=self.chunks.push_reserved_source_step(&mut self.pending,work,effect.grant());if turn.result.as_ref().is_ok_and(|step|matches!(step,RetainedCloneStep::Complete(_))){self.total=total}return Ok(reply.complete(turn.result.map(|step|match step{RetainedCloneStep::Complete(mut p)=>{p.copied_bytes=d.copy_bytes;RetainedCloneStep::Complete(p)},other=>other})))}
  if !self.chunks.has_reserved_slot(){let turn=self.chunks.reserve_one_funded(effect);let result=turn.result.map(|outcome|match outcome{semio_framework_value_paged_list::PagedListReservationOutcome::Denied=>RetainedCloneStep::Progress(Default::default()),semio_framework_value_paged_list::PagedListReservationOutcome::Ready(p)=>RetainedCloneStep::Progress(p)}).map_err(|e|match e{semio_framework_value_paged_list::PagedListReservationFailure::Work(error)=>error,semio_framework_value_paged_list::PagedListReservationFailure::Allocation{error,copied_bytes}=>ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason).with_retained_progress(RetainedCloneProgress{copied_items:usize::from(copied_bytes!=0||error.allocated_bytes!=0),copied_bytes,retained_capacity_bytes:error.allocated_bytes,..Default::default()})});return Ok(reply.complete(result))}
  let len=input.as_ref().ok_or_else(invalid)?.len();self.total.checked_add(len).ok_or_else(invalid)?;
  OriginalVectorCursor::admit_into(input,&mut self.pending,&mut Self::child(effect)?,reply.progress())
 }
 /// 🌱️ Quotes the exact backing and native output header after all cardinalities are resident.
 pub fn finish_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.phase!=0||self.pending.is_some(){return Err(invalid())}let mut d=Self::header(size_of::<Option<Vec<T>>>()+size_of::<u8>());d.capacity_bytes=semio_framework_value_control_contract::allocation::backing::exact_vector_bytes::<T>(self.total)?;Ok(d)}
 pub fn finish_work_demand(&self)->Result<u64,ValueError>{self.finish_demand()?;Ok(1)}
 pub fn finish_into<'r>(&mut self,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Complete<'r,RetainedCloneStep,ValueError>,ValueError>{
  let d=self.finish_demand()?;if !Self::funded(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if effect.actual().is_some(){return Err(invalid())}Self::pay(effect,1)?;*effect.actual()=Some(0);
  let output=match semio_framework_value_control_contract::allocation::backing::allocate_exact_vector::<T>(self.total){Ok(output)=>output,Err(error)=>return Ok(reply.complete(Err(error)))};*effect.actual()=Some(d.capacity_bytes);*self.output=Some(output);self.phase=4;Ok(reply.complete(Ok(RetainedCloneStep::Complete(Self::progress(d)))))
 }
 /// 🎒️ Quotes native row return separately from its funded output slot and index update.
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||!matches!(self.phase,1|4){return Err(invalid())}if self.phase==4{return Ok(Self::header(size_of::<u8>()))}
  if self.returned.is_some(){return Ok(Self::header(size_of::<Option<T>>()+size_of::<T>()+size_of::<Vec<T>>()))}
  let Some(chunk)=self.chunks.get(self.index)else{return Ok(Self::header(size_of::<u8>()))};
  let mut d=if !chunk.remaining().is_empty(){Self::nested(chunk.next_take_demand())?}else if chunk.terminal_is_empty(){Self::header(size_of::<usize>())}else if !chunk.close_started(){Self::nested(chunk.begin_close_demand())?}else{Self::nested(chunk.next_demand(body)?)?};d.depth=d.depth.max(mapped(self.chunks.next_get_depth_demand(self.index))?);Ok(d)
 }
 pub fn advance_work_demand(&self,body:usize)->Result<u64,ValueError>{self.demand(body)?;if self.phase==4||self.returned.is_some(){return Ok(1)}let Some(chunk)=self.chunks.get(self.index)else{return Ok(1)};if chunk.remaining().is_empty()&&!chunk.terminal_is_empty()&&chunk.close_started(){chunk.next_computational_work(body)}else{Ok(1)}}
 pub fn advance_into<'r>(&mut self,body:usize,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,Failure>)->Result<Complete<'r,RetainedCloneStep,Failure>,ValueError>{
  let d=self.demand(body)?;let work=self.advance_work_demand(body)?;if !Self::funded(effect,d,work)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if self.phase==4{Self::pay(effect,1)?;self.phase=1;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Self::progress(d)))))}
  if self.returned.is_some(){let output=self.output.as_ref().ok_or_else(invalid)?;if size_of::<T>()!=0&&output.len()==output.capacity(){return Err(invalid())}Self::pay(effect,1)?;self.output.as_mut().expect("quoted original output").push(self.returned.take().expect("quoted original row"));return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Self::progress(d)))))}
  let Some(chunk)=self.chunks.get_mut(self.index)else{Self::pay(effect,1)?;self.phase=2;return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Self::progress(d)))))};
  if !chunk.remaining().is_empty(){let mut result=None;chunk.take_front_into(&mut self.returned,&mut Self::child(effect)?,Reply::borrow(&mut result)?)?;return Ok(reply.complete(result.ok_or_else(invalid)?.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)))}
  if !chunk.terminal_is_empty(){if !chunk.close_started(){let turn=chunk.begin_close_paid(&mut Self::child(effect)?);return Ok(reply.complete(turn.result.map(|p|RetainedCloneStep::Progress(p)).map_err(Into::into)))}return chunk.close_into(body,&mut Self::child(effect)?,reply.progress())}
  Self::pay(effect,1)?;self.index+=1;Ok(reply.complete(Ok(RetainedCloneStep::Progress(Self::progress(d)))))
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.output.is_some()}
 /// 📤️ Quotes both fixed output headers and the completed phase marker.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(Self::header(2*size_of::<Option<Vec<T>>>()+size_of::<u8>()))}
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 pub fn take_into<'r>(&mut self,output:&mut Option<Vec<T>>,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Complete<'r,RetainedCloneStep,ValueError>,ValueError>{
  if output.is_some(){return Err(invalid())}let d=self.take_demand()?;if !Self::funded(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}Self::pay(effect,1)?;*output=self.output.take();self.phase=3;Ok(reply.complete(Ok(RetainedCloneStep::Complete(Self::progress(d)))))
 }
 pub fn begin_close_demand(&self)->RetirementDemand{semio_framework_value_control_contract::retained_close_start_demand(self.closing)}
 pub fn begin_close_into<'r>(&mut self,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Complete<'r,RetainedCloneStep,ValueError>,ValueError>{let turn=semio_framework_value_control_contract::start_retained_close(&mut self.closing,effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Complete)))}
 fn cursor_demand(chunk:&OriginalVectorCursor<T>,body:usize)->Result<RetirementDemand,ValueError>{if chunk.terminal_is_empty(){Ok(Self::header(size_of::<Option<OriginalVectorCursor<T>>>()))}else if !chunk.close_started(){Self::nested(chunk.begin_close_demand())}else{Self::nested(chunk.next_demand(body)?)}}
 /// ♻️ Quotes the same resident child before original rows, directory scaffolds and output.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}
  if let Some(owner)=self.row_close.as_ref(){return if owner.terminal_is_empty(){Ok(Self::header(size_of::<Option<ControlledRetirement<T>>>()))}else{Self::nested(owner.next_demand(body)?)}}
  if let Some(chunk)=self.pending.as_ref(){return Self::cursor_demand(chunk,body)}
  if self.returned.is_some(){return Ok(Self::header(size_of::<Option<T>>()+size_of::<Option<ControlledRetirement<T>>>()))}
  if let Some(chunk)=self.chunks.last(){let mut d=if chunk.terminal_is_empty(){mapped(self.chunks.pop_source_demand())?}else{Self::cursor_demand(chunk,body)?};d.depth=d.depth.max(mapped(self.chunks.next_get_depth_demand(self.chunks.len()-1))?);return Ok(d)}
  if self.chunks.allocated_bytes()!=0{return Ok(RetirementDemand{copy_bytes:mapped(self.chunks.next_release_copy_byte_demand())?,release_bytes:mapped(self.chunks.next_release_allocation_bytes())?,depth:mapped(self.chunks.next_release_depth_demand())?.max(1),..Default::default()})}
  if let Some(owner)=self.output_close.as_ref(){return if owner.terminal_is_empty(){Ok(Self::header(size_of::<Option<ControlledRetirement<Vec<T>>>>()))}else{Self::nested(owner.next_demand(body)?)}}
  if self.output.is_some(){return Ok(Self::header(size_of::<Option<Vec<T>>>()+size_of::<Option<ControlledRetirement<Vec<T>>>>()))}
  Ok(Default::default())
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demand(body)?;if self.terminal_is_empty(){return Ok(0)}
  if let Some(owner)=self.row_close.as_ref().filter(|owner|!owner.terminal_is_empty()){return ErasedSnapshotRetirement::next_computational_work(owner,body)}
  let selected=self.pending.as_ref().or_else(||self.chunks.last());if let Some(chunk)=selected{if !chunk.terminal_is_empty()&&chunk.close_started(){return chunk.next_computational_work(body)}if self.pending.is_none()&&chunk.terminal_is_empty(){return mapped(self.chunks.pop_source_work_demand())}}
  if let Some(owner)=self.output_close.as_ref().filter(|owner|!owner.terminal_is_empty()){return ErasedSnapshotRetirement::next_computational_work(owner,body)}Ok(1)
 }
 pub fn requires_original_birth(&self)->bool{if let Some(owner)=self.row_close.as_ref(){return owner.requires_original_birth()}if let Some(chunk)=self.pending.as_ref().or_else(||self.chunks.last()){return chunk.requires_original_birth()}self.output_close.as_ref().is_some_and(|owner|owner.requires_original_birth())}
 pub fn close_original_birth_step(&mut self,original:&mut Loan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};if grant.maximum_items==0||grant.maximum_depth==0{return semio_framework_value_control_contract::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}if let Some(owner)=self.row_close.as_mut(){return owner.step_original_birth(original,child)}if let Some(chunk)=self.pending.as_mut(){return chunk.close_original_birth_step(original,child)}if let Some(chunk)=self.chunks.last_mut(){return chunk.close_original_birth_step(original,child)}if let Some(owner)=self.output_close.as_mut(){return owner.step_original_birth(original,child)}semio_framework_value_control_contract::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(invalid().into()),work:0})
 }
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,Failure>)->Result<Complete<'r,RetainedCloneStep,Failure>,ValueError>{self.close_borrowed(body,None,effect,reply,false)}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,Failure>)->Result<Complete<'r,RetainedCloneStep,Failure>,ValueError>{self.close_borrowed(body,None,effect,reply,true)}
 pub fn close_original_into<'r>(&mut self,body:usize,original:&mut Loan<'_>,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,Failure>)->Result<Complete<'r,RetainedCloneStep,Failure>,ValueError>{self.close_borrowed(body,Some(original),effect,reply,false)}
 pub fn recover_close_original_into<'r>(&mut self,body:usize,original:&mut Loan<'_>,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,Failure>)->Result<Complete<'r,RetainedCloneStep,Failure>,ValueError>{self.close_borrowed(body,Some(original),effect,reply,true)}
 fn close_borrowed<'r>(&mut self,body:usize,original:Option<&mut Loan<'_>>,effect:&mut Effect<'_>,reply:Reply<'r,RetainedCloneStep,Failure>,recover:bool)->Result<Complete<'r,RetainedCloneStep,Failure>,ValueError>{
  if recover{
   let mut child=Self::child(effect)?;
   if let Some(owner)=self.row_close.as_mut(){return match original{Some(loan)=>owner.recover_step_original_into(loan,body,&mut child,reply.progress()),None=>owner.recover_step_into(body,&mut child,reply.progress())}}
   if let Some(chunk)=self.pending.as_mut(){return match original{Some(loan)=>chunk.recover_close_original_into(loan,body,&mut child,reply.progress()),None=>chunk.recover_close_into(body,&mut child,reply.progress())}}
   if let Some(chunk)=self.chunks.last_mut(){return match original{Some(loan)=>chunk.recover_close_original_into(loan,body,&mut child,reply.progress()),None=>chunk.recover_close_into(body,&mut child,reply.progress())}}
   if let Some(owner)=self.output_close.as_mut(){return match original{Some(loan)=>owner.recover_step_original_into(loan,body,&mut child,reply.progress()),None=>owner.recover_step_into(body,&mut child,reply.progress())}}
   return Err(invalid())
  }
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if self.requires_original_birth(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original concatenation requires its selected child birth"))}
  let d=self.close_demand(body)?;let work=self.close_work_demand(body)?;if !Self::funded(effect,d,work)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if let Some(owner)=self.row_close.as_mut().filter(|owner|!owner.terminal_is_empty()){let mut child=Self::child(effect)?;return match(original,recover){(Some(loan),false)=>owner.step_original_into(loan,body,&mut child,reply.progress()),(Some(loan),true)=>owner.recover_step_original_into(loan,body,&mut child,reply.progress()),(None,false)=>owner.step_into(body,&mut child,reply.progress()),(None,true)=>owner.recover_step_into(body,&mut child,reply.progress())}}
  if let Some(chunk)=self.pending.as_mut().filter(|chunk|!chunk.terminal_is_empty()){
   let mut child=Self::child(effect)?;if !chunk.close_started(){if recover{return Err(invalid())}let turn=chunk.begin_close_paid(&mut child);return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}return match(original,recover){(Some(loan),false)=>chunk.close_original_into(loan,body,&mut child,reply.progress()),(Some(loan),true)=>chunk.recover_close_original_into(loan,body,&mut child,reply.progress()),(None,false)=>chunk.close_into(body,&mut child,reply.progress()),(None,true)=>chunk.recover_close_into(body,&mut child,reply.progress())}
  }
  if self.row_close.is_none()&&self.pending.is_none()&&self.returned.is_none(){if let Some(chunk)=self.chunks.last_mut().filter(|chunk|!chunk.terminal_is_empty()){let mut child=Self::child(effect)?;if !chunk.close_started(){if recover{return Err(invalid())}let turn=chunk.begin_close_paid(&mut child);return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}return match(original,recover){(Some(loan),false)=>chunk.close_original_into(loan,body,&mut child,reply.progress()),(Some(loan),true)=>chunk.recover_close_original_into(loan,body,&mut child,reply.progress()),(None,false)=>chunk.close_into(body,&mut child,reply.progress()),(None,true)=>chunk.recover_close_into(body,&mut child,reply.progress())}}}
  if self.row_close.is_none()&&self.pending.is_none()&&self.returned.is_none()&&self.chunks.is_empty()&&self.chunks.allocated_bytes()==0{if let Some(owner)=self.output_close.as_mut().filter(|owner|!owner.terminal_is_empty()){let mut child=Self::child(effect)?;return match(original,recover){(Some(loan),false)=>owner.step_original_into(loan,body,&mut child,reply.progress()),(Some(loan),true)=>owner.recover_step_original_into(loan,body,&mut child,reply.progress()),(None,false)=>owner.step_into(body,&mut child,reply.progress()),(None,true)=>owner.recover_step_into(body,&mut child,reply.progress())}}}
  if recover{return Err(invalid())}
  if self.row_close.is_none()&&self.pending.is_none()&&self.returned.is_none()&&self.chunks.is_empty()&&self.chunks.allocated_bytes()!=0{let turn=self.chunks.release_empty_page(effect);return Ok(reply.complete(turn.result.map(|outcome|match outcome{semio_framework_value_paged_list::PagedListReleaseOutcome::Denied=>RetainedCloneStep::Progress(Default::default()),semio_framework_value_paged_list::PagedListReleaseOutcome::Ready(p)=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(p.progressed),copied_bytes:p.placed_bytes,released_bytes:p.released_allocation_bytes,..Default::default()})}).map_err(|e|match e{semio_framework_value_paged_list::PagedListReleaseFailure::Work(error)=>error,semio_framework_value_paged_list::PagedListReleaseFailure::Release(error)=>ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason)}.into())))}
  if self.row_close.is_none()&&self.pending.is_none()&&self.returned.is_none()&&!self.chunks.is_empty(){return self.chunks.pop_retirement_into(&mut self.pending,effect,reply.progress())}
  Self::pay(effect,work)?;
  if self.row_close.is_some(){*self.row_close=None}else if self.pending.is_some(){*self.pending=None}else if self.returned.is_some(){*self.row_close=Some(ControlledRetirement::retain(self.returned.take().expect("quoted original row")))}else if self.output_close.is_some(){*self.output_close=None}else if self.output.is_some(){*self.output_close=Some(ControlledRetirement::retain(self.output.take().expect("quoted original output")))}else{return Err(invalid())}
  Ok(reply.complete(Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(Self::progress(d))}else{RetainedCloneStep::Progress(Self::progress(d))})))
 }
 fn empty(&self)->bool{self.chunks.terminal_is_empty()&&self.pending.is_none()&&self.returned.is_none()&&self.output.is_none()&&self.row_close.is_none()&&self.output_close.is_none()}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.empty()}
}
impl<T:RetireOwned> Drop for OriginalVectorConcat<T>{fn drop(&mut self){let empty=self.empty();assert!(std::thread::panicking()||empty,"original vector concatenation requires paid child and backing closure");if empty{unsafe{ManuallyDrop::drop(&mut self.chunks);ManuallyDrop::drop(&mut self.pending);ManuallyDrop::drop(&mut self.returned);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.row_close);ManuallyDrop::drop(&mut self.output_close);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
