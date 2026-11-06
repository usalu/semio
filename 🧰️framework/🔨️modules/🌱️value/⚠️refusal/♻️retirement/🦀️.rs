//! ♻️ Native refusal prose retires through its original borrowed or owned text authority.
use crate::ValueError;
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::borrow::Cow;
impl RetireOwned for ValueError{
 type RetirementCursor=<String as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{match self.message{Cow::Borrowed(_)=>String::new().retirement_seed(),Cow::Owned(text)=>text.retirement_seed()}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
/// 🪪️ Keeps an original transaction inline until its actual authority has finished it.
#[doc(hidden)]
pub struct AllocationTransactionRetirement{original:std::mem::ManuallyDrop<Option<crate::NativeAllocationTransaction>>}
fn transaction_refusal()->ValueError{ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original allocation transaction requires its authority")}
impl RetirementCursor for AllocationTransactionRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{self.next_demand(0)?;Ok(u64::from(self.original.is_some()))}
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,ValueError>{if self.original.as_ref().is_some_and(|original|!original.terminal_is_empty()){return Err(transaction_refusal())}Ok(if self.original.is_some(){crate::RetirementDemand{copy_bytes:std::mem::size_of::<Option<crate::NativeAllocationTransaction>>(),depth:1,..Default::default()}}else{Default::default()})}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{if self.original.is_none(){return RetirementStep::Complete(0)}let demand=match self.next_demand(effect.grant().maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};if effect.maximum_work_units()<1||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}drop(self.original.take());RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1)}
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for AllocationTransactionRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original transaction cursor requires funded closure");unsafe{std::mem::ManuallyDrop::drop(&mut self.original)}}}
impl RetireOwned for crate::NativeAllocationTransaction{
 type RetirementCursor=AllocationTransactionRetirement;
 fn retirement_ready(&self)->bool{self.terminal_is_empty()}
 fn retirement_seed(self)->Self::RetirementCursor{AllocationTransactionRetirement{original:std::mem::ManuallyDrop::new(Some(self))}}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
/// 🔐️ Retains the actual vault handle when original readiness has not yet returned.
#[doc(hidden)]
pub enum AllocationRefusalRetirement{Owned(<ValueError as RetireOwned>::RetirementCursor),Retained(crate::NativeAllocationRefusalHandle)}
fn vault_refusal()->ValueError{ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retained allocation refusal must return to its original vault")}
impl RetirementCursor for AllocationRefusalRetirement{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{match self{Self::Owned(cursor)=>cursor.next_computational_work(body),Self::Retained(_)=>Err(vault_refusal())}}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{match self{Self::Owned(cursor)=>cursor.next_demand(body),Self::Retained(_)=>Err(vault_refusal())}}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{match self{Self::Owned(cursor)=>{match cursor.next_computational_work(effect.grant().maximum_copy_bytes){Ok(work)if work>effect.maximum_work_units()=>return RetirementStep::BudgetExhausted,Err(error)=>return RetirementStep::Failure(error.into(),0),_=>{}}cursor.close_step(effect)},Self::Retained(_)=>RetirementStep::Failure(vault_refusal().into(),0)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Owned(cursor)=>cursor.terminal_is_empty(),Self::Retained(_)=>false}}
 fn next_depth_demand(&self)->Result<usize,ValueError>{match self{Self::Owned(cursor)=>cursor.next_depth_demand(),Self::Retained(_)=>Err(vault_refusal())}}
 fn next_close_byte_demand(&self)->Option<usize>{match self{Self::Owned(cursor)=>cursor.next_close_byte_demand(),Self::Retained(_)=>None}}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{match self{Self::Owned(cursor)=>cursor.next_work_byte_demand(),Self::Retained(_)=>Err(vault_refusal())}}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{match self{Self::Owned(cursor)=>cursor.next_selected_work_byte_demand(body),Self::Retained(_)=>Err(vault_refusal())}}
 fn allows_admitted_narrow_work(&self)->bool{matches!(self,Self::Owned(cursor)if cursor.allows_admitted_narrow_work())}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{match self{Self::Owned(cursor)=>cursor.next_birth_bytes(body),Self::Retained(_)=>None}}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for AllocationRefusalRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original refusal cursor requires its original vault or funded closure")}}
impl RetireOwned for crate::NativeAllocationRefusal{
 type RetirementCursor=AllocationRefusalRetirement;
 fn retirement_ready(&self)->bool{matches!(self,Self::Owned(_))}
 fn retirement_seed(self)->Self::RetirementCursor{match self{Self::Owned(error)=>AllocationRefusalRetirement::Owned(error.retirement_seed()),Self::Retained(handle)=>AllocationRefusalRetirement::Retained(handle)}}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}

type OriginalRetirementFailureParts=(Option<ValueError>,Option<ValueError>,Option<ValueError>,Option<crate::NativeAllocationRefusal>,Option<Box<crate::OriginalRetirementFailure>>);
/// 🧳️ Owns the original failure fields through the common nonallocating tuple seed.
#[doc(hidden)]
pub struct OriginalRetirementFailureCursor(<OriginalRetirementFailureParts as RetireOwned>::RetirementCursor);
impl RetirementCursor for OriginalRetirementFailureCursor{
 fn requires_original_control(&self)->bool{self.0.requires_original_control()}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::retirement::RetirementStep{self.0.close_step_controlled(effect,control)}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{self.0.close_original_step(original,effect)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.next_computational_work(body)}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{self.0.next_demand(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{self.0.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.0.next_depth_demand()}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_byte_demand()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.0.next_work_byte_demand()}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.0.next_selected_work_byte_demand(body)}
 fn allows_admitted_narrow_work(&self)->bool{self.0.allows_admitted_narrow_work()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.next_birth_bytes(body)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl RetireOwned for crate::OriginalRetirementFailure {
 type RetirementCursor=OriginalRetirementFailureCursor;
 fn retirement_ready(&self)->bool{self.allocation_refusal().is_none_or(RetireOwned::retirement_ready)&&self.child().is_none_or(RetireOwned::retirement_ready)}
 fn retirement_seed(self)->Self::RetirementCursor{let(primary,work,receipt,allocation,child,_progress)=self.into_parts();OriginalRetirementFailureCursor((primary,work,receipt,allocation,child).retirement_seed())}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}

/// 📦️ Retains the exact original spare node until its separately funded physical release.
#[doc(hidden)]
pub struct OriginalFailureStorageRetirement(std::mem::ManuallyDrop<Option<crate::OriginalFailureStorage>>);
impl RetireOwned for crate::OriginalFailureStorage{
 type RetirementCursor=OriginalFailureStorageRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{OriginalFailureStorageRetirement(std::mem::ManuallyDrop::new(Some(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for OriginalFailureStorageRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,ValueError>{Ok(if self.0.is_some(){crate::RetirementDemand{release_bytes:crate::OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()}}else{Default::default()})}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::retirement::RetirementStep{let grant=effect.grant();
  use crate::retirement::RetirementStep;
  if self.0.is_none(){return RetirementStep::Complete(0)}
  if effect.maximum_work_units()<1{return RetirementStep::BudgetExhausted}
  if !crate::retained_clone::retained_clone_grant_admits(grant,self.next_demand(grant.maximum_copy_bytes).unwrap()){return RetirementStep::BudgetExhausted}
  let storage=self.0.take().unwrap();let progress=storage.release(grant.maximum_release_bytes).expect("original spare-node release was admitted");
  RetirementStep::Progress(crate::RetainedCloneProgress{copied_items:1,..progress},1)
 }
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for OriginalFailureStorageRetirement{
 fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"spare failure storage requires funded closure");if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.0)}}}
}


use crate::NativeAllocationFailure;

type NativeAllocationFailureParts=(Option<ValueError>,Option<ValueError>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<crate::NativeAllocationRefusal>,Option<ValueError>,Option<ValueError>);
impl RetireOwned for NativeAllocationFailure{
 type RetirementCursor=<NativeAllocationFailureParts as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{[&self.forwarded_admission,&self.scoped_admission,&self.forwarded_quote,&self.scoped_quote,&self.forwarded_post,&self.scoped_post,&self.forwarded_cancel,&self.scoped_cancel].into_iter().all(RetireOwned::retirement_ready)}
 fn retirement_seed(self)->Self::RetirementCursor{(self.primary,self.admission,self.forwarded_admission,self.scoped_admission,self.forwarded_quote,self.scoped_quote,self.forwarded_post,self.scoped_post,self.forwarded_cancel,self.scoped_cancel,self.receipt,self.extent).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
mod tests{
 use super::*;
 use crate::{RetainedCloneGrant,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
 #[test]
 fn all_original_allocation_causes_retire_under_grant(){
  let cause=||Some(ValueError::new(ValueRefusalKind::Canceled,String::from("original allocation cause")));let failure=NativeAllocationFailure{primary:cause(),admission:cause(),forwarded_admission:cause().map(Into::into),scoped_admission:cause().map(Into::into),forwarded_quote:cause().map(Into::into),scoped_quote:cause().map(Into::into),forwarded_post:cause().map(Into::into),scoped_post:cause().map(Into::into),forwarded_cancel:cause().map(Into::into),scoped_cancel:cause().map(Into::into),receipt:cause(),extent:cause(),..Default::default()};let mut owner=ControlledRetirement::new(failure).ok().unwrap();let grant=RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};while !owner.terminal_is_empty(){let work=owner.next_computational_work(grant.maximum_copy_bytes).unwrap();let mut actual=None;let mut effect=crate::RetainedAllocationEffect::new(grant,work,&mut actual);let step=owner.step(&mut effect).unwrap();assert_eq!(actual.unwrap_or(0),step.progress().retained_capacity_bytes);}
 }
}
