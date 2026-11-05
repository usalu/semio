//! 🔎️ The original authoring row comparison copies only a bounded source chunk.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,retained_clone::retained_clone_grant_admits};
use std::mem::size_of;
pub(super) struct TextCompare{binding:Option<(usize,usize,usize,usize)>,offset:usize,done:Option<bool>}
impl TextCompare{
 pub(super) fn new()->Self{Self{binding:None,offset:0,done:None}}
 pub(super) fn step(&mut self,left:&str,right:&str,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
  let binding=(left.as_ptr()as usize,left.len(),right.as_ptr()as usize,right.len());if self.binding.is_some_and(|original|original!=binding){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original authoring text comparison changed its borrowed fields").into())}
  if let Some(done)=self.done{return Ok(Some(done))}let grant=cx.retained_grant();let metadata=size_of::<Self>();let count=if left.len()==right.len(){grant.maximum_copy_bytes.saturating_sub(metadata).min(64).min(left.len().saturating_sub(self.offset))}else{0};let d=RetirementDemand{copy_bytes:metadata+count,depth:1,..Default::default()};if cx.should_yield()||!retained_clone_grant_admits(grant,d)||(left.len()==right.len()&&self.offset<left.len()&&count==0){return Ok(None)}
  self.binding=Some(binding);if left.len()!=right.len(){self.done=Some(false)}else if self.offset==left.len(){self.done=Some(true)}else{let mut bytes=[0;64];bytes[..count].copy_from_slice(&left.as_bytes()[self.offset..self.offset+count]);if bytes[..count]!=right.as_bytes()[self.offset..self.offset+count]{self.done=Some(false)}self.offset+=count;if self.offset==left.len()&&self.done.is_none(){self.done=Some(true)}}
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(self.done)
 }
 pub(super) fn reset(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()};if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}*self=Self::new();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
}
