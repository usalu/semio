//! 🧪️ Borrowed create plans match literal invariant priority without owning mutation payloads.

use super::*;
use semio_framework_value::{paged::PagedUtf8, retained_clone::RetainedCloneBorrowAuthority};

fn close(cursor: &mut Puzzle2dCreateNodePreparationCursor) {
    cursor.begin_close();
    for _ in 0..32 {
        if cursor.terminal_is_empty() { return; }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(1, 32));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!heap.overflowed);
        assert!(!matches!(step.unwrap(), SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > 32));
    }
    panic!("borrowed create preparation retained a native alias after closure");
}

#[test]
fn history_edit_puzzle2d_borrowed_create_preparation_preserves_invariant_priority_and_indexed_placement() {
    assert!(std::mem::size_of::<Puzzle2dCreateNodePreparationCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", controls["largePrefix"].as_str().unwrap().repeat(controls["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let grant = BoundedOrdGrant { maximum_items: 1, maximum_bytes: 32 };
    for case in corpus["cases"].as_array().unwrap() {
        let mut snapshot = Puzzle2dSnapshot::default();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|id| crate::Puzzle2dNode { id: text(id), ..Default::default() }).collect();
        let mut wire = case["mutation"]["node"].clone();
        for name in ["x", "y"] { if wire[name].is_string() { wire[name] = serde_json::json!(0); } }
        if let Some(handles) = wire.get_mut("handles").and_then(serde_json::Value::as_array_mut) { for handle in handles { if handle["angle"].is_string() { handle["angle"] = serde_json::json!(0); } } }
        let mut node: crate::Puzzle2dNode = serde_json::from_value(wire).unwrap();
        node.id = text(&case["mutation"]["node"]["id"]);
        for name in ["x", "y"] { if let Some(scalar) = case["mutation"]["node"][name].as_str() { let value = if scalar == "nan" { f64::NAN } else { f64::INFINITY }; if name == "x" { node.x = value; } else { node.y = value; } } }
        if let Some(handles) = case["mutation"]["node"].get("handles").and_then(serde_json::Value::as_array) { for (index, handle) in handles.iter().enumerate() { if handle["angle"] == "nan" { node.handles.get_mut(index).unwrap().angle = f64::NAN; } } }
        let payload = CreateNode { node, index: case["mutation"]["index"].as_u64().map(|value| value as usize) };
        let original = serde_json::to_value(&snapshot).unwrap();
        let original_payload = serde_json::to_value(&payload).unwrap();
        let bits = [payload.node.x.to_bits(), payload.node.y.to_bits()];
        let source = RetainedCloneBorrowAuthority::new("borrowed create native snapshot");
        let mutation = RetainedCloneBorrowAuthority::new("borrowed create original mutation");
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dCreateNodePreparationCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (zero, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), BoundedOrdGrant { maximum_items: 0, maximum_bytes: 0 }));
        assert_eq!(zero.unwrap(), Puzzle2dCreateNodePreparationStep::Pending(BoundedOrdProgress::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut plan = None;
        for _ in 0..100_000 {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflowed);
            let step = step.unwrap();
            let progress = match step { Puzzle2dCreateNodePreparationStep::Pending(progress) | Puzzle2dCreateNodePreparationStep::Complete { progress, .. } => progress };
            assert!(progress.compared_items <= 1 && progress.compared_bytes <= 32);
            if let Puzzle2dCreateNodePreparationStep::Complete { plan: value, .. } = step { plan = Some(value); break; }
        }
        let plan = plan.expect("borrowed create preparation did not finish");
        let status = match plan.disposition { Puzzle2dCreateNodeDisposition::Changed => "changed", Puzzle2dCreateNodeDisposition::DuplicateId => "duplicate-id", Puzzle2dCreateNodeDisposition::NonfiniteX => "nonfinite-x", Puzzle2dCreateNodeDisposition::NonfiniteY => "nonfinite-y", Puzzle2dCreateNodeDisposition::InvalidShape => "invalid-shape", Puzzle2dCreateNodeDisposition::InvalidRadius => "invalid-radius", Puzzle2dCreateNodeDisposition::InvalidWidth => "invalid-width", Puzzle2dCreateNodeDisposition::InvalidHeight => "invalid-height", Puzzle2dCreateNodeDisposition::InvalidScale => "invalid-scale", Puzzle2dCreateNodeDisposition::InvalidHandleAngle => "invalid-handle-angle", Puzzle2dCreateNodeDisposition::InvalidHandleRadius => "invalid-handle-radius", Puzzle2dCreateNodeDisposition::InvalidHandleScale => "invalid-handle-scale" };
        assert_eq!(status, case["status"].as_str().unwrap());
        assert_eq!(serde_json::to_value(plan.position).unwrap(), case["position"]);
        assert_eq!(cursor.take(), Some(plan));
        assert_eq!(cursor.take(), None);
        close(&mut cursor);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);
        assert_eq!(serde_json::to_value(&payload).unwrap(), original_payload);
        assert_eq!([payload.node.x.to_bits(), payload.node.y.to_bits()], bits);
        let mut cancelled = Puzzle2dCreateNodePreparationCursor::default();
        cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant).unwrap();
        let swapped = payload.clone();
        assert!(cancelled.advance(source.borrow(&snapshot), mutation.borrow(&swapped), grant).is_err());
        close(&mut cancelled);
        assert_eq!(cancelled.take(), None);
        if case["id"] == "large-prefix" { let mut cancelled = Puzzle2dCreateNodePreparationCursor::default(); for _ in 0..30 { assert!(matches!(cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant).unwrap(), Puzzle2dCreateNodePreparationStep::Pending(_))); } close(&mut cancelled); }
        eprintln!("[DEBUG] Puzzle2d create {} retained invariant priority, literal placement and original native owners with zero heap birth/copy/alias closure", case["id"].as_str().unwrap());
    }
}
