//! 🧪️ Owned inverse payloads preserve the neutral literal identifier and exact granted layouts.

use super::*;
use semio_framework_value::retained_clone::RetainedCloneBorrowAuthority;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) } }

fn observed(grant: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!allocation.overflowed);
    assert!(step.progress().fits(grant));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(allocation.requested_bytes <= step.progress().retained_capacity_bytes, "owned inverse allocated unadmitted capacity: {allocation:?}, {step:?}");
    assert!(allocation.released_bytes <= step.progress().released_bytes, "owned inverse released unadmitted layout: {allocation:?}, {step:?}");
    step
}

fn close(cursor: &mut Puzzle2dMoveNodeInverseCursor) {
    cursor.begin_close();
    for turn in 0..1_000_000 {
        if cursor.terminal_is_empty() { return; }
        let permit = grant(turn);
        observed(permit, || cursor.close_step(permit));
    }
    panic!("owned inverse did not retire every retained payload and source alias");
}

fn retire(inverse: PagedList<Puzzle2dMutation, {usize::MAX}>) {
    let (owner, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ControlledRetirement::new(inverse));
    assert_eq!(allocation.requested_bytes, 0);
    assert_eq!(allocation.released_bytes, 0);
    let mut owner = match owner { Ok(owner) => owner, Err((error, _)) => panic!("{error:?}") };
    for turn in 0..1_000_000 {
        if owner.terminal_is_empty() { return; }
        let permit = grant(turn);
        observed(permit, || owner.step(permit));
    }
    panic!("returned inverse did not retire its native paged owner");
}

#[test]
fn history_edit_puzzle2d_owned_move_inverse_preserves_literal_ids_and_controlled_cancellation() {
    assert!(size_of::<Puzzle2dMoveNodeInverseCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let scalar = |value: &serde_json::Value| match value.as_str() { Some("nan") => f64::NAN, Some("positive-infinity") => f64::INFINITY, None => value.as_f64().unwrap(), _ => panic!("unknown authored scalar") };
    for case in corpus["cases"].as_array().unwrap() {
        let mut snapshot = Puzzle2dSnapshot::default();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|row| crate::Puzzle2dNode { id: text(&row["id"]), x: scalar(&row["x"]), y: scalar(&row["y"]), ..Default::default() }).collect();
        let payload = MoveNode { id: text(&case["mutation"]["id"]), new_x: scalar(&case["mutation"]["newX"]), new_y: scalar(&case["mutation"]["newY"]) };
        let original = serde_json::to_value(&snapshot).unwrap();
        let bits = [payload.new_x.to_bits(), payload.new_y.to_bits()];
        let source = RetainedCloneBorrowAuthority::new("owned inverse snapshot");
        let mutation = RetainedCloneBorrowAuthority::new("owned inverse original leaf");
        let (mut cursor, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dMoveNodeInverseCursor::default);
        assert!(!allocation.overflowed);assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,0);
        assert_eq!(observed(RetainedCloneGrant::default(), || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default())).progress(), RetainedCloneProgress::default());
        let mut complete = false;
        for turn in 0..1_000_000 {
            let permit = grant(turn);
            let step = observed(permit, || cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));
            if matches!(step, RetainedCloneStep::Complete(_)) { complete = true; break; }
        }
        assert!(complete, "owned inverse did not complete");
        let inverse = cursor.take().unwrap();
        assert_eq!(inverse.len(), usize::from(!case["inverse"].is_null()));
        if let Some(Puzzle2dMutation::MoveNode(inverse)) = inverse.first() {
            assert_eq!(inverse.id, payload.id);
            assert_eq!(inverse.new_x.to_bits(), case["inverse"]["newX"].as_f64().unwrap().to_bits());
            assert_eq!(inverse.new_y.to_bits(), case["inverse"]["newY"].as_f64().unwrap().to_bits());
        }
        assert!(cursor.take().is_none());
        close(&mut cursor);
        retire(inverse);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);
        assert_eq!([payload.new_x.to_bits(), payload.new_y.to_bits()], bits);
        if case["id"] == "large-prefix" {
            let mut cancelled = Puzzle2dMoveNodeInverseCursor::default();
            let mut copied = 0;
            for turn in 0..1_000_000 {
                let copying = cancelled.phase == 2;
                let permit = grant(turn);
                let step = observed(permit, || cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));
                assert!(!matches!(step, RetainedCloneStep::Complete(_)));
                if copying { copied += step.progress().copied_bytes; }
                if copied >= corpus["inverseOwnership"]["cancelAfterBodyBytes"].as_u64().unwrap() as usize { break; }
            }
            assert!(copied > 0);
            close(&mut cancelled);
            assert!(cancelled.take().is_none());
        }
        eprintln!("[DEBUG] Puzzle2d owned MoveNode inverse {} retained literal ID, zero-heap construction and granted native closure", case["id"].as_str().unwrap());
    }
}
