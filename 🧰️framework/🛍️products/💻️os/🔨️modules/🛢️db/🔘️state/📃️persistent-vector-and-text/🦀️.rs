//! 📃️ Persistent vector and text: the vector nodes with `PVec` and the rope with `PText`. See [`super`].

use super::*;

//#region 🔖️PVec
/// 🪜️ 5 bits routed per `PVec` trie level (32-way branching, matching `PMap`).
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

/// 🌱️ Builds a fresh single-child chain down to a one-element leaf holding `value`. Only
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

/// 🧵️ A persistent (immutable, structurally-shared) vector: a bitmapped trie, 32-way
/// branching. `push_back`/`set`/`pop_back` are `O(log₃₂ n)`; unaffected subtrees are shared.
pub struct PVec<T> {
    pub(super) root: Rc<VecNode<T>>,
    pub(super) shift: u32,
    pub(super) len: usize,
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
    /// 🔑️ Content hash over elements in index order (order-sensitive, unlike `PMap`'s).
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

/// ➕️ Concatenates two rope nodes, dropping an empty side rather than wrapping it — keeps
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

/// ✂️ Splits at char offset `at` (`0 <= at <= rope_chars(node)`, checked by every public
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

/// 📜️ A persistent (immutable, structurally-shared) text rope. `insert`/`delete`/`slice`
/// are expressed as `split` + `concat`, sharing every untouched leaf with the rope(s) they were
/// derived from.
///
/// 🧩️ Extension seam: no active rebalancing — a long run of single-character edits at the same
/// offset grows an unbalanced concat chain (still correct, `O(n)` worst case per edit instead of
/// `O(log n)`). A rebalance-on-concat pass (e.g. depth/weight-triggered, as real rope
/// implementations do) is a self-contained follow-up that does not change this type's public API.
#[derive(Clone)]
pub struct PText(pub(super) Rc<RopeNode>);

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

    /// 🔑️ Content hash of the rope's flattened UTF-8 bytes.
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
