//! 🧪️ Native edge assembly preserves exact words, optional ownership and interior cancellation.

use super::*;
use crate::test_source_custody;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) } }
fn observed(permit: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!heap.overflowed);
    assert!(step.progress().fits(permit));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(heap.requested_bytes <= step.progress().retained_capacity_bytes, "edge assembly allocated unadmitted native capacity: {heap:?}, {step:?}");
    assert!(heap.released_bytes <= step.progress().released_bytes, "edge assembly released unadmitted native backing: {heap:?}, {step:?}");
    step
}
fn close(cursor: &mut Puzzle2dConnectEdgeCursor) {
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
    panic!("edge assembly retained native ownership after closure");
}
fn retire(edge: Puzzle2dEdge) {
    let (owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ControlledRetirement::new(edge));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let mut owner = match owner { Ok(owner) => owner, Err((error, _)) => panic!("{error:?}") };
    for turn in 0..1_000_000 { if owner.terminal_is_empty() { return; } let permit = grant(turn); observed(permit, || owner.step(permit)); }
    panic!("returned native edge did not retire");
}

#[test]
fn history_edit_puzzle2d_owned_connect_edge_assembly_preserves_native_fields_exact_words_and_granted_cancellation() {
    assert!(size_of::<Puzzle2dConnectEdgeCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => Text::from(format!("{}{suffix}", corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let optional = |value: &serde_json::Value| (!value.is_null()).then(|| text(value));
    let mut count = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let input = &case["mutation"];
        let word = |index: usize| u64::from_str_radix(input["scalarBits"][index].as_str().unwrap(), 16).unwrap();
        let payload = ConnectHandles { id: text(&input["id"]), source: text(&input["source"]), target: text(&input["target"]), edge_kind: optional(&input["edgeKind"]), source_tip: optional(&input["sourceTip"]), target_tip: optional(&input["targetTip"]), gap: f64::from_bits(word(0)), shift: f64::from_bits(word(1)), rise: f64::from_bits(word(2)), rotation: f64::from_bits(word(3)), turn: f64::from_bits(word(4)), tilt: f64::from_bits(word(5)), x: f64::from_bits(word(6)), y: f64::from_bits(word(7)), tolerance: Some(f64::from_bits(u64::from_str_radix(input["toleranceBits"].as_str().unwrap(), 16).unwrap())), index: Some(usize::MAX) };
        let mut authority = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dConnectEdgeCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(observed(Default::default(), || cursor.advance(authority.borrow(&payload), Default::default())).progress(), RetainedCloneProgress::default());
        let mut complete = false;
        for turn in 0..1_000_000 { let permit = grant(turn); if matches!(observed(permit, || cursor.advance(authority.borrow(&payload), permit)), RetainedCloneStep::Complete(_)) { complete = true; break; } }
        assert!(complete);
        let edge = cursor.take().unwrap();
        assert_eq!(edge.id, text(&case["edge"]["id"]));
        assert_eq!(edge.source, text(&case["edge"]["source"]));
        assert_eq!(edge.target, text(&case["edge"]["target"]));
        assert_eq!(edge.edge_kind, optional(&case["edge"]["edgeKind"]));
        assert_eq!(edge.source_tip, optional(&case["edge"]["sourceTip"]));
        assert_eq!(edge.target_tip, optional(&case["edge"]["targetTip"]));
        assert_eq!([edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y].map(f64::to_bits), std::array::from_fn::<_, 8, _>(word));
        assert_eq!((edge.visible, edge.locked), (None, None));
        assert!(cursor.take().is_none());
        close(&mut cursor);
        retire(edge);
        assert_eq!(payload.rotation.to_bits(), word(3));
        assert_eq!(payload.tolerance.unwrap().to_bits(), 0x7ff8000000000042);
        assert_eq!(payload.index, Some(usize::MAX));
        assert_eq!(payload.id, text(&input["id"]));
        let mut swapped = Puzzle2dConnectEdgeCursor::default();
        observed(grant(0), || swapped.advance(authority.borrow(&payload), grant(0)));
        let other = payload.clone();
        assert!(swapped.advance(authority.borrow(&other), grant(1)).is_err());
        close(&mut swapped);
        for pause in [0, 1, 17, 200] {
            let mut paused = Puzzle2dConnectEdgeCursor::default();
            for turn in 0..pause { let permit = grant(turn); let step = observed(permit, || paused.advance(authority.borrow(&payload), permit)); if matches!(step, RetainedCloneStep::Complete(_)) { break; } }
            close(&mut paused);
            assert!(paused.take().is_none());
        }
        if case["name"] == "large-native" {
            let mut cancelled = Puzzle2dConnectEdgeCursor::default();
            let mut actual_capacity = 0;
            let mut copied_body = 0;
            for turn in 0..1_000_000 { let permit = grant(turn); let step = observed(permit, || cancelled.advance(authority.borrow(&payload), permit)); assert!(!matches!(step, RetainedCloneStep::Complete(_))); actual_capacity += step.progress().retained_capacity_bytes; if actual_capacity > 0 { copied_body += step.progress().copied_bytes; } if copied_body >= 128 { break; } }
            assert!(actual_capacity > 0 && copied_body >= 128);
            close(&mut cancelled);
        }
        test_source_custody::close(&mut authority);
        count += 1;
        eprintln!("[DEBUG] Puzzle2d ConnectHandles native edge {} preserves exact scalar words, optional empty text, zero-birth ownership and every granted copy/capacity/release cancellation turn", case["name"].as_str().unwrap());
    }
    assert_eq!(count, 4);
}
