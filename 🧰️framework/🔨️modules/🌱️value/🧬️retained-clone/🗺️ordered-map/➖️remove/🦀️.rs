//! ➖️ Removes an unpublished owned entry without releasing ungranted backing.

use super::*;
use std::mem::ManuallyDrop;

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedOrderedMapRemoveGrant { pub retirement:RetainedCloneGrant,pub comparison:BoundedOrdGrant,pub maximum_moved_items:usize,pub maximum_moved_bytes:usize }
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedOrderedMapRemoveProgress { pub retirement:RetainedCloneProgress,pub comparison:BoundedOrdProgress,pub moved_items:usize,pub moved_bytes:usize }
impl RetainedOrderedMapRemoveProgress { pub fn fits(self,grant:RetainedOrderedMapRemoveGrant)->bool {self.retirement.fits(grant.retirement)&&self.comparison.fits(grant.comparison)&&self.moved_items<=grant.maximum_moved_items&&self.moved_bytes<=grant.maximum_moved_bytes} }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedOrderedMapRemoveStep { Progress(RetainedOrderedMapRemoveProgress),Complete{removed:bool,progress:RetainedOrderedMapRemoveProgress} }
impl RetainedOrderedMapRemoveStep {pub fn progress(self)->RetainedOrderedMapRemoveProgress{match self{Self::Progress(p)|Self::Complete{progress:p,..}=>p}}}

#[derive(crate::RetireOwned)]
struct RemoveWorkspace<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{map:RetainedOrderedMap<K,V>,key:K}
struct State<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> {map:Option<RetainedOrderedMap<K,V>>,key:Option<K>,removed:Option<(K,V)>,page:Option<Vec<(K,V)>>,lookup:RetainedOrderedMapLookupCursor<K>,lease:super::super::RetainedCloneBorrowAuthority<RemoveWorkspace<K,V>>,target:Option<usize>,shift:usize,phase:u8,found:bool,closing:bool,spent:bool,close:RetainedCloneClose}
/// ➖️ Retains the original map, search key and physical leaf owners through every refused turn.
pub struct RetainedOrderedMapRemoveCursor<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> {state:ManuallyDrop<State<K,V>>}
impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> RetainedOrderedMapRemoveCursor<K,V> {
 pub fn constructor_capacity_bytes()->usize {super::super::RetainedCloneBorrowAuthority::<RemoveWorkspace<K,V>>::constructor_capacity_bytes()}
 pub fn constructor_copy_bytes()->usize{super::super::RetainedCloneBorrowAuthority::<RemoveWorkspace<K,V>>::constructor_copy_bytes()}
 pub fn admit(map:RetainedOrderedMap<K,V>,key:K,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(crate::ValueError,RetainedOrderedMap<K,V>,K)> {
  if !K::controlled_retirement_supported()||!V::controlled_retirement_supported(){return Err((crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"removal requires supported original owner facets"),map,key))}
  if grant.maximum_items==0{return Err((crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"removal constructor requires an item"),map,key))}
  let(lease,receipt)=match super::super::RetainedCloneBorrowAuthority::admit(RemoveWorkspace{map,key},grant){Ok(result)=>result,Err((error,RemoveWorkspace{map,key}))=>return Err((error,map,key))};
  Ok((Self{state:ManuallyDrop::new(State{map:None,key:None,removed:None,page:None,lookup:Default::default(),lease,target:None,shift:0,phase:0,found:false,closing:false,spent:false,close:Default::default()})},receipt))
 }
 pub fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
  let s=&*self.state;if s.phase==0{return s.lookup.advance_retirement_demands(body);}if s.phase!=1{return Ok(Default::default());}
  if !s.lookup.terminal_is_empty(){return Ok(crate::RetirementDemand{copy_bytes:s.lookup.next_close_copy_byte_demand()?,capacity_bytes:s.lookup.next_close_capacity_byte_demand(body)?,release_bytes:s.lookup.next_close_release_byte_demand()?,depth:s.lookup.next_close_depth_demand()?});}
  Ok(crate::RetirementDemand{copy_bytes:s.lease.next_take_copy_byte_demand()?,capacity_bytes:s.lease.next_take_capacity_byte_demand(body)?,release_bytes:s.lease.next_take_release_byte_demand()?,depth:s.lease.next_take_depth_demand()?})
 }
 pub fn advance(&mut self,grant:RetainedOrderedMapRemoveGrant)->Result<RetainedOrderedMapRemoveStep,crate::ValueError>{
  let s=&mut *self.state;if s.closing||s.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"removal cursor is closing or spent"))}
  let mut progress=RetainedOrderedMapRemoveProgress::default();
  match s.phase {
   0=>{let(result,comparison,retirement)=s.lookup.advance(s.lease.borrow(1,|workspace|&workspace.map),s.lease.borrow(2,|workspace|&workspace.key),grant.comparison,grant.retirement)?;progress.comparison=comparison;progress.retirement=retirement;if let Some(result)=result {s.found=matches!(result,RetainedOrderedMapLookup::Found(_));s.target=match result{RetainedOrderedMapLookup::Found(index)=>Some(index),_=>None};s.lookup.begin_close();s.phase=1;}}
   1=>{if !s.lookup.terminal_is_empty(){progress.retirement=s.lookup.close_step(grant.retirement)?.progress();}else{let demand=crate::RetirementDemand{copy_bytes:s.lease.next_take_copy_byte_demand()?,capacity_bytes:s.lease.next_take_capacity_byte_demand(grant.retirement.maximum_copy_bytes)?,release_bytes:s.lease.next_take_release_byte_demand()?,depth:s.lease.next_take_depth_demand()?};if grant.retirement.maximum_items==0||demand.copy_bytes>grant.retirement.maximum_copy_bytes||demand.capacity_bytes>grant.retirement.maximum_capacity_bytes||demand.release_bytes>grant.retirement.maximum_release_bytes||demand.depth>grant.retirement.maximum_depth{return Ok(RetainedOrderedMapRemoveStep::Progress(Default::default()))}match s.lease.take_authority(grant.retirement)?{super::super::RetainedCloneSourceTake::Pending(retirement)=>progress.retirement=retirement,super::super::RetainedCloneSourceTake::Ready(RemoveWorkspace{map,key},retirement)=>{s.map=Some(map);s.key=Some(key);s.phase=if s.found{2}else{5};progress.retirement=retirement;}}}}
   2=>{let map=s.map.as_mut().unwrap();let index=s.target.unwrap();let page=index/RETAINED_ORDERED_MAP_PAGE_CAPACITY;let slot=index%RETAINED_ORDERED_MAP_PAGE_CAPACITY;progress.moved_items=map.pages[page].len()-slot;progress.moved_bytes=progress.moved_items.checked_mul(size_of::<(K,V)>()).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"removal movement overflow"))?;if !progress.fits(grant){return Ok(RetainedOrderedMapRemoveStep::Progress(Default::default()))}s.removed=Some(map.pages[page].remove(slot));s.shift=page;s.phase=3;}
   3=>{let map=s.map.as_mut().unwrap();if s.shift+1<map.pages.len(){progress.moved_items=map.pages[s.shift+1].len()+1;progress.moved_bytes=progress.moved_items.checked_mul(size_of::<(K,V)>()).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"removal page movement overflow"))?;if !progress.fits(grant){return Ok(RetainedOrderedMapRemoveStep::Progress(Default::default()))}let entry=map.pages[s.shift+1].remove(0);map.pages[s.shift].push(entry);s.shift+=1;}else{s.phase=4;}}
   4=>{progress.moved_items=1;progress.moved_bytes=size_of::<usize>()+size_of::<Vec<(K,V)>>();if !progress.fits(grant){return Ok(RetainedOrderedMapRemoveStep::Progress(Default::default()))}let map=s.map.as_mut().unwrap();map.len-=1;if map.pages.last().is_some_and(Vec::is_empty){s.page=map.pages.pop();}s.phase=5;}
   5=>return Ok(RetainedOrderedMapRemoveStep::Complete{removed:s.found,progress}),
   _=>unreachable!()
  }
  Ok(RetainedOrderedMapRemoveStep::Progress(progress))
 }
 pub fn output_ready(&self)->bool{let s=&*self.state;s.phase==5&&s.map.is_some()&&!s.closing&&!s.spent}
 pub fn take(&mut self)->Option<RetainedOrderedMap<K,V>> {let s=&mut *self.state;if s.phase!=5||s.closing||s.spent{return None}s.spent=true;s.map.take()}
 pub fn begin_close(&mut self)->bool {let s=&mut *self.state;let started=!s.closing;s.closing=true;s.lookup.begin_close();started}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{let s=&*self.state;if !s.lookup.terminal_is_empty(){s.lookup.next_close_copy_byte_demand()}else if !s.close.is_empty(){s.close.next_copy_byte_demand()}else if s.key.is_some()||s.removed.is_some()||s.page.is_some()||s.map.is_some(){Ok(0)}else{s.lease.next_close_copy_byte_demand()}}
 pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{let s=&*self.state;if !s.lookup.terminal_is_empty(){return s.lookup.next_close_capacity_byte_demand(body)}if !s.close.is_empty(){return s.close.next_capacity_byte_demand(body)}if s.key.is_some(){return s.close.next_owner_capacity_byte_demand::<K>(true,body)}if s.removed.is_some(){return s.close.next_owner_capacity_byte_demand::<(K,V)>(true,body)}if s.page.is_some(){return s.close.next_owner_capacity_byte_demand::<Vec<(K,V)>>(true,body)}if s.map.is_some(){s.close.next_owner_capacity_byte_demand::<RetainedOrderedMap<K,V>>(true,body)}else{s.lease.next_close_capacity_byte_demand(body)}}
 pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{let s=&*self.state;if !s.lookup.terminal_is_empty(){s.lookup.next_close_release_byte_demand()}else if !s.close.is_empty(){s.close.next_release_byte_demand()}else if s.key.is_some()||s.removed.is_some()||s.page.is_some()||s.map.is_some(){Ok(0)}else{s.lease.next_close_release_byte_demand()}}
 pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{let s=&*self.state;if !s.lookup.terminal_is_empty(){s.lookup.next_close_depth_demand()}else if !s.close.is_empty(){s.close.next_depth_demand()}else if s.key.is_some()||s.removed.is_some()||s.page.is_some()||s.map.is_some(){Ok(1)}else{s.lease.next_close_depth_demand()}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{let s=&mut *self.state;if !s.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"removal must begin close"))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !s.lookup.terminal_is_empty(){return s.lookup.close_step(grant)}if !s.close.is_empty(){return s.close.step_granted(grant)}if let Some(step)=s.close.begin_granted(&mut s.key,grant)?{return Ok(step)}if let Some(step)=s.close.begin_granted(&mut s.removed,grant)?{return Ok(step)}if let Some(step)=s.close.begin_granted(&mut s.page,grant)?{return Ok(step)}if let Some(step)=s.close.begin_granted(&mut s.map,grant)?{return Ok(step)}s.lease.close_step(grant)}
 pub fn terminal_is_empty(&self)->bool{let s=&*self.state;s.closing&&s.map.is_none()&&s.key.is_none()&&s.removed.is_none()&&s.page.is_none()&&s.lookup.terminal_is_empty()&&s.lease.terminal_is_empty()&&s.close.is_empty()}
}
impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> Drop for RetainedOrderedMapRemoveCursor<K,V>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("removal original owners require caller-funded closure")}}}
