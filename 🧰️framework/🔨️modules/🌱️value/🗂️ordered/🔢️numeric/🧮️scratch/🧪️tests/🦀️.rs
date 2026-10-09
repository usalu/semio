use super::*;
use std::collections::BTreeMap;
#[test]
fn numeric_index_shared_operations_match_ordered_reference(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("📜️fixtures/🔣️.json")).unwrap();let mut index=NumericIndex::<i64,i64>::new();let mut oracle=BTreeMap::new();let mut output=Vec::new();
 for row in fixture["operations"].as_array().unwrap(){let key=row["key"].as_i64().unwrap_or(0);let result=match row["kind"].as_str().unwrap(){"set"=>{let value=row["value"].as_i64().unwrap();let actual=index.insert(key,value);let expected=oracle.insert(key,value);assert_eq!(actual,expected);serde_json::json!(actual)},"remove"=>{let actual=index.remove(&key);assert_eq!(actual,oracle.remove(&key));serde_json::json!(actual)},"get"=>{let actual=index.get(&key).copied();assert_eq!(actual,oracle.get(&key).copied());serde_json::json!(actual)},"first"=>{let actual=index.pop_first();assert_eq!(actual,oracle.pop_first());serde_json::json!(actual.map(|(key,value)|[key,value]))},"reset"=>{index.reset();oracle.clear();serde_json::Value::Null},_=>unreachable!()};assert_eq!(index.len(),oracle.len());output.push(result);}
 assert_eq!(serde_json::json!(output),fixture["expected"]);
}
#[test]
fn numeric_index_rotations_keep_logarithmic_depth_and_reuse_slots(){
 let mut index=NumericIndex::<usize,usize>::new();for key in 0..4096{index.insert(key,key*2);}assert!(index.height(index.root)<=14);for key in (0..4096).step_by(3){assert_eq!(index.remove(&key),Some(key*2));}let stored=index.stored_keys();index.reset();for key in (0..4096).rev(){index.insert(key,key);}assert_eq!(index.stored_keys(),stored);for key in 0..4096{assert_eq!(index.pop_first(),Some((key,key)));}assert!(index.is_empty());assert_eq!(index.stored_keys(),stored);
}
