//! 🛬️ The native receiving flow uses the actual shared outcome owner.
pub use semio_framework_value_native_outcome::OriginalReceivingPrimary;
#[cfg(test)]
use crate::{ValueError,retirement::RetireOwned};
#[cfg(test)]
mod custody_tests{
 use super::*;
 use crate::{NativeAllocationRefusal as Refusal,NativeAllocationRefusalVault as Vault,OriginalRetirementFailure as Failure};
 #[test]
 fn receiving_refuses_retirement_until_original_refusal_vault_returns_its_owner(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🎟️control/🧬️contract/♻️failure/🧪️tests/allocation-custody.json")).unwrap();
  for slot in 0..4{
   let error=ValueError::new(crate::ValueRefusalKind::OwnershipLimit,fixture[0]["receipt"].as_str().unwrap().to_owned());let pointer=error.message.as_ptr();let mut vault=Vault::default();let handle=vault.retain(error).unwrap();
   let mut effect=crate::native_decoding::NativeReceivingEffectFailure::<Failure>{child:None,work_refusal:None,receiving:OriginalReceivingPrimary::from(ValueError::literal(crate::ValueRefusalKind::Canceled,"original context"))};
   match slot{0=>effect.receiving.retirement_failure=Some(Failure::from_allocation_refusal(Refusal::Retained(handle))),1=>effect.receiving.allocation_failure=Some(crate::NativeAllocationFailure{forwarded_post:Some(Refusal::Retained(handle)),..Default::default()}),2=>effect.receiving.native_allocation=Some(crate::native_decoding::allocation::NativeDecodeAllocationOutcome{request:None,admitted:false,primary:None,quote_refusal:None,extent_refusal:None,settlement_refusal:Some(Refusal::Retained(handle)),actual:None}),_=>effect.child=Some(Failure::from_allocation_refusal(Refusal::Retained(handle)))}
   assert!(!effect.retirement_ready());assert!(effect.retirement_birth_bytes().is_none());
   let context=ValueError::new(crate::ValueRefusalKind::InvariantViolated,String::from("actual enclosing context"));let context_pointer=context.message.as_ptr();let construction=super::super::NativeConstructionStepFailure{primary:Some(effect),receipt_refusal:None,progress:Default::default()};
   let envelope=crate::native_decoding::NativeCallFailure::<_,ValueError>{output:Some(construction),operation:None,context:Some(context),receipt_refusal:None};assert!(!envelope.retirement_ready());assert!(envelope.retirement_birth_bytes().is_none());
   let mut owner=crate::retirement::controlled::ControlledRetirement::retain(envelope);let(result,heap)=crate::allocation_testing::observe_retirement_allocations(||owner.next_demand(4096));assert!(result.is_err());assert_eq!(heap,(0,0));assert!(owner.original_is_untouched());let mut envelope=owner.take_original().unwrap();assert!(owner.terminal_is_empty());assert_eq!(envelope.context.as_ref().unwrap().message.as_ptr(),context_pointer);let mut construction=envelope.output.take().unwrap();let mut effect=construction.primary.take().unwrap();
   let refusal=match slot{0|3=>{let failure=if slot==0{effect.receiving.retirement_failure.take().unwrap()}else{effect.child.take().unwrap()};let(a,b,c,d,child,over)=failure.into_parts();assert!(a.is_none()&&b.is_none()&&c.is_none()&&child.is_none()&&over.is_none());d.unwrap()},1=>effect.receiving.allocation_failure.as_mut().unwrap().forwarded_post.take().unwrap(),_=>effect.receiving.native_allocation.as_mut().unwrap().settlement_refusal.take().unwrap()};
   let handle=refusal.into_owned().unwrap_err();let original=vault.take(handle).unwrap();assert_eq!(original.message.as_ptr(),pointer);let owned=Refusal::Owned(original);
   match slot{0=>effect.receiving.retirement_failure=Some(Failure::from_allocation_refusal(owned)),1=>effect.receiving.allocation_failure.as_mut().unwrap().forwarded_post=Some(owned),2=>effect.receiving.native_allocation.as_mut().unwrap().settlement_refusal=Some(owned),_=>effect.child=Some(Failure::from_allocation_refusal(owned))}
   assert!(effect.retirement_ready());assert!(effect.retirement_birth_bytes().is_some());construction.primary=Some(effect);envelope.output=Some(construction);assert!(envelope.retirement_ready());assert!(envelope.retirement_birth_bytes().is_some());assert_eq!(envelope.context.as_ref().unwrap().message.as_ptr(),context_pointer);assert!(vault.terminal_is_empty());
  }
 }
}
