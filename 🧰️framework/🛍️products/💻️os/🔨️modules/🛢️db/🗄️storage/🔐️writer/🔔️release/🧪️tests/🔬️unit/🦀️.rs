use super::super::super::DbIoBackendControl;
use super::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

struct Counter(AtomicUsize);
impl std::task::Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn wal_writer_release_signal_preserves_exact_waits_across_writer_and_backend_reuse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap();
    let expected = &fixture["releaseSignal"];
    let counter = Arc::new(Counter(AtomicUsize::new(0)));
    let waker = Waker::from(counter.clone());
    let context = &mut Context::from_waker(&waker);
    let mut cell = WalWriterSignalCell::new();
    assert_eq!(cell.terminal_epoch.to_string(), expected["firstEpoch"]);
    let mut waits = Vec::new();
    for row in expected["writers"].as_array().unwrap() {
        let key = WalWriterKey { backend: DbIoBackendControl::Memory { slot: 0, generation: row["backendGeneration"].as_str().unwrap().parse().unwrap() }, slot: 0, generation: row["writerGeneration"].as_str().unwrap().parse().unwrap() };
        let required = cell.prepare(key).unwrap();
        assert!(matches!(cell.prepare(key), Err(DbError::Conflict(_))));
        for (old, epoch) in &waits {
            assert_eq!(cell.request(*old), expected["staleRequestAccepted"]);
            assert!(!cell.requested(*old));
            assert!(matches!(cell.poll(*old, *epoch, context), Poll::Ready(Ok(()))));
        }
        assert!(cell.request(key));
        assert!(cell.request(key));
        assert!(cell.requested(key));
        assert_eq!(cell.poll(key, required, context).is_ready(), expected["requestCompletesRelease"]);
        assert_eq!(cell.poll(key, required, context).is_ready(), expected["requestCompletesRelease"]);
        assert_eq!(counter.0.load(Ordering::SeqCst), waits.len());
        cell.finish(key).unwrap().wake();
        assert_eq!(cell.terminal_epoch.to_string(), row["terminalEpoch"]);
        assert_eq!(counter.0.load(Ordering::SeqCst), waits.len() + 1);
        waits.push((key, required));
        assert_eq!(waits.iter().all(|(old, epoch)| matches!(cell.poll(*old, *epoch, context), Poll::Ready(Ok(())))), expected["oldWaitSurvivesReuse"]);
    }
    let key = waits.last().unwrap().0;
    let before = cell.terminal_epoch;
    cell.prepare(key).unwrap();
    cell.cancel(key);
    assert_eq!(cell.terminal_epoch, before);
    cell.terminal_epoch = expected["maximumEpoch"].as_str().unwrap().parse().unwrap();
    assert!(matches!(cell.prepare(key), Err(DbError::LimitExceeded("WAL writer terminal epoch"))));
    assert!(cell.active.is_none());
    assert!(cell.waiter.is_none());
    assert_eq!(WAL_WRITER_SIGNAL_BACKING_BYTES, DB_IO_BACKEND_CONTROLS * WAL_WRITER_CAPACITY * size_of::<Mutex<WalWriterSignalCell>>());
    eprintln!("[DEBUG] WAL release cells retained requests, woke once at terminal, rejected stale keys and overflow, and preserved all completion epochs through backend reuse");
}
