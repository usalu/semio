//! 🌿️ Alternative references follow a reidentified checkpoint through bounded original text comparisons.
use super::*;
use crate::Alternative;
use semio_framework_value::{RetainedCloneStep,retained_clone::RetainedCloneClose};
use std::mem::ManuallyDrop;
/// 🧷️ The original alternative and checkpoint identities stay borrowed until its canonical replacement is complete.
pub struct OriginalAlternativeInput<'a>{pub generation:u64,pub alternative:&'a Alternative,pub checkpoint_id:Option<&'a str>,pub replacement:&'a str}
impl OriginalAlternativeInput<'_>{fn identity(&self)->(u64,[usize;5]){(self.generation,[self.alternative as*const _ as usize,self.checkpoint_id.map_or(0,|v|v.as_ptr()as usize),self.checkpoint_id.map_or(0,str::len),self.replacement.as_ptr()as usize,self.replacement.len()])}fn count(&self)->usize{self.alternative.checkpoint_ids.len()+usize::from(self.checkpoint_id.is_none())}fn source(&self,field:usize,replaced:bool)->Option<&str>{match field{0=>Some(&self.alternative.id),1=>Some(&self.alternative.name),n if n==self.alternative.checkpoint_ids.len()+2&&self.checkpoint_id.is_none()=>Some(self.replacement),n=>self.alternative.checkpoint_ids.get(n-2).map(|value|if replaced{self.replacement}else{value.as_str()})}}}
/// 🧳️ The partial canonical row retains each separately born header and field until original publication or cancellation.
pub struct OriginalAlternativeBirth{value:ManuallyDrop<Option<Alternative>>,source:Option<(u64,[usize;5])>,retirement:RetainedCloneClose,phase:u8,field:usize,offset:Option<usize>,comparison:usize,replaced:bool,closing:bool}
impl Default for OriginalAlternativeBirth{fn default()->Self{Self{value:ManuallyDrop::new(None),source:None,retirement:Default::default(),phase:0,field:0,offset:None,comparison:0,replaced:false,closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"repin alternative changed its original borrowed source")}
fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl OriginalAlternativeBirth{
 fn target(&mut self)->Result<&mut String,ValueError>{let value=self.value.as_mut().ok_or_else(invalid)?;match self.field{0=>Ok(&mut value.id),1=>Ok(&mut value.name),n=>value.checkpoint_ids.get_mut(n-2).ok_or_else(invalid)}}
 /// 📐️ Comparison and copying inspect at most 64 bytes from each actual borrowed field.
 pub fn demands(&self,input:&OriginalAlternativeInput<'_>)->Result<RetirementDemand,ValueError>{let mut d=RetirementDemand{copy_bytes:256,depth:1,..Default::default()};match self.phase{0=>d.copy_bytes+=2*size_of::<Alternative>(),1=>d.capacity_bytes=input.count().checked_mul(size_of::<String>()).ok_or_else(invalid)?,4 if self.offset.is_none()=>d.capacity_bytes=input.source(self.field,self.replaced).map_or(0,str::len),_=>{}}Ok(d)}
 /// 🎟️ Complete canonical identities and names are preserved; every matching reference changes at its original rank.
 pub fn advance(&mut self,input:&OriginalAlternativeInput<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.closing||self.source.is_some_and(|source|source!=input.identity()){return Err(invalid())}if self.ready(){return Ok(Default::default())}let d=self.demands(input)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(input.identity());match self.phase{
  0=>{*self.value=Some(Alternative{id:String::new(),name:String::new(),checkpoint_ids:Vec::new()});self.phase=1;},
  1=>{self.value.as_mut().unwrap().checkpoint_ids.try_reserve_exact(input.count()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"repin alternative original header backing refused"))?;self.phase=2;},
  2=>{let value=self.value.as_mut().unwrap();if value.checkpoint_ids.len()<input.count(){value.checkpoint_ids.push(String::new());}else{self.phase=3;}},
  3=>{if self.field==2+input.count(){self.phase=5;}else if self.field<2||input.checkpoint_id.is_none(){self.replaced=false;self.phase=4;}else{let source=&input.alternative.checkpoint_ids[self.field-2];if source.len()!=input.checkpoint_id.unwrap().len(){self.replaced=false;self.phase=4;}else{let end=(self.comparison+64).min(source.len());if source.as_bytes()[self.comparison..end]!=input.checkpoint_id.unwrap().as_bytes()[self.comparison..end]{self.replaced=false;self.phase=4;}else if end==source.len(){self.replaced=true;self.phase=4;}else{self.comparison=end;}}}},
  4=>{let source=input.source(self.field,self.replaced).ok_or_else(invalid)?;if let Some(start)=self.offset{let mut end=(start+64).min(source.len());while !source.is_char_boundary(end){end-=1;}self.target()?.push_str(&source[start..end]);if end==source.len(){self.field+=1;self.offset=None;self.comparison=0;self.phase=3;}else{self.offset=Some(end);}}else{self.target()?.try_reserve_exact(source.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"repin alternative original text backing refused"))?;self.offset=Some(0);}},
  _=>return Err(invalid()),
 }Ok(paid(d))}
 pub fn ready(&self)->bool{self.phase==5&&!self.closing}
 pub fn prepared(&self)->Option<&Alternative>{self.ready().then(||self.value.as_ref().unwrap())}
 /// 🤝️ The canonical row moves once into the original graph publication owner.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Alternative,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Err(invalid())}let d=RetirementDemand{copy_bytes:2*size_of::<Alternative>()+128,depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}self.source=None;self.phase=6;Ok(Some((self.value.take().unwrap(),paid(d))))}
 /// 📐️ The actual partly authored canonical row supplies its full structural close quote.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.value.is_some(){return self.retirement.next_owner_demands::<Alternative>(true,&None,body)}Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,[usize;5])>>()+size_of::<bool>(),depth:1,..Default::default()})}
 /// ♻️ Closure preserves the exact body selection used by the enclosing original caller's quote.
 pub fn close_step(&mut self,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.retirement.is_empty(){return self.retirement.select(body)?.step(grant)}if self.value.is_some(){return self.retirement.begin_granted(&mut self.value,grant).map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default())))}let d=self.close_demands(body)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.source=None;self.closing=true;Ok(RetainedCloneStep::Complete(paid(d)))}
 pub fn terminal_is_empty(&self)->bool{self.value.is_none()&&self.source.is_none()&&self.retirement.is_empty()}
}
impl Drop for OriginalAlternativeBirth{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"repin alternative requires original partial row closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.value)}}}}

#[cfg(test)]
#[path="repin-alternative-tests.rs"]
mod tests;
