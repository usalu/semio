//! 🎟️ Decoder continuation receipts hold only scalar admission counters and close as one declared metadata event.
use super::*;
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use crate::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
struct DecodeContinuationRetirement(Option<NativeDecodeContinuation>);
impl RetirementCursor for DecodeContinuationRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:usize::from(self.0.is_some()),..Default::default()})}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.0.is_none(){return RetirementStep::Complete(0)}if self.0.as_ref().is_some_and(|receipt|receipt.reserved_issuer.is_some()||receipt.reserved_action.is_some()){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"decoder continuation retains unresolved work reservation").into(),0)}if grant.maximum_items==0{return RetirementStep::BudgetExhausted}if grant.maximum_depth==0{return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit,"decoding continuation requires original source depth").into(),0)}self.0.take();RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},1)}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.0.is_some()))}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for NativeDecodeContinuation{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(DecodeContinuationRetirement(Some(self)))}
 fn retirement_birth_bytes(&self)->Option<usize>{(self.reserved_issuer.is_none()&&self.reserved_action.is_none()).then_some(size_of::<DecodeContinuationRetirement>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{0}
}
#[cfg(test)]
mod tests{
 use super::*;
 fn fields(receipt:&NativeDecodeContinuation)->(u64,u64,bool,usize,usize,usize,usize,u64){(receipt.actual_work,receipt.ledger,receipt.receiving,receipt.maximum_bytes,receipt.owned_bytes,receipt.completed,receipt.total,receipt.stage)}
 #[test]
 fn original_decode_continuation_closes_exact_scalar_receipt_without_unfunded_effects(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);control.charge(law["ownedBytes"].as_u64().unwrap()as usize).unwrap();let receipt=control.pause().unwrap();let original=fields(&receipt);assert_eq!(receipt.retirement_birth_bytes(),Some(size_of::<DecodeContinuationRetirement>()));let mut source=DecodeContinuationRetirement(Some(receipt));let demand=source.next_demand(0).unwrap();assert_eq!(demand,RetirementDemand{depth:1,..Default::default()});let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
  assert_eq!(source.next_computational_work(0).unwrap(),law["work"].as_u64().unwrap());
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=crate::observe_retirement_allocations(||source.close_step(denied));assert!(matches!(step,RetirementStep::BudgetExhausted|RetirementStep::Failure(_,0)));assert_eq!(heap,(0,0));assert_eq!(fields(source.0.as_ref().unwrap()),original);assert_eq!(source.next_demand(0).unwrap(),demand);assert_eq!(source.next_computational_work(0).unwrap(),law["work"].as_u64().unwrap());}
  let(step,heap)=crate::observe_retirement_allocations(||source.close_step(grant));assert_eq!(heap,(0,0));let RetirementStep::Progress(progress,work)=step else{panic!("funded scalar receipt closure refused")};assert_eq!(progress,RetainedCloneProgress{copied_items:law["items"].as_u64().unwrap()as usize,..Default::default()});assert_eq!(work,law["work"].as_u64().unwrap());assert!(source.terminal_is_empty());assert_eq!(source.next_computational_work(0).unwrap(),0);assert_eq!(source.terminal_release_bytes(),Some(size_of::<DecodeContinuationRetirement>()));assert_eq!(crate::observe_retirement_allocations(||drop(source)).1,(0,0));
 }
}
