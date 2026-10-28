//! 🧾️ Native checkpoint selection compares original applied IDs with actual committed history.
use super::*;
use crate::HistoryPageStack;
use semio_framework_value::{RetainedCloneStep,retained_clone::RetainedCloneClose};
use std::mem::ManuallyDrop;
/// 🧺️ The canonical Change owns a Vec; its header backing is reserved once before bounded original ID births.
pub struct OriginalPendingEditIds{output:ManuallyDrop<Option<Vec<String>>>,text:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,source:Option<(u64,usize,usize,usize,usize)>,applied:usize,change:Option<ArtifactHistoryKey>,edit:usize,offset:usize,phase:u8,closing:bool,probe_only:bool,found:bool}
impl Default for OriginalPendingEditIds{fn default()->Self{Self{output:ManuallyDrop::new(None),text:ManuallyDrop::new(None),retirement:Default::default(),source:None,applied:0,change:None,edit:0,offset:0,phase:0,closing:false,probe_only:false,found:false}}}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}
fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl OriginalPendingEditIds{
 /// 🔎️ A clean/dirty probe compares the actual history and births no edit-vector backing or ID strings.
 pub fn probe()->Self{let mut probe=Self::default();probe.probe_only=true;probe}
 /// 🏁️ The fixed answer is available only after the original bounded history scan seals.
 pub fn pending_found(&self)->Option<bool>{self.ready().then(||if self.probe_only{self.found}else{!self.output.as_ref().unwrap().is_empty()})}

 /// 📐️ The exact next canonical reservation or bounded original-ID action is visible before admission.
 pub fn demands(&self,applied:&HistoryPageStack<String>)->Result<RetirementDemand,ValueError>{let mut d=RetirementDemand{copy_bytes:128,depth:1,..Default::default()};match self.phase{0 if !self.probe_only=>d.capacity_bytes=applied.len().checked_mul(size_of::<String>()).ok_or_else(invalid)?,4 if !self.probe_only=>d.capacity_bytes=applied.get(self.applied).ok_or_else(invalid)?.len(),5=>d.copy_bytes+=64,6=>d.copy_bytes+=size_of::<String>(),_=>{}}Ok(d)}
 /// 🫳️ Only the actual applied stack and committed Change ledger participate in selection.
 pub fn advance(&mut self,generation:u64,applied:&HistoryPageStack<String>,changes:&ArtifactHistoryLedger<Change>,grant:RetainedCloneGrant,mut admit:impl FnMut(usize)->Result<(),ValueError>)->Result<RetainedCloneProgress,ValueError>{
  if self.closing{return Err(invalid())}let source=(generation,applied as*const _ as usize,applied.len(),changes as*const _ as usize,changes.len());if self.source.is_some_and(|original|original!=source){return Err(invalid())}if self.ready(){return Ok(Default::default())}
  let d=self.demands(applied)?;
  if !permits(grant,d){return Ok(Default::default())}self.source=Some(source);admit(d.capacity_bytes)?;
  match self.phase{
   0=>{let mut output=Vec::new();output.try_reserve_exact(if self.probe_only{0}else{applied.len()}).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"pending edit header reservation failed"))?;*self.output=Some(output);self.phase=1;},
   1 if self.applied==applied.len()=>self.phase=7,
   1=>{self.change=changes.last_key();self.edit=0;self.offset=0;self.phase=2;},
   2=>{if let Some(key)=self.change{let row=changes.get_key(key).ok_or_else(invalid)?;if self.edit==row.edit_ids.len(){self.change=changes.previous_key(key);self.edit=0;}else{let candidate=&row.edit_ids[self.edit];let original=applied.get(self.applied).ok_or_else(invalid)?;if candidate.len()!=original.len(){self.edit+=1;}else{self.offset=0;self.phase=3;}}}else{self.phase=4;}},
   3=>{let row=changes.get_key(self.change.ok_or_else(invalid)?).ok_or_else(invalid)?;let candidate=row.edit_ids.get(self.edit).ok_or_else(invalid)?;let original=applied.get(self.applied).ok_or_else(invalid)?;let end=(self.offset+64).min(original.len());if original.as_bytes()[self.offset..end]!=candidate.as_bytes()[self.offset..end]{self.edit+=1;self.offset=0;self.phase=2;}else if end==original.len(){self.applied+=1;self.phase=1;}else{self.offset=end;}},
   4 if self.probe_only=>{self.found=true;self.phase=7;},
   4=>{let original=applied.get(self.applied).ok_or_else(invalid)?;let mut text=String::new();text.try_reserve_exact(original.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"pending edit ID reservation failed"))?;*self.text=Some(text);self.offset=0;self.phase=5;},
   5=>{let original=applied.get(self.applied).ok_or_else(invalid)?;let mut end=(self.offset+64).min(original.len());while !original.is_char_boundary(end){end-=1;}self.text.as_mut().unwrap().push_str(&original[self.offset..end]);self.offset=end;if end==original.len(){self.phase=6;}},
   6=>{self.output.as_mut().unwrap().push(self.text.take().ok_or_else(invalid)?);self.applied+=1;self.phase=1;},
   _=>return Err(invalid()),
  }Ok(paid(d))
 }
 /// 🏁️ The actual canonical pending edit vector is sealed before the original author can borrow it.
 pub fn ready(&self)->bool{self.phase==7&&!self.closing}
 /// 👁️ Borrows the prepared canonical Change field without constructing an index or adapter.
 pub fn prepared(&self)->Option<&[String]>{self.ready().then(||self.output.as_ref().unwrap().as_slice())}
 /// 🤝️ Transfers the exact reserved header and original string allocations.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<String>,RetainedCloneProgress)>,ValueError>{if !self.ready()||self.probe_only{return Err(invalid())}let d=RetirementDemand{copy_bytes:size_of::<Vec<String>>()+128,depth:1,..Default::default()};if !permits(grant,d){return Ok(None)}let output=self.output.take().ok_or_else(invalid)?;self.source=None;self.phase=8;Ok(Some((output,paid(d))))}
 /// 📏️ Prices the real partial String or canonical Vec retirement before cancellation changes custody.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.text.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<Vec<String>>(true,&None,body)}Ok(RetirementDemand{copy_bytes:128,depth:1,..Default::default()})}
 /// ♻️ The original partial canonical fields close through their defining typed retirement.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !permits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.closing=true;if !self.retirement.is_empty(){return self.retirement.select(grant.maximum_copy_bytes)?.step(grant)}if self.text.is_some(){return Ok(self.retirement.begin_granted(&mut self.text,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}if self.output.is_some(){return Ok(self.retirement.begin_granted(&mut self.output,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}self.source=None;Ok(RetainedCloneStep::Progress(paid(d)))}
 /// 🫙️ Every partial canonical field and original source coordinate has returned.
 pub fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.text.is_none()&&self.retirement.is_empty()&&self.source.is_none()}
}
impl Drop for OriginalPendingEditIds{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original pending edits require paid handoff or full retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.text);}}}}
