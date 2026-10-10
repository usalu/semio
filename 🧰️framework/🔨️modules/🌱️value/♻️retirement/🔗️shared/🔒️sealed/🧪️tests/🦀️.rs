use super::*;
use crate::value::observe_retirement_allocations as observe;
use crate::retirement::controlled::ControlledRetirement;

#[test]
fn sealed_shared_original_aliases_preserve_exact_payload_and_funded_header(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let original=row["text"].as_str().unwrap().to_owned();let original_pointer=original.as_ptr();let original_bytes=original.capacity();
  let birth=SealedShared::<String>::birth_bytes();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:birth,maximum_release_bytes:0,maximum_depth:1};
  for axis in [0,2,4]{let mut denied=grant;match axis{0=>denied.maximum_items=0,2=>denied.maximum_capacity_bytes=birth-1,_=>denied.maximum_depth=0};let candidate=original.clone();let pointer=candidate.as_ptr();let (refused,heap)=observe(||SealedShared::admit(candidate,SharedIssuer::owned(),denied));let(error,value)=refused.err().unwrap();assert_eq!(error.kind,if axis==0{ValueRefusalKind::WorkLimit}else if axis==4{ValueRefusalKind::DepthLimit}else{ValueRefusalKind::OwnershipLimit});assert_eq!(heap,(0,0));assert_eq!(value.as_ptr(),pointer);drop(value);}
  let ((source,receipt),heap)=observe(||SealedShared::admit(original,SharedIssuer::owned(),grant).unwrap_or_else(|_|panic!("original birth refused")));assert_eq!(heap,(birth,0));assert_eq!(receipt.retained_capacity_bytes,birth);assert_eq!(source.get().as_ptr(),original_pointer);
  let identity=source.identity();let mut handles=Vec::new();let mut aliases=row["aliases"].as_u64().unwrap() as usize;
  while aliases>1{let ((alias,receipt),heap)=observe(||source.try_duplicate(grant).unwrap());assert_eq!(heap,(0,0));assert!(receipt.fits(grant));assert_eq!(alias.identity(),identity);handles.push(alias);aliases-=1;}handles.push(source);
  let mut payload_owners=0;let mut headers=0;let mut terminals=0;let mut cursor_births=0;let mut cursor_releases=0;
  for handle in handles{
   let mut cursor=ControlledRetirement::new(handle).unwrap_or_else(|_|panic!("cursor refused original handle"));
   for _ in 0..256{
    if cursor.terminal_is_empty(){break;}
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:cursor.next_capacity_byte_demand(4096).unwrap(),maximum_release_bytes:cursor.next_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap()};
    for axis in 0..5{let mut denied=grant;let positive=match axis{0=>{denied.maximum_items=0;true},1=>{let demand=cursor.next_copy_byte_demand().unwrap();denied.maximum_copy_bytes=demand.saturating_sub(1);demand>0},2=>{denied.maximum_capacity_bytes=grant.maximum_capacity_bytes.saturating_sub(1);grant.maximum_capacity_bytes>0},3=>{denied.maximum_release_bytes=grant.maximum_release_bytes.saturating_sub(1);grant.maximum_release_bytes>0},_=>{denied.maximum_depth=grant.maximum_depth.saturating_sub(1);grant.maximum_depth>0}};if positive{let(step,heap)=observe(||cursor.step(denied));assert_eq!(heap,(0,0));if let Ok(step)=step{assert_eq!(step.progress(),Default::default());}}}
    let(step,heap)=observe(||cursor.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));cursor_births+=heap.0;cursor_releases+=heap.1;if receipt.released_bytes==birth{headers+=1;payload_owners+=1;}
   }
   assert!(cursor.terminal_is_empty());terminals+=1;let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));
  }
  assert_eq!(cursor_releases,birth+original_bytes+cursor_births);
  assert_eq!(serde_json::json!({"originalPayloadOwners":payload_owners,"releasedHeaders":headers,"terminalAliases":terminals}),row["expected"]);
 }
 eprintln!("[DEBUG] sealed shared original aliases preserve exact payload and actual physical header under all granted currencies");
}

#[test]
fn sealed_shared_original_alias_return_slots_transfer_parallel_custody_without_heap(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let original=row["text"].as_str().unwrap().to_owned();let pointer=original.as_ptr();let bytes=original.capacity();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()};
  let (source,_)=SealedShared::admit(original,SharedIssuer::owned(),grant).unwrap_or_else(|_|panic!("real source admission failed"));
  let mut aliases=Vec::new();let mut slots=Vec::new();for _ in 1..row["aliases"].as_u64().unwrap(){aliases.push(source.try_duplicate(grant).unwrap().0);}aliases.push(source);
  for alias in &aliases{let((slot,receipt),heap)=observe(||alias.return_slot(grant).unwrap());assert_eq!(heap,(0,0));assert!(receipt.fits(grant));assert!(!slot.terminal_is_empty());slots.push(slot);}
  std::thread::scope(|scope|{for(alias,slot)in aliases.into_iter().zip(&slots){scope.spawn(move||{let(result,heap)=observe(||alias.return_alias(slot));assert!(result.is_ok());assert_eq!(heap,(0,0));});}});
  let mut births=0;let mut released=0;
  for slot in &slots{
   assert!(!slot.terminal_is_empty());for axis in [0,1,4]{let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=size_of::<SealedShared<String>>()-1,_=>denied.maximum_depth=0};let(result,heap)=observe(||slot.drain(denied));assert!(result.is_err());assert_eq!(heap,(0,0));assert!(!slot.terminal_is_empty());}
   let (returned,heap)=observe(||slot.drain(grant).unwrap());assert_eq!(heap,(0,0));let(alias,receipt)=returned.unwrap();assert!(receipt.fits(grant));assert_eq!(alias.get().as_ptr(),pointer);assert!(slot.terminal_is_empty());
   let mut cursor=ControlledRetirement::new(alias).unwrap_or_else(|_|panic!("returned original admission failed"));
   for _ in 0..256{if cursor.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:cursor.next_capacity_byte_demand(4096).unwrap(),maximum_release_bytes:cursor.next_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap()};let(step,heap)=observe(||cursor.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;released+=heap.1;}
   assert!(cursor.terminal_is_empty());
  }
  assert_eq!(released,SealedShared::<String>::birth_bytes()+bytes+births);
 }
 eprintln!("[DEBUG] parallel original alias handback uses zero heap and exact funded original issuer drain");
}
#[test]
fn sealed_shared_stale_slot_refuses_a_distinct_original_issuance_at_same_address(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()};
 let (first,_)=SealedShared::admit("original".to_owned(),SharedIssuer::owned(),grant).unwrap_or_else(|_|panic!("first original admission refused"));
 let (second,_)=SealedShared::admit("later".to_owned(),SharedIssuer::owned(),grant).unwrap_or_else(|_|panic!("later original admission refused"));
 let (mut slot,_)=first.return_slot(grant).unwrap();slot.identity=second.identity();
 let pointer=second.get().as_ptr();let issuance=second.issuance;let (result,heap)=observe(||second.return_alias(&slot));assert_eq!(heap,(0,0));let original=result.err().unwrap();assert_eq!(original.get().as_ptr(),pointer);assert_eq!(original.issuance,issuance);assert_eq!(slot.state.load(Ordering::Acquire),0);
 assert_eq!(serde_json::json!({"sameAddress":slot.identity==original.identity(),"sameIssuance":slot.issuance==original.issuance,"accepted":false}),corpus["staleSlot"]);
 slot.retire_unused(grant).unwrap();
 for original in [first,original]{let mut cursor=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("actual original handback refused"));for _ in 0..256{if cursor.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:cursor.next_capacity_byte_demand(4096).unwrap(),maximum_release_bytes:cursor.next_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap()};let(step,heap)=observe(||cursor.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(cursor.terminal_is_empty());}
 eprintln!("[DEBUG] stale original return slot rejects address reuse while preserving whole distinct original issuance");
}