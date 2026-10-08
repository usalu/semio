//! 🧪️ Literal ConnectHandles inverse IDs retain unconditional native ownership and cancellation.

use super::*;
use crate::test_source_custody;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) } }

fn observed(grant: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!heap.overflowed);
    assert!(step.progress().fits(grant));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(heap.requested_bytes <= step.progress().retained_capacity_bytes, "connect inverse allocated unadmitted capacity: {heap:?}, {step:?}");
    assert!(heap.released_bytes <= step.progress().released_bytes, "connect inverse released unadmitted layout: {heap:?}, {step:?}");
    step
}

fn close(cursor: &mut Puzzle2dConnectHandlesInverseCursor) {
    cursor.begin_close();
    for _ in 0..1_000_000 {
        if cursor.terminal_is_empty() { return; }
        let copy=cursor.next_close_copy_byte_demand().unwrap();
        let capacity=cursor.next_close_capacity_byte_demand(copy).unwrap();
        let release=cursor.next_close_release_byte_demand().unwrap();
        let depth=cursor.next_close_depth_demand().unwrap();
        assert!(copy+capacity+release<=4096);
        let permit=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        assert_eq!(observed(Default::default(),||cursor.close_step(Default::default())).progress(),RetainedCloneProgress::default());
        for axis in 0..4 {
            let mut below=permit;
            let demand=match axis {0=>&mut below.maximum_copy_bytes,1=>&mut below.maximum_capacity_bytes,2=>&mut below.maximum_release_bytes,_=>&mut below.maximum_depth};
            if *demand==0 {continue;}*demand-=1;
            let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(below));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!heap.overflowed);
            match result{Ok(step)=>assert_eq!(step.progress(),RetainedCloneProgress::default()),Err(error)=>assert!(axis==3&&error.kind==semio_framework_value::ValueRefusalKind::DepthLimit)}
            assert_eq!((cursor.next_close_copy_byte_demand().unwrap(),cursor.next_close_capacity_byte_demand(copy).unwrap(),cursor.next_close_release_byte_demand().unwrap(),cursor.next_close_depth_demand().unwrap()),(copy,capacity,release,depth));
        }
        observed(permit, || cursor.close_step(permit));
    }
    panic!("connect inverse retained an owner or source alias after closure");
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
    panic!("connect inverse returned owner did not retire");
}

#[test]
fn history_edit_puzzle2d_owned_connect_inverse_preserves_unconditional_literal_intent_and_controlled_cancellation() {
    assert!(size_of::<Puzzle2dConnectHandlesInverseCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let mut count = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let payload = ConnectHandles { id: text(&case["id"]), source: text(&case["source"]), target: text(&case["target"]), edge_kind: None, gap: f64::from_bits(u64::from_str_radix(case["gapWord"].as_str().unwrap(), 16).unwrap()), shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0, source_tip: None, target_tip: None, tolerance: None, index: Some(case["position"].as_u64().unwrap() as usize) };
        let gap_bits = payload.gap.to_bits();
        let original = serde_json::to_value(&payload).unwrap();
        let mut mutation = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dConnectHandlesInverseCursor::default);
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
        let Some(Puzzle2dMutation::DisconnectHandles(inverse_payload)) = inverse.first() else { panic!("connect inverse lost its native DisconnectHandles intent") };
        assert_eq!(inverse_payload.id, text(&case["inverse"][0]["id"]));
        assert!(cursor.take().is_none());
        close(&mut cursor);
        retire(inverse);
        assert_eq!(serde_json::to_value(&payload).unwrap(), original);
        assert_eq!(payload.gap.to_bits(), gap_bits);
        let mut swapped = Puzzle2dConnectHandlesInverseCursor::default();
        observed(grant(0), || swapped.advance(mutation.borrow(&payload), grant(0)));
        let other = payload.clone();
        assert!(swapped.advance(mutation.borrow(&other), grant(1)).is_err());
        close(&mut swapped);
        if case["name"] == "large-id" {
            let mut cancelled = Puzzle2dConnectHandlesInverseCursor::default();
            let mut copied = 0;
            for turn in 0..1_000_000 {
                let permit = grant(turn);
                let step = observed(permit, || cancelled.advance(mutation.borrow(&payload), permit));
                assert!(!matches!(step, RetainedCloneStep::Complete(_)));
                copied += step.progress().copied_bytes;
                if copied >= corpus["control"]["cancelAfterBodyBytes"].as_u64().unwrap() as usize { break; }
            }
            assert!(copied > 0);
            close(&mut cancelled);
            assert!(cancelled.take().is_none());
        }
        test_source_custody::close(&mut mutation);
        count += 1;
        eprintln!("[DEBUG] Puzzle2d ConnectHandles inverse {} retained its unconditional DisconnectHandles intent with ID-only ownership, zero-heap construction and granted cancellation", case["name"].as_str().unwrap());
    }
    assert_eq!(count, 6);
}
