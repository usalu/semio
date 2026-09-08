
use super::*;
use std::sync::atomic::AtomicU32;

fn retry_runtime() -> (RetryRuntime, Arc<WorkerPool>) {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    (RetryRuntime::native(pool.clone()), pool)
}

//#region 🔖️Transport
/// @emoji 🧪️ An in-memory `RangeTransport` test double: serves slices of `data`, can be
/// scripted to fail transiently N times before succeeding, and records every request it
/// received for assertions. `Clone`-able (all state lives behind `Arc`) so a test can hand
/// one clone to `HttpPackSource` (which consumes it by value) while keeping another clone
/// around to inspect afterward.
#[derive(Clone)]
struct FakeTransport {
    data: Arc<Vec<u8>>,
    etag: Arc<str>,
    fail_first_n: Arc<AtomicU32>,
    call_count: Arc<AtomicU32>,
    requests_seen: Arc<Mutex<Vec<RangeRequest>>>,
}

impl FakeTransport {
    fn new(data: Vec<u8>, etag: &str) -> Self {
        Self { data: Arc::new(data), etag: Arc::from(etag), fail_first_n: Arc::new(AtomicU32::new(0)), call_count: Arc::new(AtomicU32::new(0)), requests_seen: Arc::new(Mutex::new(Vec::new())) }
    }

    fn failing_first(self, n: u32) -> Self {
        self.fail_first_n.store(n, Ordering::SeqCst);
        self
    }
}

impl RangeTransport for FakeTransport {
    async fn fetch_range(&self, request: RangeRequest) -> Result<RangeResponse, PackError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        self.requests_seen.lock().unwrap().push(RangeRequest { url: request.url.clone(), range: request.range, if_range_etag: request.if_range_etag.clone() });

        let remaining = self.fail_first_n.load(Ordering::SeqCst);
        if remaining > 0 {
            self.fail_first_n.fetch_sub(1, Ordering::SeqCst);
            return Err(PackError::Io("simulated transient failure".to_string()));
        }

        let start = request.range.offset as usize;
        let end = (start + request.range.len as usize).min(self.data.len());
        let bytes = self.data[start..end].to_vec();
        let range_satisfied = end - start == request.range.len as usize;
        Ok(RangeResponse { bytes, etag: Some(self.etag.to_string()), total_len: Some(self.data.len() as u64), range_satisfied })
    }
}

/// 🧪️ Retry waits use the process TimerWheel and make forward progress without
/// creating a per-wait OS thread.
#[test]
fn retry_runtime_sleep_resolves_via_worker_pool_timer_wheel() {
    let (runtime, pool) = retry_runtime();
    semio_framework_async::block_on(runtime.sleep(Duration::from_millis(15)));
    assert_eq!(pool.worker_count(), 2);
    pool.shutdown();
}

#[test]
fn successful_range_fetch_returns_exact_slice() {
    let data: Vec<u8> = (0..64u16).map(|value| value as u8).collect();
    let transport = FakeTransport::new(data.clone(), "etag-1");
    let (runtime, pool) = retry_runtime();
    let source = HttpPackSource::new("https://example.test/doc.pack".to_string(), transport, runtime);

    let bytes = semio_framework_async::block_on(source.read_at(10, 20)).unwrap();
    assert_eq!(bytes, data[10..30].to_vec());
    assert_eq!(source.len(), 64);
    pool.shutdown();
}

#[test]
fn etag_is_forwarded_as_if_range_on_the_next_fetch_for_revalidation() {
    let data: Vec<u8> = (0..32u16).map(|value| value as u8).collect();
    let transport = FakeTransport::new(data, "etag-abc");
    let inspector = transport.clone();
    let (runtime, pool) = retry_runtime();
    let source = HttpPackSource::new("https://example.test/doc.pack".to_string(), transport, runtime);

    // Sequential (not concurrent) reads: each completes and its coalescing group is
    // finalized before the next begins, so these must land as two separate physical fetches
    // rather than being coalesced by the internal `ReadScheduler`.
    semio_framework_async::block_on(source.read_at(0, 8)).unwrap();
    semio_framework_async::block_on(source.read_at(8, 8)).unwrap();

    let seen = inspector.requests_seen.lock().unwrap();
    assert_eq!(seen.len(), 2, "expected exactly two physical fetches, one per read_at call");
    assert_eq!(seen[0].if_range_etag, None, "the first request has no prior etag to revalidate against");
    assert_eq!(seen[1].if_range_etag.as_deref(), Some("etag-abc"), "the second request must carry the etag observed from the first response for revalidation");
    drop(seen);
    pool.shutdown();
}

#[test]
fn transient_failure_is_retried_and_eventually_succeeds() {
    let data: Vec<u8> = (0..16u16).map(|value| value as u8).collect();
    let transport = FakeTransport::new(data.clone(), "etag-r").failing_first(2);
    let inspector = transport.clone();
    let (runtime, pool) = retry_runtime();
    let source = HttpPackSource::with_retry_policy("https://example.test/doc.pack".to_string(), transport, RetryPolicy { max_retries: 5, initial_backoff: Duration::from_millis(1), max_backoff: Duration::from_millis(20) }, runtime);

    let bytes = semio_framework_async::block_on(source.read_at(0, 16)).unwrap();
    assert_eq!(bytes, data);
    assert_eq!(inspector.call_count.load(Ordering::SeqCst), 3, "two failures then one success = three calls");
    pool.shutdown();
}

#[test]
fn exhausting_retries_surfaces_the_transient_error() {
    let transport = FakeTransport::new(vec![0u8; 8], "etag-x").failing_first(10);
    let inspector = transport.clone();
    let (runtime, pool) = retry_runtime();
    let source = HttpPackSource::with_retry_policy("https://example.test/doc.pack".to_string(), transport, RetryPolicy { max_retries: 2, initial_backoff: Duration::from_millis(1), max_backoff: Duration::from_millis(5) }, runtime);

    let result = semio_framework_async::block_on(source.read_at(0, 8));
    assert!(result.is_err());
    assert_eq!(inspector.call_count.load(Ordering::SeqCst), 3, "initial attempt + 2 retries = three calls");
    pool.shutdown();
}
//#endregion 🔖️Transport
