//! 🧪️ Native topology lookup follows independent SQLite results without ownership copies.

use super::*;
fn close(cursor: &mut Puzzle2dLookupCursor) {
    cursor.begin_close();
    for _ in 0..32 {
        if cursor.terminal_is_empty() { return; }
        let copy=cursor.next_close_copy_byte_demand().unwrap();
        let capacity=cursor.next_close_capacity_byte_demand(copy).unwrap();
        let release=cursor.next_close_release_byte_demand().unwrap();
        let depth=cursor.next_close_depth_demand().unwrap();
        assert!(copy+capacity+release<=4096);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(grant));
        let progress=step.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!(allocation.requested_bytes, progress.retained_capacity_bytes);
        assert_eq!(allocation.released_bytes, progress.released_bytes);
        assert!(!allocation.overflowed);
    }
    panic!("native lookup retained an alias after bounded closure");
}

#[test]
fn history_edit_puzzle2d_native_lookup_follows_sqlite_first_match_and_exact_alias_closure() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let text = |value: &serde_json::Value| {
        let value = value.as_str().unwrap();
        match value.strip_prefix("$large:") {
            Some(suffix) => format!("{}{suffix}", controls["largePrefix"].as_str().unwrap().repeat(controls["largeRepeats"].as_u64().unwrap() as usize)).into(),
            None => PagedUtf8::<{usize::MAX}>::from(value),
        }
    };
    assert_eq!(controls["maximumItems"],1);
    let grant = RetainedCloneGrant::one_payload_turn(controls["maximumBytes"].as_u64().unwrap() as usize,1);
    for case in corpus["cases"].as_array().unwrap() {
        let scope = match case["scope"].as_str().unwrap() { "node" => Puzzle2dLookupScope::Node, "edge" => Puzzle2dLookupScope::Edge, "region" => Puzzle2dLookupScope::Region, "handle" => Puzzle2dLookupScope::Handle, _ => panic!("unknown authored lookup scope") };
        let mut snapshot = Puzzle2dSnapshot::default();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().enumerate().map(|(index, id)| crate::Puzzle2dNode { id: text(id), handles: case["handles"][index].as_array().unwrap().iter().map(|id| crate::Puzzle2dHandle { id: text(id), ..Default::default() }).collect(), ..Default::default() }).collect();
        snapshot.edges = case["edges"].as_array().unwrap().iter().map(|id| crate::Puzzle2dEdge { id: text(id), ..Default::default() }).collect();
        snapshot.target_regions = case["regions"].as_array().unwrap().iter().map(|id| crate::Puzzle2dTargetRegion { id: text(id), ..Default::default() }).collect();
        let target = text(&case["target"]);
        let original = serde_json::to_value(&snapshot).unwrap();
        let mut source = crate::test_source_custody::admit();
        let mut query = crate::test_source_custody::admit();
        let mut cursor = Puzzle2dLookupCursor::new(scope);
        let (zero, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), query.borrow(&target), RetainedCloneGrant::default()));
        assert_eq!(zero.unwrap(), Puzzle2dLookupStep::Pending(Default::default()));
        assert_eq!(allocation.requested_bytes, 0);
        assert_eq!(allocation.released_bytes, 0);
        let mut found = None;
        for turn in 0..100_000 {
            let grant=if turn%2==0{grant}else{RetainedCloneGrant::one_release_turn(4096,1)};
            let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(source.borrow(&snapshot), query.borrow(&target), grant));
            assert_eq!(allocation.requested_bytes, 0);
            assert_eq!(allocation.released_bytes, 0);
            assert!(!allocation.overflowed);
            let step = step.unwrap();
            let progress = match step { Puzzle2dLookupStep::Pending(progress) | Puzzle2dLookupStep::Complete { progress, .. } => progress };
            assert!(progress.fits(grant));
            if let Puzzle2dLookupStep::Complete { location, .. } = step { found = Some(location); break; }
        }
        let found = found.expect("native lookup failed to complete");
        let actual = found.map(|location| serde_json::json!({"outer":location.outer,"inner":location.inner})).unwrap_or(serde_json::Value::Null);
        assert_eq!(actual, case["expected"]);
        assert_eq!(cursor.take(), Some(found));
        assert_eq!(cursor.take(), None);
        close(&mut cursor);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);
        let mut cancelled = Puzzle2dLookupCursor::new(scope);
        cancelled.advance(source.borrow(&snapshot), query.borrow(&target), grant).unwrap();
        let swapped = snapshot.clone();
        assert!(cancelled.advance(source.borrow(&swapped), query.borrow(&target), grant).is_err());
        close(&mut cancelled);
        assert_eq!(cancelled.take(), None);
        if case["id"] == "large-prefix" {
            let mut cancelled = Puzzle2dLookupCursor::new(scope);
            for _ in 0..12 { assert!(matches!(cancelled.advance(source.borrow(&snapshot), query.borrow(&target), grant).unwrap(), Puzzle2dLookupStep::Pending(_))); }
            close(&mut cancelled);
            assert_eq!(cancelled.take(), None);
        }
        crate::test_source_custody::close(&mut query);
        crate::test_source_custody::close(&mut source);
    }
    eprintln!("[DEBUG] Puzzle2d native lookup matched six authored SQLite topologies, zero-allocation UTF8 comparison, source swap and exact alias closure");
}
