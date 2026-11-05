//! 🧪️ Original owned entries match neutral updates, real allocation receipts and every cancellation prefix.
use super::*;
fn post(bank:&mut RetainedCloneGrant,fuel:&mut u64,p:RetainedCloneProgress,work:u64){assert!(p.fits(*bank));bank.maximum_items=bank.maximum_items.checked_sub(p.copied_items).unwrap();bank.maximum_copy_bytes=bank.maximum_copy_bytes.checked_sub(p.copied_bytes).unwrap();bank.maximum_capacity_bytes=bank.maximum_capacity_bytes.checked_sub(p.retained_capacity_bytes).unwrap();bank.maximum_release_bytes=bank.maximum_release_bytes.checked_sub(p.released_bytes).unwrap();*fuel=fuel.checked_sub(work).unwrap();}
fn close<T:RetireOwned>(owner:T,bank:&mut RetainedCloneGrant,fuel:&mut u64){let mut close=semio_framework_value_retirement::retirement::controlled::ControlledRetirement::new(owner).unwrap_or_else(|_|panic!("original test owner supports retained closure"));for _ in 0..10000{if close.terminal_is_empty(){return}let(result,heap)=crate::observe_retirement_allocations(||close.step(RetainedCloneGrant{maximum_items:1,..*bank}));let p=result.as_ref().map_or_else(|e|e.retained_progress(),|s|s.progress());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));post(bank,fuel,p,close.last_turn_work());result.unwrap();}panic!("original entry test close exhausted finite stock");}
#[test]
fn original_owned_entry_retains_real_map_updates_and_cancel_prefixes(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){for cut in corpus["cancelAfter"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap())).chain([None]){
  let stock=&corpus["stock"];let mut bank=RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize};let mut fuel=stock["fuel"].as_u64().unwrap();
  let base:OrderedMap<u64>=row["base"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_owned(),pair[1].as_u64().unwrap())).collect();let alias=base.clone();let identity=base.original_root_identity();let key=row["key"].as_str().unwrap().to_owned();let value=row["value"].as_u64().unwrap();
  let(mut work,heap)=crate::observe_retirement_allocations(||OriginalOwnedEntry::new(base,key,value));assert_eq!(heap,(0,0));assert_eq!(work.fields.base.as_ref().unwrap().original_root_identity(),identity);let mut output=None;
  for turn in 0..corpus["maximumTurns"].as_u64().unwrap(){if cut==Some(turn){break}let quote=work.next_demands(4096).unwrap();let(paused,heap)=crate::observe_retirement_allocations(||work.advance(RetainedCloneGrant{maximum_items:0,..bank}));assert!(paused.result.unwrap().is_none());assert_eq!(paused.work,0);assert_eq!(heap,(0,0));assert_eq!(work.next_demands(4096).unwrap(),quote);
   let(outcome,heap)=crate::observe_retirement_allocations(||work.advance(RetainedCloneGrant{maximum_items:1,..bank}));let p=work.step_progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));post(&mut bank,&mut fuel,p,outcome.work);output=outcome.result.unwrap();if output.is_some(){break}assert!(turn+1<corpus["maximumTurns"].as_u64().unwrap());
  }
  if cut.is_none(){let actual:Vec<_>=output.as_ref().unwrap().iter().map(|(k,v)|(k.clone(),*v)).collect();let expected:Vec<_>=row["expected"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_owned(),pair[1].as_u64().unwrap())).collect();assert_eq!(actual,expected)}
  let original:Vec<_>=alias.iter().map(|(k,v)|(k.clone(),*v)).collect();let expected:Vec<_>=row["base"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_owned(),pair[1].as_u64().unwrap())).collect();assert_eq!(original,expected);
  work.begin_close();for turn in 0..10000{if work.terminal_is_empty(){break}let(outcome,heap)=crate::observe_retirement_allocations(||work.close_step(RetainedCloneGrant{maximum_items:1,..bank}));let p=outcome.result.as_ref().map_or_else(|e|e.retained_progress(),|s|s.progress());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));post(&mut bank,&mut fuel,p,outcome.work);outcome.result.unwrap();assert!(turn+1<10000)}
  assert!(work.terminal_is_empty());drop(work);if let Some(output)=output{close(output,&mut bank,&mut fuel)}close(alias,&mut bank,&mut fuel);
 }}
}

#[test]
fn actual_ordered_comparison_denies_unfunded_source_bytes(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let row=&corpus["comparison"];let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096};let base=OrderedMap::from([(row["key"].as_str().unwrap().to_owned(),1u64)]);let(key,_)=SharedOwner::admit(row["key"].as_str().unwrap().to_owned(),g).result.unwrap_or_else(|_|panic!("original key header"));let(value,_)=SharedOwner::admit(2u64,g).result.unwrap_or_else(|_|panic!("original value header"));let mut update=base.begin_set_shared(key,value);
 let(turn,heap)=crate::observe_retirement_allocations(||update.advance(RetainedCloneGrant{maximum_copy_bytes:0,..g}));assert_eq!(turn.work,row["zeroWork"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(update.last_compared_bytes(),0);assert_eq!(update.next_copy_byte_demand(),1);
 let(turn,heap)=crate::observe_retirement_allocations(||update.advance(RetainedCloneGrant{maximum_copy_bytes:1,..g}));assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap().progress().copied_bytes,1);assert_eq!(heap,(0,0));assert_eq!(update.last_compared_bytes(),1);assert_eq!(update.state.left_byte,Some(row["firstByte"].as_u64().unwrap()as u8));
 let mut bank=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<25,maximum_depth:4096};let mut fuel=32768;close(update,&mut bank,&mut fuel);close(base,&mut bank,&mut fuel);
}

#[path="➖remove/🧪️tests/🦀️.rs"]
mod removal_laws;
