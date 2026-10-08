//! 🧪️ Scalar preparation preserves the neutral RFC6902 inverse and refusal ordering.

use super::*;
use semio_framework_value::paged::PagedUtf8;
use crate::test_source_custody;

fn close(cursor: &mut Puzzle2dMoveNodePreparationCursor) {
    cursor.begin_close();
    for _ in 0..32 {
        if cursor.terminal_is_empty() { return; }
        let copy=cursor.next_close_copy_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy).unwrap();let release=cursor.next_close_release_byte_demand().unwrap();let depth=cursor.next_close_depth_demand().unwrap();assert!(copy+capacity+release<=4096);
        let permit=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(permit));
        assert_eq!(allocation.requested_bytes, 0);
        assert_eq!(allocation.released_bytes, 0);
        assert!(!allocation.overflowed);
        assert!(step.unwrap().progress().fits(permit));
    }
    panic!("scalar move preparation did not close every native alias");
}

#[test]
fn history_edit_puzzle2d_scalar_move_preparation_preserves_inverse_and_refusal_without_owned_payload_copy() {
    assert!(std::mem::size_of::<Puzzle2dMoveNodePreparationCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", controls["largePrefix"].as_str().unwrap().repeat(controls["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let scalar = |value: &serde_json::Value| match value.as_str() { Some("nan") => f64::NAN, Some("positive-infinity") => f64::INFINITY, None => value.as_f64().unwrap(), _ => panic!("unknown neutral scalar") };
    let grant = RetainedCloneGrant { maximum_items: controls["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: controls["maximumBytes"].as_u64().unwrap() as usize, maximum_depth:64,..Default::default() };
    for case in corpus["cases"].as_array().unwrap() {
        let mut snapshot = Puzzle2dSnapshot::default();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|row| crate::Puzzle2dNode { id: text(&row["id"]), x: scalar(&row["x"]), y: scalar(&row["y"]), ..Default::default() }).collect();
        let payload = MoveNode { id: text(&case["mutation"]["id"]), new_x: scalar(&case["mutation"]["newX"]), new_y: scalar(&case["mutation"]["newY"]) };
        let original = serde_json::to_value(&snapshot).unwrap();
        let bits = [payload.new_x.to_bits(), payload.new_y.to_bits()];
        let mut source = test_source_custody::admit();
        let mut mutation = test_source_custody::admit();
        let mut cursor = Puzzle2dMoveNodePreparationCursor::default();
        let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default()));
        assert_eq!(step.unwrap(), Puzzle2dMoveNodePreparationStep::Pending(RetainedCloneProgress::default()));
        assert_eq!(allocation.requested_bytes, 0);
        assert_eq!(allocation.released_bytes, 0);
        let mut plan = None;
        for turn in 0..100_000 {
            let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};
            let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));
            assert_eq!(allocation.requested_bytes, 0);
            assert_eq!(allocation.released_bytes, 0);
            assert!(!allocation.overflowed);
            let step = step.unwrap();
            let progress = match step { Puzzle2dMoveNodePreparationStep::Pending(progress) | Puzzle2dMoveNodePreparationStep::Complete { progress, .. } => progress };
            assert!(progress.fits(permit));
            if let Puzzle2dMoveNodePreparationStep::Complete { plan: value, .. } = step { plan = Some(value); break; }
        }
        let plan = plan.expect("scalar preparation did not complete");
        let status = match plan.disposition { Puzzle2dMoveNodeDisposition::Changed => "changed", Puzzle2dMoveNodeDisposition::NoOp => "no-op", Puzzle2dMoveNodeDisposition::TargetMissing => "target-missing", Puzzle2dMoveNodeDisposition::NonfiniteX => "nonfinite-x", Puzzle2dMoveNodeDisposition::NonfiniteY => "nonfinite-y" };
        assert_eq!(status, case["status"].as_str().unwrap());
        assert_eq!(serde_json::to_value(plan.node_index).unwrap(), case["index"]);
        match plan.previous_position {
            Some([x,y]) => { assert_eq!(case["inverse"]["id"], case["mutation"]["id"]); assert_eq!(x.to_bits(), case["inverse"]["newX"].as_f64().unwrap().to_bits()); assert_eq!(y.to_bits(), case["inverse"]["newY"].as_f64().unwrap().to_bits()); },
            None => assert!(case["inverse"].is_null()),
        }
        assert_eq!(cursor.take(), Some(plan));
        assert_eq!(cursor.take(), None);
        close(&mut cursor);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);
        assert_eq!([payload.new_x.to_bits(), payload.new_y.to_bits()], bits);
        let mut cancelled = Puzzle2dMoveNodePreparationCursor::default();
        cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant).unwrap();
        let swapped = payload.clone();
        assert!(cancelled.advance(source.borrow(&snapshot), mutation.borrow(&swapped), grant).is_err());
        close(&mut cancelled);
        assert_eq!(cancelled.take(), None);
        if case["id"] == "large-prefix" {
            let mut cancelled = Puzzle2dMoveNodePreparationCursor::default();
            for turn in 0..12 { let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};assert!(matches!(cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit).unwrap(), Puzzle2dMoveNodePreparationStep::Pending(_))); }
            close(&mut cancelled);
            assert_eq!(cancelled.take(), None);
        }
        test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
    }
    eprintln!("[DEBUG] Puzzle2d scalar preparation preserved seven RFC6902 inverse/refusal plans, original borrowed payloads, zero-allocation advance and exact alias cancellation");
}
