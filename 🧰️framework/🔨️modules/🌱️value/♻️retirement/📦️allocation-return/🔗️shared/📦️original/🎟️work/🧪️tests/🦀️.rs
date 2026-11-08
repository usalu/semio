//! 🎟️ The complete original shared admission corpus uses one actual finite Source bank.
use semio_framework_value_allocation_return::original::SharedOriginalStorage as SharedOwner;
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet,RetainedWorkPoll};
use semio_framework_value_testing_allocator::{observe_retirement_allocations as observe,refuse_one_allocation_on_this_thread as refuse};
fn stock(law:&serde_json::Value)->RetainedCloneGrant{let s=&law["stock"];RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize}}
fn run(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,expected:u64,body:impl FnOnce(&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->(Result<RetainedCloneStep,ValueError>,Option<usize>,(usize,usize)){
 let((poll,marker),heap)=observe(||bank.run(grant,maximum,|effect|RetainedWorkPoll::Complete(body(effect))));let RetainedWorkPoll::Complete(turn)=poll else{panic!("closed native shared primitive became pending")};assert_eq!(turn.work,u128::from(expected));let progress=turn.result.as_ref().map_or_else(|e|e.retained_progress(),|step|step.progress());assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(grant));if let Some(marker)=marker{assert_eq!(marker,heap.0)}else{assert_eq!(heap.0,0)}assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,grant,heap);(turn.result,marker,heap)
}
fn close(bank:&mut EffectBank,mut owner:SharedOwner<String>,pointer:*const u8,law:&serde_json::Value){
 let mut original=None;let d=owner.retire_into_demand().unwrap();let grant=bank.loan(1,d.copy_bytes);
 let result=run(bank,RetainedCloneGrant{maximum_items:0,..grant},4,law["deniedWork"].as_u64().unwrap(),|effect|owner.elect_native_into(&mut original,effect));assert_eq!(result.0.unwrap().progress(),Default::default());assert_eq!(owner.get().as_ptr(),pointer);assert!(original.is_none());
 run(bank,grant,4,law["electionWork"].as_u64().unwrap(),|effect|owner.elect_native_into(&mut original,effect)).0.unwrap();assert!(owner.terminal_is_empty());
 let result=run(bank,Default::default(),0,law["cachedWork"].as_u64().unwrap(),|effect|owner.elect_native_into(&mut None,effect));assert_eq!(result.0.unwrap().progress(),Default::default());
 let mut output=None;let elected=original.as_mut().unwrap();let d=elected.take_value_demand().unwrap();let grant=bank.loan(1,d.copy_bytes);run(bank,grant,4,law["releaseWork"].as_u64().unwrap(),|effect|{let turn=elected.take_value_step(&mut output,effect);RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Complete(p.unwrap_or_default())),work:turn.work}}).0.unwrap();assert!(elected.terminal_is_empty());assert_eq!(output.as_ref().unwrap().as_ptr(),pointer);assert_eq!(serde_json::to_value(output.as_ref().unwrap()).unwrap(),law["value"]);
 let release=output.as_ref().unwrap().capacity();let bytes=size_of::<Option<String>>();let grant=bank.loan(1,bytes);run(bank,grant,1,law["payloadWork"].as_u64().unwrap(),|effect|{assert!(effect.performed(1).unwrap());drop(output.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,released_bytes:release,..Default::default()})),work:1}}).0.unwrap();assert!(output.is_none());
}
#[test]
fn actual_ordered_shared_birth_null_and_release_keep_input_and_exact_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut bank=EffectBank::new(Wallet::new(stock(&law),law["stock"]["fuel"].as_u64().unwrap()));let mut input=Some(law["value"].as_str().unwrap().to_owned());let pointer=input.as_ref().unwrap().as_ptr();let mut output=None;let d=SharedOwner::<String>::source_admission_demand().unwrap();let grant=bank.loan(1,d.copy_bytes);
 for grant in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:SharedOwner::<String>::allocation_bytes()-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
  let result=run(&mut bank,grant,4,law["deniedWork"].as_u64().unwrap(),|effect|SharedOwner::admit_from(&mut input,&mut output,effect));assert_eq!(result.0.unwrap().progress(),Default::default());assert_eq!(input.as_ref().unwrap().as_ptr(),pointer);assert!(output.is_none());
 }
 let mut attempts=0;let result=run(&mut bank,grant,4,law["nullWork"].as_u64().unwrap(),|effect|{let(turn,count)=refuse(||SharedOwner::admit_from(&mut input,&mut output,effect));attempts=count;turn});assert!(result.0.is_err());assert_eq!(attempts,1);assert_eq!(result.1,Some(0));assert_eq!(input.as_ref().unwrap().as_ptr(),pointer);assert!(output.is_none());
 let result=run(&mut bank,grant,4,law["admitWork"].as_u64().unwrap(),|effect|SharedOwner::admit_from(&mut input,&mut output,effect));let progress=result.0.unwrap().progress();assert_eq!(progress.retained_capacity_bytes,SharedOwner::<String>::allocation_bytes());assert!(input.is_none());let owner=output.take().unwrap();assert_eq!(owner.get().as_ptr(),pointer);assert_eq!(serde_json::to_value(owner.get()).unwrap(),law["value"]);close(&mut bank,owner,pointer,&law);
}
#[test]
fn original_ordered_shared_slot_denials_keep_the_same_source(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 for axis in law["slotDenials"].as_array().unwrap(){
  let mut input=Some(law["value"].as_str().unwrap().to_owned());let pointer=input.as_ref().unwrap().as_ptr();let mut target=None;let d=SharedOwner::<String>::source_admission_demand().unwrap();let mut bank=EffectBank::new(Wallet::new(stock(&law),law["stock"]["fuel"].as_u64().unwrap()));let mut g=bank.loan(1,d.copy_bytes);
  match axis.as_str().unwrap(){"items"=>g.maximum_items=0,"copy"=>g.maximum_copy_bytes-=1,"capacity"=>g.maximum_capacity_bytes=d.capacity_bytes-1,"depth"=>g.maximum_depth=d.depth-1,_=>unreachable!()}
  let result=run(&mut bank,g,4,0,|effect|SharedOwner::admit_from(&mut input,&mut target,effect));assert_eq!(result.0.unwrap().progress(),Default::default());assert_eq!(input.as_ref().unwrap().as_ptr(),pointer);assert!(target.is_none());assert!(result.1.is_none());assert_eq!(result.2,(0,0));
 }
 let mut input=Some(law["value"].as_str().unwrap().to_owned());let pointer=input.as_ref().unwrap().as_ptr();let mut target=None;let d=SharedOwner::<String>::source_admission_demand().unwrap();let mut bank=EffectBank::new(Wallet::new(stock(&law),law["stock"]["fuel"].as_u64().unwrap()));let g=bank.loan(1,d.copy_bytes);let mut attempts=0;
 let result=run(&mut bank,g,4,1,|effect|{let(turn,count)=refuse(||SharedOwner::admit_from(&mut input,&mut target,effect));attempts=count;turn});assert!(result.0.is_err());assert_eq!(attempts,1);assert_eq!(result.1,Some(0));assert_eq!(input.as_ref().unwrap().as_ptr(),pointer);assert!(target.is_none());
 let result=run(&mut bank,g,4,law["slotWork"].as_u64().unwrap(),|effect|SharedOwner::admit_from(&mut input,&mut target,effect));let progress=result.0.unwrap().progress();assert!(input.is_none());assert_eq!(target.as_ref().unwrap().get().as_ptr(),pointer);assert_eq!(progress.copied_bytes,d.copy_bytes);assert_eq!(result.1,Some(d.capacity_bytes));close(&mut bank,target.take().unwrap(),pointer,&law);
}
