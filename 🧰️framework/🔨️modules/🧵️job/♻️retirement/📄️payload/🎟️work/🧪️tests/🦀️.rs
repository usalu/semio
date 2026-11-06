//! 📬️ Public empty outcome and payload slots preserve independent exact work witnesses.
use semio_framework_job_outcome::{JobOutcomeSlot,JobPayloadSlot,StepOutcome,close_step_outcome_slot};
use semio_framework_job_payload::{RetainedJobPayload,JobPayloadStream,RetainedCloneGrant};
#[test]
fn real_empty_outcome_and_payload_slots_keep_final_metadata_work_independent(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 for kind in row["kinds"].as_array().unwrap(){
  let mut outcome=JobOutcomeSlot::from_outcome(if kind=="cancelled"{StepOutcome::Cancelled}else{StepOutcome::Yield});let mut payload=JobPayloadSlot::from_payload(RetainedJobPayload::empty(JobPayloadStream::Fault));
  let mut work=Vec::new();let mut items=Vec::new();
  for (index,grant) in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},RetainedCloneGrant::default()].into_iter().enumerate(){
   let turn=if kind=="payload"{payload.close_step(grant)}else{close_step_outcome_slot(&mut outcome,grant)};work.push(turn.work);let progress=turn.result.unwrap().progress();items.push(progress.copied_items);assert_eq!(progress.copied_bytes,0);assert_eq!(progress.retained_capacity_bytes,0);assert_eq!(progress.released_bytes,0);if index==0{assert!(!outcome.is_empty());assert!(!payload.is_empty());}
  }
  assert_eq!(serde_json::to_value(work).unwrap(),row["work"]);assert_eq!(serde_json::to_value(items).unwrap(),row["items"]);
  if !outcome.is_empty(){let turn=close_step_outcome_slot(&mut outcome,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()});assert_eq!(turn.work,1);turn.result.unwrap();}
  if !payload.is_empty(){let turn=payload.close_step(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()});assert_eq!(turn.work,1);turn.result.unwrap();}
 }
}
