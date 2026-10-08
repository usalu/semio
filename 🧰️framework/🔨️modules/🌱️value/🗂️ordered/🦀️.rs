//! 🗂️ Immutable ordered ownership with byte-resumable comparison, path copying, and final-owner retirement.

use std::cmp::Ordering;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use crate::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};

#[path = "🧺️set/🦀️.rs"]
pub mod set;
pub use set::OrderedSet;

#[path = "🔗️shared/🦀️.rs"]
mod shared;
pub use shared::{SharedOwner, SharedRelease};

//#region 🌳️PersistentNodes
type Root<V> = Option<Arc<Node<V>>>;
/// 📏️ Fixed metadata-visit bound for one rank-iterator item; no payload bytes are read by iteration.
pub const MAX_AVL_HEIGHT: usize = 2 * usize::BITS as usize;
struct Entry<V> { key: SharedOwner<String>, value: SharedOwner<V> }
struct Node<V> { entry: Arc<Entry<V>>, left: Root<V>, right: Root<V>, height: usize, len: usize }

impl<V> Clone for Entry<V> {
    fn clone(&self) -> Self { Self { key: self.key.clone(), value: self.value.clone() } }
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

const UPDATE_RETIRED_ROOTS:usize=MAX_AVL_HEIGHT*3+4;
struct InlineStack<T,const N:usize> {values:[Option<T>;N],length:usize}
impl<T,const N:usize> Default for InlineStack<T,N> {fn default()->Self{Self{values:std::array::from_fn(|_|None),length:0}}}
impl<T,const N:usize> InlineStack<T,N> {
    fn push_front(&mut self,value:T){assert!(self.length<N,"ordered inline metadata frontier exceeded its structural bound");self.values[self.length]=Some(value);self.length+=1;}
    fn pop_front(&mut self)->Option<T>{self.length=self.length.checked_sub(1)?;self.values[self.length].take()}
    fn front(&self)->Option<&T>{self.length.checked_sub(1).and_then(|index|self.values[index].as_ref())}
    fn is_empty(&self)->bool{self.length==0}
}

fn balanced<V>(entry: Arc<Entry<V>>, left: Root<V>, right: Root<V>, retired:&mut InlineStack<Arc<Node<V>>,UPDATE_RETIRED_ROOTS>) -> Arc<Node<V>> {
    if height(&left) > height(&right) + 1 {
        let left_root=left.unwrap();let branch=&left_root;
        if height(&branch.left) >= height(&branch.right) {
            let right = node(entry, branch.right.clone(), right);
            let output=node(Arc::clone(&branch.entry), branch.left.clone(), Some(right));
            retired.push_front(left_root);return output;
        }
        let pivot = branch.right.as_ref().unwrap();
        let left = node(Arc::clone(&branch.entry), branch.left.clone(), pivot.left.clone());
        let right = node(entry, pivot.right.clone(), right);
        let output=node(Arc::clone(&pivot.entry), Some(left), Some(right));
        retired.push_front(left_root);return output;
    }
    if height(&right) > height(&left) + 1 {
        let right_root=right.unwrap();let branch=&right_root;
        if height(&branch.right) >= height(&branch.left) {
            let left = node(entry, left, branch.left.clone());
            let output=node(Arc::clone(&branch.entry), Some(left), branch.right.clone());
            retired.push_front(right_root);return output;
        }
        let pivot = branch.left.as_ref().unwrap();
        let left = node(entry, left, pivot.left.clone());
        let right = node(Arc::clone(&branch.entry), pivot.right.clone(), branch.right.clone());
        let output=node(Arc::clone(&pivot.entry), Some(left), Some(right));
        retired.push_front(right_root);return output;
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
    pub fn entry_at_rank(&self, index: usize) -> Option<(&String, &V)> { at(&self.root, index).map(|entry| (&*entry.key, &*entry.value)) }
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

    /// 🧊️ Cold input admission allocates two shared headers; retained callers use admitted begin_set_shared inputs.
    pub fn begin_set(&self, key: String, value: V) -> UpdateCursor<V> { self.begin_set_shared(SharedOwner::from_cold(key), SharedOwner::from_cold(value)) }
    /// 📥️ Begins a retained upsert by moving exactly two shared pointers; no key or value bytes are copied.
    pub fn begin_set_shared(&self, key: SharedOwner<String>, value: SharedOwner<V>) -> UpdateCursor<V> { UpdateCursor::new(self.clone(), key, Some(value)) }
    /// 🧊️ Cold key admission allocates a shared header; retained callers use begin_remove_shared.
    pub fn begin_remove(&self, key: String) -> UpdateCursor<V> { self.begin_remove_shared(SharedOwner::from_cold(key)) }
    /// 🗑️ Moves an exact shared key into retained removal without copying its bytes.
    pub fn begin_remove_shared(&self, key: SharedOwner<String>) -> UpdateCursor<V> { UpdateCursor::new(self.clone(), key, None) }
    /// 🔎️ Retains an immutable root and compares a lookup key under the caller's byte grants.
    pub fn begin_lookup(&self, key: String) -> LookupCursor<V> { self.begin_lookup_shared(SharedOwner::from_cold(key)) }
    /// 🔎️ Moves an exact shared key into retained lookup without copying its bytes.
    pub fn begin_lookup_shared(&self, key: SharedOwner<String>) -> LookupCursor<V> { LookupCursor::new(self.clone(), key) }

    /// ⚡️ Completes a synchronous convenience upsert; retained jobs must use begin_set and advance.
    pub fn insert(&mut self, key: String, value: V) -> Option<V> {
        let mut cursor = self.begin_set(key, value);
        while !cursor.is_complete() { cursor.advance(cold_update_grant(&cursor)).expect("ordered cold insert admission"); }
        let removed = cursor.take_removed(); let displaced = std::mem::replace(self, cursor.take_result().unwrap()); retire_cold(displaced.retire()); close_cold(&mut cursor); removed.and_then(release_cold)
    }
    /// ⚡️ Completes a synchronous convenience removal; retained jobs must use begin_remove and advance.
    pub fn remove(&mut self, key: &str) -> Option<V> {
        let mut cursor = self.begin_remove(key.to_owned());
        while !cursor.is_complete() { cursor.advance(cold_update_grant(&cursor)).expect("ordered cold removal admission"); }
        let removed = cursor.take_removed(); let displaced = std::mem::replace(self, cursor.take_result().unwrap()); retire_cold(displaced.retire()); close_cold(&mut cursor); removed.and_then(release_cold)
    }
    /// 🧹️ Moves this root into explicit final-owner retirement.
    pub fn retire(mut self) -> Retirement<V> { Retirement::new(self.root.take()) }
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
    if ordering.is_none() && *offset==key.len().min(other.len()) {*ordering=Some(key.len().cmp(&other.len()));}
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
    base: OrderedMap<V>, key: Option<SharedOwner<String>>, value: Option<SharedOwner<V>>, current: Root<V>, path: InlineStack<Parent<V>,MAX_AVL_HEIGHT>,
    successor_path: InlineStack<Arc<Node<V>>,MAX_AVL_HEIGHT>, retired:InlineStack<Arc<Node<V>>,UPDATE_RETIRED_ROOTS>, removed_node: Root<V>, successor_entry: Option<Arc<Entry<V>>>,
    replacement: Root<V>, removed: Option<SharedOwner<V>>, result: Option<OrderedMap<V>>, phase: Phase,
    offset: usize, left_byte: Option<u8>, ordering: Option<Ordering>, retirement: Retirement<V>, closing: bool,
}

impl<V> UpdateState<V> {
    fn new(base: OrderedMap<V>, key: SharedOwner<String>, value: Option<SharedOwner<V>>) -> Self {
        Self { current: (*base.root).clone(), base, key: Some(key), value, path: InlineStack::default(), successor_path: InlineStack::default(), retired:InlineStack::default(), removed_node: None,
            successor_entry: None, replacement: None, removed: None, result: None, phase: Phase::Search, offset: 0, left_byte: None, ordering: None, retirement: Retirement::default(), closing: false }
    }

    fn compare(&mut self, maximum_bytes: usize) -> usize {
        compare_bytes(self.key.as_ref().unwrap().as_bytes(), self.current.as_ref().unwrap().entry.key.as_bytes(), &mut self.offset, &mut self.left_byte, &mut self.ordering, maximum_bytes)
    }

    fn advance(&mut self, maximum_bytes:usize) -> Step {
        let mut bytes = 0;
        match self.phase {
            Phase::Search => {
                if self.current.is_none() {
                    self.replacement = self.value.as_ref().map(|value| node(Arc::new(Entry { key: self.key.as_ref().unwrap().clone(), value: value.clone() }), None, None));
                    self.phase = Phase::Rebuild;
                } else if self.ordering.is_none() { bytes = self.compare(maximum_bytes); }
                else {
                    let current = self.current.take().unwrap();
                    match self.ordering.take().unwrap() {
                        ordering @ (Ordering::Less | Ordering::Greater) => {
                            let direction = ordering == Ordering::Less;
                            self.current = if direction { current.left.clone() } else { current.right.clone() };
                            self.path.push_front(Parent { node: current, left: direction });
                        }
                        Ordering::Equal => {
                            self.removed = Some(current.entry.value.clone());
                            if let Some(value) = &self.value {
                                self.replacement = Some(node(Arc::new(Entry { key: self.key.as_ref().unwrap().clone(), value: value.clone() }), current.left.clone(), current.right.clone()));
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
                if let Some(parent) = self.successor_path.pop_front() { self.replacement = Some(balanced(Arc::clone(&parent.entry), self.replacement.take(), parent.right.clone(), &mut self.retired)); }
                else {
                    let removed = self.removed_node.take().unwrap();
                    self.replacement = Some(balanced(self.successor_entry.take().unwrap(), removed.left.clone(), self.replacement.take(), &mut self.retired)); self.phase = Phase::Rebuild;
                }
            }
            Phase::Rebuild => {
                if let Some(parent) = self.path.pop_front() {
                    self.replacement = Some(if parent.left { balanced(Arc::clone(&parent.node.entry), self.replacement.take(), parent.node.right.clone(), &mut self.retired) }
                        else { balanced(Arc::clone(&parent.node.entry), parent.node.left.clone(), self.replacement.take(), &mut self.retired) });
                } else { self.result = Some(OrderedMap { root: ManuallyDrop::new(self.replacement.take()) }); self.phase = Phase::Complete; }
            }
            Phase::Complete => return Step::Complete,
        }
        Step::Progress { completed_items: 1, completed_bytes: bytes }
    }

    pub fn is_complete(&self) -> bool { matches!(self.phase, Phase::Complete) }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.result.take() }
    pub fn take_removed(&mut self) -> Option<SharedOwner<V>> { self.removed.take() }
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        if self.terminal_is_empty(){return RetirementStep::Complete;}
        if !self.closing || grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { return self.retirement.advance(grant); }
        if grant.maximum_depth==0 {return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered close requires admitted metadata depth"));}
        if let Some(parent) = self.path.pop_front() { self.retirement.push(Owner::Node(parent.node)); }
        else if let Some(node) = self.successor_path.pop_front() { self.retirement.push(Owner::Node(node)); }
        else if let Some(node) = self.retired.pop_front() { self.retirement.push(Owner::Node(node)); }
        else if let Some(node) = self.current.take().or_else(|| self.removed_node.take()).or_else(|| self.replacement.take()).or_else(|| self.base.root.take()) { self.retirement.push(Owner::Node(node)); }
        else if let Some(mut map) = self.result.take() { if let Some(node) = map.root.take() { self.retirement.push(Owner::Node(node)); } }
        else if let Some(entry) = self.successor_entry.take() { self.retirement.push(Owner::Entry(entry)); }
        else if let Some(key) = self.key.take() { self.retirement.push(Owner::Key(key)); }
        else if let Some(value) = self.value.take().or_else(|| self.removed.take()) { self.retirement.push(Owner::Value(value)); }
        else { return RetirementStep::Complete; }
        RetirementStep::Progress { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_empty() && self.key.is_none() && self.value.is_none() && self.current.is_none() && self.path.is_empty()
            && self.successor_path.is_empty() && self.retired.is_empty() && self.removed_node.is_none() && self.successor_entry.is_none() && self.replacement.is_none()
            && self.removed.is_none() && self.result.is_none() && self.retirement.is_empty()
    }
    fn retained_depth(&self)->usize {
        self.path.length+self.successor_path.length+self.retired.length
            +usize::from(!self.base.is_empty())+usize::from(self.key.is_some())+usize::from(self.value.is_some())
            +usize::from(self.current.is_some())+usize::from(self.removed_node.is_some())+usize::from(self.successor_entry.is_some())
            +usize::from(self.replacement.is_some())+usize::from(self.removed.is_some())+usize::from(self.result.is_some())
    }
}

/// 🔒️ Terminal guard prevents unbudgeted destruction of any live cursor alias, including unwinding.
#[must_use = "update cursors must finish explicit close before drop"]
pub struct UpdateCursor<V> { state: ManuallyDrop<UpdateState<V>> }
impl<V> UpdateCursor<V> {
    fn new(base: OrderedMap<V>, key: SharedOwner<String>, value: Option<SharedOwner<V>>) -> Self { Self { state: ManuallyDrop::new(UpdateState::new(base, key, value)) } }
    pub fn next_copy_byte_demand(&self)->usize {
        if self.state.closing || self.state.is_complete(){return 0;}
        if !matches!(self.state.phase,Phase::Search)||self.state.ordering.is_some(){return 0;}
        self.state.current.as_ref().map_or(0,|current|usize::from(self.state.offset<self.state.key.as_ref().unwrap().len().min(current.entry.key.len())))
    }
    pub fn next_capacity_byte_demand(&self)->Result<usize,crate::ValueError> {
        if self.state.closing || self.state.is_complete(){return Ok(0);}
        let node_bytes=SharedOwner::<Node<V>>::allocation_bytes();
        let entry_bytes=SharedOwner::<Entry<V>>::allocation_bytes();
        let balanced_bytes = |left: &Root<V>, right: &Root<V>| {
            let nodes = if height(left) > height(right) + 1 { if height(&left.as_ref().unwrap().left) >= height(&left.as_ref().unwrap().right) { 2 } else { 3 } }
                else if height(right) > height(left) + 1 { if height(&right.as_ref().unwrap().right) >= height(&right.as_ref().unwrap().left) { 2 } else { 3 } } else { 1 };
            nodes * node_bytes
        };
        Ok(match self.state.phase {
            Phase::Search if self.state.current.is_none() => if self.state.value.is_some(){entry_bytes+node_bytes}else{0},
            Phase::Search => match self.state.ordering {
                Some(Ordering::Equal) if self.state.value.is_some() => entry_bytes+node_bytes,
                Some(_) => 0,
                None => 0,
            },
            Phase::Rebuild => self.state.path.front().map_or(0, |parent| if parent.left { balanced_bytes(&self.state.replacement, &parent.node.right) } else { balanced_bytes(&parent.node.left, &self.state.replacement) }),
            Phase::RebuildSuccessor=>self.state.successor_path.front().map_or_else(||balanced_bytes(&self.state.removed_node.as_ref().unwrap().left,&self.state.replacement),|parent|balanced_bytes(&self.state.replacement,&parent.right)),
            Phase::Successor|Phase::Complete => 0,
        })
    }
    pub fn next_depth_demand(&self)->usize {
        if self.state.closing||self.state.is_complete(){return 0;}
        let growth=match self.state.phase {
            Phase::Search=>match (&self.state.current,self.state.ordering){
                (None,_)=>usize::from(self.state.value.is_some()),
                (Some(current),Some(Ordering::Less))=>usize::from(current.left.is_some()),
                (Some(current),Some(Ordering::Greater))=>usize::from(current.right.is_some()),
                (Some(current),Some(Ordering::Equal))=>if self.state.value.is_some(){1}else if current.left.is_some()&&current.right.is_some(){2}else{usize::from(current.left.is_some()||current.right.is_some())},
                _=>0,
            },
            Phase::Successor=>self.state.current.as_ref().map_or(0,|current|usize::from(current.left.is_some()||current.right.is_some())),
            Phase::Rebuild|Phase::RebuildSuccessor=>{
                let sides=if matches!(self.state.phase,Phase::Rebuild){self.state.path.front().map(|parent|if parent.left{(&self.state.replacement,&parent.node.right)}else{(&parent.node.left,&self.state.replacement)})}else{self.state.successor_path.front().map(|parent|(&self.state.replacement,&parent.right))};
                usize::from(self.state.replacement.is_none()&&sides.is_some_and(|(left,right)|height(left).abs_diff(height(right))>1))
            },
            Phase::Complete=>0,
        };
        self.state.path.length+self.state.successor_path.length+self.state.retired.length
            +usize::from(!self.state.base.is_empty())+usize::from(self.state.key.is_some())+usize::from(self.state.value.is_some())
            +usize::from(self.state.current.is_some())+usize::from(self.state.removed_node.is_some())+usize::from(self.state.successor_entry.is_some())
            +usize::from(self.state.replacement.is_some())+usize::from(self.state.removed.is_some())+growth
    }
    /// 🎟️ Admits one real comparison or structural event; births are exact and obsolete roots remain in close custody.
    pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError> {
        if self.state.closing{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));}
        if self.state.is_complete(){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));}
        let capacity=self.next_capacity_byte_demand()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<self.next_copy_byte_demand()||grant.maximum_capacity_bytes<capacity{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));}
        if grant.maximum_depth<self.next_depth_demand(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered update exceeds admitted metadata depth"));}
        let progress=match self.state.advance(grant.maximum_copy_bytes){Step::Progress {completed_items,completed_bytes}=>RetainedCloneProgress {copied_items:completed_items,copied_bytes:completed_bytes,retained_capacity_bytes:capacity,released_bytes:0},Step::Complete=>RetainedCloneProgress::default(),Step::Blocked=>unreachable!()};
        Ok(if self.state.is_complete(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    /// 🛬️ The synchronous native collector additionally charges its cumulative ownership and cancellation authority before the same advance.
    pub fn advance_insert_controlled(&mut self, grant: RetainedCloneGrant, control: &mut crate::NativeDecodeControl<'_>) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.state.value.is_none(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"controlled native ordered update requires an insert"));}
        if self.state.closing||self.state.is_complete()||grant.maximum_items==0||grant.maximum_copy_bytes<self.next_copy_byte_demand()||grant.maximum_capacity_bytes<self.next_capacity_byte_demand()?{return self.advance(grant);}
        if grant.maximum_depth<self.next_depth_demand(){return self.advance(grant);}
        control.checkpoint()?;
        control.charge(self.next_capacity_byte_demand()?)?;
        self.advance(grant)
    }
    pub fn is_complete(&self) -> bool { self.state.is_complete() }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.state.take_result() }
    /// 📤️ Explicit shared-value handoff; the recipient owns its eventual domain retirement.
    pub fn take_removed(&mut self) -> Option<SharedOwner<V>> { self.state.take_removed() }
    pub fn begin_close(&mut self) { self.state.begin_close(); }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> { self.state.close_step(grant) }
    pub fn next_close_byte_demand(&self)->Result<usize,crate::ValueError>{self.state.retirement.next_close_byte_demand()}
    pub fn next_close_copy_byte_demand(&self)->usize{self.state.retirement.next_copy_byte_demand()}
    pub fn next_close_depth_demand(&self)->usize{self.state.retained_depth()+self.state.retirement.next_depth_demand()}
    pub fn terminal_is_empty(&self) -> bool { self.state.terminal_is_empty() }
}

fn cold_update_grant<V>(cursor:&UpdateCursor<V>)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:cursor.next_capacity_byte_demand().expect("ordered cold update birth demand"),maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()}}
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
    base: OrderedMap<V>, key: Option<SharedOwner<String>>, current: Root<V>, offset: usize, left_byte: Option<u8>, ordering: Option<Ordering>,
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

    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        if self.terminal_is_empty(){return RetirementStep::Complete;}
        if !self.closing || grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { return self.retirement.advance(grant); }
        if grant.maximum_depth==0{return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered close requires admitted metadata depth"));}
        if let Some(node) = self.current.take().or_else(|| self.base.root.take()) { self.retirement.push(Owner::Node(node)); }
        else if let Some(key) = self.key.take() { self.retirement.push(Owner::Key(key)); }
        else { return RetirementStep::Complete; }
        RetirementStep::Progress { released_items: 1, released_bytes: 0 }
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_empty() && self.current.is_none() && self.key.is_none() && self.retirement.is_empty() }
    fn retained_depth(&self)->usize{usize::from(!self.base.is_empty())+usize::from(self.current.is_some())+usize::from(self.key.is_some())}
}

/// 🔎️ Borrowed lookup result stays rooted until this cursor's explicit close; no payload clone.
#[must_use = "lookup cursors must finish explicit close before drop"]
pub struct LookupCursor<V> { state: ManuallyDrop<LookupState<V>> }
impl<V> LookupCursor<V> {
    fn new(base: OrderedMap<V>, key: SharedOwner<String>) -> Self {
        let current = (*base.root).clone();
        Self { state: ManuallyDrop::new(LookupState { base, key: Some(key), current, offset: 0, left_byte: None, ordering: None, complete: false, closing: false, retirement: Retirement::default() }) }
    }
    pub fn advance(&mut self, grant: Grant) -> Step { self.state.advance(grant) }
    pub fn is_complete(&self) -> bool { self.state.complete }
    pub fn result(&self) -> Option<&V> {
        if !self.state.complete || self.state.ordering != Some(Ordering::Equal) { return None; }
        self.state.current.as_ref().map(|node| &*node.entry.value)
    }
    pub fn begin_close(&mut self) { self.state.closing = true; }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> { self.state.close_step(grant) }
    pub fn next_close_byte_demand(&self)->Result<usize,crate::ValueError>{self.state.retirement.next_close_byte_demand()}
    pub fn next_close_copy_byte_demand(&self)->usize{self.state.retirement.next_copy_byte_demand()}
    pub fn next_close_depth_demand(&self)->usize{self.state.retained_depth()+self.state.retirement.next_depth_demand()}
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
enum Owner<V> {
    Node(Arc<Node<V>>), Entry(Arc<Entry<V>>), Key(SharedOwner<String>), Value(SharedOwner<V>),
}
/// 📤️ OwnedValue transfers final payload ownership to the caller's domain retirement cursor.
pub enum RetirementStep<V> { Blocked, Progress { released_items: usize, released_bytes: usize }, ProcessedBytes(usize), OwnedValue(V), Failure(crate::ValueError), Complete }
#[must_use = "retirement owners must be drained before drop"]
pub struct Retirement<V> {
    owners: ManuallyDrop<[Option<Owner<V>>; MAX_AVL_HEIGHT + 3]>,
    payload:ManuallyDrop<Option<V>>,
    key_bytes:ManuallyDrop<Option<Vec<u8>>>,
    length: usize,
}
impl<V> Default for Retirement<V> {
    fn default() -> Self { Self { owners: ManuallyDrop::new(std::array::from_fn(|_| None)), payload:ManuallyDrop::new(None), key_bytes:ManuallyDrop::new(None), length: 0 } }
}
impl<V> Drop for Retirement<V> {
    fn drop(&mut self) {
        if !self.is_empty() {
            if !std::thread::panicking() { panic!("ordered-map retirement must be empty before drop"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owners);ManuallyDrop::drop(&mut self.payload);ManuallyDrop::drop(&mut self.key_bytes); }
    }
}
impl<V> Retirement<V> {
    fn new(root: Root<V>) -> Self { let mut owner = Self::default(); if let Some(root) = root { owner.push(Owner::Node(root)); } owner }
    fn push(&mut self, owner: Owner<V>) {
        assert!(self.length < self.owners.len(), "ordered-map inline retirement frontier exceeded its AVL-height proof");
        self.owners[self.length] = Some(owner);
        self.length += 1;
    }
    fn pop(&mut self) -> Option<Owner<V>> {
        self.length = self.length.checked_sub(1)?;
        self.owners[self.length].take()
    }
    pub fn is_empty(&self) -> bool { self.length == 0 && self.payload.is_none() && self.key_bytes.is_none() }
    pub fn terminal_is_empty(&self) -> bool { self.is_empty() }
    pub fn allocated_bytes(&self) -> usize {
        self.key_bytes.as_ref().map_or(0,Vec::capacity)
    }
    pub fn next_depth_demand(&self)->usize {
        if self.payload.is_some()||self.key_bytes.is_some(){return self.retained_depth();}
        match self.length.checked_sub(1).and_then(|index|self.owners[index].as_ref()) {
            Some(Owner::Node(node))=>self.length-1+usize::from(node.left.is_some())+usize::from(node.right.is_some())+1,
            Some(Owner::Entry(_))=>self.length+1,
            Some(_)=>self.length,
            None=>0,
        }
    }
    fn retained_depth(&self)->usize{self.length+usize::from(self.payload.is_some())+usize::from(self.key_bytes.is_some())}
    pub fn next_close_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if self.payload.is_some(){return Ok(0);}
        if let Some(bytes)=self.key_bytes.as_ref(){return Ok(if bytes.is_empty(){bytes.capacity()}else{0});}
        Ok(match self.length.checked_sub(1).and_then(|index| self.owners[index].as_ref()) {
            Some(Owner::Node(_))=>SharedOwner::<Node<V>>::allocation_bytes(),
            Some(Owner::Entry(_))=>SharedOwner::<Entry<V>>::allocation_bytes(),
            Some(Owner::Key(key))=>key.next_release_byte_demand(),
            Some(Owner::Value(value))=>value.next_release_byte_demand(),
            None => 0,
        })
    }
    pub fn next_copy_byte_demand(&self)->usize {usize::from(self.key_bytes.as_ref().is_some_and(|bytes|!bytes.is_empty()))}
    pub fn advance(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        if self.is_empty(){return RetirementStep::Complete;}
        if grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if grant.maximum_depth<self.next_depth_demand(){return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered retirement frontier exceeds admitted depth"));}
        if let Some(values)=self.key_bytes.as_mut(){if !values.is_empty(){let bytes=values.len().min(grant.maximum_copy_bytes);if bytes==0{return RetirementStep::Blocked;}values.truncate(values.len()-bytes);return RetirementStep::ProcessedBytes(bytes);}}
        let demand=match self.next_close_byte_demand(){Ok(bytes)=>bytes,Err(error)=>return RetirementStep::Failure(error)};
        if demand>grant.maximum_release_bytes{return RetirementStep::Blocked;}
        if let Some(value)=self.payload.take(){return RetirementStep::OwnedValue(value);}
        if let Some(values)=self.key_bytes.take(){let bytes=values.capacity();drop(values);return RetirementStep::Progress {released_items:1,released_bytes:bytes};}
        let Some(owner) = self.pop() else { return RetirementStep::Complete; };
        let mut bytes = 0;
        match owner {
            Owner::Node(node) => if let Some(node) = Arc::into_inner(node) {
                bytes=demand;
                if let Some(left) = node.left { self.push(Owner::Node(left)); }
                if let Some(right) = node.right { self.push(Owner::Node(right)); }
                self.push(Owner::Entry(node.entry));
            },
            Owner::Entry(entry) => if let Some(entry) = Arc::into_inner(entry) {bytes=demand;self.push(Owner::Key(entry.key)); self.push(Owner::Value(entry.value)); },
            Owner::Key(mut key)=>match key.release_step(grant){Ok(step)=>{bytes=step.progress.released_bytes;*self.key_bytes=step.value.map(String::into_bytes);},Err(error)=>{self.push(Owner::Key(key));return RetirementStep::Failure(error);}},
            Owner::Value(mut value)=>match value.release_step(grant){Ok(step)=>{bytes=step.progress.released_bytes;*self.payload=step.value;},Err(error)=>{self.push(Owner::Value(value));return RetirementStep::Failure(error);}},
        }
        RetirementStep::Progress { released_items: 1, released_bytes: bytes }
    }
}

/// 🧊️ Cold-only convenience cleanup; never called by retained advance or close_step.
fn retire_cold<V>(mut retirement: Retirement<V>) {
    loop {
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:retirement.next_close_byte_demand().expect("finite ordered cold release demand"),maximum_depth:retirement.next_depth_demand()};
        match retirement.advance(grant) { RetirementStep::OwnedValue(value) => drop(value), RetirementStep::Complete => break, RetirementStep::Failure(error)=>panic!("ordered cold retirement refused: {error}"), _ => {} }
    }
}

/// 🧊️ Cold-only convenience cleanup; explicit synchronous APIs cannot earn interactive credit.
fn close_cold<V>(cursor: &mut UpdateCursor<V>) {
    cursor.begin_close();
    loop {let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().expect("ordered cold close release demand"),maximum_depth:cursor.next_close_depth_demand()};match cursor.close_step(grant) { RetirementStep::OwnedValue(value) => drop(value), RetirementStep::Complete => break, RetirementStep::Failure(error)=>panic!("ordered cold close refused: {error}"), _ => {} } }
}

fn release_cold<T>(mut value:SharedOwner<T>)->Option<T>{value.release_step(RetainedCloneGrant::one_release_turn(value.next_release_byte_demand(),1)).expect("cold shared release").value}
//#endregion 🧹️Retirement

#[path = "♻️retirement/🦀️.rs"]
mod typed_retirement;

//#region 🔀️Serde
/// 🧊️ `#[cfg(test)]`-only (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01,
/// tenth-seam pass). The three production sites that previously required this — `Dictionary`'s
/// `#[serde(transparent)]` derive over `pairs: OrderedMap<Value>`, `Value`'s derive (has a
/// `Dictionary` variant), `Neuron`'s derive (holds a `Value` via `Dictionary`) — all moved to
/// `ToValue`/`FromValue` in `💻️os/🧠️neural/⚙️engine/🦀️.rs`, including their own fan-out
/// (`FieldSpec`, `Schema`, `ChannelSpec`, `OperatorInfo`, `Tree`). See
/// `📓️orderedmap-tenth-seam.md` for the compiler-enumerated consumer list. The `Deserialize`
/// sibling below stays test-only, unchanged — nothing in production deserializes an `OrderedMap`.
#[cfg(test)]
impl<V: serde::Serialize> serde::Serialize for OrderedMap<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?; for (key, value) in self.iter() { map.serialize_entry(key, value)?; } map.end()
    }
}
/// 🧊️ `#[cfg(test)]`-only — see `Serialize` above for why.
#[cfg(test)]
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

//#region 🔁️Value
/// 🌱️ First-party replacement for the `#[cfg(test)]`-only `Serialize`/`Deserialize` above — see
/// that region's doc. Written against `super::ToValue`/`FromValue` (a RELATIVE path) rather than
/// `#[derive(ToValue, FromValue)]`, because this component is mounted by BOTH `os-kernel` (where
/// the derive's hard-literal `::semio_framework_os_kernel::…` path would resolve) and
/// `replication` (where it would not — `replication` sits below `os-kernel` in the DAG); a
/// relative path resolves correctly under either mount point.
#[path = "🚦️native/🦀️.rs"]
mod native_controlled;

impl<V: super::ToValue> super::ToValue for OrderedMap<V> {
    fn to_value_controlled(&self, control: &mut crate::NativeEncodeControl<'_>) -> Result<crate::DslValue, crate::ValueError> { native_controlled::encode(self, control) }
    fn to_value(&self) -> super::DslValue {
        super::DslValue::Object(self.iter().map(|(key, value)| (key.clone(), super::ToValue::to_value(value))).collect())
    }
}
impl<V: super::FromValue> super::FromValue for OrderedMap<V> {
    fn from_value_controlled(value: &crate::DslValue, control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, crate::ValueError> { native_controlled::decode(value, control) }
    fn default_value_controlled(control: &mut crate::NativeDecodeControl<'_>) -> Result<Self, crate::ValueError> { native_controlled::empty(control) }
    fn retire_decoded(self) { native_controlled::retire(self) }
    fn from_value(value: super::DslValue) -> Result<Self, super::ValueError> {
        let super::DslValue::Object(fields) = value else {
            return Err(super::ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an ordered string-keyed object, found {value:?}")));
        };
        let mut map = OrderedMap::new();
        for (key, entry) in fields {
            let decoded = <V as super::FromValue>::from_value(entry).map_err(|error| error.under(key.clone()))?;
            map.insert(key, decoded);
        }
        Ok(map)
    }
}
//#endregion 🔁️Value

#[cfg(test)]
#[path = "🧪️tests/🗂️ordered/🦀️.rs"]
mod tests;
