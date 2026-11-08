//! 🧪️ Public production entry/update/typed retirement share one real effect at every authored cancellation prefix.
use semio_framework_value_ordered::{OriginalOwnedEntry,OrderedMap,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand};
use semio_framework_value_retirement::{RetireOwned,retirement::controlled::ControlledRetirement};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet as BankWallet};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn done<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous original action stayed pending")}}
fn run<T,E>(bank:&mut EffectBank,g:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->(RetainedWorkOutcome<T,E>,Option<usize>){let(poll,actual)=bank.run(g,maximum,operation);assert_ne!(bank.take_admitted(),Some(false));(done(poll),actual)}
struct Ledger{born:usize,freed:usize}
impl Ledger{fn post(&mut self,bank:&mut EffectBank,p:RetainedCloneProgress,grant:RetainedCloneGrant,heap:(usize,usize)){bank.post(p,grant,heap);self.born+=p.retained_capacity_bytes;self.freed+=p.released_bytes;}}
fn close<T:RetireOwned>(value:T,bank:&mut EffectBank,ledger:&mut Ledger){let(mut owner,heap)=observe(||ControlledRetirement::retain(value));assert_eq!(heap,(0,0));
 for _ in 0..10000{if owner.terminal_is_empty(){break}let d=owner.next_demand(4096).unwrap();let units=owner.next_computational_work(4096).unwrap();let stock=bank.wallet().remaining();assert!(units<=bank.wallet().remaining_work());let g=grant(d);assert!(d.copy_bytes<=stock.maximum_copy_bytes&&d.capacity_bytes<=stock.maximum_capacity_bytes&&d.release_bytes<=stock.maximum_release_bytes&&d.depth<=stock.maximum_depth);assert_eq!(bank.actual(),None);
  let((zero,actual),heap)=observe(||run(bank,g,0,|effect|owner.step(effect)));assert_eq!(heap,(0,0));assert_eq!(zero.work,0);assert_eq!(zero.result.unwrap().progress(),Default::default());assert_eq!(owner.last_turn_work(),0);assert!(actual.is_none());assert_eq!(owner.next_demand(4096).unwrap(),d);
  let((result,actual),heap)=observe(||run(bank,g,units,|effect|owner.step(effect)));let p=result.result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));if let Some(actual)=actual{assert_eq!(actual,heap.0)}else{assert_eq!(heap.0,0)}assert_eq!(result.work,owner.last_turn_work());ledger.post(bank,p,g,heap);
 }assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1,(0,0));}
#[test]
fn public_entry_prefixes_keep_the_real_marker_and_close_every_original_owner(){
 let(_,probe)=observe(||{let value=std::hint::black_box(Vec::<u8>::with_capacity(73));std::hint::black_box(value.as_ptr());drop(value)});assert_eq!(probe,(73,73));
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for cut in law["cancelAfter"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap())).chain([None]){
 let stock=&law["stock"];let mut bank=EffectBank::new(BankWallet::from_profile(stock));let mut ledger=Ledger{born:0,freed:0};
 let(mut entry,initial)=observe(||{let base:OrderedMap<u64>=row["base"].as_array().unwrap().iter().map(|p|(p[0].as_str().unwrap().to_owned(),p[1].as_u64().unwrap())).collect();OriginalOwnedEntry::new(base,row["key"].as_str().unwrap().to_owned(),row["value"].as_u64().unwrap())});
 let mut output=None;
 for turn in 0..law["maximumTurns"].as_u64().unwrap(){if cut==Some(turn){break}let d=entry.next_demands(4096).unwrap();let units=entry.advance_work_demand(4096).unwrap();assert!(units<=bank.wallet().remaining_work());let g=grant(d);let paused=RetainedCloneGrant{maximum_items:0,..g};
 let((result,actual),heap)=observe(||run(&mut bank,paused,units,|effect|entry.advance(effect)));assert_eq!(heap,(0,0));assert_eq!(result.work,0);assert!(result.result.unwrap().is_none());assert!(actual.is_none());assert_eq!(entry.next_demands(4096).unwrap(),d);
 let((result,actual),heap)=observe(||run(&mut bank,g,units,|effect|entry.advance(effect)));let p=entry.step_progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));if let Some(actual)=actual{assert_eq!(actual,heap.0)}else{assert_eq!(heap.0,0)}ledger.post(&mut bank,p,g,heap);output=result.result.unwrap();if output.is_some(){break}assert!(turn+1<law["maximumTurns"].as_u64().unwrap());
 }
 if cut.is_none(){let actual:Vec<_>=output.as_ref().unwrap().iter().map(|(key,value)|(key.as_str(),*value)).collect();let expected:Vec<_>=row["expected"].as_array().unwrap().iter().map(|p|(p[0].as_str().unwrap(),p[1].as_u64().unwrap())).collect();assert_eq!(actual,expected)}
 close(entry,&mut bank,&mut ledger);if let Some(output)=output{close(output,&mut bank,&mut ledger)}assert_eq!(initial.0+ledger.born,initial.1+ledger.freed,"every actual cold original and selected native allocation must return");
 }}
}
