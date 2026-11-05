//! 🧪️ Paid descriptor receiving agrees with the neutral corpus and keeps cancellation and denied frontiers allocation-free.
use super::*;
use semio_framework_value::{DslValue, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome};
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneSource, RetainedCloneSourceAdmission};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_native_owned::OriginalReceivingPrimary;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeReceiveSlot;
fn corpus()->&'static str{semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🧫️mutation-inputs/🔣️.json")}


struct Fixture { caller: OriginalJobFixtureCaller, decoder: OriginalJobDecodeOwner }
impl Fixture {
    fn stock() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 2_000_000, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 4096 } }
    fn new() -> Self { Self { caller: OriginalJobFixtureCaller::admit(OperationId(881),Generation(1),Self::stock(),||Some(1),u64::MAX,2_000_000), decoder: OriginalJobDecodeOwner::empty() } }
    fn finish(&mut self) {
        for _ in 0..2_000_000 {
            if self.decoder.terminal_is_empty() { return; }
            self.caller.turn(Self::stock(),|cx|cx.with_original_retirement(|cx|{
                let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
                semio_framework_job::close_original_retained_step(cx,demand,|grant|self.decoder.close_step(grant))?;
                Ok(())
            })).unwrap();
        }
        panic!("original decoder did not retire");
    }
}
thread_local! { static SOURCE_CALLER: std::cell::RefCell<Option<Fixture>> = const { std::cell::RefCell::new(None) }; }
fn grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 } }
trait Work { fn work(&self) -> u64; }
impl<T,E> Work for RetainedWorkOutcome<T,E> { fn work(&self)->u64 { self.work } }
fn observed<T:Work>(run:impl FnOnce()->(T,RetainedCloneProgress))->T {
    SOURCE_CALLER.with(|owner| {
        let mut owner=owner.borrow_mut(); let fixture=owner.as_mut().unwrap();
        fixture.caller.turn_with_decoder(grant(),&mut fixture.decoder,|cx|{
            let ((result,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);
            assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
            cx.consume_retained(receipt)?; cx.consume_fuel(result.work()); Ok(result)
        }).unwrap()
    })
}
fn observed_native(slot:&mut NativeReceiveSlot<DslValue,Vec<ActionArgDef>>,source:&RetainedCloneSource<DslValue>,issued:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary> {
    SOURCE_CALLER.with(|owner| {
        let mut owner=owner.borrow_mut(); let fixture=owner.as_mut().unwrap();
        fixture.caller.turn_with_decoder(grant(),&mut fixture.decoder,|cx|{
            let mut output=None;
            cx.with_original_work_loan(4096,cx.deadline_us(),|cx|{
                let ((turn,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
                    let mut work=0; let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control| { let turn=slot.advance(source.borrow(),control,4096,issued); work=turn.work; turn.result });
                    let turn=RetainedWorkOutcome { result,work };
                    let receipt=received(&turn); (turn,receipt)
                });
                assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
                cx.consume_receiving_retained(issued,receipt)?; cx.consume_fuel(turn.work); output=Some(turn); Ok(())
            })?; Ok(output.unwrap())
        }).unwrap()
    })
}
fn finish() {
    SOURCE_CALLER.with(|owner| { let mut fixture=owner.borrow_mut().take().unwrap(); fixture.finish(); fixture.caller.retire(Fixture::stock()); });
}
fn plain(turn: &RetainedWorkOutcome) -> RetainedCloneProgress { match &turn.result { Ok(step) => step.progress(), Err(error) => error.retained_progress() } }
fn received(turn: &RetainedWorkOutcome<RetainedCloneStep, OriginalReceivingPrimary>) -> RetainedCloneProgress { match &turn.result { Ok(step) => step.progress(), Err(error) => error.progress } }
fn capture(value: DslValue) -> RetainedCloneSource<DslValue> {
    SOURCE_CALLER.with(|owner| { assert!(owner.borrow().is_none()); *owner.borrow_mut()=Some(Fixture::new()); });
    let mut admission = RetainedCloneSourceAdmission::new(value, (), |value| value);
    for _ in 0..64 {
        if admission.phase() == "ready" { break; }
        observed(|| { let turn = admission.advance(grant()); let receipt = plain(&turn); (turn, receipt) }).result.unwrap();
    }
    assert_eq!(admission.phase(), "ready");
    let mut source = None;
    observed(|| { let turn = admission.take_output(&mut source, grant()); let receipt = plain(&turn); (turn, receipt) }).result.unwrap();
    assert!(admission.terminal_is_empty());
    source.unwrap()
}
fn close_owner<T: RetireOwned>(value: T) {
    let mut value = Some(value); let mut close = RetainedCloneClose::default();
    for _ in 0..2_000_000 {
        if value.is_none() && close.is_empty() { return; }
        observed(|| {
            let turn = if close.is_empty() { { let turn = close.begin_granted(&mut value, grant()); RetainedWorkOutcome { result: turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))), work: turn.work } } } else { close.step_granted(grant()) };
            let receipt = plain(&turn); (turn, receipt)
        }).result.unwrap();
    }
    panic!("original descriptor owner did not retire");
}
fn close_slot(slot: &mut NativeReceiveSlot<DslValue, Vec<ActionArgDef>>) {
    slot.begin_close();
    for _ in 0..2_000_000 {
        if slot.terminal_is_empty() { return; }
        observed(|| { let turn = slot.close_step(grant()); let receipt = plain(&turn); (turn, receipt) }).result.unwrap();
    }
    panic!("descriptor receiver did not retire");
}
fn normalized(mut value: serde_json::Value) -> serde_json::Value {
    match &mut value {
        serde_json::Value::Array(values) => for value in values { *value = normalized(value.take()); },
        serde_json::Value::Object(fields) => for value in fields.values_mut() { *value = normalized(value.take()); },
        serde_json::Value::Number(number) => if let Some(value) = number.as_f64().filter(|value| value.fract() == 0.0 && value.abs() <= 9_007_199_254_740_991.0) { *number = serde_json::Number::from(value as i64); },
        _ => {},
    }
    value
}
#[test]
fn actual_paid_input_receiving_matches_all_neutral_ready_cases() {
    let corpus: serde_json::Value = serde_json::from_str(corpus()).unwrap();
    let mut count = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let Some(expected) = case.get("expectedInputs") else { continue };
        count += 1;
        let source = capture(DslValue::from(expected));
        let original = source.borrow().get() as *const DslValue;
        let mut slot = NativeReceiveSlot::<DslValue, Vec<ActionArgDef>>::default();
        for step in 0usize..2_000_000 {
            if slot.ready() { break; }
            let (demand, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| slot.demands(source.borrow(), 4096));
            let demand = demand.unwrap(); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if step < 16 || step.is_power_of_two() || demand.capacity_bytes > 0 || demand.release_bytes > 0 {
                for axis in 0..5 {
                    let mut denied = grant();
                    match axis { 0 => denied.maximum_items = 0, 1 if demand.copy_bytes > 0 => denied.maximum_copy_bytes = demand.copy_bytes - 1, 2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1, 3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1, 4 if demand.depth > 0 => denied.maximum_depth = demand.depth - 1, _ => continue }
                    let turn = observed_native(&mut slot,&source,denied);
                    assert_eq!(turn.work, 0); assert_eq!(turn.result.unwrap().progress(), Default::default());
                    assert_eq!(slot.demands(source.borrow(), 4096).unwrap(), demand);
                    assert_eq!(source.borrow().get() as *const DslValue, original);
                }
            }
            let turn = observed_native(&mut slot,&source,grant());
            if let Err(error) = turn.result { close_slot(&mut slot); close_owner(source); panic!("{} native descriptor refusal: {error:?}", case["name"]); }
            assert!(step < 1_999_999);
        }
        assert!(slot.ready());
        let turn = observed(|| { let turn = slot.take(grant()); let receipt = match &turn.result { Ok(Some((_, receipt))) => *receipt, Ok(None) => Default::default(), Err(error) => error.retained_progress() }; (turn, receipt) });
        let (actual, _) = turn.result.unwrap().unwrap();
        close_slot(&mut slot);
        assert_eq!(normalized(serde_json::to_value(&actual).unwrap()), normalized(expected.clone()), "{}", case["name"]);
        close_owner(actual); close_owner(source); finish();
    }
    assert!(count >= 18);
}
#[test]
fn cancelling_native_input_receiving_closes_every_selected_prefix_without_losing_the_original() {
    let corpus: serde_json::Value = serde_json::from_str(corpus()).unwrap();
    let expected = corpus["cases"].as_array().unwrap().iter().filter_map(|case| case.get("expectedInputs")).max_by_key(|value| value.to_string().len()).unwrap();
    for cut in [0, 1, 2, 4, 8, 16, 32, 64, 128, 256] {
        let source = capture(DslValue::from(expected));
        let original = source.borrow().get() as *const DslValue;
        let mut slot = NativeReceiveSlot::<DslValue, Vec<ActionArgDef>>::default();
        for _ in 0..cut {
            if slot.ready() { break; }
            observed_native(&mut slot,&source,grant()).result.unwrap();
        }
        assert_eq!(source.borrow().get() as *const DslValue, original);
        close_slot(&mut slot); close_owner(source); finish();
    }
}

fn static_paid_source<T>(text:&str,expected:&serde_json::Value)
where T:NativeReceive<DslValue>+Sync+serde::Serialize+'static{
    use semio_framework_pack_json::JsonReadLimits;
    use semio_framework_pack_json_native_input::JsonNativeInputCursor;
    let pointer=text.as_ptr();
    SOURCE_CALLER.with(|owner|{assert!(owner.borrow().is_none());*owner.borrow_mut()=Some(Fixture::new());});
    let mut cursor=None;
    SOURCE_CALLER.with(|owner|{
        let mut owner=owner.borrow_mut();let fixture=owner.as_mut().unwrap();
        fixture.caller.turn_with_decoder(grant(),&mut fixture.decoder,|cx|{
            let ((result,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
                let result=cx.with_native_receiving::<_,ValueError>(|control|JsonNativeInputCursor::<semio_framework_value::shared_issuance::SealedShared<T>>::admit(&text,JsonReadLimits{maximum_bytes:text.len() as u64,maximum_allocation_bytes:1<<20,maximum_depth:4096,maximum_items:2_000_000},control,grant()));
                let receipt=result.as_ref().ok().and_then(|value|value.as_ref()).map_or(Default::default(),|(_,receipt)|*receipt);
                (result,receipt)
            });
            assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
            cx.consume_receiving_retained(grant(),receipt)?;cx.consume_fuel(1);cursor=result?.map(|(cursor,_)|cursor);Ok(())
        }).unwrap();
    });
    let mut cursor=cursor.unwrap();
    for step in 0..2_000_000 {
        if cursor.ready(){break}
        SOURCE_CALLER.with(|owner|{
            let mut owner=owner.borrow_mut();let fixture=owner.as_mut().unwrap();
            fixture.caller.turn_with_decoder(grant(),&mut fixture.decoder,|cx|{
                cx.with_original_work_loan(4096,cx.deadline_us(),|cx|{
                    let ((turn,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
                        let mut work=0;let result=cx.with_native_receiving::<_,ValueError>(|control|{let turn=cursor.advance(control,4096,grant());work=turn.work;turn.result});
                        let turn=RetainedWorkOutcome{result,work};let receipt=plain(&turn);(turn,receipt)
                    });
                    assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
                    cx.consume_receiving_retained(grant(),receipt)?;cx.consume_fuel(turn.work);turn.result?;Ok(())
                })
            }).unwrap();
        });
        assert_eq!(cursor.source().as_ptr(),pointer);assert!(step<1_999_999);
    }
    let mut output=None;
    SOURCE_CALLER.with(|owner|{
        let mut owner=owner.borrow_mut();let fixture=owner.as_mut().unwrap();
        fixture.caller.turn_with_decoder(grant(),&mut fixture.decoder,|cx|{
            let result=cx.with_native_receiving::<_,ValueError>(|control|Ok(cursor.take(control,grant())));
            let turn=result?;let Some((value,receipt))=turn.result? else{panic!("ready native descriptor input must transfer")};
            cx.consume_receiving_retained(grant(),receipt)?;cx.consume_fuel(turn.work);output=Some(value);Ok(())
        }).unwrap();
    });
    for step in 0..2_000_000 {
        if cursor.terminal_is_empty(){break}
        SOURCE_CALLER.with(|owner|{
            let mut owner=owner.borrow_mut();let fixture=owner.as_mut().unwrap();
            fixture.caller.turn_with_decoder(grant(),&mut fixture.decoder,|cx|cx.with_original_retirement(|cx|{
                let ((result,work,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
                    let mut work=0;let result=cx.with_native_retirement(|control|{let turn=cursor.close_step(control,grant());work=turn.work;turn.result});
                    let receipt=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};(result,work,receipt)
                });
                assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
                cx.consume_retained(receipt)?;cx.consume_fuel(work);result?;Ok(())
            })).unwrap();
        });
        assert!(step<1_999_999);
    }
    let output=output.unwrap();
    assert_eq!(normalized(serde_json::to_value(output.get()).unwrap()),normalized(expected.clone()));
    let pointer=output.get() as *const T;
    let mut admission=RetainedCloneSourceAdmission::new(output,(),|cache|cache.get());
    for _ in 0..64 {
        if admission.phase()=="ready"{break}
        observed(||{let turn=admission.advance(grant());let receipt=plain(&turn);(turn,receipt)}).result.unwrap();
    }
    assert_eq!(admission.phase(),"ready");
    let mut source=None;
    observed(||{let turn=admission.take_output(&mut source,grant());let receipt=plain(&turn);(turn,receipt)}).result.unwrap();
    let source=source.unwrap();
    assert_eq!(source.borrow().get() as *const T,pointer);assert!(admission.terminal_is_empty());
    close_owner(source);finish();
}

#[test]
fn static_json_descriptor_inputs_have_a_paid_native_source_lifecycle(){
 let corpus:serde_json::Value=serde_json::from_str(corpus()).unwrap();
 let expected=corpus["cases"].as_array().unwrap().iter().filter_map(|case|case.get("expectedInputs")).max_by_key(|value|value.to_string().len()).unwrap();
 static_paid_source::<Vec<ActionArgDef>>(&expected.to_string(),expected);
}
#[test]
fn static_json_shape_seeds_have_a_paid_native_source_lifecycle(){
 let corpus:serde_json::Value=serde_json::from_str(corpus()).unwrap();
 let original=corpus["cases"].as_array().unwrap().iter().filter_map(|case|case.get("expectedInputs")).max_by_key(|value|value.to_string().len()).unwrap();
 let inputs:Vec<ActionArgDef>=serde_json::from_value(original.clone()).unwrap();
 let seeds=crate::action_arg_shape_seeds(&inputs);let expected=serde_json::to_value(&seeds).unwrap();
 let text=semio_framework_pack_json::to_json_string(&seeds.to_value());
 static_paid_source::<Vec<crate::ActionArgShapeSeed>>(&text,&expected);
}

#[test]
fn static_json_singleton_seed_arrays_have_a_paid_original_native_source_lifecycle(){
 let laws:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/singleton-cases.json")).unwrap();
 for row in laws["cases"].as_array().unwrap().iter().filter(|row|!row["singleton"].is_null()){
  let schema:crate::ArgSchema=serde_json::from_value(row["schema"].clone()).unwrap();
  let mut input=ActionArgDef::with_schema("/items",crate::LocalizedLabel::native("Items","Einträge"),schema);input.default=row.get("default").map(|value|serde_json::from_value(value.clone()).unwrap());
  let seeds=crate::action_arg_shape_seeds(&[input]);let expected=serde_json::to_value(&seeds).unwrap();assert_eq!(expected[0]["singleton"],row["singleton"]);
  let text=semio_framework_pack_json::to_json_string(&seeds.to_value());static_paid_source::<Vec<crate::ActionArgShapeSeed>>(&text,&expected);
 }
}
