
use super::*;
use semio_framework_async::{ProcessKind, WorkerPoolConfig};

#[test]
fn retained_future_progresses_without_adding_threads() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 2));
    let workers = pool.worker_count();
    let (tx, rx) = std::sync::mpsc::channel();
    let _task = ShellPoolFuture::spawn(pool.clone(), Lane::Io, async move {
        semio_framework_async::yield_once().await;
        let _ = tx.send(7);
    });
    assert_eq!(rx.recv_timeout(std::time::Duration::from_secs(1)), Ok(7));
    assert_eq!(pool.worker_count(), workers);
    pool.shutdown().expect("fixture worker pool shuts down");
}

#[test]
fn cancellation_drops_a_sleeping_future_before_shutdown() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 2));
    let sleep_pool = pool.clone();
    let task = ShellPoolFuture::spawn(pool.clone(), Lane::Timer, async move {
        sleep_pool.timer().sleep_until(sleep_pool.now_ms().saturating_add(60_000)).await;
    });
    task.cancel();
    let started = std::time::Instant::now();
    pool.shutdown().expect("fixture worker pool shuts down");
    assert!(started.elapsed() < std::time::Duration::from_secs(1));
}
