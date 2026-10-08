//! 📝️ Editable history drafts retain replaced inputs and original ordered index scaffolds.
use super::{HistoryFoldIndex,HistoryFoldIndexIter,InputReplacement,MutationId};
use semio_framework_value::{list::PagedList,retirement::{RetireOwned,RetirementCursor,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes}};
use std::{borrow::Borrow,ops::RangeBounds};

/// 📝️ Current inputs reference original rows; superseded rows remain owned until controlled retirement.
#[derive(Clone,Debug,Default)]
pub struct HistoryInputDrafts {
    index:HistoryFoldIndex<MutationId,usize>,
    rows:PagedList<Option<InputReplacement>,{usize::MAX}>,
}
impl HistoryInputDrafts {
    pub fn new()->Self {Self::default()}
    pub fn len(&self)->usize {self.index.len()}
    pub fn is_empty(&self)->bool {self.index.is_empty()}
    /// 🪹️ Physical emptiness requires both original indexes and all replaced input rows to be released.
    pub fn terminal_is_empty(&self)->bool {self.index.terminal_is_empty()&&self.rows.terminal_is_empty()}
    pub fn get<Q:Ord+?Sized>(&self,key:&Q)->Option<&InputReplacement> where MutationId:Borrow<Q> {self.index.get(key).and_then(|row|self.rows.get(*row)).and_then(Option::as_ref)}
    pub fn contains_key<Q:Ord+?Sized>(&self,key:&Q)->bool where MutationId:Borrow<Q> {self.index.contains_key(key)}
    /// 🌱️ Cold construction retains the previous row instead of dropping its original schema and payload.
    pub fn insert(&mut self,key:MutationId,input:InputReplacement)->Option<&InputReplacement> {
        let row=self.rows.len();self.rows.push(Some(input));self.index.insert(key,row).and_then(|previous|self.rows.get(previous)).and_then(Option::as_ref)
    }
    /// 📦️ Transfers a current original input without releasing any index or row backing.
    pub fn pop_first(&mut self)->Option<(MutationId,InputReplacement)> {let(key,row)=self.index.pop_first()?;Some((key,self.rows.get_mut(row).expect("original input row").take().expect("current original input")))}
    pub fn iter(&self)->HistoryInputDraftIter<'_> {HistoryInputDraftIter {rows:&self.rows,index:self.index.iter()}}
    pub fn range<R:RangeBounds<MutationId>>(&self,bounds:R)->HistoryInputDraftIter<'_> {HistoryInputDraftIter {rows:&self.rows,index:self.index.range(bounds)}}
    pub fn keys(&self)->impl DoubleEndedIterator<Item=&MutationId>+ExactSizeIterator {self.iter().map(|(key,_)|key)}
    pub fn values(&self)->impl DoubleEndedIterator<Item=&InputReplacement>+ExactSizeIterator {self.iter().map(|(_,input)|input)}
}
impl PartialEq for HistoryInputDrafts {fn eq(&self,other:&Self)->bool {self.len()==other.len()&&self.iter().eq(other.iter())}}
impl Eq for HistoryInputDrafts {}
impl FromIterator<(MutationId,InputReplacement)> for HistoryInputDrafts {fn from_iter<T:IntoIterator<Item=(MutationId,InputReplacement)>>(entries:T)->Self {let mut drafts=Self::new();drafts.extend(entries);drafts}}
impl<const N:usize> From<[(MutationId,InputReplacement);N]> for HistoryInputDrafts {fn from(entries:[(MutationId,InputReplacement);N])->Self {entries.into_iter().collect()}}
impl Extend<(MutationId,InputReplacement)> for HistoryInputDrafts {fn extend<T:IntoIterator<Item=(MutationId,InputReplacement)>>(&mut self,entries:T){for(key,input)in entries {self.insert(key,input);}}}
impl<Q:Ord+?Sized> std::ops::Index<&Q> for HistoryInputDrafts where MutationId:Borrow<Q> {type Output=InputReplacement;fn index(&self,key:&Q)->&InputReplacement {self.get(key).expect("current original draft")}}

/// 👁️ Ordered traversal borrows the original index and original input row without allocation.
pub struct HistoryInputDraftIter<'a> {rows:&'a PagedList<Option<InputReplacement>,{usize::MAX}>,index:HistoryFoldIndexIter<'a,MutationId,usize>}
impl<'a> Iterator for HistoryInputDraftIter<'a> {type Item=(&'a MutationId,&'a InputReplacement);fn next(&mut self)->Option<Self::Item> {self.index.next().map(|(key,row)|(key,self.rows.get(*row).and_then(Option::as_ref).expect("current original draft row")))}fn size_hint(&self)->(usize,Option<usize>){self.index.size_hint()}}
impl DoubleEndedIterator for HistoryInputDraftIter<'_> {fn next_back(&mut self)->Option<Self::Item> {self.index.next_back().map(|(key,row)|(key,self.rows.get(*row).and_then(Option::as_ref).expect("current original draft row")))}}
impl ExactSizeIterator for HistoryInputDraftIter<'_> {}
impl<'a> IntoIterator for &'a HistoryInputDrafts {type Item=(&'a MutationId,&'a InputReplacement);type IntoIter=HistoryInputDraftIter<'a>;fn into_iter(self)->Self::IntoIter {self.iter()}}
impl RetireOwned for HistoryInputDrafts {
    fn retirement(self)->Box<dyn RetirementCursor> {sequence(vec![deferred(self.index),deferred(self.rows)])}
    fn retirement_birth_bytes(&self)->Option<usize> {sequence_birth_bytes(&[deferred_birth_bytes_for(&self.index),deferred_birth_bytes_for(&self.rows)])}
    fn controlled_retirement_supported()->bool {true}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
