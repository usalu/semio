//! 🧪️ Native deletion preserves first-ID semantics and meters all partial owner retirement.

use super::*;
use semio_framework_value::paged::PagedUtf8;
use crate::test_source_custody;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 128), 1 => RetainedCloneGrant::one_payload_turn(4096, 128), _ => RetainedCloneGrant::one_release_turn(4096, 128) } }

fn observed(permit: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!heap.overflowed && step.progress().fits(permit));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(heap.requested_bytes <= step.progress().retained_capacity_bytes, "delete candidate allocated unadmitted capacity: {heap:?}, {step:?}");
    assert!(heap.released_bytes <= step.progress().released_bytes, "delete candidate released unadmitted capacity: {heap:?}, {step:?}");
    step
}

fn close(cursor: &mut Puzzle2dDeleteNodeCandidateCursor) {
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
    panic!("returned deletion snapshot did not retire its native pages");
}

#[test]
fn history_edit_puzzle2d_native_delete_candidate_preserves_first_ids_refusal_and_controlled_cancellation() {
    assert!(size_of::<Puzzle2dDeleteNodeCandidateCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let extra: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let full: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap();match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let mut count = 0;
    for case in corpus["cases"].as_array().unwrap().iter().chain(extra["cases"].as_array().unwrap()) {
        let mut snapshot: Puzzle2dSnapshot = serde_json::from_value(full["snapshot"].clone()).unwrap();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|node| Puzzle2dNode { id: text(&node["id"]), text: Some("retained\0😀".into()), handles: node["handles"].as_array().unwrap().iter().map(|id| crate::Puzzle2dHandle { id: text(id), angle: 0.25, ..Default::default() }).collect(), ..Default::default() }).collect();
        snapshot.edges = case["edges"].as_array().unwrap().iter().enumerate().map(|(index, edge)| Puzzle2dEdge { id: text(&edge["id"]), source: text(&edge["source"]), target: text(&edge["target"]), gap: -3.0, shift: index as f64, source_tip: Some("é\0😀".into()), visible: Some(index % 2 == 0), locked: Some(index % 2 != 0), ..Default::default() }).collect();
        let payload = DeleteNode { id: text(&case["target"]) };
        let original = serde_json::to_value(&snapshot).unwrap();let original_payload = serde_json::to_value(&payload).unwrap();
        let mut source = test_source_custody::admit();let mut mutation = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dDeleteNodeCandidateCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(observed(RetainedCloneGrant::default(), || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default())).progress(), RetainedCloneProgress::default());
        let mut complete = false;
        for turn in 0..1_000_000 { let permit = grant(turn);if matches!(observed(permit, || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { complete = true;break; } }
        assert!(complete);
        let result = cursor.take().unwrap();assert!(cursor.take().is_none());
        let status = case["status"].as_str().unwrap_or(if case["node"].is_null() { "unchanged" } else { "changed" });
        assert_eq!(result.disposition, match status { "changed" => Puzzle2dDeleteNodeDisposition::Changed, "refused" => Puzzle2dDeleteNodeDisposition::DuplicateRemovalTarget, _ => Puzzle2dDeleteNodeDisposition::Missing });
        assert_eq!(result.snapshot.is_some(), status == "changed");
        close(&mut cursor);
        let mut expected = original.clone();
        if status == "changed" {
            let node = case["node"].as_u64().map(|index| index as usize).unwrap_or_else(|| snapshot.nodes.iter().position(|node| node.id == payload.id).unwrap());
            expected["nodes"].as_array_mut().unwrap().remove(node);
            let removed = case.get("removed").unwrap_or(&case["severedEdges"]);
            for index in removed.as_array().unwrap().iter().rev() { expected["edges"].as_array_mut().unwrap().remove(index.as_u64().unwrap() as usize); }
        }
        assert_eq!(serde_json::to_value(result.snapshot.as_ref().unwrap_or(&snapshot)).unwrap(), expected);
        if let Some(candidate) = result.snapshot { retire(candidate); }
        for pause in [0, 1, 3, 11, 97, 257, 1021] {
            let mut cancelled = Puzzle2dDeleteNodeCandidateCursor::default();
            for turn in 0..pause { let permit = grant(turn);if matches!(observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { break; } }
            close(&mut cancelled);assert!(cancelled.take().is_none());
        }
        if case["id"] == "large-prefix" || case["id"] == "both-endpoints" {
            for phase in [10, 14, 18] {
                let mut cancelled = Puzzle2dDeleteNodeCandidateCursor::default();let mut reached = false;
                for turn in 0..1_000_000 { let active = cancelled.phase == phase;let permit = grant(turn);let step = observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));if active && step.progress().copied_bytes > 0 { reached = true;break; }assert!(!matches!(step, RetainedCloneStep::Complete(_))); }
                assert!(reached);close(&mut cancelled);
            }
        }
        let mut swapped = Puzzle2dDeleteNodeCandidateCursor::default();observed(grant(1), || swapped.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant(1)));
        let other = payload.clone();assert!(swapped.advance(source.borrow(&snapshot), mutation.borrow(&other), grant(1)).is_err());close(&mut swapped);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);assert_eq!(serde_json::to_value(&payload).unwrap(), original_payload);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(cursor));assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
        count += 1;
        eprintln!("[DEBUG] Puzzle2d DeleteNode candidate {} preserved exact first-ID removal/refusal and full untouched owners with zero-heap construction and granted cascade/clone/removal retirement", case["id"].as_str().unwrap());
    }
    assert_eq!(count, 11);
}
