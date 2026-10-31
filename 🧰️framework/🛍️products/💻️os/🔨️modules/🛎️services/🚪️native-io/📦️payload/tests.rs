//! 🧪️ Native payload chunks retain their exact copy prefix under one finite public-port bank.
use super::*;
#[path="../../../../../../🔨️modules/🧵️job/📬️outcome/📦️builder/🧪️testing/🦀️.rs"]
mod payload_case;
use payload_case::PayloadCase;
use semio_framework_job_payload::{JobPayloadStep,JobPayloadStream};
fn law()->serde_json::Value{let law:serde_json::Value=serde_json::from_str(include_str!("../🎟️original/payload.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../🎟️original/payload.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}law}
fn close(pump:&mut NativePayloadPump,case:&mut PayloadCase){for _ in 0..16384{if pump.terminal_is_empty(){return}let g=PayloadCase::exact(pump.retirement_demands().unwrap());case.step(g,|cx|pump.close_step(cx.retained_grant()));}panic!("finite native payload close did not settle")}
#[test]
fn native_payload_chunk_keeps_exact_prefix_across_every_body_and_cancellation_cut(){
 let law=law();let expected:Vec<u8>=serde_json::from_value(law["payload"].clone()).unwrap();
 for body in law["copyBodies"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;for cut in law["cancelCuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain([None]){
  let mut case=PayloadCase::new(semio_framework_job::OperationId(51),semio_framework_job::Generation(1));let mut pump=NativePayloadPump::new(JobPayloadStream::CommitOutput);let pointer=expected.as_ptr();let mut complete=false;
  for turn in 0..law["maximumTurns"].as_u64().unwrap()as usize{if cut==Some(turn){break}let offer=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64,..Default::default()};let d=pump.append_demand(&expected,offer).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:body.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..offer};let step=case.step(g,|cx|pump.append_next(&expected,cx));assert_eq!(expected.as_ptr(),pointer);assert!(pump.copy_cursor()<=expected.len());if matches!(step,RetainedCloneStep::Complete(_)){complete=true;break}}
  if cut.is_none(){assert!(complete);assert_eq!(pump.copy_cursor(),expected.len());let g=PayloadCase::exact(pump.clear_chunk_demand());case.step(g,|_|pump.clear_chunk(g));let mut output=None;for _ in 0..16384{if output.is_some(){break}let g=PayloadCase::exact(pump.finish_demand());case.step(g,|cx|pump.finish_next(&mut output,cx));}let mut output=output.unwrap();let mut actual=Vec::new();let mut reader=output.reader();while let Some(page)=reader.read_page(1,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES){actual.extend_from_slice(page)}assert_eq!(actual,expected);assert_eq!(serde_json::to_value(&actual).unwrap(),law["payload"]);for _ in 0..16384{if output.terminal_is_empty(){break}let g=PayloadCase::exact(output.retirement_demands().unwrap());case.step(g,|cx|output.close_step(cx.retained_grant()));}assert!(output.terminal_is_empty());}
  close(&mut pump,&mut case);assert_eq!(expected.as_ptr(),pointer);let(ledger,mut case)=case.into_ledger();case.close_ledger(&mut Some(ledger));
 }}
}
#[test]
fn native_payload_denial_preserves_every_current_frontier(){
 let law=law();let bytes:Vec<u8>=serde_json::from_value(law["payload"].clone()).unwrap();let mut case=PayloadCase::new(semio_framework_job::OperationId(52),semio_framework_job::Generation(1));let mut pump=NativePayloadPump::new(JobPayloadStream::CommitOutput);
 for turn in 0..16384{let offer=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:4096,maximum_depth:64,..Default::default()};let d=pump.append_demand(&bytes,offer).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:3.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..offer};let before=pump.copy_cursor();let denied=[Some(RetainedCloneGrant{maximum_items:0,..g}),d.copy_bytes.checked_sub(1).map(|maximum_copy_bytes|RetainedCloneGrant{maximum_copy_bytes,..g}),d.capacity_bytes.checked_sub(1).map(|maximum_capacity_bytes|RetainedCloneGrant{maximum_capacity_bytes,..g}),d.depth.checked_sub(1).map(|maximum_depth|RetainedCloneGrant{maximum_depth,..g})];for denied in denied.into_iter().flatten(){let step=case.step(denied,|cx|pump.append_next(&bytes,cx));assert_eq!(step.progress(),Default::default());assert_eq!(pump.copy_cursor(),before);assert_eq!(pump.append_demand(&bytes,offer).unwrap(),d)}let step=case.step(g,|cx|pump.append_next(&bytes,cx));if matches!(step,RetainedCloneStep::Complete(_)){break}assert!(turn<16383)}
 close(&mut pump,&mut case);let(ledger,mut case)=case.into_ledger();case.close_ledger(&mut Some(ledger));
}
