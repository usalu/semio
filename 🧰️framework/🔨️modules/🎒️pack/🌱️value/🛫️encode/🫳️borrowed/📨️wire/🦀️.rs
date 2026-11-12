//! 📨️ One original typed source is measured and encoded by the existing Pack codec before its exact ownership returns.
use super::{BorrowedProjectedPackCursor,BorrowedProjectedPackFailure};
use semio_framework_dsl_record::native_encoding::FieldProjectionSource;
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value_persistent::list::{PagedListReleaseFailure,PagedListReleaseOutcome,PagedListReservationFailure,PagedListReservationOutcome};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission};
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value_persistent::list::PagedList;
type WirePages=PagedList<u8,{isize::MAX as usize}>;
type WireBytes=semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>;
fn page_error(error:semio_framework_value_persistent::list::PagedListError)->ValueError{invalid(error.reason)}
fn header()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn act(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Pack action exceeded its original work"))}}
fn owe(effect:&mut RetainedAllocationEffect<'_>,work:u128)->Result<(),ValueError>{let owed=u64::try_from(work.saturating_sub(effect.performed_work())).map_err(|_|invalid("original Pack child work overflow"))?;if owed==0||effect.performed(owed)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Pack child work exceeded its original work"))}}
fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,BorrowedProjectedPackFailure>{match poll{RetainedWorkPoll::Complete(turn)=>turn.result.map(|step|step.progress()).map_err(Into::into),RetainedWorkPoll::Pending{..}=>Err(invalid("original Pack close cannot resume a pending effect").into())}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}

/// 🫴️ The caller receives the identical typed owner and encoded wire after all codec symbols and source leases close.
#[derive(semio_framework_value::RetireOwned)]
pub struct OwnedProjectedPackWireReady<T:RetireOwned>{pub source:T,pub wire:WireBytes}
pub struct OwnedProjectedPackWireStep<T:RetireOwned>{pub ready:Option<OwnedProjectedPackWireReady<T>>,pub progress:RetainedCloneProgress}

/// 🧳️ Native source custody, symbols and partial output remain owned through every denial and cancellation.
pub struct OwnedProjectedPackWireCursor<T:FieldProjectionSource+RetireOwned+Sync>{original:ManuallyDrop<Option<T>>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<T,T>>>,source:ManuallyDrop<Option<RetainedCloneSource<T>>>,cursor:ManuallyDrop<Option<BorrowedProjectedPackCursor>>,wire:ManuallyDrop<WirePages>,original_close:ManuallyDrop<Option<ControlledRetirement<T>>>,take_reply:Option<Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>>,take_pending:bool,scratch:[u8;64],pending_length:usize,pending_offset:usize,field_id:u16,length:usize,written:usize,phase:u8,closing:bool}
impl<T:FieldProjectionSource+RetireOwned+Sync> OwnedProjectedPackWireCursor<T>{
 /// 📥️ Moves only the already-owned original header into inline custody.
 pub fn new(original:T,field_id:u16)->Self{Self{original:ManuallyDrop::new(Some(original)),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),cursor:ManuallyDrop::new(None),wire:ManuallyDrop::new(WirePages::empty()),original_close:ManuallyDrop::new(None),take_reply:None,take_pending:false,scratch:[0;64],pending_length:0,pending_offset:0,field_id,length:0,written:0,phase:0,closing:false}}
 pub fn original(&self)->Option<&T>{self.original.as_ref().or_else(||self.admission.as_ref().and_then(RetainedCloneSourceAdmission::original_authority)).or_else(||self.source.as_ref().and_then(|source|source.try_borrow().ok().map(|loan|loan.get())))}
 pub fn measured_byte_length(&self)->usize{self.length}
 pub fn output_bytes_written(&self)->usize{self.written}
 pub fn phase(&self)->&'static str{if self.closing{"closing"}else{match self.phase{0=>"admittingSource",1=>"measure",2=>"measureClose",3=>"allocate",4=>"initialize",5=>"admittingEncoder",6=>"encode",7=>"encodeClose",8=>"returnSource",9=>"handoff",_=>"complete"}}}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.cursor.is_none()&&self.wire.terminal_is_empty()&&self.original_close.is_none()&&!self.take_pending}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}
  if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(header())}else{cursor.retirement_demands()}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header())}else{source.next_close_demand(body)}}
  if let Some(owner)=self.original_close.as_ref(){return if owner.terminal_is_empty(){Ok(header())}else{owner.selected_demand(body)}}
  if self.original.is_some(){return Ok(header())}if !self.wire.is_empty(){return Ok(RetirementDemand{copy_bytes:self.wire.len().min(body).min(64),depth:self.wire.next_pop_depth_demand().map_err(page_error)?.max(1),..Default::default()})}Ok(RetirementDemand{copy_bytes:self.wire.next_release_copy_byte_demand().map_err(page_error)?,release_bytes:self.wire.next_release_allocation_bytes().map_err(page_error)?,depth:self.wire.next_release_depth_demand().map_err(page_error)?.max(1),..Default::default()})
 }
 /// 📏️ Quotes the exact original source, symbol, output or ownership donation selected by this turn.
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demand(body)}
  if matches!(self.phase,6|7)&&self.pending_offset<self.pending_length{return Ok(RetirementDemand{copy_bytes:(self.pending_length-self.pending_offset).min(body),depth:1,..Default::default()})}
  Ok(match self.phase{
   0=>match self.admission.as_ref(){Some(admission) if admission.phase()!="ready"=>admission.next_demand()?,_=>header()},
   1|6=>{let cursor=self.cursor.as_ref().ok_or_else(||invalid("original Pack codec is absent"))?;RetirementDemand{copy_bytes:cursor.next_minimum_copy_bytes(),capacity_bytes:cursor.next_capacity_byte_demand()?,depth:cursor.next_advance_depth_demand()?,..Default::default()}},
   2|7=>self.cursor.as_ref().map_or(Ok(header()),|cursor|if cursor.terminal_is_empty(){Ok(header())}else{cursor.retirement_demands()})?,
   3=>RetirementDemand{capacity_bytes:self.wire.next_exact_capacity_allocation_bytes(self.length).map_err(page_error)?.unwrap_or(0),depth:self.wire.next_reserve_depth_demand().map_err(page_error)?.max(1),..Default::default()},
   4 if self.wire.len()<self.length=>RetirementDemand{copy_bytes:usize::from(body!=0),depth:1,..Default::default()},
   4|5|9=>header(),
   8=>self.source.as_ref().ok_or_else(||invalid("original Pack source return is absent"))?.next_take_demand(body)?,
   _=>Default::default(),
  })
 }
 fn advance_one(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<OwnedProjectedPackWireStep<T>,BorrowedProjectedPackFailure>{
  let grant=effect.grant();let none=|progress|OwnedProjectedPackWireStep{ready:None,progress};let demand=self.next_demand(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(none(Default::default()))}if self.closing{return self.close_one(effect).map(none)}
  if matches!(self.phase,6|7)&&self.pending_offset<self.pending_length{act(effect)?;let count=(self.pending_length-self.pending_offset).min(grant.maximum_copy_bytes);for index in self.pending_offset..self.pending_offset+count{self.wire.push_reserved(self.scratch[index]).map_err(|_|invalid("original Pack output lost its paid page"))?;}self.pending_offset+=count;return Ok(none(RetainedCloneProgress{copied_bytes:count,..item()}))}
  match self.phase{
   0=>{
    if self.admission.is_none(){act(effect)?;let original=self.original.take().ok_or_else(||invalid("original Pack input is absent"))?;*self.admission=Some(RetainedCloneSource::<T>::admission_owned(original,()));return Ok(none(item()))}
    let admission=self.admission.as_mut().ok_or_else(||invalid("original Pack admission is absent"))?;
    if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,effect);let step=turn.result?;if admission.terminal_is_empty(){self.admission.take();*self.cursor=Some(Default::default());self.phase=1;}Ok(none(step.progress()))}else{let turn=admission.advance(effect);turn.result.map(|step|none(step.progress())).map_err(Into::into)}
   },
   1|6=>{
    let source=self.source.as_ref().ok_or_else(||invalid("original Pack source is absent"))?.try_borrow()?;let cursor=self.cursor.as_mut().ok_or_else(||invalid("original Pack codec is absent"))?;
    let result=cursor.advance_intrinsic(source.get(),self.field_id,&mut self.scratch,effect);
    match result{Ok(step)=>{if self.phase==1{self.length=self.length.checked_add(step.written_bytes).ok_or_else(||invalid("original Pack measured length overflow").with_retained_progress(step.progress))?}else{self.pending_length=step.written_bytes;self.pending_offset=0;self.written=self.written.checked_add(step.written_bytes).filter(|written|*written<=self.length).ok_or_else(||invalid("original Pack source exceeded measured length").with_retained_progress(step.progress))?}if step.complete{if self.phase==6&&self.written!=self.length{return Err(invalid("original Pack source changed its measured length").with_retained_progress(step.progress).into())}self.phase+=1;}Ok(none(step.progress))},Err(original)=>{if self.phase==1{self.length=self.length.saturating_add(original.written_bytes)}else{self.written=self.written.saturating_add(original.written_bytes)}Err(original)}}
   },
   2|7=>{let cursor=self.cursor.as_mut().ok_or_else(||invalid("original Pack symbol close is absent"))?;if cursor.terminal_is_empty(){act(effect)?;self.cursor.take();self.phase+=1;return Ok(none(item()))}cursor.close(effect).map(|step|none(step.progress))},
   3=>{if demand.capacity_bytes==0{act(effect)?;self.phase=4;return Ok(none(item()))}match self.wire.reserve_exact_capacity_one(self.length,&mut effect.nested(grant)).result{Ok(PagedListReservationOutcome::Ready(step))=>Ok(none(RetainedCloneProgress{retained_capacity_bytes:step.allocated_bytes,..item()})),Ok(PagedListReservationOutcome::Denied)=>Ok(none(Default::default())),Err(PagedListReservationFailure::Work(error))=>Err(error.into()),Err(PagedListReservationFailure::Allocation{error,copied_bytes})=>Err(page_error(error.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,copied_bytes,..Default::default()}).into())}},
   4=>{act(effect)?;self.phase=5;Ok(none(item()))},
   5=>{act(effect)?;*self.cursor=Some(Default::default());self.phase=6;Ok(none(item()))},
   8=>{let source=self.source.as_mut().ok_or_else(||invalid("original Pack source return is absent"))?;let body=grant.maximum_copy_bytes;if self.take_reply.is_none(){let child=semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut self.take_reply)?;let done=if self.take_pending{source.recover_take_authority_into(&mut self.original,body,effect,child)?}else{source.take_authority_into(&mut self.original,body,effect,child)?};drop(done);}let Some(result)=self.take_reply.take()else{self.take_pending=true;return Ok(none(Default::default()))};self.take_pending=false;match result?{RetainedCloneStep::Progress(receipt)=>Ok(none(receipt)),RetainedCloneStep::Complete(receipt)=>{assert!(source.terminal_is_empty());self.source.take();self.phase=9;Ok(none(receipt))}}},
   9=>{act(effect)?;let source=self.original.take().ok_or_else(||invalid("original Pack returned input is absent"))?;let wire=WireBytes::from_retained_bytes(std::mem::replace(&mut*self.wire,WirePages::empty()));self.phase=10;Ok(OwnedProjectedPackWireStep{ready:Some(OwnedProjectedPackWireReady{source,wire}),progress:item()})},
   _=>Ok(none(Default::default())),
  }
 }
 /// ✍️ Returns every native codec cause whole while retaining original source and initialized partial bytes for paid close.
 pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<OwnedProjectedPackWireStep<T>,BorrowedProjectedPackFailure>{let result=self.advance_one(effect);if result.is_err(){self.begin_close()}result}
 fn close_one(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,BorrowedProjectedPackFailure>{
  let grant=effect.grant();
  if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);if let Some((original,()))=returned{*self.original=Some(original);}if admission.terminal_is_empty(){self.admission.take();}return turn.result.map(|step|step.progress()).map_err(Into::into)}
  if let Some(cursor)=self.cursor.as_mut(){if cursor.terminal_is_empty(){act(effect)?;self.cursor.take();return Ok(item())}return cursor.close(effect).map(|step|step.progress)}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){act(effect)?;self.source.take();return Ok(item())}return settled(source.close_step(grant.maximum_copy_bytes,effect))}
  if let Some(owner)=self.original_close.as_mut(){if owner.terminal_is_empty(){act(effect)?;self.original_close.take();return Ok(item())}return settled(owner.step(grant.maximum_copy_bytes,effect))}
  if self.original.is_some(){act(effect)?;let original=self.original.take().ok_or_else(||invalid("original Pack input is absent"))?;match ControlledRetirement::new(original){Ok(owner)=>{*self.original_close=Some(owner);return Ok(item())},Err((error,original))=>{*self.original=Some(original);return Err(error.into())}}}
  if !self.wire.is_empty(){act(effect)?;let count=self.wire.len().min(grant.maximum_copy_bytes).min(64);for _ in 0..count{self.wire.pop();}return Ok(RetainedCloneProgress{copied_bytes:count,..item()})}if !self.wire.terminal_is_empty(){return match self.wire.release_empty_page(effect).result{Ok(PagedListReleaseOutcome::Ready(step))=>Ok(RetainedCloneProgress{released_bytes:step.released_allocation_bytes,..item()}),Ok(PagedListReleaseOutcome::Denied)=>Ok(Default::default()),Err(PagedListReleaseFailure::Work(error))=>Err(error.into()),Err(PagedListReleaseFailure::Release(error))=>Err(page_error(error).into())}}Ok(Default::default())
 }
 /// ♻️ Closes the same source, symbols and partial output under the original caller's grant.
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,BorrowedProjectedPackFailure>{let grant=effect.grant();self.begin_close();let demand=self.close_demand(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}let receipt=self.close_one(effect)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)})}
}
impl<T:FieldProjectionSource+RetireOwned+Sync> Drop for OwnedProjectedPackWireCursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Pack source and partial wire require paid terminal close")}}

#[doc(hidden)]pub struct OwnedProjectedPackWireRetirement<T:FieldProjectionSource+RetireOwned+Sync>(ManuallyDrop<OwnedProjectedPackWireCursor<T>>);
impl<T:FieldProjectionSource+RetireOwned+Sync> OwnedProjectedPackWireRetirement<T>{
 fn close_step(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep as Step;let turn=self.0.close_step(effect);let work=effect.performed_work();match turn{Err(failure)=>Step::Failure(failure.reason.into(),work),Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>Step::Complete(work),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Step::Progress(progress,work)}}
}
impl<T:FieldProjectionSource+RetireOwned+Sync> semio_framework_value::retirement::RetirementCursor for OwnedProjectedPackWireRetirement<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.0.close_demand(body)}
 semio_framework_value::native_retirement_reply!();
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{!self.0.terminal_is_empty()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:FieldProjectionSource+RetireOwned+Sync> Drop for OwnedProjectedPackWireRetirement<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.terminal_is_empty(),"original Pack wire cursor abandoned its paid closure");if self.0.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.0)}}}}
impl<T:FieldProjectionSource+RetireOwned+Sync> RetireOwned for OwnedProjectedPackWireCursor<T>{
 type RetirementCursor=OwnedProjectedPackWireRetirement<T>;
 fn retirement_seed(self)->Self::RetirementCursor{OwnedProjectedPackWireRetirement(ManuallyDrop::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OwnedProjectedPackWireRetirement<T>>())}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
