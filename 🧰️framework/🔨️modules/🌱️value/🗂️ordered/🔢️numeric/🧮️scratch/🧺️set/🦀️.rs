//! 🧺️ Ranked fixed-width membership shares the original numeric page owner.
use super::{NumericIndex,NumericInsertCursor};
use crate::retirement::{RetireOwned,RetirementCursor};

pub struct NumericSet<K>{pub(super)index:NumericIndex<K,()>}
impl<K> Default for NumericSet<K>{fn default()->Self{Self{index:NumericIndex::default()}}}
impl<K:Copy+Ord> NumericSet<K>{
 pub fn new()->Self{Self::default()}
 pub fn len(&self)->usize{self.index.len()}
 pub fn is_empty(&self)->bool{self.index.is_empty()}
 pub fn contains(&self,key:&K)->bool{self.index.contains_key(key)}
 pub fn insert(&mut self,key:K)->bool{self.index.insert(key,()).is_none()}
 pub fn remove(&mut self,key:&K)->bool{self.index.remove(key).is_some()}
 pub fn clear(&mut self){self.index.reset();}
 pub fn pop_first(&mut self)->Option<K>{self.index.pop_first().map(|(key,())|key)}
 pub fn iter(&self)->NumericSetIter<'_,K>{NumericSetIter{set:self,front:0,back:self.len()}}
 pub fn key_at_rank(&self,rank:usize)->Option<&K>{self.index.entry_at_rank(rank).map(|(key,())|key)}
 pub fn begin_insert(&self,key:K)->NumericInsertCursor<K,()>{NumericInsertCursor::new(key,())}
 pub fn insert_step(&mut self,cursor:&mut NumericInsertCursor<K,()>,units:usize,control:&mut crate::NativeDecodeControl<'_>)->Result<bool,crate::ValueError>{cursor.step(&mut self.index,units,control)}
 pub fn terminal_is_empty(&self)->bool{self.index.terminal_is_empty()}
 pub fn insert_capacity_byte_demand(&self,cursor:&super::NumericInsertCursor<K,()>)->Result<usize,crate::ValueError>{cursor.next_capacity_byte_demand(&self.index)}
 pub fn insert_depth_demand(&self,cursor:&super::NumericInsertCursor<K,()>)->Result<usize,crate::ValueError>{cursor.next_depth_demand(&self.index)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,crate::list::PagedListError>{self.index.next_close_release_byte_demand()}
 pub fn close_copy_step(&mut self,maximum_items:usize,maximum_release_bytes:usize)->Result<crate::list::PagedListProgress,crate::list::PagedListError>{self.index.close_copy_step(maximum_items,maximum_release_bytes)}
}
impl<K:Copy+Ord> Clone for NumericSet<K>{fn clone(&self)->Self{self.iter().copied().collect()}}
impl<K:Copy+Ord+std::fmt::Debug> std::fmt::Debug for NumericSet<K>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_set().entries(self.iter()).finish()}}
impl<K:Copy+Ord> PartialEq for NumericSet<K>{fn eq(&self,other:&Self)->bool{self.iter().eq(other.iter())}}
impl<K:Copy+Ord> Eq for NumericSet<K>{}
impl<K:Copy+Ord> FromIterator<K> for NumericSet<K>{fn from_iter<T:IntoIterator<Item=K>>(values:T)->Self{let mut set=Self::new();for value in values{set.insert(value);}set}}
impl<K:Copy+Ord> Extend<K> for NumericSet<K>{fn extend<T:IntoIterator<Item=K>>(&mut self,values:T){for value in values{self.insert(value);}}}
impl<K:Copy+Ord,const N:usize> From<[K;N]> for NumericSet<K>{fn from(values:[K;N])->Self{values.into_iter().collect()}}
impl<'a,K:Copy+Ord> IntoIterator for &'a NumericSet<K>{type Item=&'a K;type IntoIter=NumericSetIter<'a,K>;fn into_iter(self)->Self::IntoIter{self.iter()}}
pub struct NumericSetIter<'a,K>{set:&'a NumericSet<K>,front:usize,back:usize}
impl<'a,K:Copy+Ord> Iterator for NumericSetIter<'a,K>{type Item=&'a K;fn next(&mut self)->Option<Self::Item>{if self.front==self.back{return None}let key=self.set.index.entry_at_rank(self.front).map(|(key,())|key);self.front+=1;key}fn size_hint(&self)->(usize,Option<usize>){let remaining=self.back-self.front;(remaining,Some(remaining))}}
impl<K:Copy+Ord> DoubleEndedIterator for NumericSetIter<'_,K>{fn next_back(&mut self)->Option<Self::Item>{if self.front==self.back{return None}self.back-=1;self.set.index.entry_at_rank(self.back).map(|(key,())|key)}}
impl<K:Copy+Ord> ExactSizeIterator for NumericSetIter<'_,K>{}
impl<K:RetireOwned> RetireOwned for NumericSet<K>{fn retirement(self)->Box<dyn RetirementCursor>{self.index.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.index.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()}}
