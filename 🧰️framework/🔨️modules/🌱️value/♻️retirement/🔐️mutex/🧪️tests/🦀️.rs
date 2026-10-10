use super::*;
use crate::value::observe_retirement_allocations as observe;

#[test]
fn original_mutex_owned_and_poisoned_payloads_preserve_exact_heap_custody(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let original=row["owner"].as_str().unwrap().to_owned();let pointer=original.as_ptr();let bytes=original.capacity();let mutex=std::sync::Mutex::new(original);
  if row["poisoned"].as_bool().unwrap(){let _=std::panic::catch_unwind(||{let _guard=mutex.lock().unwrap();panic!("test-only poison original owned mutex");});}
  assert_eq!(mutex.is_poisoned(),row["poisoned"].as_bool().unwrap());let (result,heap)=observe(||ControlledRetirement::new(mutex));assert_eq!(heap,(0,0));let mut owner=result.unwrap_or_else(|_|panic!("original mutex retirement refused"));
  assert_eq!(owner.original().unwrap().lock().unwrap_or_else(|error|error.into_inner()).as_ptr(),pointer);
  let mut born=0;let mut released=0;
  for _ in 0..512{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap().max(1);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
   for axis in 0..5{let mut denied=grant;let active=match axis{0=>{denied.maximum_items=0;true},1=>{let demand=owner.next_copy_byte_demand().unwrap();denied.maximum_copy_bytes=demand.saturating_sub(1);demand>0},2=>{denied.maximum_capacity_bytes=grant.maximum_capacity_bytes.saturating_sub(1);grant.maximum_capacity_bytes>0},3=>{denied.maximum_release_bytes=grant.maximum_release_bytes.saturating_sub(1);grant.maximum_release_bytes>0},_=>{denied.maximum_depth=grant.maximum_depth.saturating_sub(1);grant.maximum_depth>0}};if active{let(result,heap)=observe(||owner.step(denied));assert_eq!(heap,(0,0));if let Ok(step)=result{assert_eq!(step.progress(),Default::default());}}}
   let(step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,bytes+born);assert_eq!(observe(||drop(owner)).1,(0,0));assert_eq!(serde_json::json!({"preserved":true,"terminal":true}),row["expected"]);
 }
 eprintln!("[DEBUG] original owned/poisoned mutex payload retains exact pointer and all denied/physical retirement currencies");
}
