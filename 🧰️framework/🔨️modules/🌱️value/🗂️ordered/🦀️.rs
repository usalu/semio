//! 🗂️ Immutable ordered ownership with byte-resumable comparison, path copying, and final-owner retirement.

use crate::ValueError;
use std::cmp::Ordering;
use std::mem::{ManuallyDrop,size_of};
use crate::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
pub(crate) fn narrow_work(work:u128)->u64{u64::try_from(work).unwrap_or(u64::MAX)}

#[path = "🧺️set/🦀️.rs"]
pub mod set;
pub use set::OrderedSet;

#[path = "🔗️immutable-alias/🦀️.rs"]
mod immutable_alias;
pub use immutable_alias::{SharedOwner, SharedRelease};

//#region 🌳️PersistentNodes
use semio_framework_value_native_ordered_engine::{Root,Node,Entry,native_height as height,native_len as len,native_compare_bytes as compare_bytes};
#[path="🎮️update/🎬️plan/🦀️.rs"]mod update_plan;
use update_plan::{BalancePlan,EntryBuilder,NodeBuilder};
/// 📏️ Fixed metadata-visit bound for one rank-iterator item; no payload bytes are read by iteration.
pub const MAX_AVL_HEIGHT: usize = 2 * usize::BITS as usize;

fn node<V>(entry: SharedOwner<Entry<V>>, left: Root<V>, right: Root<V>) -> SharedOwner<Node<V>> {
    let height = 1 + height(&left).max(height(&right));
    assert!(height <= MAX_AVL_HEIGHT, "ordered-map fixed metadata frontier exceeded");
    SharedOwner::from_cold(Node { height, len: 1 + len(&left) + len(&right), entry, left, right })
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

fn balanced<V>(entry: SharedOwner<Entry<V>>, left: Root<V>, right: Root<V>, retired:&mut InlineStack<SharedOwner<Node<V>>,UPDATE_RETIRED_ROOTS>) -> SharedOwner<Node<V>> {
    if height(&left) > height(&right) + 1 {
        let left_root=left.unwrap();let branch=&left_root;
        if height(&branch.left) >= height(&branch.right) {
            let right = node(entry, branch.right.clone(), right);
            let output=node(SharedOwner::clone(&branch.entry), branch.left.clone(), Some(right));
            retired.push_front(left_root);return output;
        }
        let pivot = branch.right.as_ref().unwrap();
        let left = node(SharedOwner::clone(&branch.entry), branch.left.clone(), pivot.left.clone());
        let right = node(entry, pivot.right.clone(), right);
        let output=node(SharedOwner::clone(&pivot.entry), Some(left), Some(right));
        retired.push_front(left_root);return output;
    }
    if height(&right) > height(&left) + 1 {
        let right_root=right.unwrap();let branch=&right_root;
        if height(&branch.right) >= height(&branch.left) {
            let left = node(entry, left, branch.left.clone());
            let output=node(SharedOwner::clone(&branch.entry), Some(left), branch.right.clone());
            retired.push_front(right_root);return output;
        }
        let pivot = branch.left.as_ref().unwrap();
        let left = node(entry, left, pivot.left.clone());
        let right = node(SharedOwner::clone(&branch.entry), pivot.right.clone(), branch.right.clone());
        let output=node(SharedOwner::clone(&pivot.entry), Some(left), Some(right));
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
 fn alias_demand(&self)->crate::RetirementDemand{let mut demand=if self.root.is_some(){SharedOwner::<Node<V>>::duplicate_demand()}else{crate::RetirementDemand{depth:1,..Default::default()}};demand.copy_bytes+=size_of::<Root<V>>()+size_of::<Option<Self>>();demand}
 fn alias_work_demand(&self)->u64{if self.root.is_some(){SharedOwner::<Node<V>>::DUPLICATE_WORK+1}else{1}}
 fn admit_alias_into<'r>(&self,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if output.is_some(){return Err(ValueError::invariant("ordered alias output is occupied"))}if effect.invocation().is_none(){return Err(ValueError::invariant("ordered alias lacks its original invocation"))}
  let demand=semio_framework_value_retained_clone::RetainedAlias::alias_demand(self);let maximum=semio_framework_value_retained_clone::RetainedAlias::alias_work_demand(self);
  if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<maximum{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let mut root=None;let mut progress=RetainedCloneProgress::default();
  if let Some(source)=self.root.as_ref(){let turn=source.duplicate_into(&mut root,effect);let step=match turn.result{Ok(step)=>step,Err(error)=>return Ok(reply.complete(Err(error.into())))};progress=step.progress();}
  if !effect.performed(1)?{return Err(ValueError::invariant("ordered alias publication exceeded original Work"))}*output=Some(Self{root:ManuallyDrop::new(root.take())});progress.copied_items=1;progress.copied_bytes+=size_of::<Root<V>>()+size_of::<Option<Self>>();Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress))))
 }
 fn recover_alias_into<'r>(&self,_output:&mut Option<Self>,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::invariant("native ordered alias has no unresolved callback"))}
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
    pub fn original_root_identity(&self)->Option<usize>{self.root.as_ref().map(|root|root.identity())}
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
        while !cursor.is_complete() && cursor.advance(cold_update_grant(&cursor),u64::MAX).result.is_ok() {}
        let removed = cursor.take_removed(); if let Some(result) = cursor.take_result() { retire_cold(std::mem::replace(self, result).retire()); } close_cold(&mut cursor); removed.and_then(release_cold)
    }
    /// 🏪️ Completes a synchronous convenience removal; retained jobs must use begin_remove and advance.
    /// `Some` states that the key WAS present, whatever else still shares its payload: a clone of this map keeps the payload
    /// alive, so only [`Removal::Owned`] hands the value over and [`Removal::Shared`] reports that another owner retains it.
    pub fn remove(&mut self, key: &str) -> Option<Removal<V>> {
        let mut cursor = self.begin_remove(key.to_owned());
        while !cursor.is_complete() && cursor.advance(cold_update_grant(&cursor),u64::MAX).result.is_ok() {}
        let removed = cursor.take_removed(); if let Some(result) = cursor.take_result() { retire_cold(std::mem::replace(self, result).retire()); } close_cold(&mut cursor);
        removed.map(|shared| release_cold(shared).map_or(Removal::Shared, Removal::Owned))
    }
    /// 🧹️ Moves this root into explicit final-owner retirement.
    pub fn retire(mut self) -> Retirement<V> { Retirement::new(self.root.take()) }
    /// ❄️ Retires this root synchronously for cold paths that own no retained job.
    pub fn retire_cold(self) { retire_cold(self.retire()) }
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



semio_framework_value_native_ordered_engine::ordered_update_engine!();
//#endregion 🧵️UpdateCursor

//#region 🔎️LookupCursor
struct LookupState<V> {
    base: OrderedMap<V>, key: Option<SharedOwner<String>>, current: Root<V>, rank:usize, offset: usize, left_byte: Option<u8>, ordering: Option<Ordering>,
    complete: bool, closing: bool, retirement: Retirement<V>, last_close_work:u64,
}

impl<V> LookupState<V> {
    fn advance(&mut self,grant:Grant)->Step{
        if self.closing||grant.maximum_items==0||grant.maximum_bytes==0{return Step::Blocked}if self.complete{return Step::Complete}
        let mut bytes=0;if self.current.is_none(){self.complete=true}else if self.ordering.is_none(){bytes=compare_bytes(self.key.as_ref().expect("owned query remains resident").as_bytes(),self.current.as_ref().unwrap().entry.key.as_bytes(),&mut self.offset,&mut self.left_byte,&mut self.ordering,grant.maximum_bytes)}else if self.ordering==Some(Ordering::Equal){self.rank+=len(&self.current.as_ref().unwrap().left);self.complete=true}else{
            let mut current=self.current.take().expect("selected lookup node");self.current=if self.ordering==Some(Ordering::Less){current.left.clone()}else{self.rank+=len(&current.left)+1;current.right.clone()};if let Some(owner)=current.elect_cold(){*self.retirement.elected_node=Some(owner);panic!("immutable lookup base lost its original node alias")}self.offset=0;self.left_byte=None;self.ordering=None;
        }Step::Progress{completed_items:1,completed_bytes:bytes}
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        self.last_close_work=0;
        if self.terminal_is_empty(){return RetirementStep::Complete;}
        if !self.closing || grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if !self.retirement.is_empty() { let step=self.retirement.advance(grant);self.last_close_work=self.retirement.last_turn_work();return step; }
        if grant.maximum_depth==0{return RetirementStep::Failure(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered close requires admitted metadata depth"));}
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
    fn native_close_header_bytes(&self)->usize{size_of::<Root<V>>()+size_of::<Option<Owner<V>>>()+size_of::<usize>()}

    fn new(base: OrderedMap<V>, key: SharedOwner<String>) -> Self {
        let current = (*base.root).clone();
        Self { state: ManuallyDrop::new(LookupState { base, key: Some(key), current, rank:0, offset: 0, left_byte: None, ordering: None, complete: false, closing: false, retirement: Retirement::default(), last_close_work:0 }) }
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
    pub fn next_close_byte_demand(&self)->Result<usize,ValueError>{self.state.retirement.next_close_byte_demand()}
    pub fn next_close_copy_byte_demand(&self)->usize{self.state.retirement.next_copy_byte_demand()}
    pub fn next_close_depth_demand(&self)->usize{self.state.retained_depth()+self.state.retirement.next_depth_demand()}
    pub fn terminal_is_empty(&self) -> bool { self.state.terminal_is_empty() }
}
impl<V> Drop for LookupCursor<V>{
    fn drop(&mut self){let empty=self.state.terminal_is_empty();assert!(empty||std::thread::panicking(),"ordered-map lookup must finish explicit close before drop");if empty{unsafe{ManuallyDrop::drop(&mut self.state)}}}
}
//#endregion 🔎️LookupCursor

//#region 🧹️Retirement
enum Owner<V> {
    Node(SharedOwner<Node<V>>), Entry(SharedOwner<Entry<V>>), Key(SharedOwner<String>), Value(SharedOwner<V>),
}
/// 🪜️ OwnedValue transfers final payload ownership to the caller's domain retirement cursor.
pub enum RetirementStep<V> { Blocked, Progress { released_items: usize, released_bytes: usize }, ProcessedBytes(usize), OwnedValue(V), Failure(ValueError), Complete }

#[must_use = "retirement owners must be drained before drop"]
pub struct Retirement<V> {
    elected_node:ManuallyDrop<Option<crate::retirement::allocation_return::original::SharedOriginal<Node<V>>>>,
    elected_entry:ManuallyDrop<Option<crate::retirement::allocation_return::original::SharedOriginal<Entry<V>>>>,
    elected_key:ManuallyDrop<Option<crate::retirement::allocation_return::original::SharedOriginal<String>>>,
    elected_value:ManuallyDrop<Option<crate::retirement::allocation_return::original::SharedOriginal<V>>>,
    node_value:ManuallyDrop<Option<Node<V>>>,entry_value:ManuallyDrop<Option<Entry<V>>>,key_value:ManuallyDrop<Option<String>>,

    last_work:u64,
    owners: ManuallyDrop<[Option<Owner<V>>; MAX_AVL_HEIGHT + 3]>,
    payload:ManuallyDrop<Option<V>>,
    key_bytes:ManuallyDrop<Option<Vec<u8>>>,
    length: usize,
}
impl<V> Default for Retirement<V> {
    fn default() -> Self { Self { elected_node:ManuallyDrop::new(None),elected_entry:ManuallyDrop::new(None),elected_key:ManuallyDrop::new(None),elected_value:ManuallyDrop::new(None),node_value:ManuallyDrop::new(None),entry_value:ManuallyDrop::new(None),key_value:ManuallyDrop::new(None),last_work:0,owners: ManuallyDrop::new(std::array::from_fn(|_| None)), payload:ManuallyDrop::new(None), key_bytes:ManuallyDrop::new(None), length: 0 } }
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

    fn native_top_empty(&self)->bool{self.length.checked_sub(1).and_then(|i|self.owners[i].as_ref()).is_some_and(|owner|match owner{Owner::Node(value)=>value.terminal_is_empty(),Owner::Entry(value)=>value.terminal_is_empty(),Owner::Key(value)=>value.terminal_is_empty(),Owner::Value(value)=>value.terminal_is_empty()})}
    fn native_header<B>()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Option<B>>(),depth:1,..Default::default()}}
    fn native_demand(&self)->Result<crate::RetirementDemand,ValueError>{
        if self.native_top_empty(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<Owner<V>>>()+size_of::<usize>(),depth:1,..Default::default()})}
        macro_rules! elected{($field:ident,$type:ty)=>{if let Some(owner)=self.$field.as_ref(){return if owner.terminal_is_empty(){Ok(Self::native_header::<crate::retirement::allocation_return::original::SharedOriginal<$type>>())}else{owner.take_value_demand()}}}}
        elected!(elected_node,Node<V>);elected!(elected_entry,Entry<V>);elected!(elected_key,String);elected!(elected_value,V);
        if self.node_value.is_some(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<Node<V>>>()+3*(size_of::<Option<Owner<V>>>()+size_of::<usize>()),depth:1,..Default::default()})}
        if self.entry_value.is_some(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<Entry<V>>>()+2*(size_of::<Option<Owner<V>>>()+size_of::<usize>()),depth:1,..Default::default()})}
        if self.key_value.is_some(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<String>>()+size_of::<Option<Vec<u8>>>(),depth:1,..Default::default()})}
        if let Some(bytes)=self.key_bytes.as_ref(){return Ok(crate::RetirementDemand{copy_bytes:if bytes.is_empty(){size_of::<Option<Vec<u8>>>()}else{size_of::<usize>()},release_bytes:if bytes.is_empty(){bytes.capacity()}else{0},depth:1,..Default::default()})}
        if self.payload.is_some(){return Ok(Self::native_header::<V>())}
        match self.length.checked_sub(1).and_then(|i|self.owners[i].as_ref()){Some(Owner::Node(owner))=>owner.retire_into_demand(),Some(Owner::Entry(owner))=>owner.retire_into_demand(),Some(Owner::Key(owner))=>owner.retire_into_demand(),Some(Owner::Value(owner))=>owner.retire_into_demand(),None=>Ok(Default::default())}
    }
    fn native_work(&self)->Result<u64,ValueError>{
        if self.native_top_empty(){return Ok(1)}
        macro_rules! elected{($field:ident)=>{if let Some(owner)=self.$field.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.take_value_work_demand()}}}}
        elected!(elected_node);elected!(elected_entry);elected!(elected_key);elected!(elected_value);
        if self.node_value.is_some()||self.entry_value.is_some()||self.key_value.is_some()||self.key_bytes.is_some()||self.payload.is_some(){return Ok(1)}
        match self.length.checked_sub(1).and_then(|i|self.owners[i].as_ref()){Some(Owner::Node(owner))=>owner.retire_work_demand(),Some(Owner::Entry(owner))=>owner.retire_work_demand(),Some(Owner::Key(owner))=>owner.retire_work_demand(),Some(Owner::Value(owner))=>owner.retire_work_demand(),None=>Ok(0)}
    }
    fn native_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep>{
        let before=effect.performed_boundary();let result=(||{
            if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            let demand=self.native_demand()?;let work=self.native_work()?;
            if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}
            if self.native_top_empty(){if !effect.performed(1)?{return Err(ValueError::invariant("funded ordered frontier clear refused"))}drop(self.pop());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            macro_rules! elected{($field:ident,$output:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){if !effect.performed(1)?{return Err(ValueError::invariant("funded ordered original header clear refused"))}self.$field.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let turn=owner.take_value_step(&mut self.$output,effect);return turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default()))}}}
            elected!(elected_node,node_value);elected!(elected_entry,entry_value);elected!(elected_key,key_value);elected!(elected_value,payload);
            if self.node_value.is_some(){if self.length+3>self.owners.len(){return Err(ValueError::invariant("ordered native node frontier exceeded AVL bound"))}if !effect.performed(1)?{return Err(ValueError::invariant("funded ordered node transfer refused"))}let node=self.node_value.take().expect("funded original node");let mut copies=size_of::<Option<Node<V>>>();if let Some(left)=node.left{self.push(Owner::Node(left));copies+=size_of::<Option<Owner<V>>>()+size_of::<usize>()}if let Some(right)=node.right{self.push(Owner::Node(right));copies+=size_of::<Option<Owner<V>>>()+size_of::<usize>()}self.push(Owner::Entry(node.entry));copies+=size_of::<Option<Owner<V>>>()+size_of::<usize>();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copies,..Default::default()}))}
            if self.entry_value.is_some(){if self.length+2>self.owners.len(){return Err(ValueError::invariant("ordered native entry frontier exceeded AVL bound"))}if !effect.performed(1)?{return Err(ValueError::invariant("funded ordered entry transfer refused"))}let entry=self.entry_value.take().expect("funded original entry");self.push(Owner::Key(entry.key));self.push(Owner::Value(entry.value));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            if self.key_value.is_some(){if !effect.performed(1)?{return Err(ValueError::invariant("funded ordered key transfer refused"))}*self.key_bytes=self.key_value.take().map(String::into_bytes);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
            if let Some(bytes)=self.key_bytes.as_mut(){if !effect.performed(1)?{return Err(ValueError::invariant("funded ordered key release refused"))}let released_bytes=if bytes.is_empty(){let n=bytes.capacity();self.key_bytes.take();n}else{bytes.clear();0};return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}))}
            if self.payload.is_some(){return Err(ValueError::invariant("ordered payload must transfer into its controlled owner"))}
            let Some(owner)=self.length.checked_sub(1).and_then(|i|self.owners[i].as_mut())else{return Ok(RetainedCloneStep::Complete(Default::default()))};
            match owner{Owner::Node(owner)=>owner.elect_native_into(&mut self.elected_node,effect).result.map(|s|RetainedCloneStep::Progress(s.progress())),Owner::Entry(owner)=>owner.elect_native_into(&mut self.elected_entry,effect).result.map(|s|RetainedCloneStep::Progress(s.progress())),Owner::Key(owner)=>owner.elect_native_into(&mut self.elected_key,effect).result.map(|s|RetainedCloneStep::Progress(s.progress())),Owner::Value(owner)=>owner.elect_native_into(&mut self.elected_value,effect).result.map(|s|RetainedCloneStep::Progress(s.progress()))}
        })();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same ordered native retirement receipt")}
    }
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
    pub fn is_empty(&self) -> bool { self.length == 0 && self.payload.is_none() && self.key_bytes.is_none() && self.elected_node.is_none()&&self.elected_entry.is_none()&&self.elected_key.is_none()&&self.elected_value.is_none()&&self.node_value.is_none()&&self.entry_value.is_none()&&self.key_value.is_none() }
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
    pub fn next_close_byte_demand(&self) -> Result<usize, ValueError> {
        if self.payload.is_some(){return Ok(0);}
        if let Some(bytes)=self.key_bytes.as_ref(){return Ok(if bytes.is_empty(){bytes.capacity()}else{0});}
        Ok(match self.length.checked_sub(1).and_then(|index| self.owners[index].as_ref()) {
            Some(Owner::Node(_))=>SharedOwner::<Node<V>>::allocation_bytes(),
            Some(Owner::Entry(_))=>SharedOwner::<Entry<V>>::allocation_bytes(),
            Some(Owner::Key(key))=>if key.terminal_is_empty(){0}else{SharedOwner::<String>::allocation_bytes()},
            Some(Owner::Value(value))=>if value.terminal_is_empty(){0}else{SharedOwner::<V>::allocation_bytes()},
            None => 0,
        })
    }
    pub fn next_copy_byte_demand(&self)->usize {0}
    /// ⏱️ Reads actual work from this defining close producer, including child failure.
    pub fn last_turn_work(&self)->u64{self.last_work}
    pub fn next_work_demand(&self)->u64{u64::from(!self.is_empty())}
    pub fn advance(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
        self.last_work=0;
        if self.is_empty(){return RetirementStep::Complete;}
        if grant.maximum_items == 0 { return RetirementStep::Blocked; }
        if grant.maximum_depth<self.next_depth_demand(){return RetirementStep::Failure(ValueError::literal(crate::ValueRefusalKind::DepthLimit,"ordered retirement frontier exceeds admitted depth"));}
        if let Some(values)=self.key_bytes.as_mut(){if !values.is_empty(){let bytes=values.len();self.last_work=1;values.clear();return RetirementStep::ProcessedBytes(bytes);}}
        let demand=match self.next_close_byte_demand(){Ok(bytes)=>bytes,Err(error)=>return RetirementStep::Failure(error)};
        if demand>grant.maximum_release_bytes{return RetirementStep::Blocked;}
        if let Some(value)=self.payload.take(){self.last_work=1;return RetirementStep::OwnedValue(value);}
        if let Some(values)=self.key_bytes.take(){self.last_work=1;let bytes=values.capacity();drop(values);return RetirementStep::Progress {released_items:1,released_bytes:bytes};}
        if let Some(owner)=self.length.checked_sub(1).and_then(|index|self.owners[index].as_mut()){
            match owner{
                Owner::Key(key)=>{let step=immutable_alias::release(key,grant);self.last_work=1;return if step.progress.copied_items==0{RetirementStep::Blocked}else{drop(self.pop());*self.key_bytes=step.value.map(String::into_bytes);RetirementStep::Progress{released_items:1,released_bytes:step.progress.released_bytes}}},
                Owner::Value(value)=>{let step=immutable_alias::release(value,grant);self.last_work=1;return if step.progress.copied_items==0{RetirementStep::Blocked}else{drop(self.pop());*self.payload=step.value;RetirementStep::Progress{released_items:1,released_bytes:step.progress.released_bytes}}},
                _=>{}
            }
        }
        let Some(owner) = self.pop() else { return RetirementStep::Complete; };self.last_work=1;
        let mut bytes = 0;
        match owner {
            Owner::Node(node) => if let Some(node) = release_cold(node) {
                bytes=demand;
                if let Some(left) = node.left { self.push(Owner::Node(left)); }
                if let Some(right) = node.right { self.push(Owner::Node(right)); }
                self.push(Owner::Entry(node.entry));
            },
            Owner::Entry(entry) => if let Some(entry) = release_cold(entry) {bytes=demand;self.push(Owner::Key(entry.key)); self.push(Owner::Value(entry.value)); },
            Owner::Key(_)|Owner::Value(_)=>return RetirementStep::Failure(crate::ValueError::invariant("shared aliases release before their frontier slot transfers")),
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
    loop {let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().expect("ordered cold close release demand"),maximum_depth:cursor.next_close_depth_demand()};match cursor.close_step(grant,u64::MAX) { RetirementStep::OwnedValue(value) => drop(value), RetirementStep::Complete => break, RetirementStep::Failure(error)=>panic!("ordered cold close refused: {error}"), _ => {} } }
}

fn release_cold<T>(mut value:SharedOwner<T>)->Option<T>{immutable_alias::release(&mut value,RetainedCloneGrant::one_release_turn(SharedOwner::<T>::allocation_bytes(),1)).value}
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

#[cfg(test)]
/// 🏦️ One finite effect bank lends every ordered law its Source-bound original effect and exact settlement.
pub(crate) mod bank{
 use crate::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll};
 pub use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
 /// 🏦️ Opens the finite wallet an ordered law spends from, described by its authored stock.
 pub fn bank(stock:&serde_json::Value)->EffectBank{EffectBank::new(Wallet::from_profile(stock))}
 /// 🧵️ Runs one synchronous original action and returns its settled turn with the physical marker.
 pub fn once<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->(RetainedWorkOutcome<T,E>,Option<usize>){let(poll,actual)=bank.run(grant,maximum,operation);let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous ordered action stayed pending")};assert_ne!(bank.take_admitted(),Some(false));(turn,actual)}
 /// ⚙️ Runs one synchronous original action that already settles its own outcome.
 pub fn sync<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->(RetainedWorkOutcome<T,E>,Option<usize>){once(bank,grant,maximum,|effect|RetainedWorkPoll::Complete(operation(effect)))}
 /// 🧾️ Posts independently observed physical progress after the action has debited its work.
 pub fn post(bank:&mut EffectBank,progress:RetainedCloneProgress,grant:RetainedCloneGrant,heap:(usize,usize)){bank.post(progress,grant,heap)}
 /// 🧰️ A generous finite wallet lends cleanup effects whose grants and quotes stay the caller's own.
 pub fn scratch()->EffectBank{bank(&serde_json::json!({"items":1u64<<40,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60}))}
 /// 🎯️ Drives one controlled retirement turn under its own quoted work and returns the genuine step.
 pub fn drive<T:semio_framework_value_retirement::retirement::RetireOwned>(bank:&mut EffectBank,owner:&mut semio_framework_value_retirement::retirement::controlled::ControlledRetirement<T>,grant:RetainedCloneGrant)->crate::RetainedCloneStep{let units=owner.next_computational_work(grant.maximum_copy_bytes).unwrap();let(turn,_)=once(bank,grant,units,|effect|owner.step(grant.maximum_copy_bytes,effect));turn.result.unwrap()}

}
#[cfg(test)]#[path="♻️retirement/⏱️cursor-work/🧪️tests/🦀️.rs"]mod close_cursor_work_tests;

#[cfg(test)]#[path="🎮️update/🎟️work/🧪️tests/🦀️.rs"]mod update_work_tests;

#[path="✍️owned-entry/🦀️.rs"]
mod owned_entry;
pub use owned_entry::OriginalOwnedEntry;


#[path="🔎️lookup/🪞️borrowed/🦀️.rs"]
mod borrowed_lookup;
pub use borrowed_lookup::BorrowedLookupCursor;

#[cfg(test)]#[path="🧪️tests/bank.rs"]mod test_bank;
