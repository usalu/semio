//! 🔐️ Original borrowed authority and projected clone bindings obey exact physical custody.
use crate::{retained_clone::{RetainedCloneSource,RetainedCloneBinding,RetainedCloneGrant},value::observe_retirement_allocations};

#[derive(crate::RetireOwned)]
struct Captured{owner:String,metadata:String}

#[test]
fn sealed_clone_source_original_projected_authority_survives_every_alias_until_funded_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔐️custody.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let(mut original,heap)=observe_retirement_allocations(||Captured{owner:row["owner"].as_str().unwrap().to_string(),metadata:row["metadata"].as_str().unwrap().to_string()});
  let original_bytes=heap.0;let owner_pointer=original.owner.as_ptr();let metadata_pointer=original.metadata.as_ptr();
  let demand=RetainedCloneSource::<String>::borrowed_constructor_demand::<Captured>();
  let birth=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<String>::borrowed_constructor_copy_bytes::<Captured>(),maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
  for denied in [RetainedCloneGrant{maximum_items:0,..birth},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..birth},RetainedCloneGrant{maximum_depth:0,..birth}]{
   let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_borrowed(original,|owner:&Captured|&owner.owner,denied));let(_,returned)=result.err().unwrap();original=returned;
   assert_eq!(heap,(0,0));assert_eq!(original.owner.as_ptr(),owner_pointer);assert_eq!(original.metadata.as_ptr(),metadata_pointer);
  }
  let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_borrowed(original,|owner:&Captured|&owner.owner,birth));
  let(mut source,receipt)=result.unwrap_or_else(|_|panic!("actual original issuer admission"));assert_eq!(heap,(receipt.retained_capacity_bytes,0));assert_eq!(source.borrow().get().as_ptr(),owner_pointer);
  let mut born=heap.0;let mut released=0;let mut binding=None;let copy=source.borrow().binding_copy_bytes();
  for denied in [RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy-1,maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..Default::default()}]{
   let(result,heap)=observe_retirement_allocations(||source.borrow().bind(&mut binding,denied));assert_eq!(heap,(0,0));assert_eq!(result.unwrap().unwrap(),Default::default());assert!(binding.is_none());
  }
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()};
  let(result,heap)=observe_retirement_allocations(||source.borrow().bind(&mut binding,grant));assert_eq!(heap,(0,0));let receipt=result.unwrap().unwrap();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,copy);
  let(result,heap)=observe_retirement_allocations(||source.project_owned(1,|owner|owner,grant));let(mut projection,p)=result.unwrap();assert_eq!(heap,(0,0));assert!(p.fits(grant));
  for _ in 0..10000{
   if source.terminal_is_empty(){break;}
   let copy=source.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};
   let(step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(source.terminal_is_empty());assert!(binding.is_some());assert!(released<=born);assert_eq!(projection.borrow().unwrap().get().as_ptr(),owner_pointer);assert_eq!(projection.borrow().unwrap().get().as_str(),row["owner"].as_str().unwrap());
  for _ in 0..10000{
   if binding.is_none(){break;}
   let copy=RetainedCloneBinding::copy_demand(&binding).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:RetainedCloneBinding::capacity_demand(&binding,copy).unwrap(),maximum_release_bytes:RetainedCloneBinding::release_demand(&binding).unwrap(),maximum_depth:RetainedCloneBinding::depth_demand(&binding).unwrap()};
   let(step,heap)=observe_retirement_allocations(||RetainedCloneBinding::close_one(&mut binding,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(binding.is_none());assert_eq!(projection.borrow().unwrap().get().as_ptr(),owner_pointer);
  for _ in 0..10000{if projection.terminal_is_empty(){break;}let copy=projection.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:projection.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:projection.next_close_release_byte_demand().unwrap(),maximum_depth:projection.next_close_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||projection.close_step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
  assert!(projection.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(projection)).1,(0,0));assert_eq!(released,original_bytes+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
  println!("[DEBUG] original projected clone authority original={original_bytes} born={born} released={released}");
 }
}
   
#[test]
fn sealed_clone_source_returns_same_original_workspace_after_all_projection_aliases_return(){
 use crate::retained_clone::RetainedCloneSourceTake;
 use crate::retirement::controlled::ControlledRetirement;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔐️custody.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let(original,heap)=observe_retirement_allocations(||Captured{owner:row["owner"].as_str().unwrap().into(),metadata:row["metadata"].as_str().unwrap().into()});let bytes=heap.0;let pointer=original.owner.as_ptr();let metadata=original.metadata.as_ptr();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<Captured>::constructor_copy_bytes(),maximum_capacity_bytes:RetainedCloneSource::<Captured>::constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};
  let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(original,(),grant));let(mut source,p)=result.unwrap_or_else(|_|panic!("actual workspace admission"));assert_eq!(heap,(p.retained_capacity_bytes,0));let(mut born,mut freed)=(heap.0,0);
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:source.borrow().binding_copy_bytes(),maximum_depth:1,..Default::default()};let(mut child,_)=source.project_owned(1,|root|&root.owner,grant).unwrap();
  loop{let copy=source.next_take_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_take_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_take_release_byte_demand().unwrap(),maximum_depth:source.next_take_depth_demand().unwrap()};let(result,heap)=observe_retirement_allocations(||source.take_authority(grant));match result{Ok(RetainedCloneSourceTake::Pending(p))=>{assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;},Err(error)=>{assert_eq!(heap,(0,0));assert_eq!(error.kind,crate::ValueRefusalKind::WorkLimit);break;},Ok(RetainedCloneSourceTake::Ready(_, _))=>panic!("live genuine projection must retain original workspace")}}
  assert_eq!(child.borrow().unwrap().get().as_ptr(),pointer);
  for _ in 0..10000{if child.terminal_is_empty(){break;}let copy=child.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:child.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:child.next_close_release_byte_demand().unwrap(),maximum_depth:child.next_close_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||child.close_step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
  assert!(child.terminal_is_empty());let copy=source.next_take_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_take_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_take_release_byte_demand().unwrap(),maximum_depth:source.next_take_depth_demand().unwrap()};
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant},RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe_retirement_allocations(||source.take_authority(denied));assert!(result.is_err());assert_eq!(heap,(0,0));assert!(!source.terminal_is_empty());}
  let(result,heap)=observe_retirement_allocations(||source.take_authority(grant).unwrap());let RetainedCloneSourceTake::Ready(original,p)=result else{panic!("all genuine aliases returned")};assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;assert_eq!(original.owner.as_ptr(),pointer);assert_eq!(original.metadata.as_ptr(),metadata);
  let mut close=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("typed original workspace close"));for _ in 0..10000{if close.terminal_is_empty(){break;}let copy=close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||close.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
  assert!(close.terminal_is_empty());assert_eq!(bytes+born,freed);assert_eq!(observe_retirement_allocations(||{drop(source);drop(child);drop(close);}).1,(0,0));println!("[DEBUG] same original workspace alias denial and recovery original={bytes} born={born} release={freed}");
 }
}     