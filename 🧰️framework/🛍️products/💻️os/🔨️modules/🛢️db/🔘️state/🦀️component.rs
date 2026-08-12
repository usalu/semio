//! 🗄️ `db_state` — hand-rolled persistent (immutable, structurally-shared) diff-state overlays for
//! the `db` crate family: a 32-way HAMT `PMap`, a bitmapped-trie `PVec`, a rope `PText`, a
//! weight-balanced-by-height `PTree`, and an adjacency-map `PGraph`, plus the content-addressing
//! (`blake3`) primitives and `TouchedRegion` descriptors the rest of the family builds document
//! overlays and conflict detection on top of. No `im`/`im-rc`/`rpds`/`imbl` dependency — every
//! structure below is `Rc`-based path-copying, written against the frozen contract at
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_state` row).
//!
//! 🎯️ Design choice: every structure clones in O(1) (an `Rc` bump) regardless of whether its
//! element types implement `Clone` — `Clone` is implemented by hand rather than derived, since a
//! derived impl would wrongly require `K: Clone, V: Clone` just to bump a reference count. Mutating
//! operations (`insert`/`push_back`/`insert` text/…) are the ones that need `Clone` element bounds,
//! because a path-copy has to clone the *values* sitting along the rewritten path, not the whole
//! structure.
//!
//! 🎯️ Design choice: internal hash-bucket routing (the HAMT's `PMap` and the graph's adjacency
//! maps) uses `std::collections::hash_map::DefaultHasher` (deterministic, unseeded) rather than a
//! cryptographic hash — it only needs to be a stable *router*, not collision-resistant. Content
//! addressing (dedup/identity across snapshots) is the separate, deliberately blake3-based
//! `ContentAddressed`/`PageStore` mechanism in the `📇️Pages` region below.

use crate::db_ids::DbError;
use std::rc::Rc;

//#region 🔖️Pages
/// @emoji 📇️ A type that can serialize itself into a byte buffer in a canonical, deterministic
/// order — the basis every persistent structure's `content_hash` is built on. Implemented for the
/// handful of primitive value types the `db` family's overlays actually store; a higher crate
/// (`db_artifact`) that needs to content-address an application-level value implements this for
/// its own wrapper type rather than `db_state` growing a dependency on a serialization crate.
pub trait CanonicalEncode {
    fn encode_canonical(&self, out: &mut Vec<u8>);
}

impl CanonicalEncode for String {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.len() as u64).to_le_bytes());
        out.extend_from_slice(self.as_bytes());
    }
}

impl CanonicalEncode for Vec<u8> {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.len() as u64).to_le_bytes());
        out.extend_from_slice(self);
    }
}

impl CanonicalEncode for u64 {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}

impl CanonicalEncode for i64 {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}

impl CanonicalEncode for bool {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.push(*self as u8);
    }
}

impl CanonicalEncode for () {
    fn encode_canonical(&self, _out: &mut Vec<u8>) {}
}

/// @emoji 🔑️ Hashes `bytes` with blake3, the family's hashing algorithm throughout (matches
/// `pack`/`protocol`'s `ContentHash`).
fn hash_bytes(bytes: &[u8]) -> pack::ContentHash {
    pack::ContentHash(*blake3::hash(bytes).as_bytes())
}

/// @emoji 📦️ An immutable, content-addressed byte page: its `hash` is the blake3 digest of
/// `bytes`, computed once at construction. `db_snapshot` will use pages of this shape as the
/// unit written into `KIND_CHUNK` segments.
#[derive(Clone)]
pub struct Page {
    pub hash: pack::ContentHash,
    pub bytes: Rc<[u8]>,
}

impl Page {
    pub fn new(bytes: Vec<u8>) -> Page {
        let hash = hash_bytes(&bytes);
        Page { hash, bytes: Rc::from(bytes) }
    }
}

/// @emoji 🗃️ An in-memory content-addressed page cache: interning identical byte content twice
/// returns the same hash and shares the one underlying allocation — the structural-sharing/dedup
/// mechanism `db_snapshot`'s incremental generations build on.
#[derive(Default)]
pub struct PageStore {
    pages: std::collections::HashMap<pack::ContentHash, Rc<[u8]>>,
}

impl PageStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕️ Interns `bytes`, returning its content hash. A byte-identical page already
    /// present is reused (no duplicate allocation, no duplicate `PageStore` entry).
    pub fn intern(&mut self, bytes: Vec<u8>) -> pack::ContentHash {
        let hash = hash_bytes(&bytes);
        self.pages.entry(hash).or_insert_with(|| Rc::from(bytes));
        hash
    }

    pub fn get(&self, hash: &pack::ContentHash) -> Option<Rc<[u8]>> {
        self.pages.get(hash).cloned()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }
}
//#endregion 🔖️Pages

//#region 🔖️PMap
/// @emoji 🔀️ Internal, deterministic (unseeded) hash used only to route keys to HAMT buckets —
/// see the module doc's design-choice note on why this is not blake3.
fn hash_key<K: std::hash::Hash>(key: &K) -> u64 {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

/// @emoji 🪜️ 5 bits routed per HAMT level (32-way branching, per the contract).
const HAMT_BITS: u32 = 5;
/// @emoji 🛑️ Beyond this depth, the 64-bit routing hash is exhausted (13 × 5 = 65 > 64); further
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
    /// @emoji 💥️ A bucket of entries that share a routing hash (or that ran out of depth to
    /// split further) — degrades to a linear scan, which is fine since real-world key
    /// distributions make this vanishingly rare.
    Collision {
        entries: Vec<(u64, K, V)>,
    },
    Branch {
        bitmap: u32,
        children: Vec<Rc<HamtNode<K, V>>>,
    },
}

fn hamt_branch_from_two<K: Clone + Eq, V: Clone>(depth: u8, h1: u64, k1: K, v1: V, h2: u64, k2: K, v2: V) -> Rc<HamtNode<K, V>> {
    if depth >= HAMT_MAX_DEPTH {
        return Rc::new(HamtNode::Collision { entries: vec![(h1, k1, v1), (h2, k2, v2)] });
    }
    let i1 = hamt_index(h1, depth);
    let i2 = hamt_index(h2, depth);
    if i1 == i2 {
        let child = hamt_branch_from_two(depth + 1, h1, k1, v1, h2, k2, v2);
        Rc::new(HamtNode::Branch { bitmap: 1u32 << i1, children: vec![child] })
    } else {
        let leaf1 = Rc::new(HamtNode::Leaf { hash: h1, key: k1, value: v1 });
        let leaf2 = Rc::new(HamtNode::Leaf { hash: h2, key: k2, value: v2 });
        let bitmap = (1u32 << i1) | (1u32 << i2);
        let children = if i1 < i2 { vec![leaf1, leaf2] } else { vec![leaf2, leaf1] };
        Rc::new(HamtNode::Branch { bitmap, children })
    }
}

fn hamt_insert<K: Clone + Eq, V: Clone>(node: &Rc<HamtNode<K, V>>, depth: u8, hash: u64, key: K, value: V) -> Rc<HamtNode<K, V>> {
    match node.as_ref() {
        HamtNode::Empty => Rc::new(HamtNode::Leaf { hash, key, value }),
        HamtNode::Leaf { hash: h, key: k, value: v } => {
            if key == *k {
                Rc::new(HamtNode::Leaf { hash, key, value })
            } else if *h == hash || depth >= HAMT_MAX_DEPTH {
                Rc::new(HamtNode::Collision { entries: vec![(*h, k.clone(), v.clone()), (hash, key, value)] })
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
            Rc::new(HamtNode::Collision { entries: new_entries })
        }
        HamtNode::Branch { bitmap, children } => {
            let i = hamt_index(hash, depth);
            let bit = 1u32 << i;
            let pos = (bitmap & (bit - 1)).count_ones() as usize;
            if bitmap & bit != 0 {
                let updated = hamt_insert(&children[pos], depth + 1, hash, key, value);
                let mut new_children = children.clone();
                new_children[pos] = updated;
                Rc::new(HamtNode::Branch { bitmap: *bitmap, children: new_children })
            } else {
                let mut new_children = children.clone();
                new_children.insert(pos, Rc::new(HamtNode::Leaf { hash, key, value }));
                Rc::new(HamtNode::Branch { bitmap: bitmap | bit, children: new_children })
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

fn hamt_remove<K: Clone + Eq, V: Clone>(node: &Rc<HamtNode<K, V>>, depth: u8, hash: u64, key: &K) -> Option<Rc<HamtNode<K, V>>> {
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
                    Some(Rc::new(HamtNode::Leaf { hash: h, key: k, value: v }))
                } else if new_entries.is_empty() {
                    None
                } else {
                    Some(Rc::new(HamtNode::Collision { entries: new_entries }))
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
                        Some(Rc::new(HamtNode::Branch { bitmap: new_bitmap, children: new_children }))
                    }
                }
                Some(new_child) => {
                    if Rc::ptr_eq(&new_child, &children[pos]) {
                        Some(node.clone())
                    } else {
                        let mut new_children = children.clone();
                        new_children[pos] = new_child;
                        Some(Rc::new(HamtNode::Branch { bitmap: *bitmap, children: new_children }))
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

/// @emoji 🗺️ A persistent (immutable, structurally-shared) hash map: a 32-way HAMT. Every
/// mutating method returns a new `PMap`; unaffected subtrees are shared via `Rc` with the map(s)
/// it was derived from, so a chain of `n` single-key edits allocates `O(n log n)` nodes total,
/// not `O(n²)`.
pub struct PMap<K, V> {
    root: Rc<HamtNode<K, V>>,
    len: usize,
}

impl<K, V> PMap<K, V> {
    pub fn new() -> Self {
        PMap { root: Rc::new(HamtNode::Empty), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// @emoji 🚶️ Eagerly materializes every entry — simple to reason about at this crate's scope;
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
            None => PMap { root: Rc::new(HamtNode::Empty), len: 0 },
        }
    }
}

impl<K: Clone + Eq + std::hash::Hash + Ord + CanonicalEncode, V: Clone + CanonicalEncode> PMap<K, V> {
    /// @emoji 🔑️ Content hash over `(key, value)` pairs in sorted-by-key order — sorted so the
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

//#region 🔖️PVec
/// @emoji 🪜️ 5 bits routed per `PVec` trie level (32-way branching, matching `PMap`).
const PVEC_BITS: u32 = 5;

enum VecNode<T> {
    Leaf(Vec<T>),
    Branch(Vec<Rc<VecNode<T>>>),
}

fn pvec_get<T>(node: &VecNode<T>, shift: u32, index: usize) -> &T {
    match node {
        VecNode::Leaf(items) => &items[index & 0x1F],
        VecNode::Branch(children) => {
            let child_index = (index >> shift) & 0x1F;
            pvec_get(children[child_index].as_ref(), shift - PVEC_BITS, index)
        }
    }
}

/// @emoji 🌱️ Builds a fresh single-child chain down to a one-element leaf holding `value`. Only
/// ever called at the exact next append position of a brand-new subtree, where every lower-order
/// bit of `index` is guaranteed zero relative to that subtree — see the module's push_back note.
fn pvec_new_path<T: Clone>(shift: u32, value: T) -> Rc<VecNode<T>> {
    if shift == 0 {
        Rc::new(VecNode::Leaf(vec![value]))
    } else {
        Rc::new(VecNode::Branch(vec![pvec_new_path(shift - PVEC_BITS, value)]))
    }
}

fn pvec_set<T: Clone>(node: &Rc<VecNode<T>>, shift: u32, index: usize, value: T) -> Rc<VecNode<T>> {
    match node.as_ref() {
        VecNode::Leaf(items) => {
            let mut new_items = items.clone();
            let leaf_index = index & 0x1F;
            if leaf_index < new_items.len() {
                new_items[leaf_index] = value;
            } else {
                new_items.push(value);
            }
            Rc::new(VecNode::Leaf(new_items))
        }
        VecNode::Branch(children) => {
            let child_index = (index >> shift) & 0x1F;
            let mut new_children = children.clone();
            if child_index < new_children.len() {
                let updated = pvec_set(&new_children[child_index], shift - PVEC_BITS, index, value);
                new_children[child_index] = updated;
            } else {
                new_children.push(pvec_new_path(shift - PVEC_BITS, value));
            }
            Rc::new(VecNode::Branch(new_children))
        }
    }
}

/// @emoji 🧵️ A persistent (immutable, structurally-shared) vector: a bitmapped trie, 32-way
/// branching. `push_back`/`set`/`pop_back` are `O(log₃₂ n)`; unaffected subtrees are shared.
pub struct PVec<T> {
    root: Rc<VecNode<T>>,
    shift: u32,
    len: usize,
}

impl<T> PVec<T> {
    pub fn new() -> Self {
        PVec { root: Rc::new(VecNode::Leaf(Vec::new())), shift: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(pvec_get(self.root.as_ref(), self.shift, index))
        }
    }
}

impl<T> Default for PVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for PVec<T> {
    fn clone(&self) -> Self {
        PVec { root: self.root.clone(), shift: self.shift, len: self.len }
    }
}

impl<T: Clone> PVec<T> {
    pub fn push_back(&self, value: T) -> Self {
        let capacity = 1usize << (self.shift + PVEC_BITS);
        if self.len < capacity {
            let root = pvec_set(&self.root, self.shift, self.len, value);
            PVec { root, shift: self.shift, len: self.len + 1 }
        } else {
            let grown_root = Rc::new(VecNode::Branch(vec![self.root.clone()]));
            let new_shift = self.shift + PVEC_BITS;
            let root = pvec_set(&grown_root, new_shift, self.len, value);
            PVec { root, shift: new_shift, len: self.len + 1 }
        }
    }

    pub fn set(&self, index: usize, value: T) -> Result<Self, DbError> {
        if index >= self.len {
            return Err(DbError::InvalidArgument(format!("PVec::set index {index} out of bounds (len {})", self.len)));
        }
        let root = pvec_set(&self.root, self.shift, index, value);
        Ok(PVec { root, shift: self.shift, len: self.len })
    }

    pub fn pop_back(&self) -> Result<Self, DbError> {
        if self.len == 0 {
            return Err(DbError::InvalidArgument("PVec::pop_back on an empty vector".to_string()));
        }
        Ok(PVec { root: self.root.clone(), shift: self.shift, len: self.len - 1 })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        (0..self.len).map(move |i| self.get(i).expect("index within len"))
    }
}

impl<T: Clone + CanonicalEncode> PVec<T> {
    /// @emoji 🔑️ Content hash over elements in index order (order-sensitive, unlike `PMap`'s).
    pub fn content_hash(&self) -> pack::ContentHash {
        let mut buf = Vec::new();
        for item in self.iter() {
            item.encode_canonical(&mut buf);
        }
        hash_bytes(&buf)
    }
}
//#endregion 🔖️PVec

//#region 🔖️PText
enum RopeNode {
    Leaf { text: Rc<str>, chars: usize },
    Concat { left: Rc<RopeNode>, right: Rc<RopeNode>, chars: usize },
}

fn rope_chars(node: &RopeNode) -> usize {
    match node {
        RopeNode::Leaf { chars, .. } => *chars,
        RopeNode::Concat { chars, .. } => *chars,
    }
}

fn rope_collect(node: &RopeNode, out: &mut String) {
    match node {
        RopeNode::Leaf { text, .. } => out.push_str(text),
        RopeNode::Concat { left, right, .. } => {
            rope_collect(left, out);
            rope_collect(right, out);
        }
    }
}

/// @emoji ➕️ Concatenates two rope nodes, dropping an empty side rather than wrapping it — keeps
/// repeated split/rejoin (as `insert`/`delete` do) from growing unbounded chains of empty leaves.
fn rope_concat_nodes(left: Rc<RopeNode>, right: Rc<RopeNode>) -> Rc<RopeNode> {
    let left_chars = rope_chars(&left);
    let right_chars = rope_chars(&right);
    if left_chars == 0 {
        return right;
    }
    if right_chars == 0 {
        return left;
    }
    Rc::new(RopeNode::Concat { left, right, chars: left_chars + right_chars })
}

/// @emoji ✂️ Splits at char offset `at` (`0 <= at <= rope_chars(node)`, checked by every public
/// caller before this is reached).
fn rope_split(node: &Rc<RopeNode>, at: usize) -> (Rc<RopeNode>, Rc<RopeNode>) {
    match node.as_ref() {
        RopeNode::Leaf { text, chars } => {
            if at == 0 {
                (Rc::new(RopeNode::Leaf { text: Rc::from(""), chars: 0 }), node.clone())
            } else if at >= *chars {
                (node.clone(), Rc::new(RopeNode::Leaf { text: Rc::from(""), chars: 0 }))
            } else {
                let byte_at = text.char_indices().nth(at).map(|(b, _)| b).expect("at < chars checked above");
                let (l, r) = text.split_at(byte_at);
                (Rc::new(RopeNode::Leaf { text: Rc::from(l), chars: at }), Rc::new(RopeNode::Leaf { text: Rc::from(r), chars: *chars - at }))
            }
        }
        RopeNode::Concat { left, right, .. } => {
            let left_chars = rope_chars(left);
            if at <= left_chars {
                let (ll, lr) = rope_split(left, at);
                (ll, rope_concat_nodes(lr, right.clone()))
            } else {
                let (rl, rr) = rope_split(right, at - left_chars);
                (rope_concat_nodes(left.clone(), rl), rr)
            }
        }
    }
}

/// @emoji 📜️ A persistent (immutable, structurally-shared) text rope. `insert`/`delete`/`slice`
/// are expressed as `split` + `concat`, sharing every untouched leaf with the rope(s) they were
/// derived from.
///
/// 🧩️ Extension seam: no active rebalancing — a long run of single-character edits at the same
/// offset grows an unbalanced concat chain (still correct, `O(n)` worst case per edit instead of
/// `O(log n)`). A rebalance-on-concat pass (e.g. depth/weight-triggered, as real rope
/// implementations do) is a self-contained follow-up that does not change this type's public API.
#[derive(Clone)]
pub struct PText(Rc<RopeNode>);

impl PText {
    pub fn new() -> Self {
        PText(Rc::new(RopeNode::Leaf { text: Rc::from(""), chars: 0 }))
    }

    pub fn from_text(s: &str) -> Self {
        PText(Rc::new(RopeNode::Leaf { text: Rc::from(s), chars: s.chars().count() }))
    }

    pub fn len(&self) -> usize {
        rope_chars(&self.0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn concat(&self, other: &PText) -> PText {
        PText(rope_concat_nodes(self.0.clone(), other.0.clone()))
    }

    pub fn slice(&self, start: usize, end: usize) -> Result<PText, DbError> {
        if start > end || end > self.len() {
            return Err(DbError::InvalidArgument(format!("PText::slice range {start}..{end} out of bounds (len {})", self.len())));
        }
        let (_, suffix) = rope_split(&self.0, start);
        let (middle, _) = rope_split(&suffix, end - start);
        Ok(PText(middle))
    }

    pub fn insert(&self, at: usize, s: &str) -> Result<PText, DbError> {
        if at > self.len() {
            return Err(DbError::InvalidArgument(format!("PText::insert at {at} out of bounds (len {})", self.len())));
        }
        let (left, right) = rope_split(&self.0, at);
        Ok(PText(left).concat(&PText::from_text(s)).concat(&PText(right)))
    }

    pub fn delete(&self, start: usize, end: usize) -> Result<PText, DbError> {
        if start > end || end > self.len() {
            return Err(DbError::InvalidArgument(format!("PText::delete range {start}..{end} out of bounds (len {})", self.len())));
        }
        let (left, _) = rope_split(&self.0, start);
        let (_, right) = rope_split(&self.0, end);
        Ok(PText(left).concat(&PText(right)))
    }

    /// @emoji 🔑️ Content hash of the rope's flattened UTF-8 bytes.
    pub fn content_hash(&self) -> pack::ContentHash {
        hash_bytes(self.to_string().as_bytes())
    }
}

impl Default for PText {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(self.len());
        rope_collect(&self.0, &mut out);
        write!(f, "{out}")
    }
}
//#endregion 🔖️PText

//#region 🔖️PTree
struct TreeNode<K, V> {
    key: K,
    value: V,
    left: Option<Rc<TreeNode<K, V>>>,
    right: Option<Rc<TreeNode<K, V>>>,
    height: u32,
}

fn tree_height<K, V>(node: Option<&Rc<TreeNode<K, V>>>) -> u32 {
    node.map_or(0, |n| n.height)
}

fn tree_make<K: Clone, V: Clone>(key: K, value: V, left: Option<Rc<TreeNode<K, V>>>, right: Option<Rc<TreeNode<K, V>>>) -> Rc<TreeNode<K, V>> {
    let height = 1 + tree_height(left.as_ref()).max(tree_height(right.as_ref()));
    Rc::new(TreeNode { key, value, left, right, height })
}

fn tree_balance_factor<K, V>(node: &TreeNode<K, V>) -> i64 {
    tree_height(node.left.as_ref()) as i64 - tree_height(node.right.as_ref()) as i64
}

fn tree_rotate_right<K: Clone, V: Clone>(node: &TreeNode<K, V>) -> Rc<TreeNode<K, V>> {
    let left = node.left.as_ref().expect("rotate_right requires a left child").clone();
    let new_right = tree_make(node.key.clone(), node.value.clone(), left.right.clone(), node.right.clone());
    tree_make(left.key.clone(), left.value.clone(), left.left.clone(), Some(new_right))
}

fn tree_rotate_left<K: Clone, V: Clone>(node: &TreeNode<K, V>) -> Rc<TreeNode<K, V>> {
    let right = node.right.as_ref().expect("rotate_left requires a right child").clone();
    let new_left = tree_make(node.key.clone(), node.value.clone(), node.left.clone(), right.left.clone());
    tree_make(right.key.clone(), right.value.clone(), Some(new_left), right.right.clone())
}

/// @emoji ⚖️ Restores the AVL invariant (`|balance_factor| <= 1`) at `node`'s root via at most one
/// single or double rotation — the standard AVL rebalance, applied bottom-up after every
/// insert/remove so `PTree`'s height stays `O(log n)`.
fn tree_balance<K: Clone, V: Clone>(node: Rc<TreeNode<K, V>>) -> Rc<TreeNode<K, V>> {
    let bf = tree_balance_factor(&node);
    if bf > 1 {
        let left = node.left.as_ref().expect("bf > 1 implies a left child");
        if tree_balance_factor(left) < 0 {
            let new_left = tree_rotate_left(left);
            let rebuilt = tree_make(node.key.clone(), node.value.clone(), Some(new_left), node.right.clone());
            tree_rotate_right(&rebuilt)
        } else {
            tree_rotate_right(&node)
        }
    } else if bf < -1 {
        let right = node.right.as_ref().expect("bf < -1 implies a right child");
        if tree_balance_factor(right) > 0 {
            let new_right = tree_rotate_right(right);
            let rebuilt = tree_make(node.key.clone(), node.value.clone(), node.left.clone(), Some(new_right));
            tree_rotate_left(&rebuilt)
        } else {
            tree_rotate_left(&node)
        }
    } else {
        node
    }
}

fn tree_insert<K: Clone + Ord, V: Clone>(node: Option<&Rc<TreeNode<K, V>>>, key: K, value: V) -> Rc<TreeNode<K, V>> {
    match node {
        None => tree_make(key, value, None, None),
        Some(n) => match key.cmp(&n.key) {
            std::cmp::Ordering::Less => {
                let new_left = Some(tree_insert(n.left.as_ref(), key, value));
                tree_balance(tree_make(n.key.clone(), n.value.clone(), new_left, n.right.clone()))
            }
            std::cmp::Ordering::Greater => {
                let new_right = Some(tree_insert(n.right.as_ref(), key, value));
                tree_balance(tree_make(n.key.clone(), n.value.clone(), n.left.clone(), new_right))
            }
            std::cmp::Ordering::Equal => tree_make(key, value, n.left.clone(), n.right.clone()),
        },
    }
}

fn tree_get<'a, K: Ord, V>(node: Option<&'a Rc<TreeNode<K, V>>>, key: &K) -> Option<&'a V> {
    match node {
        None => None,
        Some(n) => match key.cmp(&n.key) {
            std::cmp::Ordering::Less => tree_get(n.left.as_ref(), key),
            std::cmp::Ordering::Greater => tree_get(n.right.as_ref(), key),
            std::cmp::Ordering::Equal => Some(&n.value),
        },
    }
}

fn tree_remove_min<K: Clone, V: Clone>(node: &Rc<TreeNode<K, V>>) -> (K, V, Option<Rc<TreeNode<K, V>>>) {
    match &node.left {
        None => (node.key.clone(), node.value.clone(), node.right.clone()),
        Some(l) => {
            let (k, v, new_left) = tree_remove_min(l);
            let rebuilt = tree_balance(tree_make(node.key.clone(), node.value.clone(), new_left, node.right.clone()));
            (k, v, Some(rebuilt))
        }
    }
}

fn tree_remove<K: Clone + Ord, V: Clone>(node: Option<&Rc<TreeNode<K, V>>>, key: &K) -> Option<Rc<TreeNode<K, V>>> {
    match node {
        None => None,
        Some(n) => match key.cmp(&n.key) {
            std::cmp::Ordering::Less => {
                let new_left = tree_remove(n.left.as_ref(), key);
                Some(tree_balance(tree_make(n.key.clone(), n.value.clone(), new_left, n.right.clone())))
            }
            std::cmp::Ordering::Greater => {
                let new_right = tree_remove(n.right.as_ref(), key);
                Some(tree_balance(tree_make(n.key.clone(), n.value.clone(), n.left.clone(), new_right)))
            }
            std::cmp::Ordering::Equal => match (&n.left, &n.right) {
                (None, None) => None,
                (Some(l), None) => Some(l.clone()),
                (None, Some(r)) => Some(r.clone()),
                (Some(_), Some(r)) => {
                    let (mk, mv, new_right) = tree_remove_min(r);
                    Some(tree_balance(tree_make(mk, mv, n.left.clone(), new_right)))
                }
            },
        },
    }
}

fn tree_collect<'a, K, V>(node: Option<&'a Rc<TreeNode<K, V>>>, out: &mut Vec<(&'a K, &'a V)>) {
    if let Some(n) = node {
        tree_collect(n.left.as_ref(), out);
        out.push((&n.key, &n.value));
        tree_collect(n.right.as_ref(), out);
    }
}

/// @emoji 🌳️ A persistent (immutable, structurally-shared) ordered map: an AVL tree. Unlike
/// `PMap`, iteration order is the key order (`K: Ord`), and height is kept `O(log n)` by
/// rebalancing on every insert/remove — the property `db_index`'s sorted-run merges will lean on.
pub struct PTree<K, V> {
    root: Option<Rc<TreeNode<K, V>>>,
    len: usize,
}

impl<K, V> PTree<K, V> {
    pub fn new() -> Self {
        PTree { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// @emoji 📏️ The tree's current height — exposed so tests (and callers with their own
    /// balance-sensitive assumptions) can assert the `O(log n)` bound directly.
    pub fn height(&self) -> u32 {
        tree_height(self.root.as_ref())
    }
}

impl<K, V> Default for PTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for PTree<K, V> {
    fn clone(&self) -> Self {
        PTree { root: self.root.clone(), len: self.len }
    }
}

impl<K: Clone + Ord, V: Clone> PTree<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        tree_get(self.root.as_ref(), key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&self, key: K, value: V) -> Self {
        let existed = self.contains_key(&key);
        let root = Some(tree_insert(self.root.as_ref(), key, value));
        PTree { root, len: if existed { self.len } else { self.len + 1 } }
    }

    pub fn remove(&self, key: &K) -> Self {
        if !self.contains_key(key) {
            return self.clone();
        }
        let root = tree_remove(self.root.as_ref(), key);
        PTree { root, len: self.len - 1 }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        let mut out = Vec::with_capacity(self.len);
        tree_collect(self.root.as_ref(), &mut out);
        out.into_iter()
    }
}

impl<K: Clone + Ord + CanonicalEncode, V: Clone + CanonicalEncode> PTree<K, V> {
    /// @emoji 🔑️ Content hash over `(key, value)` pairs in ascending key order (already the
    /// tree's natural iteration order).
    pub fn content_hash(&self) -> pack::ContentHash {
        let mut buf = Vec::new();
        for (k, v) in self.iter() {
            k.encode_canonical(&mut buf);
            v.encode_canonical(&mut buf);
        }
        hash_bytes(&buf)
    }
}
//#endregion 🔖️PTree

//#region 🔖️PGraph
/// @emoji 🕸️ A persistent (immutable, structurally-shared) directed graph: node data plus
/// adjacency, both backed by `PMap` so every mutation shares everything it didn't touch. Keeps
/// both `out_edges` and `in_edges` so neighbor and predecessor lookups are both `O(log n)`
/// (trading extra edge-side storage for that symmetry).
pub struct PGraph<N, ND, ED> {
    nodes: PMap<N, ND>,
    out_edges: PMap<N, PMap<N, ED>>,
    in_edges: PMap<N, PMap<N, ()>>,
}

impl<N, ND, ED> Default for PGraph<N, ND, ED> {
    fn default() -> Self {
        PGraph { nodes: PMap::new(), out_edges: PMap::new(), in_edges: PMap::new() }
    }
}

impl<N, ND, ED> Clone for PGraph<N, ND, ED> {
    fn clone(&self) -> Self {
        PGraph { nodes: self.nodes.clone(), out_edges: self.out_edges.clone(), in_edges: self.in_edges.clone() }
    }
}

impl<N: Clone + Eq + std::hash::Hash, ND: Clone, ED: Clone> PGraph<N, ND, ED> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn contains_node(&self, id: &N) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn node(&self, id: &N) -> Option<&ND> {
        self.nodes.get(id)
    }

    pub fn add_node(&self, id: N, data: ND) -> Self {
        PGraph { nodes: self.nodes.insert(id, data), out_edges: self.out_edges.clone(), in_edges: self.in_edges.clone() }
    }

    /// @emoji 🧹️ Removes `id` and every edge touching it (both directions) — the persistent
    /// "cascade delete", threading each intermediate persistent map through sequential
    /// (but still `O(log n)`-per-step) `insert`/`remove` calls.
    pub fn remove_node(&self, id: &N) -> Self {
        let nodes = self.nodes.remove(id);
        let successors: Vec<N> = self.out_edges.get(id).map(|m| m.iter().map(|(k, _)| k.clone()).collect()).unwrap_or_default();
        let predecessors: Vec<N> = self.in_edges.get(id).map(|m| m.iter().map(|(k, _)| k.clone()).collect()).unwrap_or_default();
        let mut out_edges = self.out_edges.remove(id);
        let mut in_edges = self.in_edges.remove(id);
        for successor in &successors {
            if let Some(preds_of_successor) = in_edges.get(successor) {
                let updated = preds_of_successor.remove(id);
                in_edges = if updated.is_empty() { in_edges.remove(successor) } else { in_edges.insert(successor.clone(), updated) };
            }
        }
        for predecessor in &predecessors {
            if let Some(succs_of_predecessor) = out_edges.get(predecessor) {
                let updated = succs_of_predecessor.remove(id);
                out_edges = if updated.is_empty() { out_edges.remove(predecessor) } else { out_edges.insert(predecessor.clone(), updated) };
            }
        }
        PGraph { nodes, out_edges, in_edges }
    }

    pub fn add_edge(&self, from: N, to: N, data: ED) -> Result<Self, DbError> {
        if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
            return Err(DbError::NotFound("PGraph::add_edge references a node not present in the graph".to_string()));
        }
        let from_out = self.out_edges.get(&from).cloned().unwrap_or_default().insert(to.clone(), data);
        let out_edges = self.out_edges.insert(from.clone(), from_out);
        let to_in = self.in_edges.get(&to).cloned().unwrap_or_default().insert(from, ());
        let in_edges = self.in_edges.insert(to, to_in);
        Ok(PGraph { nodes: self.nodes.clone(), out_edges, in_edges })
    }

    pub fn remove_edge(&self, from: &N, to: &N) -> Self {
        let out_edges = match self.out_edges.get(from) {
            Some(out) => {
                let updated = out.remove(to);
                if updated.is_empty() {
                    self.out_edges.remove(from)
                } else {
                    self.out_edges.insert(from.clone(), updated)
                }
            }
            None => self.out_edges.clone(),
        };
        let in_edges = match self.in_edges.get(to) {
            Some(ins) => {
                let updated = ins.remove(from);
                if updated.is_empty() {
                    self.in_edges.remove(to)
                } else {
                    self.in_edges.insert(to.clone(), updated)
                }
            }
            None => self.in_edges.clone(),
        };
        PGraph { nodes: self.nodes.clone(), out_edges, in_edges }
    }

    pub fn has_edge(&self, from: &N, to: &N) -> bool {
        self.edge_data(from, to).is_some()
    }

    pub fn edge_data(&self, from: &N, to: &N) -> Option<&ED> {
        self.out_edges.get(from).and_then(|m| m.get(to))
    }

    pub fn neighbors(&self, id: &N) -> Vec<&N> {
        self.out_edges.get(id).map(|m| m.iter().map(|(k, _)| k).collect()).unwrap_or_default()
    }

    pub fn predecessors(&self, id: &N) -> Vec<&N> {
        self.in_edges.get(id).map(|m| m.iter().map(|(k, _)| k).collect()).unwrap_or_default()
    }
}

impl<N: Clone + Eq + std::hash::Hash + Ord + CanonicalEncode, ND: Clone + CanonicalEncode, ED: Clone + CanonicalEncode> PGraph<N, ND, ED> {
    /// @emoji 🔑️ Content hash over the node set (sorted, via `PMap::content_hash`) followed by
    /// the edge set sorted by `(from, to)`.
    pub fn content_hash(&self) -> pack::ContentHash {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.nodes.content_hash().0);
        let mut edges: Vec<(N, N, ED)> = Vec::new();
        for (from, out) in self.out_edges.iter() {
            for (to, data) in out.iter() {
                edges.push((from.clone(), to.clone(), data.clone()));
            }
        }
        edges.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
        for (from, to, data) in &edges {
            from.encode_canonical(&mut buf);
            to.encode_canonical(&mut buf);
            data.encode_canonical(&mut buf);
        }
        hash_bytes(&buf)
    }
}
//#endregion 🔖️PGraph

//#region 🔖️Overlay
/// @emoji 🫧️ The read side of a document's immutable base — typically a lazily-decoded `pack`
/// document. `db_artifact` implements this over the real pack reader; `db_state` only depends on
/// the trait, keeping this crate `pack`-decoder-free (it depends on `pack_core`, not `pack`).
pub trait BaseSource {
    fn load(&self, path: &str) -> Result<Option<Vec<u8>>, DbError>;
}

/// @emoji 🫙️ A `BaseSource` with nothing in it — the base for a brand-new document, and useful in
/// tests that only care about overlay behavior.
pub struct EmptyBase;

impl BaseSource for EmptyBase {
    fn load(&self, _path: &str) -> Result<Option<Vec<u8>>, DbError> {
        Ok(None)
    }
}

/// @emoji ✏️ What an overlay records at a path: either an explicit value that shadows the base,
/// or an explicit tombstone that hides a base value without touching the base itself.
#[derive(Clone)]
enum OverlayValue {
    Set(Vec<u8>),
    Deleted,
}

/// @emoji 🏗️ A document's live, mutable-by-replacement state: an immutable base (read lazily,
/// never written) plus a `PMap` overlay of edits on top. Reads fall through overlay → base, per
/// the contract; every mutation returns a new `OverlayRoot` (the base `Rc` is shared, only the
/// overlay `PMap` grows/shrinks) alongside the `TouchedRegion` it touched, for `db_conflict`.
pub struct OverlayRoot<B: BaseSource> {
    base: Rc<B>,
    overlay: PMap<String, OverlayValue>,
}

impl<B: BaseSource> OverlayRoot<B> {
    pub fn new(base: B) -> Self {
        OverlayRoot { base: Rc::new(base), overlay: PMap::new() }
    }

    pub fn get(&self, path: &str) -> Result<Option<Vec<u8>>, DbError> {
        match self.overlay.get(&path.to_string()) {
            Some(OverlayValue::Set(bytes)) => Ok(Some(bytes.clone())),
            Some(OverlayValue::Deleted) => Ok(None),
            None => self.base.load(path),
        }
    }

    pub fn set(&self, path: &str, value: Vec<u8>) -> (Self, TouchedRegion) {
        let overlay = self.overlay.insert(path.to_string(), OverlayValue::Set(value));
        (OverlayRoot { base: self.base.clone(), overlay }, TouchedRegion::write(path))
    }

    pub fn delete(&self, path: &str) -> (Self, TouchedRegion) {
        let overlay = self.overlay.insert(path.to_string(), OverlayValue::Deleted);
        (OverlayRoot { base: self.base.clone(), overlay }, TouchedRegion::write(path))
    }

    /// @emoji 🔢️ How many paths the overlay has explicitly recorded (set or tombstoned) — every
    /// other path still falls through to `base` untouched.
    pub fn overlay_len(&self) -> usize {
        self.overlay.len()
    }
}

impl<B: BaseSource> Clone for OverlayRoot<B> {
    fn clone(&self) -> Self {
        OverlayRoot { base: self.base.clone(), overlay: self.overlay.clone() }
    }
}
//#endregion 🔖️Overlay

//#region 🔖️TouchedRegion
/// @emoji 👣️ Whether a `TouchedRegion` records a read or a write — two reads of the same region
/// never conflict; a write against anything intersecting it does.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TouchKind {
    Read,
    Write,
}

/// @emoji 🗺️ A single access against one path in an overlay — `db_conflict`'s primitive unit for
/// touched-region intersection (bloom filters and the command-kind matrix build on top of this,
/// they are `db_conflict`'s own concern). `path` is a `/`-separated segment string (matching
/// `OverlayRoot`'s path shape), so a coarse write to a container path is detected as conflicting
/// with a narrower access nested beneath it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TouchedRegion {
    pub path: String,
    pub kind: TouchKind,
}

impl TouchedRegion {
    pub fn read(path: impl Into<String>) -> Self {
        TouchedRegion { path: path.into(), kind: TouchKind::Read }
    }

    pub fn write(path: impl Into<String>) -> Self {
        TouchedRegion { path: path.into(), kind: TouchKind::Write }
    }

    /// @emoji 🔀️ True iff `self` and `other` name the same path, or one path is a `/`-boundary
    /// prefix of the other (so a whole-subtree write is treated as touching everything beneath
    /// it, and vice versa).
    pub fn path_intersects(&self, other: &TouchedRegion) -> bool {
        path_is_prefix(&self.path, &other.path) || path_is_prefix(&other.path, &self.path)
    }
}

fn path_is_prefix(prefix: &str, path: &str) -> bool {
    if prefix == path {
        return true;
    }
    path.len() > prefix.len() && path.starts_with(prefix) && path.as_bytes()[prefix.len()] == b'/'
}

/// @emoji 🧾️ The accumulated reads/writes of one command/transaction against an `OverlayRoot` —
/// what `db_conflict` intersects two of (from concurrent commands against the same base frontier)
/// to decide whether they conflict.
#[derive(Clone, Default, Debug)]
pub struct TouchedSet {
    pub regions: Vec<TouchedRegion>,
}

impl TouchedSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, region: TouchedRegion) {
        self.regions.push(region);
    }

    /// @emoji ⚔️ True iff any region in `self` and any region in `other` intersect with at least
    /// one side being a `Write` — read/read intersections never conflict.
    pub fn conflicts_with(&self, other: &TouchedSet) -> bool {
        self.regions.iter().any(|a| other.regions.iter().any(|b| (a.kind == TouchKind::Write || b.kind == TouchKind::Write) && a.path_intersects(b)))
    }
}
//#endregion 🔖️TouchedRegion

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️PMap
    #[test]
    fn pmap_insert_get_roundtrip_across_many_keys() {
        let mut map: PMap<String, i64> = PMap::new();
        for i in 0..2000i64 {
            map = map.insert(format!("key-{i}"), i);
        }
        assert_eq!(map.len(), 2000);
        for i in 0..2000i64 {
            assert_eq!(map.get(&format!("key-{i}")), Some(&i));
        }
        assert_eq!(map.get(&"missing".to_string()), None);
    }

    #[test]
    fn pmap_insert_is_persistent_old_version_unaffected() {
        let empty: PMap<String, i64> = PMap::new();
        let one = empty.insert("a".to_string(), 1);
        let two = one.insert("b".to_string(), 2);
        assert_eq!(empty.len(), 0);
        assert_eq!(one.len(), 1);
        assert_eq!(two.len(), 2);
        assert_eq!(one.get(&"b".to_string()), None);
        assert_eq!(two.get(&"a".to_string()), Some(&1));
        assert_eq!(two.get(&"b".to_string()), Some(&2));
    }

    #[test]
    fn pmap_remove_then_reinsert_and_replace_semantics() {
        let map: PMap<String, i64> = PMap::new().insert("a".to_string(), 1).insert("b".to_string(), 2);
        let removed = map.remove(&"a".to_string());
        assert_eq!(removed.len(), 1);
        assert_eq!(removed.get(&"a".to_string()), None);
        assert_eq!(map.get(&"a".to_string()), Some(&1), "original map must be unaffected by remove on the derived one");

        let replaced = map.insert("a".to_string(), 99);
        assert_eq!(replaced.len(), 2, "replacing an existing key must not grow len");
        assert_eq!(replaced.get(&"a".to_string()), Some(&99));

        let no_op = map.remove(&"not-present".to_string());
        assert_eq!(no_op.len(), map.len());
    }

    #[test]
    fn pmap_content_hash_is_order_independent_and_content_sensitive() {
        let forward: PMap<String, u64> = PMap::new().insert("a".to_string(), 1).insert("b".to_string(), 2).insert("c".to_string(), 3);
        let backward: PMap<String, u64> = PMap::new().insert("c".to_string(), 3).insert("b".to_string(), 2).insert("a".to_string(), 1);
        assert_eq!(forward.content_hash(), backward.content_hash());

        let different = forward.insert("a".to_string(), 999);
        assert_ne!(forward.content_hash(), different.content_hash());
    }

    #[test]
    fn pmap_iter_visits_every_entry_exactly_once() {
        let map: PMap<String, i64> = (0..500).fold(PMap::new(), |m, i| m.insert(format!("k{i}"), i));
        let mut seen: Vec<i64> = map.iter().map(|(_, v)| *v).collect();
        seen.sort();
        assert_eq!(seen, (0..500).collect::<Vec<_>>());
    }
    //#endregion 🔖️PMap

    //#region 🔖️PVec
    #[test]
    fn pvec_push_back_and_get_across_multiple_levels() {
        let mut vec: PVec<i64> = PVec::new();
        for i in 0..5000i64 {
            vec = vec.push_back(i);
        }
        assert_eq!(vec.len(), 5000);
        for i in 0..5000i64 {
            assert_eq!(vec.get(i as usize), Some(&i));
        }
        assert_eq!(vec.get(5000), None);
    }

    #[test]
    fn pvec_push_back_is_persistent() {
        let base: PVec<i64> = PVec::new().push_back(1).push_back(2);
        let extended = base.push_back(3);
        assert_eq!(base.len(), 2);
        assert_eq!(base.get(2), None);
        assert_eq!(extended.len(), 3);
        assert_eq!(extended.get(2), Some(&3));
    }

    #[test]
    fn pvec_set_and_pop_back() {
        let vec: PVec<i64> = (0..40).fold(PVec::new(), |v, i| v.push_back(i));
        let updated = vec.set(10, 999).expect("in bounds");
        assert_eq!(updated.get(10), Some(&999));
        assert_eq!(vec.get(10), Some(&10), "set must not mutate the original");

        let popped = vec.pop_back().expect("non-empty");
        assert_eq!(popped.len(), 39);
        assert_eq!(vec.len(), 40, "pop_back must not mutate the original");

        let empty: PVec<i64> = PVec::new();
        assert!(empty.pop_back().is_err());
        assert!(empty.set(0, 1).is_err());
    }

    #[test]
    fn pvec_content_hash_is_order_sensitive() {
        let a: PVec<u64> = [1u64, 2, 3].into_iter().fold(PVec::new(), |v, x| v.push_back(x));
        let b: PVec<u64> = [3u64, 2, 1].into_iter().fold(PVec::new(), |v, x| v.push_back(x));
        assert_ne!(a.content_hash(), b.content_hash());
        let a_again: PVec<u64> = [1u64, 2, 3].into_iter().fold(PVec::new(), |v, x| v.push_back(x));
        assert_eq!(a.content_hash(), a_again.content_hash());
    }
    //#endregion 🔖️PVec

    //#region 🔖️PText
    #[test]
    fn ptext_insert_delete_roundtrip() {
        let text = PText::from_text("hello world");
        let inserted = text.insert(5, ",").expect("in bounds");
        assert_eq!(inserted.to_string(), "hello, world");
        let deleted = inserted.delete(5, 6).expect("in bounds");
        assert_eq!(deleted.to_string(), "hello world");
    }

    #[test]
    fn ptext_slice_and_concat() {
        let text = PText::from_text("hello world");
        let slice = text.slice(6, 11).expect("in bounds");
        assert_eq!(slice.to_string(), "world");
        let rejoined = PText::from_text("hello ").concat(&slice);
        assert_eq!(rejoined.to_string(), "hello world");
        assert_eq!(rejoined.len(), text.len());
    }

    #[test]
    fn ptext_handles_multibyte_unicode_by_char_index() {
        let text = PText::from_text("héllo→wörld");
        let char_len = "héllo→wörld".chars().count();
        assert_eq!(text.len(), char_len);
        let inserted = text.insert(6, "🎉️").expect("in bounds");
        assert_eq!(inserted.to_string(), "héllo→🎉️wörld");
    }

    #[test]
    fn ptext_out_of_bounds_operations_error_instead_of_panicking() {
        let text = PText::from_text("abc");
        assert!(text.insert(10, "x").is_err());
        assert!(text.slice(0, 10).is_err());
        assert!(text.delete(2, 1).is_err());
    }

    #[test]
    fn ptext_edits_are_persistent() {
        let original = PText::from_text("abc");
        let edited = original.insert(1, "X").expect("in bounds");
        assert_eq!(original.to_string(), "abc");
        assert_eq!(edited.to_string(), "aXbc");
    }
    //#endregion 🔖️PText

    //#region 🔖️PTree
    #[test]
    fn ptree_insert_get_and_ordered_iteration() {
        let mut tree: PTree<i64, String> = PTree::new();
        let mut keys: Vec<i64> = (0..300).collect();
        // insertion order deliberately not sorted, to exercise rebalancing on both sides.
        keys.sort_by_key(|k| (k * 2654435761u32 as i64) % 9973);
        for k in &keys {
            tree = tree.insert(*k, format!("v{k}"));
        }
        assert_eq!(tree.len(), 300);
        for k in 0..300i64 {
            assert_eq!(tree.get(&k), Some(&format!("v{k}")));
        }
        let iterated: Vec<i64> = tree.iter().map(|(k, _)| *k).collect();
        let mut sorted = iterated.clone();
        sorted.sort();
        assert_eq!(iterated, sorted, "PTree::iter must yield ascending key order");
    }

    #[test]
    fn ptree_stays_balanced_within_the_avl_bound() {
        let tree: PTree<i64, ()> = (0..1000i64).fold(PTree::new(), |t, k| t.insert(k, ()));
        let n = tree.len() as f64;
        // AVL worst-case height bound: h <= 1.4405 * log2(n + 2) - 0.3277 (Knuth); a couple of
        // integer units of slack keeps this from being brittle to +/-1 rotation-count differences.
        let bound = (1.4405 * (n + 2.0).log2() - 0.3277).ceil() as u32 + 2;
        assert!(tree.height() <= bound, "height {} exceeds AVL bound {}", tree.height(), bound);
    }

    #[test]
    fn ptree_remove_maintains_correctness_and_persistence() {
        let full: PTree<i64, i64> = (0..200i64).fold(PTree::new(), |t, k| t.insert(k, k * 10));
        let mut reduced = full.clone();
        for k in (0..200i64).step_by(3) {
            reduced = reduced.remove(&k);
        }
        for k in 0..200i64 {
            if k % 3 == 0 {
                assert_eq!(reduced.get(&k), None);
                assert_eq!(full.get(&k), Some(&(k * 10)), "original must be unaffected by removals on the derived tree");
            } else {
                assert_eq!(reduced.get(&k), Some(&(k * 10)));
            }
        }
    }
    //#endregion 🔖️PTree

    //#region 🔖️PGraph
    #[test]
    fn pgraph_add_and_query_edges() {
        let graph: PGraph<String, (), &'static str> = PGraph::new().add_node("a".to_string(), ()).add_node("b".to_string(), ()).add_node("c".to_string(), ());
        let graph = graph.add_edge("a".to_string(), "b".to_string(), "ab").expect("nodes exist");
        let graph = graph.add_edge("a".to_string(), "c".to_string(), "ac").expect("nodes exist");

        assert!(graph.has_edge(&"a".to_string(), &"b".to_string()));
        assert_eq!(graph.edge_data(&"a".to_string(), &"b".to_string()), Some(&"ab"));
        let mut neighbors: Vec<String> = graph.neighbors(&"a".to_string()).into_iter().cloned().collect();
        neighbors.sort();
        assert_eq!(neighbors, vec!["b".to_string(), "c".to_string()]);
        assert_eq!(graph.predecessors(&"b".to_string()), vec![&"a".to_string()]);
    }

    #[test]
    fn pgraph_add_edge_rejects_missing_nodes() {
        let graph: PGraph<String, (), ()> = PGraph::new().add_node("a".to_string(), ());
        let result = graph.add_edge("a".to_string(), "ghost".to_string(), ());
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }

    #[test]
    fn pgraph_remove_node_cascades_edge_cleanup() {
        let graph: PGraph<String, (), ()> = PGraph::new().add_node("a".to_string(), ()).add_node("b".to_string(), ()).add_node("c".to_string(), ());
        let graph = graph.add_edge("a".to_string(), "b".to_string(), ()).unwrap();
        let graph = graph.add_edge("b".to_string(), "c".to_string(), ()).unwrap();

        let reduced = graph.remove_node(&"b".to_string());
        assert!(!reduced.contains_node(&"b".to_string()));
        assert!(reduced.neighbors(&"a".to_string()).is_empty(), "edge a->b must be gone");
        assert!(reduced.predecessors(&"c".to_string()).is_empty(), "edge b->c must be gone");
        assert!(graph.contains_node(&"b".to_string()), "original graph must be unaffected");
        assert!(graph.has_edge(&"a".to_string(), &"b".to_string()));
    }

    #[test]
    fn pgraph_content_hash_ignores_insertion_order() {
        let g1: PGraph<String, u64, u64> = PGraph::new().add_node("a".to_string(), 1).add_node("b".to_string(), 2).add_edge("a".to_string(), "b".to_string(), 7).unwrap();
        let g2: PGraph<String, u64, u64> = PGraph::new().add_node("b".to_string(), 2).add_node("a".to_string(), 1).add_edge("a".to_string(), "b".to_string(), 7).unwrap();
        assert_eq!(g1.content_hash(), g2.content_hash());
    }
    //#endregion 🔖️PGraph

    //#region 🔖️Pages
    #[test]
    fn page_store_interns_identical_bytes_once() {
        let mut store = PageStore::new();
        let h1 = store.intern(b"hello".to_vec());
        let h2 = store.intern(b"hello".to_vec());
        let h3 = store.intern(b"world".to_vec());
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert_eq!(store.len(), 2);
        assert_eq!(store.get(&h1).as_deref(), Some(b"hello".as_slice()));
    }
    //#endregion 🔖️Pages

    //#region 🔖️Overlay
    struct FixedBase(std::collections::HashMap<String, Vec<u8>>);
    impl BaseSource for FixedBase {
        fn load(&self, path: &str) -> Result<Option<Vec<u8>>, DbError> {
            Ok(self.0.get(path).cloned())
        }
    }

    #[test]
    fn overlay_reads_fall_through_to_base_when_unset() {
        let mut base_data = std::collections::HashMap::new();
        base_data.insert("x".to_string(), b"base-x".to_vec());
        let root = OverlayRoot::new(FixedBase(base_data));
        assert_eq!(root.get("x").unwrap(), Some(b"base-x".to_vec()));
        assert_eq!(root.get("missing").unwrap(), None);
    }

    #[test]
    fn overlay_set_shadows_base_and_delete_tombstones_it() {
        let mut base_data = std::collections::HashMap::new();
        base_data.insert("x".to_string(), b"base-x".to_vec());
        let root = OverlayRoot::new(FixedBase(base_data));

        let (set_root, touched) = root.set("x", b"overlay-x".to_vec());
        assert_eq!(touched, TouchedRegion::write("x"));
        assert_eq!(set_root.get("x").unwrap(), Some(b"overlay-x".to_vec()));
        assert_eq!(root.get("x").unwrap(), Some(b"base-x".to_vec()), "original overlay root must be unaffected");

        let (deleted_root, _) = set_root.delete("x");
        assert_eq!(deleted_root.get("x").unwrap(), None, "delete must tombstone even though base still has a value");
    }

    #[test]
    fn overlay_root_clone_shares_base_and_overlay_cheaply() {
        let root = OverlayRoot::new(EmptyBase);
        let (a, _) = root.set("p", b"1".to_vec());
        let b = a.clone();
        assert_eq!(b.get("p").unwrap(), Some(b"1".to_vec()));
        assert_eq!(a.overlay_len(), b.overlay_len());
    }
    //#endregion 🔖️Overlay

    //#region 🔖️TouchedRegion
    #[test]
    fn touched_region_prefix_intersection() {
        let whole = TouchedRegion::write("doc/fields");
        let nested = TouchedRegion::read("doc/fields/title");
        let sibling = TouchedRegion::read("doc/other");
        assert!(whole.path_intersects(&nested));
        assert!(nested.path_intersects(&whole));
        assert!(!whole.path_intersects(&sibling));
        assert!(!TouchedRegion::read("doc/fie").path_intersects(&TouchedRegion::read("doc/fields")));
    }

    #[test]
    fn touched_set_conflicts_only_when_a_write_is_involved() {
        let mut a = TouchedSet::new();
        a.record(TouchedRegion::write("doc/title"));
        let mut b_write = TouchedSet::new();
        b_write.record(TouchedRegion::write("doc/title"));
        assert!(a.conflicts_with(&b_write));

        let mut a_read = TouchedSet::new();
        a_read.record(TouchedRegion::read("doc/title"));
        let mut b_read = TouchedSet::new();
        b_read.record(TouchedRegion::read("doc/title"));
        assert!(!a_read.conflicts_with(&b_read), "two reads of the same region must not conflict");

        let mut disjoint = TouchedSet::new();
        disjoint.record(TouchedRegion::write("doc/body"));
        assert!(!a.conflicts_with(&disjoint));
    }
    //#endregion 🔖️TouchedRegion
}
//#endregion 🧪️Tests
