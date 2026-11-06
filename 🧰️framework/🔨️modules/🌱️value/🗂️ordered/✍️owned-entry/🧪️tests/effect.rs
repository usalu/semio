//! 🧪️ Public production entry/update/typed retirement share one real effect at every authored cancellation prefix.
use semio_framework_value_ordered::{OriginalOwnedEntry,OrderedMap,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
use semio_framework_value_retirement::{RetireOwned,retirement::controlled::ControlledRetirement};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
struct Bank{grant:RetainedCloneGrant,work:u64,born:usize,freed:usize}
impl Bank{fn post(&mut self,p:RetainedCloneProgress,work:u64){assert!(p.fits(self.grant));self.grant.maximum_items-=p.copied_items;self.grant.maximum_copy_bytes-=p.copied_bytes;self.grant.maximum_capacity_bytes-=p.retained_capacity_bytes;self.grant.maximum_release_bytes-=p.released_bytes;self.work=self.work.checked_sub(work).expect("same original finite CPU bank");self.born+=p.retained_capacity_bytes;self.freed+=p.released_bytes;}}
fn close<T:RetireOwned>(value:T,bank:&mut Bank){let(mut owner,heap)=observe(||ControlledRetirement::retain(value));assert_eq!(heap,(0,0));let mut actual=None;
 for _ in 0..10000{if owner.terminal_is_empty(){break}let d=owner.next_demand(4096).unwrap();let units=owner.next_computational_work(4096).unwrap();assert!(units<=bank.work);let g=grant(d);assert!(d.copy_bytes<=bank.grant.maximum_copy_bytes&&d.capacity_bytes<=bank.grant.maximum_capacity_bytes&&d.release_bytes<=bank.grant.maximum_release_bytes&&d.depth<=bank.grant.maximum_depth);assert!(actual.is_none());
 let(zero,heap)=observe(||owner.step(&mut RetainedAllocationEffect::new(g,0,&mut actual)));assert_eq!(heap,(0,0));assert_eq!(zero.unwrap().progress(),Default::default());assert_eq!(owner.last_turn_work(),0);assert!(actual.is_none());assert_eq!(owner.next_demand(4096).unwrap(),d);
 let(result,heap)=observe(||owner.step(&mut RetainedAllocationEffect::new(g,units,&mut actual)));let p=result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));if let Some(actual)=actual.take(){assert_eq!(actual,heap.0)}else{assert_eq!(heap.0,0)}bank.post(p,owner.last_turn_work());
 }assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1,(0,0));}
#[test]
fn public_entry_prefixes_keep_the_real_marker_and_close_every_original_owner(){
 let(_,probe)=observe(||{let value=std::hint::black_box(Vec::<u8>::with_capacity(73));std::hint::black_box(value.as_ptr());drop(value)});assert_eq!(probe,(73,73));
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for cut in law["cancelAfter"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap())).chain([None]){
 let stock=&law["stock"];let mut bank=Bank{grant:RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize},work:stock["fuel"].as_u64().unwrap(),born:0,freed:0};
 let(mut entry,initial)=observe(||{let base:OrderedMap<u64>=row["base"].as_array().unwrap().iter().map(|p|(p[0].as_str().unwrap().to_owned(),p[1].as_u64().unwrap())).collect();OriginalOwnedEntry::new(base,row["key"].as_str().unwrap().to_owned(),row["value"].as_u64().unwrap())});
 let mut output=None;let mut actual=None;
 for turn in 0..law["maximumTurns"].as_u64().unwrap(){if cut==Some(turn){break}let d=entry.next_demands(4096).unwrap();let units=entry.advance_work_demand(4096).unwrap();assert!(units<=bank.work);let g=grant(d);let paused=RetainedCloneGrant{maximum_items:0,..g};
 let(result,heap)=observe(||entry.advance(&mut RetainedAllocationEffect::new(paused,units,&mut actual)));assert_eq!(heap,(0,0));assert_eq!(result.work,0);assert!(result.result.unwrap().is_none());assert!(actual.is_none());assert_eq!(entry.next_demands(4096).unwrap(),d);
 let(result,heap)=observe(||entry.advance(&mut RetainedAllocationEffect::new(g,units,&mut actual)));let p=entry.step_progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));if let Some(actual)=actual.take(){assert_eq!(actual,heap.0)}else{assert_eq!(heap.0,0)}bank.post(p,result.work);output=result.result.unwrap();if output.is_some(){break}assert!(turn+1<law["maximumTurns"].as_u64().unwrap());
 }
 if cut.is_none(){let actual:Vec<_>=output.as_ref().unwrap().iter().map(|(key,value)|(key.as_str(),*value)).collect();let expected:Vec<_>=row["expected"].as_array().unwrap().iter().map(|p|(p[0].as_str().unwrap(),p[1].as_u64().unwrap())).collect();assert_eq!(actual,expected)}
 close(entry,&mut bank);if let Some(output)=output{close(output,&mut bank)}assert_eq!(initial.0+bank.born,initial.1+bank.freed,"every actual cold original and selected native allocation must return");
 }}
}
