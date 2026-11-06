//! 🧪️ Genuine shared allocations preserve input and actual attempt work.
use super::*;
use semio_framework_value_testing_allocator::{observe_retirement_allocations as observe,refuse_one_allocation_on_this_thread as refuse};

#[test]
fn shared_admission_denies_each_missing_axis_before_allocating_or_moving_input(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let extent=shared_allocation_bytes::<String>();
 let issued=RetainedCloneGrant{maximum_copy_bytes:shared_allocation_value_demand::<String>().unwrap().copy_bytes,..RetainedCloneGrant::one_capacity_turn(extent,1)};
 let input=law["text"].as_str().unwrap().to_owned();let pointer=input.as_ptr();let mut occupied=Some(law["occupiedMarker"].as_u64().unwrap()as usize);let(turn,heap)=observe(||admit_shared_allocation(input,issued,&mut occupied));assert_eq!(turn.work,0);assert_eq!(heap,(0,0));assert_eq!(occupied,Some(law["occupiedMarker"].as_u64().unwrap()as usize));let(_,input)=turn.result.unwrap_err();assert_eq!(input.as_ptr(),pointer);
 for grant in [RetainedCloneGrant{maximum_items:0,..issued},RetainedCloneGrant{maximum_copy_bytes:issued.maximum_copy_bytes-1,..issued},RetainedCloneGrant{maximum_depth:0,..issued},RetainedCloneGrant{maximum_capacity_bytes:extent-1,..issued}]{
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
 let grant=RetainedCloneGrant{maximum_copy_bytes:shared_allocation_value_demand::<String>().unwrap().copy_bytes,..RetainedCloneGrant::one_capacity_turn(shared_allocation_bytes::<String>(),1)};
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
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 #[repr(align(128))]#[derive(Debug)]struct Aligned(u64);
 let grant=RetainedCloneGrant{maximum_copy_bytes:shared_allocation_value_demand::<Aligned>().unwrap().copy_bytes,..RetainedCloneGrant::one_capacity_turn(shared_allocation_bytes::<Aligned>(),1)};
 let mut actual=None;let(turn,heap)=observe(||admit_shared_allocation(Aligned(42),grant,&mut actual));assert_eq!(actual,Some(shared_allocation_bytes::<Aligned>()));assert_eq!(turn.work,law["successWork"].as_u64().unwrap());let(owner,progress)=turn.result.unwrap();
 assert_eq!(owner.0,42);assert_eq!((Arc::as_ptr(&owner)as usize)%128,0);assert_eq!(heap,(progress.retained_capacity_bytes,0));assert_eq!(heap.0,shared_allocation_bytes::<Aligned>());drop(owner);
 let grant=RetainedCloneGrant{maximum_copy_bytes:shared_allocation_value_demand::<()>().unwrap().copy_bytes,..RetainedCloneGrant::one_capacity_turn(shared_allocation_bytes::<()>(),1)};
 actual=None;let(turn,heap)=observe(||admit_shared_allocation((),grant,&mut actual));assert_eq!(actual,Some(shared_allocation_bytes::<()>()));assert_eq!(turn.work,law["successWork"].as_u64().unwrap());let(owner,progress)=turn.result.unwrap();assert_eq!(heap,(progress.retained_capacity_bytes,0));assert!(heap.0>0);drop(owner);
}

#[test]
fn shared_source_slot_birth_matches_neutral_custody_and_actual_allocator() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🌱️birth/📐️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {
  let axis=row["axis"].as_str().unwrap();let mut source=Some(String::from("same source"));let pointer=source.as_ref().unwrap().as_ptr();let mut target=None;let mut actual=None;let d=shared_allocation_slot_demand::<String>().unwrap();
  let mut g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:0,maximum_depth:d.depth};
  match axis{"items"=>g.maximum_items=0,"copy"=>g.maximum_copy_bytes-=1,"capacity"=>g.maximum_capacity_bytes-=1,"depth"=>g.maximum_depth-=1,_=>{}}
  let(turn,heap)=observe(||if axis=="allocator"{let(turn,attempts)=refuse(||admit_shared_allocation_from(&mut source,&mut target,g,&mut actual));assert_eq!(attempts,1);turn}else{admit_shared_allocation_from(&mut source,&mut target,g,&mut actual)});
  assert_eq!(source.is_some(),row["retained"].as_bool().unwrap());assert_eq!(target.is_some(),!row["retained"].as_bool().unwrap());assert_eq!(match actual{None=>"none",Some(0)=>"zero",Some(_)=>"extent"},row["marker"].as_str().unwrap());
  if let Some(source)=source{assert_eq!(source.as_ptr(),pointer);assert_eq!(heap,(0,0));if axis=="allocator"{assert!(turn.result.is_err());assert_eq!(turn.work,1);}else{assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(turn.work,0);}}
  else{let p=turn.result.unwrap().progress();assert_eq!(turn.work,4);assert_eq!(heap,(d.capacity_bytes,0));assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!(p.retained_capacity_bytes,actual.unwrap());assert_eq!(target.as_ref().unwrap().as_ptr(),pointer);}
 }
}
