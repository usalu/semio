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

use serde::{de::DeserializeOwned, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};

//#region 🔖️DepHash
/// 🔑 Per-entity dependency hash — one link in a merkle dependency chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
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
        let own_hex = semio_framework_hash::hash(&own).to_hex().to_string();
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

/// 🕸️ One inferred field family, computed entity-by-entity over a dependency DAG. This is the
/// trait real derivation math (e.g. a flatten engine) implements; an artifact's top-level
/// `Inference::infer` assembles its `XInference` struct from one or more `InferredField`s.
pub trait InferredField<P>: Send + Sync + 'static {
    type Key: Clone + Eq + std::hash::Hash + Ord + Send + Sync + Serialize + DeserializeOwned;
    type Value: Clone + Serialize + DeserializeOwned + Send + Sync;

    const FIELD_ID: &'static str;
    const SCHEMA_VERSION: u32;

    /// 🗺️ Coarse tier-1 read-set — checked against a diff's [`crate::os_spr::command::DiffRegions::touches`]
    /// before this field's plan is even walked.
    fn reads() -> &'static [&'static str];

    /// 🧭 Deterministic topological plan over `snapshot`'s entities (roots first — entries with no
    /// parents come before anything that depends on them).
    fn plan(snapshot: &P) -> Vec<InferenceStep<Self::Key>>;

    /// 🔑 Canonical dependency-input bytes for `key` — EXACTLY the snapshot fields `compute` may
    /// read for this key (excluding parents' OWN upstream values, which are folded in separately
    /// via their already-computed [`DepHash`]es — but INCLUDING the specific edge/connector data
    /// tying `key` to each of `parents`, e.g. a compose-style attraction's params, since that lives
    /// on `key`'s own incoming edge, not on the parent's upstream chain). `parents` is `plan`'s
    /// `InferenceStep.parents` for this key, passed through so implementations don't need to
    /// re-derive "which edge connects to which parent" a second time. Honesty contract: this must
    /// cover everything `compute` reads, or a changed-but-uncovered input silently serves a stale
    /// cached value.
    fn dep_input(snapshot: &P, key: &Self::Key, parents: &[Self::Key]) -> Vec<u8>;

    /// 🧮 Pure per-entity compute, given parents' already-computed values in `plan`'s parent order.
    fn compute(snapshot: &P, key: &Self::Key, parents: &[Self::Value]) -> Self::Value;
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
    bytes: Vec<u8>,
    byte_len: usize,
}

/// 🧠 Content-addressed inference value cache — mirrors `crate::os_engine::EngineCache`'s LRU/byte-budget
/// mechanism, keyed by [`DepHash`] instead of a raw content hash of caller-supplied input.
pub struct InferenceCache {
    config: InferenceCacheConfig,
    entries: HashMap<DepHash, CacheEntry>,
    lru: VecDeque<DepHash>,
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
        Self { config, entries: HashMap::new(), lru: VecDeque::new(), used_bytes: 0, stats: InferenceCacheStats::default() }
    }

    pub async fn stats(&self) -> InferenceCacheStats {
        self.stats
    }

    /// 🧹 Explicit whole-cache invalidation (e.g. after a schema-version bump discovered at runtime).
    pub async fn clear(&mut self) {
        self.entries.clear();
        self.lru.clear();
        self.used_bytes = 0;
    }

    fn get(&mut self, key: DepHash) -> Option<Vec<u8>> {
        if !self.config.enabled {
            return None;
        }
        if let Some(entry) = self.entries.get(&key) {
            let bytes = entry.bytes.clone();
            self.touch(key);
            if self.config.record_stats {
                self.stats.hits += 1;
            }
            return Some(bytes);
        }
        if self.config.record_stats {
            self.stats.misses += 1;
        }
        None
    }

    fn insert(&mut self, key: DepHash, bytes: Vec<u8>) {
        if !self.config.enabled {
            return;
        }
        let byte_len = bytes.len();
        self.ensure_budget(byte_len);
        if self.entries.insert(key, CacheEntry { bytes, byte_len }).is_none() {
            self.lru.push_back(key);
            self.used_bytes = self.used_bytes.saturating_add(byte_len);
        }
    }

    fn touch(&mut self, key: DepHash) {
        if let Some(pos) = self.lru.iter().position(|k| *k == key) {
            self.lru.remove(pos);
        }
        self.lru.push_back(key);
    }

    fn ensure_budget(&mut self, needed: usize) {
        while self.used_bytes.saturating_add(needed) > self.config.budget_bytes {
            let Some(old) = self.lru.pop_front() else { break };
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
/// 🧭 Per-artifact-instance tier-1 gate state: one root [`DepHash`] + decoded result per field id,
/// consulted by [`infer_field_after_diff`] before even walking a field's plan.
#[derive(Default)]
pub struct InferenceSession {
    roots: HashMap<&'static str, (DepHash, Vec<u8>)>,
}

impl InferenceSession {
    pub async fn new() -> Self {
        Self::default()
    }
}
//#endregion 🔖️Session

//#region 🔖️Driver
fn encode<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).expect("inference value serialization never fails")
}

fn decode<T: DeserializeOwned>(bytes: &[u8]) -> T {
    serde_json::from_slice(bytes).expect("cached inference bytes must decode as the field's own Value type")
}

/// ⏩ THE driver: walks `F::plan(snapshot)` in order, hashing each entity's dependency chain and
/// consulting `cache` (if `Some`) before computing. `cache: None` ⇒ pure recompute — identical
/// output to a warm-cache run (cache-transparency law, proven in tests below).
pub fn infer_field<P, F: InferredField<P>>(snapshot: &P, mut cache: Option<&mut InferenceCache>) -> BTreeMap<F::Key, F::Value> {
    let plan = F::plan(snapshot);
    let mut hashes: HashMap<F::Key, DepHash> = HashMap::new();
    let mut values: BTreeMap<F::Key, F::Value> = BTreeMap::new();

    for step in plan {
        let parent_hashes: Vec<DepHash> = step.parents.iter().filter_map(|p| hashes.get(p).copied()).collect();
        let input = F::dep_input(snapshot, &step.key, &step.parents);
        let dep_hash = if step.parents.is_empty() { DepHash::root(F::FIELD_ID, F::SCHEMA_VERSION, &input) } else { DepHash::chain(F::FIELD_ID, F::SCHEMA_VERSION, &input, &parent_hashes) };

        let value = if let Some(cache) = cache.as_deref_mut() {
            match cache.get(dep_hash) {
                Some(bytes) => decode::<F::Value>(&bytes),
                None => {
                    let parent_values: Vec<F::Value> = step.parents.iter().filter_map(|p| values.get(p).cloned()).collect();
                    let computed = F::compute(snapshot, &step.key, &parent_values);
                    cache.insert(dep_hash, encode(&computed));
                    computed
                }
            }
        } else {
            let parent_values: Vec<F::Value> = step.parents.iter().filter_map(|p| values.get(p).cloned()).collect();
            F::compute(snapshot, &step.key, &parent_values)
        };

        hashes.insert(step.key.clone(), dep_hash);
        values.insert(step.key, value);
    }

    values
}

/// ⏩ Diff-gated variant: if `diff.touches()` doesn't intersect `F::reads()`, returns the session's
/// previous full result for this field unchanged (tier-1 gate) instead of walking the plan at all.
/// Falls through to [`infer_field`] (and refreshes the session) otherwise.
pub async fn infer_field_after_diff<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> BTreeMap<F::Key, F::Value>
where
    F: InferredField<P>,
    D: crate::os_spr::command::DiffRegions,
{
    if !diff.touches().intersects_any(F::reads()) {
        if let Some((_, bytes)) = session.roots.get(F::FIELD_ID) {
            return decode::<BTreeMap<F::Key, F::Value>>(bytes);
        }
    }
    // 🪡️ A future is consumed by a single `.await`; the original had `result`/`root` each awaited
    // more than once (R10 residue #2 — a bug the conversion exposed). Each is now awaited exactly
    // once, into a plain value reused by reference below.
    let result = infer_field::<P, F>(snapshot, Some(cache));
    let root = semio_framework_hash::merkle_collection(result.keys().enumerate().map(|(i, _)| i.to_string()).collect());
    let mut root_bytes = [0u8; 32];
    let _ = hex::decode_to_slice(
        &{
            let mut padded = semio_framework_hash::hash(root.as_bytes()).to_hex().to_string();
            padded.truncate(64);
            padded
        },
        &mut root_bytes,
    );
    session.roots.insert(F::FIELD_ID, (DepHash(root_bytes), encode(&result)));
    result
}
//#endregion 🔖️Driver

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    // Synthetic 3-node DAG: a "root" entity feeding two "leaf" entities, each Value = i64 sum of
    // its own weight plus every ancestor's weight — smallest fixture that exercises root vs. chain
    // hashing, multi-parent folding (leaf_both has two parents), and per-entity incrementality.
    #[derive(Clone, Debug, Default)]
    struct DagSnapshot {
        weights: BTreeMap<&'static str, i64>,
    }

    struct WeightSum;
    impl InferredField<DagSnapshot> for WeightSum {
        // 🔑️ String, not &'static str: DeserializeOwned (needed for the session's whole-result
        // cache in `infer_field_after_diff`) can't be satisfied by a borrowed str — matches real
        // usage (e.g. Puzzle3dFlatPlane's Key = object id String).
        type Key = String;
        type Value = i64;
        const FIELD_ID: &'static str = "test.dag.weight-sum";
        const SCHEMA_VERSION: u32 = 1;
        fn reads() -> &'static [&'static str] {
            &["weights"]
        }
        fn plan(_snapshot: &DagSnapshot) -> Vec<InferenceStep<Self::Key>> {
            vec![
                InferenceStep { key: "root".to_string(), parents: vec![] },
                InferenceStep { key: "leaf_a".to_string(), parents: vec!["root".to_string()] },
                InferenceStep { key: "leaf_b".to_string(), parents: vec!["root".to_string()] },
                InferenceStep { key: "leaf_both".to_string(), parents: vec!["leaf_a".to_string(), "leaf_b".to_string()] },
            ]
        }
        fn dep_input(snapshot: &DagSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
            snapshot.weights.get(key.as_str()).copied().unwrap_or(0).to_le_bytes().to_vec()
        }
        fn compute(snapshot: &DagSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
            snapshot.weights.get(key.as_str()).copied().unwrap_or(0) + parents.iter().sum::<i64>()
        }
    }

    async fn base_snapshot() -> DagSnapshot {
        DagSnapshot { weights: BTreeMap::from([("root", 1), ("leaf_a", 2), ("leaf_b", 3), ("leaf_both", 4)]) }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️PlanShape
    #[semio_framework_async_macros::async_test]
    async fn infer_field_computes_expected_values_over_the_dag() {
        let snapshot = base_snapshot().await;
        let values = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);
        assert_eq!(values["root"], 1);
        assert_eq!(values["leaf_a"], 3); // 2 + root(1)
        assert_eq!(values["leaf_b"], 4); // 3 + root(1)
        assert_eq!(values["leaf_both"], 4 + 3 + 4); // 4 + leaf_a(3) + leaf_b(4)
    }
    //#endregion 🧪️PlanShape

    //#region 🧪️CacheTransparencyLaw
    #[semio_framework_async_macros::async_test]
    async fn disabled_cache_matches_pure_recompute() {
        let snapshot = base_snapshot().await;
        let pure = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);

        let mut disabled_cache = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
        let via_disabled_cache = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut disabled_cache));
        assert_eq!(pure, via_disabled_cache);
    }

    #[semio_framework_async_macros::async_test]
    async fn cold_and_warm_cache_match_pure_recompute() {
        let snapshot = base_snapshot().await;
        let pure = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);

        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
        let cold = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache));
        assert_eq!(pure, cold);
        assert_eq!(cache.stats().await.hits, 0);
        assert!(cache.stats().await.misses > 0);

        let warm = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache));
        assert_eq!(pure, warm);
        assert!(cache.stats().await.hits > 0, "second run over the same snapshot must hit the warm cache");
    }

    #[semio_framework_async_macros::async_test]
    async fn tiny_budget_eviction_storm_still_matches_pure_recompute() {
        let snapshot = base_snapshot().await;
        let pure = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, budget_bytes: 1, ..Default::default() }).await;
        let via_tiny_cache = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache));
        assert_eq!(pure, via_tiny_cache, "an eviction storm must never change the computed result, only cache hit rate");
    }
    //#endregion 🧪️CacheTransparencyLaw

    //#region 🧪️IncrementalityLaw
    #[semio_framework_async_macros::async_test]
    async fn changing_a_leaf_weight_only_recomputes_that_leaf_and_its_descendants() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
        let base = base_snapshot().await;
        let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.weights.insert("leaf_a", 99);
        let before = cache.stats().await;
        let values = infer_field::<DagSnapshot, WeightSum>(&changed, Some(&mut cache));
        let after = cache.stats().await;

        // root is untouched by leaf_a's weight change (identical dep chain) => cache hit.
        // leaf_a changed directly => miss. leaf_b's own dep chain (root's hash) is unchanged => hit.
        // leaf_both depends on leaf_a's NEW value => its dep_input is unaffected but its parent
        // chain folds leaf_a's (changed) hash, so it also misses.
        assert_eq!(after.misses - before.misses, 2, "only leaf_a and leaf_both (its descendant) may miss");
        assert_eq!(values["leaf_a"], 99 + 1);
        assert_eq!(values["leaf_b"], 3 + 1, "leaf_b must be unaffected by leaf_a's weight change");
        assert_eq!(values["root"], 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn changing_the_root_weight_recomputes_the_entire_subtree() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
        let base = base_snapshot().await;
        let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.weights.insert("root", 999);
        let before = cache.stats().await;
        let _ = infer_field::<DagSnapshot, WeightSum>(&changed, Some(&mut cache));
        let after = cache.stats().await;

        assert_eq!(after.misses - before.misses, 4, "changing the root must miss for every entity in the DAG (all four are its descendants, root included)");
    }

    #[semio_framework_async_macros::async_test]
    async fn identical_snapshot_recompute_is_all_cache_hits() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
        let base = base_snapshot().await;
        let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));
        let before = cache.stats().await;
        let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));
        let after = cache.stats().await;
        assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
        assert_eq!(after.hits - before.hits, 4);
    }
    //#endregion 🧪️IncrementalityLaw

    //#region 🧪️VersionSaltLaw
    struct WeightSumV2;
    impl InferredField<DagSnapshot> for WeightSumV2 {
        type Key = String;
        type Value = i64;
        const FIELD_ID: &'static str = "test.dag.weight-sum";
        const SCHEMA_VERSION: u32 = 2;
        fn reads() -> &'static [&'static str] {
            WeightSum::reads()
        }
        fn plan(snapshot: &DagSnapshot) -> Vec<InferenceStep<Self::Key>> {
            WeightSum::plan(snapshot)
        }
        fn dep_input(snapshot: &DagSnapshot, key: &Self::Key, parents: &[Self::Key]) -> Vec<u8> {
            WeightSum::dep_input(snapshot, key, parents)
        }
        fn compute(snapshot: &DagSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
            WeightSum::compute(snapshot, key, parents)
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_version_bump_yields_zero_hits_on_an_otherwise_warm_cache() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
        let base = base_snapshot().await;
        let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));
        let before = cache.stats().await;
        let _ = infer_field::<DagSnapshot, WeightSumV2>(&base, Some(&mut cache));
        let after = cache.stats().await;
        assert_eq!(after.hits, before.hits, "a version-salted key must never collide with the prior version's entries");
        assert_eq!(after.misses - before.misses, 4);
    }
    //#endregion 🧪️VersionSaltLaw

    //#region 🧪️DepHash
    #[semio_framework_async_macros::async_test]
    async fn dep_hash_root_is_deterministic_and_input_sensitive() {
        let a = DepHash::root("field", 1, b"input-a");
        let b = DepHash::root("field", 1, b"input-a");
        let c = DepHash::root("field", 1, b"input-b");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[semio_framework_async_macros::async_test]
    async fn dep_hash_chain_is_order_independent_over_parent_set() {
        let p1 = DepHash::root("f", 1, b"p1");
        let p2 = DepHash::root("f", 1, b"p2");
        let forward = DepHash::chain("f", 1, b"self", &[p1, p2]);
        let backward = DepHash::chain("f", 1, b"self", &[p2, p1]);
        assert_eq!(forward, backward, "two entities with the same parent SET in different orders must hash identically");
    }

    #[semio_framework_async_macros::async_test]
    async fn dep_hash_chain_differs_from_root_for_the_same_input() {
        let root = DepHash::root("f", 1, b"same");
        let chained = DepHash::chain("f", 1, b"same", &[DepHash::root("f", 1, b"parent")]);
        assert_ne!(root, chained);
    }
    //#endregion 🧪️DepHash

    //#region 🧪️Config
    #[semio_framework_async_macros::async_test]
    async fn default_config_is_disabled() {
        assert!(!InferenceCacheConfig::default().enabled);
    }

    #[semio_framework_async_macros::async_test]
    async fn clear_drops_every_entry_and_resets_used_bytes() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
        let _ = infer_field::<DagSnapshot, WeightSum>(&base_snapshot().await, Some(&mut cache));
        assert!(!cache.entries.is_empty());
        cache.clear().await;
        assert!(cache.entries.is_empty());
        assert_eq!(cache.used_bytes, 0);
    }
    //#endregion 🧪️Config
}
//#endregion 🧪️Tests
