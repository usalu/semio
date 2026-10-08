//! 🧪️ Borrowed diagnostic composition preserves native intent text and physical owner grants.
use super::*;
use super::super::{ChangeNodeIcon, EditTargetRegionLabel};
use protocol::os_store::ArtifactMessageComposeCursor;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_trace::observe_heap_allocations_on_this_thread;

fn close(cursor: &mut ArtifactMessageComposeCursor) {
    cursor.begin_close();
    for _ in 0..64 {
        if cursor.terminal_is_empty() { return; }
        let demand = cursor.next_release_byte_demand().unwrap();
        assert!(demand <= 4096);
        if demand > 0 {
            let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.close_step(RetainedCloneGrant::one_release_turn(demand - 1, 1)).unwrap());
            assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(cursor.next_release_byte_demand().unwrap(), demand);
        }
        let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.close_step(RetainedCloneGrant::one_release_turn(demand, 1)).unwrap());
        assert_eq!(heap.requested_bytes, 0);
        assert_eq!(heap.released_bytes, demand);
        let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
        assert_eq!(progress.released_bytes, demand);
        assert_eq!((progress.copied_bytes, progress.retained_capacity_bytes), (0, 0));
        assert!(!heap.overflowed);
    }
    panic!("diagnostic owner did not close");
}

fn advance<T: Puzzle2dTextIntent>(cursor: &mut ArtifactMessageComposeCursor, source: &Puzzle2dTextDiagnosticSource<'_, T>) -> RetainedCloneStep {
    let capacity = cursor.next_capacity_byte_demand(source).unwrap();
    let copy = if capacity == 0 { cursor.next_copy_byte_demand(source).unwrap() } else { 0 };
    assert!(capacity + copy <= 4096);
    if capacity > 0 {
        let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.advance(source, RetainedCloneGrant::one_capacity_turn(capacity - 1, 1)).unwrap());
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(cursor.next_capacity_byte_demand(source).unwrap(), capacity);
    }
    if copy > 0 {
        let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.advance(source, RetainedCloneGrant::one_payload_turn(copy - 1, 1)).unwrap());
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(cursor.next_copy_byte_demand(source).unwrap(), copy);
    }
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: 0, maximum_depth: 1 };
    let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.advance(source, grant).unwrap());
    let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
    assert!(progress.fits(grant));
    assert_eq!(heap.requested_bytes, progress.retained_capacity_bytes);
    assert_eq!(heap.released_bytes, 0);
    assert!(!heap.overflowed);
    step
}

fn law<T: Puzzle2dTextIntent>(payload: T, plan: Puzzle2dTextPlan, operation: Option<u32>, expected: &serde_json::Value) {
    let (source, heap) = observe_heap_allocations_on_this_thread(|| Puzzle2dTextDiagnosticSource::new(&payload, plan, operation));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let Some(source) = source else { assert!(expected.is_null()); return; };
    assert_eq!(source.operation_index(), operation);
    assert_eq!(source.target_count(), 1);
    assert!(source.target(1).is_none());
    assert!(source.fragment(source.fragment_count()).is_none());
    assert!(std::ptr::eq(source.target(0).unwrap(), payload.identifier() as &dyn Utf8Text));
    let (mut cursor, heap) = observe_heap_allocations_on_this_thread(|| ArtifactMessageComposeCursor::new(4096));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.advance(&source, RetainedCloneGrant::default()).unwrap());
    assert_eq!(step, RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let mut completed = false;
    for _ in 0..100000 { if matches!(advance(&mut cursor, &source), RetainedCloneStep::Complete(_)) { completed = true; break; } }
    assert!(completed);
    assert!(!cursor.truncated());
    let message = cursor.take().unwrap();
    assert_eq!(message.level, if expected["level"] == "warning" { Severity::Warning } else { Severity::Error });
    assert_eq!(message.code.0, expected["code"].as_str().unwrap());
    assert_eq!(message.message, expected["body"].as_str().unwrap());
    assert_eq!(message.op_index, operation);
    assert_eq!(message.target, vec![expected["targets"][0].as_str().unwrap()]);
    close(&mut cursor);
    let changed_source = Puzzle2dTextDiagnosticSource::new(&payload, plan, operation).unwrap();
    let mut fenced = ArtifactMessageComposeCursor::new(4096);
    advance(&mut fenced, &source);
    let (result, heap) = observe_heap_allocations_on_this_thread(|| fenced.advance(&changed_source, RetainedCloneGrant::one_payload_turn(4096, 1)));
    assert_eq!(result.unwrap_err(), protocol::os_store::ArtifactMessageComposeRefusal::SourceChanged);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    close(&mut fenced);
    for pause in [0, 1, 3, 7, 17, 31] {
        let mut cursor = ArtifactMessageComposeCursor::new(4096);
        for _ in 0..pause { if matches!(advance(&mut cursor, &source), RetainedCloneStep::Complete(_)) { break; } }
        cursor.cancel();
        assert!(cursor.take().is_none());
        close(&mut cursor);
    }
}

#[test]
fn history_edit_puzzle2d_optional_text_diagnostics_borrow_raw_intent_and_compose_under_exact_heap_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let plan = Puzzle2dTextPlan { disposition: if row["expected"].is_null() { Puzzle2dTextDisposition::Changed } else if row["expected"]["level"] == "warning" { Puzzle2dTextDisposition::NoOp } else { Puzzle2dTextDisposition::TargetMissing }, index: row["target"].as_bool().unwrap().then_some(0) };
        let operation = row["operationIndex"].as_u64().map(|value| value as u32);
        let id = row["id"].as_str().unwrap().into();
        let next = row["next"].as_str().map(Into::into);
        if row["role"] == "nodeIcon" { law(ChangeNodeIcon { id, new_icon_kind: next }, plan, operation, &row["expected"]); }
        else { law(EditTargetRegionLabel { id, new_label: next }, plan, operation, &row["expected"]); }
    }
    let payload = ChangeNodeIcon { id: "😀\0甲".repeat(20000).into(), new_icon_kind: None };
    let source = Puzzle2dTextDiagnosticSource::new(&payload, Puzzle2dTextPlan { disposition: Puzzle2dTextDisposition::TargetMissing, index: None }, Some(u32::MAX)).unwrap();
    let mut cursor = ArtifactMessageComposeCursor::new(4096);
    let mut completed = false;
    for _ in 0..100000 { if matches!(advance(&mut cursor, &source), RetainedCloneStep::Complete(_)) { completed = true; break; } }
    assert!(completed && cursor.truncated());
    let output = cursor.take().unwrap();
    assert!(output.target.is_empty());
    assert!(output.message.starts_with("node \"😀\0甲"));
    let full = format!("node \"{}\" not found", payload.id.to_string_owner());
    assert!(full.starts_with(&output.message));
    assert_eq!(output.op_index, Some(u32::MAX));
    close(&mut cursor);
    eprintln!("[DEBUG] six original optional-text diagnostics preserve verbatim quote/NUL/Unicode, severity and operation indices; 160000byte target skips without copying, UTF8 prefix fits4096, six partial cancellations match actual allocation/release");
}
