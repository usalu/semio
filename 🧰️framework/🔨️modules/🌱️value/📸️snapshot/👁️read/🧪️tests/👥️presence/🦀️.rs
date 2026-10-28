//! 👁️ Original cross-worker read return, alias barrier and registry contention laws.
use crate::presence_test_retirement::CLOSE_GRANT;
use crate::*;
use crate::presence_test_retirement::{admit_returned_string,finish_box,finish_registry,finish,CLOSE_GRANT};
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[test]
fn retained_presence_read_return_releases_alias_before_cross_worker_reclamation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧪️tests/🧫️fixtures/🧹️retirement.json")).unwrap();
    let text = fixture["readerReturn"]["text"].as_str().unwrap();
    let expected: String = serde_json::from_value(fixture["readerReturn"]["text"].clone()).unwrap();
    assert_eq!(expected, text);
    for variant in fixture["readerReturn"]["variants"].as_array().unwrap() {
        let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
        let root = Arc::new(text.to_string());
        let lease = registry.try_issue(root.clone()).unwrap();
        let variant = variant.as_str().unwrap().to_string();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let worker_barrier = barrier.clone();
        let worker = std::thread::spawn(move || {
            worker_barrier.wait();
            match variant.as_str() {
                "typed-drop" => { drop(SnapshotRead::new(root, lease)); None }
                "erased-drop" => { drop(ErasedSnapshotRead::new(root, lease)); None }
                "explicit-return" => { assert!(SnapshotRead::new(root, lease).return_to_registry()); None }
                "witness-return" => Some(SnapshotRead::new(root, lease).return_to_registry_witness().unwrap()),
                _ => unreachable!(),
            }
        });
        barrier.wait();
        let mut final_owner = None;
        for _ in 0..100_000 {
            final_owner = admit_returned_string(&registry, text);
            if final_owner.is_some() { break; }
            std::thread::yield_now();
        }
        let witness = worker.join().unwrap();
        assert!(final_owner.is_some());
        finish_box(&mut final_owner);
        if let Some(witness) = witness {
            let mut close = ControlledRetirement::new(witness).unwrap_or_else(|_| panic!("original return witness"));
            finish(&mut close);
        }
        assert!(registry.terminal_is_empty());
        finish_registry(registry);
    }
}

#[test]
fn retained_presence_read_return_injected_alias_barrier_preserves_unreturned_guard() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let root = Arc::new(String::from("aä🧵"));
    let lease = registry.try_issue(root.clone()).unwrap();
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let worker_barrier = barrier.clone();
    let worker = std::thread::spawn(move || {
        let mut owner = Some(root);
        let mut lease = Some(lease);
        assert!(return_snapshot_read(&mut owner, &mut lease, || { worker_barrier.wait(); worker_barrier.wait(); }));
    });
    barrier.wait();
    assert!(!registry.has_returned());
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY { assert!(admit_returned_string(&registry, "aä🧵").is_none()); }
    assert!(!registry.terminal_is_empty());
    barrier.wait();
    worker.join().unwrap();
    let mut final_owner = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        final_owner = admit_returned_string(&registry, "aä🧵");
        if final_owner.is_some() { break; }
    }
    assert!(final_owner.is_some());
    finish_box(&mut final_owner);
    assert!(registry.terminal_is_empty());
    finish_registry(registry);
}

#[test]
fn retained_presence_read_transfer_contention_preserves_unreturned_capability() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let root = Arc::new(String::from("aä🧵"));
    let lease = registry.try_issue(root.clone()).unwrap();
    let read = ErasedSnapshotRead::new(root, lease);
    let held = registry.state.try_lock().unwrap();
    let worker_registry = registry.clone();
    let read = match std::thread::spawn(move || read.into_typed::<String>(&worker_registry)).join().unwrap() {
        Err(read) => read,
        Ok(_) => panic!("contended transfer cannot detach its unreturned guard"),
    };
    assert!(!registry.has_returned());
    let lease = read.lease.as_ref().unwrap();
    assert_eq!(registry.lease_generations[lease.index as usize].load(std::sync::atomic::Ordering::Acquire), lease.generation);
    drop(held);
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let worker_barrier = barrier.clone();
    let worker_registry = registry.clone();
    let worker = std::thread::spawn(move || {
        worker_barrier.wait();
        for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY { assert!(admit_returned_string(&worker_registry, "aä🧵").is_none()); }
        finish_registry(worker_registry);
    });
    barrier.wait();
    let mut read = read;
    let mut owner = None;
    for _ in 0..100_000 {
        match read.into_typed::<String>(&registry) {
            Ok(root) => { owner = Some(root); break; }
            Err(retained) => read = retained,
        }
        std::thread::yield_now();
    }
    worker.join().unwrap();
    assert!(!registry.has_returned());
    let original = owner.expect("exact transfer retries after bounded lock contention");
    assert_eq!(original.as_str(), "aä🧵");
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::retirement::arc_custody::admit_shared_retirement(original, CLOSE_GRANT, false));
    let (owner, receipt) = result.unwrap_or_else(|_| panic!("original transferred String"));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
    finish_box(&mut Some(owner));
    assert!(registry.terminal_is_empty());
    finish_registry(registry);
}

