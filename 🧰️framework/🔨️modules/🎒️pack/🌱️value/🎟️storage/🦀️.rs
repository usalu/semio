//! 🎟️ Original expectation-stack admission and release use separate caller currencies.
use super::{Expect,PackRefusal,RetainedRecordBodyCursor,RetainedValueCursor,ValueRefusalKind};
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;
fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
impl RetainedValueCursor{
 /// 📏️ Quotes the original Vec header separately from its exact backing capacity.
 pub fn storage_admission_demands(&mut self)->Result<RetirementDemand,PackRefusal>{
  Ok(match self.next_allocation_bytes()?{Some(bytes)=>RetirementDemand{copy_bytes:size_of::<Vec<Expect>>(),capacity_bytes:bytes,release_bytes:0,depth:1},None=>Default::default()})
 }
 /// 📦️ Allocates the same original stack only after every independent currency is present.
 pub fn admit_storage(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,PackRefusal>{
  let demand=self.storage_admission_demands()?;if demand.capacity_bytes==0||!funded(grant,demand){return Ok(Default::default())}
  if self.stack.try_reserve_exact(self.maximum_frames).is_err(){let fault=PackRefusal::RetainedAllocation{kind:ValueRefusalKind::AllocationFailed,allocated_bytes:0,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation"};self.fault.get_or_insert_with(||fault.clone());return Err(fault)}
  let actual=self.stack_allocation_bytes();if actual>demand.capacity_bytes{let fault=PackRefusal::RetainedAllocation{kind:ValueRefusalKind::OwnershipLimit,allocated_bytes:actual,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation overgrant"};self.fault.get_or_insert_with(||fault.clone());return Err(fault)}
  Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:actual,released_bytes:0})
 }
 /// 📏️ Quotes one actual pending owner, scalar frame, Vec header release, or terminal flags.
 pub fn storage_retirement_demands(&self)->Result<RetirementDemand,PackRefusal>{
  if self.closed{return Ok(Default::default())}
  let copy=if self.pending.is_some(){size_of::<Option<(u64,u8)>>()+size_of::<bool>()}else if !self.stack.is_empty(){size_of::<Expect>()+size_of::<usize>()+size_of::<u8>()+size_of::<bool>()}else if self.stack_allocation_bytes()!=0{size_of::<Vec<Expect>>()+size_of::<u8>()+size_of::<bool>()}else{size_of::<u8>()+2*size_of::<bool>()};
  Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:0,release_bytes:if self.pending.is_none()&&self.stack.is_empty(){self.stack_allocation_bytes()}else{0},depth:1})
 }
 /// ♻️ Keeps the original backing and root state unchanged on each independently denied grant.
 pub fn retire_storage(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>{
  if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.storage_retirement_demands()?;if !funded(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.closing=true;let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if self.pending.take().is_some(){return Ok(RetainedCloneStep::Progress(receipt))}
  if self.stack.pop().is_some(){if self.stack.is_empty(){self.initialized_roots=0}return Ok(RetainedCloneStep::Progress(receipt))}
  self.initialized_roots=0;if demand.release_bytes!=0{*self.stack=Vec::new();receipt.released_bytes=demand.release_bytes;return Ok(RetainedCloneStep::Progress(receipt))}
  self.closed=true;Ok(RetainedCloneStep::Complete(receipt))
 }
}
impl RetainedRecordBodyCursor{
 /// 📏️ Quotes the next symbol or value-stack backing separately from any metadata move.
 pub fn record_storage_admission_demands(&mut self)->Result<RetirementDemand,PackRefusal>{
  Ok(match self.next_allocation_bytes()?{Some(bytes)=>RetirementDemand{copy_bytes:0,capacity_bytes:bytes,release_bytes:0,depth:1},None=>Default::default()})
 }
 /// 📦️ Allocates exactly the quoted backing only after every independent currency is present.
 pub fn admit_record_storage(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,PackRefusal>{
  let demand=self.record_storage_admission_demands()?;if demand.capacity_bytes==0||!funded(grant,demand){return Ok(Default::default())}
  let step=self.reserve_allocation(demand.capacity_bytes).map_err(|error|error.fault)?;
  Ok(RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()})
 }
 /// 📏️ Quotes one logical transition or one exact physical backing release of the record-body close.
 pub fn record_storage_retirement_demands(&self,_maximum_body_bytes:usize)->Result<RetirementDemand,PackRefusal>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  Ok(RetirementDemand{copy_bytes:0,capacity_bytes:0,release_bytes:self.next_release_allocation_bytes()?.unwrap_or(0),depth:1})
 }
 /// ♻️ Keeps every owner and phase unchanged on an unfunded grant; a funded turn is exactly one close transition.
 pub fn retire_record_storage(&mut self,maximum_body_bytes:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>{
  use crate::format::RetainedPackCloseStep;
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.record_storage_retirement_demands(maximum_body_bytes)?;if !funded(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  Ok(match self.close_step(1,demand.release_bytes)?{
   RetainedPackCloseStep::Pending{released_bytes,..}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}),
   RetainedPackCloseStep::Complete=>RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),
  })
 }
}
