//! 📌️ Reidentified canonical checkpoint rows preserve original metadata through separately funded field births.
use super::*;
use crate::{Author,Checkpoint};
use semio_framework_value::{RetainedCloneStep,retained_clone::RetainedCloneClose};
use std::mem::ManuallyDrop;
/// 🧷️ All metadata remains borrowed from the actual checkpoint selected by the Store publisher.
pub struct OriginalRepinCheckpointInput<'a>{pub generation:u64,pub checkpoint:&'a Checkpoint,pub checkpoint_id:&'a str}
impl OriginalRepinCheckpointInput<'_>{
 fn identity(&self)->(u64,usize,usize,usize){(self.generation,self.checkpoint as*const _ as usize,self.checkpoint_id.as_ptr()as usize,self.checkpoint_id.len())}
 fn fields(&self)->Result<usize,ValueError>{self.checkpoint.authors.len().checked_mul(3).and_then(|n|n.checked_add(self.checkpoint.change_ids.len())).and_then(|n|n.checked_add(4)).ok_or_else(invalid)}
 fn source(&self,field:usize)->Option<&str>{match field{0=>Some(self.checkpoint_id),1=>self.checkpoint.parent_id.as_deref(),2=>self.checkpoint.message.as_deref(),3=>Some(&self.checkpoint.timestamp),n if n<4+self.checkpoint.change_ids.len()=>Some(&self.checkpoint.change_ids[n-4]),n=>{let n=n-4-self.checkpoint.change_ids.len();let author=self.checkpoint.authors.get(n/3)?;match n%3{0=>Some(&author.id),1=>Some(&author.name),_=>author.avatar.as_deref()}}}}
}
/// 🧳️ The canonical partial checkpoint and adopted pin roster remain in their original cancellation owner.
pub struct OriginalRepinCheckpointBirth{value:ManuallyDrop<Option<Checkpoint>>,source:Option<(u64,usize,usize,usize)>,retirement:RetainedCloneClose,phase:u8,field:usize,offset:Option<usize>,closing:bool}
impl Default for OriginalRepinCheckpointBirth{fn default()->Self{Self{value:ManuallyDrop::new(None),source:None,retirement:Default::default(),phase:0,field:0,offset:None,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"repin checkpoint birth changed its original source")}
fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl OriginalRepinCheckpointBirth{
 fn target(&mut self,input:&OriginalRepinCheckpointInput<'_>)->Result<&mut String,ValueError>{let value=self.value.as_mut().ok_or_else(invalid)?;match self.field{0=>Ok(&mut value.id),1=>value.parent_id.as_mut().ok_or_else(invalid),2=>value.message.as_mut().ok_or_else(invalid),3=>Ok(&mut value.timestamp),n if n<4+input.checkpoint.change_ids.len()=>value.change_ids.get_mut(n-4).ok_or_else(invalid),n=>{let n=n-4-input.checkpoint.change_ids.len();let author=value.authors.get_mut(n/3).ok_or_else(invalid)?;match n%3{0=>Ok(&mut author.id),1=>Ok(&mut author.name),_=>author.avatar.as_mut().ok_or_else(invalid)}}}}
 /// 📐️ Each event prices only its actual selected header, field backing or bounded UTF-8 fragment.
 pub fn demands(&self,input:&OriginalRepinCheckpointInput<'_>)->Result<RetirementDemand,ValueError>{let mut d=RetirementDemand{copy_bytes:128,depth:1,..Default::default()};match self.phase{0=>d.copy_bytes+=2*size_of::<Checkpoint>()+size_of::<Option<Vec<CompositionPin>>>(),1=>d.capacity_bytes=input.checkpoint.change_ids.len().checked_mul(size_of::<String>()).ok_or_else(invalid)?,2=>d.copy_bytes+=size_of::<String>(),3=>d.capacity_bytes=input.checkpoint.authors.len().checked_mul(size_of::<Author>()).ok_or_else(invalid)?,4=>d.copy_bytes+=size_of::<Author>(),5 if self.field<input.fields()?&&self.offset.is_none()=>d.capacity_bytes=input.source(self.field).map_or(0,str::len),5=>d.copy_bytes+=64,_=>{}}Ok(d)}
 /// 🎟️ The real sorted pin vector moves once; every remaining canonical field is born from its original loan.
 pub fn advance(&mut self,input:&OriginalRepinCheckpointInput<'_>,pins:&mut Option<Vec<CompositionPin>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.closing||self.source.is_some_and(|source|source!=input.identity()){return Err(invalid())}if self.ready(){return Ok(Default::default())}if self.phase==0&&pins.is_none(){return Err(invalid())}let d=self.demands(input)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(input.identity());
  match self.phase{
   0=>{*self.value=Some(Checkpoint{id:String::new(),change_ids:Vec::new(),parent_id:input.checkpoint.parent_id.as_ref().map(|_|String::new()),authors:Vec::new(),message:input.checkpoint.message.as_ref().map(|_|String::new()),timestamp:String::new(),composition_pins:pins.take().unwrap()});self.phase=1;},
   1=>{self.value.as_mut().unwrap().change_ids.try_reserve_exact(input.checkpoint.change_ids.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"repin original change-ID backing refused"))?;self.phase=2;},
   2=>{let value=self.value.as_mut().unwrap();if value.change_ids.len()<input.checkpoint.change_ids.len(){value.change_ids.push(String::new());}else{self.phase=3;}},
   3=>{self.value.as_mut().unwrap().authors.try_reserve_exact(input.checkpoint.authors.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"repin original author backing refused"))?;self.phase=4;},
   4=>{let value=self.value.as_mut().unwrap();if let Some(author)=input.checkpoint.authors.get(value.authors.len()){value.authors.push(Author{id:String::new(),name:String::new(),avatar:author.avatar.as_ref().map(|_|String::new())});}else{self.phase=5;}},
   5 if self.field==input.fields()?=>self.phase=6,
   5=>{if let Some(source)=input.source(self.field){if let Some(start)=self.offset{let mut end=(start+64).min(source.len());while !source.is_char_boundary(end){end-=1;}self.target(input)?.push_str(&source[start..end]);if end==source.len(){self.offset=None;self.field+=1;}else{self.offset=Some(end);}}else{self.target(input)?.try_reserve_exact(source.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"repin original text backing refused"))?;self.offset=Some(0);}}else{self.field+=1;}},
   _=>return Err(invalid()),
  }Ok(paid(d))
 }
 /// 🏁️ The actual reidentified canonical row is complete before publication can borrow it.
 pub fn ready(&self)->bool{self.phase==6&&!self.closing}
 pub fn prepared(&self)->Option<&Checkpoint>{self.ready().then(||self.value.as_ref().unwrap())}
 /// 🤝️ Transfers the same canonical row and adopted original pin allocations.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Checkpoint,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Err(invalid())}let d=RetirementDemand{copy_bytes:2*size_of::<Checkpoint>()+128,depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}self.source=None;self.phase=7;Ok(Some((self.value.take().unwrap(),paid(d))))}
 /// 📐️ Queries the actual partial canonical checkpoint's typed structural retirement.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.value.is_some(){return self.retirement.next_owner_demands::<Checkpoint>(true,&None,body)}Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,usize,usize,usize)>>()+size_of::<bool>(),depth:1,..Default::default()})}
 /// ♻️ Every original pin and partly initialized metadata field closes under its own exact selection.
 pub fn close_step(&mut self,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.retirement.is_empty(){return self.retirement.select(body)?.step(grant)}if self.value.is_some(){return self.retirement.begin_granted(&mut self.value,grant).map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())))}let d=self.close_demands(body)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.source=None;self.closing=true;Ok(RetainedCloneStep::Complete(paid(d)))}
 pub fn terminal_is_empty(&self)->bool{self.value.is_none()&&self.source.is_none()&&self.retirement.is_empty()}
}
impl Drop for OriginalRepinCheckpointBirth{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"repin checkpoint birth requires original partial-field closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.value)}}}}
#[cfg(test)]
#[path="repin-facts-tests.rs"]
mod tests;
