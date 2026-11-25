//! 🏛️ One original-record retirement machine for every derived record: the record keeps its allocation (a unique shared original or an original Box) while its authored field ports transfer one field at a time, then the empty shell is released.
use super::super::{RetirementCursor,RetirementStep,arc_custody::original::SharedOriginal,original_box::BoxOriginal};
use crate::{RetirementDemand,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind,OriginalRetirementFailure};
use crate::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,retained_clone_grant_admits};
use std::mem::size_of;

/// 🏠️ The original allocation a record retires in place.
pub trait OriginalRecordOwner:Send+'static{
 type Record;
 const SHARED:bool;
 fn original_bytes()->usize;
 fn original_initialized(&self)->bool;
 /// # Safety
 /// The projection must name a still-initialized field inside the record.
 unsafe fn original_field<F>(&self,project:fn(*const Self::Record)->*const F)->&F;
 /// # Safety
 /// The projection must name that field exactly once; later operations cannot read or drop it.
 unsafe fn take_original_field<F>(&mut self,project:fn(*mut Self::Record)->*mut F)->F;
 /// # Safety
 /// Every owned field must have transferred before the shell is released.
 unsafe fn release_original(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress;
}
impl<T:Send+Sync+'static> OriginalRecordOwner for SharedOriginal<T>{
 type Record=T;
 const SHARED:bool=true;
 fn original_bytes()->usize{Self::allocation_bytes()}
 fn original_initialized(&self)->bool{self.is_initialized()}
 unsafe fn original_field<F>(&self,project:fn(*const T)->*const F)->&F{unsafe{self.field(project)}}
 unsafe fn take_original_field<F>(&mut self,project:fn(*mut T)->*mut F)->F{unsafe{self.take_field(project)}}
 unsafe fn release_original(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{unsafe{self.release_empty(grant)}}
}
impl<T:Send+'static> OriginalRecordOwner for BoxOriginal<T>{
 type Record=T;
 const SHARED:bool=false;
 fn original_bytes()->usize{Self::allocation_bytes()}
 fn original_initialized(&self)->bool{self.is_initialized()}
 unsafe fn original_field<F>(&self,project:fn(*const T)->*const F)->&F{unsafe{self.field(project)}}
 unsafe fn take_original_field<F>(&mut self,project:fn(*mut T)->*mut F)->F{unsafe{self.take_field(project)}}
 unsafe fn release_original(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{unsafe{self.release_empty(grant)}}
}

/// 🧩️ The authored field ports of one record, shared by both original allocations; `None` past the last field.
pub trait OriginalRecordFields:Sized+Send+'static{
 fn original_field_demand<O:OriginalRecordOwner<Record=Self>>(original:&O,ordinal:usize)->Option<Result<RetirementDemand,ValueError>>;
 /// # Safety
 /// The ordinal names the next untransferred field of the live original.
 unsafe fn original_field_step<O:OriginalRecordOwner<Record=Self>>(original:&mut O,ordinal:usize,effect:&mut RetainedAllocationEffect<'_>)->Option<RetirementStep>;
}

/// ♻️ The native cursor that retires a record from its original allocation.
pub struct OriginalRecordRetirement<O>{original:Option<O>,ordinal:usize}

/// 📐️ Prices the original cursor birth: the moved original slot, the cursor itself, the allocator marker and the step slot.
pub fn original_record_demand<O:OriginalRecordOwner>()->RetirementDemand{let capacity_bytes=size_of::<OriginalRecordRetirement<O>>();RetirementDemand{copy_bytes:size_of::<Option<O>>()+capacity_bytes+2*size_of::<Option<usize>>()+size_of::<Option<RetirementStep>>(),capacity_bytes,depth:1,..Default::default()}}

/// 🏗️ Allocates the original cursor under the caller's effect and moves the intact original into it.
pub fn admit_original_record<O:OriginalRecordOwner>(original:&mut Option<O>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>> where O::Record:OriginalRecordFields{
 let mut work=0;let result=admit(original,effect,&mut work);RetainedWorkOutcome{result,work}
}
fn admit<O:OriginalRecordOwner>(original:&mut Option<O>,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>,ValueError> where O::Record:OriginalRecordFields{
 let grant=effect.grant();
 let Some(source)=original.as_ref()else{return Ok(None)};
 if !source.original_initialized(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,if O::SHARED{"partially transferred shared original requires its existing cursor"}else{"partially transferred original Box requires its existing cursor"}))}
 let demand=original_record_demand::<O>();
 if !retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,if O::SHARED{"shared original cursor birth is not funded"}else{"boxed original cursor birth is not funded"}))}
 if effect.remaining_work_units()?==0{return Ok(None)}
 if effect.actual().is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original cursor allocator marker occupied"))}
 let layout=std::alloc::Layout::new::<OriginalRecordRetirement<O>>();*work=1;
 if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original cursor allocator exceeded its Work"))}
 *effect.actual()=Some(0);let raw=unsafe{std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}
 let Some(pointer)=std::ptr::NonNull::new(raw.cast::<OriginalRecordRetirement<O>>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original cursor allocation failed").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<usize>>(),..Default::default()}))};
 let mut cursor=unsafe{pointer.as_ptr().write(OriginalRecordRetirement{original:None,ordinal:0});Box::from_raw(pointer.as_ptr())};
 cursor.original=original.take();
 Ok(Some((cursor as Box<dyn RetirementCursor>,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})))
}

impl<O:OriginalRecordOwner> OriginalRecordRetirement<O> where O::Record:OriginalRecordFields{
 fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
  let grant=effect.grant();
  let demand=match RetirementCursor::next_demand(self,body){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if !retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  let Some(original)=self.original.as_mut()else{return RetirementStep::Complete(0)};
  if let Some(turn)=unsafe{O::Record::original_field_step(original,self.ordinal,effect)}{if matches!(&turn,RetirementStep::ChildProgress(..)){self.ordinal+=1}return turn}
  let work=match effect.performed(1){Ok(true)=>1,Ok(false)=>return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::WorkLimit,"original header release exceeds its Work ceiling").into(),1),Err(error)=>return RetirementStep::Failure(error.into(),0)};
  let mut receipt=unsafe{original.release_original(grant)};if receipt.copied_items==0{return RetirementStep::Progress(receipt,1)}
  self.original=None;receipt.copied_bytes+=size_of::<Option<O>>();RetirementStep::Progress(receipt,work)
 }
}

impl<O:OriginalRecordOwner> RetirementCursor for OriginalRecordRetirement<O> where O::Record:OriginalRecordFields{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.original.is_some()))}
 fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{let Some(original)=self.original.as_ref()else{return Ok(Default::default())};O::Record::original_field_demand(original,self.ordinal).unwrap_or_else(||Ok(RetirementDemand{copy_bytes:size_of::<Option<O>>(),release_bytes:O::original_bytes(),depth:1,..Default::default()}))}
 fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{
  if effect.invocation().is_none(){return Err(ValueError::invariant("original native field retirement requires its actual invocation"))}
  Ok(match self.step(body,effect){RetirementStep::Failure(error,_)=>reply.complete(Err(error)),step=>reply.complete(Ok(step))})
 }
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(RetirementCursor::next_demand(self,0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{RetirementCursor::next_demand(self,body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{RetirementCursor::next_demand(self,0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(RetirementCursor::next_demand(self,0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.original.is_none().then_some(size_of::<Self>())}
}
