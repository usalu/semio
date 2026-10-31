//! 🗂️ Immutable ordered ownership with byte-resumable comparison, path copying, and final-owner retirement.

use std::cmp::Ordering;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use crate::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};

#[path = "🧺️set/🦀️.rs"]
pub mod set;
pub use set::OrderedSet;

#[path = "🔗️immutable-alias/🦀️.rs"]
mod immutable_alias;
pub use immutable_alias::{SharedOwner, SharedRelease};

//#region 🌳️PersistentNodes
type Root<V> = Option<Arc<Node<V>>>;
#[path="🎮️update/🎬️plan/🦀️.rs"]mod update_plan;
use update_plan::{BalancePlan,EntryBuilder,NodeBuilder};
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
/// 🗺️ Ordered immutable root; clones share payloads and every nonempty owner must be explicitly retired.
/// 🔒️ Dropping live ownership panics without destroying payloads; unwinding preserves it without a second panic.
#[must_use = "ordered roots must be transferred or explicitly retired"]
pub struct OrderedMap<V> { root: ManuallyDrop<Root<V>> }

impl<V> Default for OrderedMap<V> { fn default() -> Self { Self { root: ManuallyDrop::new(None) } } }
impl<V> Clone for OrderedMap<V> { fn clone(&self) -> Self { Self { root: self.root.clone() } } }
impl<V:crate::retirement::RetireOwned+Sync> semio_framework_value_retained_clone::RetainedAlias for OrderedMap<V>{
    fn alias_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*std::mem::size_of::<Self>(),depth:1,..Default::default()}}
    fn admit_alias(&self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress)>{let mut work=0;let result=(||{let d=semio_framework_value_retained_clone::RetainedAlias::alias_demand(self);if !crate::retained_clone::retained_clone_grant_admits(grant,d){return Err(crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original ordered alias requires its complete header quote"))}work=1;Ok((self.clone(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}
}
impl<V:crate::retirement::RetireOwned+Sync> semio_framework_value_retained_clone::RetainedClone for OrderedMap<V>{type Cursor=semio_framework_value_retained_clone::RetainedAliasCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl<V> Drop for OrderedMap<V> {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.root.is_none(), "ordered-map root must be explicitly retired before drop"); } }
}

impl<V> OrderedMap<V> {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { len(&self.root) }
    pub fn is_empty(&self) -> bool { self.root.is_none() }
    /// 🪪️ Borrows the original immutable node identity without issuing another ownership alias.
    pub fn original_root_identity(&self)->Option<usize>{self.root.as_ref().map(|root|Arc::as_ptr(root)as usize)}
    pub fn iter(&self) -> Iter<'_, V> { Iter { root: &self.root, front: 0, back: self.len() } }
    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &String> + ExactSizeIterator { self.iter().map(|(key, _)| key) }
    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> + ExactSizeIterator { self.iter().map(|(_, value)| value) }
    pub fn first_key_value(&self) -> Option<(&String, &V)> { self.iter().next() }
    /// 📍️ Borrows one ranked entry with at most MAX_AVL_HEIGHT metadata visits; charge one retained item.
    pub fn entry_at_rank(&self, index: usize) -> Option<(&String, &V)> { at(&self.root, index).map(|entry| (&*entry.key, &*entry.value)) }
    /// 🔗️ Retains the same two immutable entry allocations after one bounded ranked metadata visit.
    pub fn entry_shared_at_rank(&self,index:usize)->Option<(SharedOwner<String>,SharedOwner<V>)>{at(&self.root,index).map(|entry|(entry.key.clone(),entry.value.clone()))}
    /// 📎️ Retains only the exact immutable value after one bounded ranked metadata visit.
    pub fn value_shared_at_rank(&self,index:usize)->Option<SharedOwner<V>>{at(&self.root,index).map(|entry|entry.value.clone())}
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
    /// 🗝️ Cold synchronous membership; no interactive accounting is provided.
    pub fn contains_key(&self, key: &str) -> bool { self.get(key).is_some() }

    /// ▶️ Cold input admission allocates two shared headers; retained callers use admitted begin_set_shared inputs.
    pub fn begin_set(&self, key: String, value: V) -> UpdateCursor<V> { self.begin_set_shared(SharedOwner::from_cold(key), SharedOwner::from_cold(value)) }
    /// 📥️ Begins a retained upsert by moving exactly two shared pointers; no key or value bytes are copied.
    pub fn begin_set_shared(&self, key: SharedOwner<String>, value: SharedOwner<V>) -> UpdateCursor<V> { UpdateCursor::new(self.clone(), key, Some(value)) }
    /// ➖️ Cold key admission allocates a shared header; retained callers use begin_remove_shared.
    pub fn begin_remove(&self, key: String) -> UpdateCursor<V> { self.begin_remove_shared(SharedOwner::from_cold(key)) }
    /// 🗑️ Moves an exact shared key into retained removal without copying its bytes.
    pub fn begin_remove_shared(&self, key: SharedOwner<String>) -> UpdateCursor<V> { UpdateCursor::new(self.clone(), key, None) }
    /// 🔎️ Retains an immutable root and compares a lookup key under the caller's byte grants.
    pub fn begin_lookup(&self, key: String) -> LookupCursor<V> { self.begin_lookup_shared(SharedOwner::from_cold(key)) }
    /// 🟢️ Moves an exact shared key into retained lookup without copying its bytes.
    pub fn begin_lookup_shared(&self, key: SharedOwner<String>) -> LookupCursor<V> { LookupCursor::new(self.clone(), key) }

    /// ⚡️ Completes a synchronous convenience upsert; retained jobs must use begin_set and advance.
    pub fn insert(&mut self, key: String, value: V) -> Option<V> {
        let mut cursor = self.begin_set(key, value);
        while !cursor.is_complete() { cursor.advance(cold_update_grant(&cursor)).result.expect("ordered cold insert admission"); }
        let removed = cursor.take_removed(); let displaced = std::mem::replace(self, cursor.take_result().unwrap()); retire_cold(displaced.retire()); close_cold(&mut cursor); removed.and_then(release_cold)
    }
    /// 🏪️ Completes a synchronous convenience removal; retained jobs must use begin_remove and advance.
    /// `Some` states that the key WAS present, whatever else still shares its payload: a clone of this map keeps the payload
    /// alive, so only [`Removal::Owned`] hands the value over and [`Removal::Shared`] reports that another owner retains it.
    pub fn remove(&mut self, key: &str) -> Option<Removal<V>> {
        let mut cursor = self.begin_remove(key.to_owned());
        while !cursor.is_complete() { cursor.advance(cold_update_grant(&cursor)).result.expect("ordered cold removal admission"); }
        let removed = cursor.take_removed(); let displaced = std::mem::replace(self, cursor.take_result().unwrap()); retire_cold(displaced.retire()); close_cold(&mut cursor);
        removed.map(|shared| release_cold(shared).map_or(Removal::Shared, Removal::Owned))
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
/// 🚧️ Cold synchronous construction explicitly drains displaced roots; it provides no interactive credit.
impl<V> FromIterator<(String, V)> for OrderedMap<V> {
    fn from_iter<T: IntoIterator<Item = (String, V)>>(entries: T) -> Self { let mut map = Self::new(); for (key, value) in entries { map.insert(key, value); } map }
}
impl<V, const N: usize> From<[(String, V); N]> for OrderedMap<V> { fn from(entries: [(String, V); N]) -> Self { entries.into_iter().collect() } }
//#endregion 🗂️Map

/// ✅️ What a completed [`OrderedMap::remove`] took out of the map: the key was present either way.
#[derive(Debug, PartialEq, Eq)]
pub enum Removal<V> {
    /// 👪️ The payload had no other owner, so its value moves to the caller (who retires it if it owns a fail-closed root).
    Owned(V),
    /// 🌐️ Another clone of the map still shares the payload; the caller has nothing to retire.
    Shared,
}
impl<V> Removal<V> {
    /// 🎁️ The value when this removal held the last owner.
    pub fn into_owned(self) -> Option<V> { match self { Self::Owned(value) => Some(value), Self::Shared => None } }
}

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

#[derive(Clone,Copy)]
enum Phase { Search, Successor, RebuildSuccessor, Rebuild, Complete }
struct Parent<V> { node: Arc<Node<V>>, left: bool }

/// 🧵️ Retains both immutable roots until explicit result handoff and close; no payload Clone bound.
struct UpdateState<V> {
    base: OrderedMap<V>, key: Option<SharedOwner<String>>, value: Option<SharedOwner<V>>, current: Root<V>, path: InlineStack<Parent<V>,MAX_AVL_HEIGHT>,
    successor_path: InlineStack<Arc<Node<V>>,MAX_AVL_HEIGHT>, retired:InlineStack<Arc<Node<V>>,UPDATE_RETIRED_ROOTS>, removed_node: Root<V>, successor_entry: Option<Arc<Entry<V>>>,
    replacement: Root<V>, removed: Option<SharedOwner<V>>, result: Option<OrderedMap<V>>, phase: Phase, insert:bool,
    offset: usize, left_byte: Option<u8>, ordering: Option<Ordering>, retirement: Retirement<V>, closing: bool, last_close_work:u64, last_compared_bytes:usize,pending_entry:Option<EntryBuilder<V>>,entry_arc:Option<Arc<Entry<V>>>,pending_node:Option<NodeBuilder<V>>,plan:Option<BalancePlan<V>>,
}

impl<V> UpdateState<V> {
    fn new(base: OrderedMap<V>, key: SharedOwner<String>, value: Option<SharedOwner<V>>) -> Self {
        let insert=value.is_some();
        Self { insert,current: (*base.root).clone(), base, key: Some(key), value, path: InlineStack::default(), successor_path: InlineStack::default(), retired:InlineStack::default(), removed_node: None,
            successor_entry: None, replacement: None, removed: None, result: None, phase: Phase::Search, offset: 0, left_byte: None, ordering: None, retirement: Retirement::default(), closing: false, last_close_work:0,last_compared_bytes:0,pending_entry:None,entry_arc:None,pending_node:None,plan:None }
    }

    fn compare(&mut self, maximum_bytes: usize) -> usize {
        compare_bytes(self.key.as_ref().unwrap().as_bytes(), self.current.as_ref().unwrap().entry.key.as_bytes(), &mut self.offset, &mut self.left_byte, &mut self.ordering, maximum_bytes)
    }

    fn admission_growth(&self)->usize{
        if self.pending_entry.is_some()||self.pending_node.is_some(){return 1}if self.plan.is_some(){return 0}
        match self.phase{
            Phase::Search=>if self.current.is_none(){if self.value.is_some()&&self.entry_arc.is_none(){2}else{0}}else{let current=self.current.as_ref().unwrap();match self.ordering{None=>0,Some(Ordering::Less)=>usize::from(current.left.is_some()),Some(Ordering::Greater)=>usize::from(current.right.is_some()),Some(Ordering::Equal)=>if self.value.is_some(){if self.entry_arc.is_none(){2}else{1+usize::from(current.left.is_some())+usize::from(current.right.is_some())}}else{1+usize::from(current.left.is_some()||current.right.is_some())}}},
            Phase::Successor=>{let current=self.current.as_ref().unwrap();if current.left.is_some(){1}else{1+usize::from(current.right.is_some())}},
            Phase::Rebuild=>self.path.front().map_or(usize::from(self.replacement.is_none()),|parent|{let(left,right,sibling)=if parent.left{(&self.replacement,&parent.node.right,usize::from(parent.node.right.is_some()))}else{(&parent.node.left,&self.replacement,usize::from(parent.node.left.is_some()))};1+sibling+BalancePlan::new_aliases(left,right)}),
            Phase::RebuildSuccessor=>self.successor_path.front().map_or_else(||{let left=&self.removed_node.as_ref().unwrap().left;usize::from(left.is_some())+BalancePlan::new_aliases(left,&self.replacement)},|parent|1+usize::from(parent.right.is_some())+BalancePlan::new_aliases(&self.replacement,&parent.right)),Phase::Complete=>0,
        }
    }
    fn advance(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{
        self.last_compared_bytes=0;
        if let Some(staged)=self.pending_entry.as_mut(){let value=Entry{key:staged.key.take().unwrap(),value:staged.value.take().unwrap()};let turn=crate::retirement::arc_custody::admit_shared_allocation(value,grant);let result=match turn.result{Ok((entry,p))=>{self.entry_arc=Some(entry);self.pending_entry.take();Ok(RetainedCloneStep::Progress(p))},Err((error,value))=>{staged.key=Some(value.key);staged.value=Some(value.value);Err(error)}};return crate::RetainedWorkOutcome{result,work:turn.work}}
        if let Some(staged)=self.pending_node.as_mut(){let value=staged.take_node();let turn=crate::retirement::arc_custody::admit_shared_allocation(value,grant);let result=match turn.result{Ok((node,p))=>{self.plan.as_mut().unwrap().accept(node);self.pending_node.take();Ok(RetainedCloneStep::Progress(p))},Err((error,node))=>{staged.restore(node);Err(error)}};return crate::RetainedWorkOutcome{result,work:turn.work}}
        if let Some(plan)=self.plan.as_mut(){if plan.ready(){let root=plan.take_result();let after=plan.after;self.plan.take();self.replacement=root;self.phase=after;}else{self.pending_node=Some(plan.prepare());if let Err(error)=self.pending_node.as_mut().unwrap().validate(){return crate::RetainedWorkOutcome{result:Err(error.with_retained_progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}}return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
        match self.phase{
            Phase::Search=>{
                if self.current.is_none(){if let Some(value)=self.value.as_ref(){if let Some(entry)=self.entry_arc.take(){self.plan=Some(BalancePlan::single(entry,None,None,Phase::Rebuild));}else{self.pending_entry=Some(EntryBuilder{key:Some(self.key.as_ref().unwrap().clone()),value:Some(value.clone())});}}else{self.phase=Phase::Rebuild;}}
                else if self.ordering.is_none(){self.last_compared_bytes=self.compare(64);}
                else if self.ordering==Some(Ordering::Equal)&&self.value.is_some()&&self.entry_arc.is_none(){self.pending_entry=Some(EntryBuilder{key:Some(self.key.as_ref().unwrap().clone()),value:Some(self.value.as_ref().unwrap().clone())});}
                else{let current=self.current.take().unwrap();match self.ordering.take().unwrap(){direction @ (Ordering::Less|Ordering::Greater)=>{let left=direction==Ordering::Less;self.current=if left{current.left.clone()}else{current.right.clone()};self.path.push_front(Parent{node:current,left});},Ordering::Equal=>{self.removed=Some(current.entry.value.clone());if self.value.is_some(){self.plan=Some(BalancePlan::single(self.entry_arc.take().unwrap(),current.left.clone(),current.right.clone(),Phase::Rebuild));self.retired.push_front(current);}else if current.left.is_none()||current.right.is_none(){self.replacement=current.left.clone().or_else(||current.right.clone());self.retired.push_front(current);self.phase=Phase::Rebuild;}else{self.current=current.right.clone();self.removed_node=Some(current);self.phase=Phase::Successor;}}}self.offset=0;self.left_byte=None;}
            },
            Phase::Successor=>{let current=self.current.take().unwrap();if current.left.is_some(){self.current=current.left.clone();self.successor_path.push_front(current);}else{self.successor_entry=Some(Arc::clone(&current.entry));self.replacement=current.right.clone();self.retired.push_front(current);self.phase=Phase::RebuildSuccessor;}},
            Phase::RebuildSuccessor=>{if let Some(parent)=self.successor_path.pop_front(){self.plan=Some(BalancePlan::balanced(Arc::clone(&parent.entry),self.replacement.take(),parent.right.clone(),Phase::RebuildSuccessor,&mut self.retired));self.retired.push_front(parent);}else{let removed=self.removed_node.take().unwrap();self.plan=Some(BalancePlan::balanced(self.successor_entry.take().unwrap(),removed.left.clone(),self.replacement.take(),Phase::Rebuild,&mut self.retired));self.retired.push_front(removed);}},
            Phase::Rebuild=>{if let Some(parent)=self.path.pop_front(){let left=if parent.left{self.replacement.take()}else{parent.node.left.clone()};let right=if parent.left{parent.node.right.clone()}else{self.replacement.take()};self.plan=Some(BalancePlan::balanced(Arc::clone(&parent.node.entry),left,right,Phase::Rebuild,&mut self.retired));self.retired.push_front(parent.node);}else{self.result=Some(OrderedMap{root:ManuallyDrop::new(self.replacement.take())});self.phase=Phase::Complete;}},
            Phase::Complete=>return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0},
        }
        crate::RetainedWorkOutcome{result:Ok(if self.is_complete(){RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}),work:1}
    }

    pub fn is_complete(&self) -> bool { matches!(self.phase, Phase::Complete) }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.result.take() }
    pub fn take_removed(&mut self) -> Option<SharedOwner<V>> { self.removed.take() }
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        self.last_close_work=0;
        if self.terminal_is_empty(){return RetirementStep::Complete;}
        if !self.closing || grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { let step=self.retirement.advance(grant);self.last_close_work=self.retirement.last_turn_work();return step; }
        if grant.maximum_depth==0 {return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered close requires admitted metadata depth"));}
        self.last_close_work=1;
        if let Some(node)=self.pending_node.as_mut(){if let Some(owner)=node.take_owner(){self.retirement.push(owner);}else{self.pending_node.take();}return RetirementStep::Progress{released_items:1,released_bytes:0};}
        if let Some(entry)=self.pending_entry.as_mut(){if let Some(owner)=entry.take_owner(){self.retirement.push(owner);}else{self.pending_entry.take();}return RetirementStep::Progress{released_items:1,released_bytes:0};}
        if let Some(entry)=self.entry_arc.take(){self.retirement.push(Owner::Entry(entry));return RetirementStep::Progress{released_items:1,released_bytes:0};}
        if let Some(plan)=self.plan.as_mut(){if let Some(owner)=plan.take_owner(){self.retirement.push(owner);}else{self.plan.take();}return RetirementStep::Progress{released_items:1,released_bytes:0};}
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
        self.closing && self.pending_entry.is_none()&&self.entry_arc.is_none()&&self.pending_node.is_none()&&self.plan.is_none()&&self.base.is_empty() && self.key.is_none() && self.value.is_none() && self.current.is_none() && self.path.is_empty()
            && self.successor_path.is_empty() && self.retired.is_empty() && self.removed_node.is_none() && self.successor_entry.is_none() && self.replacement.is_none()
            && self.removed.is_none() && self.result.is_none() && self.retirement.is_empty()
    }
    fn retained_depth(&self)->usize {
        self.pending_entry.as_ref().map_or(0,EntryBuilder::held)+usize::from(self.entry_arc.is_some())+self.pending_node.as_ref().map_or(0,NodeBuilder::held)+self.plan.as_ref().map_or(0,BalancePlan::held)+self.path.length+self.successor_path.length+self.retired.length
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
    pub fn next_copy_byte_demand(&self)->usize{0}
    pub fn next_capacity_byte_demand(&self)->Result<usize,crate::ValueError>{if self.state.closing||self.state.is_complete(){return Ok(0)}Ok(if self.state.pending_entry.is_some(){crate::retirement::arc_custody::shared_allocation_bytes::<Entry<V>>()}else if self.state.pending_node.is_some(){crate::retirement::arc_custody::shared_allocation_bytes::<Node<V>>()}else{0})}
    pub fn next_depth_demand(&self)->usize{if self.state.closing||self.state.is_complete(){0}else{self.state.retained_depth().saturating_add(self.state.admission_growth()).max(1)}}
    pub fn last_compared_bytes(&self)->usize{self.state.last_compared_bytes}
    /// ⏩️ Every selected metadata probe or actual shared allocation is independently funded before effects.
    pub fn advance(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{
        self.state.last_compared_bytes=0;if self.state.closing{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if self.state.is_complete(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let capacity=match self.next_capacity_byte_demand(){Ok(capacity)=>capacity,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if grant.maximum_depth<self.next_depth_demand(){return crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered update exceeds admitted metadata depth")),work:0}}self.state.advance(grant)
    }
    /// 🛬️ Native receiving folds the actual child work and all physical refusals after retaining its real state.
    pub fn advance_insert_controlled(&mut self,grant:RetainedCloneGrant,control:&mut crate::NativeDecodeControl<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,semio_framework_value_native_decoding::native_decoding::OriginalReceivingPrimary>{
        if !self.state.insert{return crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"controlled native ordered update requires an insert").into()),work:0}}let capacity=match self.next_capacity_byte_demand(){Ok(bytes)=>bytes,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if self.state.closing||self.state.is_complete()||grant.maximum_items==0||grant.maximum_capacity_bytes<capacity||grant.maximum_depth<self.next_depth_demand(){let turn=self.advance(grant);return crate::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}if control.owned_bytes().checked_add(capacity).is_none_or(|next|next>control.maximum_bytes()){return crate::RetainedWorkOutcome{result:Err(crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native decoding ownership exceeds caller limit").into()),work:0}}if let Err(error)=control.checkpoint(){return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}let turn=self.advance(grant);control.receive_retained_work(grant,turn)
    }
    pub fn is_complete(&self) -> bool { self.state.is_complete() }
    pub fn take_result(&mut self) -> Option<OrderedMap<V>> { self.state.take_result() }
    /// 📤️ Explicit shared-value handoff; the recipient owns its eventual domain retirement.
    pub fn take_removed(&mut self) -> Option<SharedOwner<V>> { self.state.take_removed() }
    pub fn begin_close(&mut self) { self.state.begin_close(); }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> { self.state.close_step(grant) }
    pub fn last_close_work(&self)->u64{self.state.last_close_work}
    pub fn next_close_byte_demand(&self)->Result<usize,crate::ValueError>{self.state.retirement.next_close_byte_demand()}
    pub fn next_close_copy_byte_demand(&self)->usize{self.state.retirement.next_copy_byte_demand()}
    pub fn next_close_depth_demand(&self)->usize{(self.state.retained_depth()+self.state.retirement.next_depth_demand()).max(usize::from(!self.terminal_is_empty()))}
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
    complete: bool, closing: bool, retirement: Retirement<V>, last_close_work:u64,
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
        self.last_close_work=0;
        if self.terminal_is_empty(){return RetirementStep::Complete;}
        if !self.closing || grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { let step=self.retirement.advance(grant);self.last_close_work=self.retirement.last_turn_work();return step; }
        if grant.maximum_depth==0{return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered close requires admitted metadata depth"));}
        self.last_close_work=1;
        if let Some(node) = self.current.take().or_else(|| self.base.root.take()) { self.retirement.push(Owner::Node(node)); }
        else if let Some(key) = self.key.take() { self.retirement.push(Owner::Key(key)); }
        else { return RetirementStep::Complete; }
        RetirementStep::Progress { released_items: 1, released_bytes: 0 }
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_empty() && self.current.is_none() && self.key.is_none() && self.retirement.is_empty() }
    fn retained_depth(&self)->usize{usize::from(!self.base.is_empty())+usize::from(self.current.is_some())+usize::from(self.key.is_some())}
}

/// 🖱️ Borrowed lookup result stays rooted until this cursor's explicit close; no payload clone.
#[must_use = "lookup cursors must finish explicit close before drop"]
pub struct LookupCursor<V> { state: ManuallyDrop<LookupState<V>> }
impl<V> LookupCursor<V> {
    fn new(base: OrderedMap<V>, key: SharedOwner<String>) -> Self {
        let current = (*base.root).clone();
        Self { state: ManuallyDrop::new(LookupState { base, key: Some(key), current, offset: 0, left_byte: None, ordering: None, complete: false, closing: false, retirement: Retirement::default(), last_close_work:0 }) }
    }
    pub fn advance(&mut self, grant: Grant) -> Step { self.state.advance(grant) }
    pub fn is_complete(&self) -> bool { self.state.complete }
    pub fn result(&self) -> Option<&V> {
        if !self.state.complete || self.state.ordering != Some(Ordering::Equal) { return None; }
        self.state.current.as_ref().map(|node| &*node.entry.value)
    }
    pub fn begin_close(&mut self) { self.state.closing = true; }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> { self.state.close_step(grant) }
    pub fn last_close_work(&self)->u64{self.state.last_close_work}
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
/// 🪜️ OwnedValue transfers final payload ownership to the caller's domain retirement cursor.
pub enum RetirementStep<V> { Blocked, Progress { released_items: usize, released_bytes: usize }, ProcessedBytes(usize), OwnedValue(V), Failure(crate::ValueError), Complete }

#[must_use = "retirement owners must be drained before drop"]
pub struct Retirement<V> {
    last_work:u64,
    owners: ManuallyDrop<[Option<Owner<V>>; MAX_AVL_HEIGHT + 3]>,
    payload:ManuallyDrop<Option<V>>,
    key_bytes:ManuallyDrop<Option<Vec<u8>>>,
    length: usize,
}
impl<V> Default for Retirement<V> {
    fn default() -> Self { Self { last_work:0,owners: ManuallyDrop::new(std::array::from_fn(|_| None)), payload:ManuallyDrop::new(None), key_bytes:ManuallyDrop::new(None), length: 0 } }
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
    pub fn next_copy_byte_demand(&self)->usize {0}
    /// ⏱️ Reads actual work from this defining close producer, including child failure.
    pub fn last_turn_work(&self)->u64{self.last_work}
    pub fn advance(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        self.last_work=0;
        if self.is_empty(){return RetirementStep::Complete;}
        if grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if grant.maximum_depth<self.next_depth_demand(){return RetirementStep::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered retirement frontier exceeds admitted depth"));}
        if let Some(values)=self.key_bytes.as_mut(){if !values.is_empty(){let bytes=values.len();self.last_work=1;values.clear();return RetirementStep::ProcessedBytes(bytes);}}
        let demand=match self.next_close_byte_demand(){Ok(bytes)=>bytes,Err(error)=>return RetirementStep::Failure(error)};
        if demand>grant.maximum_release_bytes{return RetirementStep::Blocked;}
        if let Some(value)=self.payload.take(){self.last_work=1;return RetirementStep::OwnedValue(value);}
        if let Some(values)=self.key_bytes.take(){self.last_work=1;let bytes=values.capacity();drop(values);return RetirementStep::Progress {released_items:1,released_bytes:bytes};}
        if let Some(owner)=self.length.checked_sub(1).and_then(|index|self.owners[index].as_mut()){
            match owner{
                Owner::Key(key)=>{let turn=key.release_step(grant);self.last_work=turn.work;return match turn.result{Err(error)=>RetirementStep::Failure(error),Ok(step)if step.progress.copied_items==0=>RetirementStep::Blocked,Ok(step)=>{drop(self.pop());*self.key_bytes=step.value.map(String::into_bytes);RetirementStep::Progress{released_items:1,released_bytes:step.progress.released_bytes}}}},
                Owner::Value(value)=>{let turn=value.release_step(grant);self.last_work=turn.work;return match turn.result{Err(error)=>RetirementStep::Failure(error),Ok(step)if step.progress.copied_items==0=>RetirementStep::Blocked,Ok(step)=>{drop(self.pop());*self.payload=step.value;RetirementStep::Progress{released_items:1,released_bytes:step.progress.released_bytes}}}},
                _=>{}
            }
        }
        let Some(owner) = self.pop() else { return RetirementStep::Complete; };self.last_work=1;
        let mut bytes = 0;
        match owner {
            Owner::Node(node) => if let Some(node) = Arc::into_inner(node) {
                bytes=demand;
                if let Some(left) = node.left { self.push(Owner::Node(left)); }
                if let Some(right) = node.right { self.push(Owner::Node(right)); }
                self.push(Owner::Entry(node.entry));
            },
            Owner::Entry(entry) => if let Some(entry) = Arc::into_inner(entry) {bytes=demand;self.push(Owner::Key(entry.key)); self.push(Owner::Value(entry.value)); },
            Owner::Key(_)|Owner::Value(_)=>unreachable!("shared aliases release before their frontier slot transfers"),
        }
        RetirementStep::Progress { released_items: 1, released_bytes: bytes }
    }
}

/// ❄️ Cold-only convenience cleanup; never called by retained advance or close_step.
fn retire_cold<V>(mut retirement: Retirement<V>) {
    loop {
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:retirement.next_close_byte_demand().expect("finite ordered cold release demand"),maximum_depth:retirement.next_depth_demand()};
        match retirement.advance(grant) { RetirementStep::OwnedValue(value) => drop(value), RetirementStep::Complete => break, RetirementStep::Failure(error)=>panic!("ordered cold retirement refused: {error}"), _ => {} }
    }
}

/// 📪️ Cold-only convenience cleanup; explicit synchronous APIs cannot earn interactive credit.
fn close_cold<V>(cursor: &mut UpdateCursor<V>) {
    cursor.begin_close();
    loop {let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().expect("ordered cold close release demand"),maximum_depth:cursor.next_close_depth_demand()};match cursor.close_step(grant) { RetirementStep::OwnedValue(value) => drop(value), RetirementStep::Complete => break, RetirementStep::Failure(error)=>panic!("ordered cold close refused: {error}"), _ => {} } }
}

fn release_cold<T>(mut value:SharedOwner<T>)->Option<T>{value.release_step(RetainedCloneGrant::one_release_turn(value.next_release_byte_demand(),1)).result.expect("cold shared release").value}
//#endregion 🧹️Retirement

#[path = "♻️retirement/🦀️.rs"]
mod typed_retirement;

//#region 🔀️Serde
/// 🔀️ Serializes the first-party ordered object through the explicitly reexported codec interface.
impl<V: serde::Serialize> serde::Serialize for OrderedMap<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?; for (key, value) in self.iter() { map.serialize_entry(key, value)?; } map.end()
    }
}
/// 🛬️ Deserializes an ordered object at explicit synchronous codec boundaries.
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
#[path = "🧪️tests/🗂️ordered/🦀️.rs"]
mod tests;

#[cfg(test)]#[path="♻️retirement/⏱️cursor-work/🧪️tests/🦀️.rs"]mod close_cursor_work_tests;

#[cfg(test)]#[path="🎮️update/🎟️work/🧪️tests/🦀️.rs"]mod update_work_tests;
