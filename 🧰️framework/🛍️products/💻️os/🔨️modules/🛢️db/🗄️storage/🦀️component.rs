//! 🗄️ `db_storage` — the pluggable storage substrate seam for the `db` crate family: the trait
//! family (`WalStorage`, `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`,
//! `IndexStorage`, `LeaseStorage`) every backend (this crate's `MemoryStorage`/`FsStorage`, plus
//! the sibling `db_storage_sqlite`/`db_storage_postgres`/`db_storage_neo4j` modules) implements
//! identically, so `db_engine`/the `db` facade select a backend via [`DbBackend`] at
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
//! ⏳️ **Async-first, zero-dyn (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet
//! `db-dedyn`, ruling **O1**)**: every sub-trait method is a plain `async fn` returning
//! `Result<T, DbError>` directly — never a boxed `dyn Future` in trait-method return position
//! (ruling **R1**; that shape, `DbFuture<'a, T> = Pin<Box<dyn Future<..> + 'a>>`, was this family's
//! PREVIOUS state and is now deleted). Because `async fn` in a trait is not `dyn`-compatible, the
//! old `Arc<dyn DbStorage>`/`&dyn WalStorage` seams are gone too: [`DbBackend`] is a concrete enum
//! naming every backend (this crate has exactly one layer, so no layering problem — see
//! `//#region 🔖️DbBackend`), and its accessors return concrete facet-ref enums
//! ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead of
//! trait objects. `Send` on a spawned future is therefore obtained STRUCTURALLY, from the concrete
//! enum variant the compiler already knows at every call site — never from a `+ Send` bound on a
//! trait method (ruling **R3**). `#![allow(async_fn_in_trait)]` at the crate root suppresses the
//! resulting "auto trait bounds" lint (ruling **R7**): that lint's suggested fix
//! (`-> impl Future<..> + Send`) is exactly the bound R3 forbids.
//!
//! Backends split two ways: genuinely-async drivers (`db_storage_postgres`'s `sqlx`,
//! `db_storage_neo4j`'s `neo4rs`) simply `.await` their already-async bodies; genuinely-blocking
//! backends (this crate's own `FsStorage`, plus the sibling `db_storage_sqlite`) cross the
//! sync/async boundary via this crate's own dependency-free [`run_blocking_op`] bridge, which
//! submits the blocking body to the ONE process-wide `semio_framework_async::WorkerPool` on
//! `Lane::Io` — never a private `tokio::runtime` (this crate names no `tokio` at all; see the
//! repo's "`tokio` only in `🛎️services`" rule) and never `HostAsyncRuntime::run_blocking` (removed
//! from that trait — Phase 1 of `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`; callers now submit
//! directly to a `WorkerPool`). `MemoryStorage` (no real I/O) simply resolves immediately. A
//! `FsStorage`/`SqliteStorage` opened with no `WorkerPool` at all (`pool: None`, e.g. `db_cli`'s
//! single-shot binary or `db_engine::Database::open_at`'s frozen synchronous entry point) runs its
//! blocking body inline instead — there is nothing to protect from blocking with no second task in
//! flight.
//!
//! 🧊️ `FsStorage` (this crate's zero-touch default, behind the default `fs` feature) is native-only
//! (`std::fs`) and `#[cfg(not(target_arch = "wasm32"))]`-gated, mirroring `pack`'s own `pack_io`
//! convention — it compiles to an effectively-empty module on a `wasm32-unknown-unknown` target
//! check. `MemoryStorage` has no such gate and is always available.

use crate::db_durability::{DurabilityClass, EpochFence};
use crate::db_ids::{check_len, ArtifactId, DbError};
use crate::*;
use pack::{ByteRange, ContentHash};
use semio_framework_async::{Lane, WorkerPool};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// @emoji 📊️ This crate's one blocking-bridge queue depth signal — every [`run_blocking_op`]
/// submission increments it, every completion decrements it, so the Phase 1 exit gate can observe
/// the Io lane's queue never growing unbounded. One process-wide counter (not per-backend): the
/// bridge itself is the shared resource being bounded, regardless of which `DbBackend` arm called it.
static BLOCKING_QUEUE: semio_framework_trace::QueueCounter = semio_framework_trace::QueueCounter::new();

//#region 🔖️Limits
/// @emoji 🛡️ Ceiling on any single blob this crate reads into memory in one call (one WAL read
/// range, one snapshot generation, one payload, one index run, one lease record) — validated via
/// `check_len` BEFORE the read buffer is allocated, mirroring `pack_core`'s stated
/// invariant. This crate's own choice (the contract doesn't fix a number): generous enough for a
/// snapshot generation or a large payload, small enough to refuse an obviously-corrupt on-disk
/// length before trying to allocate it.
const MAX_READ_BYTES: u64 = 1024 * 1024 * 1024;
//#endregion 🔖️Limits

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
// 🚫️async: E1 pure constructor — same hand-rolled-channel shape as `db_actor::oneshot`, called
// from both async contexts (via `.await` on the surrounding fn) and a sync `Box<dyn FnOnce()>`
// work closure (`run_blocking_op`, below) — see R9
pub(crate) fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let state = Arc::new(std::sync::Mutex::new(OneshotState { value: None, waker: None }));
    (OneshotSender(state.clone()), OneshotReceiver(state))
}

impl<T> OneshotSender<T> {
    /// @emoji 📮️ Delivers `value` and wakes whatever task is currently polling the paired
    /// [`OneshotReceiver`], if any (a receiver that hasn't polled yet simply finds `value` already
    /// there on its first poll).
    // 🚫️async: E1 pure accessor, called from a sync `Box<dyn FnOnce()>` work closure — see R9
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

/// @emoji 🧱️ Dispatches `work` onto `pool`'s `Lane::Io` and resolves once `work` completes —
/// `FsStorage`/the sibling `db_storage_sqlite::SqliteStorage`'s ONLY way to run genuinely-blocking
/// I/O (`std::fs`, bundled `rusqlite`) without parking the calling async task's own thread.
/// `pool: None` (no shared `WorkerPool` threaded through, e.g. `db_cli`/`db_engine::Database::open_at`'s
/// frozen synchronous entry point) runs `work` inline instead — there is nothing to protect from
/// blocking with no second task in flight, matching the old `InlineRuntime::run_blocking`'s
/// behavior. [`BLOCKING_QUEUE`] tracks the pooled case's in-flight count for the Phase 1 exit gate;
/// the inline case never queues so it is not counted.
pub(crate) async fn run_blocking_op<T, F>(pool: Option<&WorkerPool>, work: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    match pool {
        Some(pool) => {
            let (tx, rx) = oneshot::<T>();
            BLOCKING_QUEUE.enqueued(0);
            pool.submit(
                Lane::Io,
                Box::new(move || {
                    let result = work();
                    BLOCKING_QUEUE.dequeued(0);
                    tx.send(result);
                }),
            );
            rx.await
        }
        None => work(),
    }
}

/// @emoji ✅️ Test-only sync/async bridge. 🚫️async: E5 executor bridge — poll-once: every future
/// this crate's own backends (`MemoryStorage`, and `FsStorage`/`DbBackend` driven by
/// `semio_framework_async::testkit::ManualRuntime`, whose `run_blocking` executes synchronously)
/// hand back is already `Ready` the instant it's first polled — there is no real async wait
/// anywhere in a unit test. This drives one to completion without needing a real executor,
/// mirroring `semio_framework_async`'s own `ManualRuntime` test helper. The crate's single such
/// bridge (R2 E5: "at most one per crate"); [`block_on_ready`] is a thin `Result`-shaped wrapper
/// over it, not a second one.
#[cfg(test)]
async fn poll_once<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = std::pin::pin!(fut);
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    match fut.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("db_storage test helper expected an already-ready future"),
    }
}

#[cfg(test)]
async fn block_on_ready<T>(fut: impl Future<Output = Result<T, DbError>>) -> Result<T, DbError> {
    poll_once(fut).await
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
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;

    /// @emoji ➕️ Appends `bytes` to the active segment `index`, returning the segment's new total
    /// length. Errors `NotFound` if the segment doesn't exist, `InvalidArgument` if it is sealed.
    async fn append(&self, document: &ArtifactId, index: u64, bytes: &[u8]) -> Result<u64, DbError>;

    /// @emoji 🔒️ Forces everything appended to segment `index` so far to the durability level
    /// implied by `class` — a no-op for `Memory`/`Os` (per `DurabilityClass`'s own
    /// doc: `Os` only promises "handed to the OS", not `fsync`ed), a real flush-to-disk for
    /// `Fsync`/`Quorum` (replication itself is `db_cluster`'s concern, not this trait's).
    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError>;

    /// @emoji 🏁️ Marks segment `index` sealed: no further `append`/`truncate_tail` may target it.
    /// Errors `NotFound` if the segment doesn't exist. Idempotent if already sealed.
    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;

    /// @emoji 📖️ Reads `range` of segment `index`'s bytes. Errors `NotFound` if the segment
    /// doesn't exist, `InvalidArgument` if `range` extends past the segment's current length.
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<Vec<u8>, DbError>;

    /// @emoji 📏️ The current length in bytes of segment `index`.
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError>;

    /// @emoji 📋️ Every segment index that exists for `document`, ascending. Empty (not an error)
    /// if `document` has no WAL yet.
    async fn list_segments(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError>;

    /// @emoji ✂️ Truncates the ACTIVE (unsealed) segment `index` down to `new_len` bytes — the
    /// crash-recovery primitive for discarding a torn/uncommitted tail write. Errors
    /// `InvalidArgument` if the segment is sealed or if `new_len` exceeds its current length
    /// (this trait never extends a segment via truncation).
    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError>;

    /// @emoji 🗑️ Deletes segment `index` entirely (both its bytes and seal marker), e.g. after
    /// `db_compact` has folded it into a later generation. Idempotent if already absent.
    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError>;
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
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: &[u8]) -> Result<(), DbError>;

    /// @emoji 📖️ Reads generation `generation`'s complete bytes. Errors `NotFound` if absent.
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<Vec<u8>, DbError>;

    /// @emoji 🥇️ The highest generation number stored for `document`, or `None` if it has no
    /// snapshot yet.
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError>;

    /// @emoji 📋️ Every generation number stored for `document`, ascending.
    async fn list_generations(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError>;

    /// @emoji 🗑️ Deletes generation `generation`, e.g. once `db_compact`'s retention policy
    /// supersedes it. Idempotent if already absent.
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError>;
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
/// @emoji 🫙️ Content-addressed blob storage (blake3 CAS), shared across every document — large
/// command/payload bytes referenced from a WAL record or a snapshot page are stored once here and
/// referenced by `ContentHash` everywhere else, so identical payloads never duplicate on disk.
pub trait PayloadStorage: Send + Sync {
    /// @emoji ➕️ Stores `bytes` (if not already present — `put` is idempotent under content
    /// equality) and returns its `blake3` `ContentHash`.
    async fn put(&self, bytes: &[u8]) -> Result<ContentHash, DbError>;

    /// @emoji 📖️ Reads the payload stored under `hash`. Errors `NotFound` if absent.
    async fn get(&self, hash: &ContentHash) -> Result<Vec<u8>, DbError>;

    /// @emoji ❓️ True iff a payload is stored under `hash`, without reading it.
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError>;

    /// @emoji 🗑️ Deletes the payload stored under `hash` — `db_compact`'s ref-traced payload GC.
    /// Idempotent if already absent.
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError>;

    /// @emoji 📏️ The byte length of the payload stored under `hash`, without reading it. Errors
    /// `NotFound` if absent.
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError>;
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
/// @emoji 🗂️ The single catalog root blob (the family's document directory: names, ids,
/// metadata — opaque to this crate) with compare-and-swap-by-epoch writes: the split-brain gate
/// per `EpochFence`'s doc. Exactly one root exists per `DbStorage` instance.
pub trait CatalogStorage: Send + Sync {
    /// @emoji 📖️ The current root bytes and the `EpochFence` they were written under, or `None`
    /// if `cas_root` has never succeeded yet (a fresh, empty `DbStorage`).
    async fn read_root(&self) -> Result<Option<(Vec<u8>, EpochFence)>, DbError>;

    /// @emoji ✅️ Compare-and-swap: succeeds only if `expected` matches the epoch of the currently
    /// stored root (or `EpochFence::INITIAL` if no root has ever been written), in which case the
    /// root becomes `new_bytes` under the next epoch (`expected.next()`), which is returned.
    /// Fails `DbError::Fenced` on any mismatch — a writer that lost leadership never silently
    /// overwrites a newer root.
    async fn cas_root(&self, expected: EpochFence, new_bytes: &[u8]) -> Result<EpochFence, DbError>;
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
/// @emoji 🔍️ Storage for `db_index`'s immutable sorted runs (LSM-lite) — opaque per-document,
/// per-run byte blobs; this trait has no opinion on what's inside a run.
pub trait IndexStorage: Send + Sync {
    /// @emoji ✍️ Durably writes `bytes` as run `run_id` of `document`'s index. Overwrites if the
    /// same `(document, run_id)` is written twice.
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: &[u8]) -> Result<(), DbError>;

    /// @emoji 📖️ Reads run `run_id`'s complete bytes. Errors `NotFound` if absent.
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<Vec<u8>, DbError>;

    /// @emoji 📋️ Every run id stored for `document`, ascending.
    async fn list_runs(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError>;

    /// @emoji 🗑️ Deletes run `run_id`, e.g. after `db_index`'s merge policy folds it into a
    /// larger run. Idempotent if already absent.
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError>;
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
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError>;

    /// @emoji ♻️ Extends `holder`'s existing, unexpired lease on `resource` to `now_ms + ttl_ms`.
    /// `fence` must match the lease's current `EpochFence` (`DbError::Fenced` otherwise) and
    /// `holder` must match the current holder (`DbError::Unauthorized` otherwise). Errors
    /// `NotFound`/`Unavailable` if no lease (or an already-expired one) exists on `resource`.
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError>;

    /// @emoji 🕊️ Voluntarily releases `holder`'s lease on `resource` early (`fence` and `holder`
    /// must match, same rules as `renew`), immediately freeing `resource` for another `acquire`
    /// (which will still bump the epoch, since a hand-off occurred).
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError>;

    /// @emoji 👀️ The current unexpired lease on `resource` as of `now_ms`, or `None` if unheld or
    /// expired (an expired lease is reported as absent, never as stale data).
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError>;
}
//#endregion 🔖️LeaseStorage

//#region 🔖️DbBackend
/// @emoji 🧰️ The umbrella storage substrate handle `db_engine`/the `db` facade hold as
/// `Arc<DbBackend>` (selected at `Database::open`, never compile-time-only per the contract).
/// Replaces the old `Arc<dyn DbStorage>` seam per ruling **O1** (drop dyn dispatch): every
/// former `&dyn WalStorage`/etc. accessor becomes a concrete facet-ref enum
/// ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead —
/// `Send`-ness is derived STRUCTURALLY at every spawn site (ruling **R3**) — never via a `+ Send`
/// bound on a trait method. No generic `R: HostAsyncRuntime` parameter (Phase 1 of
/// `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR` deleted it): `Fs`/`Sqlite`/`Fault` never used `R`
/// for anything but the removed `run_blocking` bridge, now [`FsStorage`]/`SqliteStorage`'s own
/// `Option<Arc<WorkerPool>>` field.
pub enum DbBackend {
    Memory(MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(crate::db_storage_neo4j::Neo4jStorage),
    Fault(Box<crate::db_testkit::FaultStorage>),
}

impl DbBackend {
    /// @emoji 🔀️ This backend's [`WalRef`] facet — replaces the old `&dyn WalStorage`.
    pub async fn wal(&self) -> WalRef<'_> {
        match self {
            Self::Memory(s) => WalRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => WalRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => WalRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => WalRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => WalRef::Neo4j(s),
            Self::Fault(s) => WalRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`SnapshotRef`] facet — replaces the old `&dyn SnapshotStorage`.
    pub async fn snapshot(&self) -> SnapshotRef<'_> {
        match self {
            Self::Memory(s) => SnapshotRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => SnapshotRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => SnapshotRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => SnapshotRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => SnapshotRef::Neo4j(s),
            Self::Fault(s) => SnapshotRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`PayloadRef`] facet — replaces the old `&dyn PayloadStorage`.
    pub async fn payload(&self) -> PayloadRef<'_> {
        match self {
            Self::Memory(s) => PayloadRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => PayloadRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => PayloadRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => PayloadRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => PayloadRef::Neo4j(s),
            Self::Fault(s) => PayloadRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`CatalogRef`] facet — replaces the old `&dyn CatalogStorage`.
    pub async fn catalog(&self) -> CatalogRef<'_> {
        match self {
            Self::Memory(s) => CatalogRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => CatalogRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => CatalogRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => CatalogRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => CatalogRef::Neo4j(s),
            Self::Fault(s) => CatalogRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`IndexRef`] facet — replaces the old `&dyn IndexStorage`.
    pub async fn index(&self) -> IndexRef<'_> {
        match self {
            Self::Memory(s) => IndexRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => IndexRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => IndexRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => IndexRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => IndexRef::Neo4j(s),
            Self::Fault(s) => IndexRef::Fault(&**s),
        }
    }

    /// @emoji 🔀️ This backend's [`LeaseRef`] facet — replaces the old `&dyn LeaseStorage`.
    pub async fn lease(&self) -> LeaseRef<'_> {
        match self {
            Self::Memory(s) => LeaseRef::Memory(s),
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => LeaseRef::Fs(s),
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => LeaseRef::Sqlite(s),
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => LeaseRef::Postgres(s),
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => LeaseRef::Neo4j(s),
            Self::Fault(s) => LeaseRef::Fault(&**s),
        }
    }

    /// @emoji 🎚️ What this concrete backend actually supports.
    pub async fn capabilities(&self) -> StorageCapabilities {
        match self {
            Self::Memory(s) => s.capabilities().await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.capabilities().await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.capabilities().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.capabilities().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.capabilities().await,
            // 🔀️ `FaultStorage::capabilities` calls back through `self.inner: DbBackend`
            // (`🧪️testkit`), so this arm is mutually recursive with this very fn — `Box::pin`
            // breaks the otherwise-infinitely-sized future (E0733).
            Self::Fault(s) => Box::pin(s.capabilities()).await,
        }
    }
}
//#endregion 🔖️DbBackend

//#region 🔖️WalRef
/// @emoji 🔀️ [`DbBackend::wal`]'s return shape — the enum that replaces
/// `&dyn WalStorage` per ruling **O1**.
pub enum WalRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> WalStorage for WalRef<'a> {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.create_segment(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.create_segment(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.create_segment(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.create_segment(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.create_segment(document, index).await,
            Self::Fault(s) => Box::pin(s.create_segment(document, index)).await,
        }
    }

    async fn append(&self, document: &ArtifactId, index: u64, bytes: &[u8]) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.append(document, index, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.append(document, index, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.append(document, index, bytes).await,
            Self::Fault(s) => Box::pin(s.append(document, index, bytes)).await,
        }
    }

    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.sync(document, index, class).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.sync(document, index, class).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.sync(document, index, class).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.sync(document, index, class).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.sync(document, index, class).await,
            Self::Fault(s) => Box::pin(s.sync(document, index, class)).await,
        }
    }

    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.seal(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.seal(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.seal(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.seal(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.seal(document, index).await,
            Self::Fault(s) => Box::pin(s.seal(document, index)).await,
        }
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<Vec<u8>, DbError> {
        match self {
            Self::Memory(s) => s.read(document, index, range).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read(document, index, range).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read(document, index, range).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read(document, index, range).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read(document, index, range).await,
            Self::Fault(s) => Box::pin(s.read(document, index, range)).await,
        }
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.segment_len(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.segment_len(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.segment_len(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.segment_len(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.segment_len(document, index).await,
            Self::Fault(s) => Box::pin(s.segment_len(document, index)).await,
        }
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        match self {
            Self::Memory(s) => s.list_segments(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_segments(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_segments(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_segments(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_segments(document).await,
            Self::Fault(s) => Box::pin(s.list_segments(document)).await,
        }
    }

    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.truncate_tail(document, index, new_len).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.truncate_tail(document, index, new_len).await,
            Self::Fault(s) => Box::pin(s.truncate_tail(document, index, new_len)).await,
        }
    }

    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_segment(document, index).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_segment(document, index).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_segment(document, index).await,
            Self::Fault(s) => Box::pin(s.delete_segment(document, index)).await,
        }
    }
}
//#endregion 🔖️WalRef

//#region 🔖️SnapshotRef
/// @emoji 🔀️ [`DbBackend::snapshot`]'s return shape — the enum that replaces
/// `&dyn SnapshotStorage` per ruling **O1**.
pub enum SnapshotRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> SnapshotStorage for SnapshotRef<'a> {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: &[u8]) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.write_generation(document, generation, bytes).await,
            Self::Fault(s) => Box::pin(s.write_generation(document, generation, bytes)).await,
        }
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<Vec<u8>, DbError> {
        match self {
            Self::Memory(s) => s.read_generation(document, generation).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_generation(document, generation).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_generation(document, generation).await,
            Self::Fault(s) => Box::pin(s.read_generation(document, generation)).await,
        }
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self {
            Self::Memory(s) => s.latest_generation(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.latest_generation(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.latest_generation(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.latest_generation(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.latest_generation(document).await,
            Self::Fault(s) => Box::pin(s.latest_generation(document)).await,
        }
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        match self {
            Self::Memory(s) => s.list_generations(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_generations(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_generations(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_generations(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_generations(document).await,
            Self::Fault(s) => Box::pin(s.list_generations(document)).await,
        }
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_generation(document, generation).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_generation(document, generation).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_generation(document, generation).await,
            Self::Fault(s) => Box::pin(s.delete_generation(document, generation)).await,
        }
    }
}
//#endregion 🔖️SnapshotRef

//#region 🔖️PayloadRef
/// @emoji 🔀️ [`DbBackend::payload`]'s return shape — the enum that replaces
/// `&dyn PayloadStorage` per ruling **O1**.
pub enum PayloadRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> PayloadStorage for PayloadRef<'a> {
    async fn put(&self, bytes: &[u8]) -> Result<ContentHash, DbError> {
        match self {
            Self::Memory(s) => s.put(bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.put(bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.put(bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.put(bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.put(bytes).await,
            Self::Fault(s) => Box::pin(s.put(bytes)).await,
        }
    }

    async fn get(&self, hash: &ContentHash) -> Result<Vec<u8>, DbError> {
        match self {
            Self::Memory(s) => s.get(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.get(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.get(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.get(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.get(hash).await,
            Self::Fault(s) => Box::pin(s.get(hash)).await,
        }
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self {
            Self::Memory(s) => s.contains(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.contains(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.contains(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.contains(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.contains(hash).await,
            Self::Fault(s) => Box::pin(s.contains(hash)).await,
        }
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete(hash).await,
            Self::Fault(s) => Box::pin(s.delete(hash)).await,
        }
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.len(hash).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.len(hash).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.len(hash).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.len(hash).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.len(hash).await,
            Self::Fault(s) => Box::pin(s.len(hash)).await,
        }
    }
}
//#endregion 🔖️PayloadRef

//#region 🔖️CatalogRef
/// @emoji 🔀️ [`DbBackend::catalog`]'s return shape — the enum that replaces
/// `&dyn CatalogStorage` per ruling **O1**.
pub enum CatalogRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> CatalogStorage for CatalogRef<'a> {
    async fn read_root(&self) -> Result<Option<(Vec<u8>, EpochFence)>, DbError> {
        match self {
            Self::Memory(s) => s.read_root().await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_root().await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_root().await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_root().await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_root().await,
            // 🔀️ `FaultStorage::read_root` calls back through `self.inner: DbBackend`, so this
            // arm is mutually recursive with this very fn — `Box::pin` breaks the otherwise-
            // infinitely-sized future (E0733), same as `DbBackend::capabilities`'s `Fault` arm.
            Self::Fault(s) => Box::pin(s.read_root()).await,
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: &[u8]) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.cas_root(expected, new_bytes).await,
            Self::Fault(s) => Box::pin(s.cas_root(expected, new_bytes)).await,
        }
    }
}
//#endregion 🔖️CatalogRef

//#region 🔖️IndexRef
/// @emoji 🔀️ [`DbBackend::index`]'s return shape — the enum that replaces
/// `&dyn IndexStorage` per ruling **O1**.
pub enum IndexRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> IndexStorage for IndexRef<'a> {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: &[u8]) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.write_run(document, run_id, bytes).await,
            Self::Fault(s) => Box::pin(s.write_run(document, run_id, bytes)).await,
        }
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<Vec<u8>, DbError> {
        match self {
            Self::Memory(s) => s.read_run(document, run_id).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.read_run(document, run_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.read_run(document, run_id).await,
            Self::Fault(s) => Box::pin(s.read_run(document, run_id)).await,
        }
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        match self {
            Self::Memory(s) => s.list_runs(document).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.list_runs(document).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.list_runs(document).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.list_runs(document).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.list_runs(document).await,
            Self::Fault(s) => Box::pin(s.list_runs(document)).await,
        }
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_run(document, run_id).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.delete_run(document, run_id).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.delete_run(document, run_id).await,
            Self::Fault(s) => Box::pin(s.delete_run(document, run_id)).await,
        }
    }
}
//#endregion 🔖️IndexRef

//#region 🔖️LeaseRef
/// @emoji 🔀️ [`DbBackend::lease`]'s return shape — the enum that replaces
/// `&dyn LeaseStorage` per ruling **O1**.
pub enum LeaseRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    Fs(&'a FsStorage),
    #[cfg(feature = "sqlite")]
    Sqlite(&'a crate::db_storage_sqlite::SqliteStorage),
    #[cfg(feature = "postgres")]
    Postgres(&'a crate::db_storage_postgres::PostgresStorage),
    #[cfg(feature = "neo4j")]
    Neo4j(&'a crate::db_storage_neo4j::Neo4jStorage),
    Fault(&'a crate::db_testkit::FaultStorage),
}

impl<'a> LeaseStorage for LeaseRef<'a> {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            Self::Fault(s) => Box::pin(s.acquire(resource, holder, ttl_ms, now_ms)).await,
        }
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            Self::Fault(s) => Box::pin(s.renew(resource, holder, fence, ttl_ms, now_ms)).await,
        }
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.release(resource, holder, fence).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.release(resource, holder, fence).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.release(resource, holder, fence).await,
            Self::Fault(s) => Box::pin(s.release(resource, holder, fence)).await,
        }
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self {
            Self::Memory(s) => s.current(resource, now_ms).await,
            #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
            Self::Fs(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "sqlite")]
            Self::Sqlite(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "postgres")]
            Self::Postgres(s) => s.current(resource, now_ms).await,
            #[cfg(feature = "neo4j")]
            Self::Neo4j(s) => s.current(resource, now_ms).await,
            Self::Fault(s) => Box::pin(s.current(resource, now_ms)).await,
        }
    }
}
//#endregion 🔖️LeaseRef

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
/// already-`Ready` `{ .. }` per the module doc's "Async-first" section.
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
// 🚫️async: E1 pure accessor (no suspension) — see R9
fn lock<'a, T>(mutex: &'a std::sync::Mutex<T>) -> std::sync::MutexGuard<'a, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl MemoryStorage {
    pub async fn new() -> Self {
        Self::default()
    }
}

impl WalStorage for MemoryStorage {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let mut wal = lock(&self.wal);
        let segments = wal.entry(document.clone()).or_default();
        if segments.contains_key(&index) {
            return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
        }
        segments.insert(index, MemWalSegment { bytes: Vec::new(), sealed: false });
        Ok(())
    }

    async fn append(&self, document: &ArtifactId, index: u64, bytes: &[u8]) -> Result<u64, DbError> {
        let mut wal = lock(&self.wal);
        let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        if segment.sealed {
            return Err(DbError::InvalidArgument(format!("cannot append to sealed wal segment {index}")));
        }
        segment.bytes.extend_from_slice(bytes);
        Ok(segment.bytes.len() as u64)
    }

    async fn sync(&self, _document: &ArtifactId, _index: u64, _class: DurabilityClass) -> Result<(), DbError> {
        // 🎯️ Nothing is ever persisted, so every durability class is trivially satisfied in-process.
        {
            Ok(())
        }
    }

    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let mut wal = lock(&self.wal);
        let segment = wal.get_mut(document).and_then(|segments| segments.get_mut(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        segment.sealed = true;
        Ok(())
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<Vec<u8>, DbError> {
        check_len(range.len, MAX_READ_BYTES, "wal_storage::read")?;
        let wal = lock(&self.wal);
        let segment = wal.get(document).and_then(|segments| segments.get(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        let start = range.offset as usize;
        let end = start.checked_add(range.len as usize).ok_or_else(|| DbError::InvalidArgument("wal read range overflows usize".to_string()))?;
        if end > segment.bytes.len() {
            return Err(DbError::InvalidArgument(format!("wal read range {start}..{end} out of bounds (len {})", segment.bytes.len())));
        }
        Ok(segment.bytes[start..end].to_vec())
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let wal = lock(&self.wal);
        let segment = wal.get(document).and_then(|segments| segments.get(&index)).ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        Ok(segment.bytes.len() as u64)
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        let wal = lock(&self.wal);
        let mut indices: Vec<u64> = wal.get(document).map(|segments| segments.keys().copied().collect()).unwrap_or_default();
        indices.sort_unstable();
        Ok(indices)
    }

    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
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
    }

    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let mut wal = lock(&self.wal);
        if let Some(segments) = wal.get_mut(document) {
            segments.remove(&index);
        }
        Ok(())
    }
}

impl SnapshotStorage for MemoryStorage {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: &[u8]) -> Result<(), DbError> {
        let mut snapshots = lock(&self.snapshots);
        snapshots.entry(document.clone()).or_default().insert(generation, bytes.to_vec());
        Ok(())
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<Vec<u8>, DbError> {
        let snapshots = lock(&self.snapshots);
        snapshots.get(document).and_then(|generations| generations.get(&generation)).cloned().ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let snapshots = lock(&self.snapshots);
        Ok(snapshots.get(document).and_then(|generations| generations.keys().max().copied()))
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        let snapshots = lock(&self.snapshots);
        let mut generations: Vec<u64> = snapshots.get(document).map(|generations| generations.keys().copied().collect()).unwrap_or_default();
        generations.sort_unstable();
        Ok(generations)
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        let mut snapshots = lock(&self.snapshots);
        if let Some(generations) = snapshots.get_mut(document) {
            generations.remove(&generation);
        }
        Ok(())
    }
}

impl PayloadStorage for MemoryStorage {
    async fn put(&self, bytes: &[u8]) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put")?;
        let hash = ContentHash(*blake3::hash(bytes).as_bytes());
        let mut payloads = lock(&self.payloads);
        payloads.entry(hash).or_insert_with(|| bytes.to_vec());
        Ok(hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<Vec<u8>, DbError> {
        let payloads = lock(&self.payloads);
        payloads.get(hash).cloned().ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        Ok(lock(&self.payloads).contains_key(hash))
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        lock(&self.payloads).remove(hash);
        Ok(())
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        let payloads = lock(&self.payloads);
        payloads.get(hash).map(|bytes| bytes.len() as u64).ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))
    }
}

impl CatalogStorage for MemoryStorage {
    async fn read_root(&self) -> Result<Option<(Vec<u8>, EpochFence)>, DbError> {
        Ok(lock(&self.catalog).clone())
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: &[u8]) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root")?;
        let mut catalog = lock(&self.catalog);
        let current_fence = catalog.as_ref().map_or(EpochFence::INITIAL, |(_, fence)| *fence);
        expected.check(current_fence)?;
        let new_fence = expected.next();
        *catalog = Some((new_bytes.to_vec(), new_fence));
        Ok(new_fence)
    }
}

impl IndexStorage for MemoryStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: &[u8]) -> Result<(), DbError> {
        let mut runs = lock(&self.index_runs);
        runs.entry(document.clone()).or_default().insert(run_id, bytes.to_vec());
        Ok(())
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<Vec<u8>, DbError> {
        let runs = lock(&self.index_runs);
        runs.get(document).and_then(|runs| runs.get(&run_id)).cloned().ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
        let runs = lock(&self.index_runs);
        let mut ids: Vec<u64> = runs.get(document).map(|runs| runs.keys().copied().collect()).unwrap_or_default();
        ids.sort_unstable();
        Ok(ids)
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        let mut runs = lock(&self.index_runs);
        if let Some(runs) = runs.get_mut(document) {
            runs.remove(&run_id);
        }
        Ok(())
    }
}

impl LeaseStorage for MemoryStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
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
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
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
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        let mut leases = lock(&self.leases);
        let info = leases.get(resource).ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
        if info.holder != holder {
            return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
        }
        fence.check(info.fence)?;
        leases.remove(resource);
        Ok(())
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        let leases = lock(&self.leases);
        Ok(leases.get(resource).filter(|info| now_ms < info.expires_at_ms).cloned())
    }
}

impl MemoryStorage {
    /// @emoji 🎚️ Pure in-memory: never durable, always CAS-capable.
    pub async fn capabilities(&self) -> StorageCapabilities {
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
    use super::check_len;
    use super::{run_blocking_op, ArtifactId, ByteRange, ContentHash, DbError, DurabilityClass, EpochFence, LeaseInfo, MAX_READ_BYTES};
    use super::{CatalogStorage, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
    use semio_framework_async::WorkerPool;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    /// @emoji 🚨️ Wraps a `std::io::Error` into `DbError::Io` — the only place `std::io::Error` is
    /// allowed to appear, per the contract's no-`std::io::Error`-in-public-signatures rule.
    #[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                             // 🚫️async: E4 fn-pointer slot
    fn io_err(err: std::io::Error) -> DbError {
        DbError::Io(err.to_string())
    }

    /// @emoji 🧭️ Maps a `std::io::Error` to `DbError::NotFound(missing())` when it's a missing-file
    /// error, or `DbError::Io` otherwise — used everywhere an open/read/stat is expected to find a
    /// caller-addressed blob that might legitimately not exist yet.
    // 🚫️async: E1 pure accessor called from sync `.map_err(|err| open_err(...))` closures — see R9
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
    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn safe_component(raw: &str) -> Result<&str, DbError> {
        if raw.is_empty() || raw == "." || raw == ".." || raw.contains('/') || raw.contains('\\') || raw.contains('\0') {
            return Err(DbError::InvalidArgument(format!("unsafe storage path component: {raw:?}")));
        }
        Ok(raw)
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn wal_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("wal").join(safe_component(&document.0)?))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn snapshot_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("snapshot").join(safe_component(&document.0)?))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn index_dir(root: &Path, document: &ArtifactId) -> Result<PathBuf, DbError> {
        Ok(root.join("index").join(safe_component(&document.0)?))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn payload_path(root: &Path, hash: &ContentHash) -> PathBuf {
        let hex = hash.to_string();
        root.join("payload").join(&hex[0..2]).join(format!("{hex}.bin"))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn catalog_path(root: &Path) -> PathBuf {
        root.join("catalog").join("root.bin")
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn lease_path(root: &Path, resource: &str) -> Result<PathBuf, DbError> {
        Ok(root.join("lease").join(format!("{}.bin", safe_component(resource)?)))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn segment_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.bin"))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn sealed_marker_path(dir: &Path, index: u64) -> PathBuf {
        dir.join(format!("segment-{index:020}.sealed"))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn generation_path(dir: &Path, generation: u64) -> PathBuf {
        dir.join(format!("gen-{generation:020}.pack"))
    }

    // 🚫️async: E1 pure accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn run_path(dir: &Path, run_id: u64) -> PathBuf {
        dir.join(format!("run-{run_id:020}.bin"))
    }

    /// @emoji 📋️ Lists `dir`'s entries matching `<prefix><20-digit-number><suffix>`, returning the
    /// parsed numbers ascending. Returns an empty list (not an error) if `dir` doesn't exist yet.
    // 🚫️async: E1 pure accessor (blocking `std::fs::read_dir`), every caller is a sync
    // `run_blocking_op` closure — see R9
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
    // 🚫️async: E5 executor bridge — every caller is a sync `run_blocking_op` closure, but
    // `pack::ByteWriter`'s methods are genuinely async (out-of-scope crate); bridged via
    // `db_actor::block_on` rather than making this fn itself async — see R9
    fn encode_lease(fence: EpochFence, expires_at_ms: u64, holder: &str) -> Vec<u8> {
        crate::db_actor::block_on(async {
            let mut writer = pack::ByteWriter::new().await;
            writer.write_u64_le(fence.epoch).await;
            writer.write_u64_le(expires_at_ms).await;
            writer.write_varint_u64(holder.len() as u64).await;
            writer.write_bytes(holder.as_bytes()).await;
            writer.into_bytes().await
        })
    }

    // 🚫️async: E5 executor bridge, same rationale as `encode_lease` — see R9
    fn decode_lease(bytes: &[u8]) -> Result<(EpochFence, u64, String), DbError> {
        crate::db_actor::block_on(async {
            let mut reader = pack::ByteReader::new(bytes).await;
            let epoch = reader.read_u64_le().await?;
            let expires_at_ms = reader.read_u64_le().await?;
            let holder_len = reader.read_varint_u64().await?;
            check_len(holder_len, MAX_READ_BYTES, "lease_storage::decode holder")?;
            let holder_bytes = reader.read_bytes(holder_len as usize).await?;
            let holder = String::from_utf8(holder_bytes.to_vec()).map_err(|_| DbError::Corrupt("lease holder is not valid utf-8".to_string()))?;
            Ok((EpochFence { epoch }, expires_at_ms, holder))
        })
    }

    // 🚫️async: E1 pure-shaped accessor, every caller is a sync `run_blocking_op` closure — see R9
    fn read_lease_file(path: &Path) -> Result<Option<(EpochFence, u64, String)>, DbError> {
        if !path.exists() {
            return Ok(None);
        }
        let bytes = std::fs::read(path).map_err(io_err)?;
        decode_lease(&bytes).map(Some)
    }

    // 🚫️async: E1 pure-shaped accessor, every caller is a sync `run_blocking_op` closure — see R9
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
    /// `write_atomic` isn't itself enough across OS processes (documented extension seam). `pool`
    /// is what every trait method dispatches its blocking body through (see module doc); `Clone`d
    /// (an `Option<Arc<..>>`, cheap either way) into each method's `'static` blocking closure.
    pub struct FsStorage {
        root: PathBuf,
        catalog_lock: Arc<Mutex<()>>,
        lease_lock: Arc<Mutex<()>>,
        pool: Option<Arc<WorkerPool>>,
    }

    impl FsStorage {
        /// @emoji 🚀️ Opens (creating if absent) a `FsStorage` rooted at `root`, dispatching every
        /// subsequent trait call's blocking body through `run_blocking_op` onto `pool`'s
        /// `Lane::Io` (or inline, if `pool` is `None` — see module doc). The initial
        /// `create_dir_all` here is a one-time, small, synchronous mkdir at construction time —
        /// not part of any storage trait method's hot path, so (unlike every trait method below)
        /// it does not go through `run_blocking_op`.
        pub async fn open(pool: Option<Arc<WorkerPool>>, root: &Path) -> Result<Self, DbError> {
            std::fs::create_dir_all(root).map_err(io_err)?;
            Ok(Self { root: root.to_path_buf(), catalog_lock: Arc::new(Mutex::new(())), lease_lock: Arc::new(Mutex::new(())), pool })
        }

        /// @emoji 🎚️ Always durable, `fsync`-capable, CAS-capable — the on-disk default.
        pub async fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
        }

        /// @emoji 🚀️ [`FsStorage::open`] with no `WorkerPool` (`pool: None`, i.e. every trait
        /// method's blocking body runs inline) — the whole bridge in one call, for callers with no
        /// shared pool of their own to pass (`db_cli`'s one-subcommand-then-exit binary,
        /// `db_engine::Database::open_at`'s frozen synchronous entry point).
        pub async fn open_inline(root: &Path) -> Result<Self, DbError> {
            FsStorage::open(None, root).await
        }
    }

    impl WalStorage for FsStorage {
        async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn append(&self, document: &ArtifactId, index: u64, bytes: &[u8]) -> Result<u64, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let bytes = bytes.to_vec();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
            // 🎯️ `Memory`/`Os` are satisfied by the ordinary `write(2)` `append` already performed;
            // only `Fsync`/`Quorum` need this trait to force data to physical storage.
            if matches!(class, DurabilityClass::Memory | DurabilityClass::Os) {
                return { Ok(()) };
            }
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    let file = std::fs::OpenOptions::new().write(true).open(&path).map_err(|err| open_err(err, || format!("wal segment {index} for {document} not found")))?;
                    file.sync_all().map_err(io_err)
                })
                .await
            }
        }

        async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    if !path.exists() {
                        return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
                    }
                    std::fs::File::create(sealed_marker_path(&dir, index)).map_err(io_err)?;
                    Ok(())
                })
                .await
            }
        }

        async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<Vec<u8>, DbError> {
            if let Err(err) = check_len(range.len, MAX_READ_BYTES, "wal_storage::read") {
                return { Err(err) };
            }
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = wal_dir(&root, &document)?;
                    let path = segment_path(&dir, index);
                    std::fs::metadata(&path).map(|meta| meta.len()).map_err(|err| open_err(err, || format!("wal segment {index} for {document} not found")))
                })
                .await
            }
        }

        async fn list_segments(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || list_indexed_files(&wal_dir(&root, &document)?, "segment-", ".bin")).await
            }
        }

        async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }
    }

    impl SnapshotStorage for FsStorage {
        async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: &[u8]) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let bytes = bytes.to_vec();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = snapshot_dir(&root, &document)?;
                    std::fs::create_dir_all(&dir).map_err(io_err)?;
                    pack::write_atomic(&generation_path(&dir, generation), &bytes)?;
                    Ok(())
                })
                .await
            }
        }

        async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<Vec<u8>, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = snapshot_dir(&root, &document)?;
                    let path = generation_path(&dir, generation);
                    let meta = std::fs::metadata(&path).map_err(|err| open_err(err, || format!("snapshot generation {generation} for {document} not found")))?;
                    check_len(meta.len(), MAX_READ_BYTES, "snapshot_storage::read_generation")?;
                    std::fs::read(&path).map_err(io_err)
                })
                .await
            }
        }

        async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || Ok(list_indexed_files(&snapshot_dir(&root, &document)?, "gen-", ".pack")?.into_iter().max())).await
            }
        }

        async fn list_generations(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || list_indexed_files(&snapshot_dir(&root, &document)?, "gen-", ".pack")).await
            }
        }

        async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = snapshot_dir(&root, &document)?;
                    let path = generation_path(&dir, generation);
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            }
        }
    }

    impl PayloadStorage for FsStorage {
        async fn put(&self, bytes: &[u8]) -> Result<ContentHash, DbError> {
            if let Err(err) = check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put") {
                return { Err(err) };
            }
            let root = self.root.clone();
            let bytes = bytes.to_vec();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn get(&self, hash: &ContentHash) -> Result<Vec<u8>, DbError> {
            let root = self.root.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let path = payload_path(&root, &hash);
                    let meta = std::fs::metadata(&path).map_err(|err| open_err(err, || format!("payload {hash} not found")))?;
                    check_len(meta.len(), MAX_READ_BYTES, "payload_storage::get")?;
                    std::fs::read(&path).map_err(io_err)
                })
                .await
            }
        }

        async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
            let root = self.root.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || Ok(payload_path(&root, &hash).exists())).await
            }
        }

        async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
            let root = self.root.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let path = payload_path(&root, &hash);
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            }
        }

        async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
            let root = self.root.clone();
            let hash = *hash;
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let path = payload_path(&root, &hash);
                    std::fs::metadata(&path).map(|meta| meta.len()).map_err(|err| open_err(err, || format!("payload {hash} not found")))
                })
                .await
            }
        }
    }

    impl CatalogStorage for FsStorage {
        async fn read_root(&self) -> Result<Option<(Vec<u8>, EpochFence)>, DbError> {
            let root = self.root.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || read_root_sync(&root)).await
            }
        }

        async fn cas_root(&self, expected: EpochFence, new_bytes: &[u8]) -> Result<EpochFence, DbError> {
            if let Err(err) = check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root") {
                return { Err(err) };
            }
            let root = self.root.clone();
            let new_bytes = new_bytes.to_vec();
            let catalog_lock = self.catalog_lock.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }
    }

    /// @emoji 📖️ The blocking body behind `CatalogStorage::read_root` — factored out so
    /// `cas_root` can reuse it under `catalog_lock` without recursing through `run_blocking_op`.
    // 🚫️async: E1 pure-shaped accessor (blocking `std::fs::read`), every caller is a sync
    // `run_blocking_op` closure — see R9
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
        async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: &[u8]) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let bytes = bytes.to_vec();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = index_dir(&root, &document)?;
                    std::fs::create_dir_all(&dir).map_err(io_err)?;
                    pack::write_atomic(&run_path(&dir, run_id), &bytes)?;
                    Ok(())
                })
                .await
            }
        }

        async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<Vec<u8>, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = index_dir(&root, &document)?;
                    let path = run_path(&dir, run_id);
                    let meta = std::fs::metadata(&path).map_err(|err| open_err(err, || format!("index run {run_id} for {document} not found")))?;
                    check_len(meta.len(), MAX_READ_BYTES, "index_storage::read_run")?;
                    std::fs::read(&path).map_err(io_err)
                })
                .await
            }
        }

        async fn list_runs(&self, document: &ArtifactId) -> Result<Vec<u64>, DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || list_indexed_files(&index_dir(&root, &document)?, "run-", ".bin")).await
            }
        }

        async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
            let root = self.root.clone();
            let document = document.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let dir = index_dir(&root, &document)?;
                    let path = run_path(&dir, run_id);
                    if path.exists() {
                        std::fs::remove_file(&path).map_err(io_err)?;
                    }
                    Ok(())
                })
                .await
            }
        }
    }

    impl LeaseStorage for FsStorage {
        async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let lease_lock = self.lease_lock.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let lease_lock = self.lease_lock.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let holder = holder.to_string();
            let lease_lock = self.lease_lock.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
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
            }
        }

        async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
            let root = self.root.clone();
            let resource = resource.to_string();
            let lease_lock = self.lease_lock.clone();
            let pool = self.pool.clone();
            {
                run_blocking_op(pool.as_deref(), move || {
                    let path = lease_path(&root, &resource)?;
                    let _guard = lease_lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    match read_lease_file(&path)? {
                        Some((fence, expires_at_ms, holder)) if now_ms < expires_at_ms => Ok(Some(LeaseInfo { resource: resource.clone(), holder, fence, expires_at_ms })),
                        _ => Ok(None),
                    }
                })
                .await
            }
        }
    }
}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub use fs_storage::FsStorage;
//#endregion 🔖️Fs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️WalStorage
    async fn exercise_wal_storage(storage: &impl WalStorage) {
        let document: ArtifactId = "doc-wal".into();

        block_on_ready(storage.create_segment(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.create_segment(&document, 0)).await, Err(DbError::AlreadyExists(_))));

        let len_after_first = block_on_ready(storage.append(&document, 0, b"hello ")).await.unwrap();
        assert_eq!(len_after_first, 6);
        let len_after_second = block_on_ready(storage.append(&document, 0, b"world")).await.unwrap();
        assert_eq!(len_after_second, 11);
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 11);

        let read_back = block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 5 })).await.unwrap();
        assert_eq!(read_back, b"world");
        assert!(matches!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 6, len: 100 })).await, Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.sync(&document, 0, DurabilityClass::Fsync)).await.unwrap();

        block_on_ready(storage.truncate_tail(&document, 0, 6)).await.unwrap();
        assert_eq!(block_on_ready(storage.segment_len(&document, 0)).await.unwrap(), 6);
        assert_eq!(block_on_ready(storage.read(&document, 0, ByteRange { offset: 0, len: 6 })).await.unwrap(), b"hello ");

        block_on_ready(storage.create_segment(&document, 1)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0, 1]);

        block_on_ready(storage.seal(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.append(&document, 0, b"!")).await, Err(DbError::InvalidArgument(_))));
        assert!(matches!(block_on_ready(storage.truncate_tail(&document, 0, 0)).await, Err(DbError::InvalidArgument(_))));

        block_on_ready(storage.delete_segment(&document, 1)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_segments(&document)).await.unwrap(), vec![0]);

        assert!(matches!(block_on_ready(storage.append(&document, 99, b"x")).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_wal_storage_laws() {
        exercise_wal_storage(&fs_scratch("wal_laws").await).await;
    }
    //#endregion 🔖️WalStorage

    //#region 🔖️SnapshotStorage
    async fn exercise_snapshot_storage(storage: &impl SnapshotStorage) {
        let document: ArtifactId = "doc-snap".into();
        assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), None);

        block_on_ready(storage.write_generation(&document, 0, b"gen-zero-bytes")).await.unwrap();
        block_on_ready(storage.write_generation(&document, 1, b"gen-one-bytes")).await.unwrap();
        assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.latest_generation(&document)).await.unwrap(), Some(1));
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-bytes");

        block_on_ready(storage.write_generation(&document, 0, b"gen-zero-overwritten")).await.unwrap();
        assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"gen-zero-overwritten");

        block_on_ready(storage.delete_generation(&document, 0)).await.unwrap();
        assert!(matches!(block_on_ready(storage.read_generation(&document, 0)).await, Err(DbError::NotFound(_))));
        assert_eq!(block_on_ready(storage.list_generations(&document)).await.unwrap(), vec![1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_snapshot_storage_laws() {
        exercise_snapshot_storage(&fs_scratch("snapshot_laws").await).await;
    }
    //#endregion 🔖️SnapshotStorage

    //#region 🔖️PayloadStorage
    async fn exercise_payload_storage(storage: &impl PayloadStorage) {
        let bytes = b"a payload blob that gets content-addressed";
        let hash_a = block_on_ready(storage.put(bytes)).await.unwrap();
        let hash_b = block_on_ready(storage.put(bytes)).await.unwrap();
        assert_eq!(hash_a, hash_b, "put is idempotent under content equality");
        assert_eq!(hash_a, ContentHash(*blake3::hash(bytes).as_bytes()));

        assert!(block_on_ready(storage.contains(&hash_a)).await.unwrap());
        assert_eq!(block_on_ready(storage.get(&hash_a)).await.unwrap(), bytes);
        assert_eq!(block_on_ready(storage.len(&hash_a)).await.unwrap(), bytes.len() as u64);

        let other_hash = ContentHash([0xAB; 32]);
        assert!(!block_on_ready(storage.contains(&other_hash)).await.unwrap());
        assert!(matches!(block_on_ready(storage.get(&other_hash)).await, Err(DbError::NotFound(_))));

        block_on_ready(storage.delete(&hash_a)).await.unwrap();
        assert!(!block_on_ready(storage.contains(&hash_a)).await.unwrap());
        assert!(matches!(block_on_ready(storage.get(&hash_a)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_payload_storage_laws() {
        exercise_payload_storage(&fs_scratch("payload_laws").await).await;
    }
    //#endregion 🔖️PayloadStorage

    //#region 🔖️CatalogStorage
    async fn exercise_catalog_storage(storage: &impl CatalogStorage) {
        assert_eq!(block_on_ready(storage.read_root()).await.unwrap(), None);

        let epoch_1 = block_on_ready(storage.cas_root(EpochFence::INITIAL, b"root-v1")).await.unwrap();
        assert_eq!(epoch_1, EpochFence::INITIAL.next());
        let (bytes, fence) = block_on_ready(storage.read_root()).await.unwrap().unwrap();
        assert_eq!(bytes, b"root-v1");
        assert_eq!(fence, epoch_1);

        // A stale `expected` (still `INITIAL`, but the root already moved to `epoch_1`) is fenced.
        assert!(matches!(block_on_ready(storage.cas_root(EpochFence::INITIAL, b"root-stale")).await, Err(DbError::Fenced { .. })));

        let epoch_2 = block_on_ready(storage.cas_root(epoch_1, b"root-v2")).await.unwrap();
        assert_eq!(epoch_2, epoch_1.next());
        assert_eq!(block_on_ready(storage.read_root()).await.unwrap().unwrap().0, b"root-v2");
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_catalog_storage_laws() {
        exercise_catalog_storage(&fs_scratch("catalog_laws").await).await;
    }
    //#endregion 🔖️CatalogStorage

    //#region 🔖️IndexStorage
    async fn exercise_index_storage(storage: &impl IndexStorage) {
        let document: ArtifactId = "doc-index".into();
        block_on_ready(storage.write_run(&document, 0, b"run-zero")).await.unwrap();
        block_on_ready(storage.write_run(&document, 1, b"run-one")).await.unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![0, 1]);
        assert_eq!(block_on_ready(storage.read_run(&document, 1)).await.unwrap(), b"run-one");

        block_on_ready(storage.delete_run(&document, 0)).await.unwrap();
        assert_eq!(block_on_ready(storage.list_runs(&document)).await.unwrap(), vec![1]);
        assert!(matches!(block_on_ready(storage.read_run(&document, 0)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_index_storage_laws() {
        exercise_index_storage(&fs_scratch("index_laws").await).await;
    }
    //#endregion 🔖️IndexStorage

    //#region 🔖️LeaseStorage
    async fn exercise_lease_storage(storage: &impl LeaseStorage) {
        let fence_1 = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 0)).await.unwrap();
        assert_eq!(fence_1, EpochFence::INITIAL);

        // Re-acquiring the same, unexpired lease by the same holder is idempotent (same fence).
        let fence_reacquire = block_on_ready(storage.acquire("shard-1", "node-a", 1_000, 100)).await.unwrap();
        assert_eq!(fence_reacquire, fence_1);

        // A different holder cannot acquire an unexpired lease.
        assert!(matches!(block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 100)).await, Err(DbError::Conflict(_))));

        block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 500)).await.unwrap();
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1.next(), 1_000, 500)).await, Err(DbError::Fenced { .. })));
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-b", fence_1, 1_000, 500)).await, Err(DbError::Unauthorized(_))));

        let current = block_on_ready(storage.current("shard-1", 600)).await.unwrap().unwrap();
        assert_eq!(current.holder, "node-a");
        assert_eq!(current.fence, fence_1);

        // After expiry (renewed at 500 for 1_000ms => expires at 1_500), a different holder can
        // take over, bumping the fence — the fencing law a stale former holder is later rejected by.
        assert_eq!(block_on_ready(storage.current("shard-1", 2_000)).await.unwrap(), None);
        let fence_2 = block_on_ready(storage.acquire("shard-1", "node-b", 1_000, 2_000)).await.unwrap();
        assert_eq!(fence_2, fence_1.next());

        // The old holder's stale fence is now rejected.
        assert!(matches!(block_on_ready(storage.renew("shard-1", "node-a", fence_1, 1_000, 2_100)).await, Err(DbError::Unauthorized(_))));

        block_on_ready(storage.release("shard-1", "node-b", fence_2)).await.unwrap();
        assert_eq!(block_on_ready(storage.current("shard-1", 2_100)).await.unwrap(), None);
        assert!(matches!(block_on_ready(storage.release("shard-1", "node-b", fence_2)).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn memory_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&MemoryStorage::new().await).await;
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_satisfies_lease_storage_laws() {
        exercise_lease_storage(&fs_scratch("lease_laws").await).await;
    }
    //#endregion 🔖️LeaseStorage

    //#region 🔖️DbBackend
    #[semio_framework_async_macros::async_test]
    async fn memory_storage_db_backend_accessors_and_capabilities() {
        let storage: DbBackend = DbBackend::Memory(MemoryStorage::new().await);
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(poll_once(storage.wal()).await.create_segment(&document, 0)).await.unwrap();
        block_on_ready(poll_once(storage.catalog()).await.cas_root(EpochFence::INITIAL, b"root")).await.unwrap();

        let capabilities = poll_once(storage.capabilities()).await;
        assert!(!capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Memory);
        assert!(capabilities.supports_cas);
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_db_backend_accessors_and_capabilities() {
        let storage: DbBackend = DbBackend::Fs(fs_scratch("umbrella").await);
        let document: ArtifactId = "doc-umbrella".into();
        block_on_ready(poll_once(storage.index()).await.write_run(&document, 0, b"run")).await.unwrap();
        assert_eq!(block_on_ready(poll_once(storage.index()).await.read_run(&document, 0)).await.unwrap(), b"run");

        let capabilities = poll_once(storage.capabilities()).await;
        assert!(capabilities.durable);
        assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
        assert!(capabilities.supports_fsync);
    }
    //#endregion 🔖️DbBackend

    //#region 🔖️Fs
    #[cfg(feature = "fs")]
    static SCRATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// @emoji 🎲️ A fresh `FsStorage` rooted at a unique scratch directory under
    /// `std::env::temp_dir()` — no external `tempfile` crate dependency, mirroring `pack_io`'s own
    /// test helper convention. Opened with `pool: None` (every blocking body runs inline), so every
    /// `DbFuture` this storage hands back resolves on its very first poll — [`block_on_ready`]
    /// above never actually parks.
    #[cfg(feature = "fs")]
    async fn fs_scratch(name: &str) -> FsStorage {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_{name}_{pid}_{counter}"));
        poll_once(FsStorage::open(None, &dir)).await.unwrap()
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_rejects_unsafe_path_components() {
        let storage = fs_scratch("path_safety").await;
        let traversal_document: ArtifactId = "../escape".into();
        assert!(matches!(block_on_ready(storage.create_segment(&traversal_document, 0)).await, Err(DbError::InvalidArgument(_))));

        let separator_document: ArtifactId = "sub/dir".into();
        assert!(matches!(block_on_ready(storage.create_segment(&separator_document, 0)).await, Err(DbError::InvalidArgument(_))));

        let empty_document: ArtifactId = "".into();
        assert!(matches!(block_on_ready(storage.create_segment(&empty_document, 0)).await, Err(DbError::InvalidArgument(_))));
    }

    #[cfg(feature = "fs")]
    #[semio_framework_async_macros::async_test]
    async fn fs_storage_write_atomic_survives_reopen_across_instances() {
        let pid = std::process::id();
        let counter = SCRATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("db_storage_test_reopen_{pid}_{counter}"));

        {
            let storage = poll_once(FsStorage::open(None, &dir)).await.unwrap();
            let document: ArtifactId = "doc-reopen".into();
            block_on_ready(storage.write_generation(&document, 0, b"persisted across reopen")).await.unwrap();
        }
        {
            let storage = poll_once(FsStorage::open(None, &dir)).await.unwrap();
            let document: ArtifactId = "doc-reopen".into();
            assert_eq!(block_on_ready(storage.read_generation(&document, 0)).await.unwrap(), b"persisted across reopen");
        }
    }
    //#endregion 🔖️Fs
}
//#endregion 🧪️Tests
