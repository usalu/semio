//! ✂️ Original byte spans copy only admitted range fragments and retain exact output/source custody.
use super::retained_mutation_demand_fits;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,close_retained_binding,admit_retained_clone_progress}};
use std::mem::{ManuallyDrop,size_of};

/// 🧵️ A byte-range cursor owns one staged contiguous range and its genuine original source alias.
pub struct RetainedByteSpan{start:usize,len:usize,copied:usize,phase:u8,source:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<Vec<u8>>>,retirement:RetainedCloneClose,closing:bool}
impl RetainedByteSpan{
 pub fn new(start:usize,len:usize)->Self{Self{start,len,copied:0,phase:0,source:None,output:ManuallyDrop::new(None),retirement:Default::default(),closing:false}}
 pub fn demand(&self,original:RetainedCloneRef<'_,Vec<u8>>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>2{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original byte span is closing or transferred"));}
  if !original.check_binding(&self.source)?{return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.start>original.get().len()||self.len>original.get().len()-self.start{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original byte span exceeds its base"));}
  let fixed=size_of::<Vec<u8>>()+size_of::<usize>()*2+size_of::<u8>();
  let copy=if self.phase==0{fixed}else{fixed+self.len.saturating_sub(self.copied).min(body.max(1)).min(semio_framework_value::retained_clone::RETAINED_CLONE_BULK_PAGE_BYTES)};
  Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:if self.phase==0{self.len}else{0},depth:1,..Default::default()})
 }
 pub fn advance(&mut self,original:RetainedCloneRef<'_,Vec<u8>>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand(original,body)?;if !retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(p)=original.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(p));}
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if self.phase==0{let mut output=Vec::new();output.try_reserve_exact(self.len).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original byte span reservation failed"))?;p.retained_capacity_bytes=output.capacity();*self.output=Some(output);self.phase=if self.len==0{2}else{1};}
  else if self.phase==1{let fixed=size_of::<Vec<u8>>()+size_of::<usize>()*2+size_of::<u8>();let count=d.copy_bytes-fixed;if count==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let end=self.copied+count;self.output.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original byte span lost its admitted output"))?.extend_from_slice(&original.get()[self.start+self.copied..self.start+end]);self.copied=end;if self.copied==self.len{self.phase=2;}}
  else{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let p=admit_retained_clone_progress(grant,p,"original byte span copy")?;Ok(if self.phase==2{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==2&&self.output.is_some()}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original byte span is not ready"));}Ok(RetirementDemand{copy_bytes:size_of::<Vec<u8>>()*2+size_of::<u8>(),depth:1,..Default::default()})}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<u8>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,grant){return Ok(None);}self.phase=3;Ok(Some((self.output.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original byte span close has not begun"));}self.retirement.next_owner_demands::<Vec<u8>>(self.output.is_some(),&self.source,body)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|p|RetainedCloneStep::Progress(p.progress()));}if let Some(p)=self.retirement.begin_granted(&mut self.output,grant)?{return Ok(RetainedCloneStep::Progress(p.progress()));}close_retained_binding(&mut self.source,grant)}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.source.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl Drop for RetainedByteSpan{fn drop(&mut self){let empty=self.source.is_none()&&self.output.is_none()&&self.retirement.is_empty();assert!(std::thread::panicking()||empty,"original byte span requires exact funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.output);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
