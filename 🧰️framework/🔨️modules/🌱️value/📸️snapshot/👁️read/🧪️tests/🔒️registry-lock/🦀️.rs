//! 🔒️ Registry state lock laws: fail-closed admission under contention, and the standard mutex oracle for contention and poison, on the fixed lock itself.
use crate::*;

#[test]
fn snapshot_read_lease_contention_returns_the_exact_untouched_owner() {
    let registry = SnapshotReadRegistryHandle::new();
    let owner = Arc::new(String::from("contention-owner"));
    let guard = registry.state.try_lock().expect("hold exact registry contention fixture");
    let rejected = match registry.try_issue(owner.clone()) {
        Ok(_) => panic!("nonblocking admission rejects while the fixed registry is contended"),
        Err(rejected) => rejected,
    };
    assert!(Arc::ptr_eq(&owner, &rejected));
    assert_eq!(Arc::strong_count(&owner), 2, "contention creates no hidden owner clone");
    drop(guard);
    let lease = registry.try_issue(rejected.clone()).expect("the same owner retries after contention");
    let read = ErasedSnapshotRead::new(rejected, lease);
    drop(match read.into_typed::<String>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("retried exact owner handback succeeds"),
    });
    assert!(registry.terminal_is_empty());
}

#[test]
fn artifact_store_resident_registry_lock_matches_standard_contention_and_poison_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let oracle = std::sync::Mutex::new(());
    let registry = SnapshotReadLeaseRegistry::new();
    let standard = oracle.try_lock().unwrap();
    let actual = registry.state.try_lock().unwrap();
    assert_eq!(fixture["registryLockStates"][0], "available");
    assert!(matches!(oracle.try_lock(), Err(std::sync::TryLockError::WouldBlock)));
    assert!(matches!(registry.state.try_lock(), Err(std::sync::TryLockError::WouldBlock)));
    assert_eq!(fixture["registryLockStates"][1], "busy");
    drop((standard, actual));
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = oracle.try_lock().unwrap();
        panic!("standard registry poison witness");
    })).is_err());
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = registry.state.try_lock().unwrap();
        panic!("actual registry poison witness");
    })).is_err());
    assert!(matches!(oracle.try_lock(), Err(std::sync::TryLockError::Poisoned(_))));
    assert!(matches!(registry.state.try_lock(), Err(std::sync::TryLockError::Poisoned(_))));
    assert_eq!(fixture["registryLockStates"][2], "poisoned");
}

#[test]
fn erased_snapshot_read_refuses_a_swapped_owner_type_and_restores_the_exact_original() {
    let registry = SnapshotReadRegistryHandle::new();
    let owner = Arc::new(41i32);
    let lease = registry.try_issue(owner.clone()).expect("exact lease");
    let mut read = ErasedSnapshotRead::new(owner, lease);
    assert_eq!(read.get::<i32>().copied(), Some(41));
    let swapped: Option<Arc<dyn std::any::Any + Send + Sync>> = Some(Arc::new(17u8));
    let original = std::mem::replace(&mut read.owner, swapped);
    assert!(read.get::<i32>().is_none());
    assert_eq!(read.get::<u8>().copied(), Some(17));
    read.owner = original;
    assert_eq!(read.get::<i32>().copied(), Some(41));
    drop(match read.into_typed::<i32>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("restored exact owner retires"),
    });
    assert!(registry.terminal_is_empty());
}
