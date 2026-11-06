//! 🧫️ Actual original receiving turns reconstruct the independent serde owner from complete Pack framing.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetireOwned};
use semio_framework_value_retirement::retirement::controlled::ControlledRetirement;
use protocol::PackLimits;
#[derive(Debug,PartialEq,serde::Deserialize,semio_framework_dsl_record_derive::DslRecord,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
struct OriginalRecord{text:String,items:Vec<String>}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(11),Generation(3),stock(),||Some(0),u64::MAX,1000000)}
fn native_failure(primary:semio_framework_value_native_owned::OriginalReceivingPrimary,receipt_refusal:Option<ValueError>)->OriginalTurnFailure{OriginalTurnFailure{effects:primary.retained_progress(),primary:semio_framework_job::OriginalTurnPrimary::Receiving(primary),receipt_refusal,context_refusal:None,work_refusal:None}}
fn receive_posted(cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,semio_framework_value_native_owned::OriginalReceivingPrimary>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let grant=cx.retained_grant();let mut work=0;
 let result=cx.with_native_receiving::<_,semio_framework_value_native_owned::OriginalReceivingPrimary>(|native|{let turn=run(native,grant);work=turn.work;turn.result});
 let progress=match &result { Ok(step)=>step.progress(),Err(error)=>error.retained_progress() };
 let receipt=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(work);
 match result { Ok(step)=>{receipt?;Ok(step.progress())},Err(primary)=>Err(native_failure(primary,receipt.err())) }
}
fn receive(cx:&mut StepContext<'_>,_demand:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let grant=cx.retained_grant();let mut work=0;
 let result=cx.with_native_receiving::<_,semio_framework_value_native_owned::OriginalReceivingPrimary>(|native|{let defining=run(native,grant);let turn=native.receive_retained_work(grant,defining);work=turn.work;turn.result});
  let progress=match &result { Ok(step)=>step.progress(),Err(error)=>error.retained_progress() };
 let receipt=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(work);
 match result { Ok(step)=>{receipt?;Ok(step.progress())},Err(primary)=>Err(native_failure(primary,receipt.err())) }
}
fn finish(mut caller:OriginalJobFixtureCaller,mut decoder:OriginalJobDecodeOwner){while !decoder.terminal_is_empty(){caller.turn(stock(),|cx|{decoder.close_on_source(cx)?;Ok(())}).unwrap();}caller.retire(stock());}
#[test]
fn original_snapshot_pack_receiving_preserves_same_source_control_and_paid_close(){
 let neutral=include_str!("../../../../../../../../🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🧫️fixtures/🔣️.json");let row:serde_json::Value=serde_json::from_str(neutral).unwrap();let expected:OriginalRecord=serde_json::from_value(serde_json::json!({"text":row["text"],"items":row["items"]})).unwrap();let body=semio_os_pack::record::encode_document(&OriginalRecord::__dsl_spec(),&expected.__dsl_to_record(),&Default::default()).unwrap();let envelope=semio_framework_os_semio::SemioEnvelope::from_envelope_id("test.original",semio_framework_os_semio::Component::Pack,1).unwrap();let source=semio_framework_os_semio::wrap_binary(&envelope,&body);let pointer=source.as_ptr();let mut original=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner:Option<OriginalSnapshotRecordReceiving<OriginalRecord>>=None;
 original.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,metadata::<OriginalSnapshotRecordReceiving<OriginalRecord>>(),|native,grant|{let result=(||{if let Some((candidate,progress))=OriginalSnapshotRecordReceiving::admit(&source,"test.original",PackLimits::default(),native,grant)?{owner=Some(candidate);return Ok(RetainedCloneStep::Progress(progress))}Ok(RetainedCloneStep::Progress(Default::default()))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})).unwrap();let mut owner=owner.unwrap();let mut turns=0;
 while !owner.is_ready(){let demand=owner.demands(&source,4096).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn_with_decoder(stock(),&mut decoder,|cx|receive_posted(cx,|native,grant|{let turn=owner.step(&source,native,4096,grant);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})));let progress=result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(source.as_ptr(),pointer);turns+=1;assert!(turns<100000);}
 let mut output=None;original.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,metadata::<OriginalRecord>(),|_,grant|semio_framework_value::RetainedWorkOutcome{result:owner.take_into(&mut output,grant).map(RetainedCloneStep::Progress),work:1})).unwrap();assert_eq!(output.as_ref(),Some(&expected));
 original.cancel().cancel_now();
 while !owner.terminal_is_empty(){let demand=owner.close_demands(4096).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|retire(cx,demand,|native,grant|owner.close_step(native,grant)))));let progress=result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
 let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){original.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,output.next_demand(4096)?,|grant|semio_framework_value::RetainedWorkOutcome{result:output.step(grant),work:output.last_turn_work()}))).unwrap();}drop(owner);drop(output);finish(original,decoder);
}

fn retire(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut dyn semio_framework_value_native_owned::native_decoding::NativeDecodeCloseControl,RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let grant=cx.retained_grant();if cx.retirement_should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default())}
 let turn=cx.with_native_retirement(|native|Ok::<_,ValueError>(run(native,grant)))?;let result=turn.result;let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let reconciled=cx.consume_retained(progress);cx.consume_fuel(turn.work);
 match(result,reconciled){(Ok(_),Ok(()))=>Ok(progress),(Err(error),Ok(()))=>Err(error.into()),(Ok(_),Err(error))=>Err(error.into()),(Err(error),Err(receipt))=>{let mut failure=OriginalTurnFailure::from(error);failure.receipt_refusal=Some(receipt);Err(failure)}}
}

#[test]
fn original_snapshot_record_prefixes_retire_through_the_same_original_snapshot_recipient(){
 let expected=OriginalRecord{text:"original retained record 雪".repeat(40),items:vec!["one".into(),"two".into()]};let body=semio_os_pack::record::encode_document(&OriginalRecord::__dsl_spec(),&expected.__dsl_to_record(),&Default::default()).unwrap();let envelope=semio_framework_os_semio::SemioEnvelope::from_envelope_id("test.original",semio_framework_os_semio::Component::Pack,1).unwrap();let source=semio_framework_os_semio::wrap_binary(&envelope,&body);
 for prefix in[0,1,8,64,512,4096,usize::MAX]{let mut original=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner:Option<OriginalSnapshotRecordReceiving<OriginalRecord>>=None;
  original.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,metadata::<OriginalSnapshotRecordReceiving<OriginalRecord>>(),|native,grant|{let result=(||{let(candidate,p)=OriginalSnapshotRecordReceiving::admit(&source,"test.original",PackLimits::default(),native,grant)?.unwrap();owner=Some(candidate);Ok(RetainedCloneStep::Progress(p))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})).unwrap();let mut owner=owner.unwrap();let mut turns=0;
  while turns<prefix&&!owner.is_ready(){let demand=owner.demands(&source,4096).unwrap();original.turn_with_decoder(stock(),&mut decoder,|cx|receive_posted(cx,|native,grant|{let turn=owner.step(&source,native,4096,grant);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})).unwrap();turns+=1;assert!(turns<100000);}
  original.cancel().cancel_now();let mut retained=ControlledRetirement::new(owner).ok().unwrap();while !retained.terminal_is_empty(){let demand=retained.next_demand(4096).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:retained.step(grant),work:retained.last_turn_work()}))));let progress=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}drop(retained);finish(original,decoder);
 }
}
