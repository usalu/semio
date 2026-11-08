//! 🧪️ Original aliases, cancellation prefixes and byte frontiers use one genuine Source bank.
use super::*;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
#[path="🧪️tests/effect-bank.rs"]mod bank;
use bank::{Harness,Op,grant};
fn funded_bank(corpus:&serde_json::Value)->EffectBank{let s=&corpus["stock"];EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},s["fuel"].as_u64().unwrap()))}
fn drain(h:&mut Harness<'_>,bank:&mut EffectBank){
 for op in [Op::EntryStart,Op::EntryClose,Op::OutputStart,Op::OutputClose,Op::ExtraStart,Op::ExtraClose]{
  for _ in 0..10000{
   let empty=h.inspect(|p|match op{Op::EntryStart=>p.entry.is_none(),Op::EntryClose=>p.entry_close.is_empty(),Op::OutputStart=>p.output.is_none(),Op::OutputClose=>p.output_close.is_empty(),Op::ExtraStart=>p.extra.as_ref().unwrap().is_none(),Op::ExtraClose=>p.extra_close.as_ref().unwrap().is_empty(),_=>unreachable!()});if empty{break}
   let(d,work)=h.quote(op);let g=grant(d);h.run(bank,op,g,0);assert_eq!(h.last_work,0);assert_eq!(h.last_heap,(0,0));assert_eq!(h.quote(op).0,d);h.run(bank,op,g,work);
  }
 }
 assert!(h.inspect(|p|p.empty()));h.release(bank)
}
#[test]
fn original_owned_entry_retains_real_map_updates_and_cancel_prefixes(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){for cut in corpus["cancelAfter"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap())).chain([None]){
  let mut source=funded_bank(&corpus);let base:OrderedMap<u64>=row["base"].as_array().unwrap().iter().map(|p|(p[0].as_str().unwrap().to_owned(),p[1].as_u64().unwrap())).collect();let mut alias=Some(base.clone());let identity=base.original_root_identity();let key=row["key"].as_str().unwrap().to_owned();let value=row["value"].as_u64().unwrap();
  let(work,heap)=crate::observe_retirement_allocations(||OriginalOwnedEntry::new(base,key,value));assert_eq!(heap,(0,0));assert_eq!(work.fields.base.as_ref().unwrap().original_root_identity(),identity);let mut work=Some(work);let mut output=None;let mut close=RetainedCloneClose::default();let mut output_close=RetainedCloneClose::default();let mut alias_close=RetainedCloneClose::default();let mut h=Harness::new_with_alias(&mut work,&mut output,&mut close,&mut output_close,&mut alias,&mut alias_close,&source);
  for turn in 0..corpus["maximumTurns"].as_u64().unwrap(){
   if cut==Some(turn){break}let(d,required)=h.quote(Op::Advance);let g=grant(d);let phase=h.inspect(|p|p.entry.as_ref().unwrap().phase);
   if required>0{let step=h.run(&mut source,Op::Advance,g,required-1);assert_eq!(step.progress(),Default::default());assert_eq!(h.last_work,0);assert_eq!(h.last_heap,(0,0));assert_eq!(h.inspect(|p|p.entry.as_ref().unwrap().phase),phase);assert_eq!(h.quote(Op::Advance).0,d)}
   let step=h.run(&mut source,Op::Advance,RetainedCloneGrant{maximum_items:0,..g},required);assert_eq!(step.progress(),Default::default());assert_eq!(h.last_work,0);assert_eq!(h.last_heap,(0,0));assert_eq!(h.quote(Op::Advance).0,d);
   h.run(&mut source,Op::Advance,g,required);if h.inspect(|p|p.output.is_some()){break}assert!(turn+1<corpus["maximumTurns"].as_u64().unwrap())
  }
  if cut.is_none(){h.inspect(|p|{let actual:Vec<_>=p.output.as_ref().unwrap().iter().map(|(k,v)|(k.clone(),*v)).collect();let expected:Vec<_>=serde_json::from_value::<Vec<(String,u64)>>(row["expected"].clone()).unwrap();assert_eq!(actual,expected)})}
  h.inspect(|p|{let original:Vec<_>=p.extra.as_ref().unwrap().as_ref().unwrap().iter().map(|(k,v)|(k.clone(),*v)).collect();let expected:Vec<_>=serde_json::from_value::<Vec<(String,u64)>>(row["base"].clone()).unwrap();assert_eq!(original,expected)});
  drain(&mut h,&mut source)
 }}
}
#[test]
fn actual_ordered_comparison_denies_unfunded_source_bytes(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let row=&corpus["comparison"];let mut source=funded_bank(&corpus);
 let base=OrderedMap::from([(row["key"].as_str().unwrap().to_owned(),1u64)]);let mut alias=Some(base.clone());let mut entry=Some(OriginalOwnedEntry::new(base,row["key"].as_str().unwrap().to_owned(),2));let mut output=None;let mut close=RetainedCloneClose::default();let mut output_close=RetainedCloneClose::default();let mut alias_close=RetainedCloneClose::default();let mut h=Harness::new_with_alias(&mut entry,&mut output,&mut close,&mut output_close,&mut alias,&mut alias_close,&source);
 while h.inspect(|p|p.entry.as_ref().unwrap().phase)!=3{let(d,w)=h.quote(Op::Advance);h.run_body(&mut source,Op::Advance,1,grant(d),w);}
 let fixed=h.inspect(|p|p.entry.as_ref().unwrap().fields.update.as_ref().unwrap().state.native_copy_demand(0).unwrap());let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixed+1,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 let step=h.run_body(&mut source,Op::Advance,1,RetainedCloneGrant{maximum_copy_bytes:0,..g},0);assert_eq!(h.last_work,row["zeroWork"].as_u64().unwrap()as u128);assert_eq!(step.progress(),Default::default());assert_eq!(h.last_heap,(0,0));assert_eq!(h.inspect(|p|p.entry.as_ref().unwrap().fields.update.as_ref().unwrap().last_compared_bytes()),0);
 let quote=h.inspect(|p|p.entry.as_ref().unwrap().fields.update.as_ref().unwrap().native_work_demand(1).unwrap());assert_eq!(quote,row["oneByteWork"].as_u64().unwrap());let step=h.run_body(&mut source,Op::Advance,1,g,row["insufficientWork"].as_u64().unwrap());assert_eq!(h.last_work,0);assert_eq!(step.progress(),Default::default());assert_eq!(h.last_heap,(0,0));assert_eq!(h.inspect(|p|p.entry.as_ref().unwrap().fields.update.as_ref().unwrap().last_compared_bytes()),0);
 let step=h.run_body(&mut source,Op::Advance,1,g,quote);assert_eq!(h.last_work,row["oneByteWork"].as_u64().unwrap()as u128);assert_eq!(step.progress().copied_bytes,fixed+1);assert_eq!(h.last_heap,(0,0));h.inspect(|p|{let cursor=p.entry.as_ref().unwrap().fields.update.as_ref().unwrap();assert_eq!(cursor.last_compared_bytes(),1);assert_eq!(cursor.state.left_byte,Some(row["firstByte"].as_u64().unwrap()as u8))});
 drain(&mut h,&mut source)
}
#[path="➖remove/🧪️tests/🦀️.rs"]mod removal_laws;
