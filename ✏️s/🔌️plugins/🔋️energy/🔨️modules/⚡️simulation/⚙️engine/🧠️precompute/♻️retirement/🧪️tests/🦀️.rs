use super::*;
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress}};
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn observed<T>(f:impl FnOnce()->T)->(T,usize,usize){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);assert!(!heap.overflowed);(result,heap.requested_bytes,heap.released_bytes)}
/// ♻️ Original precompute owners close through the shared paid factory at every portable cancellation prefix.
/// A copy grant below the full quote re-quotes the divisible body (`step_granted` forwards the granted copy bytes as the body), so that axis may shrink to a smaller step that fits the denied grant; every other denied axis makes no progress.
#[test]
fn original_precompute_retirement_preserves_denied_owners_and_physical_receipts(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let body=corpus["bodyBytes"].as_u64().unwrap() as usize;let mut total=0;
 for case in corpus["cases"].as_array().unwrap(){for cutoff in corpus["cutoffs"].as_array().unwrap(){
  let model=match case["model"].as_str().unwrap(){"empty"=>Model::default(),"single"=>crate::model::test_model_single_zone(),"600"=>semio_s_plugin_energy_engine_bestest::model("600").unwrap(),_=>unreachable!()};let mut builder=PrecomputeBuilder::new(10,10);let mut steps=0;let limit=if case["complete"].as_bool().unwrap(){usize::MAX}else{cutoff.as_u64().unwrap() as usize};while !builder.is_complete()&&steps<limit{builder.step(&model);steps+=1;assert!(steps<100_000,"original builder fixture stalled");}
  let mut original=Some(builder);let mut close=RetainedCloneClose::default();let mut turns=0;
  while original.is_some()||!close.is_empty(){
   let demand=close.next_owner_demands::<PrecomputeBuilder>(original.is_some(),&None,body).unwrap();let full=grant(demand);
   for axis in 0..5{let mut denied=full;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if demand.release_bytes>0=>denied.maximum_release_bytes-=1,4 if demand.depth>0=>denied.maximum_depth-=1,_=>continue};let before=original.as_ref().map(|o|(o.retained_wire_signature(),serde_json::to_vec(o).unwrap()));let(step,alloc,freed)=observed(||if let Some(step)=close.begin_granted(&mut original,denied).unwrap(){step}else{close.step_granted(denied).unwrap()});if step.progress()!=RetainedCloneProgress::default(){assert!(axis==1&&step.progress().fits(denied),"only a divisible copy quote may shrink to the denied body");assert!(alloc<=step.progress().retained_capacity_bytes&&freed<=step.progress().released_bytes);break;}assert_eq!((alloc,freed),(0,0));assert_eq!(close.next_owner_demands::<PrecomputeBuilder>(original.is_some(),&None,body).unwrap(),demand);if let Some((signature,wire))=before{let held=original.as_ref().expect("denied original owner retained");assert_eq!(held.retained_wire_signature(),signature);assert_eq!(serde_json::to_vec(held).unwrap(),wire);}}
   let(step,alloc,freed)=observed(||if let Some(step)=close.begin_granted(&mut original,full).unwrap(){step}else{close.step_granted(full).unwrap()});let progress=step.progress();assert!(progress.fits(full));assert!(alloc<=progress.retained_capacity_bytes,"unpriced original cursor birth");assert!(freed<=progress.released_bytes,"unpriced original backing release");turns+=1;assert!(turns<100_000,"original precompute close stalled");
  }
  assert!(original.is_none()&&close.is_empty());total+=turns;
 }}
 eprintln!("[DEBUG] original precompute retirement portable prefixes closed across {total} actual paid factory turns");
}
