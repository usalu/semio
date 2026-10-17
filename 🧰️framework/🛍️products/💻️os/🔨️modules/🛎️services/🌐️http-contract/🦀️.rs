//! 🌐️ The HTTP pool contract: request and response vocabulary, body cancellation, terminal custody and the blocking transport.

use semio_framework_actor::{ActorId, PackageId};
use semio_framework_async::{HostFuture, OperationContext, ScopeHandle};
use std::sync::{Arc, Mutex};
use super::compute_pool::{ComputeError, ComputePool};
use super::host_runtime::TokioHostRuntime;

//#region 🌐️HttpPool
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// 🧾️ Response status/headers only — the body is [`HttpBody`], pulled separately, so a caller can
/// see the head (and reject on status) before committing to draining a possibly-huge body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponseHead {
    pub status: u16,
    pub headers: Vec<(String, String)>,
}

/// 🚫️ [`HttpPool::request`]'s failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpPoolError {
    OutstandingCapReached { actor: ActorId, limit: u32 },
    ByteBudgetExhausted { package: PackageId },
    Transport(String),
    Compute(ComputeError),
}

impl std::fmt::Display for HttpPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpPoolError::OutstandingCapReached { actor, limit } => write!(f, "actor {actor:?} already has {limit} outstanding HTTP requests"),
            HttpPoolError::ByteBudgetExhausted { package } => write!(f, "package {:?} exhausted its network_bytes_per_min budget", package.0),
            HttpPoolError::Transport(message) => write!(f, "http transport error: {message}"),
            HttpPoolError::Compute(error) => write!(f, "http compute error: {error}"),
        }
    }
}
impl std::error::Error for HttpPoolError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpBodyCancellationStep {
    Idle,
    Interrupted,
    AwaitingReadDeadline { maximum_ms: u64 },
    Fault(String),
}

enum HttpBodyCancellationKind {
    Idle,
    Interrupt(Arc<dyn Fn() -> Result<(), String> + Send + Sync>),
    ReadDeadline { maximum_ms: u64 },
}

#[derive(Clone)]
pub struct HttpBodyCancellationHandle(pub(super) Arc<HttpBodyCancellationKind>);

impl HttpBodyCancellationHandle {
    pub fn idle() -> Self {
        Self(Arc::new(HttpBodyCancellationKind::Idle))
    }

    pub fn interrupt(operation: impl Fn() -> std::io::Result<()> + Send + Sync + 'static) -> Self {
        Self(Arc::new(HttpBodyCancellationKind::Interrupt(Arc::new(move || operation().map_err(|error| error.to_string())))))
    }

    pub fn read_deadline(maximum_ms: u64) -> Self {
        Self(Arc::new(HttpBodyCancellationKind::ReadDeadline { maximum_ms }))
    }

    pub fn cancel_in_flight(&self) -> HttpBodyCancellationStep {
        match self.0.as_ref() {
            HttpBodyCancellationKind::Idle => HttpBodyCancellationStep::Idle,
            HttpBodyCancellationKind::Interrupt(operation) => match operation() {
                Ok(()) => HttpBodyCancellationStep::Interrupted,
                Err(detail) => HttpBodyCancellationStep::Fault(detail),
            },
            HttpBodyCancellationKind::ReadDeadline { maximum_ms } => HttpBodyCancellationStep::AwaitingReadDeadline { maximum_ms: *maximum_ms },
        }
    }
}

/// 🌊️ One streamed HTTP response body, pulled chunk by chunk. `next_chunk` returns `Ok(None)` at
/// EOF. Implementations reach `&mut self` synchronously (to extract whatever owned state the next
/// read needs) and return a `'static` [`HostFuture`] that owns that state — never a future borrowing
/// `self` — the same shape [`AsyncHttpTransport::begin`] itself uses. This is the ONE body type fed
/// to [`HttpPool::fetch`]'s [`HttpPoolBody`] wrapper; a later packet reuses it verbatim for the WASI
/// `stream<u8>` writer and the poll world's chunked events — see the crate report's `## seam design`.
// 🔀️ dedyn-fw-os-misc: DELIBERATELY left `dyn` — a reasoned exception, not an oversight.
// `next_chunk` is plain sync (returns a `HostFuture`, doesn't take the `async` keyword), so `dyn
// HttpBody` is not an E0038 violation and stays R1-legal. Neither de-dyn mechanism fits: (a) closed
// set — the real production set is exactly ONE impl (`BufferedHttpBody`, R11 case 3 would apply),
// but `AsyncHttpTransport::begin` (a sibling trait, out of this packet's family list) returns
// `Box<dyn HttpBody>` from its OWN trait-level signature, and a `#[cfg(test)]`-only second
// implementor (`LocalSocketBody`, this file's own tests) implements that SAME trait method with a
// genuinely different concrete body type — collapsing to one concrete type would break the test
// impl, and fixing that means giving `AsyncHttpTransport` an associated type, out of scope here. (b)
// `dyn_enum_close!` — its variant DSL has no per-variant `#[cfg]`, so a `#[cfg(test)]`-only
// variant cannot be expressed in one enum declaration. Revisit alongside `AsyncHttpTransport` if
// that trait ever gains an associated `Body` type.
// 🚫️async: E6 dyn-compat — machine-readable form of the `dedyn-fw-os-misc` reasoning above, added
// so `asyncify-universal.py` stops re-breaking this trait (it does not yet recognise that tag).
pub trait HttpBody: Send {
    fn cancellation_handle(&self) -> HttpBodyCancellationHandle;
    fn next_chunk(&mut self, terminal: HttpTransportTerminalGuard) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>>;
}

/// 🌐️ The seam a real HTTP client plugs into: `begin` returns the cancellation owner alongside an
/// unpolled future that yields the head plus a [`HttpBody`] the caller streams at its own pace — no
/// whole-body buffering happens below this trait. [`BlockingHttpTransport`] is one implementation (today's
/// synchronous-`HttpTransport`-on-`ComputePool` behaviour, unchanged); a sibling packet adds a real
/// async client behind this same trait, adding no new dependency to THIS crate — see the crate
/// report's `## honest gaps`.
// 🚫️async: E6 dyn-compat — same class as `HttpTransport` above. `begin` already returns a
// `HostFuture` (R1-legal argument/return erasure per `dyn_enum_close!`'s sibling exceptions); an
// `async fn` wrapping THAT would be the literal double-future shape R1 bans, on top of breaking
// `Arc<dyn AsyncHttpTransport>`'s object safety (E0038). `asyncify-universal.py` doesn't yet
// recognise this class — see the `HttpTransport` tag above for the coordinator note.
/// 🌐️ [`AsyncHttpTransport::begin`]'s result — factored into its own alias to keep that trait
/// method's signature under clippy's `type_complexity` threshold; not otherwise meaningful on its
/// own.
pub type StartedTransport = Result<(HttpResponseHead, Box<dyn HttpBody>), HttpPoolError>;

struct HttpTransportTerminalState {
    pub(super) owners: usize,
    pub(super) release: Option<Box<dyn FnOnce() + Send>>,
}

/// 🏁️ Exact terminal group for response-head and body-reader platform work that can outlive its awaiting future.
#[derive(Clone)]
pub struct HttpTransportTerminalHandle(pub(super) Arc<Mutex<HttpTransportTerminalState>>);

/// 🧵️ Worker-owned member of [`HttpTransportTerminalHandle`], completed only when its physical
/// work closure returns or is discarded before admission.
pub struct HttpTransportTerminalGuard(pub(super) Option<HttpTransportTerminalHandle>);

impl HttpTransportTerminalHandle {
    pub fn pair() -> (Self, HttpTransportTerminalGuard) {
        let handle = Self(Arc::new(Mutex::new(HttpTransportTerminalState { owners: 1, release: None })));
        (handle.clone(), HttpTransportTerminalGuard(Some(handle)))
    }

    pub(super) fn worker_guard(&self) -> HttpTransportTerminalGuard {
        let mut state = self.0.lock().expect("HTTP transport terminal mutex poisoned");
        assert!(state.release.is_none(), "HTTP transport work cannot start after its exact pool credit was retired");
        state.owners = state.owners.checked_add(1).expect("HTTP transport terminal owner count overflow");
        HttpTransportTerminalGuard(Some(self.clone()))
    }

    pub(super) fn retain_release(&self, release: Box<dyn FnOnce() + Send>) {
        let ready = {
            let mut state = self.0.lock().expect("HTTP transport terminal mutex poisoned");
            if state.owners == 0 {
                Some(release)
            } else {
                assert!(state.release.replace(release).is_none(), "one HTTP transport terminal retains one exact pool credit");
                None
            }
        };
        if let Some(release) = ready {
            release();
        }
    }

    pub(super) fn complete(&self) {
        let release = {
            let mut state = self.0.lock().expect("HTTP transport terminal mutex poisoned");
            assert!(state.owners > 0, "HTTP transport terminal owner completed twice");
            state.owners -= 1;
            (state.owners == 0).then(|| state.release.take()).flatten()
        };
        if let Some(release) = release {
            release();
        }
    }
}

impl Drop for HttpTransportTerminalGuard {
    fn drop(&mut self) {
        if let Some(handle) = self.0.take() {
            handle.complete();
        }
    }
}

/// 🛫️ Owns cancellation before the response-head future is polled, so callers can publish the
/// exact transport owner before connect, request, or response-head work starts.
pub struct HttpTransportStart {
    pub(super) cancellation: HttpBodyCancellationHandle,
    pub(super) terminal: HttpTransportTerminalHandle,
    pub(super) response: HostFuture<StartedTransport>,
}

impl HttpTransportStart {
    pub fn new(cancellation: HttpBodyCancellationHandle, terminal: HttpTransportTerminalHandle, response: HostFuture<StartedTransport>) -> Self {
        Self { cancellation, terminal, response }
    }

    pub fn cancellation_handle(&self) -> HttpBodyCancellationHandle {
        self.cancellation.clone()
    }

    pub(super) fn terminal_handle(&self) -> HttpTransportTerminalHandle {
        self.terminal.clone()
    }

    pub(super) async fn finish(self) -> StartedTransport {
        self.response.await
    }
}

pub trait AsyncHttpTransport: Send + Sync {
    fn begin(&self, ctx: &OperationContext, request: HttpRequest) -> HttpTransportStart;
}

/// 🌐️ Blocking HTTP transport [`BlockingHttpTransport`] drives through [`ComputePool`] — the same
/// "one blocking call on a dedicated thread" technique `📇️directory/🔌️client` already uses for
/// `ureq`. NO implementation ships in this packet: [`HttpPool::new`] takes any `Arc<dyn
/// HttpTransport>`, and a real transport (a `ureq`-backed one, or a real connection-pooling client)
/// is later-packet wiring — see the crate report's `## honest gaps`. Kept as a trait rather than a
/// concrete client so this crate adds no new external HTTP dependency of its own.
// 🔀️ dedyn-fw-os-misc: DELIBERATELY left `dyn` — a reasoned exception, not an oversight. `call` is
// plain sync (no `async fn`), so `dyn HttpTransport` is not an E0038 violation and stays R1-legal.
// Closing it fully would need EITHER (a) `dyn_enum_close!`, impossible here: implementors span two
// crates in a one-directional dependency (`semio-framework-os-kernel`'s `📇️directory::
// UreqHttpTransport` implements a trait `semio-framework-os-services` defines, so `os-services`
// cannot name `UreqHttpTransport` without a reverse dependency) — OR (b) generics, which would force
// the widely-referenced, non-generic `HttpPool`/`HttpPoolTransport` (60 references across 5 files,
// including `📺️renderer/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — flagged in `📌️important.md` as live/shared with
// concurrent tickets right now) to become generic too, for a trait that costs nothing left as `dyn`.
// Revisit if `HttpTransport` ever needs to become `async fn` (R2/universal-async) — an async trait
// method genuinely cannot stay `dyn`, unlike this sync one.
//
// 🚫️async: E6 dyn-compat (gate-services, new class — not in the R2 E1–E5 list; flagged to the
// coordinator to fold in) — `async fn` in a trait breaks object safety (E0038: "for a trait to be
// dyn compatible it needs to allow building a vtable"), and `Arc<dyn HttpTransport>` is load-bearing
// per the block comment above (implementors span two crates with no valid `dyn_enum_close!` or
// generics fix). `asyncify-universal.py` does not yet recognise this class of exemption — re-scan
// after adding it, or repair by hand again, the way this packet did.
pub trait HttpTransport: Send + Sync {
    fn call(&self, request: HttpRequest) -> Result<HttpResponse, std::io::Error>;
}

/// 🚧️ The default [`HttpTransport`] until a later packet wires a real one — every call fails
/// loudly rather than silently succeeding with fake data.
pub struct UnwiredHttpTransport;
impl HttpTransport for UnwiredHttpTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag above.
    fn call(&self, _request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
        Err(std::io::Error::other("HttpPool: no HttpTransport wired yet (see the packet report's honest gaps)"))
    }
}

/// 🐌️ One whole response, buffered by a [`HttpTransport::call`], replayed as a SINGLE chunk —
/// exactly what `HttpPool::request` did before this packet, now expressed as the one degenerate case
/// of [`HttpBody`] rather than a second code path. See [`BlockingHttpTransport`].
#[doc(hidden)] pub struct BufferedHttpBody {
    pub(super) remaining: Option<Vec<u8>>,
}
impl BufferedHttpBody {
    /// 📦️ Replays one whole buffered response as the single chunk of the body.
    #[doc(hidden)] pub fn new(response: Vec<u8>) -> Self {
        Self { remaining: Some(response) }
    }
}
impl HttpBody for BufferedHttpBody {
    fn cancellation_handle(&self) -> HttpBodyCancellationHandle {
        HttpBodyCancellationHandle::idle()
    }

    // 🚫️async: E6 dyn-compat — see the `HttpBody` trait's `dedyn-fw-os-misc` tag above.
    fn next_chunk(&mut self, terminal: HttpTransportTerminalGuard) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
        let chunk = self.remaining.take();
        Box::pin(async move {
            let _terminal = terminal;
            Ok(chunk)
        })
    }
}

/// 🌐️ Wraps a legacy [`HttpTransport`] (today's ONLY shipped transport) as an [`AsyncHttpTransport`]
/// by running the whole blocking call through [`ComputePool::run_io`] and replaying the
/// buffered result as one [`BufferedHttpBody`] chunk. `runtime`/`scope` are captured at
/// CONSTRUCTION time (unlike `HttpPool::fetch`'s own `runtime`/`scope` parameters) because
/// [`AsyncHttpTransport::begin`] itself takes neither — a transport that needs to reach
/// `ComputePool::run_io` must own that context itself.
// 🔀️ dedyn-fw-os-misc: `TokioHostRuntime`, not `<R: HostAsyncRuntime>` — this is the ONE production
// spawn site for HTTP transport work, and R3 requires Send-ness be obtained STRUCTURALLY (a known
// concrete future type at the spawn site), never by bounding a generic. `TokioHostRuntime` is this
// crate's sole real runtime; `ManualRuntime` (the other `HostAsyncRuntime` impl, testkit-only) never
// constructs a `BlockingHttpTransport` — nothing in this file's tests does. Generalising over `R`
// bought no actual caller and cost exactly the unprovable-Send error this comment replaces.
pub struct BlockingHttpTransport {
    pub(super) transport: Arc<dyn HttpTransport>,
    pub(super) compute: Arc<ComputePool>,
    pub(super) runtime: Arc<TokioHostRuntime>,
    pub(super) scope: ScopeHandle,
}

impl BlockingHttpTransport {
    pub async fn new(transport: Arc<dyn HttpTransport>, compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle) -> BlockingHttpTransport {
        BlockingHttpTransport { transport, compute, runtime, scope }
    }
}

impl AsyncHttpTransport for BlockingHttpTransport {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn begin(&self, ctx: &OperationContext, request: HttpRequest) -> HttpTransportStart {
        let transport = self.transport.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx = ctx.clone();
        let (terminal, terminal_guard) = HttpTransportTerminalHandle::pair();
        let response = Box::pin(async move {
            let result = compute
                .run_io(runtime.as_ref(), &scope, ctx, move || {
                    let _terminal = terminal_guard;
                    transport.call(request)
                })
                .await;
            match result {
                Ok(Ok(response)) => {
                    let head = HttpResponseHead { status: response.status, headers: response.headers };
                    let body: Box<dyn HttpBody> = Box::new(BufferedHttpBody::new(response.body));
                    Ok((head, body))
                }
                Ok(Err(io_error)) => Err(HttpPoolError::Transport(io_error.to_string())),
                Err(compute_error) => Err(HttpPoolError::Compute(compute_error)),
            }
        });
        HttpTransportStart::new(HttpBodyCancellationHandle::idle(), terminal, response)
    }
}
//#endregion 🌐️HttpPool
