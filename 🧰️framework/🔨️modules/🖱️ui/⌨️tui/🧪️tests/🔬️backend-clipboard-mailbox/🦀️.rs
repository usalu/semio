mod clipboard_mailbox_tests {
    use super::*;

    #[test]
    fn stalled_clipboard_worker_keeps_poll_callback_p99_below_two_ms() {
        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 1)));
        let mut clipboard = HostClipboard::new(std::sync::Arc::clone(&pool));
        clipboard.submit(|| {
            std::thread::sleep(Duration::from_millis(80));
            ClipboardResult::Copied
        });
        let mut samples = Vec::with_capacity(4096);
        for _ in 0..4096 {
            let started = std::time::Instant::now();
            let _ = clipboard.poll();
            samples.push(started.elapsed());
        }
        samples.sort_unstable();
        let p99 = samples[samples.len() * 99 / 100];
        assert!(p99 < Duration::from_millis(2), "clipboard poll callback p99 was {p99:?}");
        pool.shutdown();
    }
}
