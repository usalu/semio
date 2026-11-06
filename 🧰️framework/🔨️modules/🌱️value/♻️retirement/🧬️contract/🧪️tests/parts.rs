//! 🧪️ General prepaid failure attachment and unused-node cleanup conserve actual physical ownership.
pub use semio_framework_value_retirement::*;
use semio_framework_value_testing_allocator as allocation_testing;
pub(crate) use allocation_testing::observe_retirement_allocations;
use semio_framework_value_retirement::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;

fn admit_node(capacity:usize)->RetainedWorkOutcome<(OriginalFailureStorage,RetainedCloneProgress)>{let mut actual=None;let turn=OriginalFailureStorage::admit(capacity,&mut actual);assert_eq!(turn.work,u64::from(actual.is_some()));match &turn.result{Ok((_,progress))=>assert_eq!(actual,Some(progress.retained_capacity_bytes)),Err(_)=>assert!(actual.is_none()||actual==Some(0))}turn}
fn observed_step<T:RetireOwned>(owner:&mut ControlledRetirement<T>,grant:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize)){let work=owner.next_computational_work(grant.maximum_copy_bytes).unwrap();let mut actual=None;let mut effect=RetainedAllocationEffect::new(grant,work,&mut actual);let(step,heap)=observe(||owner.step(&mut effect).unwrap());assert_eq!(actual.unwrap_or(0),heap.0);assert_eq!(actual.is_some(),heap.0!=0);(step,heap)}
fn error(text:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,text.to_owned())}
fn drain<T:RetireOwned>(value:T)->(usize,usize){
 let(mut owner,heap)=observe(||ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));let(mut allocated,mut released)=(0,0);
 for _ in 0..4096{
  if owner.terminal_is_empty(){return(allocated,released)}
  let d=owner.next_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  let(step,heap)=observed_step(&mut owner,g);let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;released+=heap.1;
 }panic!("finite paid failure drain")
}
#[test]
fn original_failure_storage_parts_preserve_every_original_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("parts.json")).unwrap();let leaf=&fixture["leaf"];let parent=&fixture["parent"];
 let values=[error(leaf["primary"].as_str().unwrap()),error(leaf["work"].as_str().unwrap()),error(leaf["receipt"].as_str().unwrap()),error(parent["primary"].as_str().unwrap()),error(parent["work"].as_str().unwrap()),error(parent["receipt"].as_str().unwrap())];let pointers=values.each_ref().map(|error|error.message.as_ptr());let prose:usize=values.iter().map(|error|error.message.len()).sum();
 let[a,b,c,d,e,f]=values;let(child,heap)=observe(||OriginalRetirementFailure::from_causes(Some(a),Some(b),Some(c),None));assert_eq!(heap,(0,0));
 let bytes=OriginalFailureStorage::birth_bytes();let(admission,heap)=observe(||admit_node(bytes));assert_eq!(heap,(bytes,0));let(storage,receipt)=admission.result.unwrap();assert_eq!(receipt.retained_capacity_bytes,bytes);
 let(failure,heap)=observe(||storage.attach_owned_parts(child,Some(d),Some(e),Some(f),None));assert_eq!(heap,(fixture["attachAllocatedBytes"].as_u64().unwrap()as usize,fixture["attachReleasedBytes"].as_u64().unwrap()as usize));
 let child=failure.child().unwrap();assert_eq!([child.primary().unwrap().message.as_ptr(),child.work_refusal().unwrap().message.as_ptr(),child.receipt_refusal().unwrap().message.as_ptr(),failure.primary().unwrap().message.as_ptr(),failure.work_refusal().unwrap().message.as_ptr(),failure.receipt_refusal().unwrap().message.as_ptr()],pointers);
 let(a,r)=drain(failure);assert_eq!(r,a+bytes+prose);
}
#[test]
fn original_unused_failure_storage_is_a_paid_retained_owner(){
 let bytes=OriginalFailureStorage::birth_bytes();let(admission,heap)=observe(||admit_node(bytes));assert_eq!(heap,(bytes,0));let(storage,_)=admission.result.unwrap();
 let(mut owner,heap)=observe(||ControlledRetirement::new(storage).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));let(mut allocated,mut released)=(0,0);let mut refused=false;
 for _ in 0..4096{
  if owner.terminal_is_empty(){assert!(refused);assert_eq!(released,allocated+bytes);return}
  let d=owner.next_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  if d.release_bytes==bytes&&!refused{let(step,heap)=observed_step(&mut owner,RetainedCloneGrant{maximum_release_bytes:bytes-1,..g});assert_eq!(heap,(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(owner.last_turn_work(),0);assert_eq!(owner.next_demand(4096).unwrap(),d);refused=true;}
  let(step,heap)=observed_step(&mut owner,g);let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));allocated+=heap.0;released+=heap.1;
 }panic!("finite paid spare-node drain")
}

#[test]
fn original_failure_own_progress_override_never_rewrites_nested_causes(){
 let a=RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()};let b=RetainedCloneProgress{copied_items:2,copied_bytes:7,..Default::default()};let c=RetainedCloneProgress{copied_items:4,copied_bytes:11,..Default::default()};let error=|message,progress|ValueError::literal(ValueRefusalKind::InvariantViolated,message).with_retained_progress(progress);
 let child=OriginalRetirementFailure::from_primary(error("original child",a));let(storage,_)=admit_node(OriginalFailureStorage::birth_bytes()).result.unwrap();let parent=storage.attach_owned_parts(child,Some(error("original parent",b)),None,None,None).with_retained_progress(c);assert_eq!(parent.retained_progress(),c);assert_eq!(parent.child().unwrap().retained_progress(),a);assert_eq!(parent.primary().unwrap().retained_progress(),b);assert_eq!(parent.child().unwrap().primary().unwrap().retained_progress(),a);
 let parent=parent.with_work_refusal(error("later work",b)).unwrap();assert_eq!(parent.retained_progress(),c);assert_eq!(parent.work_refusal().unwrap().retained_progress(),b);let(primary,work,receipt,allocation,child,over)=parent.into_parts();assert_eq!(over,Some(c));let restored=OriginalRetirementFailure::from_owned_parts(primary,work,receipt,allocation,child,over);assert_eq!(restored.retained_progress(),c);assert_eq!(restored.child().unwrap().retained_progress(),a);drain(restored);
 let only_receipt=OriginalRetirementFailure::from_causes(None,None,Some(error("receipt",a)),None);let with_work=only_receipt.with_work_refusal(error("work",b)).unwrap();assert_eq!(with_work.retained_progress(),b);assert_eq!(with_work.receipt_refusal().unwrap().retained_progress(),a);drain(with_work);let overridden=OriginalRetirementFailure::from_causes(None,None,Some(error("receipt with enclosing span",a)),None).with_retained_progress(c);let overridden=overridden.with_work_refusal(error("later work",b)).unwrap();assert_eq!(overridden.retained_progress(),c);assert_eq!(overridden.receipt_refusal().unwrap().retained_progress(),a);assert_eq!(overridden.work_refusal().unwrap().retained_progress(),b);drain(overridden);
}


#[test]
fn failure_cursor_denial_and_actual_null_keep_the_same_original_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("parts.json")).unwrap();
 for row in fixture["cursorBirth"].as_array().unwrap(){
  let primary=error("original failure cursor source");let pointer=primary.message.as_ptr();let mut original=Some(OriginalRetirementFailure::from_primary(primary));let demand=semio_framework_value_retirement::retirement_cursor_birth_demand(original.as_ref().unwrap()).unwrap();let bytes=std::mem::size_of::<<OriginalRetirementFailure as RetireOwned>::RetirementCursor>();assert_eq!(demand.capacity_bytes,bytes);let capacity=if row["capacity"]=="short"{bytes.saturating_sub(1)}else{bytes};
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:if row["copy"]=="short"{demand.copy_bytes.saturating_sub(1)}else{demand.copy_bytes},maximum_capacity_bytes:capacity,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut actual=None;let mut effect=RetainedAllocationEffect::new(grant,row["maximumWork"].as_u64().unwrap(),&mut actual);
  let((turn,attempts),heap)=if row["allocator"]=="null"{observe(||allocation_testing::refuse_one_allocation_on_this_thread(||semio_framework_value_retirement::retirement::admit_retirement_cursor(&mut original,&mut effect)))}else{let(turn,heap)=observe(||semio_framework_value_retirement::retirement::admit_retirement_cursor(&mut original,&mut effect));((turn,0),heap)};
  assert_eq!(heap,(0,0));assert_eq!(attempts,row["attempts"].as_u64().unwrap()as usize);assert_eq!(turn.work,row["reportedWork"].as_u64().unwrap());assert_eq!(actual.is_some(),row["marker"].as_bool().unwrap());if actual.is_some(){assert_eq!(actual,Some(0))}
  if row["allocator"]=="null"{match turn.result{Err(error)=>assert_eq!(error.kind,ValueRefusalKind::AllocationFailed),Ok(_)=>panic!("null allocator unexpectedly published a retirement cursor")}}else{assert!(turn.result.unwrap().is_none())}
  assert_eq!(original.is_some(),row["sourceRetained"].as_bool().unwrap());assert_eq!(original.as_ref().unwrap().primary().unwrap().message.as_ptr(),pointer);drain(original.take().unwrap());
 }
}

#[test]
fn spare_failure_seed_allocates_nothing_and_its_release_requires_real_work(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("parts.json")).unwrap();let law=&fixture["spareSeed"];let bytes=OriginalFailureStorage::birth_bytes();let(storage,_)=admit_node(bytes).result.unwrap();let(mut cursor,heap)=observe(||storage.retirement_seed());assert_eq!(heap,(law["allocatedBytes"].as_u64().unwrap()as usize,0));assert!(!semio_framework_value_retirement::retirement::RetirementCursor::terminal_is_empty(&cursor));
 let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()};let mut actual=None;let mut effect=RetainedAllocationEffect::new(grant,law["deniedWork"].as_u64().unwrap(),&mut actual);let(step,heap)=observe(||semio_framework_value_retirement::retirement::RetirementCursor::close_step(&mut cursor,&mut effect));assert!(matches!(step,semio_framework_value_retirement::retirement::RetirementStep::BudgetExhausted));assert_eq!(heap,(0,0));assert_eq!(actual,None);assert!(!semio_framework_value_retirement::retirement::RetirementCursor::terminal_is_empty(&cursor));
 let mut actual=None;let mut effect=RetainedAllocationEffect::new(grant,law["releaseWork"].as_u64().unwrap(),&mut actual);let(step,heap)=observe(||semio_framework_value_retirement::retirement::RetirementCursor::close_step(&mut cursor,&mut effect));match step{semio_framework_value_retirement::retirement::RetirementStep::Progress(progress,work)=>{assert_eq!(work,law["releaseWork"].as_u64().unwrap());assert_eq!(progress.released_bytes,bytes)},_=>panic!("funded spare-node release did not complete its actual action")}assert_eq!(heap,(0,bytes));assert_eq!(actual,None);assert!(semio_framework_value_retirement::retirement::RetirementCursor::terminal_is_empty(&cursor));
}

#[test]
fn native_allocation_failure_seed_retires_all_twelve_original_error_allocations(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("parts.json")).unwrap();let names=fixture["allocationRetirementCauses"].as_array().unwrap();assert_eq!(names.len(),12);let values:[ValueError;12]=std::array::from_fn(|index|error(names[index].as_str().unwrap()));let prose:usize=values.iter().map(|value|match &value.message{std::borrow::Cow::Owned(text)=>text.capacity(),std::borrow::Cow::Borrowed(_)=>0}).sum();let[a,b,c,d,e,f,g,h,i,j,k,l]=values;
 let failure=NativeAllocationFailure{primary:Some(a),admission:Some(b),forwarded_admission:Some(c.into()),scoped_admission:Some(d.into()),forwarded_quote:Some(e.into()),scoped_quote:Some(f.into()),forwarded_post:Some(g.into()),scoped_post:Some(h.into()),forwarded_cancel:Some(i.into()),scoped_cancel:Some(j.into()),receipt:Some(k),extent:Some(l),..Default::default()};let(allocated,released)=drain(failure);assert_eq!(released,allocated+prose);
}
#[path = "🦀️.rs"]
mod failure_storage_tests;
