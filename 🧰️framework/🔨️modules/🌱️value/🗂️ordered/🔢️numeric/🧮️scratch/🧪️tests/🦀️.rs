use super::*;
use std::collections::BTreeMap;
#[test]
fn numeric_index_shared_operations_match_ordered_reference(){
 let fixture:serde_json::Value=serde_json::from_str(crate::text("🗂️ordered/🔢️numeric/🧮️scratch/🧪️tests/📜️fixtures/🔣️.json")).unwrap();let mut index=NumericIndex::<i64,i64>::new();let mut oracle=BTreeMap::new();let mut output=Vec::new();
 for row in fixture["operations"].as_array().unwrap(){let key=row["key"].as_i64().unwrap_or(0);let result=match row["kind"].as_str().unwrap(){"set"=>{let value=row["value"].as_i64().unwrap();let actual=index.insert(key,value);let expected=oracle.insert(key,value);assert_eq!(actual,expected);serde_json::json!(actual)},"remove"=>{let actual=index.remove(&key);assert_eq!(actual,oracle.remove(&key));serde_json::json!(actual)},"get"=>{let actual=index.get(&key).copied();assert_eq!(actual,oracle.get(&key).copied());serde_json::json!(actual)},"first"=>{let actual=index.pop_first();assert_eq!(actual,oracle.pop_first());serde_json::json!(actual.map(|(key,value)|[key,value]))},"reset"=>{index.reset();oracle.clear();serde_json::Value::Null},_=>unreachable!()};assert_eq!(index.len(),oracle.len());output.push(result);}
 assert_eq!(serde_json::json!(output),fixture["expected"]);
}
#[test]
fn numeric_index_rotations_keep_logarithmic_depth_and_reuse_slots(){
 let mut index=NumericIndex::<usize,usize>::new();for key in 0..4096{index.insert(key,key*2);}assert!(index.height(index.root)<=14);for key in (0..4096).step_by(3){assert_eq!(index.remove(&key),Some(key*2));}let stored=index.stored_keys();index.reset();for key in (0..4096).rev(){index.insert(key,key);}assert_eq!(index.stored_keys(),stored);for key in 0..4096{assert_eq!(index.pop_first(),Some((key,key)));}assert!(index.is_empty());assert_eq!(index.stored_keys(),stored);
}

#[test]
fn numeric_scratch_retained_pages_match_shared_admission_and_close_contract(){
 use semio_framework_value_persistent::list::{PagedListReleaseOutcome,PagedListReservationOutcome};
 let fixture:serde_json::Value=serde_json::from_str(crate::text("🗂️ordered/🔢️numeric/🧮️scratch/🧪️tests/📜️fixtures/🔣️.json")).unwrap();
 let keys=fixture["retained"]["keys"].as_u64().unwrap() as usize;
 let mut index=NumericIndex::<usize,usize>::new();
 for key in 0..keys{
  loop{
   let demand=index.next_reserve_demand().unwrap();
   if demand.depth==0{break;}
   let grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:0,maximum_depth:demand.depth};
   let denied=crate::RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant};
   assert!(matches!(crate::effect_fixture::with_effect(denied,1024,&mut None,|effect|index.reserve_one_funded(effect)).result.unwrap(),PagedListReservationOutcome::Denied));
   let PagedListReservationOutcome::Ready(step)=crate::effect_fixture::with_effect(grant,1024,&mut None,|effect|index.reserve_one_funded(effect)).result.unwrap()else{panic!("original reservation denied")};
   assert_eq!(step.retained_capacity_bytes,demand.capacity_bytes);
  }
  assert_eq!(index.insert_reserved(key,key),Ok(None));
 }
 for key in 0..keys{assert_eq!(index.get(&key),Some(&key));}
 let stored=index.stored_keys();
 index.reset();
 for key in (0..keys).rev(){assert_eq!(index.insert_reserved(key,key*2),Ok(None));}
 assert_eq!(index.stored_keys(),stored);
 let mut turns=0;
 let mut released=0;
 while !index.terminal_is_empty(){
  let demand=index.next_close_demand().unwrap();
  let grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  let mut close=|grant|crate::effect_fixture::with_effect(grant,1024,&mut None,|effect|index.close_copy_step(effect)).result.unwrap();
  let denied=crate::RetainedCloneGrant{maximum_items:0,..grant};
  assert!(matches!(close(denied),PagedListReleaseOutcome::Denied));
  if demand.copy_bytes>0{assert!(matches!(close(crate::RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}),PagedListReleaseOutcome::Denied));}
  if demand.release_bytes>0{assert!(matches!(close(crate::RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}),PagedListReleaseOutcome::Denied));}
  let PagedListReleaseOutcome::Ready(step)=close(grant)else{panic!("original close denied")};
  assert!(step.progressed);
  assert_eq!(step.released_allocation_bytes,demand.release_bytes);
  released+=step.released_allocation_bytes;
  turns+=1;
  assert!(turns<fixture["maximumCloseTurns"].as_u64().unwrap() as usize);
 }
 assert!(released>0);
 assert_eq!(index.stored_keys(),0);
}
