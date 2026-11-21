//! 🛬️ Original Arc construction keeps the input and born backing through separately funded turns.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value_native_owned::native_decoding::{NativeSharedAdmission, NativeReceivingEffectError};
use semio_framework_value::RetainedWorkPoll;
use super::*;

fn arc_turn(
    cx: &mut StepContext<'_>,
    input: &mut Option<SemioImageSnapshot>,
    output: &mut Option<Arc<SemioImageSnapshot>>,
    owner: &mut NativeSharedAdmission<SemioImageSnapshot>,
    post_receipt: bool,
) -> Result<RetainedCloneProgress, OriginalTurnFailure> {
    let issued = cx.retained_grant();
    let mut scope = None;
    let mut stepped = None;
    if !cx.with_native_receiving(&mut scope, |control, _| {
        stepped = Some(control.receive_arc(input, output, owner));
        Ok(())
    })? {
        return Ok(Default::default());
    }
    let scope = scope.expect("original native Arc scope");
    assert!(scope.source_refusal.is_none() && scope.call.is_ok());
    let turn = stepped.expect("original native Arc action");
    cx.consume_fuel(u64::try_from(turn.work).unwrap());
    let progress = match &turn.result {
        Ok(progress) => *progress,
        Err(error) => error.retained_progress(),
    };
    let receipt = cx.consume_receiving_retained(issued, if post_receipt { progress } else { Default::default() });
    match turn.result {
        Ok(progress) => { receipt?; Ok(progress) }
        Err(primary) => Err(OriginalTurnFailure {
            effects: progress, primary: OriginalTurnPrimary::Receiving(primary),
            receipt_refusal: receipt.err(), context_refusal: None, work_refusal: None,
        }),
    }
}

fn receive_turn(
    cx: &mut StepContext<'_>,
    owner: &mut NativeReceivingOriginal<semio_framework_value::DslValue,RasterImagePayload>,
) -> Result<(), OriginalTurnFailure> {
    let issued = cx.retained_grant();
    let units = cx.fuel_remaining();
    let mut scope = None;
    let mut stepped = None;
    if !cx.with_native_receiving(&mut scope, |control, grant| {
        let witness = control.source_witness()?;
        stepped = Some(owner.advance(control,witness,grant.maximum_copy_bytes,units,grant));
        Ok(())
    })? {
        return Ok(());
    }
    let scope = scope.expect("original native image receiving scope");
    if scope.source_refusal.is_some() || scope.call.is_err() {
        return Err(ValueError::invariant("original native image receiving scope refused").with_retained_progress(scope.retained_progress()).into());
    }
    let (result, work) = match stepped.expect("original image receiving action") {
        RetainedWorkPoll::Pending { work } => (Ok(Default::default()), work),
        RetainedWorkPoll::Complete(turn) => (turn.result.map(|step| step.progress()).map_err(|error| {
            let progress = error.retained_progress();
            let cause = error.primary();
            cause.primary.as_ref().or(cause.quote_refusal.as_ref()).or(cause.allocation_refusal.as_ref())
                .or(cause.work_refusal.as_ref()).or(cause.work_overrun.as_ref()).or(cause.receipt_refusal.as_ref())
                .cloned().unwrap_or_else(|| ValueError::invariant("native image receiving failed without a cause"))
                .with_retained_progress(progress)
        }), turn.work),
    };
    let progress = match &result { Ok(progress) => *progress, Err(error) => error.retained_progress() };
    let receipt = cx.consume_receiving_retained(issued, progress);
    cx.consume_fuel(u64::try_from(work).unwrap());
    match result {
        Ok(_) => Ok(receipt?),
        Err(error) => Err(OriginalTurnFailure { receipt_refusal: receipt.err(), ..OriginalTurnFailure::from(error) }),
    }
}

#[test]
fn original_arc_quote_denial_keeps_input_before_allocating() {
    let mut caller=caller();
    let mut input=Some(image(1,1,vec![1,2,3,4]));
    let pointer=input.as_ref().unwrap().frames[0].rgba8.as_ptr();
    let mut output=None;
    let mut owner=NativeSharedAdmission::default();
    let denied=RetainedCloneGrant { maximum_capacity_bytes:0, ..loan() };
    let error=caller.turn(denied,|cx|arc_turn(cx,&mut input,&mut output,&mut owner,true))
        .expect_err("unfunded original Arc quote must refuse");
    assert_eq!(error.retained_progress(),Default::default());
    caller.retire_owned(error,loan());
    assert!(output.is_none() && !owner.has_birth());
    assert_eq!(input.as_ref().unwrap().frames[0].rgba8.as_ptr(),pointer);
    let birth=caller.turn(loan(),|cx|arc_turn(cx,&mut input,&mut output,&mut owner,true)).unwrap();
    assert_eq!(birth.retained_capacity_bytes,RasterImagePayload::owned_birth_demands().capacity_bytes);
    assert!(output.is_none() && owner.has_birth());
    caller.turn(loan(),|cx|arc_turn(cx,&mut input,&mut output,&mut owner,true)).unwrap();
    assert!(input.is_none() && owner.terminal_is_empty());
    let output=output.unwrap();
    assert_eq!(output.frames[0].rgba8.as_ptr(),pointer);
    close_alias(&mut caller,output);
    caller.finish();
}

#[test]
fn born_original_arc_survives_enclosing_receipt_refusal_and_funded_close() {
    let mut caller=caller();
    let mut input=Some(image(1,1,vec![1,2,3,4]));
    let pointer=input.as_ref().unwrap().frames[0].rgba8.as_ptr();
    let mut output=None;
    let mut owner=NativeSharedAdmission::default();
    let result=caller.turn(loan(),|cx|arc_turn(cx,&mut input,&mut output,&mut owner,false));
    let error=result.expect_err("omitted actual allocation receipt must refuse");
    assert!(owner.has_birth() && output.is_none());
    assert_eq!(input.as_ref().unwrap().frames[0].rgba8.as_ptr(),pointer);
    caller.retire_owned(error,loan());
    while !owner.terminal_is_empty() {
        caller.turn(loan(), |cx| {
            let demand=owner.close_demand()?;
            semio_framework_job::close_original_effect_step(cx,demand,|effect|owner.close_step(effect))?;
            Ok(())
        }).unwrap();
    }
    caller.retire_owned(input.take().unwrap(),loan());
    caller.finish();
}

#[test]
fn original_native_payload_cancels_at_every_actual_receiving_turn() {
    let input=image(1,1,vec![255,128,64,32]).to_value();
    let maximum=fixture()["maximumNativeTurns"].as_u64().unwrap() as usize;
    let mut baseline=caller();
    let original=source(&mut baseline,input.clone());
    let mut owner=NativeReceivingOriginal::<_,RasterImagePayload>::new(original);
    let mut turns=0;
    while !owner.output_ready() {
        baseline.turn(loan(),|cx| receive_turn(cx,&mut owner)).unwrap();
        turns+=1;
        assert!(turns<=maximum);
    }
    baseline.retire_owned(owner,loan());
    baseline.finish();
    for cut in 0..=turns {
        let mut caller=caller();
        let original=source(&mut caller,input.clone());
        let mut owner=NativeReceivingOriginal::<_,RasterImagePayload>::new(original);
        for _ in 0..cut { caller.turn(loan(),|cx|receive_turn(cx,&mut owner)).unwrap(); }
        caller.cancel().cancel_now();
        if !owner.output_ready() {
            let error=caller.turn(loan(),|cx|receive_turn(cx,&mut owner))
                .expect_err("original cancellation must refuse the next construction turn");
            assert!(format!("{error:?}").to_lowercase().contains("cancel"));
            caller.retire_owned(error,loan());
        }
        caller.retire_owned(owner,loan());
        caller.finish();
    }
}
