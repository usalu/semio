//! 🧪️ Real original vectors retain payload pointers and close every quoted physical allocation.
use super::*;
use crate::retirement::controlled::ControlledRetirement;
#[derive(crate::RetireOwned)]
struct Row{ordinal:usize,text:String}
impl Clone for Row{fn clone(&self)->Self{panic!("original concatenation must never clone payload rows")}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn denied(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut grants=vec![RetainedCloneGrant{maximum_items:0,..g}];if d.copy_bytes>0{grants.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g});}if d.capacity_bytes>0{grants.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g});}if d.release_bytes>0{grants.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g});}if d.depth>0{grants.push(RetainedCloneGrant{maximum_depth:d.depth-1,..g});}grants}
fn exact<T>(run:impl FnOnce()->T,progress:impl FnOnce(&T)->RetainedCloneProgress)->T{let(value,heap)=crate::observe_retirement_allocations(run);let progress=progress(&value);assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));value}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();while !owner.terminal_is_empty(){let d=owner.next_demand(4096).unwrap();exact(||owner.step(grant(d)).unwrap(),|step|step.progress());}}
fn check(chunks:Vec<Vec<String>>,expected:Vec<String>,cut:Option<usize>){
 let mut ordinal=0;let mut inputs:Vec<Vec<Row>>=chunks.into_iter().map(|rows|rows.into_iter().map(|text|{let row=Row{ordinal,text};ordinal+=1;row}).collect()).collect();
 let pointers:Vec<*const u8>=inputs.iter().flat_map(|rows|rows.iter().map(|row|row.text.as_ptr())).collect();let mut owner=OriginalVectorConcat::<Row>::new();let mut turns=0;let mut stopped=false;
 for input in &mut inputs{
  while cut.is_none_or(|cut|turns<cut){
   let d=owner.push_demand().unwrap();let issued=grant(d);
   for denied in denied(issued,d){let(result,heap)=crate::observe_retirement_allocations(||owner.push(input,denied).unwrap());assert!(!result.0);assert_eq!(result.1,Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.push_demand().unwrap(),d);}
   let(result,p)=exact(||owner.push(input,issued).unwrap(),|result|result.1);assert!(p.fits(issued));turns+=1;if result{assert!(input.is_empty()&&input.capacity()==0);break;}
  }
  if cut.is_some_and(|cut|turns==cut){stopped=true;break;}
 }
 if !stopped{
  let d=owner.finish_demand().unwrap();for denied in denied(grant(d),d){let(step,heap)=crate::observe_retirement_allocations(||owner.finish(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
  exact(||owner.finish(grant(d)).unwrap(),|step|step.progress());turns+=1;
  while !owner.ready()&&cut.is_none_or(|cut|turns<cut){
   let d=owner.demand(4096).unwrap();let issued=grant(d);for denied in denied(issued,d){let(step,heap)=crate::observe_retirement_allocations(||owner.advance(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demand(4096).unwrap(),d);}
   let step=exact(||owner.advance(issued).unwrap(),|step|step.progress());assert!(step.progress().fits(issued));turns+=1;
  }
 }
 if cut.is_none(){
  assert!(owner.ready());let d=owner.take_demand().unwrap();for denied in denied(grant(d),d){let(value,heap)=crate::observe_retirement_allocations(||owner.take(denied).unwrap());assert!(value.is_none());assert_eq!(heap,(0,0));}
  let(output,_)=exact(||owner.take(grant(d)).unwrap().unwrap(),|result|result.1);assert_eq!(output.iter().map(|row|row.text.as_str()).collect::<Vec<_>>(),expected.iter().map(String::as_str).collect::<Vec<_>>());for(row,pointer)in output.iter().zip(&pointers){assert_eq!(row.text.as_ptr(),*pointer);}drain(output);
 }
 owner.begin_close();while !owner.terminal_is_empty(){let d=owner.close_demand(4096).unwrap();let issued=grant(d);for denied in denied(issued,d){let(step,heap)=crate::observe_retirement_allocations(||owner.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.close_demand(4096).unwrap(),d);}let step=exact(||owner.close_step(issued).unwrap(),|step|step.progress());assert!(step.progress().fits(issued));}
 assert_eq!(crate::observe_retirement_allocations(||drop(owner)).1,(0,0));for input in inputs{for row in &input{assert_eq!(row.text.as_ptr(),pointers[row.ordinal]);}drain(input);}
 eprintln!("[DEBUG] original concat cut={cut:?} turns={turns} exact original payload pointers, five currency denial and physical terminal closure");
}
#[test]
fn original_vector_concat_preserves_original_rows_and_cancelled_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize))){
  let chunks=law["chunks"].as_array().unwrap().iter().map(|rows|rows.as_array().unwrap().iter().map(|row|row.as_str().unwrap().to_string()).collect()).collect();let expected=law["expected"].as_array().unwrap().iter().map(|row|row.as_str().unwrap().to_string()).collect();check(chunks,expected,cut);
 }
}
#[test]
fn original_vector_concat_wide_payload_and_chunk_directory_remain_bounded(){
 for cut in[Some(0),Some(1),Some(7),Some(19),Some(71),None]{let chunks=(0..257).map(|index|if index==128{vec!["雪".repeat(21846)]}else{vec![index.to_string()]}).collect::<Vec<_>>();let expected=(0..257).map(|index|if index==128{"雪".repeat(21846)}else{index.to_string()}).collect();check(chunks,expected,cut);}
}
