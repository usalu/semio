//! 🧪️ The neutral dictionary corpus validates real fixed slots against the standard independent map.
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::scratch_hash::*;
use std::hash::{Hash,Hasher};
use semio_framework_value_testing_allocator as _;
#[test]
fn original_scratch_hash_matches_neutral_dictionary(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🔑️hashed/🧮️scratch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let maximum=fixture["maximum"].as_u64().unwrap()as usize;
 let mut actual=ScratchHashMap::<String,i64>::try_with_capacity(maximum).unwrap();let mut oracle=std::collections::HashMap::new();
 let demand=ScratchHashMap::<String,i64>::allocation_demands(maximum).unwrap();assert_eq!(actual.allocation_progress().copied_bytes,demand.copy_bytes);assert_eq!(actual.allocation_progress().retained_capacity_bytes,demand.capacity_bytes);
 for row in fixture["operations"].as_array().unwrap(){
  match row["kind"].as_str().unwrap(){
   "insert"=>{let key=row["key"].as_str().unwrap().to_owned();let value=row["value"].as_i64().unwrap();let accepted=oracle.contains_key(&key)||oracle.len()<maximum;let result=actual.try_insert(key.clone(),value);assert_eq!(result.is_ok(),accepted);if accepted{assert_eq!(result.unwrap(),oracle.insert(key,value));}else{assert_eq!(result.unwrap_err(),(key,value));}},
   "get"=>{let key=row["key"].as_str().unwrap();assert_eq!(actual.get(key),oracle.get(key));},
   "remove"=>{let key=row["key"].as_str().unwrap();assert_eq!(actual.remove(key),oracle.remove(key));},
   "entries"=>{},_=>unreachable!()
  }
  assert_eq!(actual.len(),oracle.len());for(key,value)in&oracle{assert_eq!(actual.get(key),Some(value));}
 }
}
#[test]
fn scratch_hash_forced_collisions_preserve_deleted_probe_chains(){
 #[derive(Eq,PartialEq,Debug)]struct Collision(u64);impl Hash for Collision{fn hash<H:Hasher>(&self,state:&mut H){state.write_u8(0)}}
 let mut actual=ScratchHashMap::try_with_capacity(128).unwrap();
 for key in 0..128{assert!(actual.try_insert(Collision(key),key).unwrap().is_none());}
 for key in (0..128).step_by(2){assert_eq!(actual.remove(&Collision(key)),Some(key));}
 for key in (1..128).step_by(2){assert_eq!(actual.get(&Collision(key)),Some(&key));}
 for key in 128..192{assert!(actual.try_insert(Collision(key),key).unwrap().is_none());}
 assert_eq!(actual.len(),128);assert_eq!(actual.try_insert(Collision(192),192).unwrap_err(),(Collision(192),192));
}

#[test]
fn incremental_fixed_slots_reserve_initialize_probe_and_release_without_growth(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📇️incremental.json")).unwrap();let maximum=fixture["maximum"].as_u64().unwrap()as usize;let mut map=ScratchHashMap::<usize,usize>::try_reserve_uninitialized(maximum).unwrap();let demand=ScratchHashMap::<usize,usize>::reservation_demands(maximum).unwrap();assert_eq!(demand.copy_bytes,0);assert_eq!(map.backing_bytes(),demand.capacity_bytes);assert_eq!(map.initialized_slots(),0);let mut initialized=0;while map.initialize_one().unwrap(){initialized+=1;assert_eq!(map.initialized_slots(),initialized)}assert_eq!(initialized,fixture["slots"].as_u64().unwrap()as usize);let mut oracle=std::collections::BTreeMap::new();for row in fixture["entries"].as_array().unwrap(){let key=row[0].as_u64().unwrap()as usize;let value=row[1].as_u64().unwrap()as usize;let mut probe=0;while !map.slots().is_vacant(probe){probe+=1}assert!(map.insert_vacant(probe,key,value).is_ok());oracle.insert(key,value);}for at in 0..map.initialized_slots(){if let Some((key,value))=map.slots().get(at){assert_eq!(oracle.get(key),Some(value))}}assert!(map.insert_vacant(0,23,230).is_err());let mut retired=Vec::new();while let Some(slot)=map.pop_last_slot(){if let Some((key,_))=slot{retired.push(key)}}let expected:Vec<_>=fixture["retirementOrder"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize).collect();assert_eq!(retired,expected);assert_eq!(map.release_empty_backing().unwrap(),demand.capacity_bytes);assert_eq!(map.backing_bytes(),0);assert!(map.is_empty());}
