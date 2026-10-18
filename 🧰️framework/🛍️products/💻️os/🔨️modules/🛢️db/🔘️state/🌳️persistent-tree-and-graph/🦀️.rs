//! 🌳️ Persistent tree and graph: the balanced tree with `PTree` and the persistent graph `PGraph`. See [`super`].

use super::*;

//#region 🔖️PTree
struct TreeNode<K, V> {
    pub(super) key: K,
    pub(super) value: V,
    pub(super) left: Option<Rc<TreeNode<K, V>>>,
    pub(super) right: Option<Rc<TreeNode<K, V>>>,
    pub(super) height: u32,
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

/// ⚖️ Restores the AVL invariant (`|balance_factor| <= 1`) at `node`'s root via at most one
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

/// 🌳️ A persistent (immutable, structurally-shared) ordered map: an AVL tree. Unlike
/// `PMap`, iteration order is the key order (`K: Ord`), and height is kept `O(log n)` by
/// rebalancing on every insert/remove — the property `db_index`'s sorted-run merges will lean on.
pub struct PTree<K, V> {
    pub(super) root: Option<Rc<TreeNode<K, V>>>,
    pub(super) len: usize,
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

    /// 📏️ The tree's current height — exposed so tests (and callers with their own
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
    /// 🔑️ Content hash over `(key, value)` pairs in ascending key order (already the
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
/// 🕸️ A persistent (immutable, structurally-shared) directed graph: node data plus
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

    /// 🧹️ Removes `id` and every edge touching it (both directions) — the persistent
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
    /// 🔑️ Content hash over the node set (sorted, via `PMap::content_hash`) followed by
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
