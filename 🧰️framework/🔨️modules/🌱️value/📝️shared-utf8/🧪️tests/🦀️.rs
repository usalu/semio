use super::*;
use crate::{value::observe_retirement_allocations as observe,retirement::controlled::ControlledRetirement};

#[test]
fn shared_utf8_original_lease_close_preserves_actor_bytes_without_allocating_retirement_frames(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../♻️original-lease/🧫️fixtures/🔣️.json")).unwrap();
 for text in law["values"].as_array().unwrap(){for capacity in law["capacities"].as_array().unwrap(){for aliases in law["aliases"].as_array().unwrap(){
  let mut original=String::with_capacity(capacity.as_u64().unwrap()as usize);original.push_str(text.as_str().unwrap());let pointer=original.as_ptr();
  let owner=SharedUtf8::from(original);let mut leases=Vec::new();for _ in 0..aliases.as_u64().unwrap(){leases.push(owner.clone());}leases.push(owner);
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:1};
  let mut returned=0;let total=capacity.as_u64().unwrap()as usize+shared_retirement_allocation_bytes::<String>();
  for mut owner in leases{
   for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_release_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
    let(result,heap)=observe(||owner.close_original_lease(denied));owner=result.err().unwrap().1;assert_eq!(owner.as_ptr(),pointer);assert_eq!(heap,(0,0));
   }
   let(result,heap)=observe(||owner.close_original_lease(grant));let(original,receipt)=result.map_err(|(error,_)|error).unwrap();assert!(receipt.fits(grant));assert_eq!(heap,(0,receipt.released_bytes));returned+=heap.1;
   if let Some(original)=original{assert_eq!(original.as_ptr(),pointer);assert_eq!(original, text.as_str().unwrap());let(_,heap)=observe(||drop(original));returned+=heap.1;}
  }
  assert_eq!(returned,total);
 }}}
 eprintln!("[DEBUG] shared UTF8 original actor leases: every undergrant preserves source pointer and zeroheap; final lease transfers original String with zero frame allocation; complete heap release exact");
}

#[test]
fn shared_utf8_original_giant_actor_admission_and_all_lease_frontiers_pay_physical_ownership(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for value in law["values"].as_array().unwrap(){for repeats in law["repeats"].as_array().unwrap(){for aliases in 0..3{
  let(mut original,birth)=observe(||value.as_str().unwrap().repeat(repeats.as_u64().unwrap()as usize));let pointer=original.as_ptr();let capacity=original.capacity();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe(||SharedUtf8::admit(original,denied));original=result.err().unwrap().1;assert_eq!(original.as_ptr(),pointer);assert_eq!(heap,(0,0));}
  let(result,heap)=observe(||SharedUtf8::admit(original,grant));let(owner,receipt)=result.map_err(|(error,_)|error).unwrap();assert!(receipt.fits(grant));assert_eq!(owner.as_ptr(),pointer);assert_eq!(heap,(receipt.retained_capacity_bytes,0));let mut births=heap.0;let mut released=0;
  assert_eq!(serde_json::to_string(&owner).unwrap(),serde_json::to_string(&owner.as_str()).unwrap());let mut owners=Vec::new();
  for _ in 0..aliases{let(result,heap)=observe(||owner.admit_clone(RetainedCloneGrant{maximum_copy_bytes:0,..grant}));assert!(result.is_err());assert_eq!(heap,(0,0));let(result,heap)=observe(||owner.admit_clone(grant));let(lease,receipt)=result.unwrap();assert!(receipt.fits(grant));assert_eq!(heap,(0,0));assert_eq!(lease.as_ptr(),pointer);owners.push(lease);}
  owners.push(owner);
  for owner in owners{assert_eq!(owner.as_ptr(),pointer);let mut cursor=ControlledRetirement::new(owner).unwrap_or_else(|_|panic!("original shared UTF8 retirement"));for _ in 0..100000{if cursor.terminal_is_empty(){break;}let(_,heap)=observe(||cursor.next_copy_byte_demand().unwrap());assert_eq!(heap,(0,0));let(step,heap)=observe(||cursor.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let(step,heap)=observe(||cursor.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.0;released+=heap.1;}assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));}
  assert_eq!(released,birth.0+births);assert!(released>=capacity);
 }}}
 eprintln!("[DEBUG] shared UTF8 original giant actor byte pointers unchanged; fixed4096 admitted frame and leases conserve actual heap receipts and finalDrop0");
}
