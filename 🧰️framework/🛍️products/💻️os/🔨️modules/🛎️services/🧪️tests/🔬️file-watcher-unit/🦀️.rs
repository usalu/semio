
use super::*;
use semio_framework_async::{ProcessKind, WorkerPoolConfig};

fn wait_until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        if condition() {
            return;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("owned file-change watcher timed out");
}

fn wait_for_change(watcher: &mut OwnedFileChangeWatcher) {
    wait_until(|| watcher.poll_changed());
}

#[test]
fn detects_create_modify_rename_delete_and_cancellation_without_false_idle_events() {
    let root = std::env::temp_dir().join(format!("semio-owned-watcher-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&root).unwrap();
    let pool = Arc::new(WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2)));
    let cancelled;
    {
        let mut watcher = OwnedFileChangeWatcher::new(&root.join("document.db"), pool, Arc::new(|| {}));
        cancelled = watcher.cancelled.clone();
        wait_until(|| {
            let _ = watcher.poll_changed();
            watcher.previous.is_some()
        });
        assert!(!watcher.poll_changed());

        let first = root.join("document.db");
        std::fs::write(&first, b"a").unwrap();
        wait_for_change(&mut watcher);
        std::fs::write(&first, b"expanded").unwrap();
        wait_for_change(&mut watcher);
        let renamed = root.join("renamed.db");
        std::fs::rename(&first, &renamed).unwrap();
        wait_for_change(&mut watcher);
        std::fs::remove_file(&renamed).unwrap();
        wait_for_change(&mut watcher);
    }
    assert!(cancelled.load(Ordering::Acquire));
    std::fs::remove_dir_all(root).unwrap();
}
