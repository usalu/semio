//! 🗄️ `db_storage` — the pluggable storage substrate seam for the `db` crate family: the trait
//! family (`DbStorage`, `WalStorage`, `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`,
//! `IndexStorage`, `LeaseStorage`) every backend (this crate's `MemoryStorage`/`FsStorage`, plus
//! the sibling `db_storage_sqlite`/`db_storage_postgres`/`db_storage_neo4j` crates) implements
//! identically, so `db_engine`/the `db` facade select a backend via `Arc<dyn DbStorage>` at
//! `Database::open` rather than at compile time. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: this crate has no opinion on WAL record framing (`db_wal` reuses
//! `protocol::{SprWriter, FrameCursor, recover}` on top of `WalStorage`'s raw segment bytes) or
//! snapshot pack-file structure (`db_snapshot` builds `.spk` bytes handed to `SnapshotStorage`
//! whole) — every trait here stores/retrieves opaque byte blobs keyed by document + a small
//! integer, plus the two primitives (`CatalogStorage::cas_root`, `LeaseStorage`) that need
//! fencing semantics of their own. This keeps the trait family stable while the format built on
//! top of it (owned by `db_wal`/`db_snapshot`/`db_index`) can evolve independently.
//!
//! ⏳️ **Async-first (design ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet W6)**:
//! every sub-trait method returns a [`DbFuture`] — a boxed, borrowed, `Send` future — rather than a
//! bare `Result`. This is deliberately NOT `async fn` in trait: `Arc<dyn DbStorage>` is this
//! family's universal consumption shape (selected once at `Database::open`, held behind a trait
//! object everywhere downstream), and AFIT is not dyn-compatible. This mirrors
//! `semio_framework_async::HostFuture` (that type is `'static`; `DbFuture` is `'a`-bound to the
//! call's own borrowed arguments, since every call here is driven to completion by its caller —
//! there is no detached-spawn path on this trait family). Backends split two ways:
//! genuinely-async drivers (`db_storage_postgres`'s `sqlx`, `db_storage_neo4j`'s `neo4rs`) wrap
//! their already-async bodies in `Box::pin(async move { .. })` directly; genuinely-blocking
//! backends (this crate's own `FsStorage`, plus the sibling `db_storage_sqlite`) cross the
//! sync/async boundary via `semio_framework_async::HostAsyncRuntime::run_blocking` and this
//! crate's own dependency-free [`run_blocking_op`] bridge — never a private `tokio::runtime`
//! (this crate names no `tokio` at all; see the repo's "`tokio` only in `🛎️services`" rule).
//! `MemoryStorage` (no real I/O) wraps its bodies in already-`Ready` futures.
//!
//! 🧊️ `FsStorage` (this crate's zero-touch default, behind the default `fs` feature) is native-only
//! (`std::fs`) and `#[cfg(not(target_arch = "wasm32"))]`-gated, mirroring `pack`'s own `pack_io`
//! convention — it compiles to an effectively-empty module on a `wasm32-unknown-unknown` target
//! check. `MemoryStorage` has no such gate and is always available.

use crate::*;
use crate::db_ids::{check_len, DbError, ArtifactId};
use crate::db_durability::{DurabilityClass, EpochFence};
use pack::{ByteRange, ContentHash};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_async::{HostAsyncRuntime, OperationContext, ScopeHandle, TraceId};

//#region 🔖️Limits
/// @emoji 🛡️ Ceiling on any single blob this crate reads into memory in one call (one WAL read
/// range, one snapshot generation, one payload, one index run, one lease record) — validated via
/// `check_len` BEFORE the read buffer is allocated, mirroring `pack_core`'s stated
/// invariant. This crate's own choice (the contract doesn't fix a number): generous enough for a
/// snapshot generation or a large payload, small enough to refuse an obviously-corrupt on-disk
/// length before trying to allocate it.
const MAX_READ_BYTES: u64 = 1024 * 1024 * 1024;
//#endregion 🔖️Limits

//#region 🔖️DbFuture
/// @emoji ⏳️ A boxed, `Send`, borrowed future carrying this crate's own `Result<T, DbError>` — see
/// the module doc's "Async-first" section for why this shape (not `async fn` in trait, not
/// `semio_framework_async::HostFuture`'s `'static` bound) is what every `db_storage` sub-trait
/// method returns.
pub type DbFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, DbError>> + Send + 'a>>;
//#endregion 🔖️DbFuture

//#region 🔖️BlockingBridge
/// @emoji 🌉️ Shared state behind [`OneshotSender`]/[`OneshotReceiver`] — a single `Option<T>` slot
/// plus whatever `Waker` last polled and found it empty.
struct OneshotState<T> {
    value: Option<T>,
    waker: Option<std::task::Waker>,
}

/// @emoji 📤️ The write half of a dependency-free oneshot channel — see [`oneshot`]'s doc for why
/// this crate hand-rolls one instead of depending on `tokio`/`futures`.
pub(crate) struct OneshotSender<T>(Arc<std::sync::Mutex<OneshotState<T>>>);

/// @emoji 📥️ The read half (and the `Future` itself) of a dependency-free oneshot channel.
pub(crate) struct OneshotReceiver<T>(Arc<std::sync::Mutex<OneshotState<T>>>);

/// @emoji 🌉️ A minimal, dependency-free single-value async channel: this crate names no `tokio`
/// and no `futures` (the repo's "`tokio` only in `🛎️services`" rule, plus this crate's own
/// "no external libraries for runtime purposes" discipline), so [`run_blocking_op`] hand-rolls the
/// one primitive it needs — a completion signal from a `HostAsyncRuntime::run_blocking` worker
/// back to the polling async task — rather than pull in a crate for it.
pub(crate) fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let state = Arc::new(std::sync::Mutex::new(OneshotState { value: None, waker: None }));
    (OneshotSender(state.clone()), OneshotReceiver(state))
}

impl<T> OneshotSender<T> {
    /// @emoji 📮️ Delivers `value` and wakes whatever task is currently polling the paired
    /// [`OneshotReceiver`], if any (a receiver that hasn't polled yet simply finds `value` already
    /// there on its first poll).
    pub(crate) fn send(self, value: T) {
        let mut state = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.value = Some(value);
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

impl<T> Future for OneshotReceiver<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<T> {
        let mut state = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match state.value.take() {
            Some(value) => std::task::Poll::Ready(value),
            None => {
                state.waker = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        }
    }
}

/// @emoji 🧱️ Dispatches `work` onto `runtime`'s blocking thread pool (via
/// [`HostAsyncRuntime::run_blocking`], scoped to `scope`) and resolves once `work` completes —
/// `FsStorage`/the sibling `db_storage_sqlite::SqliteStorage`'s ONLY way to run genuinely-blocking
/// I/O (`std::fs`, bundled `rusqlite`) without parking the calling async task's own thread. `ctx`
/// is a synthetic, storage-internal `OperationContext`: this trait family's methods carry no
/// actor/trace identity of their own (that context lives one layer up, at whoever `.await`s this
/// future), so `actor`/`generation`/`trace` are always the zero value here — only `cancel`
/// (inherited from `scope`) is real.
pub(crate) async fn run_blocking_op<T, F>(runtime: &dyn HostAsyncRuntime, scope: &ScopeHandle, work: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = oneshot::<T>();
    let ctx = OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: scope.cancel.clone(), capability: None };
    runtime.run_blocking(
        scope,
        ctx,
        Box::new(move || {
            let result = work();
            tx.send(result);
        }),
    );
    rx.await
}

/// @emoji ✅️ Test-only: every `DbFuture` this crate's own backends (`MemoryStorage`, and
/// `FsStorage` driven by `semio_framework_async::testkit::ManualRuntime`, whose `run_blocking`
/// executes synchronously) hand back is already `Ready` the instant it's first polled — there is
/// no real async wait anywhere in a unit test. This drives one to completion without needing a
/// real executor, mirroring `semio_framework_async`'s own `ManualRuntime` test helper.
#[cfg(test)]
fn block_on_ready<'a, T>(mut fut: DbFuture<'a, T>) -> Result<T, DbError> {
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(&waker);
    match fut.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("db_storage test helper expected an already-ready future"),
    }
}
//#endregion 🔖️BlockingBridge

//#region 🔖️Capabilities
/// @emoji 🎚️ What a concrete `DbStorage` backend actually supports — negotiated once at
/// `Database::open` and folded into `DbCapabilities` alongside enabled Cargo features.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StorageCapabilities {
    /// @emoji 💾️ True iff data written through this backend survives a process restart.
    pub durable: bool,
    /// @emoji 🥇️ The strongest `DurabilityClass` this backend can actually deliver on `sync`.
    pub max_durability: DurabilityClass,
    /// @emoji 🔒️ True iff `WalStorage::sync`/equivalent can force data to physical storage.
    pub supports_fsync: bool,
    /// @emoji ✅️ True iff `CatalogStorage::cas_root`/`LeaseStorage` provide real compare-and-swap
    /// fencing (as opposed to a backend that could only ever serve a single writer).
    pub supports_cas: bool,
}
//#endregion 🔖️Capabilities

//#region 🔖️WalStorage
/// @emoji 📜️ Raw, per-document, per-segment append-only byte storage — `db_wal` frames its own
/// `.spr` records on top of what this trait stores; this trait never interprets a byte written
/// through it. A document's WAL is a sequence of segments identified by a dense `u64` index;
/// exactly one segment (the highest-index one not yet `seal`ed) is ever "active" at a time.
pub trait WalStorage: Send + Sync {
    /// @emoji 🆕️ Creates a new, empty, unsealed segment `index` for `document`. Errors
    /// `AlreadyExists` if `index` already exists for `document`.
    fn create_segment<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()>;

    /// @emoji ➕️ Appends `bytes` to the active segment `index`, returning the segment's new total
    /// length. Errors `NotFound` if the segment doesn't exist, `InvalidArgument` if it is sealed.
    fn append<'a>(&'a self, document: &'a ArtifactId, index: u64, bytes: &'a [u8]) -> DbFuture<'a, u64>;

    /// @emoji 🔒️ Forces everything appended to segment `index` so far to the durability level
    /// implied by `class` — a no-op for `Memory`/`Os` (per `DurabilityClass`'s own
    /// doc: `Os` only promises "handed to the OS", not `fsync`ed), a real flush-to-disk for
    /// `Fsync`/`Quorum` (replication itself is `db_cluster`'s concern, not this trait's).
    fn sync<'a>(&'a self, document: &'a ArtifactId, index: u64, class: DurabilityClass) -> DbFuture<'a, ()>;

    /// @emoji 🏁️ Marks segment `index` sealed: no further `append`/`truncate_tail` may target it.
    /// Errors `NotFound` if the segment doesn't exist. Idempotent if already sealed.
    fn seal<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()>;

    /// @emoji 📖️ Reads `range` of segment `index`'s bytes. Errors `NotFound` if the segment
    /// doesn't exist, `InvalidArgument` if `range` extends past the segment's current length.
    fn read<'a>(&'a self, document: &'a ArtifactId, index: u64, range: ByteRange) -> DbFuture<'a, Vec<u8>>;

    /// @emoji 📏️ The current length in bytes of segment `index`.
    fn segment_len<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, u64>;

    /// @emoji 📋️ Every segment index that exists for `document`, ascending. Empty (not an error)
    /// if `document` has no WAL yet.
    fn list_segments<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>>;

    /// @emoji ✂️ Truncates the ACTIVE (unsealed) segment `index` down to `new_len` bytes — the
    /// crash-recovery primitive for discarding a torn/uncommitted tail write. Errors
    /// `InvalidArgument` if the segment is sealed or if `new_len` exceeds its current length
    /// (this trait never extends a segment via truncation).
    fn truncate_tail<'a>(&'a self, document: &'a ArtifactId, index: u64, new_len: u64) -> DbFuture<'a, ()>;

    /// @emoji 🗑️ Deletes segment `index` entirely (both its bytes and seal marker), e.g. after
    /// `db_compact` has folded it into a later generation. Idempotent if already absent.
    fn delete_segment<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()>;
}
//#endregion 🔖️WalStorage

//#region 🔖️SnapshotStorage
/// @emoji 📸️ Storage for whole snapshot generations — `db_snapshot` hands this trait complete
/// `.spk` pack-file bytes (pages in `KIND_CHUNK`, descriptor in `KIND_SNAPSHOT`) per generation;
/// this trait never parses them, it only persists and retrieves them by `(document, generation)`.
pub trait SnapshotStorage: Send + Sync {
    /// @emoji ✍️ Durably writes `bytes` as generation `generation` of `document`'s snapshot
    /// history. Overwrites if the same `(document, generation)` is written twice (the caller's
    /// responsibility to pick a fresh generation number per the contract's
    /// `Footer.prev_footer_offset` incremental-generation chain).
    fn write_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64, bytes: &'a [u8]) -> DbFuture<'a, ()>;

    /// @emoji 📖️ Reads generation `generation`'s complete bytes. Errors `NotFound` if absent.
    fn read_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64) -> DbFuture<'a, Vec<u8>>;

    /// @emoji 🥇️ The highest generation number stored for `document`, or `None` if it has no
    /// snapshot yet.
    fn latest_generation<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Option<u64>>;

    /// @emoji 📋️ Every generation number stored for `document`, ascending.
    fn list_generations<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>>;

    /// @emoji 🗑️ Deletes generation `generation`, e.g. once `db_compact`'s retention policy
    /// supersedes it. Idempotent if already absent.
    fn delete_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64) -> DbFuture<'a, ()>;
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
/// @emoji 🫙️ Content-addressed blob storage (blake3 CAS), shared across every document — large
/// command/payload bytes referenced from a WAL record or a snapshot page are stored once here and
/// referenced by `ContentHash` everywhere else, so identical payloads never duplicate on disk.
pub trait PayloadStorage: Send + Sync {
    /// @emoji ➕️ Stores `bytes` (if not already present — `put` is idempotent under content
    /// equality) and returns its `blake3` `ContentHash`.
    fn put<'a>(&'a self, bytes: &'a [u8]) -> DbFuture<'a, ContentHash>;

    /// @emoji 📖️ Reads the payload stored under `hash`. Errors `NotFound` if absent.
    fn get<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, Vec<u8>>;

    /// @emoji ❓️ True iff a payload is stored under `hash`, without reading it.
    fn contains<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, bool>;

    /// @emoji 🗑️ Deletes the payload stored under `hash` — `db_compact`'s ref-traced payload GC.
    /// Idempotent if already absent.
    fn delete<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, ()>;

    /// @emoji 📏️ The byte length of the payload stored under `hash`, without reading it. Errors
    /// `NotFound` if absent.
    fn len<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, u64>;
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
/// @emoji 🗂️ The single catalog root blob (the family's document directory: names, ids,
/// metadata — opaque to this crate) with compare-and-swap-by-epoch writes: the split-brain gate
/// per `EpochFence`'s doc. Exactly one root exists per `DbStorage` instance.
pub trait CatalogStorage: Send + Sync {
    /// @emoji 📖️ The current root bytes and the `EpochFence` they were written under, or `None`
    /// if `cas_root` has never succeeded yet (a fresh, empty `DbStorage`).
    fn read_root<'a>(&'a self) -> DbFuture<'a, Option<(Vec<u8>, EpochFence)>>;

    /// @emoji ✅️ Compare-and-swap: succeeds only if `expected` matches the epoch of the currently
    /// stored root (or `EpochFence::INITIAL` if no root has ever been written), in which case the
    /// root becomes `new_bytes` under the next epoch (`expected.next()`), which is returned.
    /// Fails `DbError::Fenced` on any mismatch — a writer that lost leadership never silently
    /// overwrites a newer root.
    fn cas_root<'a>(&'a self, expected: EpochFence, new_bytes: &'a [u8]) -> DbFuture<'a, EpochFence>;
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
/// @emoji 🔍️ Storage for `db_index`'s immutable sorted runs (LSM-lite) — opaque per-document,
/// per-run byte blobs; this trait has no opinion on what's inside a run.
pub trait IndexStorage: Send + Sync {
    /// @emoji ✍️ Durably writes `bytes` as run `run_id` of `document`'s index. Overwrites if the
    /// same `(document, run_id)` is written twice.
    fn write_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64, bytes: &'a [u8]) -> DbFuture<'a, ()>;

    /// @emoji 📖️ Reads run `run_id`'s complete bytes. Errors `NotFound` if absent.
    fn read_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64) -> DbFuture<'a, Vec<u8>>;

    /// @emoji 📋️ Every run id stored for `document`, ascending.
    fn list_runs<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>>;

    /// @emoji 🗑️ Deletes run `run_id`, e.g. after `db_index`'s merge policy folds it into a
    /// larger run. Idempotent if already absent.
    fn delete_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64) -> DbFuture<'a, ()>;
}
//#endregion 🔖️IndexStorage

//#region 🔖️LeaseStorage
/// @emoji ⏳️ One resource's current lease state — `LeaseStorage::current`'s return shape.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LeaseInfo {
    pub resource: String,
    pub holder: String,
    pub fence: EpochFence,
    pub expires_at_ms: u64,
}

/// @emoji ⏳️ Named, TTL'd, fenced ownership leases — `db_cluster`'s shard-ownership + epoch
/// failover primitive. A lease is keyed by an opaque `resource` string (e.g. a shard id); at most
/// one `holder` may hold a given resource's lease at a time, and every successful hand-off (a
/// fresh `acquire` after the previous holder's lease expired) bumps the resource's `EpochFence` so
/// a stale former holder's writes are fenced out by `CatalogStorage`-style checks downstream.
pub trait LeaseStorage: Send + Sync {
    /// @emoji 🤝️ Acquires (or idempotently re-acquires, if `holder` already holds an unexpired
    /// lease on `resource`) `resource` for `holder`, valid until `now_ms + ttl_ms`. Returns the
    /// resource's current `EpochFence` — unchanged on re-acquire by the same holder, bumped
    /// (`.next()`) on a genuine hand-off from an expired or absent lease. Errors `Conflict` if
    /// another holder's lease on `resource` has not yet expired.
    fn acquire<'a>(&'a self, resource: &'a str, holder: &'a str, ttl_ms: u64, now_ms: u64) -> DbFuture<'a, EpochFence>;

    /// @emoji ♻️ Extends `holder`'s existing, unexpired lease on `resource` to `now_ms + ttl_ms`.
    /// `fence` must match the lease's current `EpochFence` (`DbError::Fenced` otherwise) and
    /// `holder` must match the current holder (`DbError::Unauthorized` otherwise). Errors
    /// `NotFound`/`Unavailable` if no lease (or an already-expired one) exists on `resource`.
    fn renew<'a>(&'a self, resource: &'a str, holder: &'a str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> DbFuture<'a, ()>;

    /// @emoji 🕊️ Voluntarily releases `holder`'s lease on `resource` early (`fence` and `holder`
    /// must match, same rules as `renew`), immediately freeing `resource` for another `acquire`
    /// (which will still bump the epoch, since a hand-off occurred).
    fn release<'a>(&'a self, resource: &'a str, holder: &'a str, fence: EpochFence) -> DbFuture<'a, ()>;

    /// @emoji 👀️ The current unexpired lease on `resource` as of `now_ms`, or `None` if unheld or
    /// expired (an expired lease is reported as absent, never as stale data).
    fn current<'a>(&'a self, resource: &'a str, now_ms: u64) -> DbFuture<'a, Option<LeaseInfo>>;
}
//#endregion 🔖️LeaseStorage

//#region 🔖️DbStorage
/// @emoji 🧰️ The umbrella storage substrate handle `db_engine`/the `db` facade hold as
/// `Arc<dyn DbStorage>` (selected at `Database::open`, never compile-time-only per the contract).
/// One concrete backend (`MemoryStorage`, `FsStorage`, or a sibling `db_storage_*` crate)
/// implements every sub-trait plus this accessor trait, typically by implementing each sub-trait
/// directly on itself and returning `self` from every accessor. The accessors and `capabilities`
/// stay synchronous — they are pure, in-memory dispatch/description, never I/O.
pub trait DbStorage: Send + Sync {
    fn wal(&self) -> &dyn WalStorage;
    fn snapshot(&self) -> &dyn SnapshotStorage;
    fn payload(&self) -> &dyn PayloadStorage;
    fn catalog(&self) -> &dyn CatalogStorage;
    fn index(&self) -> &dyn IndexStorage;
    fn lease(&self) -> &dyn LeaseStorage;

    /// @emoji 🎚️ What this concrete backend actually supports.
    fn capabilities(&self) -> StorageCapabilities;
}
//#endregion 🔖️DbStorage

//#region 🔖️Memory
/// @emoji 🧠️ One in-process, non-durable segment of a document's WAL — `bytes` plus whether
/// `seal` has been called on it.
struct MemWalSegment {
    bytes: Vec<u8>,
    sealed: bool,
}

/// @emoji 🧠️ A pure in-memory `DbStorage`: every store is a `Mutex`-guarded map, nothing ever
/// touches a filesystem. Not durable (`capabilities().durable == false`) — the backend for unit
/// tests and `db_testkit`'s deterministic simulation runtime, never for a real deployment. Every
/// trait method body below is synchronous (no real I/O to await), so it is simply wrapped in an
/// already-`Ready` `Box::pin(async move { .. })` per the module doc's "Async-first" section.
#[derive(Default)]
pub struct MemoryStorage {
    wal: std::sync::Mutex<std::collections::HashMap<ArtifactId, std::collections::HashMap<u64, MemWalSegment>>>,
    snapshots: std::sync::Mutex<std::collections::HashMap<ArtifactId, std::collections::HashMap<u64, Vec<u8>>>>,
    payloads: std::sync::Mutex<std::collections::HashMap<ContentHash, Vec<u8>>>,
    catalog: std::sync::Mutex<Option<(Vec<u8>, EpochFence)>>,
    index_runs: std::sync::Mutex<std::collections::HashMap<ArtifactId, std::collections::HashMap<u64, Vec<u8>>>>,
    leases: std::sync::Mutex<std::collections::HashMap<String, LeaseInfo>>,
}

/// @emoji 🩹️ Recovers a `Mutex` guard from a poisoned lock instead of panicking — a single
/// panicking mailbox/actor elsewhere in the family must not turn every other document's storage
/// access into a cascading panic.
fn lock<'a, T>(mutex: &'a std::sync::Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl WalStorage for MemoryStorage {
    fn create_segment<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut wal = lock(&self.wal);
            let segments = wal.entry(document.clone()).or_default();
            if segments.contains_key(&index) {
                return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
            }
            segments.insert(index, MemWalSegment { bytes: Vec::new(), sealed: false });
            Ok(())
        })
    }

    fn append<'a>(&'a self, document: &'a ArtifactId, index: u64, bytes: &'a [u8]) -> DbFuture<'a, u64> {
        Box::pin(async move {
            let mut wal = lock(&self.wal);
            let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            if segment.sealed {
                return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
            }
            segment.bytes.extend_from_slice(bytes);
            Ok(segment.bytes.len() as u64)
        })
    }

    fn sync<'a>(&'a self, _document: &'a ArtifactId, _index: u64, _class: DurabilityClass) -> DbFuture<'a, ()> {
        // 🎯️ Nothing is ever persisted, so every durability class is trivially satisfied in-process.
        Box::pin(async { Ok(()) })
    }

    fn seal<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut wal = lock(&self.wal);
            let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            segment.sealed = true;
            Ok(())
        })
    }

    fn read<'a>(&'a self, document: &'a ArtifactId, index: u64, range: ByteRange) -> DbFuture<'a, Vec<u8>> {
        Box::pin(async move {
            check_len(range.len, MAX_READ_BYTES, "wal_storage::read")?;
            let wal = lock(&self.wal);
            let segment = wal.get(document).and_then(|segments| segments.get(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            let start = range.offset as usize;
            let end = start.checked_add(range.len as usize).ok_or_else(|| DbError::InvalidArgument("wal read range overflows usize".to_string()))?;
            if end > segment.bytes.len() {
                return Err(DbError::InvalidArgument(format!("wal read range {start}..{end} out of bounds (len {})", segment.bytes.len())));
            }
            Ok(segment.bytes[start..end].to_vec())
        })
    }

    fn segment_len<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, u64> {
        Box::pin(async move {
            let wal = lock(&self.wal);
            let segment = wal.get(document).and_then(|segments| segments.get(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            Ok(segment.bytes.len() as u64)
        })
    }

    fn list_segments<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>> {
        Box::pin(async move {
            let wal = lock(&self.wal);
            let mut indices: Vec<u64> = wal.get(document).map(|segments| segments.keys().copied().collect()).unwrap_or_default();
            indices.sort_unstable();
            Ok(indices)
        })
    }

    fn truncate_tail<'a>(&'a self, document: &'a ArtifactId, index: u64, new_len: u64) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut wal = lock(&self.wal);
            let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
            if segment.sealed {
                return Err(DbError::InvalidArgument(format!("cannot truncate sealed wal segment {index}")));
            }
            let new_len = new_len as usize;
            if new_len > segment.bytes.len() {
                return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
            }
            segment.bytes.truncate(new_len);
            Ok(())
        })
    }

    fn delete_segment<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut wal = lock(&self.wal);
            if let Some(segments) = wal.get_mut(document) {
                segments.remove(&index);
            }
            Ok(())
        })
    }
}

impl SnapshotStorage for MemoryStorage {
    fn write_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64, bytes: &'a [u8]) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut snapshots = lock(&self.snapshots);
            snapshots.entry(document.clone()).or_default().insert(generation, bytes.to_vec());
            Ok(())
        })
    }

    fn read_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64) -> DbFuture<'a, Vec<u8>> {
        Box::pin(async move {
            let snapshots = lock(&self.snapshots);
            snapshots.get(document).and_then(|generations| generations.get(&generation)).cloned().ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))
        })
    }

    fn latest_generation<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Option<u64>> {
        Box::pin(async move {
            let snapshots = lock(&self.snapshots);
            Ok(snapshots.get(document).and_then(|generations| generations.keys().max().copied()))
        })
    }

    fn list_generations<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>> {
        Box::pin(async move {
            let snapshots = lock(&self.snapshots);
            let mut generations: Vec<u64> = snapshots.get(document).map(|generations| generations.keys().copied().collect()).unwrap_or_default();
            generations.sort_unstable();
            Ok(generations)
        })
    }

    fn delete_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut snapshots = lock(&self.snapshots);
            if let Some(generations) = snapshots.get_mut(document) {
                generations.remove(&generation);
            }
            Ok(())
        })
    }
}

impl PayloadStorage for MemoryStorage {
    fn put<'a>(&'a self, bytes: &'a [u8]) -> DbFuture<'a, ContentHash> {
        Box::pin(async move {
            check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put")?;
            let hash = ContentHash(*blake3::hash(bytes).as_bytes());
            let mut payloads = lock(&self.payloads);
            payloads.entry(hash).or_insert_with(|| bytes.to_vec());
            Ok(hash)
        })
    }

    fn get<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, Vec<u8>> {
        Box::pin(async move {
            let payloads = lock(&self.payloads);
            payloads.get(hash).cloned().ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
        })
    }

    fn contains<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, bool> {
        Box::pin(async move { Ok(lock(&self.payloads).contains_key(hash)) })
    }

    fn delete<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, ()> {
        Box::pin(async move {
            lock(&self.payloads).remove(hash);
            Ok(())
        })
    }

    fn len<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, u64> {
        Box::pin(async move {
            let payloads = lock(&self.payloads);
            payloads.get(hash).map(|bytes| bytes.len() as u64).ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
        })
    }
}

impl CatalogStorage for MemoryStorage {
    fn read_root<'a>(&'a self) -> DbFuture<'a, Option<(Vec<u8>, EpochFence)>> {
        Box::pin(async move { Ok(lock(&self.catalog).clone()) })
    }

    fn cas_root<'a>(&'a self, expected: EpochFence, new_bytes: &'a [u8]) -> DbFuture<'a, EpochFence> {
        Box::pin(async move {
            check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root")?;
            let mut catalog = lock(&self.catalog);
            let current_fence = catalog.as_ref().map_or(EpochFence::INITIAL, |(_, fence)| *fence);
            expected.check(current_fence)?;
            let new_fence = expected.next();
            *catalog = Some((new_bytes.to_vec(), new_fence));
            Ok(new_fence)
        })
    }
}

impl IndexStorage for MemoryStorage {
    fn write_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64, bytes: &'a [u8]) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut runs = lock(&self.index_runs);
            runs.entry(document.clone()).or_default().insert(run_id, bytes.to_vec());
            Ok(())
        })
    }

    fn read_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64) -> DbFuture<'a, Vec<u8>> {
        Box::pin(async move {
            let runs = lock(&self.index_runs);
            runs.get(document).and_then(|runs| runs.get(&run_id)).cloned().ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))
        })
    }

    fn list_runs<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>> {
        Box::pin(async move {
            let runs = lock(&self.index_runs);
            let mut ids: Vec<u64> = runs.get(document).map(|runs| runs.keys().copied().collect()).unwrap_or_default();
            ids.sort_unstable();
            Ok(ids)
        })
    }

    fn delete_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut runs = lock(&self.index_runs);
            if let Some(runs) = runs.get_mut(document) {
                runs.remove(&run_id);
            }
            Ok(())
        })
    }
}

impl LeaseStorage for MemoryStorage {
    fn acquire<'a>(&'a self, resource: &'a str, holder: &'a str, ttl_ms: u64, now_ms: u64) -> DbFuture<'a, EpochFence> {
        Box::pin(async move {
            let mut leases = lock(&self.leases);
            let fence = match leases.get(resource) {
                Some(info) if now_ms < info.expires_at_ms => {
                    if info.holder != holder {
                        return Err(DbError::Conflict(format!("resource {resource} is leased by another holder")));
                    }
                    info.fence
                }
                Some(info) => info.fence.next(),
                None => EpochFence::INITIAL,
            };
            leases.insert(resource.to_string(), LeaseInfo { resource: resource.to_string(), holder: holder.to_string(), fence, expires_at_ms: now_ms + ttl_ms });
            Ok(fence)
        })
    }

    fn renew<'a>(&'a self, resource: &'a str, holder: &'a str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut leases = lock(&self.leases);
            let info = leases.get_mut(resource).ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
            if now_ms >= info.expires_at_ms {
                return Err(DbError::Unavailable(format!("lease for {resource} already expired")));
            }
            if info.holder != holder {
                return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
            }
            fence.check(info.fence)?;
            info.expires_at_ms = now_ms + ttl_ms;
            Ok(())
        })
    }

    fn release<'a>(&'a self, resource: &'a str, holder: &'a str, fence: EpochFence) -> DbFuture<'a, ()> {
        Box::pin(async move {
            let mut leases = lock(&self.leases);
            let info = leases.get(resource).ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
            if info.holder != holder {
                return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
            }
            fence.check(info.fence)?;
            leases.remove(resource);
            Ok(())
        })
    }

    fn current<'a>(&'a self, resource: &'a str, now_ms: u64) -> DbFuture<'a, Option<LeaseInfo>> {
        Box::pin(async move {
            let leases = lock(&self.leases);
            Ok(leases.get(resource).filter(|info| now_ms < info.expires_at_ms).cloned())
        })
    }
}

impl DbStorage for MemoryStorage {
    fn wal(&self) -> &dyn WalStorage {
        self
    }

    fn snapshot(&self) -> &dyn SnapshotStorage {
        self
    }

    fn payload(&self) -> &dyn PayloadStorage {
        self
    }

    fn catalog(&self) -> &dyn CatalogStorage {
        self
    }

    fn index(&self) -> &dyn IndexStorage {
        self
    }

    fn lease(&self) -> &dyn LeaseStorage {
        self
    }

    fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true }
    }
}
//#endregion 🔖️Memory

//#region 🔖️Fs
/// @emoji 📁️ The zero-touch default `DbStorage`: pure files under a root directory, no new C
/// dependency (see module doc for the wasm32/native-only gating rationale). Layout under `root`:
/// `wal/<doc>/segment-<index>.bin` (+ `.sealed` marker once sealed), `snapshot/<doc>/gen-<n>.pack`,
/// `payload/<hash[0..2]>/<hash>.bin` (blake3 CAS, two-hex-char sharding), `catalog/root.bin`
/// (8-byte LE epoch prefix + opaque bytes), `index/<doc>/run-<id>.bin`, `lease/<resource>.bin`.
/// Every multi-step write (`cas_root`, lease grants, snapshot/index/payload writes) goes through
/// `pack::write_atomic` (temp file + `fsync` + `rename`) so a reader never observes a torn file.
/// Every trait method here is genuinely blocking (`std::fs`), so every body runs through
/// [`run_blocking_op`] on the `HostAsyncRuntime` handed to [`FsStorage::open`] rather than ever
/// blocking the calling async task's own thread — see the module doc's "Async-first" section.
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
mod fs_storage {
    use super::{run_blocking_op, ByteRange, ContentHash, DbError, DbFuture, DbStorage, ArtifactId, DurabilityClass, EpochFence, LeaseInfo, MAX_READ_BYTES};
    use super::{CatalogStorage, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
    use super::check_len;
    use semio_framework_async::{HostAsyncRuntime, ScopeHandle};
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    /// @emoji 🚨️ Wraps a `std::io::Error` into `DbError::Io` — the only place `std::io::Error` is
    /// allowed to appear, per the contract's no-`std::io::Error`-in-public-signatures rule.
    #[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
    fn io_err(err: std::io::Error) -> DbError {
        DbError::Io(err.to_string())
    }

    /// @emoji 🧭️ Maps a `std::io::Error` to `DbError::NotFound(missing())` when it's a missing-file
    /// error, or `DbError::Io` otherwise — used everywhere an open/read/stat is expected to find a
    /// caller-addressed blob that might legitimately not exist yet.
    fn open_err(err: std::io::Error, missing: impl FnOnce() -> String) -> DbError {
        if err.kind() == std::io::ErrorKind::NotFound {
            DbError::NotFound(missing())
        } else {
            io_err(err)
        }
    }

    /// @emoji 🛡️ Rejects a path component that could escape `root` (empty, `.`, `..`, or
    /// containing a path separator/NUL) — every document id, resource name, etc. that becomes a
    /// filesystem path component is validated through this before use.
    fn safe_component(raw: &str) -> Result<&str, DbError> {
        if raw.is_empty() || raw == "." || raw == ".." || raw.contains('/') || raw.contains('\\') || raw.contains('\0') {
            return Err(DbError::InvalidArgument(format!("unsafe storage path component: {raw:?}")));
        }
        Ok(raw)
    }

    fn wal_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("wal").join(safe_component(&document.0)?))
    }

    fn snapshot_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("snapshot").join(safe_component(&document.0)?))
    }

    fn index_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("index").join(safe_component(&document.0)?))
    }

    fn payload_path(root: &Path, hash: &ContentHash) -> PathBuf {
        let hex = hash.to_string();
        root.join("payload").join(&hex[0..2]).join(format!("{hex}.bin"))
    }

    fn catalog_path(root: &Path) -> PathBuf {
        root.join("catalog").join("root.bin")
    }

    fn lease_path(root: &Path, resource: &str) -> Result<PathBuf, DbError> {
        Ok(root.join("lease").join(format!("{}.bin", safe_component(resource)?)))
    }

    fn segment_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.bin"))
    }

    fn sealed_marker_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.sealed"))
    }

    fn generation_path(dir: &Path, generation: u64) -> PathBuf {
        dir.join(format!("gen-{generation:020}.pack"))
    }

    fn run_path(dir: &Path, run_id: u64) -> PathBuf {
        dir.join(format!("run-{run_id:020}.bin"))
    }

    /// @emoji 📋️ Lists `dir`'s entries matching `<prefix><20-digit-number><suffix>`, returning the
    /// parsed numbers ascending. Returns an empty list (not an error) if `dir` doesn't exist yet.
    fn list_indexed_files(dir: &Path, prefix: &str, suffix: &str) -> Result<Vec<u64>, DbError> {
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir).map_err(io_err)? {
            let entry = entry.map_err(io_err)?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(number) = name.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(suffix)) {
                if let Ok(parsed) = number.parse::<u64>() {
                    out.push(parsed);
                }
            }
        }
        out.sort_unstable();
        Ok(out)
    }

    /// @emoji ✍️ `epoch(8 LE) || expires_at_ms(8 LE) || holder_len(varint) || holder` — the lease
    /// file wire encoding, built on `pack`'s own byte writer rather than hand-rolled offsets.
    fn encode_lease(fence: EpochFence, expires_at_ms: u64, holder: &str) -> Vec<u8> {
        let mut writer = pack::ByteWriter::new();
        writer.write_u64_le(fence.epoch);
        writer.write_u64_le(expires_at_ms);
        writer.write_varint_u64(holder.len() as u64);
        writer.write_bytes(holder.as_bytes());
        writer.into_bytes()
    }

    fn decode_lease(bytes: &[u8]) -> Result<(EpochFence, u64, String), DbError> {
        let mut reader = pack::ByteReader::new(bytes);
        let epoch = reader.read_u64_le()?;
        let expires_at_ms = reader.read_u64_le()?;
        let holder_len = reader.read_varint_u64()?;
        check_len(holder_len, MAX_READ_BYTES, "lease_storage::decode holder")?;
        let holder_bytes = reader.read_bytes(holder_len as usize)?;
        let holder = String::from_utf8(holder_bytes.to_vec()).map_err(|_| DbError::Corrupt("lease holder is not valid utf-8".to_string()))?;
        Ok((EpochFence { epoch }, expires_at_ms, holder))
    }

    fn read_lease_file(path: &Path) -> Result<Option<(EpochFence, u64, String)>, DbError> {
        if !path.exists() {
            return Ok(None);
        }
        let bytes = std::fs::read(path).map_err(io_err)?;
        decode_lease(&bytes).map(Some)
    }

    fn write_lease_file(path: &Path, fence: EpochFence, expires_at_ms: u64, holder: &str) -> Result<(), DbError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io_err)?;
        }
        pack::write_atomic(path, &encode_lease(fence, expires_at_ms, holder))?;
        Ok(())
    }

    /// @emoji 📁️ The zero-touch default `DbStorage` backend — see module doc for the on-disk
    /// layout. `catalog_lock`/`lease_lock` serialize this-process's compare-and-swap operations;
    /// see `CatalogStorage`/`LeaseStorage` impls below for why a bare read-verify-write over
    /// `write_atomic` isn't itself enough across OS processes (documented extension seam).
    /// `runtime`/`scope` are what every trait method dispatches its blocking body through (see
    /// module doc); both are `Clone`d into each method's `'static` blocking closure since
    /// `HostAsyncRuntime::run_blocking` requires one.
    pub struct FsStorage {
        root: PathBuf,
        catalog_lock: Arc<Mutex<()>>,
        lease_lock: Arc<Mutex<()>>,
        runtime: Arc<dyn HostAsyncRuntime>,
        scope: ScopeHandle,
    }

    impl FsStorage {
        /// @emoji 🚀️ Opens (creating if absent) a `FsStorage` rooted at `root`, dispatching every
        /// subsequent trait call's blocking body through `runtime`'s `run_blocking`, scoped to
        /// `scope`. The initial `create_dir_all` here is a one-time, small, synchronous mkdir at
        /// construction time — not part of any `DbStorage` trait method's hot path, so (unlike
        /// every trait method below) it does not go through `run_blocking_op`.
        pub fn open(runtime: Arc<dyn HostAsyncRuntime>, scope: ScopeHandle, root: &Path) -> Result<Self, DbError> {
            std::fs::create_dir_all(root).map_err(io_err)?;
            Ok(Self { root: root.to_path_buf(), catalog_lock: Arc::new(Mutex::new(())), lease_lock: Arc::new(Mutex::new(())), runtime, scope })
        }
    }

    /// @emoji 🌉️ The one `HostAsyncRuntime` for a caller that has no runtime of its own to thread
    /// in: a single-threaded, strictly-sequential process (`db_cli`'s one-subcommand-then-exit
    /// binary) or a frozen synchronous entry point (`db_engine::Database::open_at`). `run_blocking`
    /// runs `work` inline on the calling thread rather than spawning a worker — with no second task
    /// in flight there is nothing to protect from blocking, and the resulting `DbFuture` is already
    /// `Ready` the first time its caller polls it.
    ///
    /// 🎯️ Deliberately lives here, beside the `FsStorage` that requires it, rather than being
    /// re-derived per caller: `db_cli` and `db_engine` both need exactly this and nothing more, and
    /// a second copy would be the same bridge maintained twice. A caller that owns a real runtime
    /// (`🛎️services`' `tokio`-backed one) passes it to [`FsStorage::open`] instead and never touches
    /// this — this crate still names no `tokio`.
    pub struct InlineRuntime;

    impl HostAsyncRuntime for InlineRuntime {
        fn open_scope(&self, owner: semio_framework_async::ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
            let cancel = match parent {
                Some(parent) => parent.cancel.child(),
                None => semio_framework_async::CancelToken::root(),
            };
            ScopeHandle { id: semio_framework_async::ScopeId(0), owner, cancel }
        }

        fn spawn_scoped(&self, _scope: &ScopeHandle, _ctx: semio_framework_async::OperationContext, fut: semio_framework_async::HostFuture<()>) {
            crate::db_actor::block_on(fut);
        }

        fn run_blocking(&self, _scope: &ScopeHandle, _ctx: semio_framework_async::OperationContext, work: Box<dyn FnOnce() + Send>) {
            work();
        }

        fn sleep_until(&self, _deadline_ms: u64) -> semio_framework_async::HostFuture<()> {
            Box::pin(std::future::ready(()))
        }

        fn cancel_scope(&self, _owner: &semio_framework_async::ScopeOwner, _grace_ms: u64) -> semio_framework_async::HostFuture<semio_framework_async::ScopeDrainReport> {
            Box::pin(std::future::ready(semio_framework_async::ScopeDrainReport::default()))
        }

        fn now_ms(&self) -> u64 {
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |since| since.as_millis() as u64)
        }
    }

    impl FsStorage {
        /// @emoji 🚀️ [`FsStorage::open`] with an [`InlineRuntime`] and a fresh scope owned by
        /// `owner` already threaded through — the whole async bridge in one call, for the two
        /// callers that have no runtime of their own to pass.
        pub fn open_inline(owner: &'static str, root: &Path) -> Result<Self, DbError> {
            let runtime: Arc<dyn HostAsyncRuntime> = Arc::new(InlineRuntime);
            let scope = runtime.open_scope(semio_framework_async::ScopeOwner::Service(owner), None);
            FsStorage::open(runtime, scope, root)
        }
    }

    impl WalStorage for FsStorage {
        fn create_segment<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    std::fs::create_dir_all(&dir).map_err(io_err)?;
                    let path = segment_path(&dir, index);
                    if path.exists() {
                        return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
                    }
                    std::fs::File::create(&path).map_err(io_err)?;
                    Ok(())
                })
                .await
            })
        }

        fn append<'a>(&'a self, document: &'a ArtifactId, index: u64, bytes: &'a [u8]) -> DbFuture<'a, u64> {
            let root = self.root.clone();
            let document = document.clone();
            let bytes = bytes.to_vec();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    if sealed_marker_path(&dir, index).exists() {
                        return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
                    }
                    let path = segment_path(&dir, index);
                    let mut file = std::fs::OpenOptions::new().append(true).open(&path).map_err(|err| open_err(err, || format!("wal segment {index} for {document} not found")))?;
                    file.write_all(&bytes).map_err(io_err)?;
                    file.metadata().map_err(io_err).map(|meta| meta.len())
                })
                .await
            })
        }

        fn sync<'a>(&'a self, document: &'a ArtifactId, index: u64, class: DurabilityClass) -> DbFuture<'a, ()> {
            // 🎯️ `Memory`/`Os` are satisfied by the ordinary `write(2)` `append` already performed;
            // only `Fsync`/`Quorum` need this trait to force data to physical storage.
            if matches!(class, DurabilityClass::Memory | DurabilityClass::Os) {
                return Box::pin(async { Ok(()) });
            }
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    let file = std::fs::OpenOptions::new().write(true).open(&path).map_err(|err| open_err(err, || format!("wal segment {index} for {document} not found")))?;
                    file.sync_all().map_err(io_err)
                })
                .await
            })
        }

        fn seal<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    if !path.exists() {
                        return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
                    }
                    std::fs::File::create(sealed_marker_path(&dir, index)).map_err(io_err)?;
                    Ok(())
                })
                .await
            })
        }

        fn read<'a>(&'a self, document: &'a ArtifactId, index: u64, range: ByteRange) -> DbFuture<'a, Vec<u8>> {
            if let Err(err) = check_len(range.len, MAX_READ_BYTES, "wal_storage::read") {
                return Box::pin(async move { Err(err) });
            }
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    let mut file = std::fs::File::open(&path).map_err(|err| open_err(err, || format!("wal segment {index} for {document} not found")))?;
                    let current_len = file.metadata().map_err(io_err)?.len();
                    let end = range.offset.checked_add(range.len).ok_or_else(|| DbError::InvalidArgument("wal read range overflows u64".to_string()))?;
                    // 🎯️ Bounds-check against the file's actual length before seeking/reading, so a
                    // too-long range reports `InvalidArgument` (matching `MemoryStorage`'s behavior)
                    // rather than surfacing a raw `UnexpectedEof` as an opaque `DbError::Io`.
                    if end > current_len {
                        return Err(DbError::InvalidArgument(format!("wal read range {}..{end} out of bounds (len {current_len})", range.offset)));
                    }
                    file.seek(SeekFrom::Start(range.offset)).map_err(io_err)?;
                    let mut buf = vec![0u8; range.len as usize];
                    file.read_exact(&mut buf).map_err(io_err)?;
                    Ok(buf)
                })
                .await
            })
        }

        fn segment_len<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, u64> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    std::fs::metadata(&path).map(|meta| meta.len()).map_err(|err| open_err(err, || format!("wal segment {index} for {document} not found")))
                })
                .await
            })
        }

        fn list_segments<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move { run_blocking_op(&*runtime, &scope, move || list_indexed_files(&wal_dir(&root, &document)?, "segment-", ".bin")).await })
        }

        fn truncate_tail<'a>(&'a self, document: &'a ArtifactId, index: u64, new_len: u64) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    if sealed_marker_path(&dir, index).exists() {
                        return Err(DbError::InvalidArgument(format!("cannot truncate sealed wal segment {index}")));
                    }
                    let path = segment_path(&dir, index);
                    let file = std::fs::OpenOptions::new().write(true).open(&path).map_err(|err| open_err(err, || format!("wal segment {index} for {document} not found")))?;
                    let current_len = file.metadata().map_err(io_err)?.len();
                    if new_len > current_len {
                        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
                    }
                    file.set_len(new_len).map_err(io_err)
                })
                .await
            })
        }

        fn delete_segment<'a>(&'a self, document: &'a ArtifactId, index: u64) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    let marker = sealed_marker_path(&dir, index);
                    if marker.exists() {
                        std::fs::remove_file(&marker).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            })
        }
    }

    impl SnapshotStorage for FsStorage {
        fn write_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64, bytes: &'a [u8]) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let bytes = bytes.to_vec();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = snapshot_dir(&root, &document)?;
                    std::fs::create_dir_all(&dir).map_err(io_err)?;
                    pack::write_atomic(&generation_path(&dir, generation), &bytes)?;
                    Ok(())
                })
                .await
            })
        }

        fn read_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64) -> DbFuture<'a, Vec<u8>> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = snapshot_dir(&root, &document)?;
                    let path = generation_path(&dir, generation);
                    let meta = std::fs::metadata(&path).map_err(|err| open_err(err, || format!("snapshot generation {generation} for {document} not found")))?;
                    check_len(meta.len(), MAX_READ_BYTES, "snapshot_storage::read_generation")?;
                    std::fs::read(&path).map_err(io_err)
                })
                .await
            })
        }

        fn latest_generation<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Option<u64>> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move { run_blocking_op(&*runtime, &scope, move || Ok(list_indexed_files(&snapshot_dir(&root, &document)?, "gen-", ".pack")?.into_iter().max())).await })
        }

        fn list_generations<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move { run_blocking_op(&*runtime, &scope, move || list_indexed_files(&snapshot_dir(&root, &document)?, "gen-", ".pack")).await })
        }

        fn delete_generation<'a>(&'a self, document: &'a ArtifactId, generation: u64) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = snapshot_dir(&root, &document)?;
                    let path = generation_path(&dir, generation);
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            })
        }
    }

    impl PayloadStorage for FsStorage {
        fn put<'a>(&'a self, bytes: &'a [u8]) -> DbFuture<'a, ContentHash> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put") {
                return Box::pin(async move { Err(err) });
            }
            let root = self.root.clone();
            let bytes = bytes.to_vec();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let hash = ContentHash(*blake3::hash(&bytes).as_bytes());
                    let path = payload_path(&root, &hash);
                    if !path.exists() {
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent).map_err(io_err)?;
                        }
                        pack::write_atomic(&path, &bytes)?;
                    }
                    Ok(hash)
                })
                .await
            })
        }

        fn get<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, Vec<u8>> {
            let root = self.root.clone();
            let hash = *hash;
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let path = payload_path(&root, &hash);
                    let meta = std::fs::metadata(&path).map_err(|err| open_err(err, || format!("payload {hash} not found")))?;
                    check_len(meta.len(), MAX_READ_BYTES, "payload_storage::get")?;
                    std::fs::read(&path).map_err(io_err)
                })
                .await
            })
        }

        fn contains<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, bool> {
            let root = self.root.clone();
            let hash = *hash;
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move { run_blocking_op(&*runtime, &scope, move || Ok(payload_path(&root, &hash).exists())).await })
        }

        fn delete<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let hash = *hash;
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let path = payload_path(&root, &hash);
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            })
        }

        fn len<'a>(&'a self, hash: &'a ContentHash) -> DbFuture<'a, u64> {
            let root = self.root.clone();
            let hash = *hash;
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let path = payload_path(&root, &hash);
                    std::fs::metadata(&path).map(|meta| meta.len()).map_err(|err| open_err(err, || format!("payload {hash} not found")))
                })
                .await
            })
        }
    }

    impl CatalogStorage for FsStorage {
        fn read_root<'a>(&'a self) -> DbFuture<'a, Option<(Vec<u8>, EpochFence)>> {
            let root = self.root.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move { run_blocking_op(&*runtime, &scope, move || read_root_sync(&root)).await })
        }

        fn cas_root<'a>(&'a self, expected: EpochFence, new_bytes: &'a [u8]) -> DbFuture<'a, EpochFence> {
            if let Err(err) = check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root") {
                return Box::pin(async move { Err(err) });
            }
            let root = self.root.clone();
            let new_bytes = new_bytes.to_vec();
            let catalog_lock = self.catalog_lock.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    // 🎯️ In-process serialization only: two OS processes racing on the same `root` could
                    // both pass this `expected.check` before either renames its `write_atomic` temp file
                    // into place. Genuinely cross-process fencing needs an OS file lock (`flock`) or a
                    // single owning process (which this campaign's `db_cluster` ownership lease already
                    // provides in front of catalog writes) — deliberately left as an extension seam rather
                    // than a half-implemented `flock` here, since `db_cluster` isn't implemented yet either.
                    let _guard = catalog_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let current_fence = read_root_sync(&root)?.map_or(EpochFence::INITIAL, |(_, fence)| fence);
                    expected.check(current_fence)?;
                    let new_fence = expected.next();
                    let mut out = Vec::with_capacity(8 + new_bytes.len());
                    out.extend_from_slice(&new_fence.epoch.to_le_bytes());
                    out.extend_from_slice(&new_bytes);
                    let path = catalog_path(&root);
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent).map_err(io_err)?;
                    }
                    pack::write_atomic(&path, &out)?;
                    Ok(new_fence)
                })
                .await
            })
        }
    }

    /// @emoji 📖️ The blocking body behind `CatalogStorage::read_root` — factored out so
    /// `cas_root` can reuse it under `catalog_lock` without recursing through `run_blocking_op`.
    fn read_root_sync(root: &Path) -> Result<Option<(Vec<u8>, EpochFence)>, DbError> {
        let path = catalog_path(root);
        if !path.exists() {
            return Ok(None);
        }
        let bytes = std::fs::read(&path).map_err(io_err)?;
        if bytes.len() < 8 {
            return Err(DbError::Corrupt("catalog root file is shorter than its 8-byte epoch header".to_string()));
        }
        let mut epoch_bytes = [0u8; 8];
        epoch_bytes.copy_from_slice(&bytes[..8]);
        Ok(Some((bytes[8..].to_vec(), EpochFence { epoch: u64::from_le_bytes(epoch_bytes) })))
    }

    impl IndexStorage for FsStorage {
        fn write_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64, bytes: &'a [u8]) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let bytes = bytes.to_vec();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = index_dir(&root, &document)?;
                    std::fs::create_dir_all(&dir).map_err(io_err)?;
                    pack::write_atomic(&run_path(&dir, run_id), &bytes)?;
                    Ok(())
                })
                .await
            })
        }

        fn read_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64) -> DbFuture<'a, Vec<u8>> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = index_dir(&root, &document)?;
                    let path = run_path(&dir, run_id);
                    let meta = std::fs::metadata(&path).map_err(|err| open_err(err, || format!("index run {run_id} for {document} not found")))?;
                    check_len(meta.len(), MAX_READ_BYTES, "index_storage::read_run")?;
                    std::fs::read(&path).map_err(io_err)
                })
                .await
            })
        }

        fn list_runs<'a>(&'a self, document: &'a ArtifactId) -> DbFuture<'a, Vec<u64>> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move { run_blocking_op(&*runtime, &scope, move || list_indexed_files(&index_dir(&root, &document)?, "run-", ".bin")).await })
        }

        fn delete_run<'a>(&'a self, document: &'a ArtifactId, run_id: u64) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let document = document.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let dir = index_dir(&root, &document)?;
                    let path = run_path(&dir, run_id);
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            })
        }
    }

    impl LeaseStorage for FsStorage {
        fn acquire<'a>(&'a self, resource: &'a str, holder: &'a str, ttl_ms: u64, now_ms: u64) -> DbFuture<'a, EpochFence> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let lease_lock = self.lease_lock.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let path = lease_path(&root, &resource)?;
                    let _guard = lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let fence = match read_lease_file(&path)? {
                        Some((fence, expires_at_ms, existing_holder)) if now_ms < expires_at_ms => {
                            if existing_holder != holder {
                                return Err(DbError::Conflict(format!("resource {resource} is leased by another holder")));
                            }
                            fence
                        }
                        Some((fence, _, _)) => fence.next(),
                        None => EpochFence::INITIAL,
                    };
                    write_lease_file(&path, fence, now_ms + ttl_ms, &holder)?;
                    Ok(fence)
                })
                .await
            })
        }

        fn renew<'a>(&'a self, resource: &'a str, holder: &'a str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let lease_lock = self.lease_lock.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let path = lease_path(&root, &resource)?;
                    let _guard = lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let (current_fence, expires_at_ms, current_holder) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
                    if now_ms >= expires_at_ms {
                        return Err(DbError::Unavailable(format!("lease for {resource} already expired")));
                    }
                    if current_holder != holder {
                        return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
                    }
                    fence.check(current_fence)?;
                    write_lease_file(&path, current_fence, now_ms + ttl_ms, &holder)?;
                    Ok(())
                })
                .await
            })
        }

        fn release<'a>(&'a self, resource: &'a str, holder: &'a str, fence: EpochFence) -> DbFuture<'a, ()> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let lease_lock = self.lease_lock.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let path = lease_path(&root, &resource)?;
                    let _guard = lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let (current_fence, _, current_holder) = read_lease_file(&path)?.ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
                    if current_holder != holder {
                        return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
                    }
                    fence.check(current_fence)?;
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            })
        }

        fn current<'a>(&'a self, resource: &'a str, now_ms: u64) -> DbFuture<'a, Option<LeaseInfo>> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let lease_lock = self.lease_lock.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            Box::pin(async move {
                run_blocking_op(&*runtime, &scope, move || {
                    let path = lease_path(&root, &resource)?;
                    let _guard = lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    match read_lease_file(&path)? {
                        Some((fence, expires_at_ms, holder)) if now_ms < expires_at_ms => Ok(Some(LeaseInfo { resource: resource.clone(), holder, fence, expires_at_ms })),
                        _ => Ok(None),
                    }
                })
                .await
            })
        }
    }

    impl DbStorage for FsStorage {
        fn wal(&self) -> &dyn WalStorage {
            self
        }

        fn snapshot(&self) -> &dyn SnapshotStorage {
            self
        }

        fn payload(&self) -> &dyn PayloadStorage {
            self
        }

        fn catalog(&self) -> &dyn CatalogStorage {
            self
        }

        fn index(&self) -> &dyn IndexStorage {
            self
        }

        fn lease(&self) -> &dyn LeaseStorage {
            self
        }

        fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
        }
    }

}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub use fs_storage::{FsStorage, InlineRuntime};
//#endregion 🔖️Fs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️WalStorage
    fn exercise_wal_storage(storage: &dyn WalStorage) {
        let document: ArtifactId = "doc-wal".into();

        block_on_ready(storage.create_segment(&document, 0)).unwrap();
        assert!(matches!(block_on_ready(storage.create_segment(&document, 0)), Err(DbError::AlreadyExists(_))));

        let len_after_first = block_on_ready(storage.append(&document, 0, b"hello ")).unwrap();
        assert_eq!(len_after_first, 6);
        let len_after_second = block_on_ready(storage.append(&document, 0, b"world")).unwrap();
        assert_eq!(len_after_second, 11);
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).unwrap(), 11);

        let read_back = block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 5 })).unwrap();
        assert_eq!(read_back, b"world");
        assert!(matches!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 100 })), Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.sync(&document, 0, DurabilityClass::Fsync)).unwrap();

        block_on_ready(storage.truncate_tail(&document, 0, 6)).unwrap();
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).unwrap(), 6);
        assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 6 })).unwrap(), b"hello ");

        block_on_ready(storage.create_segment(&document, 1)).unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).unwrap(), vec![0, 1]);

        block_on_ready(storage.seal(&document, 0)).unwrap();
        assert!(matches!(block_on_ready(storage.append(&document, 0, b"!")), Err(DbError::InvalidArgument(_))));
        assert!(matches!(block_on_ready(storage.truncate_tail(&document, 0, 0)), Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.delete_segment(&document, 1)).unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).unwrap(), vec![0]);

        assert!(matches!(block_on_ready(storage.append(&document, 99, b"x")), Err(DbError::NotFound(_))));
    }

    #[test]
    fn memory_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&MemoryStorage::new());
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&fs_scratch("wal_laws"));
    }
    //#endregion 🔖️WalStorage

    //#region 🔖️SnapshotStorage
    fn exercise_snapshot_storage(storage: &dyn SnapshotStorage) {
        let document: ArtifactId = "doc-snap".into();
        assert_eq!(block_on_ready(storage.latest_generation(&document)).unwrap(), None);

        block_on_ready(storage.write_generation(&document, 0, b"gen-zero-bytes")).unwrap();
        block_on_ready(storage.write_generation(&document, 1, b"gen-one-bytes")).unwrap();
        assert_eq!(block_on_ready(storage.list_generations(&document)).unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.latest_generation(&document)).unwrap(), Some(1));
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).unwrap(), b"gen-zero-bytes");

        block_on_ready(storage.write_generation(&document, 0, b"gen-zero-overwritten")).unwrap();
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).unwrap(), b"gen-zero-overwritten");

        block_on_ready(storage.delete_generation(&document, 0)).unwrap();
        assert!(matches!(block_on_ready(storage.read_generation(&document, 0)), Err(DbError::NotFound(_))));
        assert_eq!(block_on_ready(storage.list_generations(&document)).unwrap(), vec![1]);
    }

    #[test]
    fn memory_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&MemoryStorage::new());
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&fs_scratch("snapshot_laws"));
    }
    //#endregion 🔖️SnapshotStorage

    //#region 🔖️PayloadStorage
    fn exercise_payload_storage(storage: &dyn PayloadStorage) {
        let bytes = b"a payload blob that gets content-addressed";
        let hash_a = block_on_ready(storage.put(bytes)).unwrap();
        let hash_b = block_on_ready(storage.put(bytes)).unwrap();
        assert_eq!(hash_a, hash_b, "put is idempotent under content equality");
        assert_eq!(hash_a, ContentHash(*blake3::hash(bytes).as_bytes()));

        assert!(block_on_ready(storage.contains(&hash_a)).unwrap());
        assert_eq!(block_on_ready(storage.get(&hash_a)).unwrap(), bytes);
        assert_eq!(block_on_ready(storage.len(&hash_a)).unwrap(), bytes.len() as u64);

        let other_hash = ContentHash([0xAB; 32]);
        assert!(!block_on_ready(storage.contains(&other_hash)).unwrap());
        assert!(matches!(block_on_ready(storage.get(&other_hash)), Err(DbError::NotFound(_))));

        block_on_ready(storage.delete(&hash_a)).unwrap();
        assert!(!block_on_ready(storage.contains(&hash_a)).unwrap());
        assert!(matches!(block_on_ready(storage.get(&hash_a)), Err(DbError::NotFound(_))));
    }

    #[test]
    fn memory_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&MemoryStorage::new());
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&fs_scratch("payload_laws"));
    }
    //#endregion 🔖️PayloadStorage

    //#region 🔖️CatalogStorage
    fn exercise_catalog_storage(storage: &dyn CatalogStorage) {
        assert_eq!(block_on_ready(storage.read_root()).unwrap(), None);

        let epoch_1 = block_on_ready(storage.cas_root(EpochFence::INITIAL, b"root-v1")).unwrap();
        assert_eq!(epoch_1, EpochFence::INITIAL.next());
        let (bytes, fence) = block_on_ready(storage.read_root()).unwrap().unwrap();
        assert_eq!(bytes, b"root-v1");
        assert_eq!(fence, epoch_1);

        // A stale `expected` (still `INITIAL`, but the root already moved to `epoch_1`) is fenced.
        assert!(matches!(block_on_ready(storage.cas_root(EpochFence::INITIAL, b"root-stale")), Err(DbError::Fenced { .. })));

        let epoch_2 = block_on_ready(storage.cas_root(epoch_1, b"root-v2")).unwrap();
        assert_eq!(epoch_2, epoch_1.next());
        assert_eq!(block_on_ready(storage.read_root()).unwrap().unwrap().0, b"root-v2");
    }

    #[test]
    fn memory_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&MemoryStorage::new());
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&fs_scratch("catalog_laws"));
    }
    //#endregion 🔖️CatalogStorage

    //#region 🔖️IndexStorage
    fn exercise_index_storage(storage: &dyn IndexStorage) {
        let document: ArtifactId = "doc-index".into();
        block_on_ready(storage.write_run(&document, 0, b"run-zero")).unwrap();
        block_on_ready(storage.write_run(&document, 1, b"run-one")).unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.read_run(&document, 1)).unwrap(), b"run-one");

        block_on_ready(storage.delete_run(&document, 0)).unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).unwrap(), vec![1]);
        assert!(matches!(block_on_ready(storage.read_run(&document, 0)), Err(DbError::NotFound(_))));
    }

    #[test]
    fn memory_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&MemoryStorage::new());
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&fs_scratch("index_laws"));
    }
    //#endregion 🔖️IndexStorage

    //#region 🔖️LeaseStorage
    fn exercise_lease_storage(storage: &dyn LeaseStorage) {
        let fence_1 = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 0)).unwrap();
        assert_eq!(fence_1, EpochFence::INITIAL);

        // Re-acquiring the same, unexpired lease by the same holder is idempotent (same fence).
        let fence_reacquire = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 100)).unwrap();
        assert_eq!(fence_reacquire, fence_1);

        // A different holder cannot acquire an unexpired lease.
        assert!(matches!(block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 100)), Err(DbError::Conflict(_))));

        block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 500)).unwrap();
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1.next(), 1_000, 500)), Err(DbError::Fenced { .. })));
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-b", fence_1, 1_000, 500)), Err(DbError::Unauthorized(_))));

        let current = block_on_ready(storage.current("shard-1", 600)).unwrap().unwrap();
        assert_eq!(current.holder, "node-a");
        assert_eq!(current.fence, fence_1);

        // After expiry (renewed at 500 for 1_000ms => expires at 1_500), a different holder can
        // take over, bumping the fence — the fencing law a stale former holder is later rejected by.
        assert_eq!(block_on_ready(storage.current("shard-1", 2_000)).unwrap(), None);
        let fence_2 = block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 2_000)).unwrap();
        assert_eq!(fence_2, fence_1.next());

        // The old holder's stale fence is now rejected.
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 2_100)), Err(DbError::Unauthorized(_))));

        block_on_ready(storage.release("shard-1", "node-b", fence_2)).unwrap();
        assert_eq!(block_on_ready(storage.current("shard-1", 2_100)).unwrap(), None);
        assert!(matches!(block_on_ready(storage.release("shard-1", "node-b", fence_2)), Err(DbError::NotFound(_))));
    }

    #[test]
    fn memory_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&MemoryStorage::new());
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&fs_scratch("lease_laws"));
    }
    //#endregion 🔖️LeaseStorage

    //#region 🔖️DbStorage
    #[test]
    fn memory_storage_db_storage_accessors_and_capabilities() {
        let storage: std::sync::Arc<dyn DbStorage> = std::sync::Arc::new(MemoryStorage::new());
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(storage.wal().create_segment(&document, 0)).unwrap();
        block_on_ready(storage.catalog().cas_root(EpochFence::INITIAL, b"root")).unwrap();

        let capabilities = storage.capabilities();
        assert!(!capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Memory);
        assert!(capabilities.supports_cas);
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_db_storage_accessors_and_capabilities() {
        let storage: std::sync::Arc<dyn DbStorage> = std::sync::Arc::new(fs_scratch("umbrella"));
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(storage.index().write_run(&document, 0, b"run")).unwrap();
        assert_eq!(block_on_ready(storage.index().read_run(&document, 0)).unwrap(), b"run");

        let capabilities = storage.capabilities();
        assert!(capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
        assert!(capabilities.supports_fsync);
    }
    //#endregion 🔖️DbStorage

    //#region 🔖️Fs
    #[cfg(feature = "fs")]
    static SCRATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// @emoji 🎲️ A fresh `FsStorage` rooted at a unique scratch directory under
    /// `std::env::temp_dir()` — no external `tempfile` crate dependency, mirroring `pack_io`'s own
    /// test helper convention. Runs on a fresh `semio_framework_async::testkit::ManualRuntime`
    /// (see that type's doc: its `run_blocking` executes synchronously), so every `DbFuture` this
    /// storage hands back resolves on its very first poll — [`block_on_ready`] above never
    /// actually parks.
    #[cfg(feature = "fs")]
    fn fs_scratch(name: &str) -> FsStorage {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_{name}_{pid}_{counter}"));
        let runtime: Arc<dyn HostAsyncRuntime> = Arc::new(semio_framework_async::testkit::ManualRuntime::new(0));
        let scope = runtime.open_scope(semio_framework_async::ScopeOwner::Service("db_storage_test"), None);
        FsStorage::open(runtime, scope, &dir).unwrap()
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_rejects_unsafe_path_components() {
        let storage = fs_scratch("path_safety");
        let traversal_document: ArtifactId = "../escape".into();
        assert!(matches!(block_on_ready(storage.create_segment(&traversal_document, 0)), Err(DbError::InvalidArgument(_))));

        let separator_document: ArtifactId = "sub/dir".into();
        assert!(matches!(block_on_ready(storage.create_segment(&separator_document, 0)), Err(DbError::InvalidArgument(_))));

        let empty_document: ArtifactId = "".into();
        assert!(matches!(block_on_ready(storage.create_segment(&empty_document, 0)), Err(DbError::InvalidArgument(_))));
    }

    #[cfg(feature = "fs")]
    #[test]
    fn fs_storage_write_atomic_survives_reopen_across_instances() {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_reopen_{pid}_{counter}"));

        {
            let runtime: Arc<dyn HostAsyncRuntime> = Arc::new(semio_framework_async::testkit::ManualRuntime::new(0));
            let scope = runtime.open_scope(semio_framework_async::ScopeOwner::Service("db_storage_test"), None);
            let storage = FsStorage::open(runtime, scope, &dir).unwrap();
            let document: ArtifactId = "doc-reopen".into();
            block_on_ready(storage.write_generation(&document, 0, b"persisted across reopen")).unwrap();
        }
        {
            let runtime: Arc<dyn HostAsyncRuntime> = Arc::new(semio_framework_async::testkit::ManualRuntime::new(0));
            let scope = runtime.open_scope(semio_framework_async::ScopeOwner::Service("db_storage_test"), None);
            let storage = FsStorage::open(runtime, scope, &dir).unwrap();
            let document: ArtifactId = "doc-reopen".into();
            assert_eq!(block_on_ready(storage.read_generation(&document, 0)).unwrap(), b"persisted across reopen");
        }
    }
    //#endregion 🔖️Fs
}
//#endregion 🧪️Tests
