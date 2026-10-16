//! 📦️ `pack_http` — HTTP range-request access layer for `pack` files: a `RangeTransport`
//! injection seam so no concrete HTTP client type appears in any public signature, an
//! `HttpPackSource` implementing `crate::async_::AsyncPackSource` with retry+backoff and
//! etag-based revalidation. An optional `ureq`
//! feature (off by default) provides a native `UreqRangeTransport`.

// 🔓️ R7: `async fn` in a public trait is our deliberate crate-wide shape (R3 answers the lint's underlying `Send` concern structurally).
// Never take rustc's suggested `-> impl Future<...> + Send` fix; it re-imposes `Send` R3 forbids.
#![allow(async_fn_in_trait)]

//#region 🔖️Transport
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use semio_framework_pack_async::{AsyncPackSource, CancellationToken, LoadPriority, ReadRequest as SchedulerRead, ReadScheduler};
use semio_framework_replication_codec::ByteRange;
use semio_framework_pack_error::{PackError,PackRetryDisposition};
use semio_framework_async::WorkerPool;

/// 📨️ One range-request against `url`, optionally revalidated against a previously seen
/// etag via `if_range_etag`.
pub struct RangeRequest {
    pub url: String,
    pub range: ByteRange,
    pub if_range_etag: Option<String>,
}

/// 📬️ The bytes a `RangeTransport` fetched for a `RangeRequest`, plus the metadata needed
/// for retry/revalidation decisions upstream.
pub struct RangeResponse {
    pub bytes: Vec<u8>,
    pub etag: Option<String>,
    pub total_len: Option<u64>,
    pub range_satisfied: bool,
}

/// 🔌️ The injection seam: no concrete HTTP client type may appear in any public
/// signature outside an implementor of this trait. Browser `fetch`, native `ureq`, or a test
/// double all implement this identically.
// 🚪️ R8: plain AFIT — single genuinely-`async fn` method, zero `dyn RangeTransport` anywhere in
// the repo (verified repo-wide across `🧰️framework`, `🛍️products`, `✏️s`), so no dyn-erasure
// concern; `#[async_trait]` bought nothing here but a banned `Pin<Box<dyn Future>>` desugar.
pub trait RangeTransport: Send + Sync {
    async fn fetch_range(&self, request: RangeRequest) -> Result<RangeResponse, PackError>;
}
//#endregion 🔖️Transport

//#region 🔖️Source
/// ♻️ Retry/backoff tuning for `HttpPackSource`; transient transport failures are
/// retried up to `max_retries` times with exponentially growing delay starting at
/// `initial_backoff`, capped at `max_backoff`.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

// 🚫️async: E1 impl of the externally-declared `Default` trait; `default`'s signature is fixed
// by `std::default::Default` and must stay sync — no `.await`-able work here regardless.
impl Default for RetryPolicy {
    fn default() -> Self {
        Self { max_retries: 3, initial_backoff: Duration::from_millis(50), max_backoff: Duration::from_secs(2) }
    }
}

/// ⏱️ The process runtime used for retry deadlines. The clock must use the same
/// millisecond epoch supplied to `WorkerPool::pump` on wasm; native callers can use `native`.
#[derive(Clone)]
pub struct RetryRuntime {
    pool: Arc<WorkerPool>,
    now_ms: Arc<dyn Fn() -> u64 + Send + Sync>,
}

impl RetryRuntime {
    /// 🧩️ Injects the process pool and its matching monotonic clock.
    pub fn new(pool: Arc<WorkerPool>, now_ms: Arc<dyn Fn() -> u64 + Send + Sync>) -> Self {
        Self { pool, now_ms }
    }

    /// 🖥️ Binds retry deadlines to a native pool's monotonic clock.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn native(pool: Arc<WorkerPool>) -> Self {
        let clock_pool = pool.clone();
        Self::new(pool, Arc::new(move || clock_pool.now_ms()))
    }

    async fn sleep(&self, duration: Duration) {
        let delay_ms = u64::try_from(duration.as_millis()).unwrap_or(u64::MAX);
        let deadline_ms = (self.now_ms)().saturating_add(delay_ms);
        self.pool.timer().sleep_until(deadline_ms).await;
    }
}

/// 🚦️ `u64::MAX` sentinel meaning "length not yet observed" for `SharedState::known_len`,
/// since `AtomicU64` has no built-in `Option`.
const LEN_UNKNOWN: u64 = u64::MAX;

/// 🗄️ State shared between the public `HttpPackSource` (which must answer
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
    // 🚫️async: no suspension point — construction only; kept sync so it can be called directly
    // from `HttpPackSource::with_retry_policy` below, itself sync for the same reason that
    // function is tagged.
    fn new() -> Self {
        Self { last_etag: Mutex::new(None), known_len: AtomicU64::new(LEN_UNKNOWN) }
    }

    /// 📏️ The best length known so far, or `0` if nothing has been observed yet — mirrors
    /// `AsyncPackSource::len`'s infallible-`u64` contract.
    // 🚫️async: no suspension point — `AsyncPackSource::len` (see `⏳️async/🦀️.rs`) is
    // deliberately sync by contract, so every path reaching it must stay sync too.
    fn len(&self) -> u64 {
        match self.known_len.load(Ordering::SeqCst) {
            LEN_UNKNOWN => 0,
            value => value,
        }
    }
}

/// 🔧️ The `AsyncPackSource` actually wrapped by `HttpPackSource`'s internal
/// `ReadScheduler`: owns the transport and retry policy, performs one logical range fetch per
/// `read_at` (revalidating against `shared`'s last-seen etag, retrying transient failures with
/// backoff), and updates `shared` from every response it observes.
struct InnerSource<T: RangeTransport> {
    url: String,
    transport: T,
    retry_policy: RetryPolicy,
    shared: Arc<SharedState>,
    runtime: RetryRuntime,
}

impl<T: RangeTransport> InnerSource<T> {
    /// 🔁️ Only the provider's explicit transient disposition authorizes a retry.
    // 🚫️async: the only call site is the `match` guard in `fetch_with_retry` below
    // (`Err(error) if Self::is_transient(&error) && ...`) — `.await` is not permitted inside a
    // `match` guard, a hard syntactic restriction independent of this fn's own design.
    fn is_transient(error: &PackError) -> bool {
        match error{PackError::Refusal(semio_framework_pack_error::PackRefusal::Io{retry,..})=>*retry==PackRetryDisposition::Transient,PackError::TransportFailure(error)=>error.retry()==PackRetryDisposition::Transient,_=>false}
    }

    /// ⏱️ The backoff delay before retry attempt `attempt` (0-indexed), doubling from
    /// `initial_backoff` and capped at `max_backoff`.
    async fn backoff_for(&self, attempt: u32) -> Duration {
        let scale = 1u64.checked_shl(attempt).unwrap_or(u64::MAX);
        let millis = self.retry_policy.initial_backoff.as_millis() as u64;
        let delay = millis.saturating_mul(scale);
        Duration::from_millis(delay).min(self.retry_policy.max_backoff)
    }

    /// 📡️ Performs one logical range fetch: revalidates with the cached etag (if any),
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
                    let delay = self.backoff_for(attempt).await;
                    if delay > Duration::ZERO {
                        self.runtime.sleep(delay).await;
                    }
                    attempt += 1;
                }
                Err(error) => return Err(error),
            }
        }
    }
}

impl<T: RangeTransport> AsyncPackSource for InnerSource<T> {
    // 🚫️async: `AsyncPackSource::len` is declared sync by contract (see
    // `⏳️async/🦀️.rs`); this impl just mirrors that.
    fn len(&self) -> u64 {
        self.shared.len()
    }

    async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, PackError> {
        let range = ByteRange { offset, len: len as u64 };
        let response = self.fetch_with_retry(range).await?;
        if response.bytes.len() < len {
            return Err(PackError::Refusal(semio_framework_pack_error::PackRefusal::Truncated(offset + len as u64)));
        }
        Ok(response.bytes[..len].to_vec())
    }
}

/// 🌐️ An `AsyncPackSource` backed by HTTP range requests through an injected
/// `RangeTransport`. Caches the last-seen `etag` and revalidates on every subsequent fetch;
/// retries transient transport failures with exponential backoff; coalesces/dedups concurrent
/// overlapping reads through an internal `crate::async_::ReadScheduler` wrapping an `InnerSource`.
pub struct HttpPackSource<T: RangeTransport> {
    shared: Arc<SharedState>,
    scheduler: ReadScheduler<InnerSource<T>>,
}

impl<T: RangeTransport> HttpPackSource<T> {
    /// 🆕️ A source fetching `url` through `transport`, with default retry policy and no
    /// known length or etag yet.
    // 🚫️async: no suspension point — construction only; kept sync so existing plain-sync test
    // call sites (`let source = HttpPackSource::new(...);`, unawaited) keep compiling.
    pub fn new(url: String, transport: T, runtime: RetryRuntime) -> Self {
        Self::with_retry_policy(url, transport, RetryPolicy::default(), runtime)
    }

    /// 🆕️ As `new`, but with an explicit `RetryPolicy`.
    // 🚫️async: no suspension point — same constructor reasoning as `new` above.
    pub fn with_retry_policy(url: String, transport: T, retry_policy: RetryPolicy, runtime: RetryRuntime) -> Self {
        let shared = Arc::new(SharedState::new());
        let inner = InnerSource { url, transport, retry_policy, shared: shared.clone(), runtime };
        Self { shared, scheduler: ReadScheduler::new(inner) }
    }
}

impl<T: RangeTransport> AsyncPackSource for HttpPackSource<T> {
    // 🚫️async: `AsyncPackSource::len` is declared sync by contract (see
    // `⏳️async/🦀️.rs`); this impl just mirrors that.
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
//#endregion 🔖️Ureq

//#region 🧪️Tests
// 🚫️async: every `semio_framework_async::block_on(...)` in this module drives one `#[test] fn` —
// each is its own synchronous test-harness entry point, the same role `fn main` plays for a
// binary (R4 item 1), so none are converted to `.await` here. See
// `📓️terra-pack-waker-report.md` for the full site-by-site census.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🧭️producer-authority/🦀️.rs"]
mod producer_authority_tests;
