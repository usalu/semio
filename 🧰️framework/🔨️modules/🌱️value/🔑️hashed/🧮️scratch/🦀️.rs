//! 🗂️ Fixed scratch hash owners expose exact vector layouts and retain tombstones without hidden table growth.
use std::{borrow::Borrow,hash::{Hash,Hasher},mem::size_of};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor},retained_clone::RetainedCloneProgress};
#[derive(Debug)]
enum Slot<K,V>{Empty,Deleted,Occupied((K,V))}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for Slot<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Occupied(value)=>value.retirement(),_=>().retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Occupied(value)=>value.retirement_birth_bytes(),_=>().retirement_birth_bytes()}}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}
/// 🗺️ A declared scratch table allocates one known vector and never grows during lookup or insertion.
#[derive(Debug)]
pub struct ScratchHashMap<K,V>{slots:Vec<Slot<K,V>>,maximum:usize,len:usize}
impl<K,V> Default for ScratchHashMap<K,V>{fn default()->Self{Self{slots:Vec::new(),maximum:0,len:0}}}
impl<K,V> ScratchHashMap<K,V>{
 pub const MAXIMUM_ITEMS:usize=1_048_576;
 fn slots_for(maximum:usize)->Result<usize,ValueError>{if maximum>Self::MAXIMUM_ITEMS{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"scratch hash maximum exceeds its fixed contract"))}Ok(if maximum==0{0}else{(maximum*2).next_power_of_two()})}
 /// 📏️ One concrete vector layout prices every initialized vacancy before construction.
 pub fn allocation_demands(maximum:usize)->Result<RetirementDemand,ValueError>{let slots=Self::slots_for(maximum)?;let bytes=slots.checked_mul(size_of::<Slot<K,V>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"scratch hash layout overflow"))?;Ok(RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()})}
 /// 🧺️ A caller funds the quoted vector before invoking this fallible single-allocation constructor.
 pub fn try_with_capacity(maximum:usize)->Result<Self,ValueError>{let count=Self::slots_for(maximum)?;let mut slots=Vec::new();slots.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"scratch hash vector allocation failed"))?;slots.resize_with(count,||Slot::Empty);Ok(Self{slots,maximum,len:0})}
 pub fn allocation_progress(&self)->RetainedCloneProgress{RetainedCloneProgress{copied_bytes:self.slots.len()*size_of::<Slot<K,V>>(),retained_capacity_bytes:self.slots.capacity()*size_of::<Slot<K,V>>(),..Default::default()}}
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
 pub fn iter(&self)->impl Iterator<Item=(&K,&V)>{self.slots.iter().filter_map(|slot|match slot{Slot::Occupied((key,value))=>Some((key,value)),_=>None})}
 pub fn keys(&self)->impl Iterator<Item=&K>{self.iter().map(|(key,_)|key)}
}
impl<K:Hash+Eq,V> ScratchHashMap<K,V>{
 fn hash<Q:Hash+?Sized>(key:&Q)->usize{let mut state=std::collections::hash_map::DefaultHasher::new();key.hash(&mut state);state.finish()as usize}
 fn find<Q:Hash+Eq+?Sized>(&self,key:&Q)->Option<usize> where K:Borrow<Q>{if self.slots.is_empty(){return None}let mask=self.slots.len()-1;let mut at=Self::hash(key)&mask;for _ in 0..self.slots.len(){match &self.slots[at]{Slot::Empty=>return None,Slot::Occupied((candidate,_))if candidate.borrow()==key=>return Some(at),_=>{}}at=(at+1)&mask;}None}
 pub fn get<Q:Hash+Eq+?Sized>(&self,key:&Q)->Option<&V> where K:Borrow<Q>{match &self.slots[self.find(key)?]{Slot::Occupied((_,value))=>Some(value),_=>None}}
 pub fn contains_key<Q:Hash+Eq+?Sized>(&self,key:&Q)->bool where K:Borrow<Q>{self.find(key).is_some()}
 /// 🫴️ Full-capacity refusal returns both original operands without changing any retained slot.
 pub fn try_insert(&mut self,key:K,value:V)->Result<Option<V>,(K,V)>{if let Some(at)=self.find(&key){if let Slot::Occupied((_,previous))=&mut self.slots[at]{return Ok(Some(std::mem::replace(previous,value)))}}if self.len==self.maximum{return Err((key,value))}let mask=self.slots.len()-1;let mut at=Self::hash(&key)&mask;for _ in 0..self.slots.len(){if !matches!(self.slots[at],Slot::Occupied(_)){self.slots[at]=Slot::Occupied((key,value));self.len+=1;return Ok(None)}at=(at+1)&mask;}unreachable!("scratch hash vacancy invariant violated")}
 /// 🔐️ Bounded scratch algorithms prove their declared maximum before this infallible insertion.
 pub fn insert(&mut self,key:K,value:V)->Option<V>{self.try_insert(key,value).unwrap_or_else(|_|panic!("scratch hash declared maximum exhausted"))}
 pub fn remove<Q:Hash+Eq+?Sized>(&mut self,key:&Q)->Option<V> where K:Borrow<Q>{let at=self.find(key)?;match std::mem::replace(&mut self.slots[at],Slot::Deleted){Slot::Occupied((_,value))=>{self.len-=1;Some(value)},_=>unreachable!()}}
}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for ScratchHashMap<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{self.slots.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.slots.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{<Vec<Slot<K,V>>>::controlled_retirement_supported()}
}
/// 🔏️ A set shares the same one-vector admission and tombstone custody as its map.
#[derive(Debug)]
pub struct ScratchHashSet<K>{map:ScratchHashMap<K,()>}
impl<K> Default for ScratchHashSet<K>{fn default()->Self{Self{map:ScratchHashMap::default()}}}
impl<K> ScratchHashSet<K>{
 pub fn allocation_demands(maximum:usize)->Result<RetirementDemand,ValueError>{ScratchHashMap::<K,()>::allocation_demands(maximum)}
 pub fn try_with_capacity(maximum:usize)->Result<Self,ValueError>{ScratchHashMap::try_with_capacity(maximum).map(|map|Self{map})}
 pub fn allocation_progress(&self)->RetainedCloneProgress{self.map.allocation_progress()}
 pub fn len(&self)->usize{self.map.len()}
 pub fn is_empty(&self)->bool{self.map.is_empty()}
 pub fn iter(&self)->impl Iterator<Item=&K>{self.map.keys()}
}
impl<K:Hash+Eq> ScratchHashSet<K>{
 pub fn insert(&mut self,key:K)->bool{self.map.insert(key,()).is_none()}
 pub fn contains<Q:Hash+Eq+?Sized>(&self,key:&Q)->bool where K:Borrow<Q>{self.map.contains_key(key)}
 pub fn remove<Q:Hash+Eq+?Sized>(&mut self,key:&Q)->bool where K:Borrow<Q>{self.map.remove(key).is_some()}
}
impl<K:RetireOwned> RetireOwned for ScratchHashSet<K>{
 fn retirement(self)->Box<dyn RetirementCursor>{self.map.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.map.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{ScratchHashMap::<K,()>::controlled_retirement_supported()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
