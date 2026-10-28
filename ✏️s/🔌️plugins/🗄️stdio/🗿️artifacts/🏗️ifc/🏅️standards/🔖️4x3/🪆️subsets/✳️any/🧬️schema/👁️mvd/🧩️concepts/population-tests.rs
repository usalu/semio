//! 🧪️ Shared population laws retain one original source and index across borrowed clause turns.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader};
#[derive(Debug, semio_framework_value::RetireOwned)]
struct Request { word: String }
fn grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 512 } }
fn fixture(row: &serde_json::Value) -> Arc<IfcSnapshot> {
    Arc::new(IfcSnapshot { schema: "stdio.ifc".into(), header: IfcHeader { file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String(row["header"].as_str().unwrap().into())])], ..Default::default() }, entities: row["entities"].as_array().unwrap().iter().map(|row| IfcEntity { id: row["id"].as_str().unwrap().parse().unwrap(), name: row["name"].as_str().unwrap().into(), args: row["reference"].as_str().map(|id| vec![IfcValue::Reference(id.parse().unwrap())]).unwrap_or_default(), complex: vec![] }).collect(), ..Default::default() })
}
fn request_donor(caller: &mut OriginalWorkFixture, input: &Arc<Request>) -> Option<IfcMvdPopulationRequestSource<Request>> {
    let mut donor = None;
    let mut unborn = None;
    caller.work_turn(grant(), 1, |cx| {
        let (owner, progress) = match IfcMvdPopulationRequestSource::admit_borrowed(Arc::clone(input), |value| value.as_ref(), cx.retained_grant()) {
            Ok(value) => value,
            Err((error, owner)) => {
                unborn = Some(owner);
                let child = error.retained_progress();
                let mut failure = OriginalTurnFailure::from(error);
                if let Err(error) = cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: child.copied_bytes + size_of::<Arc<Request>>(), ..child }) { failure.context_refusal = Some(error); }
                return Err(failure);
            }
        };
        donor = Some(owner);
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: progress.copied_bytes + size_of::<Arc<Request>>(), ..progress })?;
        cx.consume_fuel(1);
        Ok(())
    }).unwrap();
    assert!(unborn.is_none());
    donor
}
fn retire<T: RetireOwned>(caller: &mut OriginalWorkFixture, owner: &mut Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..100000 {
        if owner.is_none() && close.is_empty() { return; }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
            let demand = close.next_owner_demands::<T>(owner.is_some(), &None, 32768)?;
            semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(owner, grant)?.ok_or_else(|| invalid("population.test-close")) } else { close.step_granted(grant) })?;
            Ok(())
        })).unwrap();
    }
    panic!("population test owner did not retire");
}
fn close(caller: &mut OriginalWorkFixture, work: &mut IfcMvdPopulationWork<Request>) {
    for _ in 0..100000 {
        let demand = work.close_demands().unwrap();
        for axis in 0..5 {
            let mut denied = grant();
            match axis { 0 => denied.maximum_items = 0, 1 => denied.maximum_copy_bytes = demand.copy_bytes - 1, 2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1, 3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1, 2 | 3 => continue, _ => denied.maximum_depth = 0 }
            let before = work.metadata();
            caller.turn(denied, |cx| cx.with_original_retirement(|cx| {
                let receipt = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.close_step(cx));
                assert!(!result?);
                assert_eq!(receipt, cx.retained_progress());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                Ok(())
            })).unwrap();
            assert_eq!(before, work.metadata());
        }
        if caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
            let before = cx.retained_progress();
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.close_step(cx));
            let after = cx.retained_progress();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
            result
        })).unwrap() { assert!(work.terminal_is_empty()); return; }
    }
    panic!("population original close did not complete");
}
fn prepare(caller: &mut OriginalWorkFixture, work: &mut IfcMvdPopulationWork<Request>, source: &Arc<IfcSnapshot>, donor: &mut Option<IfcMvdPopulationRequestSource<Request>>) -> usize {
    assert!(caller.work_turn(grant(), 1, |cx| work.receive(source, donor, 37, cx)).unwrap());
    assert!(donor.is_none());
    assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_view(37, cx, |_, _| -> Result<(), OriginalTurnFailure> { panic!("pending population exposed a view") })).unwrap(), None);
    for turns in 1..100000 {
        if caller.work_turn(grant(), 1, |cx| work.advance(source, 37, cx)).unwrap() { return turns; }
    }
    panic!("population original preparation did not complete");
}
#[derive(Default)]
struct Clause { pin: Option<IfcMvdPopulationPin>, count: usize }
impl Clause {
    fn turn(&mut self, view: IfcMvdPopulationView<'_, Request>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<IfcMvdPopulationLookup, semio_framework_job::OriginalTurnFailure> {
        if self.pin.is_some_and(|pin| !pin.matches(&view, cx)) { return Err(invalid("population.clause-source").into()); }
        let demand = RetirementDemand { copy_bytes: 2 * size_of::<Self>(), depth: 1, ..Default::default() };
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Err(invalid("population.clause-test-grant").into()); }
        let result = view.lookup(9, cx)?;
        assert_eq!(view.request(cx)?.word, "original");
        self.pin = Some(view.pin(cx)?);
        self.count += 1;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(result)
    }
}
#[test]
fn native_population_matches_all_neutral_container_and_exact_id_lookup_cases() {
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let source = fixture(row);
        let input = Arc::new(Request { word: "original".into() });
        let mut caller = OriginalWorkFixture::new();
        let mut donor = request_donor(&mut caller, &input);
        let mut work = IfcMvdPopulationWork::default();
        assert_eq!(Arc::strong_count(&source), 1);
        prepare(&mut caller, &mut work, &source, &mut donor);
        assert_eq!(Arc::strong_count(&source), 2);
        assert_eq!(Arc::strong_count(&input), 2);
        caller.work_turn(grant(), 1, |cx| {
            let status = work.status(37, cx)?;
            assert_eq!(status == IfcMvdPopulationStatus::Prepared, row["expected"]["container"] == "prepared");
            let mut called = false;
            work.with_view(37, cx, |view, cx| {
                called = true;
                assert!(std::ptr::eq(view.snapshot(cx)?, source.as_ref()));
                assert!(std::ptr::eq(view.request(cx)?, input.as_ref()));
                for expected in row["expected"]["matches"].as_array().unwrap() {
                    let actual = view.lookup(expected["id"].as_str().unwrap().parse().unwrap(), cx)?;
                    let expected = match expected["status"].as_str().unwrap() { "missing" => IfcMvdPopulationLookup::Missing, "ambiguous" => IfcMvdPopulationLookup::Ambiguous, _ => IfcMvdPopulationLookup::Unique(expected["ordinal"].as_u64().unwrap() as usize) };
                    assert_eq!(actual, expected);
                }
                Ok(())
            })?;
            assert_eq!(called, status == IfcMvdPopulationStatus::Prepared);
            Ok(())
        }).unwrap();
        close(&mut caller, &mut work);
        assert_eq!(Arc::strong_count(&source), 1);
        assert_eq!(Arc::strong_count(&input), 1);
        retire(&mut caller, &mut Some(work));
        caller.finish();
    }
}
#[test]
fn native_population_two_clauses_share_one_stable_index_after_parent_move() {
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    let source = fixture(&law["cases"][1]);
    let input = Arc::new(Request { word: "original".into() });
    let mut caller = OriginalWorkFixture::new();
    let mut donor = request_donor(&mut caller, &input);
    let mut original = IfcMvdPopulationWork::default();
    prepare(&mut caller, &mut original, &source, &mut donor);
    let mut first = Clause::default();
    let mut second = Clause::default();
    assert_eq!(caller.work_turn(grant(), 1, |cx| original.with_view(37, cx, |view, cx| first.turn(view, cx))).unwrap(), Some(IfcMvdPopulationLookup::Unique(0)));
    let mut moved = original;
    assert_eq!(caller.work_turn(grant(), 1, |cx| moved.with_view(37, cx, |view, cx| second.turn(view, cx))).unwrap(), Some(IfcMvdPopulationLookup::Unique(0)));
    assert_eq!(first.pin, second.pin);
    assert_eq!(Arc::strong_count(&source), 2);
    assert_eq!(Arc::strong_count(&input), 2);
    let mut other_donor = request_donor(&mut caller, &input);
    let mut other = IfcMvdPopulationWork::default();
    prepare(&mut caller, &mut other, &source, &mut other_donor);
    caller.work_turn(grant(), 1, |cx| {
        let receipt = cx.retained_progress();
        assert!(other.with_view(37, cx, |view, cx| first.turn(view, cx)).is_err());
        assert_eq!(receipt, cx.retained_progress());
        Ok(())
    }).unwrap();
    close(&mut caller, &mut other);
    close(&mut caller, &mut moved);
    assert!(caller.work_turn(grant(), 1, |cx| moved.with_view(37, cx, |_, _| Ok(()))).is_err());
    retire(&mut caller, &mut Some(other));
    retire(&mut caller, &mut Some(moved));
    caller.finish();
}
#[test]
fn native_population_denied_preparation_axes_and_substitutions_are_stationary() {
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    let source = fixture(&law["cases"][1]);
    let other_source = fixture(&law["cases"][1]);
    let input = Arc::new(Request { word: "original".into() });
    let mut caller = OriginalWorkFixture::new();
    let mut donor = request_donor(&mut caller, &input);
    let mut work = IfcMvdPopulationWork::default();
    for axis in 0..4 {
        let mut denied = grant();
        match axis { 0 => denied.maximum_items = 0, 1 => denied.maximum_copy_bytes = 0, 2 => denied.maximum_depth = 0, _ => {} }
        let before = work.metadata();
        caller.work_turn(denied, usize::from(axis != 3) as u64, |cx| {
            let receipt = cx.retained_progress();
            assert!(!work.receive(&source, &mut donor, 37, cx)?);
            assert_eq!(receipt, cx.retained_progress());
            Ok(())
        }).unwrap();
        assert_eq!(before, work.metadata());
        assert!(donor.is_some());
        assert_eq!(Arc::strong_count(&source), 1);
    }
    assert!(caller.work_turn(grant(), 1, |cx| work.receive(&source, &mut donor, 37, cx)).unwrap());
    let mut rejected = request_donor(&mut caller, &Arc::new(Request { word: "replacement".into() }));
    let before = work.metadata();
    caller.work_turn(grant(), 1, |cx| {
        let receipt = cx.retained_progress();
        assert!(work.receive(&source, &mut rejected, 37, cx).is_err());
        assert_eq!(receipt, cx.retained_progress());
        Ok(())
    }).unwrap();
    assert!(rejected.is_some());
    assert_eq!(before, work.metadata());
    retire(&mut caller, &mut rejected);
    for _ in 0..100000 {
        let demand = work.demands(&source, 37).unwrap();
        for axis in 0..6 {
            let mut denied = grant();
            match axis { 0 => denied.maximum_items = 0, 1 => denied.maximum_copy_bytes = demand.copy_bytes - 1, 2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1, 3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1, 2 | 3 => continue, 4 => denied.maximum_depth = 0, _ => {} }
            let before = work.metadata();
            caller.work_turn(denied, u64::from(axis != 5), |cx| {
                let receipt = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, 37, cx));
                assert!(!result?);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(receipt, cx.retained_progress());
                Ok(())
            }).unwrap();
            assert_eq!(before, work.metadata());
        }
        if caller.work_turn(grant(), 1, |cx| {
            let before = cx.retained_progress();
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, 37, cx));
            let after = cx.retained_progress();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
            result
        }).unwrap() { break; }
    }
    for epoch in [37, 38] {
        caller.work_turn(grant(), 1, |cx| {
            let receipt = cx.retained_progress();
            assert!(work.advance(if epoch == 37 { &other_source } else { &source }, epoch, cx).is_err());
            assert_eq!(receipt, cx.retained_progress());
            Ok(())
        }).unwrap();
    }
    let mut foreign = OriginalWorkFixture::new();
    assert!(foreign.work_turn(grant(), 1, |cx| work.with_view(37, cx, |_, _| Ok(()))).is_err());
    assert!(foreign.turn(grant(), |cx| work.close_step(cx)).is_err());
    foreign.finish();
    close(&mut caller, &mut work);
    retire(&mut caller, &mut Some(work));
    caller.finish();
}
#[test]
fn native_population_every_cancelled_prefix_retains_and_paid_closes_original_aliases() {
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    let source = fixture(&law["cases"][1]);
    let input = Arc::new(Request { word: "original".into() });
    let mut probe_caller = OriginalWorkFixture::new();
    let mut probe_donor = request_donor(&mut probe_caller, &input);
    let mut probe = IfcMvdPopulationWork::default();
    let turns = prepare(&mut probe_caller, &mut probe, &source, &mut probe_donor);
    close(&mut probe_caller, &mut probe);
    retire(&mut probe_caller, &mut Some(probe));
    probe_caller.finish();
    for prefix in 0..=turns + 1 {
        let mut caller = OriginalWorkFixture::new();
        let mut donor = request_donor(&mut caller, &input);
        let mut work = IfcMvdPopulationWork::default();
        if prefix > 0 { assert!(caller.work_turn(grant(), 1, |cx| work.receive(&source, &mut donor, 37, cx)).unwrap()); }
        for _ in 1..prefix { caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).unwrap(); }
        caller.cancel().cancel_now();
        let before = work.metadata();
        assert!(caller.work_turn(grant(), 1, |cx| work.receive(&source, &mut donor, 37, cx)).is_err());
        assert!(caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).is_err());
        assert!(caller.work_turn(grant(), 1, |cx| work.with_view(37, cx, |_, _| Ok(()))).is_err());
        assert_eq!(before, work.metadata());
        close(&mut caller, &mut work);
        retire(&mut caller, &mut donor);
        retire(&mut caller, &mut Some(work));
        assert_eq!(Arc::strong_count(&source), 1);
        assert_eq!(Arc::strong_count(&input), 1);
        caller.finish();
    }
}
#[test]
fn native_population_expired_ordinary_deadline_allows_only_original_paid_close() {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NOW: AtomicU64 = AtomicU64::new(1);
    fn clock() -> Option<u64> { Some(NOW.load(Ordering::SeqCst)) }
    NOW.store(1, Ordering::SeqCst);
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    let source = fixture(&law["cases"][1]);
    let input = Arc::new(Request { word: "original".into() });
    let mut caller = OriginalWorkFixture::with_source(semio_framework_job::OperationId(991), semio_framework_job::Generation(17), OriginalWorkFixture::stock(), clock, 10, 1 << 20);
    let mut donor = request_donor(&mut caller, &input);
    let mut work = IfcMvdPopulationWork::default();
    assert!(caller.work_turn(grant(), 1, |cx| work.receive(&source, &mut donor, 37, cx)).unwrap());
    caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).unwrap();
    NOW.store(11, Ordering::SeqCst);
    let before = work.metadata();
    caller.turn(grant(), |cx| {
        let receipt = cx.retained_progress();
        assert!(!work.advance(&source, 37, cx)?);
        assert_eq!(receipt, cx.retained_progress());
        Ok(())
    }).unwrap();
    assert_eq!(before, work.metadata());
    close(&mut caller, &mut work);
    retire(&mut caller, &mut Some(work));
    assert_eq!(Arc::strong_count(&source), 1);
    assert_eq!(Arc::strong_count(&input), 1);
    caller.finish();
}
#[test]
fn native_population_ready_flag_cannot_replace_actual_completed_shape_and_index() {
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    let source = fixture(&law["cases"][1]);
    let input = Arc::new(Request { word: "original".into() });
    let mut caller = OriginalWorkFixture::new();
    let mut donor = request_donor(&mut caller, &input);
    let mut work = IfcMvdPopulationWork::default();
    assert!(caller.work_turn(grant(), 1, |cx| work.receive(&source, &mut donor, 37, cx)).unwrap());
    caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).unwrap();
    let phase = work.state.phase;
    work.state.phase = 5;
    assert!(caller.work_turn(grant(), 1, |cx| work.with_view(37, cx, |_, _| -> Result<(), OriginalTurnFailure> { panic!("unfinished shape exposed a population view") })).is_err());
    work.state.phase = phase;
    for _ in 0..100000 {
        if work.state.phase == 3 { break; }
        assert!(!caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).unwrap());
    }
    assert!(work.shape.complete() && work.shape.valid());
    work.state.phase = 5;
    assert!(caller.work_turn(grant(), 1, |cx| work.with_view(37, cx, |_, _| -> Result<(), OriginalTurnFailure> { panic!("absent index exposed a population view") })).is_err());
    work.state.phase = 3;
    caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).unwrap();
    work.state.phase = 5;
    assert!(caller.work_turn(grant(), 1, |cx| work.with_view(37, cx, |_, _| -> Result<(), OriginalTurnFailure> { panic!("unfinished index exposed a population view") })).is_err());
    work.state.phase = 4;
    for _ in 0..100000 { if caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).unwrap() { break; } }
    assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_view(37, cx, |_, _| Ok(()))).unwrap(), Some(()));
    close(&mut caller, &mut work);
    retire(&mut caller, &mut Some(work));
    caller.finish();
}
#[test]
fn native_population_unavailable_request_borrow_preserves_actual_authority_without_a_graph_view() {
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let source = fixture(row);
        let input = Arc::new(Request { word: "original".into() });
        let mut caller = OriginalWorkFixture::new();
        let mut donor = request_donor(&mut caller, &input);
        let mut work = IfcMvdPopulationWork::default();
        prepare(&mut caller, &mut work, &source, &mut donor);
        let mut pin = None;
        assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_request(37, cx, |view, cx| {
            assert!(std::ptr::eq(view.request(cx)?, input.as_ref()));
            pin = Some(view.pin(cx)?);
            Ok(())
        })).unwrap(), Some(()));
        assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_request(37, cx, |view, cx| Ok(pin.unwrap().matches(&view, cx)))).unwrap(), Some(true));
        let mut foreign = OriginalWorkFixture::new();
        assert!(foreign.work_turn(grant(), 1, |cx| work.with_request(37, cx, |_, _| Ok(()))).is_err());
        foreign.finish();
        assert!(caller.work_turn(grant(), 1, |cx| work.with_request(38, cx, |_, _| Ok(()))).is_err());
        caller.cancel().cancel_now();
        assert!(caller.work_turn(grant(), 1, |cx| work.with_request(37, cx, |_, _| Ok(()))).is_err());
        close(&mut caller, &mut work);
        assert!(caller.turn(grant(), |cx| work.with_request(37, cx, |_, _| Ok(()))).is_err());
        retire(&mut caller, &mut Some(work));
        assert_eq!(Arc::strong_count(&source), 1);
        assert_eq!(Arc::strong_count(&input), 1);
        caller.finish();
    }
}
#[test]
fn native_population_original_snapshot_donor_moves_once_without_another_alias() {
    let law: serde_json::Value = serde_json::from_str(include_str!("population-cases.json")).unwrap();
    let source = fixture(&law["cases"][1]);
    let input = Arc::new(Request { word: "original".into() });
    let mut caller = OriginalWorkFixture::new();
    let mut request = request_donor(&mut caller, &input);
    let mut snapshot = None;
    caller.work_turn(grant(), 1, |cx| {
        snapshot = Some(Arc::clone(&source));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: 2 * size_of::<Arc<IfcSnapshot>>(), ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(())
    }).unwrap();
    let mut work = IfcMvdPopulationWork::default();
    for axis in 0..4 {
        let mut denied = grant();
        match axis { 0 => denied.maximum_items = 0, 1 => denied.maximum_copy_bytes = IfcMvdPopulationWork::<Request>::receive_demands().copy_bytes - 1, 2 => denied.maximum_depth = 0, _ => {} }
        let before = work.metadata();
        caller.work_turn(denied, u64::from(axis != 3), |cx| {
            let receipt = cx.retained_progress();
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.receive_donated(&mut snapshot, &mut request, 37, cx));
            assert!(!result?);
            assert_eq!(receipt, cx.retained_progress());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            Ok(())
        }).unwrap();
        assert_eq!(before, work.metadata());
        assert!(snapshot.is_some() && request.is_some());
        assert_eq!(Arc::strong_count(&source), 2);
    }
    assert!(caller.work_turn(grant(), 1, |cx| work.receive_donated(&mut snapshot, &mut request, 37, cx)).unwrap());
    assert!(snapshot.is_none() && request.is_none());
    assert_eq!(Arc::strong_count(&source), 2);
    for _ in 0..100000 {
        if caller.work_turn(grant(), 1, |cx| work.advance(&source, 37, cx)).unwrap() { break; }
        assert_eq!(Arc::strong_count(&source), 2);
    }
    assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_view(37, cx, |view, cx| Ok(std::ptr::eq(view.snapshot(cx)?, source.as_ref())))).unwrap(), Some(true));
    close(&mut caller, &mut work);
    retire(&mut caller, &mut Some(work));
    assert_eq!(Arc::strong_count(&source), 1);
    assert_eq!(Arc::strong_count(&input), 1);
    caller.finish();
}
