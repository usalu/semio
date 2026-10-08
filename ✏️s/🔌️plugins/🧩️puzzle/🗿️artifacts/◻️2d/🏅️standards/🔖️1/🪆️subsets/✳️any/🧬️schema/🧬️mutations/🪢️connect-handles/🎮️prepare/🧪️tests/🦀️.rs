//! 🧪️ Native connection plans preserve original paged owners and exact borrowed closure.

use super::*;
use semio_framework_value::paged::PagedUtf8;
use crate::test_source_custody;

fn close(cursor: &mut Puzzle2dConnectHandlesPreparationCursor) {
    cursor.begin_close();
    for _ in 0..128 {
        if cursor.terminal_is_empty() { return; }
        let copy=cursor.next_close_copy_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy).unwrap();let release=cursor.next_close_release_byte_demand().unwrap();let depth=cursor.next_close_depth_demand().unwrap();assert!(copy+capacity+release<=4096);
        let permit=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(permit));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!heap.overflowed);
        assert!(step.unwrap().progress().fits(permit));
    }
    panic!("borrowed connection retained native aliases after closure");
}

#[test]
fn history_edit_puzzle2d_borrowed_connect_preparation_preserves_finite_priority_and_exact_proximity() {
    assert!(std::mem::size_of::<Puzzle2dConnectHandlesPreparationCursor>() <= 4096);
    assert!(std::mem::size_of::<Puzzle2dConnectHandlesPlan>() <= 96);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", controls["largePrefix"].as_str().unwrap().repeat(controls["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let grant = RetainedCloneGrant::one_payload_turn(96,64);
    for case in corpus["cases"].as_array().unwrap() {
        let mut snapshot = Puzzle2dSnapshot::default();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|wire| { let mut node: crate::Puzzle2dNode = serde_json::from_value(wire.clone()).unwrap(); for (index, handle) in wire["handles"].as_array().unwrap().iter().enumerate() { node.handles.get_mut(index).unwrap().id = text(&handle["id"]); } node }).collect();
        snapshot.edges = case["edges"].as_array().unwrap().iter().map(|id| crate::Puzzle2dEdge { id: text(id), ..Default::default() }).collect();
        let mut wire = case["mutation"].clone();
        for field in ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y", "tolerance"] { if wire[field].is_string() { wire[field] = serde_json::json!(0); } }
        let mut payload: ConnectHandles = serde_json::from_value(wire).unwrap();
        payload.id = text(&case["mutation"]["id"]); payload.source = text(&case["mutation"]["source"]); payload.target = text(&case["mutation"]["target"]);
        for field in ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y", "tolerance"] {
            if let Some(word) = case["mutation"][field].as_str() { let value = if word == "nan" { f64::NAN } else { f64::INFINITY }; match field { "gap" => payload.gap = value, "shift" => payload.shift = value, "rise" => payload.rise = value, "rotation" => payload.rotation = value, "turn" => payload.turn = value, "tilt" => payload.tilt = value, "x" => payload.x = value, "y" => payload.y = value, _ => payload.tolerance = Some(value) } }
        }
        let original = serde_json::to_value(&snapshot).unwrap();
        let original_payload = serde_json::to_value(&payload).unwrap();
        let bits = [payload.gap.to_bits(), payload.shift.to_bits(), payload.rise.to_bits(), payload.rotation.to_bits(), payload.turn.to_bits(), payload.tilt.to_bits(), payload.x.to_bits(), payload.y.to_bits(), payload.tolerance.map(f64::to_bits).unwrap_or(0)];
        let mut source = test_source_custody::admit();
        let mut mutation = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dConnectHandlesPreparationCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (zero, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default()));
        assert_eq!(zero.unwrap(), Puzzle2dConnectHandlesPreparationStep::Pending(RetainedCloneProgress::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut output = None;
        for turn in 0..100_000 {
            let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert!(!heap.overflowed);
            let step = step.unwrap();
            let progress = match step { Puzzle2dConnectHandlesPreparationStep::Pending(progress) | Puzzle2dConnectHandlesPreparationStep::Complete { progress, .. } => progress };
            assert!(progress.fits(permit));
            if let Puzzle2dConnectHandlesPreparationStep::Complete { plan, .. } = step { output = Some(plan); break; }
        }
        let plan = output.expect("borrowed connection preparation did not finish");
        let status = match plan.disposition { Puzzle2dConnectHandlesDisposition::Changed => "changed", Puzzle2dConnectHandlesDisposition::DuplicateId => "duplicate-id", Puzzle2dConnectHandlesDisposition::NegativeTolerance => "negative-tolerance", Puzzle2dConnectHandlesDisposition::Nonfinite(index) => ["nonfinite-gap", "nonfinite-shift", "nonfinite-rise", "nonfinite-rotation", "nonfinite-turn", "nonfinite-tilt", "nonfinite-x", "nonfinite-y", "nonfinite-tolerance"][index as usize] };
        let warning = match plan.warning { Puzzle2dConnectHandlesWarning::None => "none", Puzzle2dConnectHandlesWarning::MissingHandle => "missing-handle", Puzzle2dConnectHandlesWarning::TooFar => "too-far" };
        assert_eq!(status, case["status"].as_str().unwrap()); assert_eq!(warning, case["warning"].as_str().unwrap());
        assert_eq!(serde_json::to_value(plan.position).unwrap(), case["position"]); assert_eq!(plan.distance, case["distance"].as_f64());
        assert_eq!(cursor.take(), Some(plan)); assert_eq!(cursor.take(), None); close(&mut cursor);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original); assert_eq!(serde_json::to_value(&payload).unwrap(), original_payload);
        assert_eq!([payload.gap.to_bits(), payload.shift.to_bits(), payload.rise.to_bits(), payload.rotation.to_bits(), payload.turn.to_bits(), payload.tilt.to_bits(), payload.x.to_bits(), payload.y.to_bits(), payload.tolerance.map(f64::to_bits).unwrap_or(0)], bits);
        for pause in [0, 1, 9, 12, 20, 40] {
            let mut cancelled = Puzzle2dConnectHandlesPreparationCursor::default();
            for turn in 0..pause { let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};if matches!(cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit).unwrap(), Puzzle2dConnectHandlesPreparationStep::Complete { .. }) { break; } }
            close(&mut cancelled); assert_eq!(cancelled.take(), None);
        }
        let mut fenced = Puzzle2dConnectHandlesPreparationCursor::default(); fenced.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant).unwrap();
        let swapped = payload.clone(); assert!(fenced.advance(source.borrow(&snapshot), mutation.borrow(&swapped), grant).is_err()); close(&mut fenced);
        let mut fenced = Puzzle2dConnectHandlesPreparationCursor::default(); fenced.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant).unwrap();
        let swapped = snapshot.clone(); assert!(fenced.advance(source.borrow(&swapped), mutation.borrow(&payload), grant).is_err()); close(&mut fenced);
        test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
        eprintln!("[DEBUG] Puzzle connect {} retained finite priority, first handle, proximity and original paged owners with zero heap birth/advance/alias closure", case["id"].as_str().unwrap());
    }
}
