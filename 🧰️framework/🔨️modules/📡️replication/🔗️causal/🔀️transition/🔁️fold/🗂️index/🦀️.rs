//! 🌳️ Fold-owned AVL indexes retain native inline slots until independently granted retirement.
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, deferred, deferred_birth_bytes_for, sequence, sequence_birth_bytes};
use std::{borrow::Borrow, cmp::Ordering};

#[derive(Clone, Debug, PartialEq, Eq)]
struct FoldIndexNode<K, V> { entry: Option<(K,V)>, left: Option<usize>, right: Option<usize>, height: usize, count: usize }

/// 🌳️ Ordered fold facts own every arena slot and displaced key until exact retirement.
#[derive(Clone, Debug)]
pub struct HistoryFoldIndex<K, V> { nodes: Vec<FoldIndexNode<K,V>>, root: Option<usize>, displaced_keys: Vec<K> }
impl<K:Ord,V:PartialEq> PartialEq for HistoryFoldIndex<K,V>{fn eq(&self,other:&Self)->bool{self.len()==other.len()&&self.iter().eq(other.iter())}}
impl<K:Ord,V:Eq> Eq for HistoryFoldIndex<K,V>{}
impl<K,V> Default for HistoryFoldIndex<K,V> { fn default()->Self { Self { nodes: Vec::new(), root: None, displaced_keys: Vec::new() } } }
impl<K:Ord,V> HistoryFoldIndex<K,V> {
    pub fn new()->Self { Self::default() }
    pub fn len(&self)->usize { self.count(self.root) }
    pub fn is_empty(&self)->bool { self.root.is_none() }
    /// 🪹️ Physical emptiness includes every retained arena and displaced original key backing.
    pub fn terminal_is_empty(&self)->bool {self.root.is_none()&&self.nodes.capacity()==0&&self.displaced_keys.capacity()==0}
    fn height(&self,root:Option<usize>)->usize { root.map_or(0,|root|self.nodes[root].height) }
    fn count(&self,root:Option<usize>)->usize { root.map_or(0,|root|self.nodes[root].count) }
    fn refresh(&mut self,root:usize) { self.nodes[root].height=1+self.height(self.nodes[root].left).max(self.height(self.nodes[root].right)); self.nodes[root].count=1+self.count(self.nodes[root].left)+self.count(self.nodes[root].right); }
    fn rotate_left(&mut self,root:usize)->usize { let right=self.nodes[root].right.expect("right rotation member"); self.nodes[root].right=self.nodes[right].left; self.nodes[right].left=Some(root); self.refresh(root);self.refresh(right);right }
    fn rotate_right(&mut self,root:usize)->usize { let left=self.nodes[root].left.expect("left rotation member"); self.nodes[root].left=self.nodes[left].right;self.nodes[left].right=Some(root);self.refresh(root);self.refresh(left);left }
    fn balance(&mut self,root:usize)->usize { self.refresh(root);let left=self.height(self.nodes[root].left);let right=self.height(self.nodes[root].right);if left>right+1 { let child=self.nodes[root].left.unwrap();if self.height(self.nodes[child].right)>self.height(self.nodes[child].left){self.nodes[root].left=Some(self.rotate_left(child));}return self.rotate_right(root);}if right>left+1 {let child=self.nodes[root].right.unwrap();if self.height(self.nodes[child].left)>self.height(self.nodes[child].right){self.nodes[root].right=Some(self.rotate_right(child));}return self.rotate_left(root);}root }
    fn locate<Q:Ord+?Sized>(&self,key:&Q)->Option<usize> where K:Borrow<Q> { let mut root=self.root;while let Some(index)=root {let (candidate,_)=self.nodes[index].entry.as_ref().unwrap();match key.cmp(candidate.borrow()){Ordering::Less=>root=self.nodes[index].left,Ordering::Greater=>root=self.nodes[index].right,Ordering::Equal=>return Some(index)}}None }
    pub fn get<Q:Ord+?Sized>(&self,key:&Q)->Option<&V> where K:Borrow<Q> { self.locate(key).map(|index|&self.nodes[index].entry.as_ref().unwrap().1) }
    pub fn get_mut<Q:Ord+?Sized>(&mut self,key:&Q)->Option<&mut V> where K:Borrow<Q> { self.locate(key).map(|index|&mut self.nodes[index].entry.as_mut().unwrap().1) }
    /// 🗝️ Borrows the original stored key and value without creating an owned key.
    pub fn get_key_value<Q:Ord+?Sized>(&self,key:&Q)->Option<(&K,&V)> where K:Borrow<Q>{self.locate(key).map(|index|{let(key,value)=self.nodes[index].entry.as_ref().unwrap();(key,value)})}
    /// 🌱️ Retains a Copy key while a vacant row lazily admits its value.
    pub fn entry(&mut self,key:K)->HistoryFoldIndexEntry<'_ ,K,V> where K:Copy{let slot=self.locate(&key);HistoryFoldIndexEntry{owner:self,key,slot}}
    pub fn contains_key<Q:Ord+?Sized>(&self,key:&Q)->bool where K:Borrow<Q> { self.locate(key).is_some() }
    fn insert_at(&mut self,root:Option<usize>,key:K,value:V)->(usize,Option<V>) { let Some(root)=root else {let index=self.nodes.len();self.nodes.push(FoldIndexNode{entry:Some((key,value)),left:None,right:None,height:1,count:1});return(index,None)};let order=key.cmp(&self.nodes[root].entry.as_ref().unwrap().0);let previous=match order {Ordering::Less=>{let(child,previous)=self.insert_at(self.nodes[root].left,key,value);self.nodes[root].left=Some(child);previous},Ordering::Greater=>{let(child,previous)=self.insert_at(self.nodes[root].right,key,value);self.nodes[root].right=Some(child);previous},Ordering::Equal=>{self.displaced_keys.push(key);return(root,Some(std::mem::replace(&mut self.nodes[root].entry.as_mut().unwrap().1,value)));}};(self.balance(root),previous) }
    pub fn insert(&mut self,key:K,value:V)->Option<V> { let(root,previous)=self.insert_at(self.root,key,value);self.root=Some(root);previous }
    pub fn get_or_insert(&mut self,key:K,value:V)->&mut V { if let Some(index)=self.locate(&key){self.displaced_keys.push(key);return &mut self.nodes[index].entry.as_mut().unwrap().1;}let(root,_)=self.insert_at(self.root,key,value);self.root=Some(root);let index=self.nodes.len()-1;&mut self.nodes[index].entry.as_mut().unwrap().1 }
    fn detach_first(&mut self,root:usize)->(Option<usize>,usize) {if let Some(left)=self.nodes[root].left {let(next,first)=self.detach_first(left);self.nodes[root].left=next;(Some(self.balance(root)),first)}else{(self.nodes[root].right,root)}}
    fn detach_last(&mut self,root:usize)->(Option<usize>,usize) {if let Some(right)=self.nodes[root].right {let(next,last)=self.detach_last(right);self.nodes[root].right=next;(Some(self.balance(root)),last)}else{(self.nodes[root].left,root)}}
    fn remove_at<Q:Ord+?Sized>(&mut self,root:Option<usize>,key:&Q)->(Option<usize>,Option<(K,V)>) where K:Borrow<Q> { let Some(root)=root else{return(None,None)};let order=key.cmp(self.nodes[root].entry.as_ref().unwrap().0.borrow());match order {Ordering::Less=>{let(next,entry)=self.remove_at(self.nodes[root].left,key);self.nodes[root].left=next;(Some(self.balance(root)),entry)},Ordering::Greater=>{let(next,entry)=self.remove_at(self.nodes[root].right,key);self.nodes[root].right=next;(Some(self.balance(root)),entry)},Ordering::Equal=>self.detach_entry(root)} }
    fn detach_entry(&mut self,root:usize)->(Option<usize>,Option<(K,V)>) {let entry=self.nodes[root].entry.take();let next=match(self.nodes[root].left,self.nodes[root].right){(None,right)=>right,(left,None)=>left,(left,Some(right))=>{let(next,successor)=self.detach_first(right);self.nodes[successor].left=left;self.nodes[successor].right=next;Some(self.balance(successor))}};self.nodes[root].left=None;self.nodes[root].right=None;self.nodes[root].count=0;self.nodes[root].height=0;(next,entry)}
    fn remove_rank_at(&mut self,root:Option<usize>,rank:usize)->(Option<usize>,Option<(K,V)>) {let Some(root)=root else{return(None,None)};let left=self.count(self.nodes[root].left);match rank.cmp(&left){Ordering::Less=>{let(next,entry)=self.remove_rank_at(self.nodes[root].left,rank);self.nodes[root].left=next;(Some(self.balance(root)),entry)},Ordering::Greater=>{let(next,entry)=self.remove_rank_at(self.nodes[root].right,rank-left-1);self.nodes[root].right=next;(Some(self.balance(root)),entry)},Ordering::Equal=>self.detach_entry(root)}}
    /// 🎟️ Counts original arena slots, including holes, without collecting a key directory.
    pub fn slot_count(&self)->usize {self.nodes.len()}
    /// 🍂️ Examines one original slot and transfers its original row without allocation or cloning.
    pub fn extract_slot_if(&mut self,index:usize,keep:impl FnOnce(&K,&V)->bool)->Option<(K,V)> {
        let (key,value)=self.nodes.get(index)?.entry.as_ref()?;if keep(key,value){return None;}
        let mut root=self.root;let mut rank=0;
        while let Some(cursor)=root {let left=self.count(self.nodes[cursor].left);match key.cmp(&self.nodes[cursor].entry.as_ref().unwrap().0){Ordering::Less=>root=self.nodes[cursor].left,Ordering::Greater=>{rank+=left+1;root=self.nodes[cursor].right;},Ordering::Equal=>{rank+=left;break;}}}
        let (root,entry)=self.remove_rank_at(self.root,rank);self.root=root;entry
    }
    /// 🎟️ Transfers the original key and value while retaining every arena backing allocation.
    pub fn remove_entry<Q:Ord+?Sized>(&mut self,key:&Q)->Option<(K,V)> where K:Borrow<Q> {let(root,entry)=self.remove_at(self.root,key);self.root=root;entry}
    pub fn remove<Q:Ord+?Sized>(&mut self,key:&Q)->Option<V> where K:Borrow<Q> {self.remove_entry(key).map(|(key,value)|{self.displaced_keys.push(key);value})}
    pub fn pop_first(&mut self)->Option<(K,V)> {let(root,index)=self.detach_first(self.root?);self.root=root;let entry=self.nodes[index].entry.take();self.nodes[index].left=None;self.nodes[index].right=None;self.nodes[index].height=0;self.nodes[index].count=0;entry}
    /// 🪵️ Transfers the original greatest-key row while retaining its native arena slot.
    pub fn pop_last(&mut self)->Option<(K,V)> {let(root,index)=self.detach_last(self.root?);self.root=root;let entry=self.nodes[index].entry.take();self.nodes[index].left=None;self.nodes[index].right=None;self.nodes[index].height=0;self.nodes[index].count=0;entry}
    fn ranked(&self,mut rank:usize)->Option<(&K,&V)> {let mut root=self.root;while let Some(index)=root{let left=self.count(self.nodes[index].left);if rank<left{root=self.nodes[index].left;}else if rank==left{let(key,value)=self.nodes[index].entry.as_ref().unwrap();return Some((key,value));}else{rank-=left+1;root=self.nodes[index].right;}}None}
    /// 🔎️ Borrows the next strictly greater key without allocating a range cursor.
    pub fn first_entry_after<Q:Ord+?Sized>(&self,key:&Q)->Option<(&K,&V)> where K:Borrow<Q> {let mut root=self.root;let mut candidate=None;while let Some(index)=root{let(entry,_)=self.nodes[index].entry.as_ref().unwrap();if entry.borrow()>key{candidate=Some(index);root=self.nodes[index].left;}else{root=self.nodes[index].right;}}candidate.map(|index|{let(key,value)=self.nodes[index].entry.as_ref().unwrap();(key,value)})}
    pub fn iter(&self)->HistoryFoldIndexIter<'_,K,V> { HistoryFoldIndexIter { owner:self, ranks:0..self.len() } }

    pub fn keys(&self)->impl DoubleEndedIterator<Item=&K>+ExactSizeIterator {self.iter().map(|(key,_)|key)}
    pub fn values(&self)->impl DoubleEndedIterator<Item=&V>+ExactSizeIterator {self.iter().map(|(_,value)|value)}
    /// 🪵️ Borrows live values in original arena order without a copied key directory.
    pub fn slot_values_mut(&mut self)->impl Iterator<Item=&mut V> {self.nodes.iter_mut().filter_map(|node|node.entry.as_mut().map(|(_,value)|value))}
    /// 🪨️ Borrows each live original key and mutable value in native arena order.
    pub fn slot_entries_mut(&mut self)->impl Iterator<Item=(&K,&mut V)>{self.nodes.iter_mut().filter_map(|node|node.entry.as_mut().map(|(key,value)|(&*key,value)))}
}
/// 🌿️ A lazy Copy-key lookup keeps the original index borrowed until its value is available.
pub struct HistoryFoldIndexEntry<'a,K:Ord+Copy,V>{owner:&'a mut HistoryFoldIndex<K,V>,key:K,slot:Option<usize>}
impl<'a,K:Ord+Copy,V> HistoryFoldIndexEntry<'a,K,V>{
    /// 🍃️ Mutates only an existing value while preserving the same source row.
    pub fn and_modify(self,update:impl FnOnce(&mut V))->Self{if let Some(slot)=self.slot{update(&mut self.owner.nodes[slot].entry.as_mut().unwrap().1)}self}
    /// 🌼️ Invokes the constructor only for an actually vacant original row.
    pub fn or_insert_with(self,create:impl FnOnce()->V)->&'a mut V{let slot=match self.slot{Some(slot)=>slot,None=>{let(root,previous)=self.owner.insert_at(self.owner.root,self.key,create());assert!(previous.is_none(),"vacant original row already occupied");self.owner.root=Some(root);self.owner.nodes.len()-1}};&mut self.owner.nodes[slot].entry.as_mut().unwrap().1}
    /// 🌾️ Creates an empty value only when the original row is vacant.
    pub fn or_default(self)->&'a mut V where V:Default{self.or_insert_with(V::default)}
}
/// 👁️ Sorted live entry traversal borrows the original arena without allocation.
pub struct HistoryFoldIndexIter<'a,K,V>{owner:&'a HistoryFoldIndex<K,V>,ranks:std::ops::Range<usize>}
impl<'a,K:Ord,V> Iterator for HistoryFoldIndexIter<'a,K,V>{type Item=(&'a K,&'a V);fn next(&mut self)->Option<Self::Item>{self.ranks.next().and_then(|rank|self.owner.ranked(rank))}fn size_hint(&self)->(usize,Option<usize>){self.ranks.size_hint()}}
impl<K:Ord,V> DoubleEndedIterator for HistoryFoldIndexIter<'_,K,V>{fn next_back(&mut self)->Option<Self::Item>{self.ranks.next_back().and_then(|rank|self.owner.ranked(rank))}}
impl<K:Ord,V> ExactSizeIterator for HistoryFoldIndexIter<'_,K,V>{}
impl<'a,K:Ord,V> IntoIterator for &'a HistoryFoldIndex<K,V>{type Item=(&'a K,&'a V);type IntoIter=HistoryFoldIndexIter<'a,K,V>;fn into_iter(self)->Self::IntoIter{self.iter()}}

impl<K:Ord,V> FromIterator<(K,V)> for HistoryFoldIndex<K,V>{fn from_iter<T:IntoIterator<Item=(K,V)>>(entries:T)->Self{let mut index=Self::new();for(key,value)in entries{index.insert(key,value);}index}}
impl<K:Ord,V,const N:usize> From<[(K,V);N]> for HistoryFoldIndex<K,V>{fn from(entries:[(K,V);N])->Self{entries.into_iter().collect()}}
impl<K:Ord,V> Extend<(K,V)> for HistoryFoldIndex<K,V>{fn extend<T:IntoIterator<Item=(K,V)>>(&mut self,entries:T){for(key,value)in entries{self.insert(key,value);}}}
/// 🔎️ Original sorted ranks selected by borrowed bounds without a copied directory.
impl<K:Ord,V> HistoryFoldIndex<K,V> {
    pub fn first_key_value(&self)->Option<(&K,&V)> { self.ranked(0) }
    fn bound_rank(&self,bound:std::ops::Bound<&K>,upper:bool)->usize {
        use std::ops::Bound;
        let (key,equal_before)=match bound {Bound::Unbounded=>return if upper{self.len()}else{0},Bound::Included(key)=>(key,upper),Bound::Excluded(key)=>(key,!upper)};
        let mut root=self.root;let mut rank=0;
        while let Some(index)=root {let node=&self.nodes[index];let candidate=&node.entry.as_ref().unwrap().0;let order=candidate.cmp(key);if order==Ordering::Less || (order==Ordering::Equal && equal_before) {rank+=self.count(node.left)+1;root=node.right;}else{root=node.left;}}
        rank
    }
    pub fn range<R:std::ops::RangeBounds<K>>(&self,bounds:R)->HistoryFoldIndexIter<'_,K,V> {
        let start=self.bound_rank(bounds.start_bound(),false);let end=self.bound_rank(bounds.end_bound(),true);
        HistoryFoldIndexIter {owner:self,ranks:start..end.max(start)}
    }
}
impl<K:Ord+Borrow<Q>,V,Q:Ord+?Sized> std::ops::Index<&Q> for HistoryFoldIndex<K,V> {type Output=V;fn index(&self,key:&Q)->&V {self.get(key).expect("ordered original key")}}

/// 📦️ Moves original entries while retaining the same arena backing and displaced keys.
pub struct HistoryFoldIndexIntoIter<K:Ord,V> { owner:HistoryFoldIndex<K,V> }
impl<K:Ord,V> Iterator for HistoryFoldIndexIntoIter<K,V> {type Item=(K,V);fn next(&mut self)->Option<Self::Item>{self.owner.pop_first()}fn size_hint(&self)->(usize,Option<usize>){let len=self.owner.len();(len,Some(len))}}
impl<K:Ord,V> ExactSizeIterator for HistoryFoldIndexIntoIter<K,V>{}
impl<K:Ord,V> IntoIterator for HistoryFoldIndex<K,V> {type Item=(K,V);type IntoIter=HistoryFoldIndexIntoIter<K,V>;fn into_iter(self)->Self::IntoIter{HistoryFoldIndexIntoIter{owner:self}}}
impl<K:Ord+RetireOwned,V:RetireOwned> RetireOwned for HistoryFoldIndexIntoIter<K,V> {fn retirement(self)->Box<dyn RetirementCursor>{self.owner.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owner.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{HistoryFoldIndex::<K,V>::controlled_retirement_supported()}}

impl<V:semio_framework_value::ToValue> semio_framework_value::ToValue for HistoryFoldIndex<String,V> {
    fn to_value(&self)->semio_framework_value::DslValue {semio_framework_value::DslValue::Object(self.iter().map(|(key,value)|(key.clone(),value.to_value())).collect())}
}
impl<V:semio_framework_value::FromValue> semio_framework_value::FromValue for HistoryFoldIndex<String,V> {
    fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError> {
        let mut index=Self::new();for(key,value)in value.into_object()? {if index.contains_key(&key){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"duplicate ordered key"));}index.insert(key,V::from_value(value)?);}Ok(index)
    }
}
#[cfg(any(test,feature="history-index-serde"))]
impl<K:Ord+serde::Serialize,V:serde::Serialize> serde::Serialize for HistoryFoldIndex<K,V> {
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error> {use serde::ser::SerializeMap;let mut map=serializer.serialize_map(Some(self.len()))?;for(key,value)in self{map.serialize_entry(key,value)?;}map.end()}
}
#[cfg(any(test,feature="history-index-serde"))]
impl<'de,K:Ord+serde::Deserialize<'de>,V:serde::Deserialize<'de>> serde::Deserialize<'de> for HistoryFoldIndex<K,V> {
    fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
        struct Visitor<K,V>(std::marker::PhantomData<(K,V)>);
        impl<'de,K:Ord+serde::Deserialize<'de>,V:serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<K,V>{type Value=HistoryFoldIndex<K,V>;fn expecting(&self,f:&mut std::fmt::Formatter)->std::fmt::Result{f.write_str("an original ordered map")}fn visit_map<A:serde::de::MapAccess<'de>>(self,mut access:A)->Result<Self::Value,A::Error>{let mut index=HistoryFoldIndex::new();while let Some((key,value))=access.next_entry()?{if index.insert(key,value).is_some(){return Err(serde::de::Error::custom("duplicate ordered key"));}}Ok(index)}}
        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}

impl<K:RetireOwned,V:RetireOwned> RetireOwned for FoldIndexNode<K,V>{fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.entry),deferred(self.left),deferred(self.right),deferred(self.height),deferred(self.count)])}fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.entry),deferred_birth_bytes_for(&self.left),deferred_birth_bytes_for(&self.right),deferred_birth_bytes_for(&self.height),deferred_birth_bytes_for(&self.count)])}fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for HistoryFoldIndex<K,V>{fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.nodes),deferred(self.root),deferred(self.displaced_keys)])}fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.nodes),deferred_birth_bytes_for(&self.root),deferred_birth_bytes_for(&self.displaced_keys)])}fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}}

#[derive(Clone,Debug,PartialEq,Eq)]
pub(super) struct FoldMembership<T:Ord>(HistoryFoldIndex<T,()>);
impl<T:Ord> Default for FoldMembership<T>{fn default()->Self{Self(HistoryFoldIndex::default())}}
impl<T:Ord> FoldMembership<T>{pub(super) fn new()->Self{Self::default()}pub(super) fn insert(&mut self,key:T)->bool{self.0.insert(key,()).is_none()}pub(super) fn contains<Q:Ord+?Sized>(&self,key:&Q)->bool where T:Borrow<Q>{self.0.contains_key(key)}pub(super) fn remove<Q:Ord+?Sized>(&mut self,key:&Q)->bool where T:Borrow<Q>{self.0.remove(key).is_some()}pub(super) fn pop_first(&mut self)->Option<T>{self.0.pop_first().map(|(key,())|key)}pub(super) fn iter(&self)->impl Iterator<Item=&T>{self.0.keys()}}
impl<T:RetireOwned+Ord> RetireOwned for FoldMembership<T>{fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
