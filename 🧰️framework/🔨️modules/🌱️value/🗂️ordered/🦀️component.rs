//! 🗂️ Immutable ordered ownership with byte-resumable comparison, path copying, and final-owner retirement.

use std::cmp::Ordering;
use std::collections::LinkedList;
use std::mem::ManuallyDrop;
use std::sync::Arc;

#[path = "🧺️set/🦀️component.rs"]
pub mod set;
pub use set::OrderedSet;

//#region 🌳️PersistentNodes
type Root<V> = Option<Arc<Node<V>>>;
/// 📏️ Fixed metadata-visit bound for one rank-iterator item; no payload bytes are read by iteration.
pub const MAX_AVL_HEIGHT: usize = 2 * usize::BITS as usize;
struct Entry<V> { key: Arc<String>, value: Arc<V> }
struct Node<V> { entry: Arc<Entry<V>>, left: Root<V>, right: Root<V>, height: usize, len: usize }

impl<V> Clone for Entry<V> {
    fn clone(&self) -> Self { Self { key: Arc::clone(&self.key), value: Arc::clone(&self.value) } }
}
impl<V> Clone for Node<V> {
    fn clone(&self) -> Self { Self { entry: Arc::clone(&self.entry), left: self.left.clone(), right: self.right.clone(), height: self.height, len: self.len } }
}

fn height<V>(root: &Root<V>) -> usize { root.as_ref().map_or(0, |value| value.height) }
fn len<V>(root: &Root<V>) -> usize { root.as_ref().map_or(0, |value| value.len) }
fn node<V>(entry: Arc<Entry<V>>, left: Root<V>, right: Root<V>) -> Arc<Node<V>> {
    let height = 1 + height(&left).max(height(&right));
    assert!(height <= MAX_AVL_HEIGHT, "ordered-map fixed metadata frontier exceeded");
    Arc::new(Node { height, len: 1 + len(&left) + len(&right), entry, left, right })
}

fn balanced<V>(entry: Arc<Entry<V>>, left: Root<V>, right: Root<V>) -> Arc<Node<V>> {
    if height(&left) > height(&right) + 1 {
        let branch = left.as_ref().unwrap();
        if height(&branch.left) >= height(&branch.right) {
            let right = node(entry, branch.right.clone(), right);
            return node(Arc::clone(&branch.entry), branch.left.clone(), Some(right));
        }
        let pivot = branch.right.as_ref().unwrap();
        let left = node(Arc::clone(&branch.entry), branch.left.clone(), pivot.left.clone());
        let right = node(entry, pivot.right.clone(), right);
        return node(Arc::clone(&pivot.entry), Some(left), Some(right));
    }
    if height(&right) > height(&left) + 1 {
        let branch = right.as_ref().unwrap();
        if height(&branch.right) >= height(&branch.left) {
            let left = node(entry, left, branch.left.clone());
            return node(Arc::clone(&branch.entry), Some(left), branch.right.clone());
        }
        let pivot = branch.left.as_ref().unwrap();
        let left = node(entry, left, pivot.left.clone());
        let right = node(Arc::clone(&branch.entry), pivot.right.clone(), branch.right.clone());
        return node(Arc::clone(&pivot.entry), Some(left), Some(right));
    }
    node(entry, left, right)
}

fn at<V>(mut root: &Root<V>, mut index: usize) -> Option<&Entry<V>> {
    loop {
        let current = root.as_ref()?;
        match index.cmp(&len(&current.left)) {
            Ordering::Less => root = &current.left,
            Ordering::Equal => return Some(&current.entry),
            Ordering::Greater => { index -= len(&current.left) + 1; root = &current.right; }
        }
    }
}
//#endregion 🌳️PersistentNodes

//#region 🗂️Map
/// 🗂️ Ordered immutable root; clones share payloads and every nonempty owner must be explicitly retired.
/// 🔒️ Dropping live ownership panics without destroying payloads; unwinding preserves it without a second panic.
#[must_use = "ordered roots must be transferred or explicitly retired"]
pub struct OrderedMap<V> { root: ManuallyDrop<Root<V>> }

impl<V> Default for OrderedMap<V> { fn default() -> Self { Self { root: ManuallyDrop::new(None) } } }
impl<V> Clone for OrderedMap<V> { fn clone(&self) -> Self { Self { root: self.root.clone() } } }
impl<V> Drop for OrderedMap<V> {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.root.is_none(), "ordered-map root must be explicitly retired before drop"); } }
}

impl<V> OrderedMap<V> {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { len(&self.root) }
    pub fn is_empty(&self) -> bool { self.root.is_none() }
    pub fn iter(&self) -> Iter<'_, V> { Iter { root: &self.root, front: 0, back: self.len() } }
    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &String> + ExactSizeIterator { self.iter().map(|(key, _)| key) }
    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> + ExactSizeIterator { self.iter().map(|(_, value)| value) }
    pub fn first_key_value(&self) -> Option<(&String, &V)> { self.iter().next() }
    /// 📍️ Borrows one ranked entry with at most MAX_AVL_HEIGHT metadata visits; charge one retained item.
    pub fn entry_at_rank(&self, index: usize) -> Option<(&String, &V)> { at(&self.root, index).map(|entry| (entry.key.as_ref(), entry.value.as_ref())) }
    /// 🧊️ Cold synchronous lookup; retained callers must use begin_lookup to account comparison bytes.
    pub fn get(&self, key: &str) -> Option<&V> {
        let mut root: &Root<V> = &self.root;
        loop {
            let current = root.as_ref()?;
            match key.cmp(current.entry.key.as_str()) {
                Ordering::Less => root = &current.left, Ordering::Greater => root = &current.right,
                Ordering::Equal => return Some(&current.entry.value),
            }
        }
    }
    /// 🧊️ Cold synchronous membership; no interactive accounting is provided.
    pub fn contains_key(&self, key: &str) -> bool { self.get(key).is_some() }

    /// ✏️ Begins an upsert by moving inline V; retained callers must admit its inline size or use begin_set_shared.
    pub fn begin_set(&self, key: String, value: V) -> UpdateCursor<V> { self.begin_set_shared(Arc::new(key), Arc::new(value)) }
    /// 📥️ Begins a retained upsert by moving exactly two shared pointers; no key or value bytes are copied.
    pub fn begin_set_shared(&self, key: Arc<String>, value: Arc<V>) -> UpdateCursor<V> { UpdateCursor::new(self.clone(), key, Some(value)) }
    /// 🗑️ Begins a retained removal; missing keys leave an identical shared root.
    pub fn begin_remove(&self, key: String) -> UpdateCursor<V> { self.begin_remove_shared(Arc::new(key)) }
    /// 🗑️ Moves an exact shared key into retained removal without copying its bytes.
    pub fn begin_remove_shared(&self, key: Arc<String>) -> UpdateCursor<V> { UpdateCursor::new(self.clone(), key, None) }
    /// 🔎️ Retains an immutable root and compares a lookup key under the caller's byte grants.
    pub fn begin_lookup(&self, key: String) -> LookupCursor<V> { self.begin_lookup_shared(Arc::new(key)) }
    /// 🔎️ Moves an exact shared key into retained lookup without copying its bytes.
    pub fn begin_lookup_shared(&self, key: Arc<String>) -> LookupCursor<V> { LookupCursor::new(self.clone(), key) }

    /// ⚡️ Completes a synchronous convenience upsert; retained jobs must use begin_set and advance.
    pub fn insert(&mut self, key: String, value: V) -> Option<Arc<V>> {
        let mut cursor = self.begin_set(key, value);
        while !cursor.is_complete() { cursor.advance(Grant { maximum_items: 1, maximum_bytes: 4096 }); }
        let removed = cursor.take_removed(); let displaced = std::mem::replace(self, cursor.take_result().unwrap()); retire_cold(displaced.retire()); close_cold(&mut cursor); removed
    }
    /// ⚡️ Completes a synchronous convenience removal; retained jobs must use begin_remove and advance.
    pub fn remove(&mut self, key: &str) -> Option<Arc<V>> {
        let mut cursor = self.begin_remove(key.to_owned());
        while !cursor.is_complete() { cursor.advance(Grant { maximum_items: 1, maximum_bytes: 4096 }); }
        let removed = cursor.take_removed(); let displaced = std::mem::replace(self, cursor.take_result().unwrap()); retire_cold(displaced.retire()); close_cold(&mut cursor); removed
    }
    /// 🧹️ Moves this root into explicit final-owner retirement.
    pub fn retire(mut self) -> Retirement<V> { Retirement::new(self.root.take()) }
    /// 📤️ Atomically releases a shared root or transfers its exact final ownership without traversing payloads.
    pub fn release_shared(mut self) -> Result<(), Retirement<V>> {
        let Some(root) = self.root.take() else { return Ok(()); };
        let Some(node) = Arc::into_inner(root) else { return Ok(()); };
        let mut retirement = Retirement::default();
        if let Some(left) = node.left { retirement.owners.push_front(Owner::Node(left)); }
        if let Some(right) = node.right { retirement.owners.push_front(Owner::Node(right)); }
        retirement.owners.push_front(Owner::Entry(node.entry));
        Err(retirement)
    }
}

/// 👁️ Each rank item visits at most MAX_AVL_HEIGHT fixed metadata nodes without reading payload bytes.
/// 🎟️ Retained consumers must account each next/next_back as one fixed metadata item.
pub struct Iter<'a, V> { root: &'a Root<V>, front: usize, back: usize }
impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a String, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back { return None; }
        let entry = at(self.root, self.front)?; self.front += 1; Some((&entry.key, &entry.value))
    }
    fn size_hint(&self) -> (usize, Option<usize>) { let len = self.back - self.front; (len, Some(len)) }
}
impl<V> DoubleEndedIterator for Iter<'_, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back { return None; }
        self.back -= 1; let entry = at(self.root, self.back)?; Some((&entry.key, &entry.value))
    }
}
impl<V> ExactSizeIterator for Iter<'_, V> {}
impl<'a, V> IntoIterator for &'a OrderedMap<V> {
    type Item = (&'a String, &'a V);
    type IntoIter = Iter<'a, V>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl<V: PartialEq> PartialEq for OrderedMap<V> { fn eq(&self, other: &Self) -> bool { self.iter().eq(other.iter()) } }
impl<V: Eq> Eq for OrderedMap<V> {}
impl<V: std::fmt::Debug> std::fmt::Debug for OrderedMap<V> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_map().entries(self.iter()).finish() } }
/// 🧊️ Cold synchronous construction explicitly drains displaced roots; it provides no interactive credit.
impl<V> FromIterator<(String, V)> for OrderedMap<V> {
    fn from_iter<T: IntoIterator<Item = (String, V)>>(entries: T) -> Self { let mut map = Self::new(); for (key, value) in entries { map.insert(key, value); } map }
}
impl<V, const N: usize> From<[(String, V); N]> for OrderedMap<V> { fn from(entries: [(String, V); N]) -> Self { entries.into_iter().collect() } }
//#endregion 🗂️Map

//#region 🧵️UpdateCursor
/// 🎟️ One structural phase or a byte-bounded comparison fragment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grant { pub maximum_items: usize, pub maximum_bytes: usize }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step { Blocked, Progress { completed_items: usize, completed_bytes: usize }, Complete }

fn compare_bytes(key: &[u8], other: &[u8], offset: &mut usize, left_byte: &mut Option<u8>, ordering: &mut Option<Ordering>, maximum_bytes: usize) -> usize {
    let mut bytes = 0;
    while ordering.is_none() && bytes < maximum_bytes {
        if *offset == key.len().min(other.len()) { *ordering = Some(key.len().cmp(&other.len())); break; }
        if let Some(left) = left_byte.take() {
            let right = other[*offset]; bytes += 1; *offset += 1;
            if left != right { *ordering = Some(left.cmp(&right)); }
        } else { *left_byte = Some(key[*offset]); bytes += 1; }
    }
    bytes
}

enum Phase { Search, Successor, RebuildSuccessor, Rebuild, Complete }
struct Parent<V> { node: Arc<Node<V>>, left: bool }

/// 🧵️ Retains both immutable roots until explicit result handoff and close; no payload Clone bound.
struct UpdateState<V> {
    base: OrderedMap<V>, key: Option<Arc<String>>, value: Option<Arc<V>>, current: Root<V>, path: LinkedList<Parent<V>>,
    successor_path: LinkedList<Arc<Node<V>>>, removed_node: Root<V>, successor_entry: Option<Arc<Entry<V>>>,
    replacement: Root<V>, removed: Option<Arc<V>>, result: Option<OrderedMap<V>>, phase: Phase,
    offset: usize, left_byte: Option<u8>, ordering: Option<Ordering>, retirement: Retirement<V>, closing: bool,
}

impl<V> UpdateState<V> {
    fn new(base: OrderedMap<V>, key: Arc<String>, value: Option<Arc<V>>) -> Self {
        Self { current: (*base.root).clone(), base, key: Some(key), value, path: LinkedList::new(), successor_path: LinkedList::new(), removed_node: None,
            successor_entry: None, replacement: None, removed: None, result: None, phase: Phase::Search, offset: 0, left_byte: None, ordering: None, retirement: Retirement::default(), closing: false }
    }

    fn compare(&mut self, maximum_bytes: usize) -> usize {
        compare_bytes(self.key.as_ref().unwrap().as_bytes(), self.current.as_ref().unwrap().entry.key.as_bytes(), &mut self.offset, &mut self.left_byte, &mut self.ordering, maximum_bytes)
    }

    pub fn advance(&mut self, grant: Grant) -> Step {
        if self.closing || grant.maximum_items == 0 || grant.maximum_bytes == 0 { return Step::Blocked; }
        let mut bytes = 0;
        match self.phase {
            Phase::Search => {
                if self.current.is_none() {
                    self.replacement = self.value.as_ref().map(|value| node(Arc::new(Entry { key: Arc::clone(self.key.as_ref().unwrap()), value: Arc::clone(value) }), None, None));
                    self.phase = Phase::Rebuild;
                } else if self.ordering.is_none() { bytes = self.compare(grant.maximum_bytes); }
                else {
                    let current = self.current.take().unwrap();
                    match self.ordering.take().unwrap() {
                        ordering @ (Ordering::Less | Ordering::Greater) => {
                            let direction = ordering == Ordering::Less;
                            self.current = if direction { current.left.clone() } else { current.right.clone() };
                            self.path.push_front(Parent { node: current, left: direction });
                        }
                        Ordering::Equal => {
                            self.removed = Some(Arc::clone(&current.entry.value));
                            if let Some(value) = &self.value {
                                self.replacement = Some(node(Arc::new(Entry { key: Arc::clone(self.key.as_ref().unwrap()), value: Arc::clone(value) }), current.left.clone(), current.right.clone()));
                                self.phase = Phase::Rebuild;
                            } else if current.left.is_none() || current.right.is_none() {
                                self.replacement = current.left.clone().or_else(|| current.right.clone()); self.phase = Phase::Rebuild;
                            } else { self.current = current.right.clone(); self.removed_node = Some(current); self.phase = Phase::Successor; }
                        }
                    }
                    self.offset = 0; self.left_byte = None;
                }
            }
            Phase::Successor => {
                let current = self.current.take().unwrap();
                if current.left.is_some() { self.current = current.left.clone(); self.successor_path.push_front(current); }
                else { self.successor_entry = Some(Arc::clone(&current.entry)); self.replacement = current.right.clone(); self.phase = Phase::RebuildSuccessor; }
            }
            Phase::RebuildSuccessor => {
                if let Some(parent) = self.successor_path.pop_front() { self.replacement = Some(balanced(Arc::clone(&parent.entry), self.replacement.take(), parent.right.clone())); }
                else {
                    let removed = self.removed_node.take().unwrap();
                    self.replacement = Some(balanced(self.successor_entry.take().unwrap(), removed.left.clone(), self.replacement.take())); self.phase = Phase::Rebuild;
                }
            }
            Phase::Rebuild => {
                if let Some(parent) = self.path.pop_front() {
                    self.replacement = Some(if parent.left { balanced(Arc::clone(&parent.node.entry), self.replacement.take(), parent.node.right.clone()) }
                        else { balanced(Arc::clone(&parent.node.entry), parent.node.left.clone(), self.replacement.take()) });
                } else { self.result = Some(OrderedMap { root: ManuallyDrop::new(self.replacement.take()) }); self.phase = Phase::Complete; }
            }
            Phase::Complete => return Step::Complete,
        }
        Step::Progress { completed_items: 1, completed_bytes: bytes }
    }

    pub fn is_complete(&self) -> bool { matches!(self.phase, Phase::Complete) }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.result.take() }
    pub fn take_removed(&mut self) -> Option<Arc<V>> { self.removed.take() }
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_step(&mut self, grant: Grant) -> RetirementStep<V> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { return self.retirement.advance(grant); }
        if let Some(parent) = self.path.pop_front() { self.retirement.owners.push_front(Owner::Node(parent.node)); }
        else if let Some(node) = self.successor_path.pop_front() { self.retirement.owners.push_front(Owner::Node(node)); }
        else if let Some(node) = self.current.take().or_else(|| self.removed_node.take()).or_else(|| self.replacement.take()).or_else(|| self.base.root.take()) { self.retirement.owners.push_front(Owner::Node(node)); }
        else if let Some(mut map) = self.result.take() { if let Some(node) = map.root.take() { self.retirement.owners.push_front(Owner::Node(node)); } }
        else if let Some(entry) = self.successor_entry.take() { self.retirement.owners.push_front(Owner::Entry(entry)); }
        else if let Some(key) = self.key.take() { self.retirement.owners.push_front(Owner::Key(key)); }
        else if let Some(value) = self.value.take().or_else(|| self.removed.take()) { self.retirement.owners.push_front(Owner::Value(value)); }
        else { return RetirementStep::Complete; }
        RetirementStep::Progress { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_empty() && self.key.is_none() && self.value.is_none() && self.current.is_none() && self.path.is_empty()
            && self.successor_path.is_empty() && self.removed_node.is_none() && self.successor_entry.is_none() && self.replacement.is_none()
            && self.removed.is_none() && self.result.is_none() && self.retirement.is_empty()
    }
}

/// 🔒️ Terminal guard prevents unbudgeted destruction of any live cursor alias, including unwinding.
#[must_use = "update cursors must finish explicit close before drop"]
pub struct UpdateCursor<V> { state: ManuallyDrop<UpdateState<V>> }
impl<V> UpdateCursor<V> {
    fn new(base: OrderedMap<V>, key: Arc<String>, value: Option<Arc<V>>) -> Self { Self { state: ManuallyDrop::new(UpdateState::new(base, key, value)) } }
    pub fn advance(&mut self, grant: Grant) -> Step { self.state.advance(grant) }
    pub fn is_complete(&self) -> bool { self.state.is_complete() }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.state.take_result() }
    /// 📤️ Explicit shared-value handoff; the recipient owns its eventual domain retirement.
    pub fn take_removed(&mut self) -> Option<Arc<V>> { self.state.take_removed() }
    pub fn begin_close(&mut self) { self.state.begin_close(); }
    pub fn close_step(&mut self, grant: Grant) -> RetirementStep<V> { self.state.close_step(grant) }
    pub fn terminal_is_empty(&self) -> bool { self.state.terminal_is_empty() }
}
impl<V> Drop for UpdateCursor<V> {
    fn drop(&mut self) {
        if !self.state.terminal_is_empty() {
            assert!(std::thread::panicking(), "ordered-map update must finish explicit close before drop"); return;
        }
        unsafe { ManuallyDrop::drop(&mut self.state); }
    }
}
//#endregion 🧵️UpdateCursor

//#region 🔎️LookupCursor
struct LookupState<V> {
    base: OrderedMap<V>, key: Option<Arc<String>>, current: Root<V>, offset: usize, left_byte: Option<u8>, ordering: Option<Ordering>,
    complete: bool, closing: bool, retirement: Retirement<V>,
}

impl<V> LookupState<V> {
    fn advance(&mut self, grant: Grant) -> Step {
        if self.closing || grant.maximum_items == 0 || grant.maximum_bytes == 0 { return Step::Blocked; }
        if self.complete { return Step::Complete; }
        let mut bytes = 0;
        if self.current.is_none() { self.complete = true; }
        else if self.ordering.is_none() {
            bytes = compare_bytes(self.key.as_ref().unwrap().as_bytes(), self.current.as_ref().unwrap().entry.key.as_bytes(), &mut self.offset, &mut self.left_byte, &mut self.ordering, grant.maximum_bytes);
        } else if self.ordering == Some(Ordering::Equal) { self.complete = true; }
        else {
            let current = self.current.take().unwrap(); self.current = if self.ordering == Some(Ordering::Less) { current.left.clone() } else { current.right.clone() };
            self.offset = 0; self.left_byte = None; self.ordering = None;
        }
        Step::Progress { completed_items: 1, completed_bytes: bytes }
    }

    fn close_step(&mut self, grant: Grant) -> RetirementStep<V> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { return self.retirement.advance(grant); }
        if let Some(node) = self.current.take().or_else(|| self.base.root.take()) { self.retirement.owners.push_front(Owner::Node(node)); }
        else if let Some(key) = self.key.take() { self.retirement.owners.push_front(Owner::Key(key)); }
        else { return RetirementStep::Complete; }
        RetirementStep::Progress { released_items: 1, released_bytes: 0 }
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_empty() && self.current.is_none() && self.key.is_none() && self.retirement.is_empty() }
}

/// 🔎️ Borrowed lookup result stays rooted until this cursor's explicit close; no payload clone.
#[must_use = "lookup cursors must finish explicit close before drop"]
pub struct LookupCursor<V> { state: ManuallyDrop<LookupState<V>> }
impl<V> LookupCursor<V> {
    fn new(base: OrderedMap<V>, key: Arc<String>) -> Self {
        let current = (*base.root).clone();
        Self { state: ManuallyDrop::new(LookupState { base, key: Some(key), current, offset: 0, left_byte: None, ordering: None, complete: false, closing: false, retirement: Retirement::default() }) }
    }
    pub fn advance(&mut self, grant: Grant) -> Step { self.state.advance(grant) }
    pub fn is_complete(&self) -> bool { self.state.complete }
    pub fn result(&self) -> Option<&V> {
        if !self.state.complete || self.state.ordering != Some(Ordering::Equal) { return None; }
        self.state.current.as_ref().map(|node| node.entry.value.as_ref())
    }
    pub fn begin_close(&mut self) { self.state.closing = true; }
    pub fn close_step(&mut self, grant: Grant) -> RetirementStep<V> { self.state.close_step(grant) }
    pub fn terminal_is_empty(&self) -> bool { self.state.terminal_is_empty() }
}
impl<V> Drop for LookupCursor<V> {
    fn drop(&mut self) {
        if !self.state.terminal_is_empty() { assert!(std::thread::panicking(), "ordered-map lookup must finish explicit close before drop"); return; }
        unsafe { ManuallyDrop::drop(&mut self.state); }
    }
}
//#endregion 🔎️LookupCursor

//#region 🧹️Retirement
enum Owner<V> { Node(Arc<Node<V>>), Entry(Arc<Entry<V>>), Key(Arc<String>), Value(Arc<V>), Bytes(Vec<u8>) }
/// 📤️ OwnedValue transfers final payload ownership to the caller's domain retirement cursor.
pub enum RetirementStep<V> { Blocked, Progress { released_items: usize, released_bytes: usize }, OwnedValue(V), Complete }
#[must_use = "retirement owners must be drained before drop"]
pub struct Retirement<V> { owners: ManuallyDrop<LinkedList<Owner<V>>> }
impl<V> Default for Retirement<V> { fn default() -> Self { Self { owners: ManuallyDrop::new(LinkedList::new()) } } }
impl<V> Drop for Retirement<V> { fn drop(&mut self) { if !std::thread::panicking() { assert!(self.owners.is_empty(), "ordered-map retirement must be empty before drop"); } } }
impl<V> Retirement<V> {
    fn new(root: Root<V>) -> Self { let mut owner = Self::default(); if let Some(root) = root { owner.owners.push_front(Owner::Node(root)); } owner }
    pub fn is_empty(&self) -> bool { self.owners.is_empty() }
    pub fn advance(&mut self, grant: Grant) -> RetirementStep<V> {
        if grant.maximum_items == 0 || grant.maximum_bytes == 0 { return RetirementStep::Blocked; }
        let Some(owner) = self.owners.pop_front() else { return RetirementStep::Complete; };
        let mut bytes = 0;
        match owner {
            Owner::Node(node) => if let Some(node) = Arc::into_inner(node) {
                if let Some(left) = node.left { self.owners.push_front(Owner::Node(left)); }
                if let Some(right) = node.right { self.owners.push_front(Owner::Node(right)); }
                self.owners.push_front(Owner::Entry(node.entry));
            },
            Owner::Entry(entry) => if let Some(entry) = Arc::into_inner(entry) { self.owners.push_front(Owner::Key(entry.key)); self.owners.push_front(Owner::Value(entry.value)); },
            Owner::Key(key) => if let Some(key) = Arc::into_inner(key) { self.owners.push_front(Owner::Bytes(key.into_bytes())); },
            Owner::Value(value) => if let Some(value) = Arc::into_inner(value) { return RetirementStep::OwnedValue(value); },
            Owner::Bytes(mut value) => { bytes = grant.maximum_bytes.min(value.len()); value.truncate(value.len() - bytes); if !value.is_empty() { self.owners.push_front(Owner::Bytes(value)); } }
        }
        RetirementStep::Progress { released_items: 1, released_bytes: bytes }
    }
}

/// 🧊️ Cold-only convenience cleanup; never called by retained advance or close_step.
fn retire_cold<V>(mut retirement: Retirement<V>) {
    loop { match retirement.advance(Grant { maximum_items: 1, maximum_bytes: 4096 }) { RetirementStep::OwnedValue(value) => drop(value), RetirementStep::Complete => break, _ => {} } }
}

/// 🧊️ Cold-only convenience cleanup; explicit synchronous APIs cannot earn interactive credit.
fn close_cold<V>(cursor: &mut UpdateCursor<V>) {
    cursor.begin_close();
    loop { match cursor.close_step(Grant { maximum_items: 1, maximum_bytes: 4096 }) { RetirementStep::OwnedValue(value) => drop(value), RetirementStep::Complete => break, _ => {} } }
}
//#endregion 🧹️Retirement

//#region 🔀️Serde
/// 🧊️ Cold synchronous serialization; retained canonical encoding must use the borrowed iterator seam.
impl<V: serde::Serialize> serde::Serialize for OrderedMap<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?; for (key, value) in self.iter() { map.serialize_entry(key, value)?; } map.end()
    }
}
/// 🧊️ Cold synchronous decoding retires displaced and failed partial roots without claiming bounded work.
impl<'de, V: serde::Deserialize<'de>> serde::Deserialize<'de> for OrderedMap<V> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor<V>(std::marker::PhantomData<V>);
        impl<'de, V: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<V> {
            type Value = OrderedMap<V>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("an ordered string-keyed object") }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut map = OrderedMap::new();
                loop {
                    match access.next_entry::<String, V>() {
                        Ok(Some((key, value))) => { map.insert(key, value); }
                        Ok(None) => return Ok(map),
                        Err(error) => { retire_cold(map.retire()); return Err(error); }
                    }
                }
            }
        }
        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}
//#endregion 🔀️Serde

#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;
