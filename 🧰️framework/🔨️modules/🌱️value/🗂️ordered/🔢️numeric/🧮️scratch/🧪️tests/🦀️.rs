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
 let fixture:serde_json::Value=serde_json::from_str(crate::text("🗂️ordered/🔢️numeric/🧮️scratch/🧪️tests/📜️fixtures/🔣️.json")).unwrap();let keys=fixture["retained"]["keys"].as_u64().unwrap() as usize;let mut index=NumericIndex::<usize,usize>::new();
 for key in 0..keys{loop{match index.insert_reserved(key,key){Ok(previous)=>{assert_eq!(previous,None);break;},Err((retained_key,retained_value))=>{assert_eq!((retained_key,retained_value),(key,key));let quote=index.next_allocation_bytes().unwrap();assert!(quote>0);assert!(!index.reserve_one(quote-1).unwrap().progressed);assert!(index.reserve_one(quote).unwrap().progressed);}}}}
 for key in 0..keys{assert_eq!(index.get(&key),Some(&key));}let stored=index.stored_keys();index.reset();for key in (0..keys).rev(){assert_eq!(index.insert_reserved(key,key*2).unwrap(),None);}assert_eq!(index.stored_keys(),stored);
 let mut turns=0;let mut released=0;while !index.terminal_is_empty(){let quote=index.next_close_release_byte_demand().unwrap();let copy=index.next_close_copy_byte_demand();if copy>0{assert!(!index.close_copy_step_funded(1,copy-1,quote).unwrap().progressed);}assert!(!index.close_copy_step_funded(0,copy,quote).unwrap().progressed);if quote>0{assert!(!index.close_copy_step_funded(1,copy,quote-1).unwrap().progressed);}let step=index.close_copy_step_funded(1,copy,quote).unwrap();assert!(step.progressed);released+=step.released_allocation_bytes;turns+=1;assert!(turns<fixture["maximumCloseTurns"].as_u64().unwrap() as usize);}assert!(released>0);assert_eq!(index.stored_keys(),0);
}
