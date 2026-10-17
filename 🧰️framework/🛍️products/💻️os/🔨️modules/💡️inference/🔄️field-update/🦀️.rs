//! 🔄️ Field update of the inference cache: the field change and delta records, the lane tracks and the incremental field update. See [`super`].

use super::*;

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
pub(super) struct Track<K, V> {
    pub(super) links: HashMap<K, Link<K>>,
    pub(super) digest: [u64; 4],
    pub(super) epoch: u64,
    pub(super) touched: Option<TouchedPaths>,
    pub(super) reuse: bool,
    pub(super) moved: HashSet<K>,
    pub(super) changes: Vec<FieldChange<K, V>>,
    pub(super) carried: usize,
    pub(super) confirmed: usize,
    pub(super) swept: bool,
}

fn lanes(bytes: &[u8; 32]) -> [u64; 4] {
    std::array::from_fn(|at| u64::from_le_bytes(bytes[at * 8..at * 8 + 8].try_into().expect("eight bytes")))
}

impl<K: Clone + Eq + std::hash::Hash + ToValue, V> Track<K, V> {
    pub(super) fn hash_of(&self, key: &K) -> Option<DepHash> {
        self.links.get(key).filter(|link| link.epoch == self.epoch).map(|link| link.hash)
    }

    pub(super) fn carries<P, F: InferredField<P, Key = K, Value = V>>(&mut self, snapshot: &P, step: &InferenceStep<K>, values: &BTreeMap<K, V>) -> bool
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

    pub(super) fn confirms(&mut self, step: &InferenceStep<K>, hash: DepHash, values: &BTreeMap<K, V>) -> bool
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

    pub(super) fn record(&mut self, step: &InferenceStep<K>, hash: DepHash, old: Option<V>) {
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

    pub(super) fn sweep(&mut self, values: &mut BTreeMap<K, V>, plan: &[InferenceStep<K>])
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

pub(super) fn add(digest: &mut [u64; 4], entry: &[u64; 4]) {
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
    pub(super) field: &'static str,
    pub(super) values: BTreeMap<K, V>,
    pub(super) track: Track<K, V>,
    pub(super) cursor: InferenceCursor<K>,
    pub(super) gated: bool,
    pub(super) computed: usize,
    pub(super) hits: usize,
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
    /// 🧭️ Borrows the funded order retained by this update's actual cursor.
    pub fn original_plan(&self)->Option<&OriginalInferencePlan>{self.cursor.original_plan()}
    /// 🧹️ Closes the actual dynamic compute owner before its original pending box releases.
    pub fn close_original_pending_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,InferenceWorkFailure>{self.cursor.close_original_pending_step(cx)}
    /// ♻️ Advances the actual cursor's plan retirement under its original caller.
    pub fn close_original_plan_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,InferenceWorkFailure>{self.cursor.close_original_plan_step(cx)}

    pub fn is_cancelled(&self) -> bool {
        self.cursor.is_cancelled()
    }

    /// 🗂️ The values as far as the update has come.
    pub fn values(&self) -> &BTreeMap<K, V> {
        &self.values
    }
}
//#endregion 🔖️Update
