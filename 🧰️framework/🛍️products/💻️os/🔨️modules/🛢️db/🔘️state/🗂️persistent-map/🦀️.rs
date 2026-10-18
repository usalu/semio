//! 🗂️ Persistent map: the HAMT nodes and `PMap`. See [`super`].

use super::*;

//#region 🔖️PMap
/// 🔀️ Internal, deterministic (unseeded) hash used only to route keys to HAMT buckets —
/// see the module doc's design-choice note on why this is not blake3.
fn hash_key<K: std::hash::Hash>(key: &K) -> u64 {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

/// 🪜️ 5 bits routed per HAMT level (32-way branching, per the contract).
const HAMT_BITS: u32 = 5;

/// 🛑️ Beyond this depth, the 64-bit routing hash is exhausted (13 × 5 = 65 > 64); further
/// entries that would need a 13th level instead fall into a linear `Collision` bucket. Depths
/// `0..HAMT_MAX_DEPTH` may branch; depth `HAMT_MAX_DEPTH` never does.
const HAMT_MAX_DEPTH: u8 = 12;

fn hamt_index(hash: u64, depth: u8) -> u32 {
    let shift = (depth as u32) * HAMT_BITS;
    ((hash >> shift) & 0x1F) as u32
}

enum HamtNode<K, V> {
    Empty,
    Leaf {
        hash: u64,
        key: K,
        value: V,
    },
    /// 💥️ A bucket of entries that share a routing hash (or that ran out of depth to
    /// split further) — degrades to a linear scan, which is fine since real-world key
    /// distributions make this vanishingly rare.
    Collision {
        entries: Vec<(u64, K, V)>,
    },
    Branch {
        bitmap: u32,
        children: Vec<Arc<HamtNode<K, V>>>,
    },
}

fn hamt_branch_from_two<K: Clone + Eq, V: Clone>(depth: u8, h1: u64, k1: K, v1: V, h2: u64, k2: K, v2: V) -> Arc<HamtNode<K, V>> {
    if depth >= HAMT_MAX_DEPTH {
        return Arc::new(HamtNode::Collision { entries: vec![(h1, k1, v1), (h2, k2, v2)] });
    }
    let i1 = hamt_index(h1, depth);
    let i2 = hamt_index(h2, depth);
    if i1 == i2 {
        let child = hamt_branch_from_two(depth + 1, h1, k1, v1, h2, k2, v2);
        Arc::new(HamtNode::Branch { bitmap: 1u32 << i1, children: vec![child] })
    } else {
        let leaf1 = Arc::new(HamtNode::Leaf { hash: h1, key: k1, value: v1 });
        let leaf2 = Arc::new(HamtNode::Leaf { hash: h2, key: k2, value: v2 });
        let bitmap = (1u32 << i1) | (1u32 << i2);
        let children = if i1 < i2 { vec![leaf1, leaf2] } else { vec![leaf2, leaf1] };
        Arc::new(HamtNode::Branch { bitmap, children })
    }
}

fn hamt_insert<K: Clone + Eq, V: Clone>(node: &Arc<HamtNode<K, V>>, depth: u8, hash: u64, key: K, value: V) -> Arc<HamtNode<K, V>> {
    match node.as_ref() {
        HamtNode::Empty => Arc::new(HamtNode::Leaf { hash, key, value }),
        HamtNode::Leaf { hash: h, key: k, value: v } => {
            if key == *k {
                Arc::new(HamtNode::Leaf { hash, key, value })
            } else if *h == hash || depth >= HAMT_MAX_DEPTH {
                Arc::new(HamtNode::Collision { entries: vec![(*h, k.clone(), v.clone()), (hash, key, value)] })
            } else {
                hamt_branch_from_two(depth, *h, k.clone(), v.clone(), hash, key, value)
            }
        }
        HamtNode::Collision { entries } => {
            let mut new_entries = entries.clone();
            if let Some(pos) = new_entries.iter().position(|(_, k, _)| *k == key) {
                new_entries[pos] = (hash, key, value);
            } else {
                new_entries.push((hash, key, value));
            }
            Arc::new(HamtNode::Collision { entries: new_entries })
        }
        HamtNode::Branch { bitmap, children } => {
            let i = hamt_index(hash, depth);
            let bit = 1u32 << i;
            let pos = (bitmap & (bit - 1)).count_ones() as usize;
            if bitmap & bit != 0 {
                let updated = hamt_insert(&children[pos], depth + 1, hash, key, value);
                let mut new_children = children.clone();
                new_children[pos] = updated;
                Arc::new(HamtNode::Branch { bitmap: *bitmap, children: new_children })
            } else {
                let mut new_children = children.clone();
                new_children.insert(pos, Arc::new(HamtNode::Leaf { hash, key, value }));
                Arc::new(HamtNode::Branch { bitmap: bitmap | bit, children: new_children })
            }
        }
    }
}

fn hamt_get<'a, K: Eq, V>(node: &'a HamtNode<K, V>, depth: u8, hash: u64, key: &K) -> Option<&'a V> {
    match node {
        HamtNode::Empty => None,
        HamtNode::Leaf { hash: h, key: k, value } => {
            if *h == hash && k == key {
                Some(value)
            } else {
                None
            }
        }
        HamtNode::Collision { entries } => entries.iter().find(|(_, k, _)| k == key).map(|(_, _, v)| v),
        HamtNode::Branch { bitmap, children } => {
            let i = hamt_index(hash, depth);
            let bit = 1u32 << i;
            if bitmap & bit == 0 {
                None
            } else {
                let pos = (bitmap & (bit - 1)).count_ones() as usize;
                hamt_get(&children[pos], depth + 1, hash, key)
            }
        }
    }
}

fn hamt_remove<K: Clone + Eq, V: Clone>(node: &Arc<HamtNode<K, V>>, depth: u8, hash: u64, key: &K) -> Option<Arc<HamtNode<K, V>>> {
    match node.as_ref() {
        HamtNode::Empty => None,
        HamtNode::Leaf { hash: h, key: k, .. } => {
            if *h == hash && k == key {
                None
            } else {
                Some(node.clone())
            }
        }
        HamtNode::Collision { entries } => match entries.iter().position(|(_, k, _)| k == key) {
            None => Some(node.clone()),
            Some(pos) => {
                let mut new_entries = entries.clone();
                new_entries.remove(pos);
                if new_entries.len() == 1 {
                    let (h, k, v) = new_entries.into_iter().next().expect("checked len == 1 above");
                    Some(Arc::new(HamtNode::Leaf { hash: h, key: k, value: v }))
                } else if new_entries.is_empty() {
                    None
                } else {
                    Some(Arc::new(HamtNode::Collision { entries: new_entries }))
                }
            }
        },
        HamtNode::Branch { bitmap, children } => {
            let i = hamt_index(hash, depth);
            let bit = 1u32 << i;
            if bitmap & bit == 0 {
                return Some(node.clone());
            }
            let pos = (bitmap & (bit - 1)).count_ones() as usize;
            match hamt_remove(&children[pos], depth + 1, hash, key) {
                None => {
                    let new_bitmap = bitmap & !bit;
                    if new_bitmap == 0 {
                        None
                    } else {
                        let mut new_children = children.clone();
                        new_children.remove(pos);
                        Some(Arc::new(HamtNode::Branch { bitmap: new_bitmap, children: new_children }))
                    }
                }
                Some(new_child) => {
                    if Arc::ptr_eq(&new_child, &children[pos]) {
                        Some(node.clone())
                    } else {
                        let mut new_children = children.clone();
                        new_children[pos] = new_child;
                        Some(Arc::new(HamtNode::Branch { bitmap: *bitmap, children: new_children }))
                    }
                }
            }
        }
    }
}

fn hamt_collect<'a, K, V>(node: &'a HamtNode<K, V>, out: &mut Vec<(&'a K, &'a V)>) {
    match node {
        HamtNode::Empty => {}
        HamtNode::Leaf { key, value, .. } => out.push((key, value)),
        HamtNode::Collision { entries } => {
            for (_, k, v) in entries {
                out.push((k, v));
            }
        }
        HamtNode::Branch { children, .. } => {
            for child in children {
                hamt_collect(child.as_ref(), out);
            }
        }
    }
}

/// 🗺️ A persistent (immutable, structurally-shared) hash map: a 32-way HAMT. Every
/// mutating method returns a new `PMap`; unaffected subtrees are shared via `Arc` with the map(s)
/// it was derived from, so a chain of `n` single-key edits allocates `O(n log n)` nodes total,
/// not `O(n²)`.
pub struct PMap<K, V> {
    pub root: Arc<HamtNode<K, V>>,
    pub len: usize,
}

impl<K, V> PMap<K, V> {
    pub fn new() -> Self {
        PMap { root: Arc::new(HamtNode::Empty), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 🚶️ Eagerly materializes every entry — simple to reason about at this crate's scope;
    /// a lazy tree-walking iterator is a straightforward future optimization if profiling ever
    /// shows this matters.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        let mut out = Vec::with_capacity(self.len);
        hamt_collect(self.root.as_ref(), &mut out);
        out.into_iter()
    }
}

impl<K, V> Default for PMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for PMap<K, V> {
    fn clone(&self) -> Self {
        PMap { root: self.root.clone(), len: self.len }
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> PMap<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = hash_key(key);
        hamt_get(self.root.as_ref(), 0, hash, key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&self, key: K, value: V) -> Self {
        let existed = self.contains_key(&key);
        let hash = hash_key(&key);
        let root = hamt_insert(&self.root, 0, hash, key, value);
        PMap { root, len: if existed { self.len } else { self.len + 1 } }
    }

    pub fn remove(&self, key: &K) -> Self {
        if !self.contains_key(key) {
            return self.clone();
        }
        let hash = hash_key(key);
        match hamt_remove(&self.root, 0, hash, key) {
            Some(root) => PMap { root, len: self.len - 1 },
            None => PMap { root: Arc::new(HamtNode::Empty), len: 0 },
        }
    }
}

impl<K: Clone + Eq + std::hash::Hash + Ord + CanonicalEncode, V: Clone + CanonicalEncode> PMap<K, V> {
    /// 🔑️ Content hash over `(key, value)` pairs in sorted-by-key order — sorted so the
    /// hash is independent of insertion order/HAMT bucket layout, only of logical content.
    pub fn content_hash(&self) -> pack::ContentHash {
        let mut entries: Vec<(&K, &V)> = self.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        let mut buf = Vec::new();
        for (k, v) in entries {
            k.encode_canonical(&mut buf);
            v.encode_canonical(&mut buf);
        }
        hash_bytes(&buf)
    }
}
//#endregion 🔖️PMap
