//! 🧰️ Graph support: the interner, `GraphError`, the numeric tolerances and the mapped heap.

use super::*;

// #region 🔖️Interner
/// 🔤️ Generalized, bidirectional label<->`NodeId` map — the generic successor to the string-only `algorithms::IdIndex` (which stays untouched for old call sites). `intern` is idempotent: the same label always maps to the same id.
// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01): `L` is arbitrary (`Ord + Clone + Hash` only — the ONLY instantiation anywhere in the
// repo is `Interner<String>`, confined to this file's own `#[cfg(test)] mod tests`, never consumed
// by any other module). `by_label: HashMap<L, NodeId>` needs the `BTreeMap`/`HashMap`-blanket's
// `L: ToString` bound (`🌱️value/🔁️codec`), which the derive's auto-synthesized `L: ToValue +
// FromValue` bound does not provide — forcing a `#[value(bound = "L: ToString + FromStr + …")]`
// override would newly constrain every FUTURE `L`, an API change with zero present benefit for a
// generic type nothing outside its own tests instantiates.
#[derive(Clone, Debug, Default)]
pub struct Interner<L: Ord + Clone + std::hash::Hash> {
    labels: Vec<L>,
    by_label: std::collections::HashMap<L, NodeId>,
}

impl<L: Ord + Clone + std::hash::Hash> Interner<L> {
    pub fn new() -> Self {
        Self { labels: Vec::new(), by_label: std::collections::HashMap::new() }
    }

    /// 🏗️ Builds an interner from labels sorted for deterministic id assignment; duplicate labels collapse to one id.
    pub fn from_labels(labels: impl IntoIterator<Item = L>) -> Self {
        let mut sorted: Vec<L> = labels.into_iter().collect();
        sorted.sort();
        sorted.dedup();
        let mut interner = Self::new();
        for label in sorted {
            interner.intern(label);
        }
        interner
    }

    /// ➕️ Returns the existing id for `label` if already interned, otherwise allocates the next sequential id.
    pub fn intern(&mut self, label: L) -> NodeId {
        if let Some(&id) = self.by_label.get(&label) {
            return id;
        }
        let id = self.labels.len() as NodeId;
        self.labels.push(label.clone());
        self.by_label.insert(label, id);
        id
    }

    pub fn label_of(&self, id: NodeId) -> Option<&L> {
        self.labels.get(id as usize)
    }

    pub fn id_of(&self, label: &L) -> Option<NodeId> {
        self.by_label.get(label).copied()
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}
// #endregion 🔖️Interner

// #region 🔖️GraphError
/// 🚨️ Flat, non-generic error enum mirroring the NetworkX exception hierarchy; every downstream algorithm crate returns `Result<_, GraphError>`. Nothing here is generic over node/edge label types — everything is `NodeId`/`EdgeId`/`u64`/`String` — so this shape stays stable across the whole family.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed and the derive's default
// externally-tagged shape (unit variant -> bare wire-name string, data-carrying variant ->
// `{"VariantName": payload}`) is simply the new wire shape. `NotImplementedForKind`'s two fields
// moved `&'static str` -> `String`: `FromValue` needs an OWNED `Self`, and no runtime-decoded
// string can honestly become `&'static str` without leaking memory — a greenfield, no-legacy-API
// repo (CLAUDE.md) fixes the field type instead of working around it (both call sites already
// pass a string literal, which `.into()`s into `String` for free).
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum GraphError {
    NodeNotFound(NodeId),
    EdgeNotFound(EdgeId),
    NoPath { source: NodeId, target: NodeId },
    HasACycle,
    NoCycle,
    Unfeasible(String),
    Unbounded(String),
    NotATree,
    NotAForest,
    NotBipartite,
    NotPlanar,
    NotEulerian,
    NotConnected,
    NotStronglyConnected,
    AmbiguousSolution(String),
    ExceededMaxIterations { iterations: usize },
    PowerIterationFailedConvergence { iterations: usize },
    NegativeCycle,
    NotGraphical(String),
    NotImplementedForKind { algorithm: String, kind: String },
    Io(String),
    Parse { line: usize, message: String },
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(id) => write!(f, "node {id} not found"),
            GraphError::EdgeNotFound(id) => write!(f, "edge {id} not found"),
            GraphError::NoPath { source, target } => write!(f, "no path from node {source} to node {target}"),
            GraphError::HasACycle => write!(f, "graph has a cycle"),
            GraphError::NoCycle => write!(f, "graph has no cycle"),
            GraphError::Unfeasible(msg) => write!(f, "unfeasible: {msg}"),
            GraphError::Unbounded(msg) => write!(f, "unbounded: {msg}"),
            GraphError::NotATree => write!(f, "graph is not a tree"),
            GraphError::NotAForest => write!(f, "graph is not a forest"),
            GraphError::NotBipartite => write!(f, "graph is not bipartite"),
            GraphError::NotPlanar => write!(f, "graph is not planar"),
            GraphError::NotEulerian => write!(f, "graph is not eulerian"),
            GraphError::NotConnected => write!(f, "graph is not connected"),
            GraphError::NotStronglyConnected => write!(f, "graph is not strongly connected"),
            GraphError::AmbiguousSolution(msg) => write!(f, "ambiguous solution: {msg}"),
            GraphError::ExceededMaxIterations { iterations } => write!(f, "exceeded max iterations ({iterations})"),
            GraphError::PowerIterationFailedConvergence { iterations } => {
                write!(f, "power iteration failed to converge after {iterations} iterations")
            }
            GraphError::NegativeCycle => write!(f, "graph has a negative cycle"),
            GraphError::NotGraphical(msg) => write!(f, "not a graphical degree sequence: {msg}"),
            GraphError::NotImplementedForKind { algorithm, kind } => write!(f, "{algorithm} is not implemented for {kind}"),
            GraphError::Io(msg) => write!(f, "io error: {msg}"),
            GraphError::Parse { line, message } => write!(f, "parse error at line {line}: {message}"),
        }
    }
}

impl std::error::Error for GraphError {}
// #endregion 🔖️GraphError

// #region 🔖️Utils
/// 🎚️ Strict numeric tolerance for exact-equality-sensitive comparisons (e.g. verifying a closed-form result).
pub const TOL_STRICT: f64 = 1e-9;
/// 🎚️ Loose numeric tolerance for iterative/approximate algorithm convergence checks.
pub const TOL_LOOSE: f64 = 1e-6;

/// 🔗️ Consecutive-pair iterator: `[a, b, c] -> [(a, b), (b, c)]`.
pub fn pairwise<T: Copy>(items: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    items.windows(2).map(|w| (w[0], w[1]))
}

/// 🎯️ Deterministic representative element (the first one) from a slice.
pub fn arbitrary_element<T: Copy>(items: &[T]) -> Option<T> {
    items.first().copied()
}

/// 🗳️ Binary-heap priority queue with `decrease_key`, ordered by `K` and keyed by `V` identity; a position index makes membership/decrease `O(log n)` instead of the `O(n)` a plain `BinaryHeap` needs for those operations.
// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), same rationale as `Interner` above: `K`/`V` are arbitrary, nothing outside this file's
// own `#[cfg(test)] mod tests` ever instantiates `MappedHeap` (there, `MappedHeap<i64, &str>` — a
// BORROWED `V` that structurally CANNOT implement `FromValue`, which needs an owned `Self`),
// `position: HashMap<V, usize>` needs `V: ToString` the auto bound does not provide, and no
// external consumer exists to benefit from the bound override this would force.
#[derive(Clone, Debug)]
pub struct MappedHeap<K: Ord, V: Eq + std::hash::Hash + Clone> {
    heap: Vec<(K, V)>,
    position: std::collections::HashMap<V, usize>,
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> Default for MappedHeap<K, V> {
    // 🚫️async: E1 impl of external trait `std::default::Default` — must stay sync; mirrors `new()`'s
    // literal (I/O-free) body directly. See R9, and the identical `Storage::default` fix above.
    fn default() -> Self {
        Self { heap: Vec::new(), position: std::collections::HashMap::new() }
    }
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> MappedHeap<K, V> {
    pub fn new() -> Self {
        Self { heap: Vec::new(), position: std::collections::HashMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn contains(&self, item: &V) -> bool {
        self.position.contains_key(item)
    }

    /// ➕️ Pushes `item` at `priority` if absent, or decreases its priority if `priority` is lower than its current one; no-operation if `item` is present with an already-lower-or-equal priority.
    pub fn push_or_decrease(&mut self, item: V, priority: K) {
        if let Some(&i) = self.position.get(&item) {
            if priority < self.heap[i].0 {
                self.heap[i].0 = priority;
                self.sift_up(i);
            }
        } else {
            self.heap.push((priority, item.clone()));
            let i = self.heap.len() - 1;
            self.position.insert(item, i);
            self.sift_up(i);
        }
    }

    /// 🔽️ Lowers `item`'s priority; returns `false` (no-operation) if `item` isn't present or `priority` isn't lower than its current one.
    pub fn decrease_key(&mut self, item: &V, priority: K) -> bool {
        let Some(&i) = self.position.get(item) else { return false };
        if priority < self.heap[i].0 {
            self.heap[i].0 = priority;
            self.sift_up(i);
            true
        } else {
            false
        }
    }

    pub fn pop_min(&mut self) -> Option<(K, V)> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.swap(0, last);
        let (priority, item) = self.heap.pop().expect("heap checked non-empty above");
        self.position.remove(&item);
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        Some((priority, item))
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.position.insert(self.heap[i].1.clone(), i);
        self.position.insert(self.heap[j].1.clone(), j);
    }

    // 🚨️ was a dropped-future no-op throughout: `sift_up`/`sift_down`/`swap` were never awaited at
    // ANY of their call sites in this struct (including here, sift_up/sift_down calling their own
    // `swap`), so the whole `MappedHeap` never actually maintained the heap invariant — every push,
    // decrease-key, and pop silently left `self.heap` in insertion order. See R10 header.
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i].0 < self.heap[parent].0 {
                self.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut smallest = i;
            if l < n && self.heap[l].0 < self.heap[smallest].0 {
                smallest = l;
            }
            if r < n && self.heap[r].0 < self.heap[smallest].0 {
                smallest = r;
            }
            if smallest == i {
                break;
            }
            self.swap(i, smallest);
            i = smallest;
        }
    }
}
// #endregion 🔖️Utils
