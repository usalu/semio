//! 🔢️ Original unsigned port text is parsed in paid byte chunks with overflow preserved.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use std::mem::size_of;
pub(super) struct UnsignedWork{binding:Option<(usize,usize)>,offset:usize,value:u64,any:bool,valid:bool,done:bool}
impl UnsignedWork{
 pub(super) fn new()->Self{Self{binding:None,offset:0,value:0,any:false,valid:true,done:false}}
 pub(super) fn step(&mut self,source:&str,maximum:u64,cx:&mut StepContext<'_>)->Result<Option<Option<u64>>,OriginalTurnFailure>{
  let binding=(source.as_ptr()as usize,source.len());if self.binding.is_some_and(|b|b!=binding){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original unsigned text changed its borrowed bytes").into())}if self.done{return Ok(Some((self.valid&&self.any).then_some(self.value)))}
  let count=64.min(source.len().saturating_sub(self.offset));let d=RetirementDemand{copy_bytes:size_of::<Self>()+count,depth:1,..Default::default()};if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(None)}
  self.binding=Some(binding);let end=self.offset+count;while self.offset<end{let byte=source.as_bytes()[self.offset];if self.offset==0&&byte==b'+'{}else if byte.is_ascii_digit(){self.any=true;if self.valid{if let Some(next)=self.value.checked_mul(10).and_then(|v|v.checked_add(u64::from(byte-b'0'))).filter(|v|*v<=maximum){self.value=next}else{self.valid=false}}}else{self.valid=false}self.offset+=1}
  self.done=self.offset==source.len();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(self.done.then_some((self.valid&&self.any).then_some(self.value)))
 }
 pub(super) fn reset(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()};if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}*self=Self::new();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
}
