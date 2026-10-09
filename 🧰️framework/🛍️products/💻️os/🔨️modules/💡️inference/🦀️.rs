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
use crate::os_spr::command::TouchedPaths;
use std::any::Any;
use std::collections::{BTreeMap, HashMap, HashSet};
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

    /// 🔗 Extends a chain: folds `parents` (order-independent — sorted by their own bytes, so two entities with
    /// the same parent SET in different orders hash identically) into `input` under the same `(field_id, schema_version)`
    /// salt as [`root`](Self::root). The fold is `semio_framework_hash::merkle_node` over the hex of the sorted parents, without its strings.
    pub fn chain(field_id: &str, schema_version: u32, input: &[u8], parents: &[DepHash]) -> Self {
        let mut own = field_id.as_bytes().to_vec();
        own.push(0);
        own.extend_from_slice(&schema_version.to_le_bytes());
        own.push(0);
        own.extend_from_slice(input);
        let mut sorted: Vec<[u8; 32]> = parents.iter().map(|parent| parent.0).collect();
        sorted.sort_unstable();
        let mut data = Vec::with_capacity(65 * (1 + sorted.len()));
        push_hex(&mut data, semio_framework_hash::hash(&own).as_bytes());
        data.push(0x1f);
        for parent in &sorted {
            push_hex(&mut data, parent);
            data.push(0x1f);
        }
        Self(*semio_framework_hash::hash(&data).as_bytes())
    }
}

fn push_hex(out: &mut Vec<u8>, bytes: &[u8; 32]) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for byte in bytes {
        out.push(DIGITS[usize::from(byte >> 4)]);
        out.push(DIGITS[usize::from(byte & 15)]);
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

    /// 🎯 Whether the regions a diff touched may change the dependency of `key` (the per-key twin of [`reads`](Self::reads)): `false` lets an incremental update carry the previous value of an entity whose parents did not move
    /// without evaluating its dependency. `snapshot` is the snapshot after the diff, so the rows `key` reads are the ones it names now. Over-approximation costs a hash, under-approximation serves a stale value, so the default says yes.
    fn touched_by(snapshot: &P, key: &Self::Key, touched: &TouchedPaths) -> bool {
        let _ = (snapshot, key, touched);
        true
    }

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
/// 🔗 One entity as the last run of its field left it: the dependency hash its value was computed under, the parents it was computed from, its share of the field digest and the run that last looked at it.
struct Link<K> {
    hash: DepHash,
    parents: Vec<K>,
    entry: [u64; 4],
    epoch: u64,
}

/// 🧠 What a field keeps between two runs: its values, one [`Link`] per value, the digest of all links and whether the values are known to match the last snapshot.
struct Stored<K, V> {
    values: BTreeMap<K, V>,
    links: HashMap<K, Link<K>>,
    digest: [u64; 4],
    epoch: u64,
    sound: bool,
}

impl<K, V> Stored<K, V> {
    fn empty() -> Self {
        Self { values: BTreeMap::new(), links: HashMap::new(), digest: [0; 4], epoch: 0, sound: false }
    }
}

struct SessionEntry {
    digest: [u64; 4],
    stored: Box<dyn Any + Send + Sync>,
}

/// 🧭 Per-artifact-instance state of the diff-driven entry points: per field id the values of the last run with their dependency links. [`infer_field_after_diff`] and the stepped [`InferenceSession::begin_update`] consult it
/// to skip a whole field the diff does not touch, to carry every entity a diff and its parents did not move, and to skip the compute of an entity whose dependency hash did not move.
#[derive(Default)]
pub struct InferenceSession {
    entries: HashMap<&'static str, SessionEntry>,
}

impl InferenceSession {
    pub async fn new() -> Self {
        Self::default()
    }

    /// 🌳 The root of the result last stored for `field_id`: a digest over every entity's key and dependency hash, so it moves exactly when some entity's chain moved.
    pub fn root(&self, field_id: &str) -> Option<DepHash> {
        self.entries.get(field_id).map(|entry| {
            let mut bytes = Vec::with_capacity(32);
            for lane in entry.digest {
                bytes.extend_from_slice(&lane.to_le_bytes());
            }
            DepHash(*semio_framework_hash::hash(&bytes).as_bytes())
        })
    }

    /// 🗂️ The values of the last run of field `F`, if there was one.
    pub fn field_values<P, F: InferredField<P>>(&self) -> Option<&BTreeMap<F::Key, F::Value>> {
        self.entries.get(F::FIELD_ID).and_then(|entry| entry.stored.downcast_ref::<Stored<F::Key, F::Value>>()).map(|stored| &stored.values)
    }

    /// 🚦 Starts an update of field `F` after `diff`. The field's values move into the update; [`finish_update`](Self::finish_update) gives them back. With `reuse` an entity is carried or confirmed instead of recomputed
    /// (the caller passes whether its cache is enabled: with no cache every update is a plain recompute).
    pub fn begin_update<P, F: InferredField<P>, D: crate::os_spr::command::DiffRegions>(&mut self, diff: &D, reuse: bool) -> FieldUpdate<F::Key, F::Value> {
        let stored = self.entries.remove(F::FIELD_ID).and_then(|entry| entry.stored.downcast::<Stored<F::Key, F::Value>>().ok()).map_or_else(Stored::empty, |stored| *stored);
        let touched = diff.touches();
        let known = stored.sound;
        let gated = known && !touched.intersects_any(F::reads());
        let epoch = stored.epoch + 1;
        FieldUpdate {
            field: F::FIELD_ID,
            values: stored.values,
            track: Track { links: stored.links, digest: stored.digest, epoch, touched: known.then_some(touched), reuse, moved: HashSet::new(), changes: Vec::new(), carried: 0, confirmed: 0, swept: false },
            cursor: InferenceCursor::new(),
            gated,
            computed: 0,
            hits: 0,
        }
    }

    /// 🏁 Ends an update, finished or not: the values stay in the session and are trusted by the next update only when this one ran to its end.
    pub fn finish_update<K: Clone + Eq + std::hash::Hash + Ord + Send + Sync + 'static, V: Send + Sync + 'static>(&mut self, update: FieldUpdate<K, V>) -> FieldDelta<K, V> {
        let done = update.gated || update.cursor.is_done();
        let FieldUpdate { field, values, track, gated, computed, hits, .. } = update;
        let Track { links, digest, epoch, changes, carried, confirmed, .. } = track;
        self.entries.insert(field, SessionEntry { digest, stored: Box::new(Stored { values, links, digest, epoch, sound: done }) });
        FieldDelta { gated, done, changes, computed, hits, carried, confirmed }
    }
}
//#endregion 🔖️Session

//#region 🔖️Update
/// 🔄 One entity a diff moved: its key and the value it had before (`None` when it is new). The value it has now is in the session's values, or the entity is gone from them.
#[derive(Clone, Debug)]
pub struct FieldChange<K, V> {
    pub key: K,
    pub old: Option<V>,
}

/// 📊 What an update did: the entities whose value was replaced, added or removed, and how the rest was served.
#[derive(Clone, Debug)]
pub struct FieldDelta<K, V> {
    pub gated: bool,
    pub done: bool,
    pub changes: Vec<FieldChange<K, V>>,
    pub computed: usize,
    pub hits: usize,
    pub carried: usize,
    pub confirmed: usize,
}

/// 🧮 The bookkeeping of an update beside the values: the links of the last run, the regions the diff touched (`None` when nothing is known of the last run, so every entity is examined), the entities whose
/// dependency hash moved in this run and the replaced values.
struct Track<K, V> {
    links: HashMap<K, Link<K>>,
    digest: [u64; 4],
    epoch: u64,
    touched: Option<TouchedPaths>,
    reuse: bool,
    moved: HashSet<K>,
    changes: Vec<FieldChange<K, V>>,
    carried: usize,
    confirmed: usize,
    swept: bool,
}

fn lanes(bytes: &[u8; 32]) -> [u64; 4] {
    std::array::from_fn(|at| u64::from_le_bytes(bytes[at * 8..at * 8 + 8].try_into().expect("eight bytes")))
}

impl<K: Clone + Eq + std::hash::Hash + ToValue, V> Track<K, V> {
    fn hash_of(&self, key: &K) -> Option<DepHash> {
        self.links.get(key).filter(|link| link.epoch == self.epoch).map(|link| link.hash)
    }

    fn carries<P, F: InferredField<P, Key = K, Value = V>>(&mut self, snapshot: &P, step: &InferenceStep<K>, values: &BTreeMap<K, V>) -> bool
    where
        K: Ord,
    {
        let (Some(touched), true) = (self.touched.as_ref(), self.reuse) else { return false };
        let Some(link) = self.links.get_mut(&step.key) else { return false };
        let clean = link.parents == step.parents && values.contains_key(&step.key) && !F::touched_by(snapshot, &step.key, touched) && (self.moved.is_empty() || !step.parents.iter().any(|parent| self.moved.contains(parent)));
        if clean {
            link.epoch = self.epoch;
            self.carried += 1;
        }
        clean
    }

    fn confirms(&mut self, step: &InferenceStep<K>, hash: DepHash, values: &BTreeMap<K, V>) -> bool
    where
        K: Ord,
    {
        if !self.reuse {
            return false;
        }
        let Some(link) = self.links.get_mut(&step.key) else { return false };
        let same = link.hash == hash && link.parents == step.parents && values.contains_key(&step.key);
        if same {
            link.epoch = self.epoch;
            self.confirmed += 1;
        }
        same
    }

    fn record(&mut self, step: &InferenceStep<K>, hash: DepHash, old: Option<V>) {
        let mut data = encode(&step.key);
        data.extend_from_slice(&hash.0);
        let entry = lanes(semio_framework_hash::hash(&data).as_bytes());
        add(&mut self.digest, &entry);
        match self.links.get_mut(&step.key) {
            Some(link) => {
                sub(&mut self.digest, &link.entry);
                link.hash = hash;
                link.entry = entry;
                link.epoch = self.epoch;
                link.parents.clone_from(&step.parents);
            }
            None => {
                self.links.insert(step.key.clone(), Link { hash, parents: step.parents.clone(), entry, epoch: self.epoch });
            }
        }
        self.moved.insert(step.key.clone());
        self.changes.push(FieldChange { key: step.key.clone(), old });
    }

    fn sweep(&mut self, values: &mut BTreeMap<K, V>, plan: &[InferenceStep<K>])
    where
        K: Ord,
    {
        if std::mem::replace(&mut self.swept, true) || values.len() == plan.len() {
            return;
        }
        let keep: HashSet<&K> = plan.iter().map(|step| &step.key).collect();
        let gone: Vec<K> = values.keys().filter(|key| !keep.contains(key)).cloned().collect();
        for key in gone {
            let old = values.remove(&key);
            if let Some(link) = self.links.remove(&key) {
                sub(&mut self.digest, &link.entry);
            }
            self.changes.push(FieldChange { key, old });
        }
    }
}

fn add(digest: &mut [u64; 4], entry: &[u64; 4]) {
    for (lane, part) in digest.iter_mut().zip(entry) {
        *lane = lane.wrapping_add(*part);
    }
}

fn sub(digest: &mut [u64; 4], entry: &[u64; 4]) {
    for (lane, part) in digest.iter_mut().zip(entry) {
        *lane = lane.wrapping_sub(*part);
    }
}

/// 🔄 A resumable update of one field after a diff: the plan walk of [`step_field_update`] with the field's previous values and links in hand. It can be [`cancel`](Self::cancel)led; what it finished stays valid.
pub struct FieldUpdate<K, V> {
    field: &'static str,
    values: BTreeMap<K, V>,
    track: Track<K, V>,
    cursor: InferenceCursor<K>,
    gated: bool,
    computed: usize,
    hits: usize,
}

impl<K: Eq + std::hash::Hash, V> FieldUpdate<K, V> {
    /// 🛑 Cancels the in-flight compute; every later step refuses with [`InferenceError::Cancelled`] and the session distrusts the values until a whole update ran.
    pub fn cancel(&mut self) {
        self.cursor.cancel();
    }

    pub fn is_done(&self) -> bool {
        self.gated || self.cursor.is_done()
    }

    /// 📈 Finished entities as a fraction of the plan (one when the diff touched nothing of the field).
    pub fn fraction(&self) -> f32 {
        if self.gated { 1.0 } else { self.cursor.fraction() }
    }

    /// ✅ Entities of the plan finished so far (computed, served by the cache, confirmed or carried).
    pub fn completed(&self) -> usize {
        self.cursor.completed()
    }

    /// 🧮 Entities in the plan; zero until the first step has planned.
    pub fn total(&self) -> usize {
        self.cursor.total()
    }

    /// 🛑 Whether [`cancel`](Self::cancel) was called.
    pub fn is_cancelled(&self) -> bool {
        self.cursor.is_cancelled()
    }

    /// 🗂️ The values as far as the update has come.
    pub fn values(&self) -> &BTreeMap<K, V> {
        &self.values
    }
}
//#endregion 🔖️Update

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
    drive::<P, F>(snapshot, cache, cursor, values, fuel, hashing, None)
}

fn missing<K: ToValue>(key: &K, parent: &K) -> InferenceError {
    InferenceError::MissingParent { key: key_text(key), parent: key_text(parent) }
}

fn commit<K: Clone + Eq + std::hash::Hash + Ord + ToValue, V>(values: &mut BTreeMap<K, V>, hashes: &mut HashMap<K, DepHash>, track: Option<&mut Track<K, V>>, step: &InferenceStep<K>, hash: Option<DepHash>, value: V) {
    let old = values.insert(step.key.clone(), value);
    match (track, hash) {
        (Some(track), Some(hash)) => track.record(step, hash, old),
        (None, Some(hash)) => {
            hashes.insert(step.key.clone(), hash);
        }
        _ => {}
    }
}

fn drive<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values: &mut BTreeMap<F::Key, F::Value>, fuel: usize, hashing: bool, mut track: Option<&mut Track<F::Key, F::Value>>) -> Result<InferenceStepReport, InferenceError> {
    if cursor.cancelled {
        return Err(InferenceError::Cancelled);
    }
    if cursor.plan.is_none() {
        cursor.plan = Some(F::plan(snapshot));
    }
    let InferenceCursor { plan, next, hashes, pending, progress, .. } = &mut *cursor;
    let plan = plan.as_deref().expect("the plan is set above");
    let mut remaining = fuel.max(1);
    let mut report = InferenceStepReport::default();

    while *next < plan.len() {
        let step = &plan[*next];
        if pending.is_none() && track.as_deref_mut().is_some_and(|track| track.carries::<P, F>(snapshot, step, values)) {
            *next += 1;
            continue;
        }
        let dep_hash = if hashing {
            let mut parent_hashes: Vec<DepHash> = Vec::with_capacity(step.parents.len());
            for parent in &step.parents {
                let hash = match track.as_deref() {
                    Some(track) => track.hash_of(parent),
                    None => hashes.get(parent).copied(),
                };
                parent_hashes.push(hash.ok_or_else(|| missing(&step.key, parent))?);
            }
            let input = encode(&F::dep_input(snapshot, &step.key, &step.parents));
            Some(if step.parents.is_empty() { DepHash::root(F::FIELD_ID, F::SCHEMA_VERSION, &input) } else { DepHash::chain(F::FIELD_ID, F::SCHEMA_VERSION, &input, &parent_hashes) })
        } else {
            None
        };

        if pending.is_none() {
            if let Some((hash, track)) = dep_hash.zip(track.as_deref_mut()) {
                if track.confirms(step, hash, values) {
                    *next += 1;
                    continue;
                }
            }
            if let Some((hash, value)) = dep_hash.zip(cache.as_deref_mut()).and_then(|(hash, cache)| cache.get::<F::Value>(hash).map(|value| (hash, value))) {
                commit(values, hashes, track.as_deref_mut(), step, Some(hash), value);
                *next += 1;
                report.hits += 1;
                continue;
            }
        }

        let mut parent_values: Vec<F::Value> = Vec::with_capacity(step.parents.len());
        for parent in &step.parents {
            parent_values.push(values.get(parent).cloned().ok_or_else(|| missing(&step.key, parent))?);
        }
        match F::compute_step(snapshot, &step.key, &parent_values, pending, remaining) {
            Err(fault) => {
                if let Some(mut stale) = pending.take() {
                    stale.cancel();
                }
                return Err(InferenceError::Compute { key: key_text(&step.key), fault });
            }
            Ok(ComputeStep::Working { fuel_used, progress: partial }) => {
                *progress = partial;
                report.fuel_used += fuel_used.max(1);
                break;
            }
            Ok(ComputeStep::Done { value, fuel_used }) => {
                *pending = None;
                *progress = 0.0;
                if let Some((hash, cache)) = dep_hash.zip(cache.as_deref_mut()).filter(|(_, cache)| cache.enabled()) {
                    cache.insert(hash, value.clone(), F::value_bytes(&value));
                }
                commit(values, hashes, track.as_deref_mut(), step, dep_hash, value);
                *next += 1;
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

    report.done = *next >= plan.len();
    if let Some(track) = track.filter(|_| report.done) {
        track.sweep(values, plan);
    }
    report.progress = cursor.fraction();
    Ok(report)
}

/// ⏩ The unbounded driver: [`infer_field_step`] with unlimited fuel until the run is done. Fallible so a plan-order violation or a failing compute is an error value.
pub fn try_infer_field<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>) -> Result<BTreeMap<F::Key, F::Value>, InferenceError> {
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    let hashing = cache.as_deref().is_some_and(InferenceCache::enabled);
    loop {
        if drive::<P, F>(snapshot, cache.as_deref_mut(), &mut cursor, &mut values, usize::MAX, hashing, None)?.done {
            return Ok(values);
        }
    }
}

/// ⏩ The plain driver: [`try_infer_field`] for fields whose plan is a topological order and whose compute is total; a violation of that contract is a loud failure, never a silently misaligned compute.
pub fn infer_field<P, F: InferredField<P>>(snapshot: &P, cache: Option<&mut InferenceCache>) -> BTreeMap<F::Key, F::Value> {
    try_infer_field::<P, F>(snapshot, cache).unwrap_or_else(|error| panic!("{error}"))
}

/// ⏭️ One slice of an [`InferenceSession::begin_update`]: walks the plan of the new snapshot with the same fuel and cancellation as [`infer_field_step`], but an entity that neither the diff
/// ([`InferredField::touched_by`]) nor a moved parent reaches is carried with its previous value and hash (no dependency evaluation, no hashing), and an entity whose dependency hash did not move keeps its previous value
/// (no compute); only the rest is looked up in `cache` or computed. Entities that left the plan are removed when the plan is done.
pub fn step_field_update<P, F: InferredField<P>>(snapshot: &P, cache: &mut InferenceCache, update: &mut FieldUpdate<F::Key, F::Value>, fuel: usize) -> Result<InferenceStepReport, InferenceError> {
    if update.gated {
        return Ok(InferenceStepReport { done: true, progress: 1.0, ..InferenceStepReport::default() });
    }
    let report = drive::<P, F>(snapshot, Some(cache), &mut update.cursor, &mut update.values, fuel, true, Some(&mut update.track))?;
    update.computed += report.computed;
    update.hits += report.hits;
    Ok(report)
}

/// ⏩ The unbounded diff-driven update: [`InferenceSession::begin_update`], [`step_field_update`] to the end and [`InferenceSession::finish_update`]. The values stay in the session ([`InferenceSession::field_values`]);
/// the delta names the entities that moved, so a consumer copies only those. Fails like [`try_infer_field`]; the session then distrusts its values and the next update examines every entity.
pub fn infer_field_delta<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> Result<FieldDelta<F::Key, F::Value>, InferenceError>
where
    F: InferredField<P>,
    D: crate::os_spr::command::DiffRegions,
{
    let mut update = session.begin_update::<P, F, D>(diff, cache.enabled());
    let outcome = loop {
        match step_field_update::<P, F>(snapshot, cache, &mut update, usize::MAX) {
            Ok(report) if report.done => break Ok(()),
            Ok(_) => {}
            Err(error) => break Err(error),
        }
    };
    let delta = session.finish_update(update);
    outcome.map(|()| delta)
}

/// ⏩ Diff-gated variant: if `diff.touches()` doesn't intersect `F::reads()`, returns the session's
/// previous full result for this field unchanged (tier-1 gate) instead of walking the plan at all.
/// Otherwise it updates the session through [`infer_field_delta`] and returns a copy of the values.
pub async fn infer_field_after_diff<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> BTreeMap<F::Key, F::Value>
where
    F: InferredField<P>,
    D: crate::os_spr::command::DiffRegions,
{
    infer_field_delta::<P, F, D>(snapshot, diff, session, cache).unwrap_or_else(|error| panic!("{error}"));
    session.field_values::<P, F>().cloned().unwrap_or_default()
}
//#endregion 🔖️Driver

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
