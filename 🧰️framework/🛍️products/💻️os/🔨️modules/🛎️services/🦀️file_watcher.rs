use semio_framework_async::{Lane, WorkerPool};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant, UNIX_EPOCH};

//#region 📸️Snapshot

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FileStamp {
    path: PathBuf,
    directory: bool,
    len: u64,
    modified_ns: u128,
}

struct DirectoryProbe {
    entries: std::fs::ReadDir,
    stamps: Vec<FileStamp>,
}

//#endregion 📸️Snapshot

//#region 🔭️Probe

fn submit_probe_step(pool: &Arc<WorkerPool>, root: PathBuf, cancelled: Arc<AtomicBool>, sender: mpsc::SyncSender<Result<Vec<FileStamp>, String>>, probe: Option<DirectoryProbe>) {
    let next_pool = pool.clone();
    pool.submit(
        Lane::Io,
        Box::new(move || {
            if cancelled.load(Ordering::Acquire) {
                return;
            }
            let mut probe = match probe {
                Some(probe) => probe,
                None => {
                    if let Err(error) = std::fs::create_dir_all(&root) {
                        let _ = sender.send(Err(format!("{}: {error}", root.display())));
                        return;
                    }
                    match std::fs::read_dir(&root) {
                        Ok(entries) => DirectoryProbe { entries, stamps: Vec::new() },
                        Err(error) => {
                            let _ = sender.send(Err(format!("{}: {error}", root.display())));
                            return;
                        }
                    }
                }
            };
            for _ in 0..8 {
                let Some(entry) = probe.entries.next() else {
                    probe.stamps.sort();
                    if !cancelled.load(Ordering::Acquire) {
                        let _ = sender.send(Ok(probe.stamps));
                    }
                    return;
                };
                let Ok(entry) = entry else { continue };
                let path = entry.path();
                let Ok(metadata) = entry.metadata() else {
                    continue;
                };
                let modified_ns = metadata.modified().ok().and_then(|modified| modified.duration_since(UNIX_EPOCH).ok()).map_or(0, |duration| duration.as_nanos());
                probe.stamps.push(FileStamp { path, directory: metadata.is_dir(), len: metadata.len(), modified_ns });
            }
            submit_probe_step(&next_pool, root, cancelled, sender, Some(probe));
        }),
    );
}

//#endregion 🔭️Probe

//#region 👁️Watcher

/// 👁️ Cancellable non-recursive directory watcher whose bounded probes run on the injected
/// worker pool's I/O lane. Polling consumes only completed immutable snapshots.
pub struct OwnedFileChangeWatcher {
    pool: Arc<WorkerPool>,
    root: PathBuf,
    cancelled: Arc<AtomicBool>,
    sender: mpsc::SyncSender<Result<Vec<FileStamp>, String>>,
    receiver: mpsc::Receiver<Result<Vec<FileStamp>, String>>,
    previous: Option<Vec<FileStamp>>,
    in_flight: bool,
    next_probe_at: Instant,
}

impl OwnedFileChangeWatcher {
    /// 🌱️ Watches the containing directory so create, replace, rename, and delete are observable.
    pub fn new(watch_path: &Path, pool: Arc<WorkerPool>) -> Self {
        let root = watch_path.parent().map_or_else(|| watch_path.to_path_buf(), Path::to_path_buf);
        let (sender, receiver) = mpsc::sync_channel(1);
        Self { pool, root, cancelled: Arc::new(AtomicBool::new(false)), sender, receiver, previous: None, in_flight: false, next_probe_at: Instant::now() }
    }

    /// 🔎 Polls completed probes without waiting and schedules the next bounded probe when due.
    pub fn poll_changed(&mut self) -> bool {
        let mut changed = false;
        while let Ok(result) = self.receiver.try_recv() {
            self.in_flight = false;
            if let Ok(snapshot) = result {
                changed |= self.previous.as_ref().is_some_and(|previous| previous != &snapshot);
                self.previous = Some(snapshot);
            }
        }
        let now = Instant::now();
        if !self.in_flight && now >= self.next_probe_at {
            self.in_flight = true;
            self.next_probe_at = now + Duration::from_millis(20);
            submit_probe_step(&self.pool, self.root.clone(), self.cancelled.clone(), self.sender.clone(), None);
        }
        changed
    }
}

impl Drop for OwnedFileChangeWatcher {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

//#endregion 👁️Watcher

#[cfg(test)]
mod tests {
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
            let mut watcher = OwnedFileChangeWatcher::new(&root.join("document.db"), pool);
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
}
