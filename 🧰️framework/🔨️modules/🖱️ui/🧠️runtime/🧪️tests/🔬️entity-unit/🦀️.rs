
use super::*;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};

#[test]
fn stale_entity_id_never_resolves_to_new_occupant() {
    let mut store = EntityStore::new();
    let e1 = store.insert(1i32);
    let id1 = e1.id();
    drop(e1);
    store.flush_releases();
    let e2 = store.insert(2i32);
    assert_eq!(e2.id().slot(), id1.slot());
    assert_ne!(e2.id().generation(), id1.generation());
    let stale: Entity<i32> = Entity { id: id1, handle: Rc::new(Handle { id: id1, release_queue: Weak::new() }), _marker: PhantomData };
    assert!(store.try_read(&stale).is_none());
    assert_eq!(store.try_read(&e2), Some(&2));
}

#[test]
fn release_is_queued_until_flush_releases() {
    let mut store = EntityStore::new();
    let e1 = store.insert(1i32);
    let id1 = e1.id();
    drop(e1);
    let e2 = store.insert(2i32);
    assert_ne!(e2.id().slot(), id1.slot());
    store.flush_releases();
    let e3 = store.insert(3i32);
    assert_eq!(e3.id().slot(), id1.slot());
}

#[test]
fn weak_entity_upgrade_fails_after_last_strong_drops() {
    let mut store = EntityStore::new();
    let e = store.insert(42i32);
    let weak = e.downgrade();
    assert!(weak.upgrade().is_some());
    drop(e);
    assert!(weak.upgrade().is_none());
}

#[test]
fn nested_lease_is_rejected_not_aliased() {
    let mut store = EntityStore::new();
    let e = store.insert(1i32);
    let taken = store.take_for_lease::<i32>(e.id);
    let result = catch_unwind(AssertUnwindSafe(|| store.take_for_lease::<i32>(e.id)));
    assert!(result.is_err());
    store.restore_after_lease(e.id, taken);
    assert_eq!(*store.read(&e), 1);
}

#[test]
fn read_during_lease_is_rejected() {
    let mut store = EntityStore::new();
    let e = store.insert(7i32);
    let taken = store.take_for_lease::<i32>(e.id);
    assert!(store.try_read(&e).is_none());
    store.restore_after_lease(e.id, taken);
    assert_eq!(store.try_read(&e), Some(&7));
}

#[test]
fn value_restored_after_panicking_closure() {
    let mut store = EntityStore::new();
    let e = store.insert(10i32);
    let result = catch_unwind(AssertUnwindSafe(|| {
        store.update(&e, |_, _cx| panic!("boom"));
    }));
    assert!(result.is_err());
    assert_eq!(*store.read(&e), 10);
}

#[test]
fn effects_queue_rather_than_run_inline() {
    let mut store = EntityStore::new();
    let source = store.insert(0i32);
    let observer = store.insert(0i32);
    let counter = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let _sub = store.update(&observer, |_, cx| {
        cx.observe(&source, move |t, _cx| {
            *t += 1;
            counter_clone.set(counter_clone.get() + 1);
        })
    });
    store.update(&source, |_, cx| cx.notify());
    assert_eq!(counter.get(), 0);
    store.flush_effects();
    assert_eq!(counter.get(), 1);
}

#[test]
fn dropped_subscription_stops_delivering() {
    let mut store = EntityStore::new();
    let source = store.insert(0i32);
    let observer = store.insert(0i32);
    let counter = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let sub = store.update(&observer, |_, cx| {
        cx.observe(&source, move |_, _cx| {
            counter_clone.set(counter_clone.get() + 1);
        })
    });
    store.update(&source, |_, cx| cx.notify());
    store.flush_effects();
    assert_eq!(counter.get(), 1);
    drop(sub);
    store.update(&source, |_, cx| cx.notify());
    store.flush_effects();
    assert_eq!(counter.get(), 1);
}

#[test]
fn defer_effects_queue_rather_than_run_inline() {
    let mut store = EntityStore::new();
    let e = store.insert(0i32);
    let flag = Rc::new(Cell::new(false));
    let flag_clone = flag.clone();
    store.update(&e, |_, cx| {
        cx.defer(move |_store| flag_clone.set(true));
    });
    assert!(!flag.get());
    let deferred = store.drain_deferred();
    assert_eq!(deferred.len(), 1);
    assert!(!flag.get());
}

#[test]
fn spawn_local_queues_future_for_the_embedder() {
    let mut store = EntityStore::new();
    let e = store.insert(0i32);
    store.update(&e, |_, cx| {
        cx.spawn_local(async {});
    });
    assert_eq!(store.drain_tasks().len(), 1);
}
