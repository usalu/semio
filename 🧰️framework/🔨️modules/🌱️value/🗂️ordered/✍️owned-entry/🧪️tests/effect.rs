//! 🧪️ Original entry prefixes share one real Source bank through update, output and all retirement.
use semio_framework_value_ordered::*;
use semio_framework_value_retained_clone::RetainedCloneClose;
use semio_framework_value_control_contract::{OriginalEffectProducer,OriginalEffectAuthority,OriginalEffectAction,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,ValueError,RetainedWorkPoll,RetainedWorkOutcome};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use std::mem::{ManuallyDrop,size_of};
#[path="effect-bank.rs"]mod bank;
use bank::{Harness,Op,grant};
#[test]
fn public_entry_prefixes_keep_the_real_marker_and_close_every_original_owner(){
 let(_,probe)=observe(||{let value=std::hint::black_box(Vec::<u8>::with_capacity(73));std::hint::black_box(value.as_ptr());drop(value)});assert_eq!(probe,(73,73));
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for cut in law["cancelAfter"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap())).chain([None]){
  let s=&law["stock"];let stock=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let mut bank=EffectBank::new(Wallet::new(stock,s["fuel"].as_u64().unwrap()));
  let(entry,initial)=observe(||{let base:OrderedMap<u64>=row["base"].as_array().unwrap().iter().map(|p|(p[0].as_str().unwrap().to_owned(),p[1].as_u64().unwrap())).collect();OriginalOwnedEntry::new(base,row["key"].as_str().unwrap().to_owned(),row["value"].as_u64().unwrap())});let mut entry=Some(entry);let mut output=None;let mut entry_close=RetainedCloneClose::default();let mut output_close=RetainedCloneClose::default();let mut h=Harness::new(&mut entry,&mut output,&mut entry_close,&mut output_close,&bank);
  for turn in 0..law["maximumTurns"].as_u64().unwrap(){if cut==Some(turn){break}let(d,units)=h.quote(Op::Advance);let g=grant(d);let paused=RetainedCloneGrant{maximum_items:0,..g};let remaining=bank.wallet().remaining_work();let step=h.run(&mut bank,Op::Advance,paused,units);assert_eq!(step.progress(),Default::default());assert_eq!(bank.wallet().remaining_work(),remaining);assert_eq!(h.quote(Op::Advance).0,d);h.run(&mut bank,Op::Advance,g,units);if h.inspect(|p|p.output.is_some()){break}assert!(turn+1<law["maximumTurns"].as_u64().unwrap());}
  if cut.is_none(){h.inspect(|p|{let actual:Vec<_>=p.output.as_ref().unwrap().iter().map(|(key,value)|(key.as_str(),*value)).collect();let expected:Vec<_>=row["expected"].as_array().unwrap().iter().map(|p|(p[0].as_str().unwrap(),p[1].as_u64().unwrap())).collect();assert_eq!(actual,expected)})}
  for op in [Op::EntryStart,Op::EntryClose,Op::OutputStart,Op::OutputClose]{for _ in 0..10000{let done=h.inspect(|p|match op{Op::EntryStart=>p.entry.is_none(),Op::EntryClose=>p.entry_close.is_empty(),Op::OutputStart=>p.output.is_none(),Op::OutputClose=>p.output_close.is_empty(),_=>unreachable!()});if done{break}let(d,units)=h.quote(op);let g=grant(d);let remaining=bank.wallet().remaining_work();let step=h.run(&mut bank,op,g,0);assert_eq!(step.progress(),Default::default());assert_eq!(bank.wallet().remaining_work(),remaining);assert_eq!(h.quote(op).0,d);h.run(&mut bank,op,g,units);} }
  assert!(h.inspect(|p|p.empty()));h.release(&mut bank);assert_eq!(initial.0+h.born,initial.1+h.freed,"every actual cold original and selected native allocation must return");
 }}
}
