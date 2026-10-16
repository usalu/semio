//! 🗺️ `ShardedMap`: the fixed-shard concurrent map and its deterministic shard hasher.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{PoisonError, RwLock};

//#region 🗺️ShardedMap
struct ShardHasher(u64);

impl Default for ShardHasher {
    fn default() -> Self {
        Self(0xcbf29ce484222325)
    }
}

impl Hasher for ShardHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }
}

/// 🗺️ Fixed-shard concurrent map whose closure API never leaks a lock guard or external type.
pub struct ShardedMap<K, V, const SHARDS: usize = 32> {
    shards: [RwLock<HashMap<K, V>>; SHARDS],
}

impl<K, V, const SHARDS: usize> Default for ShardedMap<K, V, SHARDS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, const SHARDS: usize> ShardedMap<K, V, SHARDS> {
    /// 🌱️ Creates an empty map with a fixed, non-zero shard count.
    pub fn new() -> Self {
        assert!(SHARDS != 0, "ShardedMap requires at least one shard");
        Self { shards: std::array::from_fn(|_| RwLock::new(HashMap::new())) }
    }

    /// 🔢️ Returns the exact number of entries across all shards.
    pub fn len(&self) -> usize {
        self.shards.iter().map(|shard| shard.read().unwrap_or_else(PoisonError::into_inner).len()).sum()
    }

    /// 🫙 Reports whether every shard is empty.
    pub fn is_empty(&self) -> bool {
        self.shards.iter().all(|shard| shard.read().unwrap_or_else(PoisonError::into_inner).is_empty())
    }

    /// 🧭 Visits every key/value pair while holding one read shard at a time.
    pub fn for_each(&self, mut visit: impl FnMut(&K, &V)) {
        for shard in &self.shards {
            let entries = shard.read().unwrap_or_else(PoisonError::into_inner);
            for (key, value) in entries.iter() {
                visit(key, value);
            }
        }
    }
}

impl<K, V, const SHARDS: usize> ShardedMap<K, V, SHARDS>
where
    K: Eq + Hash,
{
    fn shard_index<Q>(key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        let mut hasher = ShardHasher::default();
        key.hash(&mut hasher);
        hasher.finish() as usize % SHARDS
    }

    /// ➕️ Inserts or replaces one value and returns the previous value.
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.shards[Self::shard_index(&key)].write().unwrap_or_else(PoisonError::into_inner).insert(key, value)
    }

    /// 🔎️ Reads one entry through a closure that cannot outlive its shard guard.
    pub fn with<Q, R>(&self, key: &Q, read: impl FnOnce(Option<&V>) -> R) -> R
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let entries = self.shards[Self::shard_index(key)].read().unwrap_or_else(PoisonError::into_inner);
        read(entries.get(key))
    }

    /// ✏️ Mutates one existing entry through a closure that cannot retain its shard guard.
    pub fn with_mut<Q, R>(&self, key: &Q, mutate: impl FnOnce(Option<&mut V>) -> R) -> R
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let mut entries = self.shards[Self::shard_index(key)].write().unwrap_or_else(PoisonError::into_inner);
        mutate(entries.get_mut(key))
    }

    /// 📤️ Clones one value without exposing a guard.
    pub fn get_cloned<Q>(&self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
        V: Clone,
    {
        self.with(key, |value| value.cloned())
    }

    /// 🌱️ Returns the existing value or atomically inserts and clones a new one.
    pub fn get_or_insert_with_cloned(&self, key: K, create: impl FnOnce() -> V) -> V
    where
        V: Clone,
    {
        self.shards[Self::shard_index(&key)].write().unwrap_or_else(PoisonError::into_inner).entry(key).or_insert_with(create).clone()
    }

    /// 🧰️ Mutates a value, inserting its default when absent.
    pub fn mutate_or_default<R>(&self, key: K, mutate: impl FnOnce(&mut V) -> R) -> R
    where
        V: Default,
    {
        let mut entries = self.shards[Self::shard_index(&key)].write().unwrap_or_else(PoisonError::into_inner);
        mutate(entries.entry(key).or_default())
    }

    /// 🗑️ Removes one entry and returns its value.
    pub fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.shards[Self::shard_index(key)].write().unwrap_or_else(PoisonError::into_inner).remove(key)
    }

    /// 🧹️ Removes an entry only when the in-lock predicate accepts its current value.
    pub fn remove_if<Q>(&self, key: &Q, predicate: impl FnOnce(&V) -> bool) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let mut entries = self.shards[Self::shard_index(key)].write().unwrap_or_else(PoisonError::into_inner);
        if entries.get(key).is_some_and(predicate) {
            entries.remove(key);
            true
        } else {
            false
        }
    }
}
//#endregion 🗺️ShardedMap
