//! 🧪️ Native deletion descriptors match literal SQLite/RFC6902 cascade order with zero heap.

use super::*;
use semio_framework_value::paged::PagedUtf8;
use crate::test_source_custody;

fn close(cursor: &mut Puzzle2dDeleteNodePreparationCursor) {
    cursor.begin_close();
    for _ in 0..64 {
        if cursor.terminal_is_empty() { return; }
        let copy=cursor.next_close_copy_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy).unwrap();let release=cursor.next_close_release_byte_demand().unwrap();let depth=cursor.next_close_depth_demand().unwrap();assert!(copy+capacity+release<=4096);
        let permit=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(permit));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!heap.overflowed);
        assert!(step.unwrap().progress().fits(permit));
    }
    panic!("borrowed deletion retained source aliases after closure");
}

#[test]
fn history_edit_puzzle2d_borrowed_delete_preparation_streams_cascade_ordinals_without_owned_payloads() {
    assert!(std::mem::size_of::<Puzzle2dDeleteNodePreparationCursor>() <= 4096);
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let text = |value: &serde_json::Value| { let value = value.as_str().unwrap(); match value.strip_prefix("$large:") { Some(suffix) => PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}", controls["largePrefix"].as_str().unwrap().repeat(controls["largeRepeats"].as_u64().unwrap() as usize))), None => value.into() } };
    let grant = RetainedCloneGrant::one_payload_turn(32,64);
    for case in corpus["cases"].as_array().unwrap() {
        let mut snapshot = Puzzle2dSnapshot::default();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().map(|node| crate::Puzzle2dNode { id: text(&node["id"]), handles: node["handles"].as_array().unwrap().iter().map(|id| crate::Puzzle2dHandle { id: text(id), ..Default::default() }).collect(), ..Default::default() }).collect();
        snapshot.edges = case["edges"].as_array().unwrap().iter().map(|edge| crate::Puzzle2dEdge { id: text(&edge["id"]), source: text(&edge["source"]), target: text(&edge["target"]), ..Default::default() }).collect();
        let payload = DeleteNode { id: text(&case["target"]) };
        let original = serde_json::to_value(&snapshot).unwrap();
        let original_payload = serde_json::to_value(&payload).unwrap();
        let mut source = test_source_custody::admit();
        let mut mutation = test_source_custody::admit();
        let (mut cursor, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dDeleteNodePreparationCursor::default);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (zero, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), RetainedCloneGrant::default()));
        assert_eq!(zero.unwrap(), Puzzle2dDeleteNodePreparationStep::Pending(RetainedCloneProgress::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut node = None;
        let mut edges = Vec::new();
        let mut complete = false;
        for turn in 0..250_000 {
            let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflowed);
            let progress = match step.unwrap() {
                Puzzle2dDeleteNodePreparationStep::Pending(progress) => progress,
                Puzzle2dDeleteNodePreparationStep::Node { index, progress } => { assert!(node.is_none()); node = Some(index); progress }
                Puzzle2dDeleteNodePreparationStep::Edge { index, progress } => { edges.push(index); progress }
                Puzzle2dDeleteNodePreparationStep::Complete(progress) => { complete = true; progress }
            };
            assert!(progress.fits(permit));
            if complete { break; }
        }
        assert!(complete);
        assert_eq!(serde_json::to_value(node.expect("one node descriptor")).unwrap(), case["node"]);
        assert_eq!(serde_json::to_value(&edges).unwrap(), case["severedEdges"]);
        assert_eq!(cursor.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant).unwrap(), Puzzle2dDeleteNodePreparationStep::Complete(RetainedCloneProgress::default()));
        close(&mut cursor);
        for pause in [0, 1, 3, 11, 97, 257] {
            let mut cancelled = Puzzle2dDeleteNodePreparationCursor::default();
            for turn in 0..pause { let permit=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,64)};if matches!(cancelled.advance(source.borrow(&snapshot), mutation.borrow(&payload), permit).unwrap(), Puzzle2dDeleteNodePreparationStep::Complete(_)) { break; } }
            close(&mut cancelled);
        }
        let mut swapped = Puzzle2dDeleteNodePreparationCursor::default();
        swapped.advance(source.borrow(&snapshot), mutation.borrow(&payload), grant).unwrap();
        let other = payload.clone();
        assert!(swapped.advance(source.borrow(&snapshot), mutation.borrow(&other), grant).is_err());
        close(&mut swapped);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);
        assert_eq!(serde_json::to_value(&payload).unwrap(), original_payload);
        test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
        eprintln!("[DEBUG] Puzzle2d deletion {} streamed one first-node descriptor and {} ordered cascade edges with constructor/normal advance/cancel alias closure zero heap", case["id"].as_str().unwrap(), edges.len());
    }
}
