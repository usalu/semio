//! 🪢️ Actual Direct clause cursors share their original wrapper's one stable prepared population.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneClose;
fn grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 512 } }
fn retire<T: semio_framework_value::retirement::RetireOwned>(caller: &mut OriginalWorkFixture, owner: &mut Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..100000 {
        if owner.is_none() && close.is_empty() { return; }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
            let demand = close.next_owner_demands::<T>(owner.is_some(), &None, 32768)?;
            semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(owner, grant)?.ok_or_else(|| invalid("mvd.direct.test-close")) } else { close.step_granted(grant) })?;
            Ok(())
        })).unwrap();
    }
    panic!("Direct population test did not retire its original owners");
}
#[test]
fn native_direct_actual_local_cursors_share_one_index_and_survive_parent_move() {
    let source = Arc::new(super::super::tests::fixture("zero-length"));
    let input = super::super::tests::request("horizontal", true);
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcAlignmentDirectWork::default();
    for _ in 0..100000 {
        if work.preparation.ready() { break; }
        assert!(!caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap());
    }
    assert!(work.preparation.ready());
    assert!(!work.preparation.born_snapshot());
    let before = caller.work_turn(grant(), 1, |cx| work.preparation.with_prepared(31, cx, |view, _requests, cx| Ok(view.pin(cx)?))).unwrap().unwrap();
    let mut second = IfcAlignmentDirectClause::default();
    let mut moved = work;
    let mut first_done = false;
    let mut second_done = false;
    for _ in 0..100000 {
        if !first_done { first_done = caller.work_turn(grant(), 1, |cx| moved.advance(&source, &input, 31, cx)).unwrap(); }
        if !second_done {
            second_done = caller.work_turn(grant(), 1, |cx| moved.preparation.with_prepared(31, cx, |view, _requests, cx| {
                assert!(before.matches(&view, cx));
                second.advance_prepared(view, 31, cx)
            })).unwrap().unwrap();
        }
        assert_eq!(Arc::strong_count(&source), 2);
        assert_eq!(Arc::strong_count(&input), 2);
        if first_done && second_done { break; }
    }
    assert!(first_done && second_done);
    let mut first_report = None;
    let mut second_report = None;
    assert!(caller.work_turn(grant(), 1, |cx| moved.take(&mut first_report, cx)).unwrap());
    assert!(caller.work_turn(grant(), 1, |cx| moved.preparation.with_prepared(31, cx, |view, _requests, cx| second.take_prepared(view, &mut second_report, cx))).unwrap().unwrap());
    assert_eq!(first_report, second_report);
    assert_eq!(first_report.as_ref().unwrap().status, "pass");
    retire(&mut caller, &mut first_report);
    retire(&mut caller, &mut second_report);
    retire(&mut caller, &mut Some(second));
    retire(&mut caller, &mut Some(moved));
    assert_eq!(Arc::strong_count(&source), 1);
    assert_eq!(Arc::strong_count(&input), 1);
    caller.finish();
}
#[test]
fn native_direct_request_only_borrow_cannot_downgrade_a_prepared_population() {
    let source = Arc::new(super::super::tests::fixture("zero-length"));
    let input = super::super::tests::request("horizontal", true);
    let mut caller = OriginalWorkFixture::new();
    let mut work = IfcAlignmentDirectWork::default();
    for _ in 0..100000 { if work.preparation.ready() { break; } caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap(); }
    let mut clause = IfcAlignmentDirectClause::default();
    let before = clause.metadata();
    caller.work_turn(grant(), 1, |cx| {
        let receipt = cx.retained_progress();
        assert_eq!(work.preparation.with_unavailable(31, cx, |view, _requests, cx| clause.advance_unavailable(view, 31, cx))?, None);
        assert_eq!(receipt, cx.retained_progress());
        assert!(work.preparation.with_prepared(31, cx, |view, _requests, cx| clause.advance_prepared(view, 32, cx)).is_err());
        assert_eq!(receipt, cx.retained_progress());
        Ok(())
    }).unwrap();
    assert_eq!(before, clause.metadata());
    assert!(!caller.work_turn(grant(), 1, |cx| work.preparation.with_prepared(31, cx, |view, _requests, cx| clause.advance_prepared(view, 31, cx))).unwrap().unwrap());
    let mut other = IfcAlignmentDirectWork::default();
    for _ in 0..100000 { if other.preparation.ready() { break; } caller.work_turn(grant(), 1, |cx| other.advance(&source, &input, 31, cx)).unwrap(); }
    let before = clause.metadata();
    caller.work_turn(grant(), 1, |cx| {
        let receipt = cx.retained_progress();
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| other.preparation.with_prepared(31, cx, |view, _requests, cx| clause.advance_prepared(view, 31, cx)));
        assert!(result.is_err());
        assert_eq!(receipt, cx.retained_progress());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        Ok(())
    }).unwrap();
    assert_eq!(before, clause.metadata());
    retire(&mut caller, &mut Some(other));
    retire(&mut caller, &mut Some(clause));
    retire(&mut caller, &mut Some(work));
    caller.finish();
}
#[test]
fn native_direct_first_accepted_request_turn_retains_one_source_alias_before_validation() {
    for valid in [true, false] {
        let source = Arc::new(super::super::tests::fixture("zero-length"));
        let mut input = super::super::tests::request("horizontal", true);
        if !valid { Arc::get_mut(&mut input).unwrap().targets[0].segment = "01".into(); }
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcAlignmentDirectWork::default();
        assert!(!caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap());
        assert_eq!(Arc::strong_count(&source), 2);
        assert!(work.preparation.born_snapshot());
        assert_eq!(work.preparation.stage(), 1);
        assert_eq!(caller.work_turn(grant(), 1, |cx| work.preparation.with_prepared(31, cx, |_, _, _| Ok(()))).unwrap(), None);
        if valid {
            for _ in 0..100000 {
                if work.preparation.ready() { break; }
                assert!(!caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).unwrap());
                assert_eq!(Arc::strong_count(&source), 2);
            }
            assert!(work.preparation.ready());
            assert!(!work.preparation.born_snapshot());
        } else {
            let mut refused = false;
            for _ in 0..100000 { if caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 31, cx)).is_err() { refused = true; break; } }
            assert!(refused);
            assert!(work.preparation.born_snapshot());
            assert_eq!(work.preparation.stage(), 1);
            assert_eq!(caller.work_turn(grant(), 1, |cx| work.preparation.with_prepared(31, cx, |_, _, _| Ok(()))).unwrap(), None);
        }
        caller.cancel().cancel_now();
        retire(&mut caller, &mut Some(work));
        assert_eq!(Arc::strong_count(&source), 1);
        assert_eq!(Arc::strong_count(&input), 1);
        caller.finish();
    }
}
