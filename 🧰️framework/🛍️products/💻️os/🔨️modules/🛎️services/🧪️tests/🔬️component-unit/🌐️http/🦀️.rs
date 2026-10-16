//! 🌐️ Tests of the HTTP pool over recording, blocking, abandoning and local-socket transports: byte charging, mid-stream drops and cancellation.

use semio_framework_actor::{ActorId, PackageId};
use semio_framework_async::{HostAsyncRuntime, HostFuture, Lane, OperationContext, ScopeHandle, ScopeOwner};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::task::{Context, Waker};
use std::time::Duration;
use super::super::completion_sink::MockCompletionSink;
use super::super::compute_pool::ComputePool;
use super::super::host_runtime::TokioHostRuntime;
use super::super::http_contract::{
    AsyncHttpTransport,
    BufferedHttpBody,
    HttpBody,
    HttpBodyCancellationHandle,
    HttpBodyCancellationStep,
    HttpPoolError,
    HttpRequest,
    HttpResponse,
    HttpResponseHead,
    HttpTransport,
    HttpTransportStart,
    HttpTransportTerminalGuard,
    HttpTransportTerminalHandle,
    UnwiredHttpTransport,
};
use super::super::http_pool::{HttpPool, TokenBucket};
use super::super::http_socket::{SOCKET_HTTP_BODY_PAGE_BYTES, SOCKET_HTTP_HOST_BYTES, SocketHttpCancellation, SocketHttpTransport, socket_http_read_line, socket_http_url};
use super::super::timer_wheel::TimerWheel;
use super::support::{sleep_ms, test_ctx, test_pool};

//#region 🌐️HttpPoolTests
struct RecordingTransport {
    calls: Arc<Mutex<u32>>,
}
impl HttpTransport for RecordingTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        *self.calls.lock().unwrap() += 1;
        Ok(HttpResponse { status: 200, headers: Vec::new(), body: request.body })
    }
}

async fn sample_request() -> HttpRequest {
    HttpRequest { method: "GET".to_string(), url: "https://example.invalid/x".to_string(), headers: Vec::new(), body: Vec::new() }
}

struct BlockingTransport(Mutex<std::sync::mpsc::Receiver<()>>);
impl HttpTransport for BlockingTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        self.0.lock().unwrap().recv().ok();
        Ok(HttpResponse { status: 200, headers: Vec::new(), body: request.body })
    }
}

struct AbandonedStartTransport {
    calls: AtomicU32,
    started: Mutex<Option<std::sync::mpsc::Sender<()>>>,
    pub(super) release: Mutex<std::sync::mpsc::Receiver<()>>,
    pub(super) terminal: Mutex<Option<std::sync::mpsc::Sender<()>>>,
}

impl HttpTransport for AbandonedStartTransport {
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            if let Some(started) = self.started.lock().unwrap().take() {
                let _ = started.send(());
            }
            let _ = self.release.lock().unwrap().recv();
            if let Some(terminal) = self.terminal.lock().unwrap().take() {
                let _ = terminal.send(());
            }
        }
        Ok(HttpResponse { status: 200, headers: Vec::new(), body: request.body })
    }
}

/// 🌐️ Runs a first request on a background OS thread (this crate no longer builds a tokio
/// `Runtime`, so `tokio::spawn` is gone — a plain `std::thread::spawn` driving its own
/// `runtime.block_on` call is the direct replacement), blocked on `unblock_rx` so it stays
/// genuinely outstanding, then waits 40ms (well past the background thread's own startup) before
/// issuing a second request that must be rejected while the first still holds the actor's one
/// slot.
#[semio_framework_async_macros::async_test]
async fn http_pool_rejects_past_the_per_actor_outstanding_cap() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-test"), None).await;
    let (unblock_tx, unblock_rx) = std::sync::mpsc::channel::<()>();
    let compute = Arc::new(ComputePool::new(4).await);
    let pool = Arc::new(HttpPool::new(Arc::new(BlockingTransport(Mutex::new(unblock_rx))), compute, 1_000_000, 1).await);
    let actor = ActorId(9);

    let pool_bg = pool.clone();
    let runtime_bg = runtime.clone();
    let scope_bg = scope.clone();
    let ctx_bg = test_ctx(0, scope.cancel.clone()).await;
    let request_bg = sample_request().await;
    let handle = std::thread::spawn(move || {
        let fut = pool_bg.request(runtime_bg.as_ref(), &scope_bg, ctx_bg, PackageId("pkg".to_string()), actor, request_bg);
        runtime_bg.block_on(fut)
    });
    sleep_ms(runtime.as_ref(), 40).await;
    let ctx2 = test_ctx(0, scope.cancel.clone()).await;
    let second = pool.request(runtime.as_ref(), &scope, ctx2, PackageId("pkg".to_string()), actor, sample_request().await).await;
    assert_eq!(second, Err(HttpPoolError::OutstandingCapReached { actor, limit: 1 }));
    let _ = unblock_tx.send(());
    let first_result = handle.join().expect("background request thread must not panic");
    assert!(first_result.is_ok(), "the first request must still complete once unblocked");
}

/// 🧷️ Dropping a checked-out response-head future after its worker reaches terminal must retire
/// that future's exact actor credit, so a sibling can use the one-slot pool.
#[semio_framework_async_macros::async_test]
async fn http_pool_abandoned_response_head_retires_its_exact_outstanding_credit_after_worker_terminal() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-abandoned-start"), None).await;
    let compute = Arc::new(ComputePool::new(2).await);
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let (terminal_tx, terminal_rx) = std::sync::mpsc::channel();
    let transport = Arc::new(AbandonedStartTransport { calls: AtomicU32::new(0), started: Mutex::new(Some(started_tx)), release: Mutex::new(release_rx), terminal: Mutex::new(Some(terminal_tx)) });
    let pool = HttpPool::new(transport, compute, 1_000_000, 1).await;
    let actor = ActorId(19);
    let package = PackageId("http-abandoned-start".into());
    let observed_owner = Arc::new(Mutex::new(None));
    let owner_slot = observed_owner.clone();
    let abandoned_ctx = test_ctx(0, scope.cancel.clone()).await;
    let abandoned_request = sample_request().await;
    let mut abandoned = Box::pin(pool.fetch_started(runtime.as_ref(), &scope, abandoned_ctx, package.clone(), actor, abandoned_request, move |owner| {
        *owner_slot.lock().unwrap() = Some(owner);
    }));
    let mut context = Context::from_waker(Waker::noop());
    assert!(Future::poll(abandoned.as_mut(), &mut context).is_pending());
    started_rx.recv_timeout(Duration::from_secs(2)).expect("first transport worker must start");
    assert!(observed_owner.lock().unwrap().is_some(), "the cancellation owner must publish before the response-head future waits");
    drop(abandoned);
    let premature = pool.fetch(runtime.as_ref(), &scope, test_ctx(0, scope.cancel.clone()).await, package.clone(), actor, sample_request().await).await;
    assert!(matches!(premature, Err(HttpPoolError::OutstandingCapReached { actor: rejected, limit: 1 }) if rejected == actor), "a dropped response-head future must retain its actor credit while the physical worker remains live");
    release_tx.send(()).unwrap();
    terminal_rx.recv_timeout(Duration::from_secs(2)).expect("abandoned transport worker must reach terminal before sibling admission");

    let sibling = pool.fetch(runtime.as_ref(), &scope, test_ctx(0, scope.cancel.clone()).await, package, actor, sample_request().await).await;
    assert!(sibling.is_ok(), "dropping an abandoned response-head future must return its exact outstanding credit after its worker terminates");
}

#[semio_framework_async_macros::async_test]
async fn http_pool_rejects_when_byte_budget_exhausted_and_transport_is_never_called() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let scope = runtime.open_scope(ScopeOwner::Service("http-budget-test"), None).await;
    let calls = Arc::new(Mutex::new(0));
    let compute = Arc::new(ComputePool::new(4).await);
    let pool = HttpPool::new(Arc::new(RecordingTransport { calls: calls.clone() }), compute, 4, 4).await;
    let actor = ActorId(10);
    let package = PackageId("pkg".to_string());
    let mut big_request = sample_request().await;
    big_request.body = vec![0u8; 10];
    let ctx = test_ctx(0, scope.cancel.clone()).await;
    let result = runtime.block_on(pool.request(&runtime, &scope, ctx, package.clone(), actor, big_request));
    assert_eq!(result, Err(HttpPoolError::ByteBudgetExhausted { package }));
    assert_eq!(*calls.lock().unwrap(), 0, "the transport must never be called once the budget rejects the request");
}

/// ♻️ Directly seeds a package's bucket down to a known remainder (module-private field access
/// from this `tests` submodule — no new production API needed for it), then proves
/// `spawn_refill_driver`'s job actually RUNS on a tick: no refill before the interval elapses, a
/// full top-up once it does. Injects a SHORT `interval_ms` (real wall-clock milliseconds, not
/// [`HTTP_BUCKET_REFILL_INTERVAL_MS`]'s real 60 seconds — `spawn_refill_driver` now drives real
/// [`WorkerPool`] worker threads on the real clock, no injected `ManualRuntime` clock any more).
#[semio_framework_async_macros::async_test]
async fn http_pool_refill_driver_actually_refills_a_consumed_bucket_on_its_tick() {
    const REFILL_INTERVAL_MS: u64 = 40;
    let pool_workers = test_pool(2);
    let compute = Arc::new(ComputePool::new(2).await);
    let pool = HttpPool::new(Arc::new(UnwiredHttpTransport), compute, 100, 4).await;
    let package = PackageId("pkg-refill".to_string());
    {
        let mut buckets = pool.buckets.lock().unwrap();
        buckets.entry(package.clone()).or_insert_with(|| TokenBucket::new_at(100, 0)).try_consume(70);
    }
    assert_eq!(pool.remaining_package_budget(&package).await, 30);

    pool.spawn_refill_driver(&pool_workers, REFILL_INTERVAL_MS);
    assert_eq!(pool.remaining_package_budget(&package).await, 30, "must not refill before the tick interval elapses");
    let start = std::time::Instant::now();
    loop {
        if pool.remaining_package_budget(&package).await == 100 {
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(5), "the refill driver must actually run its loop and top the bucket back up on its own tick");
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[semio_framework_async_macros::async_test]
async fn finite_timer_and_refill_drivers_do_not_starve_a_single_worker() {
    let workers = test_pool(1);
    let wheel = TimerWheel::new(10).await;
    wheel.spawn_driver(&workers, Arc::new(MockCompletionSink::new().await));
    let compute = Arc::new(ComputePool::with_pool(1, workers.clone()));
    let http = HttpPool::new(Arc::new(UnwiredHttpTransport), compute, 100, 1).await;
    http.spawn_refill_driver(&workers, 10);
    let (tx, rx) = std::sync::mpsc::channel();
    workers.submit(Lane::Interactive, Box::new(move || tx.send(()).expect("interactive signal")));
    rx.recv_timeout(Duration::from_secs(1)).expect("finite service drivers must leave the only worker available");
    workers.shutdown();
}

#[test]
fn http_body_cancellation_handles_report_each_exact_owner_outcome() {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_witness = interrupted.clone();
    let interrupt = HttpBodyCancellationHandle::interrupt(move || {
        interrupted_witness.store(true, Ordering::SeqCst);
        Ok(())
    });
    assert_eq!(HttpBodyCancellationHandle::idle().cancel_in_flight(), HttpBodyCancellationStep::Idle);
    assert_eq!(interrupt.cancel_in_flight(), HttpBodyCancellationStep::Interrupted);
    assert!(interrupted.load(Ordering::SeqCst));
    assert_eq!(HttpBodyCancellationHandle::read_deadline(15_000).cancel_in_flight(), HttpBodyCancellationStep::AwaitingReadDeadline { maximum_ms: 15_000 });
    assert_eq!(HttpBodyCancellationHandle::interrupt(|| Err(std::io::Error::other("transport cancellation probe failed"))).cancel_in_flight(), HttpBodyCancellationStep::Fault("transport cancellation probe failed".into()));
    assert_eq!(BufferedHttpBody { remaining: Some(vec![1]) }.cancellation_handle().cancel_in_flight(), HttpBodyCancellationStep::Idle);
}

/// 🌐️ A test-only `AsyncHttpTransport`/`HttpBody` over a REAL local TCP socket — the harness the
/// packet report's `## honest gaps` asks for if a raw listener inside a unit test is awkward.
/// Every `next_chunk` call does one real blocking `read` through `ComputePool`, so bytes charged
/// against the package bucket are genuinely read off the wire, not buffered/estimated upfront.
struct LocalSocketBody {
    pub(super) stream: Arc<Mutex<std::net::TcpStream>>,
    pub(super) cancellation: HttpBodyCancellationHandle,
    pub(super) compute: Arc<ComputePool>,
    pub(super) runtime: Arc<TokioHostRuntime>,
    pub(super) scope: ScopeHandle,
    pub(super) ctx: OperationContext,
    /// 🔍️ Set when this value drops, so a test can OBSERVE that `HttpPoolBody`'s own `Drop`
    /// really did drop the transport body (and therefore the socket) rather than merely
    /// stopping the caller from polling it further.
    dropped: Arc<AtomicBool>,
    read_probe: Option<Arc<LocalSocketReadProbe>>,
}

struct LocalSocketReadProbe {
    started: Mutex<Option<std::sync::mpsc::Sender<()>>>,
    returned: Mutex<Option<std::sync::mpsc::Sender<()>>>,
    pub(super) release: Mutex<Option<std::sync::mpsc::Receiver<()>>>,
    pub(super) terminal: Mutex<Option<std::sync::mpsc::Sender<()>>>,
}

impl HttpBody for LocalSocketBody {
    fn cancellation_handle(&self) -> HttpBodyCancellationHandle {
        self.cancellation.clone()
    }

    // 🚫️async: E6 dyn-compat — see the `HttpBody` trait's tag.
    fn next_chunk(&mut self, terminal: HttpTransportTerminalGuard) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
        let stream = self.stream.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx = self.ctx.clone();
        let read_probe = self.read_probe.clone();
        Box::pin(async move {
            let outcome = compute
                .run_io(runtime.as_ref(), &scope, ctx, move || {
                    let _terminal = terminal;
                    use std::io::Read;
                    if let Some(started) = read_probe.as_ref().and_then(|probe| probe.started.lock().unwrap().take()) {
                        let _ = started.send(());
                    }
                    let mut buf = [0u8; 64];
                    let mut guard = stream.lock().expect("test socket mutex poisoned");
                    let result = match guard.read(&mut buf) {
                        Ok(0) => None,
                        Ok(n) => Some(buf[..n].to_vec()),
                        Err(_) => None,
                    };
                    if let Some(probe) = read_probe {
                        if let Some(returned) = probe.returned.lock().unwrap().take() {
                            let _ = returned.send(());
                        }
                        if let Some(release) = probe.release.lock().unwrap().take() {
                            let _ = release.recv_timeout(Duration::from_secs(5));
                        }
                        if let Some(terminal) = probe.terminal.lock().unwrap().take() {
                            let _ = terminal.send(());
                        }
                    }
                    result
                })
                .await;
            outcome.map_err(HttpPoolError::Compute)
        })
    }
}
impl Drop for LocalSocketBody {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::SeqCst);
    }
}

struct LocalSocketTransport {
    addr: std::net::SocketAddr,
    pub(super) compute: Arc<ComputePool>,
    pub(super) runtime: Arc<TokioHostRuntime>,
    pub(super) scope: ScopeHandle,
    dropped: Arc<AtomicBool>,
    read_probe: Option<Arc<LocalSocketReadProbe>>,
}
impl AsyncHttpTransport for LocalSocketTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn begin(&self, ctx: &OperationContext, _request: HttpRequest) -> HttpTransportStart {
        let addr = self.addr;
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx_for_connect = ctx.clone();
        let ctx_for_body = ctx.clone();
        let dropped = self.dropped.clone();
        let read_probe = self.read_probe.clone();
        let cancellation_owner = Arc::new(SocketHttpCancellation::new());
        let cancellation = SocketHttpCancellation::handle(&cancellation_owner);
        let body_cancellation = cancellation.clone();
        let (terminal, terminal_guard) = HttpTransportTerminalHandle::pair();
        let response = Box::pin(async move {
            let connect_result = compute
                .run_io(runtime.as_ref(), &scope, ctx_for_connect, move || {
                    let _terminal = terminal_guard;
                    std::net::TcpStream::connect(addr)
                })
                .await;
            let stream = match connect_result {
                Ok(Ok(stream)) => stream,
                Ok(Err(io_error)) => return Err(HttpPoolError::Transport(io_error.to_string())),
                Err(compute_error) => return Err(HttpPoolError::Compute(compute_error)),
            };
            let head = HttpResponseHead { status: 200, headers: Vec::new() };
            cancellation_owner.bind(&stream).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
            let body: Box<dyn HttpBody> = Box::new(LocalSocketBody { stream: Arc::new(Mutex::new(stream)), cancellation, compute, runtime, scope, ctx: ctx_for_body, dropped, read_probe });
            Ok((head, body))
        });
        HttpTransportStart::new(body_cancellation, terminal, response)
    }
}

/// 🐌️ Binds an ephemeral local listener and, for every accepted connection, writes `chunks` in
/// order with a small delay between each — a genuine streamed response, not a single buffered
/// write. Accepts indefinitely (background thread lives for the test's duration) so more than
/// one test connection can be served.
async fn spawn_chunk_server(chunks: Vec<Vec<u8>>) -> std::net::SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind local test listener");
    let addr = listener.local_addr().expect("read local test listener addr");
    std::thread::spawn(move || {
        for incoming in listener.incoming() {
            let Ok(mut stream) = incoming else { continue };
            let chunks = chunks.clone();
            std::thread::spawn(move || {
                use std::io::Write;
                for chunk in chunks {
                    if stream.write_all(&chunk).is_err() {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(5));
                }
            });
        }
    });
    addr
}

/// 🌐️ The run-the-real-thing case: a genuine multi-chunk response over a real local TCP socket,
/// asserting the package bucket is charged EXACTLY each chunk's real length as it arrives — not
/// an upfront estimate, not the whole body's length in one shot.
#[semio_framework_async_macros::async_test]
async fn http_pool_fetch_charges_real_bytes_per_chunk_over_a_local_tcp_listener() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-stream-test"), None).await;
    let compute = Arc::new(ComputePool::new(4).await);
    let chunks = vec![vec![1u8; 10], vec![2u8; 15], vec![3u8; 7]];
    let addr = spawn_chunk_server(chunks.clone()).await;
    let dropped = Arc::new(AtomicBool::new(false));
    let transport = Arc::new(LocalSocketTransport { addr, compute, runtime: runtime.clone(), scope: scope.clone(), dropped, read_probe: None });
    let pool = HttpPool::new_with_async_transport(transport, 1_000_000, 4).await;
    let package = PackageId("pkg-stream".to_string());
    let actor = ActorId(11);

    runtime.block_on(async {
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let (head, mut body) = pool.fetch(runtime.as_ref(), &scope, ctx, package.clone(), actor, sample_request().await).await.expect("fetch should succeed");
        assert_eq!(head.status, 200);
        let mut before = pool.remaining_package_budget(&package).await;
        let mut total = Vec::new();
        while let Some(chunk) = body.next_chunk().await.expect("chunk read should succeed") {
            let after = pool.remaining_package_budget(&package).await;
            assert_eq!(before - after, chunk.len() as u64, "each real chunk must charge exactly its own real byte length, not an estimate");
            before = after;
            total.extend(chunk);
        }
        let expected: Vec<u8> = chunks.into_iter().flatten().collect();
        assert_eq!(total, expected, "streamed bytes must match what the server actually sent");
    });
}

/// 🌐️ The cancellation case: drop a fetched body after reading only its FIRST chunk (a consumer
/// bailing out mid-stream). This must (a) drop the transport's own `HttpBody` — closing the
/// connection, observed via `dropped` — and (b) free the actor's outstanding slot immediately,
/// proved by a SECOND `fetch` against the same actor succeeding under `outstanding_cap: 1`
/// rather than being rejected.
#[semio_framework_async_macros::async_test]
async fn http_pool_dropping_a_body_mid_stream_frees_the_outstanding_slot_and_drops_the_connection() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-cancel-test"), None).await;
    let compute = Arc::new(ComputePool::new(4).await);
    let chunks: Vec<Vec<u8>> = (0..20).map(|_| vec![9u8; 8]).collect();
    let addr = spawn_chunk_server(chunks).await;
    let dropped = Arc::new(AtomicBool::new(false));
    let transport = Arc::new(LocalSocketTransport { addr, compute, runtime: runtime.clone(), scope: scope.clone(), dropped: dropped.clone(), read_probe: None });
    let pool = HttpPool::new_with_async_transport(transport, 1_000_000, 1).await;
    let package = PackageId("pkg-cancel".to_string());
    let actor = ActorId(12);

    runtime.block_on(async {
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let (_head, mut body) = pool.fetch(runtime.as_ref(), &scope, ctx, package.clone(), actor, sample_request().await).await.expect("fetch should succeed");
        let first_chunk = body.next_chunk().await.expect("first chunk should read").expect("server must have sent at least one chunk");
        assert_eq!(first_chunk.len(), 8);
        drop(body);
    });
    assert!(dropped.load(Ordering::SeqCst), "dropping HttpPoolBody mid-stream must drop the underlying transport body, closing its connection");

    runtime.block_on(async {
        let ctx2 = test_ctx(0, scope.cancel.clone()).await;
        let second = pool.fetch(runtime.as_ref(), &scope, ctx2, package.clone(), actor, sample_request().await).await;
        assert!(second.is_ok(), "the outstanding slot must have been freed by the drop, not held open until a full response finished");
    });
}

/// 🪢 An abandoned pending body read retains its actor credit until the physical reader terminates.
#[semio_framework_async_macros::async_test]
async fn http_pool_abandoned_pending_body_read_retires_its_actor_credit_after_reader_terminal() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../../../📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/📄️native-asset-response/🔣️.json")).expect("native asset response contract");
    let expected = &contract["transportCancellation"]["abandonedBodyRead"];
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind abandoned body listener");
    let addr = listener.local_addr().expect("abandoned body listener address");
    listener.set_nonblocking(true).expect("nonblocking abandoned body listener");
    let (release_socket_tx, release_socket_rx) = std::sync::mpsc::channel();
    let (sibling_served_tx, sibling_served_rx) = std::sync::mpsc::channel();
    let stop = Arc::new(AtomicBool::new(false));
    let server_stop = stop.clone();
    let server = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        let mut first = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock && std::time::Instant::now() < deadline => std::thread::yield_now(),
                Err(error) => panic!("abandoned body first accept failed: {error}"),
            }
        };
        let first = std::thread::spawn(move || {
            let _ = release_socket_rx.recv_timeout(Duration::from_secs(5));
            let _ = std::io::Write::write_all(&mut first, b"a");
        });
        while !server_stop.load(Ordering::Acquire) {
            match listener.accept() {
                Ok((mut sibling, _)) => {
                    let _ = std::io::Write::write_all(&mut sibling, b"b");
                    let _ = sibling_served_tx.send(());
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => std::thread::yield_now(),
                Err(_) => break,
            }
        }
        let _ = first.join();
    });
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("http-abandoned-body"), None).await;
    let compute = Arc::new(ComputePool::new(4).await);
    let (read_started_tx, read_started_rx) = std::sync::mpsc::channel();
    let (read_returned_tx, read_returned_rx) = std::sync::mpsc::channel();
    let (release_reader_tx, release_reader_rx) = std::sync::mpsc::channel();
    let (read_terminal_tx, read_terminal_rx) = std::sync::mpsc::channel();
    let probe = Arc::new(LocalSocketReadProbe { started: Mutex::new(Some(read_started_tx)), returned: Mutex::new(Some(read_returned_tx)), release: Mutex::new(Some(release_reader_rx)), terminal: Mutex::new(Some(read_terminal_tx)) });
    let dropped = Arc::new(AtomicBool::new(false));
    let transport = Arc::new(LocalSocketTransport { addr, compute, runtime: runtime.clone(), scope: scope.clone(), dropped: dropped.clone(), read_probe: Some(probe) });
    let pool = HttpPool::new_with_async_transport(transport, 1_000_000, expected["outstandingCap"].as_u64().expect("outstanding cap") as u32).await;
    let package = PackageId("http-abandoned-body".into());
    let actor = ActorId(20);
    let (_, mut body) = pool.fetch(runtime.as_ref(), &scope, test_ctx(0, scope.cancel.clone()).await, package.clone(), actor, sample_request().await).await.expect("first response head");
    let mut read = Box::pin(body.next_chunk());
    let mut context = Context::from_waker(Waker::noop());
    let read_pending = Future::poll(read.as_mut(), &mut context).is_pending();
    read_started_rx.recv_timeout(Duration::from_secs(2)).expect("physical body reader starts");
    drop(read);
    drop(body);
    assert!(dropped.load(Ordering::SeqCst), "the abandoned pool body drops its transport body");
    let _ = release_socket_tx.send(());
    read_returned_rx.recv_timeout(Duration::from_secs(2)).expect("physical read returns into its terminal barrier");

    let sibling_owner = Arc::new(AtomicBool::new(false));
    let observed_sibling_owner = sibling_owner.clone();
    let mut premature = Box::pin(pool.fetch_started(runtime.as_ref(), &scope, test_ctx(0, scope.cancel.clone()).await, package.clone(), actor, sample_request().await, move |_| {
        observed_sibling_owner.store(true, Ordering::Release);
    }));
    let premature_poll = Future::poll(premature.as_mut(), &mut context);
    let sibling_refused_before_reader_terminal = matches!(&premature_poll, std::task::Poll::Ready(Err(HttpPoolError::OutstandingCapReached { actor: rejected, limit: 1 })) if *rejected == actor);
    let prematurely_admitted = sibling_owner.load(Ordering::Acquire);
    drop(premature_poll);
    drop(premature);
    if prematurely_admitted {
        sibling_served_rx.recv_timeout(Duration::from_secs(2)).expect("prematurely admitted sibling cleanup");
    }
    let _ = release_reader_tx.send(());
    let reader_terminal_before_sibling = read_terminal_rx.recv_timeout(Duration::from_secs(2)).is_ok();
    let terminal_deadline = std::time::Instant::now() + Duration::from_secs(2);
    while pool.outstanding.lock().unwrap().get(&actor).copied().unwrap_or(0) != 0 && std::time::Instant::now() < terminal_deadline {
        std::thread::yield_now();
    }
    let sibling = pool.fetch(runtime.as_ref(), &scope, test_ctx(0, scope.cancel.clone()).await, package, actor, sample_request().await).await;
    let sibling_admitted = sibling.is_ok();
    if let Ok((_, mut body)) = sibling {
        let _ = body.next_chunk().await;
        drop(body);
        let _ = sibling_served_rx.recv_timeout(Duration::from_secs(2));
    }
    stop.store(true, Ordering::Release);
    server.join().expect("abandoned body server cleanup");

    assert_eq!(read_pending, expected["readPendingBeforeBodyDrop"].as_bool().expect("pending body read"));
    assert_eq!(sibling_refused_before_reader_terminal, expected["siblingRefusedBeforeReaderTerminal"].as_bool().expect("pre-terminal sibling refusal"));
    assert_eq!(reader_terminal_before_sibling, expected["readerTerminalBeforeSibling"].as_bool().expect("reader terminal ordering"));
    assert_eq!(sibling_admitted, expected["siblingAdmitted"].as_bool().expect("post-terminal sibling admission"));
}

#[semio_framework_async_macros::async_test]
async fn socket_http_transport_streams_chunked_and_content_length_bodies_without_aggregation() {
    fn server(response: &'static [u8]) -> std::net::SocketAddr {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0; 1024];
            let _ = std::io::Read::read(&mut stream, &mut request);
            for page in response.chunks(3) {
                std::io::Write::write_all(&mut stream, page).unwrap();
            }
        });
        addr
    }

    async fn collect(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, addr: std::net::SocketAddr) -> (HttpResponseHead, Vec<Vec<u8>>) {
        let compute = Arc::new(ComputePool::with_pool(2, test_pool(2)));
        let transport = Arc::new(SocketHttpTransport::new(compute, runtime.clone(), scope.clone()));
        let pool = HttpPool::new_with_async_transport_now(transport, 1_000_000, 1);
        let request = HttpRequest { method: "GET".into(), url: format!("http://{addr}/asset"), headers: Vec::new(), body: Vec::new() };
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let (head, mut body) = pool.fetch(runtime.as_ref(), &scope, ctx, PackageId("socket-http".into()), ActorId(17), request).await.unwrap();
        let mut pages = Vec::new();
        while let Some(page) = body.next_chunk().await.unwrap() {
            assert!(!page.is_empty());
            assert!(page.len() <= SOCKET_HTTP_BODY_PAGE_BYTES);
            pages.push(page);
        }
        (head, pages)
    }

    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("socket-http"), None).await;
    let chunked = server(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n3\r\nbye\r\n0\r\n\r\n");
    let (head, pages) = collect(runtime.clone(), scope.clone(), chunked).await;
    assert_eq!(head.status, 200);
    assert_eq!(pages, vec![b"hello".to_vec(), b"bye".to_vec()]);

    let fixed = server(b"HTTP/1.1 206 Partial Content\r\nContent-Length: 8\r\n\r\n12345678");
    let (head, pages) = collect(runtime, scope, fixed).await;
    assert_eq!(head.status, 206);
    assert_eq!(pages.concat(), b"12345678");
}

#[semio_framework_async_macros::async_test]
async fn socket_http_body_cancellation_interrupts_a_blocked_read_and_releases_its_exact_pool_slot() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let server = std::thread::spawn(move || {
        use std::io::{Read, Write};
        fn accept_before(listener: &std::net::TcpListener, deadline: std::time::Instant) -> Option<std::net::TcpStream> {
            while std::time::Instant::now() < deadline {
                match listener.accept() {
                    Ok((stream, _)) => return Some(stream),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => std::thread::yield_now(),
                    Err(_) => return None,
                }
            }
            None
        }
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        let Some(mut stalled) = accept_before(&listener, deadline) else { return };
        stalled.set_nonblocking(false).unwrap();
        let mut request = [0; 1024];
        let _ = stalled.read(&mut request);
        stalled.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n").unwrap();
        stalled.flush().unwrap();
        let _ = release_rx.recv_timeout(Duration::from_secs(5));
        let _ = stalled.write_all(b"a");

        let Some(mut sibling) = accept_before(&listener, deadline) else { return };
        sibling.set_nonblocking(false).unwrap();
        let _ = sibling.read(&mut request);
        sibling.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\nb").unwrap();
    });
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("socket-http-cancel"), None).await;
    let transport = Arc::new(SocketHttpTransport::new(Arc::new(ComputePool::with_pool(2, test_pool(2))), runtime.clone(), scope.clone()));
    let pool = HttpPool::new_with_async_transport_now(transport, 1_000_000, 1);
    let request = HttpRequest { method: "GET".into(), url: format!("http://{addr}/asset"), headers: Vec::new(), body: Vec::new() };
    let ctx = test_ctx(0, scope.cancel.clone()).await;
    let (_, mut body) = pool.fetch(runtime.as_ref(), &scope, ctx, PackageId("socket-http-cancel".into()), ActorId(18), request.clone()).await.unwrap();
    let cancellation = body.cancellation_handle();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (result_tx, result_rx) = std::sync::mpsc::channel();
    let reader_runtime = runtime.clone();
    let reader = std::thread::spawn(move || {
        started_tx.send(()).unwrap();
        let result = reader_runtime.block_on(async move { body.next_chunk().await });
        let _ = result_tx.send(result);
    });
    started_rx.recv_timeout(Duration::from_secs(1)).unwrap();
    let cancellation_step = cancellation.cancel_in_flight();
    let first_result = result_rx.recv_timeout(Duration::from_secs(2));
    let _ = release_tx.send(());
    let completed_before_release = first_result.is_ok();
    let _result = first_result.unwrap_or_else(|_| result_rx.recv_timeout(Duration::from_secs(5)).expect("blocked body read cleanup"));
    reader.join().unwrap();

    let sibling_ctx = test_ctx(0, scope.cancel.clone()).await;
    let (_, mut sibling) = pool.fetch(runtime.as_ref(), &scope, sibling_ctx, PackageId("socket-http-cancel".into()), ActorId(18), request).await.unwrap();
    let sibling_page = sibling.next_chunk().await.unwrap().unwrap();
    let sibling_eof = sibling.next_chunk().await.unwrap();
    drop(sibling);
    server.join().unwrap();

    assert_eq!(cancellation_step, HttpBodyCancellationStep::Interrupted);
    assert!(completed_before_release, "the exact socket cancellation handle must wake the blocked body read before server cleanup releases it");
    assert_eq!(sibling_page, b"b");
    assert!(sibling_eof.is_none());
}

#[test]
fn socket_http_schema_rejects_tls_and_overlong_lines_before_unbounded_ownership() {
    assert!(socket_http_url("https://example.com/asset").is_err());
    let overlong = format!("http://{}/asset", "x".repeat(SOCKET_HTTP_HOST_BYTES + 1));
    assert!(socket_http_url(&overlong).is_err());
    let mut line = std::io::Cursor::new(vec![b'x'; 129]);
    assert!(socket_http_read_line(&mut line, 128).is_err());
}
//#endregion 🌐️HttpPoolTests
