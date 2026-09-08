
use super::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountWake(AtomicUsize);

impl std::task::Wake for CountWake {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn worker_deferred_wake_matches_neutral_capacity_generation_and_shutdown_drain() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️.json")).expect("strict deferred-wake fixture");
    assert_eq!(fixture["capacity"]["totalWaiters"].as_u64().unwrap() as usize, WORKER_DEFERRED_WAKE_CAPACITY);
    let registry = WorkerDeferredWakeRegistry::new();
    let counter = Arc::new(CountWake(AtomicUsize::new(0)));
    let waker = Waker::from(counter.clone());
    let mut tickets = Vec::new();
    for _ in 0..WORKER_DEFERRED_WAKE_PARTITIONS {
        let ticket = registry.install().expect("exact deferred partition capacity");
        for slot in 0..WORKER_DEFERRED_WAKES_PER_PARTITION {
            registry.defer_wake(ticket, slot, waker.clone()).expect("one exact waker per signal slot");
        }
        tickets.push(ticket);
    }
    assert_eq!(registry.install(), Err(WorkerDeferredWakeError::Capacity));
    assert_eq!(counter.0.load(Ordering::SeqCst), 0, "enqueue invokes no waker inline");
    registry.shutdown();
    let rejected = registry.defer_wake(tickets[0], 0, waker.clone()).expect_err("terminal admission returns its owner");
    assert_eq!(rejected.error(), WorkerDeferredWakeError::Shutdown);
    drop(rejected.into_waker());
    let mut drained = 0usize;
    while let Some(invocation) = registry.select() {
        invocation.run();
        drained += 1;
    }
    assert_eq!(drained, WORKER_DEFERRED_WAKE_CAPACITY);
    assert_eq!(counter.0.load(Ordering::SeqCst), WORKER_DEFERRED_WAKE_CAPACITY);
    assert!(!registry.has_pending());
    for ticket in tickets {
        assert!(registry.remove(ticket).expect("shutdown retains exact ticket generations until drain"));
    }
}
