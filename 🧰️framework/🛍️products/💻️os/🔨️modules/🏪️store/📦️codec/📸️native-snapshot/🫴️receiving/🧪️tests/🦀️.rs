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
fn receive(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let result=cx.with_native_receiving::<_,semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure<ValueError>>(|native|native.receive_construction_step(demand,run));
 let progress=match &result { Ok(step)=>step.progress(),Err(error)=>error.retained_progress() };
 let reconciled=if let Some(grant)=cx.receiving_issued_grant() { cx.consume_receiving_retained(grant,progress) } else { assert_eq!(progress,Default::default());Ok(()) };
 cx.consume_fuel(progress.copied_items as u64);
 match(result,reconciled) {
  (Ok(step),Ok(()))=>Ok(step.progress()),
  (Err(error),Ok(()))=>Err(error.into()),
  (Ok(_),Err(error))=>Err(error.into()),
  (Err(error),Err(receipt))=>{let mut failure=OriginalTurnFailure::from(error);failure.context_refusal=Some(receipt);Err(failure)}
 }
}
fn finish(mut caller:OriginalJobFixtureCaller,mut decoder:OriginalJobDecodeOwner){while !decoder.terminal_is_empty(){caller.turn(stock(),|cx|{let step=decoder.close_step(cx.retained_grant())?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(())}).unwrap();}caller.retire(stock());}
#[test]
fn original_snapshot_pack_receiving_preserves_same_source_control_and_paid_close(){
 let neutral=include_str!("../../../../../../../../🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🧫️fixtures/🔣️.json");let row:serde_json::Value=serde_json::from_str(neutral).unwrap();let expected:OriginalRecord=serde_json::from_value(serde_json::json!({"text":row["text"],"items":row["items"]})).unwrap();let body=crate::os_pack::record::encode_document(&OriginalRecord::__dsl_spec(),&expected.__dsl_to_record(),&Default::default()).unwrap();let envelope=semio_framework_os_semio::SemioEnvelope::from_envelope_id("test.original",semio_framework_os_semio::Component::Pack,1).unwrap();let source=semio_framework_os_semio::wrap_binary(&envelope,&body);let pointer=source.as_ptr();let mut original=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner:Option<OriginalSnapshotRecordReceiving<OriginalRecord>>=None;
 original.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,metadata::<OriginalSnapshotRecordReceiving<OriginalRecord>>(),|native,grant|{if let Some((candidate,progress))=OriginalSnapshotRecordReceiving::admit(&source,"test.original",PackLimits::default(),native,grant)?{owner=Some(candidate);return Ok(RetainedCloneStep::Progress(progress))}Ok(RetainedCloneStep::Progress(Default::default()))})).unwrap();let mut owner=owner.unwrap();let mut turns=0;
 while !owner.is_ready(){let demand=owner.demands(&source,4096).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,demand,|native,grant|owner.step(&source,native,4096,grant).map(RetainedCloneStep::Progress))));let progress=result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(source.as_ptr(),pointer);turns+=1;assert!(turns<100000);}
 let mut output=None;original.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,metadata::<OriginalRecord>(),|_,grant|owner.take_into(&mut output,grant).map(RetainedCloneStep::Progress))).unwrap();assert_eq!(output.as_ref(),Some(&expected));
 original.cancel().cancel_now();
 while !owner.terminal_is_empty(){let demand=owner.close_demands(4096).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|retire(cx,demand,|native,grant|owner.close_step(native,grant)))));let progress=result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
 let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();while !output.terminal_is_empty(){original.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,output.next_demand(4096)?,|grant|output.step(grant)))).unwrap();}drop(owner);drop(output);finish(original,decoder);println!("[DEBUG] originalSnapshot completePackFraming=true independentSerde=true sameOriginalControl=true paidPartialAndOutputClose=true turns={turns}");
}

fn retire(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut dyn semio_framework_value_native_owned::native_decoding::NativeDecodeCloseControl,RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let grant=cx.retained_grant();if cx.retirement_should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default())}
 let result=cx.with_native_retirement(|native|run(native,grant));let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let reconciled=cx.consume_retained(progress);cx.consume_fuel(progress.copied_items as u64);
 match(result,reconciled){(Ok(_),Ok(()))=>Ok(progress),(Err(error),Ok(()))=>Err(error.into()),(Ok(_),Err(error))=>Err(error.into()),(Err(error),Err(receipt))=>{let mut failure=OriginalTurnFailure::from(error);failure.receipt_refusal=Some(receipt);Err(failure)}}
}
