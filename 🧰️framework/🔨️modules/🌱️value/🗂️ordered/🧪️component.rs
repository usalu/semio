//! 🧪️ Exact ordered-map ownership and byte-frontier laws against std BTreeMap and committed fixtures.

use super::*;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

//#region 🔣️Fixture
fn key(value: &serde_json::Value) -> String {
    value["prefix"].as_str().unwrap().repeat(value["repetitions"].as_u64().unwrap() as usize) + value["suffix"].as_str().unwrap()
}

fn update<V>(cursor: &mut UpdateCursor<V>, grant: Grant) -> usize {
    let mut compared = 0;
    for _ in 0..1_000_000 {
        if cursor.is_complete() { return compared; }
        match cursor.advance(grant) {
            Step::Progress { completed_items, completed_bytes } => { assert!(completed_items <= 1 && completed_bytes <= grant.maximum_bytes); compared += completed_bytes; }
            Step::Complete => return compared,
            Step::Blocked => panic!("positive grant blocked ordered-map update"),
        }
    }
    panic!("ordered-map update did not finish")
}

fn close<V>(cursor: &mut UpdateCursor<V>, grant: Grant) -> Vec<V> {
    cursor.begin_close(); let mut values = Vec::new();
    for _ in 0..1_000_000 {
        match cursor.close_step(grant) {
            RetirementStep::Progress { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes),
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => { assert!(cursor.terminal_is_empty()); return values; }
            RetirementStep::Blocked => panic!("positive grant blocked ordered-map close"),
        }
    }
    panic!("ordered-map close did not finish")
}

fn retire<V>(map: OrderedMap<V>, grant: Grant) -> Vec<V> {
    let mut retirement = map.retire(); let mut values = Vec::new();
    for _ in 0..1_000_000 {
        match retirement.advance(grant) {
            RetirementStep::Progress { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes),
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => { assert!(retirement.is_empty()); return values; }
            RetirementStep::Blocked => panic!("positive grant blocked ordered-map retirement"),
        }
    }
    panic!("ordered-map retirement did not finish")
}

fn lookup<V>(cursor: &mut LookupCursor<V>, grant: Grant) -> usize {
    let mut compared = 0;
    for _ in 0..1_000_000 {
        if cursor.is_complete() { return compared; }
        match cursor.advance(grant) {
            Step::Progress { completed_items, completed_bytes } => { assert!(completed_items <= 1 && completed_bytes <= grant.maximum_bytes); compared += completed_bytes; }
            Step::Complete => return compared,
            Step::Blocked => panic!("positive grant blocked ordered-map lookup"),
        }
    }
    panic!("ordered-map lookup did not finish")
}

fn close_lookup<V>(cursor: &mut LookupCursor<V>, grant: Grant) -> Vec<V> {
    cursor.begin_close(); let mut values = Vec::new();
    for _ in 0..1_000_000 {
        match cursor.close_step(grant) {
            RetirementStep::Progress { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes),
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => { assert!(cursor.terminal_is_empty()); return values; }
            RetirementStep::Blocked => panic!("positive grant blocked ordered-map lookup close"),
        }
    }
    panic!("ordered-map lookup close did not finish")
}

fn check_tree<V>(root: &Root<V>) -> (usize, usize) {
    let Some(root) = root else { return (0, 0); };
    let (left_height, left_len) = check_tree(&root.left); let (right_height, right_len) = check_tree(&root.right);
    assert!(left_height.abs_diff(right_height) <= 1);
    assert!(root.height <= MAX_AVL_HEIGHT);
    assert_eq!(root.height, left_height.max(right_height) + 1); assert_eq!(root.len, left_len + right_len + 1);
    (root.height, root.len)
}
//#endregion 🔣️Fixture

//#region ⚖️Oracle
#[test]
fn fixture_operations_match_btree_map_under_every_actual_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️ordered-map.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        for maximum_bytes in fixture["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let grant = Grant { maximum_items: 1, maximum_bytes }; let mut map = OrderedMap::new(); let mut oracle = BTreeMap::new();
            for operation in row["operations"].as_array().unwrap() {
                let key = key(&operation["key"]);
                let mut cursor = if operation["op"] == "set" {
                    let value = operation["value"].as_str().unwrap().to_owned(); oracle.insert(key.clone(), value.clone()); map.begin_set(key, value)
                } else { oracle.remove(&key); map.begin_remove(key) };
                update(&mut cursor, grant); let displaced = std::mem::replace(&mut map, cursor.take_result().unwrap()); retire(displaced, grant); close(&mut cursor, grant);
                check_tree(&map.root);
                assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
                assert_eq!(serde_json::to_vec(&map).unwrap(), serde_json::to_vec(&oracle).unwrap());
                assert_eq!(map.iter().rev().map(|(key, _)| key).collect::<Vec<_>>(), oracle.keys().rev().collect::<Vec<_>>());
            }
            let expected: BTreeMap<_, _> = row["expected"].as_array().unwrap().iter().map(|entry| (key(&entry["key"]), entry["value"].as_str().unwrap().to_owned())).collect();
            assert_eq!(oracle, expected);
            retire(map, grant);
        }
    }
}

#[test]
fn all_rotation_and_successor_shapes_match_btree_map() {
    let grant = Grant { maximum_items: 1, maximum_bytes: 7 }; let mut map = OrderedMap::new(); let mut oracle = BTreeMap::new(); let mut state = 0x517c_c1b7_u64;
    for index in 0..4096 {
        state ^= state << 13; state ^= state >> 7; state ^= state << 17; let key = format!("{:03}", state % 257);
        let mut cursor = if index % 3 == 0 { oracle.remove(&key); map.begin_remove(key) } else { oracle.insert(key.clone(), index); map.begin_set(key, index) };
        update(&mut cursor, grant); let displaced = std::mem::replace(&mut map, cursor.take_result().unwrap()); retire(displaced, grant); close(&mut cursor, grant); check_tree(&map.root);
        assert_eq!(map.iter().map(|(key, value)| (key.clone(), *value)).collect::<BTreeMap<_, _>>(), oracle);
    }
    retire(map, grant);
}

#[test]
fn sorted_construction_preserves_fixed_height_and_double_ended_rank_bound() {
    let mut map = OrderedMap::new();
    for index in 0..4096 { map.insert(format!("{index:04}"), index); check_tree(&map.root); }
    let mut iter = map.iter();
    for index in 0..2048 {
        assert_eq!(*iter.next().unwrap().1, index);
        assert_eq!(*iter.next_back().unwrap().1, 4095 - index);
        assert_eq!(iter.len(), 4094 - index * 2);
    }
    assert!(iter.next().is_none() && iter.next_back().is_none());
    retire(map, Grant { maximum_items: 1, maximum_bytes: 1 });
}

#[test]
fn cold_serde_preserves_duplicate_semantics_and_retires_failed_partial_roots() {
    let map: OrderedMap<i64> = serde_json::from_str(r#"{"b":2,"a":1,"b":3}"#).unwrap();
    let oracle: BTreeMap<String, i64> = serde_json::from_str(r#"{"b":2,"a":1,"b":3}"#).unwrap();
    assert_eq!(serde_json::to_vec(&map).unwrap(), serde_json::to_vec(&oracle).unwrap());
    retire(map, Grant { maximum_items: 1, maximum_bytes: 1 });
    assert!(serde_json::from_str::<OrderedMap<i64>>(r#"{"a":1,"b":"bad"}"#).is_err());
}
//#endregion ⚖️Oracle

//#region 🔎️Lookup
#[test]
fn lookup_matches_oracle_with_exact_long_key_comparison_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️ordered-map.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        for row in fixture["lookupCases"].as_array().unwrap() {
            let grant = Grant { maximum_items: 1, maximum_bytes }; let mut map = OrderedMap::new();
            let source_value = row["sourceValue"].as_i64().unwrap();
            map.insert(key(&row["sourceKey"]), source_value);
            let mut cursor = map.begin_lookup(key(&row["query"]));
            assert_eq!(cursor.advance(Grant { maximum_items: 0, maximum_bytes }), Step::Blocked);
            assert_eq!(cursor.advance(Grant { maximum_items: 1, maximum_bytes: 0 }), Step::Blocked);
            assert_eq!(lookup(&mut cursor, grant), row["expectedComparedBytes"].as_u64().unwrap() as usize);
            assert_eq!(cursor.result().copied(), row["expected"].as_i64());
            assert!(retire(map, grant).is_empty());
            assert_eq!(close_lookup(&mut cursor, grant), [source_value]);
        }
    }
}

#[test]
fn lookup_cancellation_keeps_borrowed_payload_alive_until_owned_transfer() {
    for cancel in [0, 1, 9, 100, 20_000] {
        let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new();
        map.insert("🌊".repeat(2048), Payload(Arc::clone(&drops)));
        let mut cursor = map.begin_lookup("🌊".repeat(2048)); let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
        for _ in 0..cancel { cursor.advance(grant); }
        assert!(retire(map, grant).is_empty());
        let values = close_lookup(&mut cursor, grant);
        assert_eq!(drops.load(AtomicOrdering::SeqCst), 0); assert_eq!(values.len(), 1);
        drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
    }
}
//#endregion 🔎️Lookup

//#region 🧹️CancellationAndOwnership
struct Payload(Arc<AtomicUsize>);
impl Drop for Payload { fn drop(&mut self) { self.0.fetch_add(1, AtomicOrdering::SeqCst); } }

#[test]
fn cancellation_never_clones_payload_or_drops_final_value_inside_a_step() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️ordered-map.json")).unwrap();
    for cancel in fixture["cancelAfterSteps"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new(); map.insert("🌊".repeat(2048) + "a", Payload(Arc::clone(&drops)));
        let mut cursor = map.begin_set("🌊".repeat(2048) + "b", Payload(Arc::clone(&drops)));
        let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
        for _ in 0..cancel { cursor.advance(grant); }
        assert!(retire(map, grant).is_empty()); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
        let values = close(&mut cursor, grant); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
        assert_eq!(values.len(), 2); drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 2);
    }
}

#[test]
fn every_update_phase_closes_unclaimed_result_and_aliases_without_payload_drop() {
    for remove in [false, true] {
        for cancel in 0..48 {
            let grant = Grant { maximum_items: 1, maximum_bytes: 1 }; let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new();
            for key in ["4", "2", "6", "1", "3", "5", "7"] { map.insert(key.into(), Payload(Arc::clone(&drops))); }
            let mut cursor = if remove { map.begin_remove("4".into()) } else { map.begin_set("4".into(), Payload(Arc::clone(&drops))) };
            assert_eq!(cursor.advance(Grant { maximum_items: 0, maximum_bytes: 1 }), Step::Blocked);
            assert_eq!(cursor.advance(Grant { maximum_items: 1, maximum_bytes: 0 }), Step::Blocked);
            for _ in 0..cancel { cursor.advance(grant); }
            assert!(retire(map, grant).is_empty());
            let values = close(&mut cursor, grant); let expected = if remove { 7 } else { 8 };
            assert_eq!(values.len(), expected); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
            drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), expected);
        }
    }
}

#[test]
fn removed_payload_handoff_preserves_exact_last_owner() {
    let grant = Grant { maximum_items: 1, maximum_bytes: 1 }; let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new();
    map.insert("key".into(), Payload(Arc::clone(&drops)));
    let mut cursor = map.begin_remove("key".into()); update(&mut cursor, grant);
    let removed = cursor.take_removed().unwrap(); let result = cursor.take_result().unwrap();
    assert!(retire(map, grant).is_empty()); assert!(close(&mut cursor, grant).is_empty()); assert!(retire(result, grant).is_empty());
    assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    let payload = Arc::into_inner(removed).expect("removed payload must have one transferred owner");
    assert_eq!(drops.load(AtomicOrdering::SeqCst), 0); drop(payload); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
}

/// 🔒️ Deliberate contract violations retain their tiny test allocations instead of invoking recursive payload destruction.
#[test]
fn live_owner_drop_guards_never_destroy_payloads() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️ordered-map.json")).unwrap();
    assert_eq!(fixture["ownership"]["terminalOwners"], 0);
    for kind in 0..fixture["ownership"]["liveOwners"].as_array().unwrap().len() {
        let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new(); map.insert("key".into(), Payload(Arc::clone(&drops)));
        let guarded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match kind {
            0 => drop(map),
            1 => { let cursor = map.begin_set("next".into(), Payload(Arc::clone(&drops))); retire(map, Grant { maximum_items: 1, maximum_bytes: 1 }); drop(cursor); }
            2 => { let cursor = map.begin_lookup("key".into()); retire(map, Grant { maximum_items: 1, maximum_bytes: 1 }); drop(cursor); }
            _ => drop(map.retire()),
        }));
        assert!(guarded.is_err()); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn long_key_comparison_transfers_workers_and_retirement_counts_exact_key_bytes() {
    let prefix = "🌊".repeat(2048); let mut map = OrderedMap::new(); map.insert(prefix.clone() + "a", 1);
    let mut cursor = map.begin_set(prefix.clone() + "b", 2);
    let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
    assert!(retire(map, grant).is_empty());
    assert_eq!(cursor.advance(grant), Step::Progress { completed_items: 1, completed_bytes: 1 });
    let mut cursor = std::thread::spawn(move || { let bytes = update(&mut cursor, grant); assert_eq!(bytes + 1, (8192 + 1) * 2); cursor }).join().unwrap();
    let map = cursor.take_result().unwrap(); close(&mut cursor, grant);
    let mut retirement = map.retire(); let mut bytes = 0; let mut values = Vec::new();
    for _ in 0..100_000 {
        match retirement.advance(grant) {
            RetirementStep::Progress { released_bytes, .. } => { assert!(released_bytes <= 1); bytes += released_bytes; }
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => break, RetirementStep::Blocked => panic!("retirement blocked"),
        }
    }
    assert!(retirement.is_empty()); assert_eq!(bytes, 2 * (8192 + 1)); values.sort(); assert_eq!(values, [1, 2]);
}
//#endregion 🧹️CancellationAndOwnership

//#region 📤️SharedOwnership
#[test]
fn shared_release_is_empty_or_transfers_the_exact_final_frontier() {
    assert!(OrderedMap::<Payload>::new().release_shared().is_ok());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/📤️shared-owner.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        let drops = Arc::new(AtomicUsize::new(0));
        let key = fixture["key"]["text"].as_str().unwrap().repeat(fixture["key"]["repetitions"].as_u64().unwrap() as usize);
        let mut map = OrderedMap::new(); map.insert(key, Payload(Arc::clone(&drops)));
        assert!(map.clone().release_shared().is_ok());
        let mut retirement = match map.release_shared() { Err(frontier) => frontier, Ok(()) => panic!("final root was lost") };
        assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
        let mut released_bytes = 0; let mut payloads = Vec::new();
        loop {
            match retirement.advance(Grant { maximum_items: 1, maximum_bytes }) {
                RetirementStep::Progress { released_items, released_bytes: bytes } => { assert!(released_items <= 1 && bytes <= maximum_bytes); released_bytes += bytes; }
                RetirementStep::OwnedValue(value) => payloads.push(value),
                RetirementStep::Complete => break, RetirementStep::Blocked => panic!("positive grant blocked"),
            }
        }
        assert_eq!(released_bytes, 16384); assert!(retirement.is_empty()); assert_eq!(payloads.len(), 1);
        assert_eq!(drops.load(AtomicOrdering::SeqCst), 0); drop(payloads); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn racing_shared_releases_transfer_one_final_root_without_payload_destruction() {
    let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new(); map.insert("🌊".repeat(4096), Payload(Arc::clone(&drops)));
    let barrier = Arc::new(std::sync::Barrier::new(8));
    let mut owners: Vec<_> = (0..7).map(|_| map.clone()).collect(); owners.push(map);
    let workers: Vec<_> = owners.into_iter().map(|owner| { let barrier = Arc::clone(&barrier); std::thread::spawn(move || { barrier.wait(); owner.release_shared() }) }).collect();
    let mut shared = 0; let mut finals = Vec::new();
    for worker in workers { match worker.join().unwrap() { Ok(()) => shared += 1, Err(frontier) => finals.push(frontier) } }
    assert_eq!(shared, 7); assert_eq!(finals.len(), 1); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    let mut retirement = finals.pop().unwrap(); let mut values = Vec::new();
    loop { match retirement.advance(Grant { maximum_items: 1, maximum_bytes: 1 }) { RetirementStep::OwnedValue(value) => values.push(value), RetirementStep::Complete => break, _ => {} } }
    assert!(retirement.is_empty()); assert_eq!(values.len(), 1); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
}

#[test]
fn shared_upsert_preserves_key_and_nonclone_payload_allocations() {
    let drops = Arc::new(AtomicUsize::new(0)); let map = OrderedMap::new();
    let key = Arc::new("🌊".repeat(4096)); let value = Arc::new(Payload(Arc::clone(&drops)));
    let key_pointer = key.as_ptr(); let value_pointer = Arc::as_ptr(&value);
    let mut cursor = map.begin_set_shared(key, value); let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
    update(&mut cursor, grant); let result = cursor.take_result().unwrap();
    let (key, value) = result.iter().next().unwrap(); assert_eq!(key.as_ptr(), key_pointer); assert_eq!(value as *const Payload, value_pointer);
    assert!(map.release_shared().is_ok()); assert!(close(&mut cursor, grant).is_empty());
    let values = retire(result, grant); assert_eq!(values.len(), 1); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
}
//#endregion 📤️SharedOwnership
