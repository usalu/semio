//! ⏱️ Law guards: fresh temp directories, the socket test runner thread and the hang watchdogs with their bounded steps.

use super::*;

/// 📁️ A fresh, never-reused temp directory per call — the owned `time_ordered_id` rather than
/// `now_ms()` alone, since `cargo test` runs this whole module's `#[tokio::test]`s
/// concurrently within one process: two tests calling `test_state()` in the same millisecond
/// would otherwise collide on the identical `os-hub-test-db-<pid>-<ms>` path and open the SAME
/// `db::Database` storage root, corrupting each other's catalog/WAL state.
pub fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = semio_hub_testing::test_artifact_root();
    dir.push(format!("os-hub-test-{name}-{}", directory::os_identity::time_ordered_id()));
    std::fs::create_dir_all(&dir).expect("owned temporary fixture directory");
    dir
}

pub fn run_socket_test<F, Fut>(test: F)
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + 'static,
{
    std::thread::Builder::new()
        .name("hub-socket-test".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || tokio::runtime::Builder::new_current_thread().enable_all().build().expect("test runtime").block_on(test()))
        .expect("socket test thread")
        .join()
        .expect("socket test");
}

/// ⏳️ The single wall-clock bound every in-process rendezvous in this suite answers to — a socket
/// close, a live-gate permit, a server task's reply. It is a hang guard, never a measurement: a law
/// that waits on a signal the product genuinely never sends must fail here, fast and locally, instead
/// of being killed 300 s later by nextest with no site. It replaces the per-call 2 s and 5 s bounds
/// that fired on fleet scheduling latency alone.
pub const SOCKET_RENDEZVOUS_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(15);

/// ⏳️ The bound a law's whole body answers to when the runtime itself may be the thing that wedged.
/// `tokio::time::timeout` cannot fire on a starved executor — a current-thread runtime whose single
/// task blocks, or spins without yielding, never advances the timer wheel — so a law that hangs that
/// way is killed 300 s later by nextest with no site at all. `LawHangWatchdogV1` is a plain OS thread
/// that owns a wall clock nothing in the runtime can starve.
const LAW_BODY_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(25);

/// ⏱️ An out-of-runtime wall-clock watchdog: armed at the top of a law, disarmed by its own `Drop`
/// when the law returns. On expiry it names the law on stderr and aborts the process, so nextest
/// reports that law — and only that law — as failed within seconds instead of after the 300 s
/// per-test kill. It is a hang guard, never a measurement: a law that legitimately takes this long
/// does not belong in the shared bin binary.
pub struct LawHangWatchdogV1 {
    finished: Arc<std::sync::atomic::AtomicBool>,
    pub phase: Arc<std::sync::Mutex<&'static str>>,
}

impl LawHangWatchdogV1 {
    pub fn arm(law: &'static str) -> Self {
        let finished = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let phase: Arc<std::sync::Mutex<&'static str>> = Arc::new(std::sync::Mutex::new("armed, before the first step"));
        let watched = finished.clone();
        let observed = phase.clone();
        std::thread::Builder::new()
            .name("law-hang-watchdog".into())
            .spawn(move || {
                let deadline = std::time::Instant::now() + LAW_BODY_HANG_GUARD;
                while std::time::Instant::now() < deadline {
                    if watched.load(std::sync::atomic::Ordering::Acquire) {
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                if !watched.load(std::sync::atomic::Ordering::Acquire) {
                    let stuck = *observed.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    eprintln!("law hang watchdog: {law} did not finish within {LAW_BODY_HANG_GUARD:?} — the runtime is wedged, not slow; last phase entered: {stuck}");
                    std::process::abort();
                }
            })
            .expect("law hang watchdog thread");
        Self { finished, phase }
    }

    pub fn at(&self, phase: &'static str) {
        *self.phase.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = phase;
    }
}

impl Drop for LawHangWatchdogV1 {
    fn drop(&mut self) {
        self.finished.store(true, std::sync::atomic::Ordering::Release);
    }
}

/// ⏳️ One awaited fixture step, bounded and named, so a law that wedges says WHICH await never
/// returned instead of naming only itself. It complements `LawHangWatchdogV1`: this one fires when
/// the future genuinely never resolves, that one when the executor can no longer run timers at all.
pub async fn bounded_law_step<T>(step: &str, future: impl std::future::Future<Output = T>) -> T {
    match tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, future).await {
        Ok(value) => value,
        Err(_) => panic!("{step} did not complete within {SOCKET_RENDEZVOUS_HANG_GUARD:?}"),
    }
}

/// ⏳️ The single wall-clock bound every HTTP read in this suite answers to. It exists only to stop a
/// route that never answers from hanging the shared bin binary; it is never the thing a law measures.
/// The transport used to carry a second, shorter bound of its own, which silently pre-empted this one
/// and turned fleet scheduling latency into red laws that had nothing to do with the hub.
pub const RAW_HTTP_READ_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(20);
