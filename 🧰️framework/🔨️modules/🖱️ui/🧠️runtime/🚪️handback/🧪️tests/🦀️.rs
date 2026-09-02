use super::*;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc};

//#region 🧪️HandbackEntry
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }
fn queued() -> SurfaceReconcileHandbackKey {
    let data = fixture();
    let owner = SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(data["surface"].as_str().unwrap()), data["generation"].as_u64().unwrap()).unwrap();
    let key = owner.handback_key().unwrap();
    drop(owner);
    key
}
fn drain() {
    for _ in 0..10000 {
        close_surface_reconcile_handback_one().unwrap();
        let empty = SURFACE_RECONCILE_HANDBACKS.lock().unwrap().retirement_len == 0;
        if empty { return; }
    }
    panic!("handback did not drain after the held mutex was released");
}
fn holder() -> (mpsc::Sender<()>, Arc<AtomicBool>, std::thread::JoinHandle<()>) {
    let (ready_tx, ready_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let released = Arc::new(AtomicBool::new(false));
    let witness = released.clone();
    let thread = std::thread::spawn(move || {
        let _guard = SURFACE_RECONCILE_HANDBACKS.lock().unwrap();
        ready_tx.send(()).unwrap();
        let _ = release_rx.recv_timeout(std::time::Duration::from_millis(100));
        witness.store(true, Ordering::Release);
    });
    ready_rx.recv().unwrap();
    (release_tx, released, thread)
}

#[test]
fn retained_handback_maintenance_entry_does_not_wait_for_registry() {
    let key = queued();
    let (release, released, thread) = holder();
    let _ = close_surface_reconcile_handback_one();
    let waited = released.load(Ordering::Acquire);
    let _ = release.send(()); thread.join().unwrap();
    let retained = SURFACE_RECONCILE_HANDBACKS.lock().unwrap().slots[key.slot].state.is_some();
    drain();
    assert!(retained);
    assert_eq!(waited, fixture()["entryWaits"].as_bool().unwrap());
}

#[test]
fn retained_handback_take_entry_does_not_wait_for_registry() {
    let key = queued();
    let (release, released, thread) = holder();
    let found = take_surface_reconcile_terminal(key);
    let waited = released.load(Ordering::Acquire);
    let _ = release.send(()); thread.join().unwrap();
    drop(found);
    let retained = SURFACE_RECONCILE_HANDBACKS.lock().unwrap().slots[key.slot].state.is_some();
    drain();
    assert!(retained);
    assert_eq!(waited, fixture()["entryWaits"].as_bool().unwrap());
}

#[test]
fn retained_handback_poison_is_fault_without_mutating_queued_owner() {
    let key = queued();
    let poisoned = std::thread::spawn(|| { let _guard = SURFACE_RECONCILE_HANDBACKS.lock().unwrap(); panic!("fixture registry poison"); });
    assert!(poisoned.join().is_err());
    let observed = format!("{:?}", close_surface_reconcile_handback_one());
    let unchanged = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|error| error.into_inner()).slots[key.slot].state.as_ref().unwrap().current.as_ref().unwrap().retire_scalar == 0;
    SURFACE_RECONCILE_HANDBACKS.clear_poison();
    drain();
    assert!(unchanged);
    assert_eq!(observed, "Err(\"surface handback registry is poisoned\")");
}
//#endregion 🧪️HandbackEntry
