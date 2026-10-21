//! 🔐️ Original borrowed authority and projected clone bindings obey exact physical custody.
use crate::{RetainedCloneSource,RetainedCloneBinding,RetainedCloneGrant,observe_retirement_allocations};

fn exact_grant(d:crate::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
struct Captured{owner:String,metadata:String}

#[test]
fn sealed_clone_source_original_projected_authority_survives_every_alias_until_funded_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔐️custody.json")).unwrap();
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
  let mut born=heap.0;let mut released=0;let mut binding=None;let copy=source.borrow().binding_copy_bytes();assert_eq!(copy,law["currency"]["metadataCopyBytes"].as_u64().unwrap()as usize);
  for denied in [RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..Default::default()}]{
   let(result,heap)=observe_retirement_allocations(||source.borrow().bind(&mut binding,denied));assert_eq!(heap,(0,0));assert_eq!(result.unwrap().unwrap(),Default::default());assert!(binding.is_none());
  }
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_depth:1,..Default::default()};
  let(result,heap)=observe_retirement_allocations(||source.borrow().bind(&mut binding,grant));assert_eq!(heap,(0,0));let receipt=result.unwrap().unwrap();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,copy);
  let(result,heap)=observe_retirement_allocations(||source.project_owned(1,|owner|owner,grant));let(mut projection,p)=result.unwrap();assert_eq!(heap,(0,0));assert!(p.fits(grant));
  for _ in 0..10000{
   if source.terminal_is_empty(){break;}
   let d=source.next_close_demand(4096).unwrap();let grant=exact_grant(d);assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);
   let(step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(source.terminal_is_empty());assert!(binding.is_some());assert!(released<=born);assert_eq!(projection.borrow().unwrap().get().as_ptr(),owner_pointer);assert_eq!(projection.borrow().unwrap().get().as_str(),row["owner"].as_str().unwrap());
  for _ in 0..10000{
   if binding.is_none(){break;}
   let d=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let grant=exact_grant(d);assert_eq!(RetainedCloneBinding::close_demand(&binding,grant.maximum_copy_bytes).unwrap(),d);
   let(step,heap)=observe_retirement_allocations(||RetainedCloneBinding::close_one(&mut binding,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(binding.is_none());assert_eq!(projection.borrow().unwrap().get().as_ptr(),owner_pointer);
  for _ in 0..10000{if projection.terminal_is_empty(){break;}let d=projection.next_close_demand(4096).unwrap();let grant=exact_grant(d);assert_eq!(projection.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||projection.close_step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
  assert!(projection.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(projection)).1,(0,0));assert_eq!(released,original_bytes+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
 }
}
   
#[test]
fn sealed_clone_source_returns_same_original_workspace_after_all_projection_aliases_return(){
 use crate::RetainedCloneSourceTake;
 use crate::retirement::controlled::ControlledRetirement;
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔐️custody.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let(original,heap)=observe_retirement_allocations(||Captured{owner:row["owner"].as_str().unwrap().into(),metadata:row["metadata"].as_str().unwrap().into()});let bytes=heap.0;let pointer=original.owner.as_ptr();let metadata=original.metadata.as_ptr();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<Captured>::constructor_copy_bytes(),maximum_capacity_bytes:RetainedCloneSource::<Captured>::constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};
  let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(original,(),grant));let(mut source,p)=result.unwrap_or_else(|_|panic!("actual workspace admission"));assert_eq!(heap,(p.retained_capacity_bytes,0));let(mut born,mut freed)=(heap.0,0);
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:source.borrow().binding_copy_bytes(),maximum_depth:1,..Default::default()};let(mut child,_)=source.project_owned(1,|root|&root.owner,grant).unwrap();
  let mut retained=false;for _ in 0..10000{let d=source.next_take_demand(4096).unwrap();let grant=exact_grant(d);assert_eq!(source.next_take_demand(grant.maximum_copy_bytes).unwrap(),d);let(result,heap)=observe_retirement_allocations(||source.take_authority(grant));match result{Ok(RetainedCloneSourceTake::Pending(p))=>{assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;},Err(error)=>{assert_eq!(heap,(0,0));assert_eq!(error.kind,crate::ValueRefusalKind::WorkLimit);retained=true;break;},Ok(RetainedCloneSourceTake::Ready(_, _))=>panic!("live genuine projection must retain original workspace")}}
  assert!(retained,"live original projection did not reach its retained handback frontier");assert_eq!(child.borrow().unwrap().get().as_ptr(),pointer);
  for _ in 0..10000{if child.terminal_is_empty(){break;}let d=child.next_close_demand(4096).unwrap();let grant=exact_grant(d);assert_eq!(child.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||child.close_step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
  assert!(child.terminal_is_empty());let d=source.next_take_demand(4096).unwrap();let grant=exact_grant(d);assert_eq!(source.next_take_demand(grant.maximum_copy_bytes).unwrap(),d);assert_eq!(d.copy_bytes,law["currency"]["metadataCopyBytes"].as_u64().unwrap()as usize);assert!(grant.maximum_release_bytes>0);
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe_retirement_allocations(||source.take_authority(denied));assert!(result.is_err());assert_eq!(heap,(0,0));assert!(!source.terminal_is_empty());}
  let(result,heap)=observe_retirement_allocations(||source.take_authority(grant).unwrap());let RetainedCloneSourceTake::Ready(original,p)=result else{panic!("all genuine aliases returned")};assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;assert_eq!(original.owner.as_ptr(),pointer);assert_eq!(original.metadata.as_ptr(),metadata);
  let mut close=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("typed original workspace close"));for _ in 0..10000{if close.terminal_is_empty(){break;}let d=close.next_demand(4096).unwrap();let grant=exact_grant(d);assert_eq!(close.next_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||close.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
  assert!(close.terminal_is_empty());assert_eq!(bytes+born,freed);assert_eq!(observe_retirement_allocations(||{drop(source);drop(child);drop(close);}).1,(0,0));
 }
}     
#[test]
fn immutable_binding_query_matches_exact_original_lease_and_projected_path(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔐️custody.json")).unwrap();let law=&fixture["bindingIdentity"];let demand=RetainedCloneSource::<u64>::owned_constructor_demand::<()>();let birth=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};let(mut first,_)=RetainedCloneSource::admit_owned(1u64,(),birth).unwrap_or_else(|_|panic!("original first admission"));let(mut second,_)=RetainedCloneSource::admit_owned(1u64,(),birth).unwrap_or_else(|_|panic!("original second admission"));let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let mut binding=None;first.borrow().bind(&mut binding,grant).unwrap();let bound=binding.as_ref().unwrap();let query=first.borrow();assert!(std::ptr::eq(query.get(),first.borrow().get()));assert!(!std::ptr::eq(query.get(),second.borrow().get()));assert_eq!(query.matches_binding(bound),law["sameOrigin"].as_bool().unwrap());assert_eq!(query.project(1,|value|value).matches_binding(bound),law["differentProjection"].as_bool().unwrap());assert_eq!(second.borrow().matches_binding(bound),law["differentOrigin"].as_bool().unwrap());
 for _ in 0..10000{if binding.is_none(){break;}let d=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let grant=exact_grant(d);assert_eq!(RetainedCloneBinding::close_demand(&binding,grant.maximum_copy_bytes).unwrap(),d);RetainedCloneBinding::close_one(&mut binding,grant).unwrap();}assert!(binding.is_none());for owner in [&mut first,&mut second]{for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.next_close_demand(4096).unwrap();let grant=exact_grant(d);assert_eq!(owner.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);owner.close_step(grant).unwrap();}assert!(owner.terminal_is_empty());}
}
