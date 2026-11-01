//! 🧫️ Child identity receiving retains the actual original source through every interrupted frontier.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,OriginalTurnPrimary,StepContext,close_original_retained_step,receive_original_retained_work};
use semio_framework_value::{DslValue,retained_clone::RetainedCloneSource};
use semio_framework_value_retirement::retirement::controlled::ControlledRetirement;

fn stock()->RetainedCloneGrant { RetainedCloneGrant { maximum_items:100000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128 } }
fn native_failure(primary:OriginalReceivingPrimary,receipt_refusal:Option<ValueError>)->OriginalTurnFailure { OriginalTurnFailure { effects:primary.retained_progress(),primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal,context_refusal:None } }
fn receive(cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>)->Result<RetainedCloneProgress,OriginalTurnFailure> {
 let grant=cx.retained_grant();let mut work=0;
 let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=run(control,grant);work=turn.work;turn.result});
 let progress=match &result { Ok(step)=>step.progress(),Err(error)=>error.retained_progress() };
 let receipt=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(work);
 match result { Ok(step)=>{receipt?;Ok(step.progress())},Err(primary)=>Err(native_failure(primary,receipt.err())) }
}
fn input(law:&serde_json::Value)->DslValue {
    let dialect=&law["target"]["dialect"];
    DslValue::Object(vec![("childId".into(),DslValue::String(law["childId"].as_str().unwrap().into())),("target".into(),DslValue::Object(vec![("artifactId".into(),DslValue::String(law["target"]["artifactId"].as_str().unwrap().into())),("dialect".into(),DslValue::Object(["artifactKind","standard","subset"].map(|key|(key.into(),DslValue::String(dialect[key].as_str().unwrap().into()))).into()))]))])
}
fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner) {
    let mut owner=ControlledRetirement::retain(value);
    while !owner.terminal_is_empty() {
        caller.turn_with_decoder(stock(),decoder,|cx|cx.with_original_retirement(|cx|close_original_retained_step(cx,owner.next_demand(4096)?,|grant|{let result=owner.step(grant);RetainedWorkOutcome{result,work:owner.last_turn_work()}}))).unwrap();
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
            let mut source=None;
            let mut admission=RetainedCloneSource::<DslValue>::admission_owned(input(law),());
            while source.is_none() {
                caller.turn(stock(),|cx|{
                    let turn=if admission.phase()=="ready" { admission.take_output(&mut source,cx.retained_grant()) } else { admission.advance(cx.retained_grant()) };
                    receive_original_retained_work(cx,turn)?;Ok(())
                }).unwrap();
            }
            assert!(admission.terminal_is_empty());
            let source=source.unwrap();
            let mut receiver=<ArtifactChild<()> as NativeReceive<DslValue>>::receiver();
            let mut turns=0;
            while !receiver.output_ready()&&turns<cut {
                let demand=receiver.demands(source.borrow(),4096).unwrap();
                caller.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,|native,mut grant| {
                    grant.maximum_items=0;
                    let turn=receiver.advance(source.borrow(),native,4096,grant);
                    assert_eq!(turn.result.as_ref().unwrap().progress(),Default::default());
                    turn
                })).unwrap();
                assert_eq!(receiver.demands(source.borrow(),4096).unwrap(),demand);
                let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|receive(cx,|native,grant|receiver.advance(source.borrow(),native,4096,grant))));
                let progress=result.unwrap();
                assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
                turns+=1;
            }
            if receiver.output_ready() {
                completed=turns;
                let mut output=None;
                caller.turn(stock(),|cx|{
                    let turn=receiver.take(cx.retained_grant());
                    let result=turn.result.map(|value|{let (child,progress)=value.expect("ready child has its funded output");output=Some(child);RetainedCloneStep::Progress(progress)});
                    receive_original_retained_work(cx,RetainedWorkOutcome { result,work:turn.work })?;Ok(())
                }).unwrap();
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
