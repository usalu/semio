use super::*;
use crate::value::observe_retirement_allocations as observe;
#[test]
fn shared_unique_original_arc_retains_weak_custody_then_credits_actual_frame(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for copy in fixture["copyGrants"].as_array().unwrap(){
  let value=row["value"].as_u64().unwrap();let (source,birth)=observe(||Arc::new(value));assert_eq!(birth,(arc_bytes::<u64>(),0));let pointer=Arc::as_ptr(&source);
  let mut weak:Vec<_>=(0..row["weakAliases"].as_u64().unwrap()).map(|_|Arc::downgrade(&source)).collect();
  let (mut cursor,heap)=observe(||SharedControlledRetirement::new(source));assert_eq!(heap,(0,0));
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:0,maximum_release_bytes:arc_bytes::<u64>(),maximum_depth:1};
  let (step,heap)=observe(||cursor.step(RetainedCloneGrant{maximum_release_bytes:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(Arc::as_ptr(cursor.source.as_ref().unwrap()),pointer);
  if !weak.is_empty(){let(step,heap)=observe(||cursor.step(grant).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(**cursor.source.as_ref().unwrap(),value);}
  weak.clear();
  let(step,heap)=observe(||cursor.step(grant).unwrap());assert_eq!(step.progress().released_bytes,arc_bytes::<u64>());assert_eq!(heap,(0,arc_bytes::<u64>()));assert!(cursor.source.is_none());assert_eq!(cursor.owned.as_ref().unwrap().original(),Some(&value));
  let mut births=0;let mut released=0;
  for _ in 0..64{if cursor.terminal_is_empty(){break;}let copy=grant.maximum_copy_bytes;let grant=RetainedCloneGrant{maximum_capacity_bytes:cursor.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:cursor.next_release_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand().unwrap(),..grant};let(step,heap)=observe(||cursor.step(grant).unwrap());let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));births+=heap.0;released+=heap.1;}
  assert!(cursor.terminal_is_empty());assert_eq!(births,released);let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));
 }}
 eprintln!("[DEBUG] shared unique Arc original weak custody and physical receipts conserved");
}
