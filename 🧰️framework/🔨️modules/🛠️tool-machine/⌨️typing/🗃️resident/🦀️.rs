//! 🗃️ Original typing resident addresses expose bounded native growth, exchange and physical release.
use super::*;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind,SharedUtf8};
use std::mem::size_of;
#[derive(Debug,PartialEq)]
pub(crate) struct TypingWindow<M>{pub(super) key:SharedUtf8,pub(super) state:RetainedOriginalSlot<TypingState<M>>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original typing resident address changed")}
fn movement(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn admits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items!=0&&semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}
impl<M> TypingLedger<M>{
 pub fn retained_read_pending(&self,window:&str)->bool{self.original_slot(window).is_some_and(RetainedOriginalSlot::has_read)}

 pub(super) fn position(&self,window:&str)->Result<usize,usize>{self.windows.binary_search_by(|row|row.key.as_str().cmp(window))}
 pub(crate) fn original_slot(&self,window:&str)->Option<&RetainedOriginalSlot<TypingState<M>>>{self.position(window).ok().map(|i|&self.windows[i].state)}
 pub(crate) fn original_slot_mut(&mut self,window:&str)->Option<&mut RetainedOriginalSlot<TypingState<M>>>{let i=self.position(window).ok()?;Some(&mut self.windows[i].state)}
 pub(super) fn cold_take(&mut self,window:&str)->Option<TypingState<M>>{let slot=self.original_slot_mut(window)?;if slot.is_empty()||slot.has_read(){return None}let slot=std::mem::replace(slot,RetainedOriginalSlot::vacant());Some(slot.try_into_original().unwrap_or_else(|_|unreachable!("unissued original typing row")))}
 pub(super) fn cold_install(&mut self,window:&str,state:TypingState<M>){let index=match self.position(window){Ok(i)=>i,Err(i)=>{self.windows.insert(i,TypingWindow{key:window.to_owned().into(),state:RetainedOriginalSlot::vacant()});i}};self.windows[index].state=RetainedOriginalSlot::new(state)}
 pub fn original_write_pending(&self)->bool{self.writing}
 pub fn begin_original_write(&mut self){assert!(!self.writing,"original typing writer already retained");self.writing=true}
 pub fn end_original_write(&mut self){assert!(self.writing,"original typing writer absent");self.writing=false}
 pub fn original_window_demands(&self,window:&str)->Result<RetirementDemand,ValueError>{
  if !self.writing{return Err(invalid())}match self.position(window){Ok(_)=>Ok(movement(size_of::<Option<RetainedOriginalSlotKey>>())),Err(i)=>if self.windows.len()==self.windows.capacity(){let bytes=self.windows.capacity().checked_mul(size_of::<TypingWindow<M>>()).ok_or_else(invalid)?;Ok(RetirementDemand{copy_bytes:bytes+2*size_of::<Vec<TypingWindow<M>>>(),capacity_bytes:(self.windows.len()+1).checked_mul(size_of::<TypingWindow<M>>()).ok_or_else(invalid)?,release_bytes:bytes,depth:1})}else{Ok(movement((self.windows.len()-i+1)*size_of::<TypingWindow<M>>()+size_of::<u64>()+size_of::<Option<RetainedOriginalSlotKey>>()))}}
 }
 pub fn prepare_original_window(&mut self,key:&mut Option<SharedUtf8>,grant:RetainedCloneGrant)->Result<(Option<RetainedOriginalSlotKey>,RetainedCloneProgress),ValueError>{
  let name=key.as_ref().ok_or_else(invalid)?.as_str();let d=self.original_window_demands(name)?;if !admits(grant,d){return Ok((None,Default::default()))}
  match self.position(name){Ok(i)=>Ok((Some(self.windows[i].state.key()),progress(d))),Err(i)=>{
   if self.windows.len()==self.windows.capacity(){let old=self.windows.capacity()*size_of::<TypingWindow<M>>();self.windows.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original typing resident backing refused"))?;return Ok((None,RetainedCloneProgress{retained_capacity_bytes:self.windows.capacity()*size_of::<TypingWindow<M>>(),released_bytes:old,..progress(d)}))}
   let revision=self.source_revision.checked_add(1).ok_or_else(invalid)?;self.windows.insert(i,TypingWindow{key:key.take().unwrap(),state:RetainedOriginalSlot::vacant()});self.source_revision=revision;Ok((Some(self.windows[i].state.key()),progress(d)))
  }}
 }
 pub fn original_edit_transform_demands<R>(&self)->RetirementDemand{let mut d=RetainedOriginalSlot::<TypingState<M>>::transform_demand::<R>();d.copy_bytes+=size_of::<u64>()+size_of::<usize>();d}
 pub fn transform_original_edit<R>(&mut self,window:&str,key:RetainedOriginalSlotKey,grant:RetainedCloneGrant,transform:impl FnOnce(Option<TypingState<M>>)->(Option<TypingState<M>>,R))->Result<Option<(R,RetainedCloneProgress)>,ValueError>{
  if !self.writing{return Err(invalid())}let d=self.original_edit_transform_demands::<R>();if !admits(grant,d){return Ok(None)}let revision=self.source_revision.checked_add(1).ok_or_else(invalid)?;let i=self.position(window).map_err(|_|invalid())?;if self.windows[i].state.has_read(){return Ok(None)}let old=self.windows[i].state.is_empty();let overhead=size_of::<u64>()+size_of::<usize>();let Some((result,mut p))=self.windows[i].state.transform_original(key,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant},transform)?else{return Ok(None)};let new=self.windows[i].state.is_empty();self.live_windows=self.live_windows.checked_sub(usize::from(!old)).and_then(|n|n.checked_add(usize::from(!new))).ok_or_else(invalid)?;self.source_revision=revision;p.copied_bytes+=overhead;Ok(Some((result,p)))
 }
 pub fn original_close_window_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.writing{return Err(invalid())}if let Some(row)=self.windows.last(){if !row.state.is_empty(){let mut d=RetainedOriginalSlot::<TypingState<M>>::transfer_demand();d.copy_bytes+=size_of::<u64>()+size_of::<usize>();return Ok(d)}return Ok(movement(size_of::<TypingWindow<M>>()+size_of::<Option<SharedUtf8>>()))}
  if self.windows.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.windows.capacity()*size_of::<TypingWindow<M>>(),..movement(2*size_of::<Vec<TypingWindow<M>>>())})}Ok(Default::default())
 }
 pub fn close_original_window(&mut self,state:&mut Option<TypingState<M>>,key:&mut Option<SharedUtf8>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if state.is_some()||key.is_some(){return Err(invalid())}let d=self.original_close_window_demands()?;if !admits(grant,d){return Ok(Default::default())}
  if let Some(row)=self.windows.last_mut(){if row.state.has_read(){return Ok(Default::default())}if !row.state.is_empty(){let revision=self.source_revision.checked_add(1).ok_or_else(invalid)?;let overhead=size_of::<u64>()+size_of::<usize>();let Some(mut p)=row.state.take_original(row.state.key(),state,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant})?else{return Ok(Default::default())};self.live_windows-=1;self.source_revision=revision;p.copied_bytes+=overhead;return Ok(p)}
   let row=self.windows.pop().unwrap();*key=Some(row.key);return Ok(progress(d))
  }if self.windows.capacity()!=0{drop(std::mem::take(&mut self.windows));return Ok(progress(d))}Ok(Default::default())
 }
 pub fn original_windows_terminal_is_empty(&self)->bool{self.windows.is_empty()&&self.windows.capacity()==0&&!self.writing}
}
