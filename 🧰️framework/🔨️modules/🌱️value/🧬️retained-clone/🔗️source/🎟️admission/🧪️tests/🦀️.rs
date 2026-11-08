//! 🧪️ Staged source births preserve typed originals across real nulls, denial and cancellation.
use super::*;
use crate::allocation_testing::{observe_retirement_allocations,refuse_one_allocation_on_this_thread};
use crate::source_law_bank::{observed,settled,CPU};
use crate::{RetainedCloneGrant as G,RetainedCloneProgress as P,RetainedCloneStep as S,ValueRefusalKind};
fn fixture()->serde_json::Value{let f:serde_json::Value=serde_json::from_str(include_str!("../case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../case.schema.json")).unwrap();for(k,v)in f.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}f}
struct Bank{left:G,fuel:u128,actual:Option<usize>}
impl Bank{
 fn new(f:&serde_json::Value)->Self{let s=&f["stock"];Self{left:G{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},actual:None,fuel:u128::from(s["fuel"].as_u64().unwrap())}}
 fn grant(&self)->G{G{maximum_items:self.left.maximum_items.min(1),..self.left}}
 fn post<E:std::fmt::Debug>(&mut self,turn:crate::RetainedWorkOutcome<S,E>,heap:(usize,usize))->Result<S,E>{if let Some(actual)=self.actual.take(){assert_eq!(actual,heap.0)}assert!(turn.work<=self.fuel);self.fuel-=turn.work;match turn.result{Ok(step)=>{let p=step.progress();assert!(p.fits(self.grant()));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));self.left.maximum_items-=p.copied_items;self.left.maximum_copy_bytes-=p.copied_bytes;self.left.maximum_capacity_bytes-=p.retained_capacity_bytes;self.left.maximum_release_bytes-=p.released_bytes;Ok(step)},Err(error)=>{assert_eq!(heap,(0,0));Err(error)}}}
}

#[test]
fn source_wrappers_forward_actual_allocation_markers_and_refuse_unposted_reentry(){
 let f=fixture();
 for text in f["texts"].as_array().unwrap(){
  let value=text.as_str().unwrap().to_owned();let pointer=value.as_ptr();let mut staged=crate::StagedSourceAdmission::<String,String>::owned(value);let mut output=None;let mut bank=Bank::new(&f);
  for phase in f["phases"].as_array().unwrap(){
   let grant=bank.grant();let(turn,heap)=observed(grant,CPU,&mut bank.actual,|effect|staged.advance(&mut output,effect));let allocating=f["allocationPhases"].as_array().unwrap().contains(phase);
   assert_eq!(bank.actual,allocating.then_some(heap.0));assert!(output.is_none());
   if allocating{let marker=bank.actual;let demand=staged.next_demand().unwrap();let(denied,effects)=observed(grant,CPU,&mut bank.actual,|effect|staged.advance(&mut output,effect));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(effects,(0,0));assert_eq!(bank.actual,marker);assert_eq!(staged.next_demand().unwrap(),demand);assert!(output.is_none());}
   bank.post(turn,heap).unwrap();assert!(bank.actual.is_none());
  }
  let grant=bank.grant();let(turn,heap)=observed(grant,CPU,&mut bank.actual,|effect|staged.advance(&mut output,effect));assert!(bank.actual.is_none());bank.post(turn,heap).unwrap();assert!(staged.terminal_is_empty());let source=output.unwrap();assert_eq!(source.try_borrow().unwrap().get().as_ptr(),pointer);close_value(source,&mut bank,&f);
  let value=text.as_str().unwrap().to_owned();let pointer=value.as_ptr();let mut authority=crate::RetainedCloneBorrowAuthority::<String>::admission(value);let mut bank=Bank::new(&f);
  for phase in f["phases"].as_array().unwrap(){let grant=bank.grant();let(turn,heap)=observed(grant,CPU,&mut bank.actual,|effect|authority.advance(effect));assert_eq!(bank.actual,f["allocationPhases"].as_array().unwrap().contains(phase).then_some(heap.0));bank.post(turn,heap).unwrap();}
  let mut output=None;let grant=bank.grant();let(turn,heap)=observe_retirement_allocations(||authority.take_output(&mut output,grant));bank.post(turn,heap).unwrap();assert!(authority.terminal_is_empty());let mut output=output.unwrap();assert_eq!(output.try_authority().unwrap().as_ptr(),pointer);
  for _ in 0..f["maximumTurns"].as_u64().unwrap(){if output.terminal_is_empty(){break}let demand=output.next_close_demand(4096).unwrap();let grant=G{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(turn,heap)=observed(grant,CPU,&mut None,|effect|settled(output.close_step(effect)));bank.post(turn,heap).unwrap();}assert!(output.terminal_is_empty());
 }
}
fn close_value<T:crate::retirement::RetireOwned>(value:T,bank:&mut Bank,f:&serde_json::Value){let mut close=crate::retirement::controlled::ControlledRetirement::retain(value);for _ in 0..f["maximumTurns"].as_u64().unwrap(){if close.terminal_is_empty(){break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut None,|effect|settled(close.step(effect)));bank.post(turn,heap).unwrap();}assert!(close.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(close)).1,(0,0));}
fn cancel(owner:&mut RetainedCloneSourceAdmission<String,String,()>,bank:&mut Bank,pointer:*const u8,f:&serde_json::Value){let mut returned=None;for _ in 0..f["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut None,|effect|owner.return_inputs(&mut returned,effect));bank.post(turn,heap).unwrap();}assert!(owner.terminal_is_empty());let(value,())=returned.take().unwrap();assert_eq!(value.as_ptr(),pointer);let actual=serde_json::to_value(&value).unwrap();assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&value).unwrap()).unwrap(),value);assert_eq!(actual,value);close_value(value,bank,f);assert_eq!(observe_retirement_allocations(||drop(returned)).1,(0,0));}
#[test]
fn every_actual_source_allocation_null_keeps_the_same_inputs_and_reports_direct_attempt_work(){
 let f=fixture();for text in f["texts"].as_array().unwrap(){for phase in ["owner","payload","lease"]{let value=text.as_str().unwrap().to_owned();let pointer=value.as_ptr();let mut owner=RetainedCloneSourceAdmission::<String,String,()>::new(value,());let mut bank=Bank::new(&f);
 for _ in 0..f["maximumTurns"].as_u64().unwrap(){if owner.phase()==phase{break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut bank.actual,|effect|owner.advance(effect));bank.post(turn,heap).unwrap();}
 assert_eq!(owner.phase(),phase);let g=bank.grant();let((turn,count),heap)=observed(g,CPU,&mut bank.actual,|effect|refuse_one_allocation_on_this_thread(||owner.advance(effect)));assert_eq!(count,1);assert_eq!(bank.actual,Some(f["allocationMarker"]["failed"].as_u64().unwrap()as usize));assert_eq!(turn.work,u128::from(f["nullWork"][phase].as_u64().unwrap()));assert_eq!(bank.post(turn,heap).unwrap_err().kind,ValueRefusalKind::AllocationFailed);assert_eq!(owner.phase(),phase);cancel(&mut owner,&mut bank,pointer,&f);assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));
 }}
}
#[test]
fn each_partial_source_cancels_under_the_original_bank_and_denied_turns_are_unchanged(){
 let f=fixture();for text in f["texts"].as_array().unwrap(){for cut in f["cancelCuts"].as_array().unwrap(){let value=text.as_str().unwrap().to_owned();let pointer=value.as_ptr();let mut owner=RetainedCloneSourceAdmission::<String,String,()>::new(value,());let mut bank=Bank::new(&f);
 for _ in 0..cut.as_u64().unwrap(){let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut bank.actual,|effect|owner.advance(effect));bank.post(turn,heap).unwrap();}
 let before=owner.phase();let d=owner.next_demand().unwrap();for axis in ["items","capacity","depth"]{if axis=="capacity"&&d.capacity_bytes==0{continue}let mut g=bank.grant();match axis{"items"=>g.maximum_items=0,"capacity"=>g.maximum_capacity_bytes=d.capacity_bytes-1,_=>g.maximum_depth=0}let(turn,heap)=observed(g,CPU,&mut bank.actual,|effect|owner.advance(effect));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),P::default());assert_eq!(heap,(0,0));assert_eq!(owner.phase(),before);assert!(bank.actual.is_none());}
 cancel(&mut owner,&mut bank,pointer,&f);assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));
 }}
}
#[test]
fn complete_source_publication_is_separate_and_original_handback_closes_every_actual_backing(){
 let f=fixture();for text in f["texts"].as_array().unwrap(){let value=text.as_str().unwrap().to_owned();let pointer=value.as_ptr();let mut owner=RetainedCloneSourceAdmission::<String,String,()>::new(value,());let mut bank=Bank::new(&f);
 for _ in 0..f["maximumTurns"].as_u64().unwrap(){if owner.phase()=="ready"{break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut bank.actual,|effect|owner.advance(effect));bank.post(turn,heap).unwrap();}assert_eq!(owner.phase(),"ready");let mut source=None;let(turn,heap)=observe_retirement_allocations(||owner.take_output(&mut source,G{maximum_items:0,..bank.grant()}));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),P::default());assert!(source.is_none());assert_eq!(heap,(0,0));
 let g=bank.grant();let(turn,heap)=observe_retirement_allocations(||owner.take_output(&mut source,g));bank.post(turn,heap).unwrap();assert!(owner.terminal_is_empty());let mut source=source.unwrap();assert_eq!(source.borrow().get().as_ptr(),pointer);assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),*text);let mut output=None;
 for _ in 0..f["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut None,|effect|settled(source.take_authority(effect)));let work=turn.work;let mapped=turn.result.map(|step|match step{crate::RetainedCloneSourceTake::Pending(p)=>S::Progress(p),crate::RetainedCloneSourceTake::Ready(value,p)=>{output=Some(value);S::Complete(p)}});bank.post(crate::RetainedWorkOutcome{result:mapped,work},heap).unwrap();}
 assert!(source.terminal_is_empty());assert_eq!(output.as_ref().unwrap().as_ptr(),pointer);assert_eq!(observe_retirement_allocations(||drop((source,owner))).1,(0,0));close_value(output.unwrap(),&mut bank,&f);
 }
}
#[test]
fn cancelling_a_prepared_typed_payload_returns_both_distinct_original_allocations(){
 let f=fixture();for cut in f["cancelCuts"].as_array().unwrap(){let value="source λ".to_owned();let authority="authority 🌿".to_owned();let pointers=(value.as_ptr(),authority.as_ptr());let mut owner=RetainedCloneSourceAdmission::<String,String,String>::new(value,authority);let mut bank=Bank::new(&f);for _ in 0..cut.as_u64().unwrap(){let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut bank.actual,|effect|owner.advance(effect));bank.post(turn,heap).unwrap();}
 let mut returned=None;for _ in 0..f["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut None,|effect|owner.return_inputs(&mut returned,effect));bank.post(turn,heap).unwrap();}assert!(owner.terminal_is_empty());let(value,authority)=returned.unwrap();assert_eq!((value.as_ptr(),authority.as_ptr()),pointers);assert_eq!(serde_json::to_string(&authority).unwrap(),"\"authority 🌿\"");close_value(value,&mut bank,&f);close_value(authority,&mut bank,&f);assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));
 }
}

/// 📤️ A published source is visible as such, returning inputs before it is taken is refused, and taking it first lets the owner close.
#[test]
fn a_published_source_is_reported_and_must_be_taken_before_inputs_return(){
 let f=fixture();let value=f["texts"][0].as_str().unwrap().to_owned();let pointer=value.as_ptr();let mut owner=RetainedCloneSourceAdmission::<String,String,()>::new(value,());let mut bank=Bank::new(&f);
 assert!(owner.phase()!="ready");
 for _ in 0..f["maximumTurns"].as_u64().unwrap(){if owner.phase()=="ready"{break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut bank.actual,|effect|owner.advance(effect));bank.post(turn,heap).unwrap();}
 assert!(owner.phase()=="ready");
 let mut returned=None;let refused=observed(bank.grant(),CPU,&mut None,|effect|owner.return_inputs(&mut returned,effect)).0;assert_eq!(refused.result.unwrap_err().kind,ValueRefusalKind::InvalidValue);assert!(returned.is_none());assert!(owner.phase()=="ready");
 let mut source=None;let g=bank.grant();let(turn,heap)=observe_retirement_allocations(||owner.take_output(&mut source,g));bank.post(turn,heap).unwrap();
 assert!(owner.phase()!="ready");assert!(owner.terminal_is_empty());let mut source=source.unwrap();assert_eq!(source.borrow().get().as_ptr(),pointer);
 let mut output=None;for _ in 0..f["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break}let g=bank.grant();let(turn,heap)=observed(g,CPU,&mut None,|effect|settled(source.take_authority(effect)));let work=turn.work;let mapped=turn.result.map(|step|match step{crate::RetainedCloneSourceTake::Pending(p)=>S::Progress(p),crate::RetainedCloneSourceTake::Ready(value,p)=>{output=Some(value);S::Complete(p)}});bank.post(crate::RetainedWorkOutcome{result:mapped,work},heap).unwrap();}
 assert!(source.terminal_is_empty());assert_eq!(output.as_ref().unwrap().as_ptr(),pointer);drop((source,owner));close_value(output.unwrap(),&mut bank,&f);
}
