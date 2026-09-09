
use super::*;

#[test]
fn surface_output_pool_contended_drop_preserves_reserved_entry_until_exact_drain() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut queue = SurfaceReconcileOutputs::default();
    let reservation = queue.try_reserve(81, 32768).unwrap().unwrap();
    let entry_key = reservation.key.unwrap();
    let queue_key = queue.key.unwrap();
    let guard = REGISTRY.lock().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        drop(reservation);
        drop(queue);
        tx.send(()).unwrap();
    });
    let waited = rx.recv_timeout(std::time::Duration::from_millis(100)).is_err();
    assert!(guard.entry(entry_key, queue_key, 81).is_ok());
    drop(guard);
    worker.join().unwrap();
    for _ in 0..SLOTS * 3 {
        SurfaceReconcileOutputs::drain_one(1, 1).unwrap();
    }
    let registry = REGISTRY.lock().unwrap();
    assert!(!registry.queues[queue_key.index].occupied);
    assert!(registry.entries[entry_key.index].queue.is_none());
    assert!(!ENTRY_RETURNS[entry_key.index].load(Ordering::Acquire));
    assert!(!QUEUE_RETURNS[queue_key.index].load(Ordering::Acquire));
    assert_eq!(waited, fixture["dropWaits"].as_bool().unwrap());
    eprintln!("[DEBUG] output-pool held-mutex-drop-waits={waited} exact-return-drained=true");
}

#[test]
fn surface_output_pool_defers_reuse_and_rejects_stale_epoch_after_final_return() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut queue = SurfaceReconcileOutputs::default();
    let mut reservations: Vec<_> = (1..=64).map(|generation| queue.try_reserve(generation, 32768).unwrap().unwrap()).collect();
    let old = reservations.pop().unwrap();
    let old_key = old.key.unwrap();
    let queue_key = old.queue;
    let old_generation = old.generation;
    drop(old);
    assert_eq!(queue.try_reserve(100, 32768).unwrap().is_some(), fixture["returnedEntryReusableBeforeDrain"].as_bool().unwrap());
    for _ in 0..SLOTS {
        SurfaceReconcileOutputs::drain_one(1, 1).unwrap();
    }
    let mut replacement = queue.try_reserve(100, 32768).unwrap().unwrap();
    let new_key = replacement.key.unwrap();
    assert_eq!(old_key.index, new_key.index);
    assert_eq!(new_key.epoch, old_key.epoch + 1);
    assert_eq!(REGISTRY.lock().unwrap().entry(old_key, queue_key, old_generation).is_ok(), fixture["staleEpochAccepted"].as_bool().unwrap());
    while !replacement.close_step(1).unwrap().complete {}
    drop(replacement);
    assert!(!ENTRY_RETURNS[new_key.index].load(Ordering::Acquire));
    for owner in &mut reservations {
        while !owner.close_step(1).unwrap().complete {}
    }
    while !queue.close_step(1, 1).unwrap().complete {}
    eprintln!("[DEBUG] output-pool reuse-before-drain=false exact-epoch={} explicit-close-no-second-return=true", new_key.epoch);
}

#[test]
fn surface_output_pool_zero_grant_and_busy_registry_leave_authority_unchanged() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut queue = SurfaceReconcileOutputs::default();
    assert!(!queue.close_step(0, 4096).unwrap().progressed);
    let mut reservation = queue.try_reserve(91, 32768).unwrap().unwrap();
    let key = queue.key;
    let entry = reservation.key;
    let guard = REGISTRY.lock().unwrap();
    assert!(!queue.close_step(1, 4096).unwrap().progressed);
    assert!(!reservation.close_step(1).unwrap().progressed);
    assert!(queue.try_reserve(92, 32768).unwrap().is_none());
    assert_eq!(queue.key, key);
    assert_eq!(reservation.key, entry);
    assert!(!queue.closing);
    drop(guard);
    assert_eq!(queue.close_step(1, 0).unwrap().progressed, fixture["zeroGrantMutates"].as_bool().unwrap());
    while !reservation.close_step(1).unwrap().complete {}
    while !queue.close_step(1, 1).unwrap().complete {}
    eprintln!("[DEBUG] output-pool busy-refusal-exact=true zero-grant-mutates=false static-bytes={}", SurfaceReconcileOutputs::static_backing_bytes());
}
