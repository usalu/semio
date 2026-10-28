//! 🧪️ Shared custody laws use genuine original sources and independently funded transfer receipts.
use super::*;
use crate::retained_clone::{RetainedCloneBinding};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn checked<S:RetireOwned+Sync>(owner:&mut OriginalPostRead<S>,post:&mut Option<S>,mode:PagedOneItemPostMode){
 for _ in 0..1000{let Some(d)=owner.demand(post,mode,4096).unwrap()else{return;};let g=grant(d);
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth-1,..g})].into_iter().flatten(){let(result,heap)=observe_retirement_allocations(||owner.prepare(post,mode,4096,denied).unwrap());assert_eq!(result,Some(Default::default()));assert_eq!(heap,(0,0));assert_eq!(owner.demand(post,mode,4096).unwrap(),Some(d));}
  let(result,heap)=observe_retirement_allocations(||owner.prepare(post,mode,4096,g).unwrap());let p=result.unwrap();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
 }panic!("original post transfer stalled");
}
#[test]
fn original_post_lends_actual_projected_value_and_returns_same_payload_before_commit_or_cancel(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../👁️post/🧪️tests/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){let original=row["value"].to_string();let pointer=original.as_ptr();let mut post=Some(original);let mut owner=OriginalPostRead::<String>::default();
  if row["reads"].as_u64().unwrap()>0{checked(&mut owner,&mut post,PagedOneItemPostMode::Original);let mut aliases=[None,None,None,None];for alias in aliases.iter_mut().take(row["reads"].as_u64().unwrap()as usize){let value=owner.read(&post,PagedOneItemPostMode::Original).unwrap().original().unwrap();assert_eq!(value.get().as_ptr(),pointer);let d=RetirementDemand{copy_bytes:value.binding_copy_bytes(),depth:1,..Default::default()};value.bind(alias,grant(d)).unwrap();}
   assert!(owner.access(&mut post,PagedOneItemPostMode::Mutable).is_err());
   for alias in &mut aliases{while alias.is_some(){let d=RetirementDemand{copy_bytes:RetainedCloneBinding::copy_demand(alias).unwrap(),capacity_bytes:RetainedCloneBinding::capacity_demand(alias,4096).unwrap(),release_bytes:RetainedCloneBinding::release_demand(alias).unwrap(),depth:RetainedCloneBinding::depth_demand(alias).unwrap()};let(step,heap)=observe_retirement_allocations(||RetainedCloneBinding::close_one(alias,grant(d)).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}
  }
  checked(&mut owner,&mut post,PagedOneItemPostMode::Mutable);assert!(owner.terminal_is_empty());assert_eq!(post.as_ref().unwrap().as_ptr(),pointer);assert_eq!(serde_json::from_str::<serde_json::Value>(post.as_ref().unwrap()).unwrap(),row["value"]);
  let mut retirement=crate::retirement::controlled::ControlledRetirement::new(post.take().unwrap()).map_err(|(e,_)|e).unwrap();while !retirement.terminal_is_empty(){let g=grant(retirement.next_demand(4096).unwrap());retirement.step(g).unwrap();}assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));
 }
}
