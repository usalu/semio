//! 🧪️ Real original vectors retain payload pointers and close every quoted physical allocation.
#[path="../../../♻️retirement/🧪️tests/⏱️work/🦀️.rs"]mod close_work_policy;
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::retirement::controlled::ControlledRetirement;
#[derive(crate::RetireOwned)]#[value(crate="crate")]
struct Row{ordinal:usize,text:String}
impl Clone for Row{fn clone(&self)->Self{panic!("original concatenation must never clone payload rows")}}
struct Bank{grant:RetainedCloneGrant,fuel:u64,turns:usize,maximum:usize}
thread_local!{static BANK:std::cell::RefCell<Option<Bank>>=const{std::cell::RefCell::new(None)};}
fn begin_bank(){let law:serde_json::Value=serde_json::from_str(include_str!("🎟️work/🔣️.json")).unwrap();let s=&law["stock"];BANK.with(|bank|{assert!(bank.borrow().is_none());*bank.borrow_mut()=Some(Bank{grant:RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},fuel:s["fuel"].as_u64().unwrap(),turns:0,maximum:s["maximumTurns"].as_u64().unwrap()as usize});});}
fn end_bank(){BANK.with(|bank|{assert!(bank.borrow_mut().take().is_some());});}
fn cap(g:RetainedCloneGrant)->RetainedCloneGrant{BANK.with(|bank|{let bank=bank.borrow();let b=bank.as_ref().unwrap().grant;RetainedCloneGrant{maximum_items:g.maximum_items.min(b.maximum_items),maximum_copy_bytes:g.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:g.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:g.maximum_depth.min(b.maximum_depth)}})}
fn grant(d:RetirementDemand)->RetainedCloneGrant{let g=cap(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth});assert!(retained_clone_grant_admits(g,d),"finite Concat stock depleted before its concrete demand");g}
trait Receipt{fn receipt(&self)->RetainedCloneProgress;}
impl Receipt for RetainedCloneStep{fn receipt(&self)->RetainedCloneProgress{self.progress()}}
impl<T> Receipt for(T,RetainedCloneProgress){fn receipt(&self)->RetainedCloneProgress{self.1}}
impl<T:Receipt> Receipt for Option<T>{fn receipt(&self)->RetainedCloneProgress{self.as_ref().map_or(Default::default(),Receipt::receipt)}}
fn paid<T:Receipt>(turn:RetainedWorkOutcome<T>)->T{let p=match &turn.result{Ok(value)=>value.receipt(),Err(error)=>error.retained_progress()};BANK.with(|bank|{let mut bank=bank.borrow_mut();let bank=bank.as_mut().unwrap();assert!(p.fits(bank.grant));bank.grant.maximum_items=bank.grant.maximum_items.checked_sub(p.copied_items).unwrap();bank.grant.maximum_copy_bytes=bank.grant.maximum_copy_bytes.checked_sub(p.copied_bytes).unwrap();bank.grant.maximum_capacity_bytes=bank.grant.maximum_capacity_bytes.checked_sub(p.retained_capacity_bytes).unwrap();bank.grant.maximum_release_bytes=bank.grant.maximum_release_bytes.checked_sub(p.released_bytes).unwrap();bank.fuel=bank.fuel.checked_sub(u64::try_from(turn.work).unwrap()).unwrap();bank.turns+=1;assert!(bank.turns<=bank.maximum);});turn.result.unwrap()}
fn close<T:RetireOwned>(owner:&mut ControlledRetirement<T>,g:RetainedCloneGrant)->RetainedCloneStep{let turn=crate::effect::settle(g,|effect|owner.step(effect));paid(RetainedWorkOutcome{result:turn.result.map_err(|failure|failure.into_value_error()),work:turn.work})}
fn denied(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut grants=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes>0{grants.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g});}if d.capacity_bytes>0{grants.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g});}if d.release_bytes>0{grants.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g});}if d.depth>0{grants.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g});}grants}
fn exact<T>(run:impl FnOnce()->T,progress:impl FnOnce(&T)->RetainedCloneProgress)->T{let(value,heap)=crate::observe_retirement_allocations(run);let progress=progress(&value);assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));value}
/// 🪢️ Settles one concat turn inside one actual original effect and keeps the value-error receipt shape `paid` bills.
fn advance_turn(owner:&mut OriginalVectorConcat<Row>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{let turn=crate::effect::settle(g,|effect|owner.advance(effect));RetainedWorkOutcome{result:turn.result.map_err(|failure|failure.into_value_error()),work:turn.work}}
fn close_turn(owner:&mut OriginalVectorConcat<Row>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{let turn=crate::effect::settle(g,|effect|owner.close_step(effect));RetainedWorkOutcome{result:turn.result.map_err(|failure|failure.into_value_error()),work:turn.work}}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();let mut work=close_work_policy::Work::new();while !owner.terminal_is_empty(){let d=owner.next_demand(4096).unwrap();assert!(d.copy_bytes<=4096);let g=cap(RetainedCloneGrant{maximum_copy_bytes:4096,..grant(d)});let step=exact(||close(&mut owner,g),|step|step.progress());assert!(step.progress().fits(g));work.observe(step.progress()==Default::default()&&!owner.terminal_is_empty()&&owner.next_demand(4096).unwrap()==d);}}
fn check(chunks:Vec<Vec<String>>,expected:Vec<String>,cut:Option<usize>){
 begin_bank();
 let mut ordinal=0;let mut inputs:Vec<Vec<Row>>=chunks.into_iter().map(|rows|rows.into_iter().map(|text|{let row=Row{ordinal,text};ordinal+=1;row}).collect()).collect();
 let pointers:Vec<*const u8>=inputs.iter().flat_map(|rows|rows.iter().map(|row|row.text.as_ptr())).collect();let mut owner=OriginalVectorConcat::<Row>::new();let mut turns=0;let mut stopped=false;
 for input in &mut inputs{
  while cut.is_none_or(|cut|turns<cut){
   let d=owner.push_demand().unwrap();let issued=grant(d);
   for denied in denied(issued,d){let(result,heap)=crate::observe_retirement_allocations(||paid(crate::effect::settle(denied,|effect|owner.push(input,effect))));assert!(!result.0);assert_eq!(result.1,Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.push_demand().unwrap(),d);}
   let(result,p)=exact(||paid(crate::effect::settle(issued,|effect|owner.push(input,effect))),|result|result.1);assert!(p.fits(issued));turns+=1;if result{assert!(input.is_empty()&&input.capacity()==0);break;}
  }
  if cut.is_some_and(|cut|turns==cut){stopped=true;break;}
 }
 if !stopped{
  let d=owner.finish_demand().unwrap();for denied in denied(grant(d),d){let(step,heap)=crate::observe_retirement_allocations(||paid(owner.finish(denied)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
  exact(||paid(owner.finish(grant(d))),|step|step.progress());turns+=1;
  while !owner.ready()&&cut.is_none_or(|cut|turns<cut){
   let d=owner.demand(4096).unwrap();let issued=grant(d);for denied in denied(issued,d){let(step,heap)=crate::observe_retirement_allocations(||paid(advance_turn(&mut owner,denied)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demand(4096).unwrap(),d);}
   let step=exact(||paid(advance_turn(&mut owner,issued)),|step|step.progress());assert!(step.progress().fits(issued));turns+=1;
  }
 }
 if cut.is_none(){
  assert!(owner.ready());let d=owner.take_demand().unwrap();for denied in denied(grant(d),d){let(value,heap)=crate::observe_retirement_allocations(||paid(owner.take(denied)));assert!(value.is_none());assert_eq!(heap,(0,0));}
  let(output,_)=exact(||paid(owner.take(grant(d))).unwrap(),|result|result.1);assert_eq!(output.iter().map(|row|row.text.as_str()).collect::<Vec<_>>(),expected.iter().map(String::as_str).collect::<Vec<_>>());for(row,pointer)in output.iter().zip(&pointers){assert_eq!(row.text.as_ptr(),*pointer);}drain(output);
 }
 owner.begin_close();let mut close_work=close_work_policy::Work::new();while !owner.terminal_is_empty(){let d=owner.close_demand(4096).unwrap();assert!(d.copy_bytes<=4096,"selected close quote exceeds its authored body");let issued=cap(RetainedCloneGrant{maximum_copy_bytes:4096,..grant(d)});for denied in denied(issued,d){let(step,heap)=crate::observe_retirement_allocations(||paid(close_turn(&mut owner,denied)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.close_demand(4096).unwrap(),d);}let step=exact(||paid(close_turn(&mut owner,issued)),|step|step.progress());assert!(step.progress().fits(issued));close_work.observe(step.progress()==Default::default()&&!owner.terminal_is_empty()&&owner.close_demand(4096).unwrap()==d);}
 assert_eq!(crate::observe_retirement_allocations(||drop(owner)).1,(0,0));for input in inputs{for row in &input{assert_eq!(row.text.as_ptr(),pointers[row.ordinal]);}drain(input);}
 end_bank();
}
#[test]
fn original_vector_concat_preserves_original_rows_and_cancelled_custody(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🪢️concat/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize))){
  let chunks=law["chunks"].as_array().unwrap().iter().map(|rows|rows.as_array().unwrap().iter().map(|row|row.as_str().unwrap().to_string()).collect()).collect();let expected=law["expected"].as_array().unwrap().iter().map(|row|row.as_str().unwrap().to_string()).collect();check(chunks,expected,cut);
 }
}
#[test]
fn original_vector_concat_wide_payload_and_chunk_directory_remain_bounded(){
 for cut in[Some(0),Some(1),Some(7),Some(19),Some(71),None]{let chunks=(0..257).map(|index|if index==128{vec!["雪".repeat(21846)]}else{vec![index.to_string()]}).collect::<Vec<_>>();let expected=(0..257).map(|index|if index==128{"雪".repeat(21846)}else{index.to_string()}).collect();check(chunks,expected,cut);}
}

#[test]
fn original_vector_concat_phases_actual_storage_and_preserves_serde_flattened_identity(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🎟️work/🔣️.json")).unwrap();
 let chunks:Vec<Vec<String>>=serde_json::from_value(law["chunks"].clone()).unwrap();
 let expected:Vec<String>=serde_json::from_value(law["expected"].clone()).unwrap();
 assert_eq!(serde_json::to_value(chunks.iter().flatten().collect::<Vec<_>>()).unwrap(),law["expected"]);
 check(chunks,expected,None);
}
