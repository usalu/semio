//! 🧪️ General prepaid failure attachment and unused-node cleanup conserve actual physical ownership.
pub use semio_framework_value_retirement::*;
use semio_framework_value_testing_allocator as allocation_testing;
pub(crate) use allocation_testing::observe_retirement_allocations;
use semio_framework_value_retirement::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
fn error(text:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,text.to_owned())}
fn drain<T:RetireOwned>(value:T)->(usize,usize){
 let(mut owner,heap)=observe(||ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));let(mut allocated,mut released)=(0,0);
 for _ in 0..4096{
  if owner.terminal_is_empty(){return(allocated,released)}
  let d=owner.next_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  let(step,heap)=observe(||owner.step(g).unwrap());let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;released+=heap.1;
 }panic!("finite paid failure drain")
}
#[test]
fn original_failure_storage_parts_preserve_every_original_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("parts.json")).unwrap();let leaf=&fixture["leaf"];let parent=&fixture["parent"];
 let values=[error(leaf["primary"].as_str().unwrap()),error(leaf["work"].as_str().unwrap()),error(leaf["receipt"].as_str().unwrap()),error(parent["primary"].as_str().unwrap()),error(parent["work"].as_str().unwrap()),error(parent["receipt"].as_str().unwrap())];let pointers=values.each_ref().map(|error|error.message.as_ptr());let prose:usize=values.iter().map(|error|error.message.len()).sum();
 let[a,b,c,d,e,f]=values;let(child,heap)=observe(||OriginalRetirementFailure::from_causes(Some(a),Some(b),Some(c)));assert_eq!(heap,(0,0));
 let bytes=OriginalFailureStorage::birth_bytes();let(admission,heap)=observe(||OriginalFailureStorage::admit(bytes));assert_eq!(heap,(bytes,0));let(storage,receipt)=admission.result.unwrap();assert_eq!(receipt.retained_capacity_bytes,bytes);
 let(failure,heap)=observe(||storage.attach_owned_parts(child,Some(d),Some(e),Some(f)));assert_eq!(heap,(fixture["attachAllocatedBytes"].as_u64().unwrap()as usize,fixture["attachReleasedBytes"].as_u64().unwrap()as usize));
 let child=failure.child().unwrap();assert_eq!([child.primary().unwrap().message.as_ptr(),child.work_refusal().unwrap().message.as_ptr(),child.receipt_refusal().unwrap().message.as_ptr(),failure.primary().unwrap().message.as_ptr(),failure.work_refusal().unwrap().message.as_ptr(),failure.receipt_refusal().unwrap().message.as_ptr()],pointers);
 let(a,r)=drain(failure);assert_eq!(r,a+bytes+prose);
}
#[test]
fn original_unused_failure_storage_is_a_paid_retained_owner(){
 let bytes=OriginalFailureStorage::birth_bytes();let(admission,heap)=observe(||OriginalFailureStorage::admit(bytes));assert_eq!(heap,(bytes,0));let(storage,_)=admission.result.unwrap();
 let(mut owner,heap)=observe(||ControlledRetirement::new(storage).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));let(mut allocated,mut released)=(0,0);let mut refused=false;
 for _ in 0..4096{
  if owner.terminal_is_empty(){assert!(refused);assert_eq!(released,allocated+bytes);return}
  let d=owner.next_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  if d.release_bytes==bytes&&!refused{let(step,heap)=observe(||owner.step(RetainedCloneGrant{maximum_release_bytes:bytes-1,..g}).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(owner.last_turn_work(),0);assert_eq!(owner.next_demand(4096).unwrap(),d);refused=true;}
  let(step,heap)=observe(||owner.step(g).unwrap());let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;released+=heap.1;
 }panic!("finite paid spare-node drain")
}

#[test]
fn original_failure_own_progress_override_never_rewrites_nested_causes(){
 let a=RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()};let b=RetainedCloneProgress{copied_items:2,copied_bytes:7,..Default::default()};let c=RetainedCloneProgress{copied_items:4,copied_bytes:11,..Default::default()};let error=|message,progress|ValueError::literal(ValueRefusalKind::InvariantViolated,message).with_retained_progress(progress);
 let child=OriginalRetirementFailure::from_primary(error("original child",a));let(storage,_)=OriginalFailureStorage::admit(OriginalFailureStorage::birth_bytes()).result.unwrap();let parent=storage.attach_owned_parts(child,Some(error("original parent",b)),None,None).with_retained_progress(c);assert_eq!(parent.retained_progress(),c);assert_eq!(parent.child().unwrap().retained_progress(),a);assert_eq!(parent.primary().unwrap().retained_progress(),b);assert_eq!(parent.child().unwrap().primary().unwrap().retained_progress(),a);
 let parent=parent.with_work_refusal(error("later work",b)).unwrap();assert_eq!(parent.retained_progress(),c);assert_eq!(parent.work_refusal().unwrap().retained_progress(),b);let(primary,work,receipt,child,over)=parent.into_parts();assert_eq!(over,Some(c));let restored=OriginalRetirementFailure::from_owned_parts(primary,work,receipt,child,over);assert_eq!(restored.retained_progress(),c);assert_eq!(restored.child().unwrap().retained_progress(),a);drain(restored);
 let only_receipt=OriginalRetirementFailure::from_causes(None,None,Some(error("receipt",a)));let with_work=only_receipt.with_work_refusal(error("work",b)).unwrap();assert_eq!(with_work.retained_progress(),b);assert_eq!(with_work.receipt_refusal().unwrap().retained_progress(),a);drain(with_work);let overridden=OriginalRetirementFailure::from_causes(None,None,Some(error("receipt with enclosing span",a))).with_retained_progress(c);let overridden=overridden.with_work_refusal(error("later work",b)).unwrap();assert_eq!(overridden.retained_progress(),c);assert_eq!(overridden.receipt_refusal().unwrap().retained_progress(),a);assert_eq!(overridden.work_refusal().unwrap().retained_progress(),b);drain(overridden);
}

#[path = "🦀️.rs"]
mod failure_storage_tests;
