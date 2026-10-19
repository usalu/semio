//! 🧪️ Native ordered insertion preserves every unrelated owner and retires partial candidates.

use crate::standards::v1::subsets::any::schema::mutations::CreateNode;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::records::Puzzle2dNode;
use semio_framework_value::{ValueError, retirement::controlled::ControlledRetirement, retained_clone::RetainedCloneGrant, retained_clone::RetainedCloneProgress, retained_clone::RetainedCloneStep};
use std::mem::size_of;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::preparation::create_node_preparation::{Puzzle2dCreateNodeDisposition, Puzzle2dCreateNodePlan, Puzzle2dCreateNodePreparationCursor, Puzzle2dCreateNodePreparationStep};
#[allow(unused_imports)]
use semio_framework_value::{ValueRefusalKind, retained_clone::RetainedCloneBinding, retained_clone::RetainedFieldCursor, retained_clone::RetainedCloneCursor, retained_clone::RetainedCloneRef};
#[allow(unused_imports)]
use semio_framework_value_persistent::list::PagedListEditCursor;
#[allow(unused_imports)]
use std::{mem::ManuallyDrop, ops::Deref, ops::DerefMut};
use crate::standards::v1::subsets::any::schema::preparation::create_node_preparation::candidate::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::paged::PagedUtf8;
use crate::standards::v1::subsets::any::schema::testkit as test_source_custody;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 128), 1 => RetainedCloneGrant::one_payload_turn(4096, 128), _ => RetainedCloneGrant::one_release_turn(4096, 128) } }

fn observed(permit: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!heap.overflowed && step.progress().fits(permit));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(heap.requested_bytes <= step.progress().retained_capacity_bytes, "create candidate allocated unadmitted capacity: {heap:?}, {step:?}");
    assert!(heap.released_bytes <= step.progress().released_bytes, "create candidate released unadmitted capacity: {heap:?}, {step:?}");
    step
}

fn close(cursor: &mut Puzzle2dCreateNodeCandidateCursor) {
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
    panic!("returned create candidate did not retire its paged snapshot");
}

#[test]
fn history_edit_puzzle2d_native_create_candidate_preserves_ordered_placement_and_controlled_cancellation() {
    assert!(size_of::<Puzzle2dCreateNodeCandidateCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🟢️node/🌱create-node/🎮️prepare/🔣️.json")).unwrap();
    let full: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap();match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let mut count = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let mut snapshot: Puzzle2dSnapshot = serde_json::from_value(full["snapshot"].clone()).unwrap();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|id| Puzzle2dNode { id: text(id), text: Some("retained\0😀".into()), handles: [crate::standards::v1::subsets::any::schema::records::Puzzle2dHandle { id: "retained-handle".into(), angle: 0.25, ..Default::default() }].into_iter().collect(), ..Default::default() }).collect();
        let mut wire = case["mutation"]["node"].clone();
        for name in ["x", "y"] { if wire[name].is_string() { wire[name] = serde_json::json!(0); } }
        if let Some(handles) = wire.get_mut("handles").and_then(serde_json::Value::as_array_mut) { for handle in handles { if handle["angle"].is_string() { handle["angle"] = serde_json::json!(0); } } }
        let mut node: Puzzle2dNode = serde_json::from_value(wire).unwrap();node.id = text(&case["mutation"]["node"]["id"]);
        for name in ["x", "y"] { if let Some(scalar) = case["mutation"]["node"][name].as_str() { let value = if scalar == "nan" { f64::NAN } else { f64::INFINITY };if name == "x" { node.x = value; }else { node.y = value; } } }
        if let Some(handles) = case["mutation"]["node"].get("handles").and_then(serde_json::Value::as_array) { for (index, handle) in handles.iter().enumerate() { if handle["angle"] == "nan" { node.handles.get_mut(index).unwrap().angle = f64::NAN; } } }
        if case["id"] == "large-prefix" { node.text = Some(PagedUtf8::from("node-only\0😀".repeat(12000))); }
        let payload = CreateNode { node, index: case["mutation"]["index"].as_u64().map(|value| value as usize) };
        let original = serde_json::to_value(&snapshot).unwrap();let original_payload = serde_json::to_value(&payload).unwrap();
        let mut source = test_source_custody::admit();let mut mutation = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dCreateNodeCandidateCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(observed(RetainedCloneGrant::default(), || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default())).progress(), RetainedCloneProgress::default());
        let mut complete = false;
        for turn in 0..1_000_000 { let permit = grant(turn);if matches!(observed(permit, || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { complete = true;break; } }
        assert!(complete);
        let result = cursor.take().unwrap();assert!(cursor.take().is_none());
        assert_eq!(result.snapshot.is_some(), case["status"] == "changed");assert_eq!(serde_json::to_value(result.plan.position).unwrap(), case["position"]);
        close(&mut cursor);
        let mut expected = original.clone();
        if case["status"] == "changed" { expected["nodes"].as_array_mut().unwrap().insert(case["position"].as_u64().unwrap() as usize, serde_json::to_value(&payload.node).unwrap()); }
        assert_eq!(serde_json::to_value(result.snapshot.as_ref().unwrap_or(&snapshot)).unwrap(), expected);
        if let Some(candidate) = result.snapshot { retire(candidate); }
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);assert_eq!(serde_json::to_value(&payload).unwrap(), original_payload);
        if case["id"] == "large-prefix" {
            for phase in [2, 5] {
                let mut cancelled = Puzzle2dCreateNodeCandidateCursor::default();let mut copied = 0;
                for turn in 0..1_000_000 { let copying = cancelled.phase == phase;let permit = grant(turn);let step = observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));assert!(!matches!(step, RetainedCloneStep::Complete(_)));if copying { copied += step.progress().copied_bytes; }if copied >= corpus["forwardOwnership"]["cancelAfterBodyBytes"].as_u64().unwrap() as usize { break; } }
                assert!(copied > 0);close(&mut cancelled);assert!(cancelled.take().is_none());
            }
        }
        if case["id"] == "insert" {
            let mut cancelled = Puzzle2dCreateNodeCandidateCursor::default();let mut placed = false;
            for turn in 0..1_000_000 { let inserting = cancelled.phase == 9;let permit = grant(turn);let step = observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));assert!(!matches!(step, RetainedCloneStep::Complete(_)));if inserting && cancelled.pending.is_none() && cancelled.phase == 9 { placed = true;break; } }
            assert!(placed);close(&mut cancelled);assert!(cancelled.take().is_none());
        }
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(cursor));assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
        count += 1;
    }
    assert_eq!(count, 16);
}
