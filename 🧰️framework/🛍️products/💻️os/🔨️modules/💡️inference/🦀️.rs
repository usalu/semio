//! 💡 Optional, configurable dependency-aware inference cache. `Inference<P>::infer` (`os_spr::command`)
//! is the single semantics source; everything here is a pure optimization over it — with the cache
//! disabled, [`infer_field`] degenerates to a plain recompute with byte-identical output
//! (cache-transparency law, proven by tests below). Content-addressed per-entity dependency-hash
//! chains (blake3, via `semio-framework-hash`'s `merkle_node`/`merkle_collection`) give "full-blown
//! dependency support" for free: an entity whose dependency chain is byte-identical hits the cache,
//! one whose chain changed misses and recomputes — no explicit invalidation bookkeeping to get wrong.
//! Canonical worked example: `flatPosition` (plane + center) per object, invalidated only when parent
//! position, parent vortex, or the object's own vortex changes (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING; dependency-hash design
//! from the closed ticket 26/04/17/OPTIMIZE-FLATTEN-DESIGN-WITH-MERKLE-HASH-CACHE).

use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

//#region 🔖️DepHash
/// 🔑 Per-entity dependency hash — one link in a merkle dependency chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DepHash(pub [u8; 32]);

impl DepHash {
    /// 🏗️ Roots a chain: `blake3(field_id ‖ 0 ‖ schema_version ‖ 0 ‖ input)`, no parent hashes folded in.
    pub fn root(field_id: &str, schema_version: u32, input: &[u8]) -> Self {
        let mut data = field_id.as_bytes().to_vec();
        data.push(0);
        data.extend_from_slice(&schema_version.to_le_bytes());
        data.push(0);
        data.extend_from_slice(input);
        Self(*semio_framework_hash::hash(&data).as_bytes())
    }

    /// 🔗 Extends a chain: folds `parents` (order-independent — sorted by their own bytes via
    /// `merkle_node`, so two entities with the same parent SET in different orders hash identically)
    /// into `input` under the same `(field_id, schema_version)` salt as [`root`](Self::root).
    pub fn chain(field_id: &str, schema_version: u32, input: &[u8], parents: &[DepHash]) -> Self {
        let mut own = field_id.as_bytes().to_vec();
        own.push(0);
        own.extend_from_slice(&schema_version.to_le_bytes());
        own.push(0);
        own.extend_from_slice(input);
        let own_hex = semio_framework_hash::hash(&own).to_hex();
        // 🪡️ `hex::encode` is async; `Iterator::map`'s closure is sync (E0728), so the await is
        // hoisted into a plain loop instead (R10 residue #1).
        let mut parent_hexes: Vec<String> = Vec::with_capacity(parents.len());
        for parent in parents {
            parent_hexes.push(hex::encode(parent.0));
        }
        let folded = semio_framework_hash::merkle_node(&[&own_hex], parent_hexes);
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(&folded, &mut bytes).expect("merkle_node returns 64 hex chars");
        Self(bytes)
    }
}

mod hex {
    pub fn encode(bytes: [u8; 32]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
    pub fn decode_to_slice(s: &str, out: &mut [u8; 32]) -> Result<(), &'static str> {
        if s.len() != 64 {
            return Err("expected 64 hex chars");
        }
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let byte_str = std::str::from_utf8(chunk).map_err(|_| "invalid utf8")?;
            out[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "invalid hex")?;
        }
        Ok(())
    }
}
//#endregion 🔖️DepHash

//#region 🔖️InferredField
/// 🧭 One entity in a field's deterministic evaluation plan (roots first).
#[derive(Clone, Debug)]
pub struct InferenceStep<K> {
    pub key: K,
    pub parents: Vec<K>,
}

/// 🚫 A compute that could not produce a value: a stable machine code and a readable message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceFault {
    pub code: String,
    pub message: String,
}

impl InferenceFault {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into() }
    }
}

/// 🛑 Why a driver call could not finish: a plan that names a parent before computing it, a compute that failed, or a cancelled run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferenceError {
    MissingParent { key: String, parent: String },
    Compute { key: String, fault: InferenceFault },
    Cancelled,
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingParent { key, parent } => write!(formatter, "inference plan violation: {key} names parent {parent} before it is computed"),
            Self::Compute { key, fault } => write!(formatter, "inference compute of {key} failed: {}: {}", fault.code, fault.message),
            Self::Cancelled => write!(formatter, "inference cancelled"),
        }
    }
}

impl std::error::Error for InferenceError {}

/// ⏳ A compute that spans several driver calls (a kernel job): the driver keeps it in the cursor between calls and cancels it with the run.
pub trait InferencePending: Send {
    fn cancel(&mut self);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 🪜 One slice of a compute: it finished (`Done`) or yielded for another driver call (`Working`). Either way it reports the fuel it consumed.
pub enum ComputeStep<V> {
    Done { value: V, fuel_used: usize },
    Working { fuel_used: usize, progress: f32 },
}

/// 🕸️ One inferred field family, computed entity-by-entity over a dependency DAG. This is the
/// trait real derivation math (e.g. a flatten engine) implements; an artifact's top-level
/// `Inference::infer` assembles its `XInference` struct from one or more `InferredField`s.
pub trait InferredField<P>: Send + Sync + 'static {
    type Key: Clone + Eq + std::hash::Hash + Ord + Send + Sync + ToValue + FromValue + 'static;
    type Value: Clone + Send + Sync + 'static;
    type Dependency: ToValue;

    const FIELD_ID: &'static str;
    const SCHEMA_VERSION: u32;

    /// 🗺️ Coarse tier-1 read-set — checked against a diff's [`crate::os_spr::command::DiffRegions::touches`]
    /// before this field's plan is even walked.
    fn reads() -> &'static [&'static str];

    /// 🧭 Deterministic topological plan over `snapshot`'s entities (roots first — entries with no
    /// parents come before anything that depends on them).
    fn plan(snapshot: &P) -> Vec<InferenceStep<Self::Key>>;

    /// 🔑 Owned dependency values for `key` — EXACTLY the snapshot fields `compute` may
    /// read for this key (excluding parents' OWN upstream values, which are folded in separately
    /// via their already-computed [`DepHash`]es — but INCLUDING the specific edge/connector data
    /// tying `key` to each of `parents`, e.g. a compose-style attraction's params, since that lives
    /// on `key`'s own incoming edge, not on the parent's upstream chain). `parents` is `plan`'s
    /// `InferenceStep.parents` for this key, passed through so implementations don't need to
    /// re-derive "which edge connects to which parent" a second time. Honesty contract: this must
    /// cover everything `compute` reads, or a changed-but-uncovered input silently serves a stale
    /// cached value.
    fn dep_input(snapshot: &P, key: &Self::Key, parents: &[Self::Key]) -> Self::Dependency;

    /// 🧮 Pure per-entity compute, given parents' already-computed values in `plan`'s parent order.
    fn compute(snapshot: &P, key: &Self::Key, parents: &[Self::Value]) -> Self::Value;

    /// ⚖️ The bytes a cached `value` is accounted with against the cache budget; the default is the value's inline size.
    fn value_bytes(value: &Self::Value) -> usize {
        let _ = value;
        size_of::<Self::Value>()
    }

    /// 🪜 Resumable, fallible variant of [`compute`](Self::compute): consumes at most `fuel` units and either finishes or parks its progress in `pending`
    /// for the next call. The default finishes in one slice at the cost of one unit, so `compute` stays the single semantic source of every field that does not override this.
    fn compute_step(snapshot: &P, key: &Self::Key, parents: &[Self::Value], pending: &mut Option<Box<dyn InferencePending>>, fuel: usize) -> Result<ComputeStep<Self::Value>, InferenceFault> {
        let _ = (pending, fuel);
        Ok(ComputeStep::Done { value: Self::compute(snapshot, key, parents), fuel_used: 1 })
    }
}
//#endregion 🔖️InferredField

//#region 🔖️Config
/// ⚙️ Optionality + configuration surface. Default = DISABLED: with no cache every path degenerates
/// to plain recompute — caching is strictly opt-in per host.
#[derive(Clone, Debug)]
pub struct InferenceCacheConfig {
    pub enabled: bool,
    pub budget_bytes: usize,
    pub persistence: InferencePersistence,
    pub record_stats: bool,
}

impl Default for InferenceCacheConfig {
    fn default() -> Self {
        Self { enabled: false, budget_bytes: 16 * 1024 * 1024, persistence: InferencePersistence::None, record_stats: false }
    }
}

/// 💾 Whether cached inference results are also checkpointed durably (via a `db_projection`
/// adapter, kept out of this wasm-safe kernel crate — see the framework-level `db_artifact` module).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferencePersistence {
    None,
    Projection,
}
//#endregion 🔖️Config

//#region 🔖️Cache
struct CacheEntry {
    value: Arc<dyn Any + Send + Sync>,
    byte_len: usize,
    tick: u64,
}

/// 🧠 Content-addressed inference value cache — mirrors `semio_framework_2d::compute::EngineCache`'s LRU/byte-budget
/// mechanism, keyed by [`DepHash`] instead of a raw content hash of caller-supplied input. Values are held typed
/// and in memory (no encode/decode round trip), recency is a monotonic tick so a hit is O(log n), and an entry that
/// alone exceeds the budget is never stored, so `used_bytes` never exceeds `budget_bytes`.
pub struct InferenceCache {
    config: InferenceCacheConfig,
    entries: HashMap<DepHash, CacheEntry>,
    recency: BTreeMap<u64, DepHash>,
    clock: u64,
    used_bytes: usize,
    stats: InferenceCacheStats,
}

/// 📊 Hit/miss counters — populated only when `config.record_stats` is set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InferenceCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

impl InferenceCache {
    pub async fn new(config: InferenceCacheConfig) -> Self {
        Self { config, entries: HashMap::new(), recency: BTreeMap::new(), clock: 0, used_bytes: 0, stats: InferenceCacheStats::default() }
    }

    pub async fn stats(&self) -> InferenceCacheStats {
        self.stats
    }

    /// 🔌 Whether this cache stores and serves anything at all.
    pub fn enabled(&self) -> bool {
        self.config.enabled
    }

    /// 📏 The bytes currently held, never above the configured budget.
    pub fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    /// 🔢 How many entries are held.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 🧹 Explicit whole-cache invalidation (e.g. after a schema-version bump discovered at runtime).
    pub async fn clear(&mut self) {
        self.entries.clear();
        self.recency.clear();
        self.used_bytes = 0;
    }

    fn get<V: Clone + 'static>(&mut self, key: DepHash) -> Option<V> {
        if !self.config.enabled {
            return None;
        }
        let hit = self.entries.get(&key).and_then(|entry| entry.value.downcast_ref::<V>().cloned());
        if hit.is_some() {
            self.touch(key);
        }
        if self.config.record_stats {
            if hit.is_some() {
                self.stats.hits += 1;
            } else {
                self.stats.misses += 1;
            }
        }
        hit
    }

    fn insert<V: Send + Sync + 'static>(&mut self, key: DepHash, value: V, byte_len: usize) {
        if !self.config.enabled {
            return;
        }
        if let Some(old) = self.entries.remove(&key) {
            self.recency.remove(&old.tick);
            self.used_bytes = self.used_bytes.saturating_sub(old.byte_len);
        }
        if byte_len > self.config.budget_bytes {
            return;
        }
        self.ensure_budget(byte_len);
        self.clock += 1;
        self.recency.insert(self.clock, key);
        self.entries.insert(key, CacheEntry { value: Arc::new(value), byte_len, tick: self.clock });
        self.used_bytes = self.used_bytes.saturating_add(byte_len);
    }

    fn touch(&mut self, key: DepHash) {
        self.clock += 1;
        if let Some(entry) = self.entries.get_mut(&key) {
            self.recency.remove(&entry.tick);
            entry.tick = self.clock;
            self.recency.insert(self.clock, key);
        }
    }

    fn ensure_budget(&mut self, needed: usize) {
        while self.used_bytes.saturating_add(needed) > self.config.budget_bytes {
            let Some((_, old)) = self.recency.pop_first() else { break };
            if let Some(entry) = self.entries.remove(&old) {
                self.used_bytes = self.used_bytes.saturating_sub(entry.byte_len);
                if self.config.record_stats {
                    self.stats.evictions += 1;
                }
            }
        }
    }
}
//#endregion 🔖️Cache

//#region 🔖️Session
struct SessionEntry {
    root: DepHash,
    result: Box<dyn Any + Send + Sync>,
}

/// 🧭 Per-artifact-instance tier-1 gate state: one root [`DepHash`] + typed result per field id,
/// consulted by [`infer_field_after_diff`] before even walking a field's plan. The root is the merkle
/// fold of every entity's own dependency hash, so it changes exactly when some entity's chain changed.
#[derive(Default)]
pub struct InferenceSession {
    roots: HashMap<&'static str, SessionEntry>,
}

impl InferenceSession {
    pub async fn new() -> Self {
        Self::default()
    }

    /// 🌳 The root of the result last stored for `field_id`.
    pub fn root(&self, field_id: &str) -> Option<DepHash> {
        self.roots.get(field_id).map(|entry| entry.root)
    }
}
//#endregion 🔖️Session

//#region 🔖️Cursor
/// 🧷 The explicit, resumable position of one field run: the plan, the next entity, the dependency hash of every finished
/// entity and the compute in flight. The caller owns it between [`infer_field_step`] calls and may [`cancel`](Self::cancel) it.
pub struct InferenceCursor<K> {
    plan: Option<Vec<InferenceStep<K>>>,
    next: usize,
    hashes: HashMap<K, DepHash>,
    pending: Option<Box<dyn InferencePending>>,
    progress: f32,
    cancelled: bool,
}

impl<K> Default for InferenceCursor<K> {
    fn default() -> Self {
        Self { plan: None, next: 0, hashes: HashMap::new(), pending: None, progress: 0.0, cancelled: false }
    }
}

impl<K: Eq + std::hash::Hash> InferenceCursor<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🛑 Cancels the in-flight compute and makes every later step refuse with [`InferenceError::Cancelled`]; finished values stay valid.
    pub fn cancel(&mut self) {
        if let Some(mut pending) = self.pending.take() {
            pending.cancel();
        }
        self.cancelled = true;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// ✅ Entities finished so far.
    pub fn completed(&self) -> usize {
        self.next
    }

    /// 🧮 Entities in the plan; zero until the first step has planned.
    pub fn total(&self) -> usize {
        self.plan.as_ref().map_or(0, Vec::len)
    }

    /// 📈 Finished entities plus the in-flight compute's own progress, as a fraction of the plan.
    pub fn fraction(&self) -> f32 {
        match self.total() {
            0 => 0.0,
            total => ((self.next as f32 + self.progress.clamp(0.0, 1.0)) / total as f32).min(1.0),
        }
    }

    pub fn is_done(&self) -> bool {
        self.plan.as_ref().is_some_and(|plan| self.next >= plan.len())
    }

    /// 🔗 The dependency hash of a finished entity.
    pub fn hash(&self, key: &K) -> Option<DepHash> {
        self.hashes.get(key).copied()
    }
}

/// 📋 What one driver call did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InferenceStepReport {
    pub done: bool,
    pub fuel_used: usize,
    pub computed: usize,
    pub hits: usize,
    pub progress: f32,
}
//#endregion 🔖️Cursor

//#region 🔖️Driver
use crate::io::text::inferences::encode;

fn key_text<K: ToValue>(key: &K) -> String {
    String::from_utf8_lossy(&encode(key)).into_owned()
}

/// ⏩ THE driver: resumes `cursor` over `F::plan(snapshot)`, hashing each entity's dependency chain and consulting
/// `cache` (if `Some`) before computing, and spends at most `fuel` units (a cache hit is free, a compute costs what
/// [`InferredField::compute_step`] reports). `cache: None` ⇒ pure recompute — identical output to a warm-cache run
/// (cache-transparency law, proven in tests below). The snapshot must not change between calls of one cursor.
/// Dependency hashes exist to address the cache, so they are computed (and [`InferenceCursor::hash`] answers) only when `cache` is enabled;
/// without one `dep_input` is never evaluated.
pub fn infer_field_step<P, F: InferredField<P>>(snapshot: &P, cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values: &mut BTreeMap<F::Key, F::Value>, fuel: usize) -> Result<InferenceStepReport, InferenceError> {
    let hashing = cache.as_deref().is_some_and(InferenceCache::enabled);
    step_driver::<P, F>(snapshot, cache, cursor, values, fuel, hashing)
}

fn step_driver<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values: &mut BTreeMap<F::Key, F::Value>, fuel: usize, hashing: bool) -> Result<InferenceStepReport, InferenceError> {
    if cursor.cancelled {
        return Err(InferenceError::Cancelled);
    }
    if cursor.plan.is_none() {
        cursor.plan = Some(F::plan(snapshot));
    }
    let total = cursor.total();
    let mut remaining = fuel.max(1);
    let mut report = InferenceStepReport::default();

    while cursor.next < total {
        let step = cursor.plan.as_ref().map(|plan| plan[cursor.next].clone()).expect("the plan is set above");
        let mut parent_hashes: Vec<DepHash> = Vec::with_capacity(if hashing { step.parents.len() } else { 0 });
        let mut parent_values: Vec<F::Value> = Vec::with_capacity(step.parents.len());
        for parent in &step.parents {
            let missing = || InferenceError::MissingParent { key: key_text(&step.key), parent: key_text(parent) };
            if hashing {
                parent_hashes.push(cursor.hashes.get(parent).copied().ok_or_else(missing)?);
            }
            parent_values.push(values.get(parent).cloned().ok_or_else(missing)?);
        }
        let dep_hash = hashing.then(|| {
            let input = encode(&F::dep_input(snapshot, &step.key, &step.parents));
            if step.parents.is_empty() { DepHash::root(F::FIELD_ID, F::SCHEMA_VERSION, &input) } else { DepHash::chain(F::FIELD_ID, F::SCHEMA_VERSION, &input, &parent_hashes) }
        });

        if cursor.pending.is_none() {
            if let Some((hash, value)) = dep_hash.zip(cache.as_deref_mut()).and_then(|(hash, cache)| cache.get::<F::Value>(hash).map(|value| (hash, value))) {
                cursor.hashes.insert(step.key.clone(), hash);
                values.insert(step.key, value);
                cursor.next += 1;
                report.hits += 1;
                continue;
            }
        }

        match F::compute_step(snapshot, &step.key, &parent_values, &mut cursor.pending, remaining) {
            Err(fault) => {
                if let Some(mut pending) = cursor.pending.take() {
                    pending.cancel();
                }
                return Err(InferenceError::Compute { key: key_text(&step.key), fault });
            }
            Ok(ComputeStep::Working { fuel_used, progress }) => {
                cursor.progress = progress;
                report.fuel_used += fuel_used.max(1);
                break;
            }
            Ok(ComputeStep::Done { value, fuel_used }) => {
                cursor.pending = None;
                cursor.progress = 0.0;
                if let Some(hash) = dep_hash {
                    if let Some(cache) = cache.as_deref_mut().filter(|cache| cache.enabled()) {
                        cache.insert(hash, value.clone(), F::value_bytes(&value));
                    }
                    cursor.hashes.insert(step.key.clone(), hash);
                }
                values.insert(step.key, value);
                cursor.next += 1;
                report.computed += 1;
                let spent = fuel_used.max(1);
                report.fuel_used += spent;
                remaining = remaining.saturating_sub(spent);
                if remaining == 0 {
                    break;
                }
            }
        }
    }

    report.done = cursor.is_done();
    report.progress = cursor.fraction();
    Ok(report)
}

/// ⏩ The unbounded driver: [`infer_field_step`] with unlimited fuel until the run is done. Fallible so a plan-order violation or a failing compute is an error value.
pub fn try_infer_field<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>) -> Result<BTreeMap<F::Key, F::Value>, InferenceError> {
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    let hashing = cache.as_deref().is_some_and(InferenceCache::enabled);
    run_to_end::<P, F>(snapshot, cache.as_deref_mut(), &mut cursor, &mut values, hashing)?;
    Ok(values)
}

fn run_to_end<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values: &mut BTreeMap<F::Key, F::Value>, hashing: bool) -> Result<(), InferenceError> {
    loop {
        if step_driver::<P, F>(snapshot, cache.as_deref_mut(), cursor, values, usize::MAX, hashing)?.done {
            return Ok(());
        }
    }
}

/// ⏩ The plain driver: [`try_infer_field`] for fields whose plan is a topological order and whose compute is total; a violation of that contract is a loud failure, never a silently misaligned compute.
pub fn infer_field<P, F: InferredField<P>>(snapshot: &P, cache: Option<&mut InferenceCache>) -> BTreeMap<F::Key, F::Value> {
    try_infer_field::<P, F>(snapshot, cache).unwrap_or_else(|error| panic!("{error}"))
}

/// ⏩ Diff-gated variant: if `diff.touches()` doesn't intersect `F::reads()`, returns the session's
/// previous full result for this field unchanged (tier-1 gate) instead of walking the plan at all.
/// Falls through to [`infer_field`] (and refreshes the session when the result's root moved) otherwise.
pub async fn infer_field_after_diff<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> BTreeMap<F::Key, F::Value>
where
    F: InferredField<P>,
    D: crate::os_spr::command::DiffRegions,
{
    if !diff.touches().intersects_any(F::reads()) {
        if let Some(stored) = session.roots.get(F::FIELD_ID).and_then(|entry| entry.result.downcast_ref::<BTreeMap<F::Key, F::Value>>()) {
            return stored.clone();
        }
    }
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    run_to_end::<P, F>(snapshot, Some(cache), &mut cursor, &mut values, true).unwrap_or_else(|error| panic!("{error}"));
    let root = result_root::<F::Key, F::Value>(&values, &cursor);
    if session.root(F::FIELD_ID) != Some(root) {
        session.roots.insert(F::FIELD_ID, SessionEntry { root, result: Box::new(values.clone()) });
    }
    values
}

fn result_root<K: ToValue + Eq + std::hash::Hash, V>(values: &BTreeMap<K, V>, cursor: &InferenceCursor<K>) -> DepHash {
    let leaves: Vec<String> = values.keys().map(|key| format!("{}={}", key_text(key), cursor.hash(key).map(|hash| hex::encode(hash.0)).unwrap_or_default())).collect();
    DepHash(*semio_framework_hash::hash(semio_framework_hash::merkle_collection(leaves).as_bytes()).as_bytes())
}
//#endregion 🔖️Driver

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
