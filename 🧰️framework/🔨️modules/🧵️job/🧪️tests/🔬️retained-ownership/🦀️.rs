//! 🧪️ Real payload storage retains every original backing and independent stream under one finite issuer.
use semio_framework_job::*;
use semio_framework_test_fixtures::fixture_text;
#[path="../../../📬️outcome/📦️builder/🧪️testing/🦀️.rs"]
mod payload_case;
use payload_case::PayloadCase;
fn physical_close_grant(release:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:release,maximum_depth:64}}
#[test]
fn retained_writer_physical_close_preserves_staged_and_rejected_backing(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧫️fixtures/📦️physical-close/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut case=PayloadCase::new(OperationId(90020),Generation(21));let mut writer=case.initialize(JobPayloadStream::Preview);case.stage(&mut writer,&vec![42;row["logicalBytes"].as_u64().unwrap()as usize]);let mut source=Some(case.source(JOB_PAYLOAD_PAGE_BYTES));let rejected_pointer=source.as_ref().unwrap().backing_identity();let d=writer.original_page_admission_demand(&source).unwrap();case.step(PayloadCase::exact(d),|cx|writer.accept_original_page(&mut source,cx));assert!(source.is_none());let staged_pointer=writer.staged_backing_identity().unwrap();assert_eq!(writer.incoming_backing_identity(),Some(rejected_pointer));let(ledger,mut case)=case.into_ledger();
  let mut observations=Vec::new();for(staged,pointer)in[(false,rejected_pointer),(true,staged_pointer)]{
   assert_eq!(writer.retirement_demands().unwrap().release_bytes,JOB_PAYLOAD_PAGE_BYTES);let under=physical_close_grant(row["insufficientGrant"].as_u64().unwrap()as usize);
   for _ in 0..4{assert_eq!(case.step(under,|_|writer.close_step_granted(under)),RetainedCloneStep::Progress(Default::default()));}
   let refused=case.step(under,|_|writer.close_step_granted(under));let retained_pointer=if staged{writer.staged_backing_identity()==Some(pointer)}else{writer.incoming_backing_identity()==Some(pointer)};
   let g=physical_close_grant(writer.retirement_demands().unwrap().release_bytes);let released=case.step(g,|_|writer.close_step_granted(g));observations.push((staged,refused,retained_pointer,released));
  }
  while !writer.terminal_is_empty(){let g=physical_close_grant(JOB_PAYLOAD_PAGE_BYTES);case.step(g,|_|writer.close_step_granted(g));}assert_eq!(ledger.process_share_bytes(),0);
  for(staged,refused,retained_pointer,released)in observations{assert_eq!(refused,RetainedCloneStep::Progress(Default::default()),"staged={staged} {}",row["name"]);assert!(retained_pointer,"staged={staged} {}",row["name"]);assert_eq!(released,RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:JOB_PAYLOAD_PAGE_BYTES,..Default::default()}),"staged={staged} {}",row["name"]);}case.close_ledger(&mut Some(ledger));
 }
}
#[test]
fn payload_ledger_identity_must_match_the_exact_step_context(){
 let mut case=PayloadCase::new(OperationId(90000),Generation(6));let mut cancel=case.apply(RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:65536,maximum_depth:1,..Default::default()},|cx|CancelToken::admit_root(JobPayloadStep::retained_grant(cx)),|value|value.as_ref().map_or(Default::default(),|(_,p)|*p)).unwrap().0;
 for(operation,generation)in[(OperationId(90001),Generation(6)),(OperationId(90000),Generation(7))]{let mut sequence=0;let mut actual=Default::default();let result=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,physical_close_grant(65536)),&cancel,||Some(1),&mut sequence,&mut actual,case.authority());assert!(result.is_err());}
 cancel.cancel_now();let mut close=semio_framework_async::CancelTokenRetirement::from_token(cancel);while !close.terminal_is_empty(){let g=physical_close_grant(65536);case.step(g,|_|{let turn=close.close_step(g);semio_framework_value::RetainedWorkOutcome{work:turn.work,result:turn.result.map_err(|e|match e{semio_framework_async::CancelTokenRetirementError::Refused(e)=>e,semio_framework_async::CancelTokenRetirementError::Blocked(_)=>ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original test root remains blocked")})}});}let(ledger,mut case)=case.into_ledger();case.close_ledger(&mut Some(ledger));
}
#[test]
fn retained_state_and_output_have_separate_credits_and_close_one_page_per_grant(){
 let mut case=PayloadCase::new(OperationId(90002),Generation(8));let mut state=case.initialize(JobPayloadStream::CommitState);let mut output=case.initialize(JobPayloadStream::CommitOutput);
 let g=PayloadCase::exact(state.staged_page_birth_demands().unwrap());case.step(g,|cx|{let turn=state.begin_staged_page_funded(cx);let denied=output.begin_staged_page_funded(cx);assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),Default::default());turn});
 case.page(&mut state,b"state");case.page(&mut output,b"output");assert_eq!(state.original().unwrap().page_count(),1);let mut terminal=StepOutcome::Complete(CommitCandidate{state:case.finish(&mut state),output:case.finish(&mut output)});let(ledger,mut case)=case.into_ledger();let g=physical_close_grant(JOB_PAYLOAD_PAGE_BYTES);
 assert_eq!(case.step(g,|_|terminal.close_step(g)),RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:JOB_PAYLOAD_PAGE_BYTES,..Default::default()}));assert!(!terminal.terminal_is_empty());assert_eq!(case.step(g,|_|terminal.close_step(g)).progress().released_bytes,0);assert!(!terminal.terminal_is_empty());assert_eq!(case.step(g,|_|terminal.close_step(g)),RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:JOB_PAYLOAD_PAGE_BYTES,..Default::default()}));assert_eq!(case.step(g,|_|terminal.close_step(g)).progress().released_bytes,0);assert!(terminal.terminal_is_empty());case.close_ledger(&mut Some(ledger));
}
#[test]
fn retained_writer_and_reader_advance_exactly_one_page_per_opportunity(){
 let mut case=PayloadCase::new(OperationId(90008),Generation(14));let bytes=vec![19u8;JOB_PAYLOAD_PAGE_BYTES+1];let mut writer=case.initialize(JobPayloadStream::CommitOutput);let mut cursor=0;let full=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_capacity_bytes:JOB_PAYLOAD_PAGE_BYTES,maximum_depth:64,..Default::default()};let zero=RetainedCloneGrant{maximum_items:0,..full};
 assert_eq!(case.step(zero,|cx|writer.append_original(cx,&bytes,&mut cursor)),RetainedCloneStep::Progress(Default::default()));assert_eq!(cursor,0);
 for target in[JOB_PAYLOAD_PAGE_BYTES,bytes.len()]{for _ in 0..16{if cursor==target{break}let step=case.step(full,|cx|writer.append_original(cx,&bytes,&mut cursor));assert_eq!(matches!(step,RetainedCloneStep::Complete(_)),cursor==bytes.len());}assert_eq!(cursor,target);}
 let mut payload=case.finish(&mut writer);let(ledger,mut case)=case.into_ledger();let mut reader=payload.reader();assert_eq!(reader.read_page(0,JOB_PAYLOAD_PAGE_BYTES),None);assert_eq!(reader.read_page(1,JOB_PAYLOAD_PAGE_BYTES).map(|p|p.len()),Some(JOB_PAYLOAD_PAGE_BYTES));assert_eq!(reader.read_page(1,JOB_PAYLOAD_PAGE_BYTES).map(|p|p.len()),Some(1));assert!(reader.terminal_is_empty());let g=physical_close_grant(JOB_PAYLOAD_PAGE_BYTES);
 for _ in 0..2{assert_eq!(case.step(g,|_|payload.close_step(g)),RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:JOB_PAYLOAD_PAGE_BYTES,..Default::default()}));}assert_eq!(case.step(g,|_|payload.close_step(g)).progress().released_bytes,0);assert_eq!(case.step(g,|_|payload.close_step(g)),RetainedCloneStep::Complete(Default::default()));assert!(payload.terminal_is_empty());case.close_ledger(&mut Some(ledger));
}
