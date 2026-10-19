//! 🎟️ Fixed exact-key slot leases isolate registry ownership and reject stale reuse.
use std::sync::atomic::{AtomicU64,Ordering};

static NEXT_ISSUER:AtomicU64=AtomicU64::new(1);

/// 🪪️ Only the issuing registry and current reuse generation may release this slot.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FixedKeySlot{index:usize,generation:u64,issuer:u64}
impl FixedKeySlot{
 /// 🔢️ The caller's parallel fixed arrays use this exact slot position.
 pub fn index(self)->usize{self.index}
 /// 🔁️ Each successful reuse advances the original slot generation.
 pub fn generation(self)->u64{self.generation}
}

/// 🧺️ The registry owns fixed Copy keys, never application payloads or cleanup authority.
pub struct FixedKeySlots<K,const N:usize>{keys:[Option<K>;N],generations:[u64;N],issuer:u64,len:usize}
impl<K:Copy+Eq,const N:usize> FixedKeySlots<K,N>{
 /// 🌱️ Installation creates one fixed registry identity without a heap allocation.
 pub fn new()->Self{Self{keys:[None;N],generations:[0;N],issuer:NEXT_ISSUER.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).expect("fixed registry issuer space exhausted"),len:0}}
 /// 🔎️ Exact key lookup cannot alias another key through a hash bucket.
 pub fn lookup(&self,key:K)->Option<FixedKeySlot>{self.keys.iter().position(|current|*current==Some(key)).map(|index|FixedKeySlot{index,generation:self.generations[index],issuer:self.issuer})}
 /// 🧭️ Lends one declared position's exact key for bounded ownership scans.
 pub fn key_at(&self,index:usize)->Option<K>{self.keys.get(index).copied().flatten()}
 /// 🔭️ Admission can quote payload birth before claiming a reusable exact position.
 pub fn can_claim(&self,key:K)->bool{self.lookup(key).is_some()||self.keys.iter().enumerate().any(|(index,current)|current.is_none()&&self.generations[index]<u64::MAX)}
 /// 🎫️ Admission reuses an existing exact key or a free declared slot, without evicting an owner.
 pub fn claim(&mut self,key:K)->Option<FixedKeySlot>{if let Some(slot)=self.lookup(key){return Some(slot);}let index=self.keys.iter().enumerate().position(|(index,current)|current.is_none()&&self.generations[index]<u64::MAX)?;self.generations[index]+=1;self.keys[index]=Some(key);self.len+=1;Some(FixedKeySlot{index,generation:self.generations[index],issuer:self.issuer})}
 /// ♻️ The caller releases a claim only after its original child owners have returned.
 pub fn release(&mut self,key:K,slot:FixedKeySlot)->bool{if slot.issuer!=self.issuer||self.keys.get(slot.index)!=Some(&Some(key))||self.generations.get(slot.index)!=Some(&slot.generation){return false;}self.keys[slot.index]=None;self.len-=1;true}
 /// 📊️ The occupied count is independent of key spelling and magnitude.
 pub fn len(&self)->usize{self.len}
 /// 🫙️ No claimed key remains in the fixed registry.
 pub fn is_empty(&self)->bool{self.len==0}
}
impl<K:Copy+Eq,const N:usize> Default for FixedKeySlots<K,N>{fn default()->Self{Self::new()}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
