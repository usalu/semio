//! 📨️ A completed replay's original messages move into canonical grouped receipt storage without cloning payload fields.
use crate::PendingCommandReport;
use semio_framework_os_spr::{ReplayReport,EditMessages,MutationMessage};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"pending replay report changed its original outcome owner")}
fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
/// 🧳️ The enclosing finalizer owns the completed Report through every partial move and paid cancellation.
pub struct OriginalReplayPendingReport{report:ManuallyDrop<Option<PendingCommandReport>>,retirement:RetainedCloneClose,source:Option<(usize,u64,usize,usize)>,phase:u8,scan:usize,anchor:Option<usize>,candidate:Option<usize>,compare:usize,groups:usize,total:usize,current:Option<usize>,stop:Option<usize>,swap:usize,closing:bool}
impl Default for OriginalReplayPendingReport{fn default()->Self{Self{report:ManuallyDrop::new(None),retirement:Default::default(),source:None,phase:0,scan:0,anchor:None,candidate:None,compare:0,groups:0,total:0,current:None,stop:None,swap:0,closing:false}}}
impl OriginalReplayPendingReport{
 fn compare_ids(&mut self,source:&ReplayReport)->Result<Option<bool>,ValueError>{let left=source.outcomes.get(self.anchor.ok_or_else(invalid)?).ok_or_else(invalid)?.edit_id.as_bytes();let right=source.outcomes.get(self.candidate.ok_or_else(invalid)?).ok_or_else(invalid)?.edit_id.as_bytes();if left.len()!=right.len(){self.compare=0;return Ok(Some(false))}let end=(self.compare+64).min(left.len());if left[self.compare..end]!=right[self.compare..end]{self.compare=0;return Ok(Some(false))}if end==left.len(){self.compare=0;Ok(Some(true))}else{self.compare=end;Ok(None)}}
 /// 📏️ Every canonical header reservation, original message move, reversal and source backing return is quoted first.
 pub fn demands(&self,source:&ReplayReport)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(4096)}let mut d=RetirementDemand{copy_bytes:128,depth:1,..Default::default()};match self.phase{2|6=>d.copy_bytes=256,3=>d.capacity_bytes=self.groups.checked_mul(size_of::<EditMessages>()).ok_or_else(invalid)?,7=>{d.copy_bytes+=2*size_of::<EditMessages>()+size_of::<String>();d.capacity_bytes=self.total.checked_mul(size_of::<MutationMessage>()).ok_or_else(invalid)?;},8=>{if let Some(index)=self.current.filter(|index|Some(*index)!=self.stop){let messages=&source.outcomes.get(index).ok_or_else(invalid)?.messages;if !messages.is_empty(){d.copy_bytes+=2*size_of::<MutationMessage>();}else{d.copy_bytes+=size_of::<Vec<MutationMessage>>();d.release_bytes=messages.capacity().checked_mul(size_of::<MutationMessage>()).ok_or_else(invalid)?;}}},9=>d.copy_bytes+=2*size_of::<MutationMessage>(),10=>d.copy_bytes+=2*size_of::<EditMessages>(),_=>{}}Ok(d)}
 /// 🪡️ Reverse extraction uses only original Vec pops; bounded swaps restore the exact public receipt order.
 pub fn advance(&mut self,source:&mut ReplayReport,epoch:u64,grant:RetainedCloneGrant,mut admit:impl FnMut(usize)->Result<(),ValueError>)->Result<RetainedCloneProgress,ValueError>{
  if self.closing{return Err(invalid())}let identity=(source as*const _ as usize,epoch,source.outcomes.as_ptr()as usize,source.outcomes.len());if self.source.is_some_and(|original|original!=identity){return Err(invalid())}if self.prepared(){return Ok(Default::default())}let d=self.demands(source)?;if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}admit(d.capacity_bytes)?;if self.source.is_none(){self.source=Some(identity);}
  match self.phase{
   0=>{self.scan=source.outcomes.len();self.phase=1;},
   1=>{if self.scan==0{self.phase=3;}else{self.scan-=1;if !source.outcomes[self.scan].messages.is_empty(){if self.anchor.is_none(){self.anchor=Some(self.scan);self.groups+=1;}else{self.candidate=Some(self.scan);self.compare=0;self.phase=2;}}}},
   2=>{if let Some(equal)=self.compare_ids(source)?{if !equal{self.groups=self.groups.checked_add(1).ok_or_else(invalid)?;}self.anchor=self.candidate.take();self.phase=1;}},
   3=>{let mut messages=Vec::new();messages.try_reserve_exact(self.groups).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"pending replay group header reservation failed"))?;*self.report=Some(PendingCommandReport{edit_ids:Some(Vec::new()),messages,worst:source.worst,outbound:Vec::new()});self.scan=source.outcomes.len();self.anchor=None;self.phase=4;},
   4=>{if self.scan==0{self.swap=0;self.phase=10;}else{self.scan-=1;if !source.outcomes[self.scan].messages.is_empty(){self.anchor=Some(self.scan);self.current=Some(self.scan);self.total=source.outcomes[self.scan].messages.len();self.stop=None;self.phase=5;}}},
   5=>{if self.scan==0{self.phase=7;}else{self.scan-=1;if !source.outcomes[self.scan].messages.is_empty(){self.candidate=Some(self.scan);self.compare=0;self.phase=6;}}},
   6=>{if let Some(equal)=self.compare_ids(source)?{if equal{self.total=self.total.checked_add(source.outcomes[self.candidate.unwrap()].messages.len()).ok_or_else(invalid)?;self.phase=5;}else{self.stop=self.candidate;self.phase=7;}self.candidate=None;}},
   7=>{let mut messages=Vec::new();messages.try_reserve_exact(self.total).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"pending replay message header reservation failed"))?;let edit_id=std::mem::take(&mut source.outcomes[self.anchor.ok_or_else(invalid)?].edit_id);let target=self.report.as_mut().unwrap();if target.messages.len()==target.messages.capacity(){return Err(invalid())}target.messages.push(EditMessages{edit_id,messages});self.phase=8;},
   8=>{if let Some(index)=self.current.filter(|index|Some(*index)!=self.stop){let messages=&mut source.outcomes[index].messages;if let Some(message)=messages.pop(){let target=&mut self.report.as_mut().unwrap().messages.last_mut().unwrap().messages;if target.len()==target.capacity(){return Err(invalid())}target.push(message);}else if messages.capacity()!=0{drop(std::mem::take(messages));}else{self.current=index.checked_sub(1);}}else{self.swap=0;self.phase=9;}},
   9=>{let messages=&mut self.report.as_mut().unwrap().messages.last_mut().unwrap().messages;if self.swap<messages.len()/2{let right=messages.len()-1-self.swap;messages.swap(self.swap,right);self.swap+=1;}else{self.scan=self.stop.map_or(0,|index|index+1);self.anchor=None;self.phase=4;}},
   10=>{let messages=&mut self.report.as_mut().unwrap().messages;if self.swap<messages.len()/2{let right=messages.len()-1-self.swap;messages.swap(self.swap,right);self.swap+=1;}else{self.phase=11;}},
   _=>return Err(invalid()),
  }Ok(paid(d))
 }
 /// 🏁️ Only completely grouped, ordered canonical receipt messages are visible to the original publisher.
 pub fn prepared(&self)->bool{self.phase==11&&!self.closing}
 /// 🤝️ The actual canonical pending report moves without another message or identifier allocation.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(PendingCommandReport,RetainedCloneProgress)>,ValueError>{if !self.prepared(){return Err(invalid())}let d=RetirementDemand{copy_bytes:size_of::<PendingCommandReport>()+128,depth:1,..Default::default()};if !retained_clone_grant_admits(grant,d){return Ok(None)}let report=self.report.take().ok_or_else(invalid)?;self.source=None;self.phase=12;Ok(Some((report,paid(d))))}
 /// 📐️ Partial message ownership returns through the canonical PendingCommandReport retirement contract.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.report.is_some(){return self.retirement.next_owner_demands::<PendingCommandReport>(true,&None,body)}Ok(RetirementDemand{copy_bytes:128,depth:1,..Default::default()})}
 /// ♻️ Cancellation closes only messages already moved; the enclosing owner retires the remaining original result.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.closing=true;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}if self.report.is_some(){return Ok(self.retirement.begin_granted(&mut self.report,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}self.source=None;Ok(RetainedCloneStep::Progress(paid(d)))}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.report.is_none()&&self.retirement.is_empty()}
}
impl Drop for OriginalReplayPendingReport{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"pending replay receipt requires paid handoff or cancellation");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.report);}}}}

#[cfg(test)]
#[path="tests.rs"]
mod tests;
