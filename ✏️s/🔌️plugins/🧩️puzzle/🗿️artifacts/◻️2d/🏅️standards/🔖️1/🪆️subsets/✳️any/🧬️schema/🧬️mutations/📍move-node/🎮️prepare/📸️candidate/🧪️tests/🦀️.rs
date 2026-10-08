//! 🧪️ Native candidate ownership preserves complete snapshots and bounded cancellation.

use super::*;
use semio_framework_value::retained_clone::RetainedCloneBorrowAuthority;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 128), 1 => RetainedCloneGrant::one_payload_turn(4096, 128), _ => RetainedCloneGrant::one_release_turn(4096, 128) } }

fn observed(permit: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!allocation.overflowed && step.progress().fits(permit));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(allocation.requested_bytes <= step.progress().retained_capacity_bytes, "candidate allocated unadmitted capacity: {allocation:?}, {step:?}");
    assert!(allocation.released_bytes <= step.progress().released_bytes, "candidate released unadmitted layout: {allocation:?}, {step:?}");
    step
}

fn close(cursor: &mut Puzzle2dMoveNodeCandidateCursor) {
    cursor.begin_close();
    for turn in 0..1_000_000 { if cursor.terminal_is_empty() { return; } let permit = grant(turn); observed(permit, || cursor.close_step(permit)); }
    panic!("candidate retained a native owner after bounded cancellation");
}

fn retire(snapshot: Puzzle2dSnapshot) {
    let (result, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ControlledRetirement::new(snapshot));
    assert_eq!(allocation.requested_bytes, 0);
    assert_eq!(allocation.released_bytes, 0);
    let mut owner = match result { Ok(owner) => owner, Err((error, _)) => panic!("{error:?}") };
    for turn in 0..1_000_000 { if owner.terminal_is_empty() { return; } let permit = grant(turn); observed(permit, || owner.step(permit)); }
    panic!("returned native snapshot did not retire every physical page");
}

#[test]
fn history_edit_puzzle2d_native_move_candidate_preserves_untouched_owners_and_controlled_cancellation() {
    assert!(size_of::<Puzzle2dMoveNodeCandidateCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let full: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let scalar = |value: &serde_json::Value| match value.as_str() { Some("nan") => f64::NAN, Some("positive-infinity") => f64::INFINITY, None => value.as_f64().unwrap(), _ => panic!("unknown authored scalar") };
    for case in corpus["cases"].as_array().unwrap() {
        let mut snapshot: Puzzle2dSnapshot = serde_json::from_value(full["snapshot"].clone()).unwrap();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|row| crate::Puzzle2dNode { id: text(&row["id"]), x: scalar(&row["x"]), y: scalar(&row["y"]), text: Some("untouched\0😀".into()), handles: [crate::Puzzle2dHandle { id: "untouched-handle".into(), angle: 0.25, ..Default::default() }].into_iter().collect(), ..Default::default() }).collect();
        let payload = MoveNode { id: text(&case["mutation"]["id"]), new_x: scalar(&case["mutation"]["newX"]), new_y: scalar(&case["mutation"]["newY"]) };
        let original = serde_json::to_value(&snapshot).unwrap();
        let bits = [payload.new_x.to_bits(), payload.new_y.to_bits()];
        let source = RetainedCloneBorrowAuthority::new("native move original snapshot");
        let mutation = RetainedCloneBorrowAuthority::new("native move original payload");
        let (mut cursor, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dMoveNodeCandidateCursor::default);
        assert!(!allocation.overflowed);assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,0);
        assert_eq!(observed(RetainedCloneGrant::default(), || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default())).progress(), RetainedCloneProgress::default());
        let mut complete = false;
        for turn in 0..1_000_000 { let permit = grant(turn); if matches!(observed(permit, || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { complete = true; break; } }
        assert!(complete);
        let result = cursor.take().unwrap();
        assert_eq!(result.snapshot.is_some(), case["status"] == "changed");
        assert_eq!(result.plan.node_index, case["index"].as_u64().map(|value| value as usize));
        assert!(cursor.take().is_none());
        close(&mut cursor);
        let actual = result.snapshot.as_ref().unwrap_or(&snapshot);
        let expected_nodes = case["after"].as_array().unwrap();
        assert_eq!(actual.nodes.len(), expected_nodes.len());
        for (node, row) in actual.nodes.iter().zip(expected_nodes) { assert_eq!(node.id, text(&row["id"])); assert_eq!(node.x.to_bits(), scalar(&row["x"]).to_bits()); assert_eq!(node.y.to_bits(), scalar(&row["y"]).to_bits()); }
        let mut unchanged = serde_json::to_value(actual).unwrap();
        for (index, row) in unchanged["nodes"].as_array_mut().unwrap().iter_mut().enumerate() { row["x"] = original["nodes"][index]["x"].clone(); row["y"] = original["nodes"][index]["y"].clone(); }
        assert_eq!(unchanged, original);
        if let Some(candidate) = result.snapshot { retire(candidate); }
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);
        assert_eq!([payload.new_x.to_bits(), payload.new_y.to_bits()], bits);
        if case["id"] == "large-prefix" {
            let mut cancelled = Puzzle2dMoveNodeCandidateCursor::default();
            let mut copied = 0;
            for turn in 0..1_000_000 { let copying = cancelled.phase == 2; let permit = grant(turn); let step = observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit)); assert!(!matches!(step, RetainedCloneStep::Complete(_))); if copying { copied += step.progress().copied_bytes; } if copied >= corpus["forwardOwnership"]["cancelAfterBodyBytes"].as_u64().unwrap() as usize { break; } }
            assert!(copied > 0);
            close(&mut cancelled);
            assert!(cancelled.take().is_none());
        }
        eprintln!("[DEBUG] Puzzle2d native MoveNode candidate {} preserved full untouched owners, zero-heap construction and granted closure", case["id"].as_str().unwrap());
    }
}
