//! 🧪️ Genuine shared allocations preserve input and actual attempt work.
use super::*;
use semio_framework_value_testing_allocator::{observe_retirement_allocations as observe,refuse_one_allocation_on_this_thread as refuse};

#[test]
fn shared_admission_denies_each_missing_axis_before_allocating_or_moving_input(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let extent=shared_allocation_bytes::<String>();
 let issued=RetainedCloneGrant::one_capacity_turn(extent,1);
 let input=law["text"].as_str().unwrap().to_owned();let pointer=input.as_ptr();let mut occupied=Some(law["occupiedMarker"].as_u64().unwrap()as usize);let(turn,heap)=observe(||admit_shared_allocation(input,issued,&mut occupied));assert_eq!(turn.work,0);assert_eq!(heap,(0,0));assert_eq!(occupied,Some(law["occupiedMarker"].as_u64().unwrap()as usize));let(_,input)=turn.result.unwrap_err();assert_eq!(input.as_ptr(),pointer);
 for grant in [RetainedCloneGrant{maximum_items:0,..issued},RetainedCloneGrant{maximum_depth:0,..issued},RetainedCloneGrant{maximum_capacity_bytes:extent-1,..issued}]{
  let input=law["text"].as_str().unwrap().to_owned();let pointer=input.as_ptr();
  let mut actual=None;let(turn,heap)=observe(||admit_shared_allocation(input,grant,&mut actual));assert_eq!(actual,law["deniedActual"].as_u64().map(|v|v as usize));
  assert_eq!(turn.work,law["deniedWork"].as_u64().unwrap());assert_eq!(heap,(0,0));
  let(_,input)=turn.result.unwrap_err();assert_eq!(input.as_ptr(),pointer);assert_eq!(serde_json::to_value(&input).unwrap(),law["text"]);
 }
}

#[test]
fn shared_admission_reports_real_null_and_returns_the_same_owned_input(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let input=law["text"].as_str().unwrap().to_owned();let pointer=input.as_ptr();
 let grant=RetainedCloneGrant::one_capacity_turn(shared_allocation_bytes::<String>(),1);
 let mut actual=None;let((turn,attempts),heap)=observe(||refuse(||admit_shared_allocation(input,grant,&mut actual)));assert_eq!(actual,Some(law["failedActual"].as_u64().unwrap()as usize));
 assert_eq!(attempts,1);assert_eq!(heap,(0,0));assert_eq!(turn.work,law["attemptWork"].as_u64().unwrap());
 let(error,input)=turn.result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::AllocationFailed);assert_eq!(input.as_ptr(),pointer);
 assert_eq!(serde_json::to_value(&input).unwrap(),law["text"]);
 let mut stock=law["stock"].as_u64().unwrap()-turn.work;
 actual=None;let(turn,heap)=observe(||admit_shared_allocation(input,grant,&mut actual));assert_eq!(actual,Some(shared_allocation_bytes::<String>()));assert!(turn.work<=stock);stock-=turn.work;assert_eq!(stock,0);
 assert_eq!(heap,(shared_allocation_bytes::<String>(),0));let(owner,progress)=turn.result.unwrap();
 assert_eq!(owner.as_ptr(),pointer);assert_eq!(serde_json::to_value(&*owner).unwrap(),law["text"]);assert!(progress.fits(grant));assert_eq!(progress.retained_capacity_bytes,heap.0);assert_eq!(Arc::strong_count(&owner),1);
 let input=Arc::try_unwrap(owner).unwrap();assert_eq!(input.as_ptr(),pointer);
}

#[test]
fn shared_admission_uses_the_real_layout_for_aligned_and_zero_sized_payloads(){
 #[repr(align(128))]#[derive(Debug)]struct Aligned(u64);
 let grant=RetainedCloneGrant::one_capacity_turn(shared_allocation_bytes::<Aligned>(),1);
 let mut actual=None;let(turn,heap)=observe(||admit_shared_allocation(Aligned(42),grant,&mut actual));assert_eq!(actual,Some(shared_allocation_bytes::<Aligned>()));assert_eq!(turn.work,1);let(owner,progress)=turn.result.unwrap();
 assert_eq!(owner.0,42);assert_eq!((Arc::as_ptr(&owner)as usize)%128,0);assert_eq!(heap,(progress.retained_capacity_bytes,0));assert_eq!(heap.0,shared_allocation_bytes::<Aligned>());drop(owner);
 let grant=RetainedCloneGrant::one_capacity_turn(shared_allocation_bytes::<()>(),1);
 actual=None;let(turn,heap)=observe(||admit_shared_allocation((),grant,&mut actual));assert_eq!(actual,Some(shared_allocation_bytes::<()>()));assert_eq!(turn.work,1);let(owner,progress)=turn.result.unwrap();assert_eq!(heap,(progress.retained_capacity_bytes,0));assert!(heap.0>0);drop(owner);
}
