//! 🧪️ Original removal cases use the actual finite Source, native effects and paid cancellation.
use super::*;
use super::edit_test_support as native;
use crate::observe_retirement_allocations as observe;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Fixture{page_capacity:usize,cases:Vec<Case>,interruption_turns:Vec<u64>}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Case{name:String,count:usize,key:u64,expected_removed:bool}
fn bank()->EffectBank{EffectBank::new(Wallet::from_profile(&native::law()["stock"]))}
fn add(total:&mut(usize,usize),heap:(usize,usize)){total.0+=heap.0;total.1+=heap.1;}
fn advance<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>(cursor:&mut RetainedOrderedMapRemoveCursor<K,V>,bank:&mut EffectBank,body:usize)->native::Observed{
 let p=cursor.next_advance_demands(body).unwrap();assert!(p.maximum_moved_items<=32);assert!(p.maximum_moved_bytes<=4096);let d=cursor.advance_demands(body,p).unwrap();let g=native::grant(d);let units=cursor.advance_work_demand(body,p).unwrap();
 for denied in native::denials(g,d){let result=native::action(cursor,bank,body,p,denied,units);assert_eq!(result.step.progress(),Default::default());assert_eq!(result.work,0);assert_eq!(result.heap,(0,0));assert_eq!(cursor.advance_demands(body,p).unwrap(),d);}
 native::action(cursor,bank,body,p,g,units)
}
#[test]
fn owned_removal_matches_std_map_and_retires_original_pages_on_completion_and_cancellation(){
 let fixture:Fixture=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture.page_capacity,RETAINED_ORDERED_MAP_PAGE_CAPACITY);
 for case in fixture.cases{for interrupted in fixture.interruption_turns.iter().copied().map(Some).chain([None]){
  let(map,heap)=observe(||(0..case.count as u64).map(|key|(key,())).collect::<RetainedOrderedMap<_,_>>());let original=heap.0-heap.1;let pointer=map.pages.as_ptr();let(mut cursor,heap)=observe(||RetainedOrderedMapRemoveCursor::new(map,case.key));assert_eq!(heap,(0,0));assert!(!cursor.birth_complete());let mut bank=bank();let mut total=(0,0);let mut output=None;
  for turn in 0..100000{if interrupted==Some(turn){break}if cursor.output_ready(){assert_eq!(cursor.removed(),Some(case.expected_removed));output=Some(native::take(&mut cursor,&mut bank,8));assert!(!cursor.output_ready());break}add(&mut total,advance(&mut cursor,&mut bank,8).heap);}
  if interrupted.is_none(){let map=output.as_ref().expect("completed original map");let mut oracle=(0..case.count as u64).map(|key|(key,())).collect::<std::collections::BTreeMap<_,_>>();oracle.remove(&case.key);assert_eq!(map.iter().map(|(key,_)|*key).collect::<Vec<_>>(),oracle.keys().copied().collect::<Vec<_>>());assert_eq!(map.pages.as_ptr(),pointer);assert_eq!(map.page_count(),map.len().div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY));}
  add(&mut total,native::retire_observed(cursor,&mut bank,8,100000));if let Some(map)=output{add(&mut total,native::retire_observed(map,&mut bank,8,100000));}assert_eq!(original+total.0,total.1,"{} interrupted {interrupted:?}",case.name);
 }}
}
#[test]
fn owned_removal_string_keys_and_values_keep_exact_heap_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let keys=fixture["stringKeys"].as_array().unwrap();
 for target in fixture["stringTargets"].as_array().unwrap(){let target=target.as_str().unwrap();let(map,heap)=observe(||keys.iter().map(|key|{let key=key.as_str().unwrap();(key.to_owned(),key.repeat(3))}).collect::<RetainedOrderedMap<_,_>>());let mut original=heap.0-heap.1;let(key,heap)=observe(||target.to_owned());original+=heap.0-heap.1;let(mut cursor,heap)=observe(||RetainedOrderedMapRemoveCursor::new(map,key));assert_eq!(heap,(0,0));let mut bank=bank();let mut total=(0,0);
  for _ in 0..100000{if cursor.output_ready(){break}add(&mut total,advance(&mut cursor,&mut bank,1).heap);}assert!(cursor.output_ready());let map=native::take(&mut cursor,&mut bank,1);let mut oracle=keys.iter().map(|key|{let key=key.as_str().unwrap();(key.to_owned(),key.repeat(3))}).collect::<std::collections::BTreeMap<_,_>>();oracle.remove(target);assert_eq!(serde_json::to_value(&map).unwrap(),serde_json::to_value(oracle).unwrap());add(&mut total,native::retire_observed(cursor,&mut bank,1,100000));add(&mut total,native::retire_observed(map,&mut bank,1,100000));assert_eq!(original+total.0,total.1);
 }
}
fn controlled_edit<C:native::NativeEdit>(mut cursor:C,original:usize,turns:u64,body:usize){
 let mut bank=bank();let mut total=(0,0);for _ in 0..turns{if native::NativeEdit::ready(&cursor){break}let p=cursor.policy(body);let d=cursor.demand(body,p);let units=cursor.work(body,p);let result=native::action_result(&mut cursor,&mut bank,body,p,native::grant(d),units);add(&mut total,result.heap);if let Err(error)=result.result{assert!(error.primary().unwrap().message.contains("duplicate"));add(&mut total,native::retire_observed(error,&mut bank,body,100000));break}}
 add(&mut total,native::retire_observed(cursor,&mut bank,body,100000));assert_eq!(original+total.0,total.1);
}
#[test]
fn owned_removal_and_insertion_frames_close_original_edit_owners_under_controlled_retirement(){
 for turns in [0,1,17,1000]{for key in [16,40]{let(map,heap)=observe(||(0u64..33).map(|key|(key,())).collect::<RetainedOrderedMap<_,_>>());let original=heap.0-heap.1;let(cursor,heap)=observe(||RetainedOrderedMapInsertCursor::new(map,key,()));assert_eq!(heap,(0,0));controlled_edit(cursor,original,turns,8);let(map,heap)=observe(||(0u64..33).map(|key|(key,())).collect::<RetainedOrderedMap<_,_>>());let original=heap.0-heap.1;let(cursor,heap)=observe(||RetainedOrderedMapRemoveCursor::new(map,key));assert_eq!(heap,(0,0));controlled_edit(cursor,original,turns,8);}}
}
#[test]
fn staged_authority_birth_charges_exact_native_permanent_and_transient_owners(){
 type W=super::edit_input::EditWorkspace<u64,()>;let(map,heap)=observe(||(0..33u64).map(|key|(key,())).collect::<RetainedOrderedMap<_,_>>());let original=heap.0-heap.1;let(mut cursor,heap)=observe(||RetainedOrderedMapRemoveCursor::new(map,7));assert_eq!(heap,(0,0));let permanent=crate::RetainedCloneSource::<W>::owned_constructor_capacity_bytes::<()>();let transient=size_of::<crate::RetainedCloneSourceAdmission<W,W>>();let mut bank=bank();let mut total=(0,0);let mut copied=0;let mut turns=0;let mut events=Vec::new();
 while !cursor.birth_complete(){let result=advance(&mut cursor,&mut bank,8);events.push(serde_json::json!({"allocated":result.heap.0,"released":result.heap.1,"copied":result.step.progress().copied_bytes,"work":result.work}));copied+=result.step.progress().copied_bytes;add(&mut total,result.heap);turns+=1;assert!(turns<64);}
 native::assert_birth(permanent,transient,events);assert_eq!(total,(permanent+transient,transient));assert!(copied>0);assert!(turns>1);add(&mut total,native::retire_observed(cursor,&mut bank,8,100000));assert_eq!(original+total.0,total.1);
}
