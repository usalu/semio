//! 📐️ Exact pure append quotes preserve original text and native allocation receipts.
use super::*;
use crate::{RetirementDemand,retirement::controlled::ControlledRetirement};
use crate::value::observe_retirement_allocations as observe;
fn grant(demand:RetirementDemand)->RetainedCloneGrant{assert!(demand.copy_bytes+demand.capacity_bytes<=4096);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn exact(step:RetainedCloneStep,heap:(usize,usize),grant:RetainedCloneGrant){assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
#[test]
fn paged_native_utf8_append_pure_advance_demands_admit_exact_original_work_and_cancel(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for text in fixture["texts"].as_array().unwrap(){for repeat in [1,4097]{for pause in [Some(0),Some(1),Some(3),Some(17),None]{
  let source=text.as_str().unwrap().repeat(repeat);let pointer=source.as_ptr();let mut destination=PagedUtf8::<{usize::MAX}>::new();let(mut cursor,heap)=observe(PagedUtf8AppendCursor::default);assert_eq!(heap,(0,0));let mut turns=0;let mut complete=false;
  loop{assert!(turns<100000);if pause==Some(turns){break;}let(demand,heap)=observe(||cursor.next_advance_demand(&source,&destination).unwrap());assert_eq!(heap,(0,0));let admitted=grant(demand);let(step,heap)=observe(||cursor.advance(&source,&mut destination,Default::default()).unwrap());exact(step,heap,Default::default());assert_eq!(step.progress(),Default::default());
   for refused in [RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..admitted},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..admitted}]{if refused==admitted{continue;}let(step,heap)=observe(||cursor.advance(&source,&mut destination,refused).unwrap());exact(step,heap,refused);assert_eq!(step.progress(),Default::default());}
   let(step,heap)=observe(||cursor.advance(&source,&mut destination,admitted).unwrap());exact(step,heap,admitted);turns+=1;if matches!(step,RetainedCloneStep::Complete(_)){complete=true;break;}
  }
  if complete{assert!(destination.eq_str(&source));}assert_eq!(source.as_ptr(),pointer);cursor.begin_close();for turn in 0..100000{if cursor.terminal_is_empty(){break;}let work=cursor.next_close_copy_byte_demand().unwrap();let admitted=grant(RetirementDemand{copy_bytes:work,capacity_bytes:cursor.next_close_capacity_byte_demand(work).unwrap(),release_bytes:cursor.next_close_release_byte_demand().unwrap(),depth:cursor.next_depth_demand().unwrap()});let(step,heap)=observe(||cursor.close_step(admitted).unwrap());exact(step,heap,admitted);assert!(turn<99999||cursor.terminal_is_empty());}let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));
  let(mut owner,heap)=observe(||ControlledRetirement::new(destination).unwrap_or_else(|_|panic!("native paged output")));assert_eq!(heap,(0,0));for turn in 0..100000{if owner.terminal_is_empty(){break;}let work=owner.next_copy_byte_demand().unwrap();let admitted=grant(RetirementDemand{copy_bytes:work,capacity_bytes:owner.next_capacity_byte_demand(work).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()});let(step,heap)=observe(||owner.step(admitted).unwrap());exact(step,heap,admitted);assert!(turn<99999||owner.terminal_is_empty());}let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
  println!("[DEBUG] native pure append bytes={} repeat={repeat} pause={pause:?} turns={turns} originalPointer exactUTF8/Serde; pure getter/zero/below/capacity/copy/close heap4096/100000/drop0",source.len());
 }}}
}
