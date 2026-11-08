//! 🧱️ Concrete record seeds transfer only the selected initialized field.
use super::{RetireOwned,RetirementCursor,RetirementStep,retirement_cursor_birth_demand};
use crate::{RetirementDemand,RetainedAllocationEffect,ValueError,ValueRefusalKind};
use std::mem::{MaybeUninit,size_of};
/// 📐️ Prices field transfer, its defining cursor and the retained ordinal update.
pub fn field_birth_demand<T:RetireOwned>(value:&T)->Result<RetirementDemand,ValueError>{let mut demand=retirement_cursor_birth_demand(value)?;demand.copy_bytes=size_of::<T>().checked_add(size_of::<T::RetirementCursor>()).and_then(|n|n.checked_add(size_of::<usize>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"record field header overflow"))?;Ok(demand)}
/// 🧷️ Borrows only a type witness for recursive native support.
pub fn field_supported<T:RetireOwned>(_:*const T,path:&super::RetirementSupport<'_>)->bool{path.supports::<T>()}
/// 🏗️ Allocates before moving the exact initialized field into its concrete seed.
/// # Safety
/// The pointer names one initialized exclusive field; after a child is returned that field must never be read or dropped again.
pub unsafe fn admit_field<T:RetireOwned>(value:*mut T,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{unsafe{admit_field_with(value,effect,||value.read())}}
/// 🎟️ Preserves the defining source owner's field-transfer bookkeeping after allocation admission.
/// # Safety
/// The pointer and transfer must name the same exclusive initialized field; transfer must only move inline ownership and cannot allocate, release, call out or unwind.
pub unsafe fn admit_field_with<T:RetireOwned>(value:*const T,effect:&mut RetainedAllocationEffect<'_>,take:impl FnOnce()->T)->RetirementStep{
 let demand=match field_birth_demand(unsafe{&*value}){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};let grant=effect.grant();if effect.maximum_work_units()==0||grant.maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}if effect.actual().is_some(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"record field allocator marker occupied").into(),0)}
 let layout=std::alloc::Layout::new::<T::RetirementCursor>();let pointer=if layout.size()==0{std::ptr::NonNull::<T::RetirementCursor>::dangling()}else{*effect.actual()=Some(0);let raw=unsafe{std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}let Some(pointer)=std::ptr::NonNull::new(raw.cast::<T::RetirementCursor>())else{return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::AllocationFailed,"record field cursor allocation failed").into(),1)};pointer};
 let cursor=unsafe{pointer.as_ptr().write(take().retirement_seed());Box::from_raw(pointer.as_ptr())};RetirementStep::ChildProgress(cursor,crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()},1)
}
/// 🧳️ Holds the original record inline while authored field ports price and transfer one owner at a time.
pub struct RetirementRecord<T:Send+'static>{original:MaybeUninit<T>,ordinal:usize,count:usize,demand:unsafe fn(*const T,usize)->Result<RetirementDemand,ValueError>,step:unsafe fn(*mut T,usize,&mut RetainedAllocationEffect<'_>)->RetirementStep,release:unsafe fn(*mut T,usize)}
impl<T:Send+'static> RetirementRecord<T>{
 /// 🌱️ Initializes metadata without calling a producer.
 /// # Safety
 /// Ports must select each declared field exactly once; the source type must have no custom Drop and every field must transfer before terminal; `release` drops exactly the fields whose ordinal is at least the given one.
 pub unsafe fn new(original:T,count:usize,demand:unsafe fn(*const T,usize)->Result<RetirementDemand,ValueError>,step:unsafe fn(*mut T,usize,&mut RetainedAllocationEffect<'_>)->RetirementStep,release:unsafe fn(*mut T,usize))->Self{Self{original:MaybeUninit::new(original),ordinal:0,count,demand,step,release}}
}
impl<T:Send+'static> RetirementCursor for RetirementRecord<T>{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){Ok(Default::default())}else{unsafe{(self.demand)(self.original.as_ptr(),self.ordinal)}}}
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(!self.terminal_is_empty()))}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete(0)}let turn=unsafe{(self.step)(self.original.as_mut_ptr(),self.ordinal,effect)};if matches!(&turn,RetirementStep::ChildProgress(..)){self.ordinal+=1}turn}
 fn terminal_is_empty(&self)->bool{self.ordinal==self.count}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
}
impl<T:Send+'static> Drop for RetirementRecord<T>{fn drop(&mut self){let settled=self.terminal_is_empty();assert!(super::unsettled_drop_allowed()||settled,"record fields remain owned before terminal closure");if !settled&&super::controlled::abandoning(){unsafe{(self.release)(self.original.as_mut_ptr(),self.ordinal)}}}}
