//! 🧪️ Portable registry cases prove original custody and atomic visibility through each insertion prefix.
#[path="../🦀️.rs"]mod registry;
use registry::WindowRegistry;
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn keys(registry:&WindowRegistry<String,u64>)->serde_json::Value{serde_json::to_value(registry.iter().map(|(key,_)|key.as_str()).collect::<Vec<_>>()).unwrap()}
fn measured<T>(g:RetainedCloneGrant,run:impl FnOnce()->(T,RetainedCloneProgress))->T{let((output,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);assert!(receipt.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));output}
#[test]
fn native_registry_custody_matches_portable_atomic_publication_and_cancellation(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for case in law["cases"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){
  let mut registry=WindowRegistry::new();for key in case["before"].as_array().unwrap(){registry.insert(key.as_str().unwrap().to_owned(),1);}
  let mut key=Some(law["key"].as_str().unwrap().to_owned());let original=key.as_ref().unwrap().as_ptr();let mut value=Some(law["value"].as_u64().unwrap());let mut ready=false;
  for _ in 0..cut.as_u64().unwrap(){let demand=registry.insert_original_demands().unwrap();let funded=grant(demand);
   for axis in 0..5{let mut denied=funded;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if demand.release_bytes>0=>denied.maximum_release_bytes-=1,4=>denied.maximum_depth=0,_=>continue}let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.insert_original(&mut key,&mut value,denied).unwrap());assert_eq!(done,(false,RetainedCloneProgress::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(keys(&registry),case["before"]);assert_eq!(key.as_ref().unwrap().as_ptr(),original);}
   ready=measured(funded,||registry.insert_original(&mut key,&mut value,funded).unwrap());assert_eq!(keys(&registry),case[if ready{"after"}else{"before"}]);if ready{break;}assert_eq!(key.as_ref().unwrap().as_ptr(),original);assert_eq!(value,law["value"].as_u64());
  }
  if ready{assert!(key.is_none()&&value.is_none());assert_eq!(registry.get(law["key"].as_str().unwrap()),law["value"].as_u64().as_ref());}else{for turn in 0..32{let d=registry.cancel_insert_original_demands().unwrap();let funded=grant(d);let done=measured(funded,||registry.cancel_insert_original(&mut key,&mut value,funded).unwrap());assert_eq!(keys(&registry),case["before"]);assert_eq!(key.as_ref().unwrap().as_ptr(),original);assert_eq!(value,law["value"].as_u64());if done{break;}assert!(turn<31,"original registry hole failed to cancel");}}
  while !registry.is_empty(){let d=registry.pop_demand().unwrap();let g=grant(d);let original=measured(g,||{let(original,p)=registry.pop_original(g).unwrap().unwrap();(original,p)});drop(original);}
  for turn in 0..32{if registry.terminal_is_empty(){break;}let d=registry.backing_demand().unwrap();let g=grant(d);measured(g,||{let step=registry.close_backing_step(g).unwrap();((),step.progress())});assert!(turn<31,"original registry backing failed to close");}
  assert!(registry.terminal_is_empty());
 }}
}
