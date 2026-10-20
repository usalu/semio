//! 🧫️ Child identity receiving retains the actual original source through every interrupted frontier.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext,close_original_retained_step};
use semio_framework_value::{DslValue,retained_clone::RetainedCloneSource};
use semio_framework_value_retirement::retirement::controlled::ControlledRetirement;

fn stock()->RetainedCloneGrant { RetainedCloneGrant { maximum_items:100000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128 } }
fn receive(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<RetainedCloneProgress,OriginalTurnFailure> {
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
fn input(law:&serde_json::Value)->DslValue {
    let dialect=&law["target"]["dialect"];
    DslValue::Object(vec![("childId".into(),DslValue::String(law["childId"].as_str().unwrap().into())),("target".into(),DslValue::Object(vec![("artifactId".into(),DslValue::String(law["target"]["artifactId"].as_str().unwrap().into())),("dialect".into(),DslValue::Object(["artifactKind","standard","subset"].map(|key|(key.into(),DslValue::String(dialect[key].as_str().unwrap().into()))).into()))]))])
}
fn retire<T:semio_framework_value::RetireOwned>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner) {
    let mut owner=ControlledRetirement::retain(value);
    while !owner.terminal_is_empty() {
        caller.turn_with_decoder(stock(),decoder,|cx|cx.with_original_retirement(|cx|close_original_retained_step(cx,owner.next_demand(4096)?,|grant|owner.step(grant)))).unwrap();
    }
}

#[test]
fn original_child_receive_same_caller_all_partial_prefixes_and_literal_target() {
    let laws:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();
    for law in laws["cases"].as_array().unwrap() {
        let mut completed=0;
        for cut in 0..2048 {
            let mut caller=OriginalJobFixtureCaller::admit(OperationId(24),Generation(3),stock(),||Some(0),u64::MAX,1000000);
            let mut decoder=OriginalJobDecodeOwner::empty();
            let mut input=Some(input(law));
            let mut source=None;
            let demand=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();
            let demand=RetirementDemand { copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),capacity_bytes:demand.capacity_bytes,depth:demand.depth,..Default::default() };
            caller.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,demand,|native,grant| {
                native.charge(demand.capacity_bytes)?;
                match RetainedCloneSource::admit_owned(input.take().unwrap(),(),grant) {
                    Ok((owner,progress))=>{source=Some(owner);Ok(RetainedCloneStep::Progress(progress))},
                    Err((error,original,_))=>{input=Some(original);Err(error)},
                }
            })).unwrap();
            let source=source.unwrap();
            let mut receiver=None;
            caller.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,<ArtifactChild<()> as NativeReceive<DslValue>>::receive_birth_demand(),|_,grant| {
                let (owner,progress)=<ArtifactChild<()> as NativeReceive<DslValue>>::begin_receive(grant)?;
                receiver=Some(owner);Ok(RetainedCloneStep::Progress(progress))
            })).unwrap();
            let mut receiver=receiver.unwrap();
            let mut turns=0;
            while !receiver.output_ready()&&turns<cut {
                let demand=receiver.demands(source.borrow(),4096).unwrap();
                caller.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,demand,|native,mut grant| {
                    grant.maximum_items=0;
                    assert_eq!(receiver.advance(source.borrow(),native,grant)?.progress(),Default::default());
                    Ok(RetainedCloneStep::Progress(Default::default()))
                })).unwrap();
                assert_eq!(receiver.demands(source.borrow(),4096).unwrap(),demand);
                let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,demand,|native,grant|receiver.advance(source.borrow(),native,grant))));
                let progress=result.unwrap();
                assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
                turns+=1;
            }
            if receiver.output_ready() {
                completed=turns;
                let mut output=None;
                caller.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,receiver.take_demand()?,|_,grant| {
                    let (child,progress)=receiver.take(grant)?.unwrap();output=Some(child);Ok(RetainedCloneStep::Progress(progress))
                })).unwrap();
                let output=output.unwrap();
                assert_eq!(output.child_id,law["childId"].as_str().unwrap());
                assert_eq!(output.target.artifact_id,law["target"]["artifactId"].as_str().unwrap());
                assert_eq!(output.target.dialect.subset,law["target"]["dialect"]["subset"].as_str().unwrap());
                assert!(output.local_owner.is_none());
                retire(output,&mut caller,&mut decoder);
            }
            receiver.begin_close();
            while !receiver.terminal_is_empty() {
                let demand=receiver.close_demands(4096).unwrap();
                caller.turn_with_decoder(stock(),&mut decoder,|cx|cx.with_original_retirement(|cx|close_original_retained_step(cx,demand,|grant|receiver.close_step(grant)))).unwrap();
            }
            retire(source,&mut caller,&mut decoder);
            while !decoder.terminal_is_empty() {
                caller.turn(stock(),|cx|close_original_retained_step(cx,decoder.retirement_demands(4096)?,|grant|decoder.close_step(grant))).unwrap();
            }
            caller.retire(stock());
            if completed>0&&cut>completed { break; }
        }
        assert!(completed>0);
    }
}
