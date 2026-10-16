//! 🪣️ The HTTP pool: the per-package byte bucket, the per-actor outstanding credit, the refill driver and the pooled response body.

use semio_framework_actor::{ActorId, PackageId};
use semio_framework_async::{HostAsyncRuntime, Lane, OperationContext, ScopeHandle, WorkerPool};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use super::compute_pool::ComputePool;
use super::http_contract::{
    AsyncHttpTransport,
    BufferedHttpBody,
    HttpBody,
    HttpBodyCancellationHandle,
    HttpPoolError,
    HttpRequest,
    HttpResponse,
    HttpResponseHead,
    HttpTransport,
    HttpTransportTerminalHandle,
};

//#region 🌐️HttpPool
#[doc(hidden)] pub struct TokenBucket {
    remaining_bytes: u64,
    pub(super) capacity_bytes: u64,
    refill_epoch: u64,
}

impl TokenBucket {
    #[doc(hidden)] pub fn new_at(capacity_bytes: u64, refill_epoch: u64) -> TokenBucket {
        TokenBucket { remaining_bytes: capacity_bytes, capacity_bytes, refill_epoch }
    }

    fn observe_refill_epoch(&mut self, refill_epoch: u64) {
        if self.refill_epoch < refill_epoch {
            self.remaining_bytes = self.capacity_bytes;
            self.refill_epoch = refill_epoch;
        }
    }

    // 🚫️async: E1-adjacent — same reasoning as `new` above (R9).
    pub(super) fn try_consume(&mut self, bytes: u64) -> bool {
        if bytes > self.remaining_bytes {
            false
        } else {
            self.remaining_bytes -= bytes;
            true
        }
    }

    /// ♻️ Refills back toward capacity. Driven for real, once a minute, by
    /// [`HttpPool::spawn_refill_driver`] — see that method's doc; this method itself stays the pure
    /// arithmetic step so tests can also exercise refill deterministically without a driver.
    // 🚫️async: E1-adjacent — same reasoning as `new` above (R9).
    fn refill(&mut self, bytes: u64) {
        self.remaining_bytes = (self.remaining_bytes + bytes).min(self.capacity_bytes);
    }
}

/// 💰️ One package's byte budget at one instant: what it may still move before the next refill turn, and its whole
/// per-turn allowance. A caller about to move a known number of bytes compares the two and waits for a refill turn instead
/// of starting a transfer the budget would cut short.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpPackageBudget {
    pub remaining_bytes: u64,
    pub capacity_bytes: u64,
}

/// 🐌️ How often [`HttpPool::spawn_refill_driver`] tops every tracked package's bucket back to its
/// `network_bytes_per_min` cap in production: `NativeDirectoryTransport::with_new_http_pool_now` starts its
/// pool's driver at this interval, so the budget is an allowance per minute, never a lifetime one. This
/// crate's own tests pass a short interval instead.
pub const HTTP_BUCKET_REFILL_INTERVAL_MS: u64 = 60_000;

/// 🔓️ Releases one outstanding-request slot for `actor` when its exact
/// [`HttpOutstandingCredit`] reaches terminal ownership.
// 🚫️async: E1 pure in-memory decrement whose consumer chain reaches `Drop::drop` (external trait,
// cannot be async) via `HttpPoolBody::finish` — see R9.
fn release_outstanding_slot(outstanding: &Mutex<HashMap<ActorId, u32>>, actor: ActorId) {
    let mut outstanding = outstanding.lock().expect("HttpPool outstanding mutex poisoned");
    if let Some(count) = outstanding.get_mut(&actor) {
        *count = count.saturating_sub(1);
    }
}

struct HttpOutstandingCredit {
    pub(super) outstanding: Arc<Mutex<HashMap<ActorId, u32>>>,
    pub(super) actor: ActorId,
    pub(super) terminal: Option<HttpTransportTerminalHandle>,
}

impl HttpOutstandingCredit {
    pub(super) fn new(outstanding: Arc<Mutex<HashMap<ActorId, u32>>>, actor: ActorId) -> Self {
        Self { outstanding, actor, terminal: None }
    }

    fn bind_terminal(&mut self, terminal: HttpTransportTerminalHandle) {
        assert!(self.terminal.replace(terminal).is_none(), "one HTTP pool credit binds one exact transport terminal");
    }
}

impl Drop for HttpOutstandingCredit {
    fn drop(&mut self) {
        let outstanding = self.outstanding.clone();
        let actor = self.actor;
        if let Some(terminal) = self.terminal.take() {
            terminal.retain_release(Box::new(move || release_outstanding_slot(&outstanding, actor)));
        } else {
            release_outstanding_slot(&outstanding, actor);
        }
    }
}

enum HttpPoolTransport {
    Blocking { transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool> },
    Async(Arc<dyn AsyncHttpTransport>),
}

/// 🌐️ Shared connection-pool boundary: a per-package `network_bytes_per_min` token bucket and a
/// per-actor `outstanding_requests` cap, gating an [`AsyncHttpTransport`]. [`HttpPool::fetch`]
/// charges the bucket per REAL response chunk (via [`HttpPoolBody`]) rather than a pre-request
/// estimate, and releases the outstanding slot on EOF or on the caller dropping the body early —
/// see the crate report's `## one-implementation argument`.
pub struct HttpPool {
    pub(super) transport: HttpPoolTransport,
    #[doc(hidden)] pub buckets: Arc<Mutex<HashMap<PackageId, TokenBucket>>>,
    bytes_per_minute_cap: u64,
    refill_epoch: Arc<AtomicU64>,
    refill_driver_started: Arc<AtomicBool>,
    #[doc(hidden)] pub outstanding: Arc<Mutex<HashMap<ActorId, u32>>>,
    outstanding_cap: u32,
}

impl HttpPool {
    /// 🌐️ Today's only shipped shape: stores `transport`/`compute` directly, dispatched inline by
    /// [`HttpPool::fetch`] with the runtime/scope IT receives per call — the same dispatch
    /// [`BlockingHttpTransport::begin`] performs, just not routed through that type here, because
    /// `fetch`/`request` keep their runtime/scope as borrowed PER-CALL parameters (so existing
    /// callers built against today's `HttpPool::new`/`request` keep compiling unchanged) while
    /// [`AsyncHttpTransport::begin`] needs a transport that OWNS them — see the crate report's
    /// `## honest gaps` for this one acknowledged duplication.
    pub async fn new(transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        Self::new_now(transport, compute, bytes_per_minute_cap, outstanding_cap)
    }

    pub fn new_now(transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        HttpPool {
            transport: HttpPoolTransport::Blocking { transport, compute },
            buckets: Arc::new(Mutex::new(HashMap::new())),
            bytes_per_minute_cap,
            refill_epoch: Arc::new(AtomicU64::new(0)),
            refill_driver_started: Arc::new(AtomicBool::new(false)),
            outstanding: Arc::new(Mutex::new(HashMap::new())),
            outstanding_cap: outstanding_cap.max(1),
        }
    }

    /// 🌐️ For a real [`AsyncHttpTransport`] (a sibling packet's real async client) that needs no
    /// [`ComputePool`] of its own — the transport already does real async I/O.
    pub async fn new_with_async_transport(transport: Arc<dyn AsyncHttpTransport>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        Self::new_with_async_transport_now(transport, bytes_per_minute_cap, outstanding_cap)
    }

    pub fn new_with_async_transport_now(transport: Arc<dyn AsyncHttpTransport>, bytes_per_minute_cap: u64, outstanding_cap: u32) -> HttpPool {
        HttpPool {
            transport: HttpPoolTransport::Async(transport),
            buckets: Arc::new(Mutex::new(HashMap::new())),
            bytes_per_minute_cap,
            refill_epoch: Arc::new(AtomicU64::new(0)),
            refill_driver_started: Arc::new(AtomicBool::new(false)),
            outstanding: Arc::new(Mutex::new(HashMap::new())),
            outstanding_cap: outstanding_cap.max(1),
        }
    }

    /// ♻️ Test/operator hook for a manual top-up outside the once-a-minute driver — see
    /// [`TokenBucket::refill`]'s doc.
    pub async fn refill_package_budget(&self, package: &PackageId, bytes: u64) {
        let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
        let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
        let bucket = buckets.entry(package.clone()).or_insert_with(|| TokenBucket::new_at(self.bytes_per_minute_cap, refill_epoch));
        bucket.observe_refill_epoch(refill_epoch);
        bucket.refill(bytes);
    }

    /// 🔍️ `package`'s remaining bytes this minute — untracked packages read as a full bucket
    /// (nothing has been charged against them yet).
    pub async fn remaining_package_budget(&self, package: &PackageId) -> u64 {
        let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
        let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
        let Some(bucket) = buckets.get_mut(package) else { return self.bytes_per_minute_cap };
        bucket.observe_refill_epoch(refill_epoch);
        bucket.remaining_bytes
    }

    /// 💰️ `package`'s budget right now — an untracked package reads as a full bucket.
    // 🚫️async: E1 pure in-memory read a blocking gateway thread polls while it waits for a refill turn — see R9.
    pub fn package_budget_now(&self, package: &PackageId) -> HttpPackageBudget {
        let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
        let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
        let remaining_bytes = buckets.get_mut(package).map_or(self.bytes_per_minute_cap, |bucket| {
            bucket.observe_refill_epoch(refill_epoch);
            bucket.remaining_bytes
        });
        HttpPackageBudget { remaining_bytes, capacity_bytes: self.bytes_per_minute_cap }
    }

    /// ▶️ Starts one finite maintenance turn per refill interval. A turn advances a shared epoch
    /// in O(1); each package observes that epoch and refills lazily on its next access, so no turn
    /// scans an unbounded package map or occupies a worker while waiting.
    // 🚫️async: E1-adjacent — no suspension point of its own (only SUBMITS the job). See R9.
    pub fn spawn_refill_driver(&self, pool: &WorkerPool, interval_ms: u64) {
        if self.refill_driver_started.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            schedule_http_refill_turn(pool, self.refill_epoch.clone(), interval_ms.max(1));
        }
    }

    /// 🌊️ Starts `request` and returns as soon as the head is known, plus a [`HttpPoolBody`] the
    /// caller streams at its own pace. Charges the per-actor outstanding cap up front (released on
    /// the body's EOF or drop — see [`HttpPoolBody`]) and the per-package byte bucket for the
    /// EXACT, known outbound bytes (`request.url`/`request.body`, not header framing — see the
    /// crate report's `## honest gaps`) up front; RESPONSE bytes are charged separately, for real,
    /// per chunk, as [`HttpPoolBody::next_chunk`] pulls them — this is the fix for the estimate-only
    /// accounting this packet was measured against.
    pub async fn fetch<R: HostAsyncRuntime>(&self, runtime: &R, scope: &ScopeHandle, ctx: OperationContext, package: PackageId, actor: ActorId, request: HttpRequest) -> Result<(HttpResponseHead, HttpPoolBody), HttpPoolError> {
        self.fetch_started(runtime, scope, ctx, package, actor, request, |_| {}).await
    }

    /// 🧷 Publishes the transport's exact cancellation owner through `on_started` before the
    /// response-head future is polled, while retaining the same pool credits as [`Self::fetch`].
    pub async fn fetch_started<R: HostAsyncRuntime, F: FnOnce(HttpBodyCancellationHandle)>(
        &self,
        runtime: &R,
        scope: &ScopeHandle,
        ctx: OperationContext,
        package: PackageId,
        actor: ActorId,
        request: HttpRequest,
        on_started: F,
    ) -> Result<(HttpResponseHead, HttpPoolBody), HttpPoolError> {
        {
            let mut outstanding = self.outstanding.lock().expect("HttpPool outstanding mutex poisoned");
            let count = outstanding.entry(actor).or_insert(0);
            if *count >= self.outstanding_cap {
                return Err(HttpPoolError::OutstandingCapReached { actor, limit: self.outstanding_cap });
            }
            *count += 1;
        }
        let mut credit = HttpOutstandingCredit::new(self.outstanding.clone(), actor);
        let outbound_bytes = (request.body.len() + request.url.len()) as u64;
        let admitted = {
            let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
            let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
            let bucket = buckets.entry(package.clone()).or_insert_with(|| TokenBucket::new_at(self.bytes_per_minute_cap, refill_epoch));
            bucket.observe_refill_epoch(refill_epoch);
            bucket.try_consume(outbound_bytes)
        };
        if !admitted {
            return Err(HttpPoolError::ByteBudgetExhausted { package });
        }
        let start_result = match &self.transport {
            HttpPoolTransport::Blocking { transport, compute } => {
                on_started(HttpBodyCancellationHandle::idle());
                let transport = transport.clone();
                let compute = compute.clone();
                let ctx_for_run = ctx.clone();
                let (terminal, terminal_guard) = HttpTransportTerminalHandle::pair();
                credit.bind_terminal(terminal.clone());
                let result = compute
                    .run_io(runtime, scope, ctx_for_run, move || {
                        let _terminal = terminal_guard;
                        transport.call(request)
                    })
                    .await;
                match result {
                    Ok(Ok(response)) => {
                        let head = HttpResponseHead { status: response.status, headers: response.headers };
                        let body: Box<dyn HttpBody> = Box::new(BufferedHttpBody { remaining: Some(response.body) });
                        Ok((head, body, terminal))
                    }
                    Ok(Err(io_error)) => Err(HttpPoolError::Transport(io_error.to_string())),
                    Err(compute_error) => Err(HttpPoolError::Compute(compute_error)),
                }
            }
            HttpPoolTransport::Async(async_transport) => {
                let start = async_transport.begin(&ctx, request);
                let terminal = start.terminal_handle();
                credit.bind_terminal(terminal.clone());
                on_started(start.cancellation_handle());
                start.finish().await.map(|(head, body)| (head, body, terminal))
            }
        };
        match start_result {
            Ok((head, body, terminal)) => Ok((head, HttpPoolBody { inner: body, package, buckets: self.buckets.clone(), bytes_per_minute_cap: self.bytes_per_minute_cap, refill_epoch: self.refill_epoch.clone(), terminal, credit: Some(credit) })),
            Err(error) => Err(error),
        }
    }

    /// 🌐️ The pre-streaming buffered shape: built ENTIRELY on [`HttpPool::fetch`] — collects every
    /// chunk into one `Vec<u8>` — so there is exactly ONE request/response code path in this crate;
    /// see the crate report's `## one-implementation argument`.
    pub async fn request<R: HostAsyncRuntime>(&self, runtime: &R, scope: &ScopeHandle, ctx: OperationContext, package: PackageId, actor: ActorId, request: HttpRequest) -> Result<HttpResponse, HttpPoolError> {
        let (head, mut body) = self.fetch(runtime, scope, ctx, package, actor, request).await?;
        let mut collected = Vec::new();
        while let Some(chunk) = body.next_chunk().await? {
            collected.extend_from_slice(&chunk);
        }
        Ok(HttpResponse { status: head.status, headers: head.headers, body: collected })
    }
}

fn schedule_http_refill_turn(pool: &WorkerPool, refill_epoch: Arc<AtomicU64>, interval_ms: u64) {
    let deadline_ms = pool.now_ms().saturating_add(interval_ms);
    let turn_pool = pool.clone();
    pool.submit_at(
        deadline_ms,
        Lane::Maintenance,
        Box::new(move || {
            refill_epoch.fetch_add(1, Ordering::SeqCst);
            schedule_http_refill_turn(&turn_pool, refill_epoch, interval_ms);
        }),
    );
}

/// 🌊️ A [`HttpPool::fetch`]'d body: wraps the transport's own [`HttpBody`], charging the
/// per-package byte bucket for the REAL length of every chunk actually pulled (never an estimate),
/// and releasing the actor's outstanding slot exactly once after EOF or cancellation and every
/// physical body reader has reached terminal ownership. Dropping this value interrupts the exact
/// body owner and parks its actor credit behind any reader closure that outlives the awaiting
/// future.
pub struct HttpPoolBody {
    inner: Box<dyn HttpBody>,
    pub(super) package: PackageId,
    pub(super) buckets: Arc<Mutex<HashMap<PackageId, TokenBucket>>>,
    bytes_per_minute_cap: u64,
    refill_epoch: Arc<AtomicU64>,
    pub(super) terminal: HttpTransportTerminalHandle,
    credit: Option<HttpOutstandingCredit>,
}

impl HttpPoolBody {
    pub fn cancellation_handle(&self) -> HttpBodyCancellationHandle {
        self.inner.cancellation_handle()
    }

    // 🚫️async: E1 pure bookkeeping consumed by `Drop::drop` below (external trait, cannot be
    // async) — see R9.
    pub(super) fn finish(&mut self, cancel_in_flight: bool) {
        if cancel_in_flight && self.credit.is_some() {
            let _ = self.inner.cancellation_handle().cancel_in_flight();
        }
        self.credit.take();
    }

    /// 🌊️ Pulls the next real chunk, charging the per-package bucket for its EXACT length before
    /// handing it back. Once the bucket cannot afford a chunk that has ALREADY arrived, this returns
    /// [`HttpPoolError::ByteBudgetExhausted`] and releases the outstanding slot — the caller is
    /// expected to drop this value on error, which closes the underlying connection (see this
    /// type's own doc).
    pub async fn next_chunk(&mut self) -> Result<Option<Vec<u8>>, HttpPoolError> {
        if self.credit.is_none() {
            return Ok(None);
        }
        let terminal = self.terminal.worker_guard();
        match self.inner.next_chunk(terminal).await {
            Ok(Some(chunk)) => {
                let admitted = {
                    let refill_epoch = self.refill_epoch.load(Ordering::SeqCst);
                    let mut buckets = self.buckets.lock().expect("HttpPool buckets mutex poisoned");
                    let bucket = buckets.entry(self.package.clone()).or_insert_with(|| TokenBucket::new_at(self.bytes_per_minute_cap, refill_epoch));
                    bucket.observe_refill_epoch(refill_epoch);
                    bucket.try_consume(chunk.len() as u64)
                };
                if !admitted {
                    self.finish(true);
                    return Err(HttpPoolError::ByteBudgetExhausted { package: self.package.clone() });
                }
                Ok(Some(chunk))
            }
            Ok(None) => {
                self.finish(false);
                Ok(None)
            }
            Err(error) => {
                self.finish(true);
                Err(error)
            }
        }
    }
}

impl Drop for HttpPoolBody {
    fn drop(&mut self) {
        self.finish(true);
    }
}
//#endregion 🌐️HttpPool
