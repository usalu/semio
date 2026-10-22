//! 🗄️ Cache and session of the inference cache: the owned value erasure, the content-addressed cache with its statistics and the per-session entry tables. See [`super`].

use super::*;

//#region 🔖️Cache
/// 🧬️ Concrete inference values carry a mandatory typed retirement producer through erasure.
trait InferenceOwnedValue:Any+Send+Sync {
 fn as_any(&self)->&dyn Any;
 fn into_any(self:Box<Self>)->Box<dyn Any+Send+Sync>;
 fn boxed_retirement_birth_bytes(&self)->usize;
 fn retire_boxed(self:Box<Self>)->Box<dyn RetirementCursor>;
}

impl<T:RetireOwned+Sync> InferenceOwnedValue for T {
 fn as_any(&self)->&dyn Any{self}
 fn into_any(self:Box<Self>)->Box<dyn Any+Send+Sync>{self}
 fn boxed_retirement_birth_bytes(&self)->usize{semio_framework_value::retirement::boxed_owner_birth_bytes::<T>()}
 fn retire_boxed(self:Box<Self>)->Box<dyn RetirementCursor>{self.retirement()}
}

impl RetireOwned for Box<dyn InferenceOwnedValue>{
 fn retirement(self)->Box<dyn RetirementCursor>{self.retire_boxed()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(self.as_ref().boxed_retirement_birth_bytes())}
 fn controlled_retirement_supported()->bool{true}
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct CacheEntry {
    pub(super) value: Box<dyn InferenceOwnedValue>,
    byte_len: usize,
    tick: u64,
}

/// 🧠 Content-addressed inference value cache — mirrors `semio_framework_2d_compute::EngineCache`'s LRU/byte-budget
/// mechanism, keyed by [`DepHash`] instead of a raw content hash of caller-supplied input. Values are held typed
/// and in memory (no encode/decode round trip), recency is a monotonic tick so a hit is O(log n), and an entry that
/// alone exceeds the budget is never stored, so `used_bytes` never exceeds `budget_bytes`.
#[derive(semio_framework_value::RetireOwned)]
pub struct InferenceCache {
    config: InferenceCacheConfig,
    pub(super) entries: HashMap<DepHash, CacheEntry>,
    recency: BTreeMap<u64, DepHash>,
    pub(super) clock: u64,
    pub(super) used_bytes: usize,
    pub(super) stats: InferenceCacheStats,
}

/// 📊 Hit/miss counters — populated only when `config.record_stats` is set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
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

    pub(super) fn get<V: Clone + 'static>(&mut self, key: DepHash) -> Option<V> {
        if !self.config.enabled {
            return None;
        }
        let hit = self.entries.get(&key).and_then(|entry| entry.value.as_ref().as_any().downcast_ref::<V>().cloned());
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

    pub(super) fn insert<V: RetireOwned + Send + Sync + 'static>(&mut self, key: DepHash, value: V, byte_len: usize) {
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
        self.entries.insert(key, CacheEntry { value: Box::new(value), byte_len, tick: self.clock });
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
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Link<K> {
    pub(super) hash: DepHash,
    pub(super) parents: Vec<K>,
    pub(super) entry: [u64; 4],
    pub(super) epoch: u64,
}

/// 🧠 What a field keeps between two runs: its values, one [`Link`] per value, the digest of all links and whether the values are known to match the last snapshot.
#[derive(semio_framework_value::RetireOwned)]
struct Stored<K:Eq+std::hash::Hash+Ord, V> {
    pub(super) values: BTreeMap<K, V>,
    pub(super) links: HashMap<K, Link<K>>,
    pub(super) digest: [u64; 4],
    pub(super) epoch: u64,
    sound: bool,
}

impl<K:Eq+std::hash::Hash+Ord, V> Stored<K, V> {
    fn empty() -> Self {
        Self { values: BTreeMap::new(), links: HashMap::new(), digest: [0; 4], epoch: 0, sound: false }
    }
}

#[derive(semio_framework_value::RetireOwned)]
struct SessionEntry {
    pub(super) digest: [u64; 4],
    stored: Box<dyn InferenceOwnedValue>,
}

/// 🧭 Per-artifact-instance state of the diff-driven entry points: per field id the values of the last run with their dependency links. [`infer_field_after_diff`] and the stepped [`InferenceSession::begin_update`] consult it
/// to skip a whole field the diff does not touch, to carry every entity a diff and its parents did not move, and to skip the compute of an entity whose dependency hash did not move.
#[derive(Default)]
#[derive(semio_framework_value::RetireOwned)]
pub struct InferenceSession {
    pub(super) entries: HashMap<&'static str, SessionEntry>,
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
        self.entries.get(F::FIELD_ID).and_then(|entry| entry.stored.as_ref().as_any().downcast_ref::<Stored<F::Key, F::Value>>()).map(|stored| &stored.values)
    }

    /// 🚦 Starts an update of field `F` after `diff`. The field's values move into the update; [`finish_update`](Self::finish_update) gives them back. With `reuse` an entity is carried or confirmed instead of recomputed
    /// (the caller passes whether its cache is enabled: with no cache every update is a plain recompute).
    pub fn begin_update<P, F: InferredField<P>, D: semio_framework_os_command::DiffRegions>(&mut self, diff: &D, reuse: bool) -> FieldUpdate<F::Key, F::Value> {
        let stored = self.entries.remove(F::FIELD_ID).and_then(|entry| entry.stored.into_any().downcast::<Stored<F::Key, F::Value>>().ok()).map_or_else(Stored::empty, |stored| *stored);
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

    /// 🎟️ Admits the original first field run before any keyed entry, diff or retained graph exists.
    pub fn begin_update_funded<P,F:InferredField<P>>(&mut self,reuse:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<FieldUpdate<F::Key,F::Value>>,InferenceWorkFailure>{
        cx.original_source_parent()?;
        if !self.entries.is_empty(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original inference session entry lookup and diff donation require their funded producer").into())}
        let grant=cx.retained_grant();
        if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        cx.consume_fuel(1);cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()})?;
        let stored=Stored::empty();
        Ok(Some(FieldUpdate{field:F::FIELD_ID,values:stored.values,track:Track{links:stored.links,digest:stored.digest,epoch:1,touched:None,reuse,moved:HashSet::new(),changes:Vec::new(),carried:0,confirmed:0,swept:false},cursor:InferenceCursor::new(),gated:false,computed:0,hits:0}))
    }

    /// 🏁 Ends an update, finished or not: the values stay in the session and are trusted by the next update only when this one ran to its end.
    pub fn finish_update<K: RetireOwned + Clone + Eq + std::hash::Hash + Ord + Send + Sync + 'static, V: RetireOwned + Send + Sync + 'static>(&mut self, update: FieldUpdate<K, V>) -> FieldDelta<K, V> {
        let done = update.gated || update.cursor.is_done();
        let FieldUpdate { field, values, track, gated, computed, hits, .. } = update;
        let Track { links, digest, epoch, changes, carried, confirmed, .. } = track;
        self.entries.insert(field, SessionEntry { digest, stored: Box::new(Stored { values, links, digest, epoch, sound: done }) });
        FieldDelta { gated, done, changes, computed, hits, carried, confirmed }
    }
}
//#endregion 🔖️Session
