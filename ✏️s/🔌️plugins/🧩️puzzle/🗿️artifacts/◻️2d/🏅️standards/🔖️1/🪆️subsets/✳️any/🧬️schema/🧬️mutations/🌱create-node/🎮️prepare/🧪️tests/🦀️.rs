//! 🧪️ Borrowed create plans match literal invariant priority without owning mutation payloads.

use super::*;
use semio_framework_value::paged::PagedUtf8;
use crate::test_source_custody;

fn close(cursor: &mut Puzzle2dCreateNodePreparationCursor) {
    cursor.begin_close();
    for _ in 0..32 {
        if cursor.terminal_is_empty() { return; }
        let copy=cursor.next_close_copy_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy).unwrap();let release=cursor.next_close_release_byte_demand().unwrap();let depth=cursor.next_close_depth_demand().unwrap();assert!(copy+capacity+release<=4096);
        let permit=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(permit));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!heap.overflowed);
        assert!(step.unwrap().progress().fits(permit));
    }
    panic!("borrowed create preparation retained a native alias after closure");
}

#[test]
fn history_edit_puzzle2d_borrowed_create_preparation_preserves_invariant_priority_and_indexed_placement() {
    assert!(std::mem::size_of::<Puzzle2dCreateNodePreparationCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", controls["largePrefix"].as_str().unwrap().repeat(controls["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let grant = RetainedCloneGrant::one_payload_turn(32,64);
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
        let mut source = test_source_custody::admit();
        let mut mutation = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dCreateNodePreparationCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (zero, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default()));
        assert_eq!(zero.unwrap(), Puzzle2dCreateNodePreparationStep::Pending(RetainedCloneProgress::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut plan = None;
        for turn in 0..100_000 {
            let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflowed);
            let step = step.unwrap();
            let progress = match step { Puzzle2dCreateNodePreparationStep::Pending(progress) | Puzzle2dCreateNodePreparationStep::Complete { progress, .. } => progress };
            assert!(progress.fits(permit));
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
        if case["id"] == "large-prefix" { let mut cancelled = Puzzle2dCreateNodePreparationCursor::default(); for turn in 0..30 { let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};assert!(matches!(cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit).unwrap(), Puzzle2dCreateNodePreparationStep::Pending(_))); } close(&mut cancelled); }
        test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
        eprintln!("[DEBUG] Puzzle2d create {} retained invariant priority, literal placement and original native owners with zero heap birth/copy/alias closure", case["id"].as_str().unwrap());
    }
}
