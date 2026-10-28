//! 📇️ The Draw index stores only original paged keys, original node/asset loans and fixed graph marks.
use super::*;
use semio_framework_value::scratch_hash::{ScratchHashMap,ScratchHashSlots};
use std::sync::atomic::{AtomicU8,AtomicUsize};
use super::source::{PagedHash,PagedCompare};
#[derive(Clone,Copy)]pub(super) enum Target{Layer(&'static crate::DrawingLayerNode),Asset(&'static crate::DrawingImageAsset)}
pub(super) struct Entry{pub key:&'static Text,pub target:Target,pub mark:AtomicU8,pub parent:AtomicUsize,pub edge:AtomicUsize}
impl Entry{fn new(key:&'static Text,target:Target)->Self{Self{key,target,mark:AtomicU8::new(0),parent:AtomicUsize::new(usize::MAX),edge:AtomicUsize::new(0)}}}
pub(super) type Slots<'a>=ScratchHashSlots<'a,(u8,u64),Entry>;
pub(super) fn start(kind:u8,hash:u64,length:usize)->usize{(hash^(kind as u64).wrapping_mul(0x9e3779b97f4a7c15))as usize&(length-1)}
pub(super) struct Index{map:ScratchHashMap<(u8,u64),Entry>,pending:Option<(u8,&'static Text,Target)>,hash:PagedHash,comparison:PagedCompare,probe:Option<usize>}
impl Default for Index{fn default()->Self{Self{map:Default::default(),pending:None,hash:Default::default(),comparison:Default::default(),probe:None}}}
impl Index{
 pub fn reserve_demand(count:usize)->Result<RetirementDemand,ValueError>{ScratchHashMap::<(u8,u64),Entry>::reservation_demands(count)}
 pub fn reserve(&mut self,count:usize,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=Self::reserve_demand(count)?;if !retained_clone_grant_admits(g,d){return Ok(Default::default())}self.map=ScratchHashMap::try_reserve_uninitialized(count)?;let p=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.map.backing_bytes(),..Default::default()};if p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault("Draw index reservation exceeded original grant").with_retained_progress(p))}Ok(p)}
 pub fn initialized(&self)->bool{self.map.initialized_slots()==self.map.required_slots().unwrap_or(usize::MAX)}
 pub fn initialize_demand()->RetirementDemand{RetirementDemand{copy_bytes:ScratchHashMap::<(u8,u64),Entry>::slot_bytes(),depth:1,..Default::default()}}
 pub fn initialize(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=Self::initialize_demand();if !retained_clone_grant_admits(g,d){return Ok(Default::default())}let changed=self.map.initialize_one()?;Ok(RetainedCloneProgress{copied_items:usize::from(changed),copied_bytes:if changed{d.copy_bytes}else{0},..Default::default()})}
 pub fn begin_key(&mut self,kind:u8,key:&'static Text,target:Target){self.pending=Some((kind,key,target));self.hash=Default::default();self.comparison=Default::default();self.probe=None}
 pub fn pending(&self)->bool{self.pending.is_some()}
 pub fn step(&mut self)->Option<&'static str>{let(kind,key,target)=self.pending.expect("selected original Draw key");if key.is_empty(){return Some("Drawing identities must be non-empty and unique within their namespace.")}if self.probe.is_none(){if self.hash.step(key){self.probe=Some(start(kind,self.hash.hash,self.map.initialized_slots()))}return None}let at=self.probe.unwrap();let slots=self.map.slots();if slots.is_vacant(at){self.map.insert_vacant(at,(kind,self.hash.hash),Entry::new(key,target)).unwrap_or_else(|_|unreachable!("admitted Draw index slot"));self.pending=None;return None}let(held,entry)=slots.get(at).expect("original occupied Draw slot");if *held==(kind,self.hash.hash){match self.comparison.step(key,entry.key){Some(true)=>return Some("Drawing identities must be non-empty and unique within their namespace."),None=>return None,Some(false)=>{}}}self.probe=Some((at+1)&(slots.len()-1));self.comparison=Default::default();None}
 pub fn slots(&self)->Slots<'_>{self.map.slots()}
 pub fn close_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:ScratchHashMap::<(u8,u64),Entry>::slot_bytes(),release_bytes:if self.pending.is_none()&&self.map.initialized_slots()==0{self.map.backing_bytes()}else{0},depth:1,..Default::default()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.close_demand();if !retained_clone_grant_admits(g,d){return Ok(Default::default())}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if self.pending.take().is_some(){return Ok(p)}if self.map.pop_last_slot().is_none(){p.released_bytes=self.map.release_empty_backing()?}Ok(p)}
 pub fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.map.initialized_slots()==0&&self.map.backing_bytes()==0}
}
