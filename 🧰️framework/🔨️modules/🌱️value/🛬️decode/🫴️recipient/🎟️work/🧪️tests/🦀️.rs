use super::*;
#[test]
fn native_decoder_recipient_records_only_actual_close_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
 let mut recipient=NativeDecodeRetirementRecipient::new();
 recipient.reserve().unwrap();
 let mut observations=Vec::new();
 for (label,g) in [
  ("deniedItems",RetainedCloneGrant{maximum_items:0,..grant}),
  ("deniedDepth",RetainedCloneGrant{maximum_depth:0,..grant}),
  ("paidClear",grant),("cachedComplete",grant)
 ]{
  let turn=ErasedSnapshotRetirement::close_step(&mut recipient,g);
  let p=match turn.result{
   Ok(step)=>step.progress(),
   Err(error)=>{assert_eq!(error.primary().unwrap().kind,crate::ValueRefusalKind::DepthLimit);error.retained_progress()}
  };
  assert_eq!((p.copied_bytes,p.retained_capacity_bytes,p.released_bytes),(0,0,0));
  observations.push(serde_json::json!({"label":label,"work":turn.work,"copiedItems":p.copied_items,"reserved":recipient.is_reserved()}));
 }
 assert_eq!(serde_json::Value::Array(observations),law["expected"]);
 assert!(ErasedSnapshotRetirement::terminal_is_empty(&recipient));
}

struct OriginalFuel(u64);
impl crate::native_decoding::OriginalWorkRecipient for OriginalFuel{fn admits<'a>(&mut self,quote:crate::native_decoding::OriginalWorkQuote<'a>,observed:u64,additional:u64)->crate::native_decoding::OriginalWorkQuoteComplete<'a>{quote.complete((||{Ok(observed.checked_add(additional).is_some_and(|sum|sum<=self.0))})())}}
#[test]
fn paid_spare_is_not_pending_owner_and_drains_after_semantic_work(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["spareWork"];{let mut recipient=NativeDecodeRetirementRecipient::new();let mut observer=|_|true;let mut denied=crate::NativeDecodeControl::new_retained(&mut observer);denied.install_retirement_recipient(&mut recipient).unwrap();let mut fuel=OriginalFuel(0);let mut work_issuer=crate::native_decoding::OriginalWorkActionIssuer::new();let mut work_loan=work_issuer.loan(&mut fuel);denied.with_work_recipient(&mut work_loan,|same|->Result<(),ValueError>{let birth=semio_framework_value_retirement::original_failure_storage_birth_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()};let(turn,heap)=crate::observe_retirement_allocations(||same.with_retirement::<_,ValueError>(grant,|loan|{let funding:&mut dyn semio_framework_value_retirement::OriginalRetirementControl=loan;Ok(semio_framework_value_retirement::refill_original_failure_storage(funding,grant))}));let turn=turn?;assert_eq!(turn.work,law["denied"].as_u64().unwrap());assert_eq!(heap,(0,0));assert!(!same.has_retirement_owner());assert!(!same.has_retirement_failure_storage());Ok(())}).unwrap();}let mut recipient=NativeDecodeRetirementRecipient::new();let mut observer=|_|true;let mut control=crate::NativeDecodeControl::new_retained(&mut observer);control.install_retirement_recipient(&mut recipient).unwrap();let mut fuel=OriginalFuel(law["total"].as_u64().unwrap());let mut work_issuer=crate::native_decoding::OriginalWorkActionIssuer::new();let mut work_loan=work_issuer.loan(&mut fuel);
 control.with_work_recipient(&mut work_loan,|same|->Result<(),ValueError>{let birth=semio_framework_value_retirement::original_failure_storage_birth_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()};let turn=same.with_retirement::<_,ValueError>(grant,|loan|{let funding:&mut dyn semio_framework_value_retirement::OriginalRetirementControl=loan;Ok(semio_framework_value_retirement::refill_original_failure_storage(funding,grant))})?;assert_eq!(turn.work,law["birth"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress().retained_capacity_bytes,birth.capacity_bytes);assert!(!same.has_retirement_owner());assert!(same.has_retirement_failure_storage());let release=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:birth.capacity_bytes,maximum_depth:1,..Default::default()};let turn=same.close_retirement_recipient(release);assert_eq!(turn.work,law["release"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress().released_bytes,birth.capacity_bytes);assert!(!same.has_retirement_owner());assert!(!same.has_retirement_failure_storage());assert_eq!(same.actual_work(),law["total"].as_u64().unwrap());Ok(())}).unwrap();
}
