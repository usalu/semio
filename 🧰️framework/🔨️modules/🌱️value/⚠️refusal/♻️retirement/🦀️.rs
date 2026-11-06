//! ♻️ Native refusal prose retires through its original borrowed or owned text authority.
use crate::ValueError;
use crate::retirement::{RetireOwned,RetirementCursor};
use std::borrow::Cow;
impl RetireOwned for ValueError {
 fn retirement(self)->Box<dyn RetirementCursor>{match self.message{Cow::Borrowed(_)=>().retirement(),Cow::Owned(text)=>text.retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match &self.message{Cow::Borrowed(_)=>().retirement_birth_bytes(),Cow::Owned(text)=>text.retirement_birth_bytes()}}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for crate::OriginalRetirementFailure {
 fn retirement(self)->Box<dyn RetirementCursor>{let(primary,work,receipt,child,_progress)=self.into_parts();(primary,work,receipt,child).retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use crate::retirement::{sequence_birth_bytes,deferred_birth_bytes};sequence_birth_bytes(&[deferred_birth_bytes::<Option<ValueError>>(),deferred_birth_bytes::<Option<ValueError>>(),deferred_birth_bytes::<Option<ValueError>>(),deferred_birth_bytes::<Option<Box<Self>>>()])}
 fn controlled_retirement_supported()->bool{true}
}

struct OriginalFailureStorageRetirement(std::mem::ManuallyDrop<Option<crate::OriginalFailureStorage>>);
impl RetireOwned for crate::OriginalFailureStorage{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OriginalFailureStorageRetirement(std::mem::ManuallyDrop::new(Some(self))))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<OriginalFailureStorageRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for OriginalFailureStorageRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,ValueError>{Ok(if self.0.is_some(){crate::RetirementDemand{release_bytes:crate::OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()}}else{Default::default()})}
 fn close_step(&mut self,grant:crate::RetainedCloneGrant)->crate::retirement::RetirementStep{
  use crate::retirement::RetirementStep;
  if self.0.is_none(){return RetirementStep::Complete(0)}
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
use crate::retirement as allocation_retirement;

impl RetireOwned for NativeAllocationFailure{
 fn retirement(self)->Box<dyn RetirementCursor>{allocation_retirement::sequence(vec![allocation_retirement::deferred(self.primary),allocation_retirement::deferred(self.admission),allocation_retirement::deferred(self.forwarded_admission),allocation_retirement::deferred(self.scoped_admission),allocation_retirement::deferred(self.forwarded_quote),allocation_retirement::deferred(self.scoped_quote),allocation_retirement::deferred(self.forwarded_post),allocation_retirement::deferred(self.scoped_post),allocation_retirement::deferred(self.forwarded_cancel),allocation_retirement::deferred(self.scoped_cancel),allocation_retirement::deferred(self.receipt),allocation_retirement::deferred(self.extent)])}
 fn retirement_birth_bytes(&self)->Option<usize>{allocation_retirement::sequence_birth_bytes(&[allocation_retirement::deferred_birth_bytes_for(&self.primary),allocation_retirement::deferred_birth_bytes_for(&self.admission),allocation_retirement::deferred_birth_bytes_for(&self.forwarded_admission),allocation_retirement::deferred_birth_bytes_for(&self.scoped_admission),allocation_retirement::deferred_birth_bytes_for(&self.forwarded_quote),allocation_retirement::deferred_birth_bytes_for(&self.scoped_quote),allocation_retirement::deferred_birth_bytes_for(&self.forwarded_post),allocation_retirement::deferred_birth_bytes_for(&self.scoped_post),allocation_retirement::deferred_birth_bytes_for(&self.forwarded_cancel),allocation_retirement::deferred_birth_bytes_for(&self.scoped_cancel),allocation_retirement::deferred_birth_bytes_for(&self.receipt),allocation_retirement::deferred_birth_bytes_for(&self.extent)])}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
mod tests{
 use super::*;
 use crate::{RetainedCloneGrant,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
 #[test]
 fn all_original_allocation_causes_retire_under_grant(){
  let cause=||Some(ValueError::new(ValueRefusalKind::Canceled,String::from("original allocation cause")));let failure=NativeAllocationFailure{primary:cause(),admission:cause(),forwarded_admission:cause(),scoped_admission:cause(),forwarded_quote:cause(),scoped_quote:cause(),forwarded_post:cause(),scoped_post:cause(),forwarded_cancel:cause(),scoped_cancel:cause(),receipt:cause(),extent:cause(),..Default::default()};let mut owner=ControlledRetirement::new(failure).ok().unwrap();let grant=RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};while !owner.terminal_is_empty(){owner.step(grant).unwrap();}
 }
}
