//! 🧪️ Native connection candidates preserve all untouched records and exact granted cancellation.

use super::*;
use semio_framework_value::paged::PagedUtf8;
use crate::test_source_custody;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 128), 1 => RetainedCloneGrant::one_payload_turn(4096, 128), _ => RetainedCloneGrant::one_release_turn(4096, 128) } }

fn observed(permit: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!heap.overflowed && step.progress().fits(permit));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(heap.requested_bytes <= step.progress().retained_capacity_bytes, "connect candidate unadmitted allocation: {heap:?}, {step:?}");
    assert!(heap.released_bytes <= step.progress().released_bytes, "connect candidate unadmitted release: {heap:?}, {step:?}");
    step
}

fn close(cursor: &mut Puzzle2dConnectHandlesCandidateCursor) {
    cursor.begin_close();
    test_source_custody::close_cursor(cursor,1_000_000,|cursor|{
        let copy=cursor.next_close_copy_byte_demand().unwrap();
        (copy,cursor.next_close_capacity_byte_demand(copy).unwrap(),cursor.next_close_release_byte_demand().unwrap(),cursor.next_close_depth_demand().unwrap())
    },|cursor,grant|cursor.close_step(grant),|cursor|cursor.terminal_is_empty());
}

fn retire(snapshot: Puzzle2dSnapshot) {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ControlledRetirement::new(snapshot));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let mut owner = match result { Ok(owner) => owner, Err((error, _)) => panic!("{error:?}") };
    for turn in 0..1_000_000 {
        if owner.terminal_is_empty() { let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));return; }
        let permit = grant(turn);observed(permit, || owner.step(permit));
    }
    panic!("returned connection candidate retained its native snapshot");
}

#[test]
fn history_edit_puzzle2d_native_connect_candidate_preserves_ordered_native_fields_and_granted_cancellation() {
    assert!(size_of::<Puzzle2dConnectHandlesCandidateCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let base: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let full: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap();match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", base["control"]["largePrefix"].as_str().unwrap().repeat(base["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let mut count = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let intent = base["cases"].as_array().unwrap().iter().find(|entry| entry["id"] == case["preparationCase"]).unwrap();
        let mut snapshot: Puzzle2dSnapshot = serde_json::from_value(full["snapshot"].clone()).unwrap();
        snapshot.nodes = intent["nodes"].as_array().unwrap().iter().map(|wire| { let mut node: crate::Puzzle2dNode = serde_json::from_value(wire.clone()).unwrap();for (index, handle) in wire["handles"].as_array().unwrap().iter().enumerate() { node.handles.get_mut(index).unwrap().id = text(&handle["id"]); }node.text = Some("untouched\0😀".into());node }).collect();
        snapshot.edges = case["edges"].as_array().unwrap().iter().map(|id| Puzzle2dEdge { id: text(id), visible: Some(false), locked: Some(true), source_tip: Some("retained\0😀".into()), ..Default::default() }).collect();
        let mut wire = intent["mutation"].clone();
        for field in ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y", "tolerance"] { if wire[field].is_string() { wire[field] = serde_json::json!(0); } }
        let mut payload: ConnectHandles = serde_json::from_value(wire).unwrap();
        payload.id = text(&intent["mutation"]["id"]);payload.source = text(&intent["mutation"]["source"]);payload.target = text(&intent["mutation"]["target"]);payload.index = case["index"].as_u64().map(|value| value as usize);
        payload.edge_kind = Some("".into());payload.source_tip = Some("original\0😀".into());payload.target_tip = Some("é".into());
        if intent["mutation"]["gap"] == "nan" { payload.gap = f64::NAN; }
        if case["name"] == "large-native" { payload.edge_kind = Some(PagedUtf8::from("kind-only\0😀".repeat(20000)));snapshot.meta.manifest_id = Some(PagedUtf8::from("retained-metadata\0😀".repeat(10000))); }
        let original = serde_json::to_value(&snapshot).unwrap();let original_payload = serde_json::to_value(&payload).unwrap();let bits = payload.gap.to_bits();
        let mut source = test_source_custody::admit();let mut mutation = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dConnectHandlesCandidateCursor::default);assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(observed(RetainedCloneGrant::default(), || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default())).progress(), RetainedCloneProgress::default());
        let mut complete = false;
        for turn in 0..1_000_000 { let permit = grant(turn);if matches!(observed(permit, || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { complete = true;break; } }
        assert!(complete);let result = cursor.take().unwrap();assert!(cursor.take().is_none());close(&mut cursor);
        let warning = match result.plan.warning { super::super::Puzzle2dConnectHandlesWarning::None => "none", super::super::Puzzle2dConnectHandlesWarning::MissingHandle => "missing-handle", super::super::Puzzle2dConnectHandlesWarning::TooFar => "too-far" };assert_eq!(warning, intent["warning"].as_str().unwrap());
        let changed = intent["status"] == "changed";assert_eq!(result.snapshot.is_some(), changed);assert_eq!(serde_json::to_value(result.plan.position).unwrap(), if changed { serde_json::json!(payload.index.unwrap_or(snapshot.edges.len()).min(snapshot.edges.len())) } else { serde_json::Value::Null });
        let mut expected = original.clone();
        if changed { let edge = Puzzle2dEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), edge_kind: payload.edge_kind.clone(), gap: payload.gap, shift: payload.shift, rise: payload.rise, rotation: payload.rotation, turn: payload.turn, tilt: payload.tilt, x: payload.x, y: payload.y, source_tip: payload.source_tip.clone(), target_tip: payload.target_tip.clone(), visible: None, locked: None };expected["edges"].as_array_mut().unwrap().insert(result.plan.position.unwrap(), serde_json::to_value(edge).unwrap()); }
        let current = result.snapshot.as_ref().unwrap_or(&snapshot);assert_eq!(serde_json::to_value(current).unwrap(), expected);assert_eq!(current.edges.iter().map(|edge| edge.id.to_string_owner()).collect::<Vec<_>>(), case["after"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>());
        if let Some(candidate) = result.snapshot { retire(candidate); }
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);assert_eq!(serde_json::to_value(&payload).unwrap(), original_payload);assert_eq!(payload.gap.to_bits(), bits);
        for pause in [0, 1, 17, 100] { let mut cancelled = Puzzle2dConnectHandlesCandidateCursor::default();for turn in 0..pause { let permit = grant(turn);if matches!(observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { break; } }close(&mut cancelled);assert!(cancelled.take().is_none()); }
        if case["name"] == "large-native" {
            for phase in [2, 5] { let mut cancelled = Puzzle2dConnectHandlesCandidateCursor::default();let mut copied = 0;for turn in 0..1_000_000 { let copying = cancelled.phase == phase;let permit = grant(turn);let step = observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));assert!(!matches!(step, RetainedCloneStep::Complete(_)));if copying { copied += step.progress().copied_bytes; }if copied >= corpus["control"]["cancelAfterBodyBytes"].as_u64().unwrap() as usize { break; } }assert!(copied >= 128);close(&mut cancelled); }
        }
        if case["name"] == "middle" { let mut cancelled = Puzzle2dConnectHandlesCandidateCursor::default();let mut placed = false;for turn in 0..1_000_000 { let inserting = cancelled.phase == 9;let permit = grant(turn);let step = observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));assert!(!matches!(step, RetainedCloneStep::Complete(_)));if inserting && cancelled.pending.is_none() && cancelled.phase == 9 { placed = true;break; } }assert!(placed);close(&mut cancelled); }
        let mut fenced = Puzzle2dConnectHandlesCandidateCursor::default();fenced.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant(1)).unwrap();let other = payload.clone();assert!(fenced.advance(source.borrow(&snapshot), mutation.borrow(&other), grant(1)).is_err());close(&mut fenced);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(cursor));assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);count += 1;
        eprintln!("[DEBUG] Puzzle2d ConnectHandles candidate {} preserves native ordered fields and untouched owners with zero constructor/terminal heap and exact three-lane clone/edge/insertion cancellation", case["name"].as_str().unwrap());
    }
    assert_eq!(count, 9);
}
