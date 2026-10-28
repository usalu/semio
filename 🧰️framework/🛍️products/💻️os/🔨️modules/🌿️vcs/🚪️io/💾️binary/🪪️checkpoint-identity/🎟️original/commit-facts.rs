//! 🚩️ Canonical checkpoint facts adopt original authors, message and edit IDs before bounded parent-field copying.
use super::*;
use crate::{Author,Checkpoint};
use semio_framework_value::{RetainedCloneStep,retained_clone::RetainedCloneClose};
use std::mem::ManuallyDrop;
/// 📚️ Immutable original parent and authored scalar fields remain borrowed through preparation.
pub struct OriginalCommitFactInput<'a>{pub generation:u64,pub parent:Option<&'a Checkpoint>,pub change_id:&'a str,pub saved_at:&'a str,pub timestamp:&'a str}
impl OriginalCommitFactInput<'_>{fn identity(&self)->(u64,[usize;7]){(self.generation,[self.parent.map_or(0,|value|value as*const _ as usize),self.change_id.as_ptr()as usize,self.change_id.len(),self.saved_at.as_ptr()as usize,self.saved_at.len(),self.timestamp.as_ptr()as usize,self.timestamp.len()])}fn changes(&self)->usize{self.parent.map_or(0,|parent|parent.change_ids.len())+1}fn source(&self,index:usize)->Option<&str>{match index{0=>Some(self.change_id),1=>Some(self.saved_at),2=>None,3=>self.parent.map(|parent|parent.id.as_str()),4=>Some(self.timestamp),rank=>{let rank=rank-5;if rank+1==self.changes(){Some(self.change_id)}else{self.parent.and_then(|parent|parent.change_ids.get(rank)).map(String::as_str)}}}}}
/// 🧳️ A partial canonical pair owns every donated allocation until native handoff or paid cancellation.
pub struct OriginalCommitFactBirth{value:ManuallyDrop<Option<(Change,Checkpoint)>>,source:Option<(u64,[usize;7])>,slots:Option<[usize;3]>,retirement:RetainedCloneClose,phase:u8,field:usize,offset:Option<usize>,closing:bool}
impl Default for OriginalCommitFactBirth{fn default()->Self{Self{value:ManuallyDrop::new(None),source:None,slots:None,retirement:Default::default(),phase:0,field:0,offset:None,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original checkpoint facts changed their authored source or donor slots")}
fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl OriginalCommitFactBirth{
 fn field_source<'a>(&'a self,input:&'a OriginalCommitFactInput<'_>)->Option<&'a str>{if self.field==2{self.value.as_ref().and_then(|value|value.1.message.as_deref())}else{input.source(self.field)}}
 fn target(&mut self)->Result<&mut String,ValueError>{let(change,checkpoint)=self.value.as_mut().ok_or_else(invalid)?;match self.field{0=>Ok(&mut change.id),1=>Ok(&mut change.saved_at),2=>change.description.as_mut().ok_or_else(invalid),3=>checkpoint.parent_id.as_mut().ok_or_else(invalid),4=>Ok(&mut checkpoint.timestamp),rank=>checkpoint.change_ids.get_mut(rank-5).ok_or_else(invalid)}}
 /// 📏️ Every header birth, reservation or UTF-8 fragment has its own exact original quote.
 pub fn demands(&self,input:&OriginalCommitFactInput<'_>)->Result<RetirementDemand,ValueError>{let mut d=RetirementDemand{copy_bytes:128,depth:1,..Default::default()};match self.phase{0=>d.copy_bytes+=2*size_of::<(Change,Checkpoint)>()+size_of::<Vec<Author>>()+size_of::<Option<String>>()+size_of::<Vec<String>>(),1=>d.capacity_bytes=input.changes().checked_mul(size_of::<String>()).ok_or_else(invalid)?,3 if self.field<5+input.changes()&&self.offset.is_none()=>d.capacity_bytes=self.field_source(input).map_or(0,str::len),3=>d.copy_bytes+=64,_=>{}}Ok(d)}
 /// 🎟️ The actual owned edit IDs, message and author vector move once after all currencies admit the source action.
 pub fn advance(&mut self,input:&OriginalCommitFactInput<'_>,pending:&mut Option<Vec<String>>,message:&mut Option<String>,authors:&mut Vec<Author>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let identity=input.identity();let slots=[pending as*const _ as usize,message as*const _ as usize,authors as*const _ as usize];if self.closing||self.source.is_some_and(|source|source!=identity)||self.slots.is_some_and(|original|original!=slots){return Err(invalid())}if self.ready(){return Ok(Default::default())}let d=self.demands(input)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}
  match self.phase{
   0=>{let edit_ids=pending.take().ok_or_else(invalid)?;let description=message.as_ref().map(|_|String::new());self.source=Some(identity);self.slots=Some(slots);*self.value=Some((Change{id:String::new(),edit_ids,description,saved_at:String::new()},Checkpoint{id:String::new(),change_ids:Vec::new(),parent_id:input.parent.map(|_|String::new()),authors:std::mem::take(authors),message:message.take(),timestamp:String::new(),composition_pins:Vec::new()}));self.phase=1;},
   1=>{self.value.as_mut().unwrap().1.change_ids.try_reserve_exact(input.changes()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original checkpoint change identity backing refused"))?;self.phase=2;},
   2=>{let checkpoint=&mut self.value.as_mut().unwrap().1;if checkpoint.change_ids.len()<input.changes(){checkpoint.change_ids.push(String::new());}else{self.phase=3;}},
   3 if self.field==5+input.changes()=>self.phase=4,
   3=>{if let Some(original)=self.field_source(input){if let Some(start)=self.offset{let mut end=(start+64).min(original.len());while !original.is_char_boundary(end){end-=1;}let length=original.len();let count=end-start;let mut bytes=[0u8;64];bytes[..count].copy_from_slice(&original.as_bytes()[start..end]);self.target()?.push_str(std::str::from_utf8(&bytes[..count]).map_err(|_|invalid())?);if end==length{self.offset=None;self.field+=1;}else{self.offset=Some(end);}}else{let length=original.len();self.target()?.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original checkpoint text backing refused"))?;self.offset=Some(0);}}else{self.field+=1;}},
   _=>return Err(invalid()),
  }Ok(paid(d))
 }
 /// 🏁️ The caller supplies the later content-addressed ID only after this original candidate is complete.
 pub fn ready(&self)->bool{self.phase==4&&!self.closing}
 /// 👁️ The actual pair remains borrowed without cloning any authored field.
 pub fn prepared(&self)->Option<&(Change,Checkpoint)>{self.ready().then(||self.value.as_ref().unwrap())}
 /// 📐️ The same original pair crosses the final handoff in one fixed ownership transfer.
 pub fn take_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<(Change,Checkpoint)>(),depth:1,..Default::default()}}
 /// 🤝️ Every authored vector and string retains its original allocation in the returned canonical pair.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<((Change,Checkpoint),RetainedCloneProgress)>,ValueError>{if !self.ready(){return Err(invalid())}let d=Self::take_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let value=self.value.take().ok_or_else(invalid)?;self.source=None;self.slots=None;self.phase=5;Ok(Some((value,paid(d))))}
 /// 📋️ Cancellation first returns the actual partial typed pair and then its immutable source coordinates.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(RetirementDemand{copy_bytes:1,depth:1,..Default::default()})}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.value.is_some(){return self.retirement.next_owner_demands::<(Change,Checkpoint)>(true,&None,body)}Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,[usize;7])>>()+size_of::<Option<[usize;3]>>()+size_of::<bool>(),depth:1,..Default::default()})}
 /// ♻️ A denied close preserves both the selected quote and every original owner.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.closing{self.closing=true;return Ok(RetainedCloneStep::Progress(paid(d)))}if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}if self.value.is_some(){return Ok(self.retirement.begin_granted(&mut self.value,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}self.closing=true;self.source=None;self.slots=None;Ok(RetainedCloneStep::Complete(paid(d)))}
 /// 🫙️ The enclosing native source may release this inline frame only after all authored owners return.
 pub fn terminal_is_empty(&self)->bool{self.value.is_none()&&self.retirement.is_empty()&&self.source.is_none()&&self.slots.is_none()}
}
impl Drop for OriginalCommitFactBirth{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original checkpoint facts retain authored owners");}}

#[cfg(test)]
#[path="commit-facts-tests.rs"]mod tests;
