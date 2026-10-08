//! 🧪️ Literal CreateNode inverse IDs retain unconditional native ownership and cancellation.

use super::*;
use semio_framework_value::retained_clone::RetainedCloneBorrowAuthority;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) } }

fn observed(grant: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!heap.overflowed);
    assert!(step.progress().fits(grant));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(heap.requested_bytes <= step.progress().retained_capacity_bytes, "create inverse allocated unadmitted capacity: {heap:?}, {step:?}");
    assert!(heap.released_bytes <= step.progress().released_bytes, "create inverse released unadmitted layout: {heap:?}, {step:?}");
    step
}

fn close(cursor: &mut Puzzle2dCreateNodeInverseCursor) {
    cursor.begin_close();
    for turn in 0..1_000_000 {
        if cursor.terminal_is_empty() { return; }
        let permit = grant(turn);
        observed(permit, || cursor.close_step(permit));
    }
    panic!("create inverse retained an owner or source alias after closure");
}

fn retire(inverse: PagedList<Puzzle2dMutation, {usize::MAX}>) {
    let (owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ControlledRetirement::new(inverse));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let mut owner = match owner { Ok(owner) => owner, Err((error, _)) => panic!("{error:?}") };
    for turn in 0..1_000_000 {
        if owner.terminal_is_empty() { return; }
        let permit = grant(turn);
        observed(permit, || owner.step(permit));
    }
    panic!("create inverse returned owner did not retire");
}

#[test]
fn history_edit_puzzle2d_owned_create_inverse_preserves_unconditional_literal_intent_and_controlled_cancellation() {
    assert!(size_of::<Puzzle2dCreateNodeInverseCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let mut count = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let payload = CreateNode { node: crate::Puzzle2dNode { id: text(&case["mutation"]["node"]["id"]), text: Some(PagedUtf8::from("irrelevant immutable payload".repeat(6000))), ..Default::default() }, index: case["mutation"]["index"].as_u64().map(|value| value as usize) };
        let original = serde_json::to_value(&payload).unwrap();
        let mutation = RetainedCloneBorrowAuthority::new("create inverse original native leaf");
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dCreateNodeInverseCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(observed(RetainedCloneGrant::default(), || cursor.advance(mutation.borrow(&payload), RetainedCloneGrant::default())).progress(), RetainedCloneProgress::default());
        let mut complete = false;
        for turn in 0..1_000_000 {
            let permit = grant(turn);
            if matches!(observed(permit, || cursor.advance(mutation.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { complete = true; break; }
        }
        assert!(complete);
        let inverse = cursor.take().unwrap();
        assert_eq!(inverse.len(), 1);
        let Some(Puzzle2dMutation::DeleteNode(inverse_payload)) = inverse.first() else { panic!("create inverse lost its native DeleteNode intent") };
        assert_eq!(inverse_payload.id, text(&case["inverse"]["payload"]["id"]));
        assert!(cursor.take().is_none());
        close(&mut cursor);
        retire(inverse);
        assert_eq!(serde_json::to_value(&payload).unwrap(), original);
        let mut swapped = Puzzle2dCreateNodeInverseCursor::default();
        observed(grant(0), || swapped.advance(mutation.borrow(&payload), grant(0)));
        let other = payload.clone();
        assert!(swapped.advance(mutation.borrow(&other), grant(1)).is_err());
        close(&mut swapped);
        if case["id"] == "large-prefix" {
            let mut cancelled = Puzzle2dCreateNodeInverseCursor::default();
            let mut copied = 0;
            for turn in 0..1_000_000 {
                let permit = grant(turn);
                let step = observed(permit, || cancelled.advance(mutation.borrow(&payload), permit));
                assert!(!matches!(step, RetainedCloneStep::Complete(_)));
                copied += step.progress().copied_bytes;
                if copied >= corpus["inverseOwnership"]["cancelAfterBodyBytes"].as_u64().unwrap() as usize { break; }
            }
            assert!(copied > 0);
            close(&mut cancelled);
            assert!(cancelled.take().is_none());
        }
        count += 1;
        eprintln!("[DEBUG] Puzzle2d CreateNode inverse {} retained its unconditional DeleteNode intent with ID-only ownership, zero-heap construction and granted cancellation", case["id"].as_str().unwrap());
    }
    assert_eq!(count, 16);
}
