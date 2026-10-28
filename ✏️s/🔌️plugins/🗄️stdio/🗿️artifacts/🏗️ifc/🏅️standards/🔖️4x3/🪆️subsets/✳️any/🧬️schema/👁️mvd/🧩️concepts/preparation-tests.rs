//! 🎟️ Neutral request admission laws use the canonical original preparation and private index views.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
#[derive(semio_framework_value::RetireOwned)]
struct Row { id: String, kind: String }
#[derive(semio_framework_value::RetireOwned)]
struct Request { targets: Vec<Row> }
impl IfcSourceIds for Request { fn source_len(&self) -> usize { self.targets.len() } fn source_id(&self, row: usize) -> Result<u64, ValueError> { super::super::native::positive_id(&self.targets[row].id) } }
impl IfcMvdRequest for Request { fn valid_row(&self, ordinal: usize) -> bool { matches!(self.targets[ordinal].kind.as_str(), "horizontal" | "vertical" | "cant") } }
fn grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 512 } }
fn source() -> Arc<IfcSnapshot> { Arc::new(IfcSnapshot { schema: "stdio.ifc".into(), header: IfcHeader { file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])], ..Default::default() }, ..Default::default() }) }
fn request(row: &serde_json::Value) -> Arc<Request> { Arc::new(Request { targets: row["targets"].as_array().unwrap().iter().map(|row| Row { id: row["id"].as_str().unwrap().into(), kind: row["kind"].as_str().unwrap().into() }).collect() }) }
fn retire<T: RetireOwned>(caller: &mut OriginalWorkFixture, value: &mut Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..100000 {
        if value.is_none() && close.is_empty() { return; }
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
            let demand = close.next_owner_demands::<T>(value.is_some(), &None, 32768)?;
            semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(value, grant)?.ok_or_else(|| invalid("mvd.preparation.test-close")) } else { close.step_granted(grant) })?;
            Ok(())
        })).unwrap();
    }
    panic!("original preparation did not retire");
}
#[test]
fn native_mvd_canonical_preparation_matches_all_request_cases_and_actual_index_roles() {
    let law: serde_json::Value = serde_json::from_str(include_str!("preparation-cases.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let source = source(); let input = request(row);
        let mut caller = OriginalWorkFixture::new(); let mut work = IfcMvdPreparation::default();
        let mut done = false; let mut refused = false;
        for _ in 0..100000 { match caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 71, cx)) { Ok(true) => { done = true; break; }, Err(_) => { refused = true; break; }, _ => {} } }
        assert_eq!(done, row["admitted"].as_bool().unwrap()); assert_eq!(refused, !done);
        if done {
            assert_eq!(Arc::strong_count(&source), 2); assert_eq!(Arc::strong_count(&input), 2);
            assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_prepared(71, cx, |view, requests, cx| {
                assert!(requests.matches_prepared(&view, cx)); assert!(std::ptr::eq(view.request(cx)?, input.as_ref()));
                for (ordinal, target) in input.targets.iter().enumerate() { let id = target.id.parse().unwrap(); assert_eq!(requests.lookup(id, cx)?, IfcMvdPopulationLookup::Unique(ordinal)); assert_eq!(view.lookup(id, cx)?, IfcMvdPopulationLookup::Missing); }
                Ok(())
            })).unwrap(), Some(()));
            assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_unavailable(71, cx, |_, _, _| -> Result<(), OriginalTurnFailure> { panic!("prepared input downgraded") })).unwrap(), None);
        }
        retire(&mut caller, &mut Some(work)); assert_eq!(Arc::strong_count(&source), 1); assert_eq!(Arc::strong_count(&input), 1); caller.finish();
    }
}
#[test]
fn native_mvd_canonical_preparation_denial_prefix_cancellation_and_foreign_roots_are_closed() {
    let row = serde_json::json!({"targets":[{"id":"9","kind":"horizontal"},{"id":"2","kind":"cant"}]});
    let source = source(); let input = request(&row);
    let mut caller = OriginalWorkFixture::new(); let mut work = IfcMvdPreparation::default(); let mut turns = 0;
    loop {
        let demand = work.demands(&source, &input).unwrap();
        for axis in 0..6 {
            let mut denied = grant();
            match axis { 0 => denied.maximum_items = 0, 1 => denied.maximum_copy_bytes = demand.copy_bytes - 1, 2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1, 3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1, 2 | 3 => continue, 4 => denied.maximum_depth = 0, _ => {} }
            let before = work.metadata();
            caller.work_turn(denied, u64::from(axis != 5), |cx| { let receipt = cx.retained_progress(); let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 71, cx)); assert!(!result?); assert_eq!(receipt, cx.retained_progress()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); Ok(()) }).unwrap();
            assert_eq!(before, work.metadata());
        }
        turns += 1;
        if caller.work_turn(grant(), 1, |cx| { let before = cx.retained_progress(); let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 71, cx)); let after = cx.retained_progress(); assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes)); result }).unwrap() { break; }
        assert!(turns < 100000);
    }
    let other_source = Arc::new(source.as_ref().clone()); let other_input = request(&row);
    for mode in 0..3 { caller.work_turn(grant(), 1, |cx| { let before = cx.retained_progress(); assert!(work.advance(if mode == 0 { &other_source } else { &source }, if mode == 1 { &other_input } else { &input }, if mode == 2 { 72 } else { 71 }, cx).is_err()); assert_eq!(before, cx.retained_progress()); Ok(()) }).unwrap(); }
    let mut foreign = OriginalWorkFixture::new(); assert!(foreign.work_turn(grant(), 1, |cx| work.with_prepared(71, cx, |_, _, _| Ok(()))).is_err()); foreign.finish();
    retire(&mut caller, &mut Some(work)); caller.finish();
    for prefix in 0..=turns {
        let mut caller = OriginalWorkFixture::new(); let mut work = IfcMvdPreparation::default();
        for _ in 0..prefix { caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 71, cx)).unwrap(); }
        if prefix > 0 {
            let before = work.metadata();
            for mode in 0..3 { caller.work_turn(grant(), 1, |cx| { let receipt = cx.retained_progress(); let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(if mode == 0 { &other_source } else { &source }, if mode == 1 { &other_input } else { &input }, if mode == 2 { 72 } else { 71 }, cx)); assert!(result.is_err()); assert_eq!(receipt, cx.retained_progress()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); Ok(()) }).unwrap(); }
            let mut foreign = OriginalWorkFixture::new();
            foreign.work_turn(grant(), 1, |cx| { let receipt = cx.retained_progress(); assert!(work.advance(&source, &input, 71, cx).is_err()); assert_eq!(receipt, cx.retained_progress()); Ok(()) }).unwrap(); foreign.finish();
            assert_eq!(before, work.metadata());
        }
        caller.cancel().cancel_now(); let before = work.metadata();
        assert!(caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 71, cx)).is_err()); assert!(caller.work_turn(grant(), 1, |cx| work.with_prepared(71, cx, |_, _, _| Ok(()))).is_err()); assert_eq!(before, work.metadata());
        retire(&mut caller, &mut Some(work)); assert_eq!(Arc::strong_count(&source), 1); assert_eq!(Arc::strong_count(&input), 1); caller.finish();
    }
}
#[test]
fn native_mvd_unavailable_request_index_never_exposes_a_graph_or_unrelated_prepared_origin() {
    let row = serde_json::json!({"targets":[{"id":"9","kind":"horizontal"}]});
    let mut source = source(); Arc::get_mut(&mut source).unwrap().entities.push(IfcEntity { id: 0, name: "IFCUNKNOWN".into(), args: vec![], complex: vec![] });
    let input = request(&row); let mut caller = OriginalWorkFixture::new(); let mut work = IfcMvdPreparation::default();
    for _ in 0..100000 { if caller.work_turn(grant(), 1, |cx| work.advance(&source, &input, 71, cx)).unwrap() { break; } }
    assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_prepared(71, cx, |_, _, _| -> Result<(), OriginalTurnFailure> { panic!("unavailable graph was lent") })).unwrap(), None);
    assert_eq!(caller.work_turn(grant(), 1, |cx| work.with_unavailable(71, cx, |view, requests, cx| { assert!(requests.matches_unavailable(&view, cx)); assert_eq!(requests.lookup(9, cx)?, IfcMvdPopulationLookup::Unique(0)); Ok(()) })).unwrap(), Some(()));
    retire(&mut caller, &mut Some(work)); caller.finish();
}
#[test]
fn native_mvd_rejected_request_turn_posts_only_its_actual_metadata_prefix() {
    for row in [serde_json::json!({"targets":[]}), serde_json::json!({"targets":[{"id":"9","kind":"other"}]})] {
        let source = source(); let input = request(&row);
        let mut caller = OriginalWorkFixture::new(); let mut work = IfcMvdPreparation::default(); let mut refused = false;
        for _ in 0..100000 {
            let result = caller.work_turn(grant(), 1, |cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| work.advance(&source, &input, 71, cx));
                let after = cx.retained_progress();
                assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                if result.is_err() { assert_eq!(after.copied_items - before.copied_items, 1); assert_eq!(after.copied_bytes - before.copied_bytes, IfcMvdPreparation::<Request>::base().copy_bytes); }
                result
            });
            if result.is_err() { refused = true; break; }
        }
        assert!(refused);
        let metadata = work.metadata();
        caller.work_turn(grant(), 1, |cx| { let before = cx.retained_progress(); assert!(work.advance(&source, &input, 71, cx).is_err()); assert_eq!(before, cx.retained_progress()); Ok(()) }).unwrap();
        assert_eq!(metadata, work.metadata());
        retire(&mut caller, &mut Some(work)); assert_eq!(Arc::strong_count(&source), 1); assert_eq!(Arc::strong_count(&input), 1); caller.finish();
    }
}
