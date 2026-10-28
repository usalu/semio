//! 📙️ Editable history drafts retain replaced inputs and original ordered index scaffolds.
use super::{HistoryFoldIndex,HistoryFoldIndexIter,InputReplacement,MutationId};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, deferred, deferred_birth_bytes_for, sequence, sequence_birth_bytes};
use semio_framework_value_persistent::list::PagedList;
use std::{borrow::Borrow,ops::RangeBounds};

/// 📝️ Current inputs reference original rows; superseded rows remain owned until controlled retirement.
#[derive(Clone,Debug,Default)]
pub struct HistoryInputDrafts {
    epoch:u64,
    index:HistoryFoldIndex<MutationId,usize>,
    rows:PagedList<Option<InputReplacement>,{usize::MAX}>,
}
impl HistoryInputDrafts {
    pub fn new()->Self {Self::default()}
    pub fn len(&self)->usize {self.index.len()}
    pub fn is_empty(&self)->bool {self.index.is_empty()}
    /// 🔎️ Borrows one sorted original input through its existing ranked tree without scanning preceding rows.
    pub fn at_rank(&self,rank:usize)->Option<(&MutationId,&InputReplacement)>{self.index.ranked(rank).and_then(|(key,row)|self.rows.get(*row).and_then(Option::as_ref).map(|input|(key,input)))}
    /// 🪪️ Pure codec cursors reject a draft edit between bounded field reads.
    pub fn mutation_epoch(&self)->u64{self.epoch}

    /// 🪹️ Physical emptiness requires both original indexes and all replaced input rows to be released.
    pub fn terminal_is_empty(&self)->bool {self.index.terminal_is_empty()&&self.rows.terminal_is_empty()}
    pub fn get<Q:Ord+?Sized>(&self,key:&Q)->Option<&InputReplacement> where MutationId:Borrow<Q> {self.index.get(key).and_then(|row|self.rows.get(*row)).and_then(Option::as_ref)}
    /// 🪡️ A mutable original input invalidates previously borrowed wire coordinates.
    pub fn get_mut<Q:Ord+?Sized>(&mut self,key:&Q)->Option<&mut InputReplacement> where MutationId:Borrow<Q>{let row=*self.index.get(key)?;self.epoch=self.epoch.checked_add(1).expect("original draft mutation epoch exhausted");self.rows.get_mut(row).and_then(Option::as_mut)}
    pub fn contains_key<Q:Ord+?Sized>(&self,key:&Q)->bool where MutationId:Borrow<Q> {self.index.contains_key(key)}
    /// 🌱️ Cold construction retains the previous row instead of dropping its original schema and payload.
    pub fn insert(&mut self,key:MutationId,input:InputReplacement)->Option<&InputReplacement> {
        self.epoch=self.epoch.checked_add(1).expect("original draft mutation epoch exhausted");let row=self.rows.len();self.rows.push(Some(input));self.index.insert(key,row).and_then(|previous|self.rows.get(previous)).and_then(Option::as_ref)
    }
    /// 📦️ Transfers a current original input without releasing any index or row backing.
    pub fn pop_first(&mut self)->Option<(MutationId,InputReplacement)> {let(key,row)=self.index.pop_first()?;self.epoch=self.epoch.checked_add(1).expect("original draft mutation epoch exhausted");Some((key,self.rows.get_mut(row).expect("original input row").take().expect("current original input")))}
    /// 📏️ Prices the original sorted index path and retained payload-row handoff without copying its bytes.
    pub fn take_first_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let Some((key,row))=self.index.ranked(0)else{return Ok(Default::default())};let depth=self.index.next_contains_depth_demand(key)?.max(self.rows.next_get_depth_demand(*row).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original draft payload page depth overflow"))?).saturating_add(1);Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<(MutationId,InputReplacement)>()+3*std::mem::size_of::<usize>(),depth,..Default::default()})}
    /// 🤝️ Moves the same original key and replacement only under all independently admitted currencies.
    pub fn take_first(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<((MutationId,InputReplacement),semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{if self.is_empty(){return Ok(None)}let d=self.take_first_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}Ok(Some((self.pop_first().expect("admitted original draft row"),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
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
