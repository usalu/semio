use super::*;

define_id!(TestId, "test");

#[semio_framework_async_macros::async_test]
async fn insert_and_get_round_trips() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(42);
    assert_eq!(store.get(id), Some(&42));
}

#[semio_framework_async_macros::async_test]
async fn remove_then_get_returns_none() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(1);
    assert_eq!(store.remove(id), Some(1));
    assert_eq!(store.get(id), None);
}

#[semio_framework_async_macros::async_test]
async fn stale_handle_after_reuse_returns_none() {
    let mut store: Store<i32, TestId> = Store::new();
    let a = store.insert(1);
    store.remove(a);
    let b = store.insert(2);
    assert_eq!(b.raw_index(), a.raw_index(), "the freed slot should be reused (LIFO free list)");
    assert_ne!(b.raw_generation(), a.raw_generation());
    assert_eq!(store.get(a), None, "the stale handle must not alias the new value");
    assert_eq!(store.get(b), Some(&2));
}

#[semio_framework_async_macros::async_test]
async fn iteration_is_index_ordered_and_skips_removed_slots() {
    let mut store: Store<i32, TestId> = Store::new();
    let a = store.insert(10);
    let _b = store.insert(20);
    let c = store.insert(30);
    store.remove(a);
    let collected: Vec<(TestId, i32)> = store.iter().map(|(id, v)| (id, *v)).collect();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].1, 20);
    assert_eq!(collected[1].1, 30);
    assert!(collected[1].0.raw_index() == c.raw_index());
}

#[semio_framework_async_macros::async_test]
async fn len_reflects_only_live_entries() {
    let mut store: Store<i32, TestId> = Store::new();
    let a = store.insert(1);
    store.insert(2);
    assert_eq!(store.len(), 2);
    store.remove(a);
    assert_eq!(store.len(), 1);
    assert!(!store.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn free_bumps_generation_and_is_live_reflects_it() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(7);
    assert!(store.is_live(id));
    assert!(store.free(id));
    assert!(!store.is_live(id));
    assert!(!store.free(id), "freeing an already-stale id is a no-op, not a panic");
}

#[semio_framework_async_macros::async_test]
async fn display_uses_readable_tag_index_generation_format() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(1);
    assert_eq!(id.to_string(), format!("test-{}-{}", id.raw_index(), id.raw_generation()));
}

#[semio_framework_async_macros::async_test]
async fn serde_round_trips_an_id() {
    let mut store: Store<i32, TestId> = Store::new();
    let id = store.insert(1);
    let json = pack::to_json_string(&id);
    let back: TestId = pack::from_json_str(&json).unwrap();
    assert_eq!(back, id);
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn random_insert_remove_sequence_never_aliases_a_removed_id() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(83);
        let mut store: Store<u64, TestId> = Store::new();
        let mut live: Vec<(TestId, u64)> = Vec::new();
        let mut removed: Vec<TestId> = Vec::new();
        for i in 0..2000u64 {
            if !live.is_empty() && rng.next_bool(0.4) {
                let idx = rng.next_range(0, live.len() as u64) as usize;
                let (id, _) = live.remove(idx);
                store.remove(id);
                removed.push(id);
            } else {
                let id = store.insert(i);
                live.push((id, i));
            }
        }
        for (id, value) in &live {
            assert_eq!(store.get(*id), Some(value));
        }
        for id in &removed {
            if !live.iter().any(|(lid, _)| lid == id) {
                assert_eq!(store.get(*id), None, "removed id {id:?} must not resolve");
            }
        }
    }
}
