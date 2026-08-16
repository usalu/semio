//! 📦️ `pack_http` — HTTP range-request access layer for `pack` files: a `RangeTransport`
//! injection seam so no concrete HTTP client type appears in any public signature, an
//! `HttpPackSource` implementing `crate::os_pack::async_::AsyncPackSource` with retry+backoff and
//! etag-based revalidation. An optional `ureq`
//! feature (off by default) provides a native `UreqRangeTransport`.

//#region 🔖️Transport
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::os_pack::async_::{AsyncPackSource, CancellationToken, LoadPriority, ReadRequest as SchedulerRead, ReadScheduler};
use crate::os_pack::{ByteRange, PackError};

/// @emoji 📨️ One range-request against `url`, optionally revalidated against a previously seen
/// etag via `if_range_etag`.
pub struct RangeRequest {
    pub url: String,
    pub range: ByteRange,
    pub if_range_etag: Option<String>,
}

/// @emoji 📬️ The bytes a `RangeTransport` fetched for a `RangeRequest`, plus the metadata needed
/// for retry/revalidation decisions upstream.
pub struct RangeResponse {
    pub bytes: Vec<u8>,
    pub etag: Option<String>,
    pub total_len: Option<u64>,
    pub range_satisfied: bool,
}

/// @emoji 🔌️ The injection seam: no concrete HTTP client type may appear in any public
/// signature outside an implementor of this trait. Browser `fetch`, native `ureq`, or a test
/// double all implement this identically.
#[async_trait::async_trait]
pub trait RangeTransport: Send + Sync {
    async fn fetch_range(&self, request: RangeRequest) -> Result<RangeResponse, PackError>;
}
//#endregion 🔖️Transport

//#region 🔖️Source
/// @emoji ♻️ Retry/backoff tuning for `HttpPackSource`; transient transport failures are
/// retried up to `max_retries` times with exponentially growing delay starting at
/// `initial_backoff`, capped at `max_backoff`.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self { max_retries: 3, initial_backoff: Duration::from_millis(50), max_backoff: Duration::from_secs(2) }
    }
}

/// @emoji 🚦️ `u64::MAX` sentinel meaning "length not yet observed" for `SharedState::known_len`,
/// since `AtomicU64` has no built-in `Option`.
const LEN_UNKNOWN: u64 = u64::MAX;

/// @emoji 🗄️ State shared between the public `HttpPackSource` (which must answer
/// `AsyncPackSource::len` synchronously) and the private `InnerSource` living inside its
/// internal `ReadScheduler` (which performs the actual retrying, etag-revalidating physical
/// fetches and is the only thing that ever observes a fresh etag/length from the transport).
/// `HttpPackSource` and `InnerSource` each hold an `Arc` clone of the same `SharedState`, since
/// `ReadScheduler` does not expose its wrapped source back out.
struct SharedState {
    last_etag: Mutex<Option<String>>,
    known_len: AtomicU64,
}

impl SharedState {
    fn new() -> Self {
        Self { last_etag: Mutex::new(None), known_len: AtomicU64::new(LEN_UNKNOWN) }
    }

    /// @emoji 📏️ The best length known so far, or `0` if nothing has been observed yet — mirrors
    /// `AsyncPackSource::len`'s infallible-`u64` contract.
    fn len(&self) -> u64 {
        match self.known_len.load(Ordering::SeqCst) {
            LEN_UNKNOWN => 0,
            value => value,
        }
    }
}

/// @emoji 🔧️ The `AsyncPackSource` actually wrapped by `HttpPackSource`'s internal
/// `ReadScheduler`: owns the transport and retry policy, performs one logical range fetch per
/// `read_at` (revalidating against `shared`'s last-seen etag, retrying transient failures with
/// backoff), and updates `shared` from every response it observes.
struct InnerSource<T: RangeTransport> {
    url: String,
    transport: T,
    retry_policy: RetryPolicy,
    shared: Arc<SharedState>,
}

impl<T: RangeTransport> InnerSource<T> {
    /// @emoji 🔁️ True iff `error` represents a transient condition worth retrying (currently:
    /// any `PackError::Io`, which is how transport failures are surfaced across the trait
    /// boundary).
    fn is_transient(error: &PackError) -> bool {
        matches!(error, PackError::Io(_))
    }

    /// @emoji ⏱️ The backoff delay before retry attempt `attempt` (0-indexed), doubling from
    /// `initial_backoff` and capped at `max_backoff`.
    fn backoff_for(&self, attempt: u32) -> Duration {
        let scale = 1u64.checked_shl(attempt).unwrap_or(u64::MAX);
        let millis = self.retry_policy.initial_backoff.as_millis() as u64;
        let delay = millis.saturating_mul(scale);
        Duration::from_millis(delay).min(self.retry_policy.max_backoff)
    }

    /// @emoji 📡️ Performs one logical range fetch: revalidates with the cached etag (if any),
    /// retries transient failures per `retry_policy`, and remembers the response's etag/length
    /// in `shared` for next time.
    async fn fetch_with_retry(&self, range: ByteRange) -> Result<RangeResponse, PackError> {
        let if_range_etag = self.shared.last_etag.lock().unwrap().clone();
        let mut attempt = 0u32;
        loop {
            let request = RangeRequest { url: self.url.clone(), range, if_range_etag: if_range_etag.clone() };
            match self.transport.fetch_range(request).await {
                Ok(response) => {
                    if let Some(etag) = &response.etag {
                        *self.shared.last_etag.lock().unwrap() = Some(etag.clone());
                    }
                    if let Some(total_len) = response.total_len {
                        self.shared.known_len.store(total_len, Ordering::SeqCst);
                    }
                    return Ok(response);
                }
                Err(error) if Self::is_transient(&error) && attempt < self.retry_policy.max_retries => {
                    let delay = self.backoff_for(attempt);
                    if delay > Duration::ZERO {
                        sleep(delay).await;
                    }
                    attempt += 1;
                }
                Err(error) => return Err(error),
            }
        }
    }
}

/// @emoji 💤️ A tiny runtime-neutral async sleep (no `tokio` dependency): spin-parks the current
/// task with a short thread sleep between polls, mirroring `pack_async`'s own cancellation
/// watcher pattern.
async fn sleep(duration: Duration) {
    struct Sleep {
        deadline: std::time::Instant,
    }
    impl std::future::Future for Sleep {
        type Output = ();
        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
            if std::time::Instant::now() >= self.deadline {
                std::task::Poll::Ready(())
            } else {
                cx.waker().wake_by_ref();
                std::thread::sleep(Duration::from_micros(200));
                std::task::Poll::Pending
            }
        }
    }
    Sleep { deadline: std::time::Instant::now() + duration }.await;
}

#[async_trait::async_trait]
impl<T: RangeTransport> AsyncPackSource for InnerSource<T> {
    fn len(&self) -> u64 {
        self.shared.len()
    }

    async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, PackError> {
        let range = ByteRange { offset, len: len as u64 };
        let response = self.fetch_with_retry(range).await?;
        if response.bytes.len() < len {
            return Err(PackError::Truncated(offset + len as u64));
        }
        Ok(response.bytes[..len].to_vec())
    }
}

/// @emoji 🌐️ An `AsyncPackSource` backed by HTTP range requests through an injected
/// `RangeTransport`. Caches the last-seen `etag` and revalidates on every subsequent fetch;
/// retries transient transport failures with exponential backoff; coalesces/dedups concurrent
/// overlapping reads through an internal `crate::os_pack::async_::ReadScheduler` wrapping an `InnerSource`.
pub struct HttpPackSource<T: RangeTransport> {
    shared: Arc<SharedState>,
    scheduler: ReadScheduler<InnerSource<T>>,
}

impl<T: RangeTransport> HttpPackSource<T> {
    /// @emoji 🆕️ A source fetching `url` through `transport`, with default retry policy and no
    /// known length or etag yet.
    pub fn new(url: String, transport: T) -> Self {
        Self::with_retry_policy(url, transport, RetryPolicy::default())
    }

    /// @emoji 🆕️ As `new`, but with an explicit `RetryPolicy`.
    pub fn with_retry_policy(url: String, transport: T, retry_policy: RetryPolicy) -> Self {
        let shared = Arc::new(SharedState::new());
        let inner = InnerSource { url, transport, retry_policy, shared: shared.clone() };
        Self { shared, scheduler: ReadScheduler::new(inner) }
    }
}

#[async_trait::async_trait]
impl<T: RangeTransport> AsyncPackSource for HttpPackSource<T> {
    fn len(&self) -> u64 {
        self.shared.len()
    }

    async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, PackError> {
        let cancel = CancellationToken::new();
        let request = SchedulerRead { range: ByteRange { offset, len: len as u64 }, priority: LoadPriority::Requested };
        self.scheduler.read(request, &cancel).await
    }
}
//#endregion 🔖️Source

//#region 🔖️Ureq
#[cfg(feature = "ureq")]
mod ureq_transport {
    //! @emoji 🚚️ Native `RangeTransport` impl over the blocking `ureq` HTTP client, gated
    //! behind the `ureq` feature so wasm/browser builds of the facade stay lean.
    use super::{RangeRequest, RangeResponse, RangeTransport};
    use crate::os_pack::PackError;
    use std::io::Read;

    /// @emoji 🐎️ A `RangeTransport` backed by `ureq`, issuing a single blocking HTTP `Range`
    /// request per `fetch_range` call on a dedicated thread (so the `async fn` never blocks the
    /// caller's executor).
    pub struct UreqRangeTransport {
        agent: ureq::Agent,
    }

    impl UreqRangeTransport {
        /// @emoji 🆕️ A transport using `ureq`'s default agent configuration.
        pub fn new() -> Self {
            Self { agent: ureq::Agent::new() }
        }
    }

    impl Default for UreqRangeTransport {
        fn default() -> Self {
            Self::new()
        }
    }

    #[async_trait::async_trait]
    impl RangeTransport for UreqRangeTransport {
        async fn fetch_range(&self, request: RangeRequest) -> Result<RangeResponse, PackError> {
            let agent = self.agent.clone();
            let end_inclusive = request.range.offset + request.range.len.saturating_sub(1);
            let range_header = format!("bytes={}-{}", request.range.offset, end_inclusive);

            std::thread::spawn(move || -> Result<RangeResponse, PackError> {
                let mut call = agent.get(&request.url).set("Range", &range_header);
                if let Some(etag) = &request.if_range_etag {
                    call = call.set("If-Range", etag);
                }
                let response = call.call().map_err(|error| PackError::Io(error.to_string()))?;
                let range_satisfied = response.status() == 206;
                let etag = response.header("ETag").map(|value| value.to_string());
                let total_len = response.header("Content-Range").and_then(|value| value.rsplit('/').next()).and_then(|value| value.parse::<u64>().ok());
                let mut bytes = Vec::new();
                response.into_reader().read_to_end(&mut bytes).map_err(|error| PackError::Io(error.to_string()))?;
                Ok(RangeResponse { bytes, etag, total_len, range_satisfied })
            })
            .join()
            .map_err(|_| PackError::Io("ureq worker thread panicked".to_string()))?
        }
    }
}
#[cfg(feature = "ureq")]
pub use ureq_transport::UreqRangeTransport;
//#endregion 🔖️Ureq

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

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

    #[async_trait::async_trait]
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

    #[test]
    fn successful_range_fetch_returns_exact_slice() {
        let data: Vec<u8> = (0..64u16).map(|value| value as u8).collect();
        let transport = FakeTransport::new(data.clone(), "etag-1");
        let source = HttpPackSource::new("https://example.test/doc.pack".to_string(), transport);

        let bytes = futures_lite::future::block_on(source.read_at(10, 20)).unwrap();
        assert_eq!(bytes, data[10..30].to_vec());
        assert_eq!(source.len(), 64);
    }

    #[test]
    fn etag_is_forwarded_as_if_range_on_the_next_fetch_for_revalidation() {
        let data: Vec<u8> = (0..32u16).map(|value| value as u8).collect();
        let transport = FakeTransport::new(data, "etag-abc");
        let inspector = transport.clone();
        let source = HttpPackSource::new("https://example.test/doc.pack".to_string(), transport);

        // Sequential (not concurrent) reads: each completes and its coalescing group is
        // finalized before the next begins, so these must land as two separate physical fetches
        // rather than being coalesced by the internal `ReadScheduler`.
        futures_lite::future::block_on(source.read_at(0, 8)).unwrap();
        futures_lite::future::block_on(source.read_at(8, 8)).unwrap();

        let seen = inspector.requests_seen.lock().unwrap();
        assert_eq!(seen.len(), 2, "expected exactly two physical fetches, one per read_at call");
        assert_eq!(seen[0].if_range_etag, None, "the first request has no prior etag to revalidate against");
        assert_eq!(seen[1].if_range_etag.as_deref(), Some("etag-abc"), "the second request must carry the etag observed from the first response for revalidation");
    }

    #[test]
    fn transient_failure_is_retried_and_eventually_succeeds() {
        let data: Vec<u8> = (0..16u16).map(|value| value as u8).collect();
        let transport = FakeTransport::new(data.clone(), "etag-r").failing_first(2);
        let inspector = transport.clone();
        let source = HttpPackSource::with_retry_policy("https://example.test/doc.pack".to_string(), transport, RetryPolicy { max_retries: 5, initial_backoff: Duration::from_millis(1), max_backoff: Duration::from_millis(20) });

        let bytes = futures_lite::future::block_on(source.read_at(0, 16)).unwrap();
        assert_eq!(bytes, data);
        assert_eq!(inspector.call_count.load(Ordering::SeqCst), 3, "two failures then one success = three calls");
    }

    #[test]
    fn exhausting_retries_surfaces_the_transient_error() {
        let transport = FakeTransport::new(vec![0u8; 8], "etag-x").failing_first(10);
        let inspector = transport.clone();
        let source = HttpPackSource::with_retry_policy("https://example.test/doc.pack".to_string(), transport, RetryPolicy { max_retries: 2, initial_backoff: Duration::from_millis(1), max_backoff: Duration::from_millis(5) });

        let result = futures_lite::future::block_on(source.read_at(0, 8));
        assert!(result.is_err());
        assert_eq!(inspector.call_count.load(Ordering::SeqCst), 3, "initial attempt + 2 retries = three calls");
    }
    //#endregion 🔖️Transport

}
//#endregion 🧪️Tests
