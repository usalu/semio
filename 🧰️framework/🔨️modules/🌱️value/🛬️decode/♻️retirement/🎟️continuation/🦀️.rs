//! 🎟️ Consuming decoder receipts retain their original work scope until every reservation and proof returns.
use super::*;
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use crate::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
pub struct DecodeContinuationRetirement(Option<NativeDecodeContinuation>);
impl RetirementCursor for DecodeContinuationRetirement{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.0.is_some()))}
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:if self.0.is_some(){size_of::<Option<NativeDecodeContinuation>>()}else{0},depth:usize::from(self.0.is_some()),..Default::default()})}
 fn close_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetirementStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'reply,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{if self.0.is_none(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}if self.0.as_ref().is_some_and(|receipt|!RetireOwned::retirement_ready(receipt)){return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"decoder continuation retains original ledger or work").into())))}let demand=self.next_demand(body)?;if effect.remaining_work_units()?<1||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"decoder continuation close exceeded original Work").into())))}self.0.take();Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1))))}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn terminal_is_empty(&self)->bool{self.0.is_none()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.0.is_some()))}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for DecodeContinuationRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.0.is_none(),"original decode continuation requires paid closure")}}
impl RetireOwned for NativeDecodeContinuation{
 type RetirementCursor=DecodeContinuationRetirement;
 fn retirement_ready(&self)->bool{!self.work.has_live_actions()&&self.ledger.as_ref().is_some_and(|ledger|!ledger.is_bound())}
 fn retirement_seed(self)->Self::RetirementCursor{DecodeContinuationRetirement(Some(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{RetireOwned::retirement_ready(self).then_some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{0}
}
#[cfg(test)]
mod tests{
 use super::*;
 fn fields(receipt:&NativeDecodeContinuation)->(u128,crate::NativeAllocationIdentity,bool,usize,usize,usize,usize,u64){(receipt.work.actual(),receipt.ledger.as_ref().expect("original decode ledger present").identity(),receipt.receiving,receipt.maximum_bytes,receipt.ledger.as_ref().expect("original decode ledger present").owned_bytes(),receipt.completed,receipt.total,receipt.stage)}
 fn close_once(source:&mut DecodeContinuationRetirement,bank:&mut semio_framework_value_testing_effect_bank::EffectBank,grant:RetainedCloneGrant)->(RetirementStep,(usize,usize)){let mut reply=None;let((turn,marker),heap)=crate::observe_retirement_allocations(||bank.run(grant,1,|effect|{let token=crate::OriginalEffectReply::borrow(&mut reply).unwrap();let completed=source.close_into(grant.maximum_copy_bytes,effect,token).unwrap();assert!(completed.is_recorded());drop(completed);crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work:effect.performed_work()})}));assert!(matches!(turn,crate::RetainedWorkPoll::Complete(_)));let _=bank.take_admitted();assert_eq!(marker,None);let step=reply.take().unwrap().unwrap();if let RetirementStep::Progress(progress,_)=&step{bank.post(*progress,grant,heap)}(step,heap)}
 #[test]
 fn original_decode_continuation_closes_exact_scalar_receipt_without_unfunded_effects(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(law["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);control.charge(law["ownedBytes"].as_u64().unwrap()as usize).unwrap();let receipt=control.pause().unwrap();let original=fields(&receipt);assert_eq!(receipt.retirement_birth_bytes(),Some(size_of::<DecodeContinuationRetirement>()));let mut source=DecodeContinuationRetirement(Some(receipt));let demand=source.next_demand(0).unwrap();assert_eq!(law["copyBytes"],"native-header");assert_eq!(demand,RetirementDemand{copy_bytes:size_of::<Option<NativeDecodeContinuation>>(),depth:1,..Default::default()});let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:1,..Default::default()};let mut bank=semio_framework_value_testing_effect_bank::EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::new(grant,8));assert_eq!(source.next_computational_work(0).unwrap(),law["work"].as_u64().unwrap());
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}]{let(step,heap)=close_once(&mut source,&mut bank,denied);assert!(matches!(step,RetirementStep::BudgetExhausted|RetirementStep::Failure(_,0)));assert_eq!(heap,(0,0));assert_eq!(fields(source.0.as_ref().unwrap()),original);assert_eq!(source.next_demand(0).unwrap(),demand);assert_eq!(source.next_computational_work(0).unwrap(),law["work"].as_u64().unwrap());}
  let(step,heap)=close_once(&mut source,&mut bank,grant);assert_eq!(heap,(0,0));let RetirementStep::Progress(progress,work)=step else{panic!("funded scalar receipt closure refused")};assert_eq!(progress,RetainedCloneProgress{copied_items:law["items"].as_u64().unwrap()as usize,copied_bytes:demand.copy_bytes,..Default::default()});assert_eq!(work,law["work"].as_u64().unwrap());assert!(source.terminal_is_empty());assert_eq!(source.next_computational_work(0).unwrap(),0);assert_eq!(source.terminal_release_bytes(),Some(size_of::<DecodeContinuationRetirement>()));assert_eq!(crate::observe_retirement_allocations(||drop(source)).1,(0,0));
  }
}
