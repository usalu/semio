//! 🔑️ Exact immutable key lookup funds byte pages and never treats a prefix as identity.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,RETAINED_CLONE_BULK_PAGE_BYTES}};
use std::mem::size_of;
/// 🧭️ The caller retains immutable key/directory sources and rejects changed source revisions before reusing this cursor.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ExactKeyCursor{index:usize,offset:usize,found:Option<usize>,ready:bool}
impl ExactKeyCursor{
 pub fn new()->Self{Self::default()}
 pub fn next_index(&self)->usize{self.index}
 pub fn ready(&self)->bool{self.ready}
 pub fn found(&self)->Option<usize>{self.ready.then_some(self.found).flatten()}
 pub fn fixed_copy_bytes()->usize{size_of::<Self>()*2+size_of::<usize>()*2}
 pub fn demand(&self,key:&[u8],candidate:Option<&[u8]>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.ready{return Ok(Default::default());}
  if self.offset>key.len()||self.offset>0&&candidate.is_none_or(|candidate|candidate.len()!=key.len()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"exact lookup original candidate changed"));}
  let remaining=if candidate.is_some_and(|candidate|candidate.len()==key.len()){key.len()-self.offset}else{0};
  let count=remaining.min(body.saturating_sub(Self::fixed_copy_bytes()).max(2)/2).min(RETAINED_CLONE_BULK_PAGE_BYTES);
  Ok(RetirementDemand{copy_bytes:Self::fixed_copy_bytes()+2*count,depth:1,..Default::default()})
 }
 pub fn advance(&mut self,key:&[u8],candidate:Option<&[u8]>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.ready{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.demand(key,candidate,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let count=(demand.copy_bytes-Self::fixed_copy_bytes())/2;
  if let Some(candidate)=candidate{
   if candidate.len()!=key.len()||candidate[self.offset..self.offset+count]!=key[self.offset..self.offset+count]{self.index=self.index.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"exact lookup original source ordinal overflow"))?;self.offset=0;}
   else{self.offset+=count;if self.offset==key.len(){self.found=Some(self.index);self.ready=true;}}
  }else{self.ready=true;}
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  Ok(if self.ready{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
