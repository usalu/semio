//! 🗄️ DB fault and deterministic-simulation testing: a seeded
//! deterministic `SimRuntime`/`SimClock` (bounded interleaving explorer for model checks),
//! `FaultStorage` (a scriptable-fault `db_storage::DbStorage` wrapper: fail-nth-write/sync, torn
//! write, fsync-lie, CAS-conflict injection), `CrashHarness` (`run_crash_after_every_write`), splitmix64
//! generators (`CommandGen`/`WorkloadGen` — NOT `proptest`/`quickcheck`, matching `pack_corruption`'s
//! precedent), and the family's cross-crate law assertions (`assert_replay_deterministic`,
//! `assert_snapshot_plus_suffix_equals_replay`, `assert_projection_rebuild_equals_incremental`,
//! `assert_inverse_undo_roundtrip`, `assert_sync_convergence`, `assert_fencing_excludes_stale_writer`,
//! `assert_preview_never_durable`, `assert_overlay_structural_sharing`). Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_fault_testing` row).
//!
//! 🎯️ Design choice (which "real db" each law drives): the `db` facade (`db/rs`) is still an
//! unimplemented stub as of this wave, so the facade-shaped laws that only need `Database::{open_at,
//! create_document, document}`/`ArtifactHandle::{submit, frontier}` (`assert_replay_deterministic`)
//! drive `db_engine::Database` directly — the exact type the facade re-exports verbatim once it
//! lands (see `db_engine`'s own module doc: "the crate that assembles every other `db_*` crate into
//! the stable, contract-frozen `Database`/`ArtifactHandle` API"). Laws needing capabilities
//! `db_engine`'s current `ArtifactHandle` documents as deliberately unreachable this wave (real
//! snapshot publish, real preview publish/read, inverse undo, and projection wiring — see
//! `db_engine`'s own `//🎯️ Design choice (scope)` note) drive `db_artifact::ArtifactEngine`
//! directly instead, one layer below the facade: still the family's real, complete command
//! pipeline (admit → dedupe → authz → conflict → execute → WAL append → durability → publish →
//! project → vcs → preview-reconcile), composing real `db_wal`/`db_storage`/`db_state`/
//! `db_conflict`/`db_preview` underneath it — genuinely "a minimal real db", just not yet reachable
//! through the facade's actor-mailbox boundary. This is a deliberate, documented scope choice, not
//! a workaround: it exercises exactly the same cross-crate machinery the facade will call into once
//! `db_engine` wires the remaining mailbox variants.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::db_durability::Frontier;
use crate::db_ids::DbError;
use crate::*;
use db_storage::{db_io_copy_pages, CatalogStorage, DbBackend, DbIoPages, DbIoU64List, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage};

//#region 🔖️Prng
/// @emoji 🎲️ splitmix64 — see <https://prng.di.unimi.it/splitmix64.c>. Small, dependency-free,
/// good enough statistical spread for deterministic test-data/schedule generation (not
/// cryptography) — the same seed always produces the same draw sequence, matching `pack_corruption`'s
/// `RecordValueGen` precedent (this crate hand-rolls its own rather than depending on
/// `pack_corruption`'s, since that generator is `dsl_schema::RecordSpec`-shaped, not
/// `protocol::MutationEnvelope`-shaped).
#[derive(Clone, Debug)]
pub struct SplitMix64(u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> SplitMix64 {
        SplitMix64(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// @emoji 🎯️ A uniform draw in `0..bound`, or `0` if `bound == 0` (never divides by zero).
    pub fn next_range(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            0
        } else {
            self.next_u64() % bound
        }
    }
}
//#endregion 🔖️Prng

//#region 🔖️Generators
/// @emoji 🎲️ Deterministic seeded fabricator for the primitive pieces of a `protocol::
/// MutationEnvelope` (paths, actors, JSON values, operation ids) — the unit `WorkloadGen` builds
/// whole envelopes from.
pub struct CommandGen {
    rng: SplitMix64,
    counter: u64,
}

impl CommandGen {
    pub fn new(seed: u64) -> CommandGen {
        CommandGen { rng: SplitMix64::new(seed), counter: 0 }
    }

    /// @emoji 🛤️ One of a bounded pool of `/`-free path names, so generated workloads have a
    /// realistic amount of path collision (and thus conflict/last-writer activity) rather than
    /// every draw being trivially disjoint.
    pub fn random_path(&mut self) -> String {
        format!("path-{}", self.rng.next_range(1024))
    }

    pub fn random_actor(&mut self) -> protocol::ActorId {
        protocol::ActorId(format!("actor-{}", self.rng.next_range(8)))
    }

    pub fn random_json(&mut self) -> serde_json::Value {
        serde_json::json!(self.rng.next_u64() % 1_000_000)
    }

    /// @emoji 🆔️ A fresh, seed-derived but still call-order-unique operation id — unique because
    /// `counter` (not just the rng draw) is folded in, so two draws never collide even if the rng
    /// itself repeats within one generator's lifetime.
    pub fn next_operation_id(&mut self) -> protocol::MutationId {
        self.counter += 1;
        protocol::MutationId(format!("gen-{:016x}-{}", self.rng.next_u64(), self.counter))
    }
}

/// @emoji 🎲️ Builds whole, self-contained `protocol::MutationEnvelope` sequences from `CommandGen`
/// — the unit the law assertions and `CrashHarness` drive as their workload.
pub struct WorkloadGen(CommandGen);

impl WorkloadGen {
    pub fn new(seed: u64) -> WorkloadGen {
        WorkloadGen(CommandGen::new(seed))
    }

    /// @emoji 🧩️ `count` sequential, mutually DISJOINT-path envelopes (`path-0` .. `path-{count-1}`,
    /// deterministic naming by index, not by the rng draw) with empty `dependencies` — deterministic
    /// given `seed`, and disjoint so the final materialized state never depends on submit order,
    /// which is exactly the property `assert_sync_convergence`/`SimRuntime`'s interleaving tests
    /// need from their workload.
    // 🔀️ `db_artifact::encode_pathmap_json` genuinely awaits (`store::pack_rt::encode_wire_value`,
    // outside this packet's scope) — stays `async fn` even though the rest of this file's
    // Prng/Generators/SimRuntime regions are pure (R9 does not apply to a fn with a real
    // suspension point). The `.map`-chain-vs-sync-closure shape (R10 residue 1) is why this is
    // an explicit `for` loop rather than an `Iterator::map`.
    pub async fn disjoint_batch(&mut self, document: &protocol::ArtifactId, count: usize) -> Vec<protocol::MutationEnvelope> {
        let mut out = Vec::with_capacity(count);
        for index in 0..count {
            let path = format!("path-{index}");
            let value = self.0.random_json();
            let actor = self.0.random_actor();
            let mut payload = serde_json::Map::with_capacity(1);
            payload.insert(path.clone(), value);
            let mut inverse_payload = serde_json::Map::with_capacity(1);
            inverse_payload.insert(path, serde_json::Value::Null);
            out.push(protocol::MutationEnvelope {
                mutation_id: protocol::MutationId(format!("wg-{index}")),
                document_id: document.clone(),
                actor,
                dependencies: Vec::new(),
                diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap_or_default() },
                inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(inverse_payload)).await.unwrap_or_default() },
                timestamp: protocol::HybridLogicalTimestamp::new(0, index as u64),
            });
        }
        out
    }
}
//#endregion 🔖️Generators

//#region 🔖️SimRuntime
/// @emoji ⏱️ A manually-advanced virtual clock — never reads the wall clock, so a `SimRuntime` run
/// is reproducible byte-for-byte from its seed alone.
#[derive(Clone, Copy, Debug, Default)]
pub struct SimClock {
    now_ms: u64,
}

impl SimClock {
    pub fn new() -> SimClock {
        SimClock::default()
    }

    pub fn now_ms(&self) -> u64 {
        self.now_ms
    }

    pub fn advance(&mut self, delta_ms: u64) -> u64 {
        self.now_ms += delta_ms;
        self.now_ms
    }
}

/// @emoji 📋️ One schedulable unit of `SimRuntime` work: a name (surfaced in the run order for
/// assertions/debugging) plus a closure run synchronously when its turn comes. Deliberately not
/// `Send` — `SimRuntime` is a single-threaded cooperative scheduler, so a scheduled action may
/// freely capture `!Send` state (e.g. a `Rc<RefCell<db_artifact::ArtifactEngine>>`, itself `!Send`
/// per the family's convention — see `db_artifact::ArtifactAuthority`'s own doc).
struct SimTask {
    name: String,
    action: Box<dyn FnOnce(&mut SimClock)>,
}

/// @emoji 🎲️ Seeded deterministic scheduler: collects named tasks, then runs them in a
/// seed-derived permutation (Fisher–Yates over the same `SplitMix64` the generators use),
/// advancing `SimClock` by a small seeded jitter between each. The same seed always reproduces the
/// identical interleaving + timing — what makes a failing model-check reproducible from one logged
/// `u64`.
pub struct SimRuntime {
    rng: SplitMix64,
    clock: SimClock,
    tasks: Vec<SimTask>,
}

impl SimRuntime {
    pub fn new(seed: u64) -> SimRuntime {
        SimRuntime { rng: SplitMix64::new(seed), clock: SimClock::new(), tasks: Vec::new() }
    }

    pub fn clock(&self) -> &SimClock {
        &self.clock
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn schedule(&mut self, name: impl Into<String>, action: impl FnOnce(&mut SimClock) + 'static) {
        self.tasks.push(SimTask { name: name.into(), action: Box::new(action) });
    }

    /// @emoji ▶️ Shuffles every scheduled task via a seed-derived Fisher–Yates permutation, then
    /// runs each in that order, advancing the clock by `0..=max_jitter_ms` (also seed-derived)
    /// before each. Returns the task names in the order they actually ran, for assertions.
    pub fn run(mut self, max_jitter_ms: u64) -> Vec<String> {
        let mut tasks = self.tasks;
        for i in (1..tasks.len()).rev() {
            let j = self.rng.next_range((i + 1) as u64) as usize;
            tasks.swap(i, j);
        }
        let mut order = Vec::with_capacity(tasks.len());
        for task in tasks {
            if max_jitter_ms > 0 {
                let jitter = self.rng.next_range(max_jitter_ms + 1);
                self.clock.advance(jitter);
            }
            order.push(task.name.clone());
            (task.action)(&mut self.clock);
        }
        order
    }
}

/// @emoji 🔭️ Bounded interleaving explorer for model checks: derives `permutations` distinct seeds
/// from `base_seed` and calls `run_one(seed)` once per derived seed, collecting whatever
/// caller-chosen observation `run_one` returns (typically a `SimRuntime::run` task order, or a
/// resulting state hash). A panicking assertion inside `run_one` names its own seed in the failure,
/// which is what makes a failing interleaving reproducible from a single logged `u64` — `run_one`
/// owns building its own fresh `SimRuntime`/system-under-test per call, since the "system" (e.g. a
/// `db_artifact::ArtifactEngine`) must start clean for every explored permutation.
pub fn explore_interleavings<T>(base_seed: u64, permutations: u32, mut run_one: impl FnMut(u64) -> T) -> Vec<T> {
    (0..permutations)
        .map(|i| {
            let seed = base_seed.wrapping_add(u64::from(i)).wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            run_one(seed)
        })
        .collect()
}
//#endregion 🔖️SimRuntime

//#region 🔖️FaultStorage
/// @emoji 💥️ One knob of `FaultStorage`'s injectable fault script — every field independently
/// optional/off by default, so a freshly `FaultScript::default()`ed `FaultStorage` behaves exactly
/// like its inner backend.
#[derive(Clone, Copy, Default, Debug)]
pub struct FaultScript {
    /// @emoji 🚫️ The Nth call to `WalStorage::append` (1-indexed, counting EVERY append this
    /// storage sees, including a document's own genesis header write) fails outright with
    /// `DbError::Io` before reaching the inner backend — models a crash where the physical write
    /// never lands at all.
    pub fail_nth_write: Option<u64>,
    /// @emoji ✂️ `(nth, keep_bytes)`: the Nth `append` call forwards only its first `keep_bytes`
    /// bytes to the inner backend (still succeeding, reporting the inner backend's real new
    /// length) — models a crash mid-`write(2)`, the "torn write" fault `db_wal`'s own recovery
    /// (`WalRecoveryReport.torn_tail_bytes`) exists to detect and truncate.
    pub torn_write_at: Option<(u64, u64)>,
    /// @emoji 🚫️ The Nth call to `WalStorage::sync` (1-indexed) fails before reaching the
    /// inner backend, after any preceding append has already completed.
    pub fail_nth_sync: Option<u64>,
    /// @emoji 🤥️ `WalStorage::sync` returns `Ok(())` without ever forwarding to the inner backend —
    /// models a storage device that acknowledges `fsync` without actually forcing data to physical
    /// storage (a caller relying on `DurabilityClass::Fsync` alone, without independently verifying
    /// durability, cannot tell the difference from the return value).
    pub fsync_lies: bool,
    /// @emoji ⚔️ The Nth call to `CatalogStorage::cas_root` (1-indexed) fails with a synthetic
    /// `DbError::Fenced` (as if a competing writer had already advanced the epoch) without ever
    /// touching the inner backend — models exercising a caller's CAS-retry path without needing a
    /// second real writer.
    pub cas_conflict_nth: Option<u64>,
}

/// @emoji 💥️ Wraps a real `db_storage::DbStorage` backend with a scriptable `FaultScript`, injected
/// only at the WAL append/sync and catalog-CAS boundaries (see `FaultScript`'s fields) — every
/// other operation (segment lifecycle, snapshot/payload/index/lease storage) passes straight
/// through to `inner` untouched, since those are outside this testkit's stated crash-simulation
/// scope (fail-nth-write/sync / torn write / fsync-lie / CAS-conflict injection).
pub struct FaultStorage {
    inner: Arc<DbBackend>,
    script: Mutex<FaultScript>,
    append_calls: AtomicU64,
    sync_calls: AtomicU64,
    sync_delegated_calls: AtomicU64,
    cas_calls: AtomicU64,
}

impl FaultStorage {
    pub async fn new(inner: Arc<DbBackend>) -> FaultStorage {
        FaultStorage { inner, script: Mutex::new(FaultScript::default()), append_calls: AtomicU64::new(0), sync_calls: AtomicU64::new(0), sync_delegated_calls: AtomicU64::new(0), cas_calls: AtomicU64::new(0) }
    }

    pub async fn set_script(&self, script: FaultScript) {
        *self.script.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = script;
    }

    async fn script(&self) -> FaultScript {
        *self.script.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// @emoji 🔢️ Every `WalStorage::append` call this storage has seen so far, faulted or not —
    /// `CrashHarness` uses this to discover exactly how many write boundaries a workload has.
    pub async fn append_calls(&self) -> u64 {
        self.append_calls.load(Ordering::SeqCst)
    }

    /// @emoji 🔢️ Every `WalStorage::sync` call this storage has seen, faulted or not.
    pub async fn sync_calls(&self) -> u64 {
        self.sync_calls.load(Ordering::SeqCst)
    }

    /// @emoji 🔢️ How many `WalStorage::sync` calls actually reached the inner backend (as opposed
    /// to being lied about under `FaultScript::fsync_lies`).
    pub async fn sync_delegated_calls(&self) -> u64 {
        self.sync_delegated_calls.load(Ordering::SeqCst)
    }

    pub async fn cas_calls(&self) -> u64 {
        self.cas_calls.load(Ordering::SeqCst)
    }

    /// @emoji 🎚️ Passes the inner backend's own capabilities straight through — fault injection
    /// never changes what the backend claims to support, only what it actually does on a call.
    pub async fn capabilities(&self) -> StorageCapabilities {
        self.inner.capabilities().await
    }
}

impl WalStorage for FaultStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<db_storage::WalWriterPermit, DbError> {
        self.inner.wal().await.acquire_writer(document).await
    }
    async fn create_segment(&self, writer: &db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.create_segment(writer, index).await
    }

    async fn append(&self, writer: &db_storage::WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        let call = self.append_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let script = self.script().await;
        {
            if script.fail_nth_write == Some(call) {
                return Err(DbError::Io(format!("fault_storage: injected failure on wal append #{call}")));
            }
            if let Some((torn_call, keep_bytes)) = script.torn_write_at {
                if torn_call == call {
                    let keep = (keep_bytes as usize).min(bytes.len());
                    let pages = match bytes.try_prefix(keep) {
                        Ok(pages) => pages,
                        Err(_) => return Err(DbError::Internal("fault_storage torn prefix exceeded the retained DB page owner".to_string())),
                    };
                    return self.inner.wal().await.append(writer, index, pages).await;
                }
            }
            self.inner.wal().await.append(writer, index, bytes).await
        }
    }

    async fn sync(&self, writer: &db_storage::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        let call = self.sync_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let script = self.script().await;
        if script.fail_nth_sync == Some(call) {
            return Err(DbError::Io(format!("fault_storage: injected failure on wal sync #{call}")));
        }
        if script.fsync_lies {
            return Ok(());
        }
        self.sync_delegated_calls.fetch_add(1, Ordering::SeqCst);
        self.inner.wal().await.sync(writer, index, class).await
    }

    async fn seal(&self, writer: &db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.seal(writer, index).await
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: pack::ByteRange) -> Result<DbIoPages, DbError> {
        self.inner.wal().await.read(document, index, range).await
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        self.inner.wal().await.segment_len(document, index).await
    }

    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        self.inner.wal().await.segment_state(document, index).await
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.wal().await.list_segments(document).await
    }

    async fn truncate_tail(&self, writer: &db_storage::WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        self.inner.wal().await.truncate_tail(writer, index, new_len).await
    }

    async fn delete_segment(&self, writer: &db_storage::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.delete_segment(writer, index).await
    }
}

impl SnapshotStorage for FaultStorage {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        self.inner.snapshot().await.write_generation(document, generation, bytes).await
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        self.inner.snapshot().await.read_generation(document, generation).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        self.inner.snapshot().await.latest_generation(document).await
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.snapshot().await.list_generations(document).await
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        self.inner.snapshot().await.delete_generation(document, generation).await
    }
}

impl PayloadStorage for FaultStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        self.inner.payload().await.put(bytes).await
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        self.inner.payload().await.get(hash).await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        self.inner.payload().await.contains(hash).await
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        self.inner.payload().await.delete(hash).await
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        self.inner.payload().await.len(hash).await
    }
}

impl CatalogStorage for FaultStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        self.inner.catalog().await.read_root().await
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        let call = self.cas_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let conflict = self.script().await.cas_conflict_nth == Some(call);
        {
            if conflict {
                let current = self.inner.catalog().await.read_root().await?.map_or(EpochFence::INITIAL, |(_, fence)| fence);
                return Err(DbError::Fenced { expected: current.epoch, actual: expected.epoch });
            }
            self.inner.catalog().await.cas_root(expected, new_bytes).await
        }
    }
}

impl IndexStorage for FaultStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        self.inner.index().await.write_run(document, run_id, bytes).await
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        self.inner.index().await.read_run(document, run_id).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.index().await.list_runs(document).await
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        self.inner.index().await.delete_run(document, run_id).await
    }
}

impl LeaseStorage for FaultStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        self.inner.lease().await.acquire(resource, holder, ttl_ms, now_ms).await
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        self.inner.lease().await.renew(resource, holder, fence, ttl_ms, now_ms).await
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        self.inner.lease().await.release(resource, holder, fence).await
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        self.inner.lease().await.current(resource, now_ms).await
    }
}
//#endregion 🔖️FaultStorage

//#region 🔖️CrashHarness
/// @emoji 📋️ What `CrashHarness::run_crash_after_every_write` found across every injected write
/// boundary.
#[derive(Clone, Debug, Default)]
pub struct CrashHarnessReport {
    pub writes_tested: u64,
    /// @emoji 🚨️ `(write index crashed after, error)` for every crash point where reopening the
    /// faulted storage itself errored (recovery must never do this).
    pub reopen_failures: Vec<(u64, String)>,
    /// @emoji 🚨️ Write indices where the reopened, recovered document's state did not match the
    /// expected "exactly the prefix of commands durably committed before the fault" invariant.
    pub state_mismatches: Vec<u64>,
}

impl CrashHarnessReport {
    pub async fn is_clean(&self) -> bool {
        self.reopen_failures.is_empty() && self.state_mismatches.is_empty()
    }
}

/// @emoji 💥️ Drives `db_artifact::ArtifactEngine` through a fixed, seeded workload once per WAL
/// write boundary, injecting a `FaultScript::fail_nth_write` at that exact boundary and verifying
/// recovery afterward — the family's "crash after every write boundary; recovery invariants hold"
/// law.
pub struct CrashHarness;

#[cfg(not(target_arch = "wasm32"))]
impl CrashHarness {
    /// @emoji 💥️ For `seed`/`op_count`'s deterministic `WorkloadGen::disjoint_batch` workload:
    /// discovers the true `WalStorage::append` call count of a fault-free run (the document's own
    /// genesis header write, plus one call per committed `submit()` at `DurabilityClass::Fsync`),
    /// then for every write boundary strictly after genesis, reruns the workload against a fresh
    /// `FaultStorage`-wrapped `MemoryStorage` with `fail_nth_write` set to that boundary (the
    /// workload loop stops at the first injected error, simulating a crash mid-run), and reopens a
    /// fault-free `ArtifactEngine` on the very same (now-faulted) storage — recovery must never
    /// error, and the recovered document's frontier/state must be EXACTLY the prefix of commands
    /// that landed strictly before the crashed write, no more, no less.
    ///
    /// 🧩️ Extension seam (deliberately, honestly scoped): write boundary #1 — the document's own
    /// genesis header write — is not exercised here. Recovering from a genesis-write failure would
    /// need `ArtifactEngine::open` to tolerate a pre-existing, empty (zero committed records)
    /// segment left behind by a partially-failed `ArtifactEngine::create` (today's `create` always
    /// assumes a clean slate and would hit `AlreadyExists` on `create_segment` if retried against
    /// that same storage) — a real gap, but in `db_artifact`'s `create`/`open` split, not in this
    /// testkit, so it is documented here rather than silently special-cased away.
    pub async fn run_crash_after_every_write(seed: u64, op_count: usize) -> CrashHarnessReport {
        assert!(op_count >= 1, "run_crash_after_every_write needs at least one committed operation to crash between");
        let document = protocol::ArtifactId(format!("testkit-crash-{seed:x}"));
        let ops = WorkloadGen::new(seed).disjoint_batch(&document, op_count).await;

        let baseline = new_fault_backend().await;
        run_workload_against(&document, &ops, baseline.clone()).await;
        let total_appends = as_fault(&baseline).await.append_calls().await;

        let mut report = CrashHarnessReport::default();
        for crash_at in 2..=total_appends {
            report.writes_tested += 1;
            let expected_committed = (crash_at - 2) as usize;

            let faulted = new_fault_backend().await;
            as_fault(&faulted).await.set_script(FaultScript { fail_nth_write: Some(crash_at), ..FaultScript::default() }).await;
            run_workload_until_fault(&document, &ops, faulted.clone()).await;

            match db_artifact::ArtifactEngine::open(document.clone(), &faulted, db_artifact::ArtifactEngineConfig::default(), 0) {
                Ok((recovered, _report)) => {
                    if !recovered_state_matches_prefix(&recovered, &ops, expected_committed).await {
                        report.state_mismatches.push(crash_at);
                    }
                }
                Err(mut rejected) => loop {
                    match rejected.retry_close().await {
                        Ok(error) => {
                            report.reopen_failures.push((crash_at, error.to_string()));
                            break;
                        }
                        Err(retained) => rejected = retained,
                    }
                },
            }
        }
        report
    }
}

/// @emoji 💥️ A fresh `FaultStorage`-wrapped `MemoryStorage`, already living inside its
/// [`DbBackend::Fault`] variant — `MemoryStorage` does no genuinely-blocking I/O, so
/// `CrashHarness`'s workloads never enter the shared typed I/O lane.
async fn new_fault_backend() -> Arc<DbBackend> {
    Arc::new(DbBackend::Fault(Box::new(FaultStorage::new(Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))).await)))
}

/// @emoji 🔍️ Recovers the `&FaultStorage` a [`new_fault_backend`] produced, so `CrashHarness` can
/// call its `append_calls`/`set_script` inherent methods without a second, unwrapped handle to the
/// same storage (an `Arc` clone can't be un-wrapped back to an owned value while other references
/// are still live, so the enum is the single source of truth, matched into on demand instead).
async fn as_fault(storage: &DbBackend) -> &FaultStorage {
    match storage {
        DbBackend::Fault(inner) => inner,
        #[allow(unreachable_patterns)]
        _ => panic!("as_fault: backend is not a DbBackend::Fault"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn run_workload_against(document: &protocol::ArtifactId, ops: &[protocol::MutationEnvelope], storage: Arc<DbBackend>) {
    let mut engine = db_artifact::ArtifactEngine::create(document.clone(), storage, db_artifact::ArtifactEngineConfig::default(), 0).expect("testkit: baseline engine create must not fault");
    for (i, envelope) in ops.iter().enumerate() {
        let batch = db_artifact::CommandBatch::new(vec![envelope.clone()]).await.expect("testkit: single-envelope batch");
        engine.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.expect("testkit: baseline submit must not fault");
    }
}

/// @emoji 💥️ Like `run_workload_against`, but stops silently at the first injected fault instead of
/// panicking — the fault IS the point, simulating a crash mid-workload.
#[cfg(not(target_arch = "wasm32"))]
async fn run_workload_until_fault(document: &protocol::ArtifactId, ops: &[protocol::MutationEnvelope], storage: Arc<DbBackend>) {
    let created = db_artifact::ArtifactEngine::create(document.clone(), storage, db_artifact::ArtifactEngineConfig::default(), 0);
    let mut engine = match created {
        Ok(engine) => engine,
        Err(_) => return, // the fault fired during genesis itself — nothing further to submit.
    };
    for (i, envelope) in ops.iter().enumerate() {
        let batch = db_artifact::CommandBatch::new(vec![envelope.clone()]).await.expect("testkit: single-envelope batch");
        if engine.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.is_err() {
            return;
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn recovered_state_matches_prefix(recovered: &db_artifact::ArtifactEngine, ops: &[protocol::MutationEnvelope], expected_committed: usize) -> bool {
    if recovered.frontier().await.head_seq != expected_committed as u64 {
        return false;
    }
    for i in 0..expected_committed {
        if !query_result_is_present(recovered.get(&format!("path-{i}")).await).await {
            return false;
        }
    }
    expected_committed >= ops.len() || !query_result_is_present(recovered.get(&format!("path-{expected_committed}")).await).await
}

async fn query_bytes_into_vec(mut bytes: db_query::QueryBytes) -> Vec<u8> {
    let mut owned = Vec::with_capacity(bytes.len());
    for fragment in bytes.fragments() {
        owned.extend_from_slice(fragment);
    }
    while !bytes.terminal_is_empty() {
        bytes.close_step().expect("testkit: query bytes close");
        semio_framework_async::yield_once().await;
    }
    owned
}

#[cfg(test)]
async fn db_io_pages_into_vec(mut pages: DbIoPages) -> Vec<u8> {
    let mut owned = Vec::with_capacity(pages.len());
    for fragment in pages.fragments() {
        owned.extend_from_slice(fragment);
    }
    while !pages.terminal_is_empty() {
        pages.close_step().expect("testkit: DB I/O pages close");
        semio_framework_async::yield_once().await;
    }
    owned
}

async fn query_result_is_present(result: Result<Option<db_query::QueryBytes>, DbError>) -> bool {
    let Ok(Some(bytes)) = result else { return false };
    query_bytes_into_vec(bytes).await;
    true
}
//#endregion 🔖️CrashHarness

//#region 🔖️Laws
#[cfg(not(target_arch = "wasm32"))]
async fn temp_dir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    dir.push(format!("db_fault_testing-{name}-{}-{nonce}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("testkit: create temp dir");
    dir
}

async fn single_envelope_batch(envelope: protocol::MutationEnvelope) -> db_artifact::CommandBatch {
    db_artifact::CommandBatch::new(vec![envelope]).await.expect("testkit: single-envelope batch")
}

/// @emoji 🔁️ The replay-determinism law: (1) a document's WAL replay after a clean shutdown+reopen
/// reproduces an identical `Frontier`; (2) an entirely independent `Database` driven by the same
/// seeded workload converges on a byte-identical `Frontier` (proving the generator itself, and the
/// whole submit→WAL→materialize pipeline, are both deterministic — not just idempotent once).
/// Drives real `db_engine::Database::open_at` over `FsStorage` (see module doc).
#[cfg(not(target_arch = "wasm32"))]
pub async fn assert_replay_deterministic(pool: Arc<semio_framework_async::WorkerPool>, seed: u64, op_count: usize) {
    let document = protocol::ArtifactId(format!("testkit-replay-{seed:x}"));
    let ops = WorkloadGen::new(seed).disjoint_batch(&document, op_count.max(1)).await;

    let root = temp_dir("replay").await;
    let frontier_first_run = {
        let mut database = Database::open_at(pool.clone(), &root, Profile::Test).await.expect("testkit: open_at for replay law");
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.expect("testkit: create_document for replay law");
        for envelope in &ops {
            db_actor::block_on(handle.submit(single_envelope_batch(envelope.clone()).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).expect("submit future resolved").expect("submit succeeded");
        }
        let frontier = handle.frontier().await.expect("frontier");
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("shutdown");
        frontier
    };

    let frontier_after_reopen = {
        let database = Database::open_at(pool.clone(), &root, Profile::Test).await.expect("testkit: reopen for replay law");
        let handle = database.document(&document).await.expect("testkit: document must survive reopen");
        handle.frontier().await.expect("frontier after reopen")
    };
    assert_eq!(frontier_first_run, frontier_after_reopen, "WAL replay after a clean reopen must reproduce an identical frontier");

    let ops_regenerated = WorkloadGen::new(seed).disjoint_batch(&document, op_count.max(1)).await;
    assert_eq!(ops, ops_regenerated, "the same seed must generate an identical workload — generator determinism");

    let root_independent = temp_dir("replay-independent").await;
    let frontier_independent = {
        let database = Database::open_at(pool, &root_independent, Profile::Test).await.expect("testkit: open independent replica");
        let handle = database.create_document(ArtifactSpec::new(document).await).await.expect("testkit: create independent replica document");
        for envelope in &ops_regenerated {
            db_actor::block_on(handle.submit(single_envelope_batch(envelope.clone()).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).expect("submit future resolved").expect("submit succeeded");
        }
        handle.frontier().await.expect("frontier")
    };
    assert_eq!(frontier_first_run, frontier_independent, "an independent replica driven by the same seeded workload must converge to an identical frontier");
}

/// @emoji 📸️ The "snapshot + WAL suffix == full replay" law: a replica that snapshots partway
/// through a workload and reopens (materializing from snapshot ⊕ suffix) must reach the EXACT same
/// frontier as a replica that never snapshots and reopens via full-from-genesis replay. Drives real
/// `db_artifact::ArtifactEngine::{create, submit, snapshot_now, open}` over two independent
/// `MemoryStorage` backends (see module doc on why this law is driven one layer below the facade).
#[cfg(not(target_arch = "wasm32"))]
pub async fn assert_snapshot_plus_suffix_equals_replay(seed: u64, before_snapshot: usize, after_snapshot: usize) {
    let document = protocol::ArtifactId(format!("testkit-snap-{seed:x}"));
    let ops = WorkloadGen::new(seed).disjoint_batch(&document, before_snapshot + after_snapshot.max(1)).await;

    let storage_snapshotting: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    {
        let mut engine = db_artifact::ArtifactEngine::create(document.clone(), storage_snapshotting.clone(), db_artifact::ArtifactEngineConfig::default(), 0).expect("create engine a");
        for (i, envelope) in ops.iter().enumerate() {
            engine.submit(single_envelope_batch(envelope.clone()).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.expect("submit a");
            if i + 1 == before_snapshot {
                engine.snapshot_now((i + 1) as u64).await.expect("snapshot_now");
            }
        }
    }
    let (materialized_from_snapshot, report_a) = db_artifact::ArtifactEngine::open(document.clone(), &storage_snapshotting, db_artifact::ArtifactEngineConfig::default(), 0).expect("open engine a");
    assert!(before_snapshot == 0 || report_a.from_snapshot, "replica a must have materialized from a real snapshot when one was published");

    let storage_full_replay: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    {
        let mut engine = db_artifact::ArtifactEngine::create(document.clone(), storage_full_replay.clone(), db_artifact::ArtifactEngineConfig::default(), 0).expect("create engine b");
        for (i, envelope) in ops.iter().enumerate() {
            engine.submit(single_envelope_batch(envelope.clone()).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.expect("submit b");
        }
    }
    let (materialized_full_replay, report_b) = db_artifact::ArtifactEngine::open(document, &storage_full_replay, db_artifact::ArtifactEngineConfig::default(), 0).expect("open engine b");
    assert!(!report_b.from_snapshot, "replica b must never have snapshotted");
    assert_eq!(report_b.commands_replayed as usize, ops.len(), "a never-snapshotted replica must replay every command from genesis");

    assert_eq!(materialized_from_snapshot.frontier().await, materialized_full_replay.frontier().await, "snapshot ⊕ suffix materialization must reach the same frontier as full replay");
    for (i, _) in ops.iter().enumerate() {
        let path = format!("path-{i}");
        assert_eq!(materialized_from_snapshot.get(&path).await, materialized_full_replay.get(&path).await, "path {path:?} must materialize identically via either path");
    }
}

/// @emoji 🧬️ A minimal, always-triggered projection: counts how many committed envelopes it has
/// seen — enough to distinguish "ran" from "carried forward" without interpreting any operation
/// semantics (per `db_projection`'s own "semantics-free" contract).
struct CountingProjection;

impl db_projection::ProjectionClass for CountingProjection {
    type State = u64;

    async fn id(&self) -> &'static str {
        "db_fault_testing.command_count"
    }

    async fn schema_version(&self) -> u32 {
        1
    }

    async fn affected_by(&self, _touched: &db_state::TouchedSet) -> bool {
        true
    }

    async fn initial(&self) -> u64 {
        0
    }

    async fn apply(&self, state: &u64, _envelope: &protocol::MutationEnvelope, _deps: &db_projection::DepView) -> Result<u64, DbError> {
        Ok(state + 1)
    }
}

/// @emoji 🧬️ The "rebuild == incremental apply" law: applying a projection envelope-by-envelope via
/// `apply_envelope` (persisting a checkpoint each step) must reach the exact same final state as a
/// pure, storage-independent `rebuild_in_memory` pass over the same event sequence. Drives real
/// `db_projection::ProjectionEngine` against a real `db_storage::MemoryStorage`'s `IndexStorage`.
pub async fn assert_projection_rebuild_equals_incremental(seed: u64, op_count: usize) {
    let document_core = ArtifactId(format!("testkit-proj-{seed:x}"));
    let document = protocol::ArtifactId(document_core.0.clone());
    let ops = WorkloadGen::new(seed).disjoint_batch(&document, op_count.max(1)).await;

    let storage = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let build_projections = || vec![db_projection::erase(CountingProjection)];

    let incremental_engine = db_projection::ProjectionEngine::new(&storage, document_core.clone(), build_projections()).await.expect("projection engine (incremental)");
    let mut events = Vec::with_capacity(ops.len());
    let mut final_incremental = db_state::PMap::new();
    for (i, envelope) in ops.iter().enumerate() {
        let seq = (i + 1) as u64;
        let mut touched = db_state::TouchedSet::new();
        touched.record(db_state::TouchedRegion::write(format!("path-{i}")));
        final_incremental = incremental_engine.apply_envelope(seq, envelope, &touched).await.expect("apply_envelope");
        events.push((seq, envelope.clone(), touched));
    }

    let rebuild_engine = db_projection::ProjectionEngine::new(&storage, document_core, build_projections()).await.expect("projection engine (rebuild)");
    let rebuilt = rebuild_engine.rebuild_in_memory(&events).await.expect("rebuild_in_memory");

    for id in incremental_engine.topological_order().await {
        assert_eq!(final_incremental.get(&id.to_string()), rebuilt.get(&id.to_string()), "projection {id:?}: incremental result must equal pure rebuild");
    }
}

async fn schema_erased_envelope(document: &protocol::ArtifactId, mutation_id: &str, actor: &str, path: &str, forward: serde_json::Value, inverse: serde_json::Value) -> protocol::MutationEnvelope {
    let mut payload = serde_json::Map::with_capacity(1);
    payload.insert(path.to_string(), forward);
    let mut inverse_payload = serde_json::Map::with_capacity(1);
    inverse_payload.insert(path.to_string(), inverse);
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(mutation_id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap_or_default() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(inverse_payload)).await.unwrap_or_default() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

/// @emoji ↩️ The inverse-undo roundtrip law: undoing a committed operation must apply its recorded
/// inverse exactly; undoing THAT undo (a redo, via the compensating envelope's own flipped inverse
/// — `db_artifact::ArtifactEngine::undo`'s "inverse of inverse" mechanism) must restore the exact
/// original value. Drives a real `db_artifact::ArtifactEngine` over `MemoryStorage`.
#[cfg(not(target_arch = "wasm32"))]
pub async fn assert_inverse_undo_roundtrip(seed: u64) {
    let document = protocol::ArtifactId(format!("testkit-undo-{seed:x}"));
    let storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_actor::block_on(db_storage::MemoryStorage::new(db_storage::db_io_test_pool())).unwrap()));
    let mut engine = db_artifact::ArtifactEngine::create(document.clone(), storage, db_artifact::ArtifactEngineConfig::default(), 0).expect("create engine");

    let path = CommandGen::new(seed).random_path();
    let forward_value = serde_json::json!(seed % 1000);
    let envelope = schema_erased_envelope(&document, "op-forward", "actor-1", &path, forward_value.clone(), serde_json::Value::Null).await;
    let target = envelope.mutation_id.clone();
    engine.submit(single_envelope_batch(envelope).await, db_artifact::SubmitOptions::default(), 1).await.expect("submit forward");

    let after_forward_bytes = engine.get(&path).await.expect("forward value query").expect("forward value present");
    let after_forward: serde_json::Value = db_artifact::decode_pathmap_json(&query_bytes_into_vec(after_forward_bytes).await).await.expect("json");
    assert_eq!(after_forward, forward_value);

    engine.undo(&target, protocol::MutationId("op-undo".to_string()), protocol::ActorId("actor-1".to_string()), 2).await.expect("undo");
    assert!(!query_result_is_present(engine.get(&path).await).await, "undo must apply the recorded inverse — path deleted");

    engine.undo(&protocol::MutationId("op-undo".to_string()), protocol::MutationId("op-redo".to_string()), protocol::ActorId("actor-1".to_string()), 3).await.expect("redo (undo of undo)");
    let after_redo_bytes = engine.get(&path).await.expect("redo value query").expect("redo value present");
    let after_redo: serde_json::Value = db_artifact::decode_pathmap_json(&query_bytes_into_vec(after_redo_bytes).await).await.expect("json");
    assert_eq!(after_redo, forward_value, "undoing the undo (redo) must restore the exact original value — inverse-of-inverse roundtrip");
}

/// @emoji 🔀️ The sync-convergence law: a replica that catches up in one shot, and a replica that
/// catches up across two resumed batches, must both converge to the EXACT same `Frontier` as the
/// canonical source they are replicating from — real `db_sync::{replay_sync_state, missing_commands}`
/// missing-command transfer over real `db_artifact::ArtifactEngine` replicas.
#[cfg(not(target_arch = "wasm32"))]
pub async fn assert_sync_convergence(seed: u64, op_count: usize) {
    let document = protocol::ArtifactId(format!("testkit-sync-{seed:x}"));
    let document_core = ArtifactId(document.0.clone());
    let ops = WorkloadGen::new(seed).disjoint_batch(&document, op_count.max(2)).await;

    let server_storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut server = db_artifact::ArtifactEngine::create(document.clone(), server_storage.clone(), db_artifact::ArtifactEngineConfig::default(), 0).expect("create server");
    for (i, envelope) in ops.iter().enumerate() {
        server.submit(single_envelope_batch(envelope.clone()).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.expect("server submit");
    }
    let server_frontier = server.frontier().await;
    let sync_state = db_actor::block_on(async { db_sync::replay_sync_state(&server_storage.wal().await, document_core.clone()).await }).expect("replay_sync_state");

    let replica1_storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut replica1 = db_artifact::ArtifactEngine::create(document.clone(), replica1_storage, db_artifact::ArtifactEngineConfig::default(), 0).expect("create replica1");
    let missing1 = db_sync::missing_commands(&sync_state, &Frontier::genesis(document_core.clone())).await.expect("missing_commands one-shot");
    for (i, envelope) in missing1.into_iter().enumerate() {
        replica1.submit(single_envelope_batch(envelope).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.expect("replica1 submit");
    }

    let replica2_storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut replica2 = db_artifact::ArtifactEngine::create(document, replica2_storage, db_artifact::ArtifactEngineConfig::default(), 0).expect("create replica2");
    let half = ops.len() / 2;
    let missing2_first = db_sync::missing_commands(&sync_state, &Frontier::genesis(document_core)).await.expect("missing_commands first half");
    for (i, envelope) in missing2_first.into_iter().take(half).enumerate() {
        replica2.submit(single_envelope_batch(envelope).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.expect("replica2 submit (first half)");
    }
    let replica2_resume_frontier = replica2.frontier().await;
    let missing2_rest = db_sync::missing_commands(&sync_state, &replica2_resume_frontier).await.expect("missing_commands resumed");
    for (i, envelope) in missing2_rest.into_iter().enumerate() {
        replica2.submit(single_envelope_batch(envelope).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, (half + i) as u64).await.expect("replica2 submit (rest)");
    }

    assert_eq!(server_frontier, replica1.frontier().await, "a one-shot replica must converge to the server's exact frontier");
    assert_eq!(server_frontier, replica2.frontier().await, "a resumed replica must converge to the server's exact frontier");
}

/// @emoji 🚧️ The fencing law: once a writer's `cas_root` succeeds, a second writer presenting the
/// now-superseded epoch must be fenced (`DbError::Fenced`), and the root must remain exactly what
/// the winning writer left it as — the split-brain gate `EpochFence`/`CatalogStorage`
/// exist for. Generic over any real `&impl CatalogStorage` backend (exercised against both
/// `MemoryStorage` and `FsStorage` in this crate's own tests).
pub async fn assert_fencing_excludes_stale_writer(storage: &impl CatalogStorage) {
    let stale_epoch = storage.read_root().await.expect("read_root").map_or(EpochFence::INITIAL, |(_, fence)| fence);
    let winner_pages = db_io_copy_pages(b"writer-a").expect("law root writer admitted").await.expect("law root pages retained");
    let winner_epoch = storage.cas_root(stale_epoch, winner_pages).await.expect("the first writer presenting the current epoch must win");
    assert_ne!(winner_epoch, stale_epoch, "a successful cas_root must advance the epoch");

    let stale_pages = db_io_copy_pages(b"writer-b-should-be-rejected").expect("law stale writer admitted").await.expect("law stale pages retained");
    let stale_attempt = storage.cas_root(stale_epoch, stale_pages).await;
    assert!(matches!(stale_attempt, Err(DbError::Fenced { .. })), "a writer presenting a superseded epoch must be fenced, not silently accepted");

    let (root_bytes, root_epoch) = storage.read_root().await.expect("read_root").expect("root must exist after the winning write");
    assert_eq!(root_bytes, b"writer-a".as_slice(), "the fenced writer must never have overwritten the root");
    assert_eq!(root_epoch, winner_epoch);
}

/// @emoji 🌫️ The preview-never-durable law: publishing a preview must never append a single byte to
/// the document's WAL, never create a new segment, and never advance the committed frontier — while
/// still correctly shadowing the committed value for the preview's own reader. Drives a real
/// `db_artifact::ArtifactEngine` (backed by a real `db_preview::PreviewStore`) over `MemoryStorage`.
#[cfg(not(target_arch = "wasm32"))]
pub async fn assert_preview_never_durable(seed: u64) {
    let document = protocol::ArtifactId(format!("testkit-preview-{seed:x}"));
    let storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_actor::block_on(db_storage::MemoryStorage::new(db_storage::db_io_test_pool())).unwrap()));
    let core_document = ArtifactId(document.0.clone());
    let mut engine = db_artifact::ArtifactEngine::create(document.clone(), storage.clone(), db_artifact::ArtifactEngineConfig::default(), 0).expect("create engine");

    let path = CommandGen::new(seed).random_path();
    let committed_value = serde_json::json!("committed");
    let envelope = schema_erased_envelope(&document, "op-committed", "actor-1", &path, committed_value.clone(), serde_json::Value::Null);
    engine.submit(single_envelope_batch(envelope.await).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.expect("submit committed");

    let wal_facet = db_actor::block_on(storage.wal());
    let segments_before = db_actor::block_on(wal_facet.list_segments(&core_document)).expect("list_segments");
    let lengths_before: Vec<u64> = segments_before.iter().map(|&index| db_actor::block_on(wal_facet.segment_len(&core_document, index)).expect("segment_len")).collect();
    let frontier_before = engine.frontier().await;

    let preview_value = serde_json::json!("preview-only");
    let preview_id = engine.publish_preview(&[(path.clone(), Some(preview_value.clone()))], 2).await.expect("publish_preview");

    let segments_after = db_actor::block_on(wal_facet.list_segments(&core_document)).expect("list_segments");
    let lengths_after: Vec<u64> = segments_after.iter().map(|&index| db_actor::block_on(wal_facet.segment_len(&core_document, index)).expect("segment_len")).collect();
    assert_eq!(segments_before, segments_after, "publishing a preview must never create a new wal segment");
    assert_eq!(lengths_before, lengths_after, "publishing a preview must never append a single byte to the wal");
    assert_eq!(engine.frontier().await, frontier_before, "a preview must never advance the document's committed frontier");

    let previewed_bytes = engine.preview_get(&preview_id, &path).await.expect("preview_get").expect("preview value present");
    let previewed: serde_json::Value = db_artifact::decode_pathmap_json(&query_bytes_into_vec(previewed_bytes).await).await.expect("json");
    assert_eq!(previewed, preview_value, "the preview overlay must shadow the committed value for its own reader");

    let committed_bytes = engine.get(&path).await.expect("committed value query").expect("committed value still present");
    let committed_still: serde_json::Value = db_artifact::decode_pathmap_json(&query_bytes_into_vec(committed_bytes).await).await.expect("json");
    assert_eq!(committed_still, committed_value, "a preview must never mutate the canonical committed state");
}

/// @emoji 🫧️ The overlay structural-sharing law: each `db_state::OverlayRoot::set` grows the
/// overlay by exactly one entry (no hidden base copy), and every earlier snapshot remains exactly
/// as it was — unaffected by later derivations sharing the same immutable base. This is the
/// operationally-relevant, functionally-testable half of "structural sharing" (persistence: an
/// older root is never mutated by deriving a newer one from it) rather than a literal memory-layout
/// assertion, which a black-box unit test cannot make.
pub async fn assert_overlay_structural_sharing(writes: usize) {
    let base: db_state::OverlayRoot<db_state::EmptyBase> = db_state::OverlayRoot::new(db_state::EmptyBase);
    assert_eq!(base.overlay_len(), 0, "a fresh overlay over an empty base starts with nothing recorded");

    let mut snapshots = vec![base.clone()];
    let mut current = base;
    let mut written_paths = Vec::new();
    for i in 0..writes.max(1) {
        let path = format!("overlay-path-{i}");
        let (next, _touched) = current.set(&path, format!("value-{i}").into_bytes());
        assert_eq!(next.overlay_len(), current.overlay_len() + 1, "each write must grow the overlay by exactly one entry");
        snapshots.push(next.clone());
        written_paths.push(path);
        current = next;
    }

    for (index, snapshot) in snapshots.iter().enumerate() {
        assert_eq!(snapshot.overlay_len(), index, "snapshot #{index} must retain its own overlay size forever, unaffected by later derivations");
        for (later_index, path) in written_paths.iter().enumerate() {
            let expected = if later_index < index { Some(format!("value-{later_index}").into_bytes()) } else { None };
            assert_eq!(snapshot.get(path).expect("overlay get"), expected, "snapshot #{index} must see exactly the writes that existed when it was taken");
        }
    }
}
//#endregion 🔖️Laws

//#region 🧪️Tests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../🧯️fault-storage-laws/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
