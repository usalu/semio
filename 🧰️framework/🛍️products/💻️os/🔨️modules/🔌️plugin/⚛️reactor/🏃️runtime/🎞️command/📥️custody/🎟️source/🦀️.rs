//! 🎟️ Actor-local command descendants retain their real admitting Source until controlled retirement returns its loan.
use semio_framework_job::{OriginalCallerSourceLoan,OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"command origin differs from its actual admitting Source")}
pub struct OriginalCommandOrigin{loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,binding:Option<OriginalSourceBinding>}
impl OriginalCommandOrigin{
 pub fn empty()->Self{Self{loan:ManuallyDrop::new(None),binding:None}}
 pub fn is_admitted(&self)->bool{self.binding.is_some()&&self.loan.is_some()}
 pub fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(refused().into())}if let Some(loan)=self.loan.as_ref(){loan.validate_retirement_context(cx)?;}Ok(())}
 /// 🌱️ The actual alias is retained before its receipt is posted and before any page or decoder can be captured.
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.is_admitted(){return Ok(true)}let bytes=size_of::<Option<OriginalSourceBinding>>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()<2{return Ok(false)}if !cx.original_source_parent()?.admit_loan(&mut*self.loan,cx)?{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.binding=Some(OriginalSourceBinding::borrow(cx));Ok(true)}
 /// 📮️ The defining command owner invokes this only after its actual descendants have completed controlled retirement.
 pub(super) fn return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;let bytes=size_of::<Option<OriginalCallerSourceLoan>>()+size_of::<Option<OriginalSourceBinding>>();let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()<2{return Ok(false)}if let Some(loan)=self.loan.as_mut(){if !loan.terminal_is_empty(){let parent=cx.original_source_parent()?;loan.close_step(parent,cx)?;return Ok(false)}}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.loan.take();self.binding=None;Ok(true)}
 pub fn terminal_is_empty(&self)->bool{self.loan.is_none()&&self.binding.is_none()}
}
impl Drop for OriginalCommandOrigin{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"command Source origin requires controlled descendant retirement and paid return")}}
#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_value::RetainedCloneGrant;
 fn clock()->Option<u64>{Some(1)}
 #[test]
 fn instance_command_origin_fences_actual_source_and_denies_foreign_retirement(){
  let cases:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();assert_eq!(cases["cases"].as_array().unwrap().len(),8);
  let grant=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(671),Generation(1),grant,clock,1000000,100000);
  let mut foreign=OriginalJobFixtureCaller::admit(OperationId(672),Generation(1),grant,clock,1000000,100000);let mut origin=OriginalCommandOrigin::empty();
  assert!(!caller.turn(RetainedCloneGrant{maximum_items:0,..grant},|cx|origin.admit_step(cx)).unwrap());assert!(origin.terminal_is_empty());
  assert!(caller.turn(grant,|cx|origin.admit_step(cx)).unwrap());
  caller.turn(grant,|cx|{assert!(!cx.original_source_parent()?.original_loans_are_returned());origin.validate(cx)}).unwrap();
  let before=foreign.cumulative_receipt();assert!(foreign.turn(grant,|cx|origin.return_step(cx)).is_err());assert_eq!(before,foreign.cumulative_receipt());assert!(origin.is_admitted());
  caller.cancel().cancel_now();for _ in 0..32{if origin.terminal_is_empty(){break}caller.turn(grant,|cx|cx.with_original_retirement(|cx|origin.return_step(cx))).unwrap();}
  assert!(origin.terminal_is_empty());caller.turn(grant,|cx|{assert!(cx.original_source_parent()?.original_loans_are_returned());Ok(())}).unwrap();drop(origin);caller.retire(grant);foreign.retire(grant);
 }


}
