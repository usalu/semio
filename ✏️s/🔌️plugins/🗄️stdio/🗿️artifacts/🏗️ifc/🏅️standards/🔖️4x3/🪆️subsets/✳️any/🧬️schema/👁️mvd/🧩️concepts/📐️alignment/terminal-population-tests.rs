//! 🚆️ Actual ordered cursors borrow canonical preparation and its source-qualified request index.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneClose;
fn grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 512 } }
fn source() -> Arc<IfcSnapshot> { Arc::new(super::super::tests::fixture(&serde_json::json!({"kind":"horizontal","lengths":[10,0]}))) }
fn input() -> Arc<IfcAlignmentTerminalRequest> { super::super::tests::request("1", "horizontal", Some(true)) }
fn retire<T: semio_framework_value::retirement::RetireOwned>(caller: &mut OriginalWorkFixture, owner: &mut Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..100000 {
        if owner.is_none() && close.is_empty() { return; }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
            let demand = close.next_owner_demands::<T>(owner.is_some(), &None, 32768)?;
            semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(owner, grant)?.ok_or_else(|| invalid("mvd.terminal.test-close")) } else { close.step_granted(grant) })?; Ok(())
        })).unwrap();
    }
    panic!("Terminal population test did not retire its original owners");
}
fn prepare(caller: &mut OriginalWorkFixture, work: &mut IfcAlignmentTerminalWork, source: &Arc<IfcSnapshot>, input: &Arc<IfcAlignmentTerminalRequest>) {
    for _ in 0..100000 { if work.preparation.ready() { return; } assert!(!caller.work_turn(grant(), 1, |cx| work.advance(source, input, 31, cx)).unwrap()); }
    panic!("Terminal population did not finish preparation");
}
#[test]
fn native_terminal_actual_cursors_share_canonical_preparation_and_request_index_after_move() {
    let source = source(); let input = input(); let mut caller = OriginalWorkFixture::new(); let mut work = IfcAlignmentTerminalWork::default();
    prepare(&mut caller, &mut work, &source, &input);
    let (pin, request_pin) = caller.work_turn(grant(), 1, |cx| work.preparation.with_prepared(31, cx, |view, requests, cx| Ok((view.pin(cx)?, requests.pin(cx)?)))).unwrap().unwrap();
    let mut second = IfcAlignmentTerminalClause::default(); let mut moved = work; let mut first_done = false; let mut second_done = false;
    for _ in 0..100000 {
        if !first_done { first_done = caller.work_turn(grant(), 1, |cx| moved.advance(&source, &input, 31, cx)).unwrap(); }
        if !second_done { second_done = caller.work_turn(grant(), 1, |cx| moved.preparation.with_prepared(31, cx, |view, requests, cx| { assert!(pin.matches(&view, cx)); assert!(request_pin.matches(&requests, cx)); second.advance_prepared(view, requests, 31, cx) })).unwrap().unwrap(); }
        assert_eq!(Arc::strong_count(&source), 2); assert_eq!(Arc::strong_count(&input), 2); if first_done && second_done { break; }
    }
    assert!(first_done && second_done); let mut first_report = None; let mut second_report = None;
    assert!(caller.work_turn(grant(), 1, |cx| moved.take(&mut first_report, cx)).unwrap());
    assert!(caller.work_turn(grant(), 1, |cx| moved.preparation.with_prepared(31, cx, |view, requests, cx| second.take_prepared(view, requests, &mut second_report, cx))).unwrap().unwrap());
    assert_eq!(first_report, second_report); assert_eq!(first_report.as_ref().unwrap().status, "pass");
    retire(&mut caller, &mut first_report); retire(&mut caller, &mut second_report); retire(&mut caller, &mut Some(second)); retire(&mut caller, &mut Some(moved)); assert_eq!(Arc::strong_count(&source), 1); assert_eq!(Arc::strong_count(&input), 1); caller.finish();
}
#[test]
fn native_terminal_request_index_from_another_population_and_wrong_epoch_refuse_before_effects() {
    let source = source(); let input = input(); let mut caller = OriginalWorkFixture::new(); let mut work = IfcAlignmentTerminalWork::default(); prepare(&mut caller, &mut work, &source, &input);
    let mut clause = IfcAlignmentTerminalClause::default(); let before = clause.metadata();
    caller.work_turn(grant(), 1, |cx| { let receipt = cx.retained_progress(); assert_eq!(work.preparation.with_unavailable(31, cx, |_, _, _| -> Result<(), semio_framework_job::OriginalTurnFailure> { panic!("prepared input downgraded") })?, None); assert!(work.preparation.with_prepared(31, cx, |view, requests, cx| clause.advance_prepared(view, requests, 32, cx)).is_err()); assert_eq!(receipt, cx.retained_progress()); Ok(()) }).unwrap(); assert_eq!(before, clause.metadata());
    let mut other = IfcMvdPreparation::default(); for _ in 0..100000 { if caller.work_turn(grant(), 1, |cx| other.advance(&source, &input, 31, cx)).unwrap() { break; } }
    caller.work_turn(grant(), 1, |cx| {
        let receipt = cx.retained_progress();
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.preparation.with_prepared(31, cx, |view, _, cx| other.with_prepared(31, cx, |_, requests, cx| clause.advance_prepared(view, requests, 31, cx))));
        assert!(result.is_err()); assert_eq!(receipt, cx.retained_progress()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); Ok(())
    }).unwrap(); assert_eq!(before, clause.metadata()); retire(&mut caller, &mut Some(other)); retire(&mut caller, &mut Some(clause)); retire(&mut caller, &mut Some(work)); caller.finish();
}
#[test]
fn native_terminal_first_accepted_request_retains_and_donates_one_snapshot_without_model_reads() {
    for valid in [true, false] {
        let source = source(); let mut input = input(); if !valid { Arc::get_mut(&mut input).unwrap().targets[0].layout = "01".into(); }
        let mut caller = OriginalWorkFixture::new(); let mut work = IfcAlignmentTerminalWork::default(); assert!(!caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap());
        assert_eq!(Arc::strong_count(&source), 2); assert!(work.preparation.born_snapshot()); assert_eq!(work.preparation.stage(), 1);
        assert_eq!(caller.work_turn(grant(), 1, |cx| work.preparation.with_prepared(31, cx, |_, _, _| Ok(()))).unwrap(), None);
        if valid { prepare(&mut caller, &mut work, &source, &input); assert!(!work.preparation.born_snapshot()); assert_eq!(Arc::strong_count(&source), 2); }
        else { let mut refused = false; for _ in 0..100000 { if caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).is_err() { refused = true; break; } } assert!(refused && work.preparation.born_snapshot()); assert_eq!(work.preparation.stage(), 1); }
        caller.cancel().cancel_now(); retire(&mut caller, &mut Some(work)); assert_eq!(Arc::strong_count(&source), 1); assert_eq!(Arc::strong_count(&input), 1); caller.finish();
    }
}
