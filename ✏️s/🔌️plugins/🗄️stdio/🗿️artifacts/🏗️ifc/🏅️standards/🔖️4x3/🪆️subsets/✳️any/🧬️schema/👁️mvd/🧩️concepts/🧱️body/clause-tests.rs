//! 🧪️ Body clauses authenticate both actual route views before local progress and donation.
use super::*;
use super::clause::IfcBodyClause;
use super::super::preparation::IfcMvdPreparation;
use super::tessellation_tests::{graph, loan, request};
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::{retained_clone::RetainedCloneClose, retirement::RetireOwned};
fn prepare(caller: &mut OriginalWorkFixture, source: &Arc<IfcSnapshot>, input: &Arc<IfcBodyConceptRequest>) -> IfcMvdPreparation<IfcBodyConceptRequest> {
    let mut work = IfcMvdPreparation::default();
    for _ in 0..100000 { if caller.work_turn(loan(), 1, |cx| work.advance(source, input, 71, cx)).unwrap() { return work; } }
    panic!("Body request/source preparation did not complete");
}
fn retire<T: RetireOwned>(caller: &mut OriginalWorkFixture, owner: &mut Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..100000 {
        if owner.is_none() && close.is_empty() { return; }
        caller.turn(loan(), |cx| cx.with_original_retirement(|cx| {
            let demand = close.next_owner_demands::<T>(owner.is_some(), &None, 32768)?;
            semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(owner, grant)?.ok_or_else(|| invalid("mvd.body.test-close")) } else { close.step_granted(grant) })?;
            Ok(())
        })).unwrap();
    }
    panic!("Body clause did not retire all original owners");
}
#[test]
fn native_body_actual_prepared_index_cannot_be_replaced_before_or_after_completion() {
    let source = graph(&serde_json::json!({"identifier":"Body","type":"SweptSolid","items":["IfcExtrudedAreaSolid"]}));
    let input = request(&[("1", Some(true))]);
    let mut caller = OriginalWorkFixture::new();
    let first = prepare(&mut caller, &source, &input);
    let second = prepare(&mut caller, &source, &input);
    let mut clause = IfcBodyClause::new(BodyPolicy::SweptSolid);
    let before = clause.metadata();
    caller.work_turn(loan(), 1, |cx| {
        let receipt = cx.retained_progress();
        assert!(first.with_prepared(71, cx, |view, _requests, cx| {
            second.with_prepared(71, cx, |_other, requests, cx| clause.advance_prepared(view, requests, 71, cx)).map(|done| done.unwrap())
        }).is_err());
        assert_eq!(receipt, cx.retained_progress());
        Ok(())
    }).unwrap();
    assert_eq!(before, clause.metadata());
    let mut done = false;
    for _ in 0..100000 {
        if caller.work_turn(loan(), 1, |cx| first.with_prepared(71, cx, |view, requests, cx| clause.advance_prepared(view, requests, 71, cx))).unwrap().unwrap() { done = true; break; }
    }
    assert!(done);
    let before = clause.metadata();
    let mut output = None;
    caller.work_turn(loan(), 1, |cx| {
        let receipt = cx.retained_progress();
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| second.with_prepared(71, cx, |view, requests, cx| clause.advance_prepared(view, requests, 71, cx)));
        assert!(result.is_err());
        assert!(first.with_prepared(71, cx, |view, _requests, cx| second.with_prepared(71, cx, |_other, requests, cx| clause.take_prepared(view, requests, &mut output, cx)).map(|done| done.unwrap())).is_err());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(receipt, cx.retained_progress());
        Ok(())
    }).unwrap();
    assert_eq!(before, clause.metadata());
    assert!(output.is_none());
    let mut foreign = OriginalWorkFixture::new();
    assert!(foreign.work_turn(loan(), 1, |cx| first.with_prepared(71, cx, |view, requests, cx| clause.take_prepared(view, requests, &mut output, cx))).is_err());
    foreign.finish();
    assert!(caller.work_turn(loan(), 1, |cx| first.with_prepared(71, cx, |view, requests, cx| clause.take_prepared(view, requests, &mut output, cx))).unwrap().unwrap());
    assert_eq!(output.as_ref().unwrap().status, "pass");
    retire(&mut caller, &mut output);
    retire(&mut caller, &mut Some(clause));
    retire(&mut caller, &mut Some(second));
    retire(&mut caller, &mut Some(first));
    caller.finish();
    assert_eq!(Arc::strong_count(&source), 1);
    assert_eq!(Arc::strong_count(&input), 1);
}
#[test]
fn native_body_unavailable_route_stays_bound_to_its_actual_request_index_and_provenance() {
    let mut source = graph(&serde_json::json!({"identifier":"Body","type":"SweptSolid","items":["IfcExtrudedAreaSolid"]}));
    Arc::get_mut(&mut source).unwrap().schema = "foreign".into();
    let input = request(&[("1", Some(true)), ("9", Some(false))]);
    let mut caller = OriginalWorkFixture::new();
    let first = prepare(&mut caller, &source, &input);
    let second = prepare(&mut caller, &source, &input);
    let mut clause = IfcBodyClause::new(BodyPolicy::SweptSolid);
    let before = clause.metadata();
    assert_eq!(caller.work_turn(loan(), 1, |cx| first.with_prepared(71, cx, |view, requests, cx| clause.advance_prepared(view, requests, 71, cx))).unwrap(), None);
    assert!(caller.work_turn(loan(), 1, |cx| first.with_unavailable(71, cx, |view, requests, cx| clause.advance_unavailable(view, requests, 72, cx))).is_err());
    assert_eq!(before, clause.metadata());
    assert!(!caller.work_turn(loan(), 1, |cx| first.with_unavailable(71, cx, |view, requests, cx| clause.advance_unavailable(view, requests, 71, cx))).unwrap().unwrap());
    let before = clause.metadata();
    caller.work_turn(loan(), 1, |cx| {
        let receipt = cx.retained_progress();
        assert!(second.with_unavailable(71, cx, |view, requests, cx| clause.advance_unavailable(view, requests, 71, cx)).is_err());
        assert!(first.with_unavailable(71, cx, |view, _requests, cx| second.with_unavailable(71, cx, |_other, requests, cx| clause.advance_unavailable(view, requests, 71, cx)).map(|done| done.unwrap())).is_err());
        assert_eq!(receipt, cx.retained_progress());
        Ok(())
    }).unwrap();
    assert_eq!(before, clause.metadata());
    let mut done = false;
    for _ in 0..100000 { if caller.work_turn(loan(), 1, |cx| first.with_unavailable(71, cx, |view, requests, cx| clause.advance_unavailable(view, requests, 71, cx))).unwrap().unwrap() { done = true; break; } }
    assert!(done);
    let mut output = None;
    caller.work_turn(loan(), 1, |cx| {
        let before = cx.retained_progress();
        assert!(second.with_unavailable(71, cx, |view, requests, cx| clause.take_unavailable(view, requests, &mut output, cx)).is_err());
        assert_eq!(before, cx.retained_progress());
        Ok(())
    }).unwrap();
    assert!(caller.work_turn(loan(), 1, |cx| first.with_unavailable(71, cx, |view, requests, cx| clause.take_unavailable(view, requests, &mut output, cx))).unwrap().unwrap());
    let report = output.as_ref().unwrap();
    assert_eq!(report.concept, BodyPolicy::SweptSolid.concept());
    assert_eq!(report.status, "incomplete");
    assert_eq!(report.diagnostics[0].code, "mvd.source");
    assert_eq!(report.targets[1].status, "not-applicable");
    retire(&mut caller, &mut output);
    retire(&mut caller, &mut Some(clause));
    retire(&mut caller, &mut Some(second));
    retire(&mut caller, &mut Some(first));
    caller.finish();
    assert_eq!(Arc::strong_count(&source), 1);
    assert_eq!(Arc::strong_count(&input), 1);
}
static CLOCK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
fn clock() -> Option<u64> { Some(CLOCK.load(std::sync::atomic::Ordering::SeqCst)) }
#[test]
fn native_body_expired_original_deadline_yields_work_but_pays_every_partial_close() {
    for policy in [BodyPolicy::Tessellation, BodyPolicy::SweptSolid, BodyPolicy::AdvancedSweptSolid, BodyPolicy::SurfaceModel, BodyPolicy::Clipping, BodyPolicy::Brep, BodyPolicy::AdvancedBrep] {
        for prefix in [0, 1, 2, 5, 11, 30, 60] {
            CLOCK.store(1, std::sync::atomic::Ordering::SeqCst);
            let source = graph(&serde_json::json!({"identifier":"Axis","type":policy.representation_type(),"items":["IfcPolyline"]}));
            let input = request(&[("1", Some(true))]);
            let mut caller = OriginalWorkFixture::with_source(semio_framework_job::OperationId(991), semio_framework_job::Generation(17), OriginalWorkFixture::stock(), clock, 100, 1 << 20);
            let mut work = super::native_tests::new_work(policy);
            for _ in 0..prefix { caller.work_turn(loan(), 1, |cx| work.advance(&source, &input, 71, cx)).unwrap(); }
            CLOCK.store(100, std::sync::atomic::Ordering::SeqCst);
            let before = work.metadata();
            caller.turn(loan(), |cx| { let receipt = cx.retained_progress(); assert!(!work.advance(&source, &input, 71, cx)?); assert_eq!(receipt, cx.retained_progress()); Ok(()) }).unwrap();
            assert_eq!(before, work.metadata());
            caller.cancel().cancel_now();
            retire(&mut caller, &mut Some(work));
            caller.finish();
            assert_eq!(Arc::strong_count(&source), 1);
            assert_eq!(Arc::strong_count(&input), 1);
        }
    }
}
