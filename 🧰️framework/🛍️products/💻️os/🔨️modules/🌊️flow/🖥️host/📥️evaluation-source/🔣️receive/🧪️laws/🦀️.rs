//! 🧪️ Genuine original JSON receiving and cancellation match actual physical receipts.
use super::*;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::retained_clone::RetainedCloneClose;
 use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,close_original_retained_step};
 fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,run));let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));result.unwrap()
 }
 #[test]
 fn original_json_field_receives_actual_snapshot_and_closes_canceled_partial_custody(){
  let original=semio_framework_artifact_flow_flow::FlowHostSnapshot::default();let wire=semio_framework_pack_json::to_json_string(&original);
  let independent:serde_json::Value=serde_json::from_str(&wire).unwrap();assert_eq!(independent["widgets"].as_array().unwrap().len(),3);
  for canceled in [false,true]{
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128};
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(983),Generation(68),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<26,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:128},semio_framework_job::default_now_us,u64::MAX,1<<20);
   let mut decoder=OriginalJobDecodeOwner::empty();turn(&mut caller,&mut decoder,grant,|_|Ok(()));
   let(mut field,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(JsonField::<semio_framework_artifact_flow_flow::FlowHostSnapshot>::new);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let mut output=None;
   for index in 0..200_000{
    if canceled&&index==17{caller.cancel().cancel_now();break}
    if field.ready(){turn(&mut caller,&mut decoder,grant,|cx|{let taken=field.take(cx.retained_grant())?;if let Some((value,progress))=taken{output=Some(value);cx.consume_retained(progress)?;cx.consume_fuel(1);}Ok(())});break}
    let demand=field.demands(wire.as_str(),4096).unwrap();
    turn(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..grant},|cx|field.step(wire.as_str(),cx));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(field.demands(wire.as_str(),4096).unwrap(),demand);
    turn(&mut caller,&mut decoder,grant,|cx|field.step(wire.as_str(),cx));
    assert!(index+1<200_000,"original JSON field did not finish its bounded source");
   }
   if !canceled{assert_eq!(output.as_ref(),Some(&original))}
   field.begin_close();
   for index in 0..200_000{if field.terminal_is_empty(){break}turn(&mut caller,&mut decoder,grant,|cx|cx.with_original_retirement(|cx|{let demand=field.close_demands(4096)?;close_original_retained_step(cx,demand,|grant|field.close_step(grant))?;Ok(())}));assert!(index+1<200_000);}
   assert!(field.terminal_is_empty());drop(field);
   let mut retirement=RetainedCloneClose::default();
   for index in 0..200_000{if output.is_none()&&retirement.is_empty(){break}turn(&mut caller,&mut decoder,grant,|cx|cx.with_original_retirement(|cx|{let demand=retirement.next_owner_demands::<semio_framework_artifact_flow_flow::FlowHostSnapshot>(output.is_some(),&None,4096)?;close_original_retained_step(cx,demand,|grant|if retirement.is_empty(){let issued=retirement.begin_granted(&mut output,grant);semio_framework_value::RetainedWorkOutcome{result:issued.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:issued.work}}else{retirement.step_granted(grant)})?;Ok(())}));assert!(index+1<200_000);}
   while !decoder.terminal_is_empty(){caller.turn(grant,|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(4096)?;close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}caller.retire(grant);
  }original.retire_cold();
 }

