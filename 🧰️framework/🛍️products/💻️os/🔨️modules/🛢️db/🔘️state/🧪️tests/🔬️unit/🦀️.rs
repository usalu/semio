use super::*;

//#region 🔖️PMap
#[test]
fn pmap_insert_get_roundtrip_across_many_keys() {
    let mut map: PMap<String, i64> = PMap::new();
    for i in 0..2000i64 {
        map = map.insert(format!("key-{i}"), i);
    }
    assert_eq!(map.len(), 2000);
    for i in 0..2000i64 {
        assert_eq!(map.get(&format!("key-{i}")), Some(&i));
    }
    assert_eq!(map.get(&"missing".to_string()), None);
}

#[test]
fn pmap_insert_is_persistent_old_version_unaffected() {
    let empty: PMap<String, i64> = PMap::new();
    let one = empty.insert("a".to_string(), 1);
    let two = one.insert("b".to_string(), 2);
    assert_eq!(empty.len(), 0);
    assert_eq!(one.len(), 1);
    assert_eq!(two.len(), 2);
    assert_eq!(one.get(&"b".to_string()), None);
    assert_eq!(two.get(&"a".to_string()), Some(&1));
    assert_eq!(two.get(&"b".to_string()), Some(&2));
}

#[test]
fn pmap_remove_then_reinsert_and_replace_semantics() {
    let map: PMap<String, i64> = PMap::new().insert("a".to_string(), 1).insert("b".to_string(), 2);
    let removed = map.remove(&"a".to_string());
    assert_eq!(removed.len(), 1);
    assert_eq!(removed.get(&"a".to_string()), None);
    assert_eq!(map.get(&"a".to_string()), Some(&1), "original map must be unaffected by remove on the derived one");

    let replaced = map.insert("a".to_string(), 99);
    assert_eq!(replaced.len(), 2, "replacing an existing key must not grow len");
    assert_eq!(replaced.get(&"a".to_string()), Some(&99));

    let no_op = map.remove(&"not-present".to_string());
    assert_eq!(no_op.len(), map.len());
}

#[test]
fn pmap_content_hash_is_order_independent_and_content_sensitive() {
    let forward: PMap<String, u64> = PMap::new().insert("a".to_string(), 1).insert("b".to_string(), 2).insert("c".to_string(), 3);
    let backward: PMap<String, u64> = PMap::new().insert("c".to_string(), 3).insert("b".to_string(), 2).insert("a".to_string(), 1);
    assert_eq!(forward.content_hash(), backward.content_hash());

    let different = forward.insert("a".to_string(), 999);
    assert_ne!(forward.content_hash(), different.content_hash());
}

#[test]
fn pmap_iter_visits_every_entry_exactly_once() {
    let map: PMap<String, i64> = (0..500).fold(PMap::new(), |m, i| m.insert(format!("k{i}"), i));
    let mut seen: Vec<i64> = map.iter().map(|(_, v)| *v).collect();
    seen.sort();
    assert_eq!(seen, (0..500).collect::<Vec<_>>());
}
//#endregion 🔖️PMap

//#region 🔖️PVec
#[test]
fn pvec_push_back_and_get_across_multiple_levels() {
    let mut vec: PVec<i64> = PVec::new();
    for i in 0..5000i64 {
        vec = vec.push_back(i);
    }
    assert_eq!(vec.len(), 5000);
    for i in 0..5000i64 {
        assert_eq!(vec.get(i as usize), Some(&i));
    }
    assert_eq!(vec.get(5000), None);
}

#[test]
fn pvec_push_back_is_persistent() {
    let base: PVec<i64> = PVec::new().push_back(1).push_back(2);
    let extended = base.push_back(3);
    assert_eq!(base.len(), 2);
    assert_eq!(base.get(2), None);
    assert_eq!(extended.len(), 3);
    assert_eq!(extended.get(2), Some(&3));
}

#[test]
fn pvec_set_and_pop_back() {
    let vec: PVec<i64> = (0..40).fold(PVec::new(), |v, i| v.push_back(i));
    let updated = vec.set(10, 999).expect("in bounds");
    assert_eq!(updated.get(10), Some(&999));
    assert_eq!(vec.get(10), Some(&10), "set must not mutate the original");

    let popped = vec.pop_back().expect("non-empty");
    assert_eq!(popped.len(), 39);
    assert_eq!(vec.len(), 40, "pop_back must not mutate the original");

    let empty: PVec<i64> = PVec::new();
    assert!(empty.pop_back().is_err());
    assert!(empty.set(0, 1).is_err());
}

#[test]
fn pvec_content_hash_is_order_sensitive() {
    let a: PVec<u64> = [1u64, 2, 3].into_iter().fold(PVec::new(), |v, x| v.push_back(x));
    let b: PVec<u64> = [3u64, 2, 1].into_iter().fold(PVec::new(), |v, x| v.push_back(x));
    assert_ne!(a.content_hash(), b.content_hash());
    let a_again: PVec<u64> = [1u64, 2, 3].into_iter().fold(PVec::new(), |v, x| v.push_back(x));
    assert_eq!(a.content_hash(), a_again.content_hash());
}
//#endregion 🔖️PVec

//#region 🔖️PText
#[test]
fn ptext_insert_delete_roundtrip() {
    let text = PText::from_text("hello world");
    let inserted = text.insert(5, ",").expect("in bounds");
    assert_eq!(inserted.to_string(), "hello, world");
    let deleted = inserted.delete(5, 6).expect("in bounds");
    assert_eq!(deleted.to_string(), "hello world");
}

#[test]
fn ptext_slice_and_concat() {
    let text = PText::from_text("hello world");
    let slice = text.slice(6, 11).expect("in bounds");
    assert_eq!(slice.to_string(), "world");
    let rejoined = PText::from_text("hello ").concat(&slice);
    assert_eq!(rejoined.to_string(), "hello world");
    assert_eq!(rejoined.len(), text.len());
}

#[test]
fn ptext_handles_multibyte_unicode_by_char_index() {
    let text = PText::from_text("héllo→wörld");
    let char_len = "héllo→wörld".chars().count();
    assert_eq!(text.len(), char_len);
    let inserted = text.insert(6, "🎉️").expect("in bounds");
    assert_eq!(inserted.to_string(), "héllo→🎉️wörld");
}

#[test]
fn ptext_out_of_bounds_operations_error_instead_of_panicking() {
    let text = PText::from_text("abc");
    assert!(text.insert(10, "x").is_err());
    assert!(text.slice(0, 10).is_err());
    assert!(text.delete(2, 1).is_err());
}

#[test]
fn ptext_edits_are_persistent() {
    let original = PText::from_text("abc");
    let edited = original.insert(1, "X").expect("in bounds");
    assert_eq!(original.to_string(), "abc");
    assert_eq!(edited.to_string(), "aXbc");
}
//#endregion 🔖️PText

//#region 🔖️PTree
#[test]
fn ptree_insert_get_and_ordered_iteration() {
    let mut tree: PTree<i64, String> = PTree::new();
    let mut keys: Vec<i64> = (0..300).collect();
    // insertion order deliberately not sorted, to exercise rebalancing on both sides.
    keys.sort_by_key(|k| (k * 2654435761u32 as i64) % 9973);
    for k in &keys {
        tree = tree.insert(*k, format!("v{k}"));
    }
    assert_eq!(tree.len(), 300);
    for k in 0..300i64 {
        assert_eq!(tree.get(&k), Some(&format!("v{k}")));
    }
    let iterated: Vec<i64> = tree.iter().map(|(k, _)| *k).collect();
    let mut sorted = iterated.clone();
    sorted.sort();
    assert_eq!(iterated, sorted, "PTree::iter must yield ascending key order");
}

#[test]
fn ptree_stays_balanced_within_the_avl_bound() {
    let tree: PTree<i64, ()> = (0..1000i64).fold(PTree::new(), |t, k| t.insert(k, ()));
    let n = tree.len() as f64;
    // AVL worst-case height bound: h <= 1.4405 * log2(n + 2) - 0.3277 (Knuth); a couple of
    // integer units of slack keeps this from being brittle to +/-1 rotation-count differences.
    let bound = (1.4405 * (n + 2.0).log2() - 0.3277).ceil() as u32 + 2;
    assert!(tree.height() <= bound, "height {} exceeds AVL bound {}", tree.height(), bound);
}

#[test]
fn ptree_remove_maintains_correctness_and_persistence() {
    let full: PTree<i64, i64> = (0..200i64).fold(PTree::new(), |t, k| t.insert(k, k * 10));
    let mut reduced = full.clone();
    for k in (0..200i64).step_by(3) {
        reduced = reduced.remove(&k);
    }
    for k in 0..200i64 {
        if k % 3 == 0 {
            assert_eq!(reduced.get(&k), None);
            assert_eq!(full.get(&k), Some(&(k * 10)), "original must be unaffected by removals on the derived tree");
        } else {
            assert_eq!(reduced.get(&k), Some(&(k * 10)));
        }
    }
}
//#endregion 🔖️PTree

//#region 🔖️PGraph
#[test]
fn pgraph_add_and_query_edges() {
    let graph: PGraph<String, (), &'static str> = PGraph::new().add_node("a".to_string(), ()).add_node("b".to_string(), ()).add_node("c".to_string(), ());
    let graph = graph.add_edge("a".to_string(), "b".to_string(), "ab").expect("nodes exist");
    let graph = graph.add_edge("a".to_string(), "c".to_string(), "ac").expect("nodes exist");

    assert!(graph.has_edge(&"a".to_string(), &"b".to_string()));
    assert_eq!(graph.edge_data(&"a".to_string(), &"b".to_string()), Some(&"ab"));
    let mut neighbors: Vec<String> = graph.neighbors(&"a".to_string()).into_iter().cloned().collect();
    neighbors.sort();
    assert_eq!(neighbors, vec!["b".to_string(), "c".to_string()]);
    assert_eq!(graph.predecessors(&"b".to_string()), vec![&"a".to_string()]);
}

#[test]
fn pgraph_add_edge_rejects_missing_nodes() {
    let graph: PGraph<String, (), ()> = PGraph::new().add_node("a".to_string(), ());
    let result = graph.add_edge("a".to_string(), "ghost".to_string(), ());
    assert!(matches!(result, Err(DbError::NotFound(_))));
}

#[test]
fn pgraph_remove_node_cascades_edge_cleanup() {
    let graph: PGraph<String, (), ()> = PGraph::new().add_node("a".to_string(), ()).add_node("b".to_string(), ()).add_node("c".to_string(), ());
    let graph = graph.add_edge("a".to_string(), "b".to_string(), ()).unwrap();
    let graph = graph.add_edge("b".to_string(), "c".to_string(), ()).unwrap();

    let reduced = graph.remove_node(&"b".to_string());
    assert!(!reduced.contains_node(&"b".to_string()));
    assert!(reduced.neighbors(&"a".to_string()).is_empty(), "edge a->b must be gone");
    assert!(reduced.predecessors(&"c".to_string()).is_empty(), "edge b->c must be gone");
    assert!(graph.contains_node(&"b".to_string()), "original graph must be unaffected");
    assert!(graph.has_edge(&"a".to_string(), &"b".to_string()));
}

#[test]
fn pgraph_content_hash_ignores_insertion_order() {
    let g1: PGraph<String, u64, u64> = PGraph::new().add_node("a".to_string(), 1).add_node("b".to_string(), 2).add_edge("a".to_string(), "b".to_string(), 7).unwrap();
    let g2: PGraph<String, u64, u64> = PGraph::new().add_node("b".to_string(), 2).add_node("a".to_string(), 1).add_edge("a".to_string(), "b".to_string(), 7).unwrap();
    assert_eq!(g1.content_hash(), g2.content_hash());
}
//#endregion 🔖️PGraph

//#region 🔖️Pages
#[semio_framework_async_macros::async_test]
async fn retained_state_exact_backing_cancel_capacity_and_close_are_hostile() {
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut control = StateCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut source = Vec::with_capacity(db_storage::DB_IO_PAGE_BYTES + 1);
    source.push(0x41);
    let mut entry = StateEntry::try_admit("one-byte", source, (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut control).await.unwrap();
    assert_eq!(entry.value().len(), 1);
    while entry.close_step().unwrap() {}
    assert!(entry.terminal_is_empty());

    let mut source = Vec::with_capacity(17);
    source.push(7);
    let pointer = source.as_ptr();
    let mut rejected = StateEntry::try_admit("max-plus-one", source, 16, &mut control).await.unwrap_err();
    while rejected.close_step().unwrap() {
        control.grant().unwrap();
    }
    let source = rejected.into_source().unwrap();
    assert_eq!(source.as_ptr(), pointer);
    assert_eq!(source.capacity(), 17);

    cancelled.store(true, std::sync::atomic::Ordering::Release);
    let mut cancelled_rejection = StateEntry::try_admit("cancelled", vec![1], 1, &mut control).await.unwrap_err();
    assert!(matches!(cancelled_rejection.error(), DbError::Unavailable(_)));
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    while cancelled_rejection.close_step().unwrap() {
        control.grant().unwrap();
    }

    let mut deadline_control = StateCursorControl::new(cancelled, std::time::Instant::now(), 16).unwrap();
    let mut source = Vec::with_capacity(2);
    source.push(0x44);
    let pointer = source.as_ptr();
    let mut deadline_rejection = StateEntry::try_admit("deadline", source, 2, &mut deadline_control).await.unwrap_err();
    assert!(matches!(deadline_rejection.error(), DbError::Unavailable(message) if message == "state cursor deadline reached"));
    while deadline_rejection.close_step().unwrap() {}
    assert_eq!(deadline_rejection.into_source().unwrap().as_ptr(), pointer);
}

#[semio_framework_async_macros::async_test]
async fn retained_state_sorted_fixed_capacity_hash_and_terminal_close_are_deterministic() {
    let mut map = RetainedStateMap::new();
    let mut control = StateCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    for index in (0..RETAINED_STATE_ENTRIES).rev() {
        let entry = StateEntry::try_admit(&format!("key-{index:02}"), vec![index as u8], 1, &mut control).await.unwrap();
        assert!(map.insert(entry).unwrap().is_none());
    }
    assert!(map.iter().map(StateEntry::key).is_sorted());
    let first = map.content_hash(&mut control).await.unwrap();
    let rejected = StateEntry::try_admit("overflow", vec![0xff], 1, &mut control).await.unwrap();
    let mut rejected = map.insert(rejected).unwrap_err();
    while rejected.close_step().unwrap() {
        control.grant().unwrap();
    }
    assert_eq!(first, map.content_hash(&mut control).await.unwrap());
    while map.close_step().unwrap() {
        control.grant().unwrap();
    }
    assert!(map.terminal_is_empty());
}

#[test]
fn page_store_interns_identical_bytes_once() {
    let mut store = PageStore::new();
    let h1 = store.intern(b"hello".to_vec());
    let h2 = store.intern(b"hello".to_vec());
    let h3 = store.intern(b"world".to_vec());
    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
    assert_eq!(store.len(), 2);
    assert_eq!(store.get(&h1).as_deref(), Some(b"hello".as_slice()));
}
//#endregion 🔖️Pages

//#region 🔖️Overlay
struct FixedBase(std::collections::HashMap<String, Vec<u8>>);
impl BaseSource for FixedBase {
    fn load(&self, path: &str) -> Result<Option<Vec<u8>>, DbError> {
        Ok(self.0.get(path).cloned())
    }
}

#[test]
fn overlay_reads_fall_through_to_base_when_unset() {
    let mut base_data = std::collections::HashMap::new();
    base_data.insert("x".to_string(), b"base-x".to_vec());
    let root = OverlayRoot::new(FixedBase(base_data));
    assert_eq!(root.get("x").unwrap(), Some(b"base-x".to_vec()));
    assert_eq!(root.get("missing").unwrap(), None);
}

#[test]
fn overlay_set_shadows_base_and_delete_tombstones_it() {
    let mut base_data = std::collections::HashMap::new();
    base_data.insert("x".to_string(), b"base-x".to_vec());
    let root = OverlayRoot::new(FixedBase(base_data));

    let (set_root, touched) = root.set("x", b"overlay-x".to_vec());
    assert_eq!(touched, TouchedRegion::write("x"));
    assert_eq!(set_root.get("x").unwrap(), Some(b"overlay-x".to_vec()));
    assert_eq!(root.get("x").unwrap(), Some(b"base-x".to_vec()), "original overlay root must be unaffected");

    let (deleted_root, _) = set_root.delete("x");
    assert_eq!(deleted_root.get("x").unwrap(), None, "delete must tombstone even though base still has a value");
}

#[test]
fn overlay_root_clone_shares_base_and_overlay_cheaply() {
    let root = OverlayRoot::new(EmptyBase);
    let (a, _) = root.set("p", b"1".to_vec());
    let b = a.clone();
    assert_eq!(b.get("p").unwrap(), Some(b"1".to_vec()));
    assert_eq!(a.overlay_len(), b.overlay_len());
}
//#endregion 🔖️Overlay

//#region 🔖️TouchedRegion
#[test]
fn touched_region_prefix_intersection() {
    let whole = TouchedRegion::write("doc/fields");
    let nested = TouchedRegion::read("doc/fields/title");
    let sibling = TouchedRegion::read("doc/other");
    assert!(whole.path_intersects(&nested));
    assert!(nested.path_intersects(&whole));
    assert!(!whole.path_intersects(&sibling));
    assert!(!TouchedRegion::read("doc/fie").path_intersects(&TouchedRegion::read("doc/fields")));
}

#[test]
fn touched_set_conflicts_only_when_a_write_is_involved() {
    let mut a = TouchedSet::new();
    a.record(TouchedRegion::write("doc/title"));
    let mut b_write = TouchedSet::new();
    b_write.record(TouchedRegion::write("doc/title"));
    assert!(a.conflicts_with(&b_write));

    let mut a_read = TouchedSet::new();
    a_read.record(TouchedRegion::read("doc/title"));
    let mut b_read = TouchedSet::new();
    b_read.record(TouchedRegion::read("doc/title"));
    assert!(!a_read.conflicts_with(&b_read), "two reads of the same region must not conflict");

    let mut disjoint = TouchedSet::new();
    disjoint.record(TouchedRegion::write("doc/body"));
    assert!(!a.conflicts_with(&disjoint));
}
//#endregion 🔖️TouchedRegion
