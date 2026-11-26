//! 🧪️ Native topology lookup follows independent SQLite results without ownership copies.
use crate::standards::v1::subsets::any::schema::testkit as test_source_custody;

use crate::Puzzle2dSnapshot;
use semio_framework_value::paged::PagedUtf8;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use crate::standards::v1::subsets::any::schema::lookup::*;
use semio_framework_test_fixtures::fixture_text;
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
        let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| test_source_custody::close_turn(grant, |effect| cursor.close_step(effect)));
        let progress=step.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!(allocation.requested_bytes, progress.retained_capacity_bytes);
        assert_eq!(allocation.released_bytes, progress.released_bytes);
        assert_eq!((allocation.requested_bytes, allocation.released_bytes), (0, 0));
        assert!(!allocation.overflowed);
    }
    panic!("native lookup retained an alias after bounded closure");
}

#[test]
fn history_edit_puzzle2d_native_lookup_follows_sqlite_first_match_and_exact_alias_closure() {
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/📸️snapshot/🔎️lookup/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let text = |value: &serde_json::Value| {
        let value = value.as_str().unwrap();
        match value.strip_prefix("$large:") {
            Some(suffix) => format!("{}{suffix}", controls["largePrefix"].as_str().unwrap().repeat(controls["largeRepeats"].as_u64().unwrap() as usize)).into(),
            None => PagedUtf8::<{usize::MAX}>::from(value),
        }
    };
    assert_eq!(controls["maximumItems"],1);
    let comparison_bytes = controls["maximumComparisonBytes"].as_u64().unwrap() as usize;
    let grant = RetainedCloneGrant::one_payload_turn(4096, 1);
    for case in corpus["cases"].as_array().unwrap() {
        let scope = match case["scope"].as_str().unwrap() { "node" => Puzzle2dLookupScope::Node, "edge" => Puzzle2dLookupScope::Edge, "region" => Puzzle2dLookupScope::Region, "handle" => Puzzle2dLookupScope::Handle, _ => panic!("unknown authored lookup scope") };
        let mut snapshot = Puzzle2dSnapshot::default();
        snapshot.nodes = case["nodes"].as_array().unwrap().iter().enumerate().map(|(index, id)| crate::standards::v1::subsets::any::schema::records::Puzzle2dNode { id: text(id), handles: case["handles"][index].as_array().unwrap().iter().map(|id| crate::standards::v1::subsets::any::schema::records::Puzzle2dHandle { id: text(id), ..Default::default() }).collect(), ..Default::default() }).collect();
        snapshot.edges = case["edges"].as_array().unwrap().iter().map(|id| crate::standards::v1::subsets::any::schema::records::Puzzle2dEdge { id: text(id), ..Default::default() }).collect();
        snapshot.target_regions = case["regions"].as_array().unwrap().iter().map(|id| crate::standards::v1::subsets::any::schema::records::Puzzle2dTargetRegion { id: text(id), ..Default::default() }).collect();
        let target = text(&case["target"]);
        let original = serde_json::to_value(&snapshot).unwrap();
        let mut source = crate::standards::v1::subsets::any::schema::testkit::admit();
        let mut query = crate::standards::v1::subsets::any::schema::testkit::admit();
        let mut cursor = Puzzle2dLookupCursor::new(scope);
        let narrow_effect = RetainedCloneGrant::one_payload_turn(comparison_bytes, 1);
        assert!(source.borrow(&snapshot).binding_copy_bytes() > narrow_effect.maximum_copy_bytes);
        for denied in [RetainedCloneGrant::default(), narrow_effect] {
            let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| test_source_custody::run(denied, |effect| cursor.advance_with_comparison_limit(source.borrow(&snapshot), query.borrow(&target), comparison_bytes, effect)));
            assert_eq!(step.unwrap(), Puzzle2dLookupStep::Pending(Default::default()));
            assert_eq!(allocation.requested_bytes, 0);
            assert_eq!(allocation.released_bytes, 0);
        }
        let mut found = None;
        for turn in 1usize..=100_000 {
            let effect_grant=if turn%2==1{grant}else{RetainedCloneGrant::one_release_turn(4096,1)};
            let (step, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| test_source_custody::run(effect_grant, |effect| cursor.advance_with_comparison_limit(source.borrow(&snapshot), query.borrow(&target), comparison_bytes, effect)));
            assert_eq!(allocation.requested_bytes, 0);
            assert_eq!(allocation.released_bytes, 0);
            assert!(!allocation.overflowed);
            let step = step.unwrap();
            let progress = match step { Puzzle2dLookupStep::Pending(progress) | Puzzle2dLookupStep::Complete { progress, .. } => progress };
            assert!(progress.fits(effect_grant));
            if let Puzzle2dLookupStep::Complete { location, .. } = step {
                if case["id"] == "large-prefix" { assert!(turn.div_ceil(2) >= 2 * target.len().div_ceil(comparison_bytes)); }
                found = Some(location);
                break;
            }
        }
        let found = found.unwrap_or_else(|| panic!("native lookup failed to complete for {}", case["id"]));
        let actual = found.map(|location| serde_json::json!({"outer":location.outer,"inner":location.inner})).unwrap_or(serde_json::Value::Null);
        assert_eq!(actual, case["expected"]);
        assert_eq!(cursor.take(), Some(found));
        assert_eq!(cursor.take(), None);
        close(&mut cursor);
        assert_eq!(serde_json::to_value(&snapshot).unwrap(), original);
        let mut cancelled = Puzzle2dLookupCursor::new(scope);
        let bound = test_source_custody::run(grant, |effect| cancelled.advance_with_comparison_limit(source.borrow(&snapshot), query.borrow(&target), comparison_bytes, effect)).unwrap();
        assert!(matches!(bound, Puzzle2dLookupStep::Pending(progress) if progress.copied_items == 1));
        let swapped = snapshot.clone();
        assert!(test_source_custody::run(grant, |effect| cancelled.advance_with_comparison_limit(source.borrow(&swapped), query.borrow(&target), comparison_bytes, effect)).is_err());
        close(&mut cancelled);
        assert_eq!(cancelled.take(), None);
        if case["id"] == "large-prefix" {
            let mut cancelled = Puzzle2dLookupCursor::new(scope);
            for _ in 0..12 {
                let step = test_source_custody::run(grant, |effect| cancelled.advance_with_comparison_limit(source.borrow(&snapshot), query.borrow(&target), comparison_bytes, effect)).unwrap();
                assert!(matches!(step, Puzzle2dLookupStep::Pending(progress) if progress.copied_items == 1));
            }
            close(&mut cancelled);
            assert_eq!(cancelled.take(), None);
        }
        crate::standards::v1::subsets::any::schema::testkit::close(&mut query);
        crate::standards::v1::subsets::any::schema::testkit::close(&mut source);
    }
}
