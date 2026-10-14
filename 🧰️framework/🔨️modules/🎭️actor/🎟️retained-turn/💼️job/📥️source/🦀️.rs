//! 🧳️ A guest job crossing retains its original source while caller-funded contiguous copies and closure advance.
use super::*;
use std::mem::ManuallyDrop;
use semio_framework_job::StepContext;
use semio_framework_value::{retirement::controlled::ControlledRetirement,RetirementDemand};

/// 📥️ The existing registered-job source schema carries one owned kind, input and optional checkpoint.
#[derive(Debug,serde::Serialize,serde::Deserialize,semio_framework_value_derive::FromValue,semio_framework_value_derive::ToValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="::protocol::value",rename_all="camelCase")]
pub struct OriginalJobSource{pub job:u64,pub kind:String,pub input:Option<Vec<u8>>,pub checkpoint:Option<Vec<u8>>}

/// ⏱️ The original caller deadline is absolute and shares the caller clock across the actual guest loan.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,semio_framework_value_derive::FromValue,semio_framework_value_derive::ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="::protocol::value",rename_all="camelCase")]
pub struct OriginalJobBudget{pub input:RetainedTurnInput,pub retained_policy:RetainedCloneGrant,pub fuel:u64,pub deadline_us:u64,pub cancelled:bool}
impl OriginalJobBudget{
 pub fn borrow_original(input:RetainedTurnInput,cx:&StepContext<'_>)->Result<Self,ValueError>{input.validate()?;cx.original_source_parent()?;if input.work.clock_binding!=cx.original_clock_binding().ok_or_else(||refusal("original job loan lacks its issuer clock binding"))?||input.work.absolute_deadline_us!=cx.original_absolute_deadline_us()||input.work.fuel>cx.original_total_fuel_remaining(){return Err(refusal("original job loan changed its original clock deadline or renewed total work"))}if (input.operation,input.generation)!=(cx.operation().0,cx.generation().0){return Err(refusal("original job budget belongs to another caller"))}let grant=cx.retained_grant();if grant.maximum_items>input.grant.maximum_items||grant.maximum_copy_bytes>input.grant.maximum_copy_bytes||grant.maximum_capacity_bytes>input.grant.maximum_capacity_bytes||grant.maximum_release_bytes>input.grant.maximum_release_bytes||grant.maximum_depth>input.grant.maximum_depth{return Err(refusal("original job budget exceeds the issuing turn"))}Ok(Self{input,retained_policy:grant,fuel:cx.fuel_remaining(),deadline_us:cx.deadline_us(),cancelled:cx.is_cancelled()})}
 pub fn step_budget(self)->semio_framework_job::StepBudget{semio_framework_job::StepBudget::new(self.fuel,self.deadline_us,self.retained_policy)}
}

/// 🎟️ A persistent crossing debits actual returned retained and CPU work once in its initiating caller.
pub struct OriginalJobTurnLoan{pending:Option<OriginalJobBudget>,clock:Option<fn()->Option<u64>>,identity:Option<(usize,usize,u64,u64)>}
impl OriginalJobTurnLoan{
 pub const fn new()->Self{Self{pending:None,clock:None,identity:None}}
 pub fn checkout(&mut self,input:RetainedTurnInput,cx:&StepContext<'_>)->Result<Option<OriginalJobBudget>,ValueError>{if self.pending.is_some(){return Err(refusal("original job crossing already has an outstanding turn"))}let budget=OriginalJobBudget::borrow_original(input,cx)?;if cx.should_yield(){return Ok(None)}self.pending=Some(budget);self.clock=Some(cx.clock_source());self.identity=Some((cx.original_source_parent()?as*const _ as usize,cx.original_recipient_identity(),cx.original_clock_binding().ok_or_else(||refusal("original job crossing lacks bound clock"))?,cx.original_absolute_deadline_us()));Ok(Some(budget))}
 pub fn clock_source(&self)->Option<fn()->Option<u64>>{self.clock}
 pub fn pending(&self)->Option<OriginalJobBudget>{self.pending}
 pub fn receive(&mut self,input:RetainedTurnInput,spent:RetainedCloneProgress,work_units_spent:u64,cx:&mut StepContext<'_>)->Result<(),ValueError>{let original=self.pending.ok_or_else(||refusal("original job crossing receipt has already returned"))?;if self.identity!=Some((cx.original_source_parent()?as*const _ as usize,cx.original_recipient_identity(),cx.original_clock_binding().ok_or_else(||refusal("original job returned context lacks bound clock"))?,cx.original_absolute_deadline_us()))||input!=original.input||(cx.operation().0,cx.generation().0)!=(input.operation,input.generation)||cx.deadline_us()!=original.deadline_us||cx.fuel_remaining()!=original.fuel||cx.retained_grant()!=original.retained_policy||self.clock.map(|clock|clock as usize)!=Some(cx.clock_source() as usize){return Err(refusal("original job crossing receipt differs from its initiating loan"))}self.pending=None;self.clock=None;self.identity=None;cx.consume_fuel(work_units_spent);let retained=cx.consume_retained(spent);retained?;if work_units_spent>original.fuel{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original guest returned CPU work beyond its caller loan").with_retained_progress(spent))}input.return_original(spent,work_units_spent)?;Ok(())}
 pub fn terminal_is_empty(&self)->bool{self.pending.is_none()&&self.clock.is_none()&&self.identity.is_none()}
}
impl Default for OriginalJobTurnLoan{fn default()->Self{Self::new()}}
impl Drop for OriginalJobTurnLoan{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original job crossing dropped before its actual returned work receipt")}}

/// ⚠️ The guest retains semantic causes; an authority refusal preserves its defining first-party identity.
#[derive(Clone,Copy,Debug,serde::Serialize,serde::Deserialize)]
#[serde(tag="kind",content="refusal",rename_all="camelCase")]
pub enum OriginalJobCallFailure{Authority(ValueRefusalKind),Semantic}
semio_framework_value::artifact_retire_leaf!(OriginalJobCallFailure);
/// 📬️ Admission returns the exact incoming source when it was refused while another source stayed retained.
#[derive(Debug,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct OriginalJobStartReply{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub work_units_spent:u64,pub admitted:bool,pub returned_source:Option<OriginalJobSource>,pub refusal:Option<OriginalJobCallFailure>}
/// 🛑️ Cancellation acknowledgement means admission; actual source closure arrives on a later terminal job step.
#[derive(Debug,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct OriginalJobCancelReply{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub work_units_spent:u64,pub admitted:bool,pub refusal:Option<OriginalJobCallFailure>}

/// ▶️ A pending turn may have no payload; source closure is acknowledged independently of semantic output.
#[derive(Debug,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct OriginalJobStepReply<T>{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub work_units_spent:u64,pub outcome:Option<T>,pub source_closed:bool,pub refusal:Option<OriginalJobCallFailure>}

/// 🤝️ The original Host request remains in custody until the real guest releases its admitted copy.
pub struct OriginalJobSourceCursor{original:ManuallyDrop<Option<OriginalJobSource>>,prepared:ManuallyDrop<Option<OriginalJobSource>>,bytes:ManuallyDrop<Vec<u8>>,field:u8,position:usize,allocated:bool,identity:(u64,u64),source_outstanding:bool,closing:bool,retirement:ManuallyDrop<Option<ControlledRetirement<OriginalJobSource>>>}
impl OriginalJobSourceCursor{
 pub fn new(original:OriginalJobSource,input:RetainedTurnInput)->Result<Self,(ValueError,OriginalJobSource)>{if let Err(error)=input.validate(){return Err((error,original))}Ok(Self{original:ManuallyDrop::new(Some(original)),prepared:ManuallyDrop::new(None),bytes:ManuallyDrop::new(Vec::new()),field:0,position:0,allocated:false,identity:(input.operation,input.generation),source_outstanding:false,closing:false,retirement:ManuallyDrop::new(None)})}
 pub fn original(&self)->Option<&OriginalJobSource>{self.original.as_ref()}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if self.identity!=(cx.operation().0,cx.generation().0){return Err(refusal("original job source loan belongs to another operation or generation"))}Ok(())}
 fn field_source(&self)->Result<Option<&[u8]>,ValueError>{let source=self.original.as_ref().ok_or_else(||refusal("original job source has already closed"))?;Ok(match self.field{0=>Some(source.kind.as_bytes()),1=>source.input.as_deref(),2=>source.checkpoint.as_deref(),_=>None})}
 /// ✍️ Allocates one exact destination once, then copies only the original granted octets without reallocation.
 pub fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  self.validate(cx)?;if self.closing{return Err(refusal("original job source is closing"))}if self.source_outstanding{return Ok(false)}if self.field==3{return Ok(true)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  if self.prepared.is_none(){let job=self.original.as_ref().unwrap().job;*self.prepared=Some(OriginalJobSource{job,kind:String::new(),input:None,checkpoint:None});cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  let source=self.field_source()?;let length=source.map_or(0,<[u8]>::len);
  if !self.allocated{if grant.maximum_capacity_bytes<length{return Ok(false)}if length>0{self.bytes.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original job crossing destination allocation failed"))?;}self.allocated=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.bytes.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if self.position<length{let count=grant.maximum_copy_bytes.min(64).min(length-self.position);if count==0{return Ok(false)}let original=self.original.as_ref().unwrap();let source=match self.field{0=>original.kind.as_bytes(),1=>original.input.as_ref().unwrap(),2=>original.checkpoint.as_ref().unwrap(),_=>unreachable!()};self.bytes.extend_from_slice(&source[self.position..self.position+count]);self.position+=count;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  let present=self.field_source()?.is_some();let bytes=std::mem::take(&mut*self.bytes);let prepared=self.prepared.as_mut().unwrap();match self.field{0=>prepared.kind=unsafe{String::from_utf8_unchecked(bytes)},1=>prepared.input=present.then_some(bytes),2=>prepared.checkpoint=present.then_some(bytes),_=>unreachable!()};self.field+=1;self.position=0;self.allocated=false;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(self.field==3)
 }
 /// 📤️ Transfers only the admitted copy; the exact original source stays held through guest execution.
 pub fn take_prepared(&mut self,cx:&mut StepContext<'_>)->Result<Option<OriginalJobSource>,ValueError>{self.validate(cx)?;let grant=cx.retained_grant();if self.closing||self.field!=3||self.source_outstanding||grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return Ok(None)}let source=self.prepared.take().ok_or_else(||refusal("original job source has no prepared copy"))?;self.source_outstanding=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(Some(source))}
 /// ↩️ A refused crossing returns its whole prepared source to the same original cursor.
 pub fn receive_returned(&mut self,source:&mut Option<OriginalJobSource>,cx:&mut StepContext<'_>)->Result<bool,ValueError>{self.validate(cx)?;if !self.source_outstanding||self.prepared.is_some()||source.is_none(){return Err(refusal("original job returned source has no matching outstanding loan"))}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||cx.fuel_remaining()==0{return Ok(false)}*self.prepared=source.take();self.source_outstanding=false;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 pub fn source_outstanding(&self)->bool{self.source_outstanding}
 /// 🏁️ Receives the actual terminal guest source-close acknowledgement before releasing original Host input.
 pub fn receive_guest_source_closed(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{self.validate(cx)?;if !self.source_outstanding{return Err(refusal("original job source has no outstanding guest copy"))}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||cx.fuel_remaining()==0{return Ok(false)}self.source_outstanding=false;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn terminal_is_empty(&self)->bool{!self.source_outstanding&&self.original.is_none()&&self.prepared.is_none()&&self.bytes.is_empty()&&self.bytes.capacity()==0&&self.retirement.is_none()}
 /// 📏️ Exact existing retirement producers quote source backing; a live guest copy prevents source release.
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.source_outstanding{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original job source still has an outstanding guest copy"))}if let Some(owner)=self.retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||refusal("original job source retirement depth overflow"))?})}if self.bytes.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.bytes.capacity(),depth:1,..Default::default()})}Ok(RetirementDemand{depth:usize::from(self.original.is_some()||self.prepared.is_some()),..Default::default()})
 }
 /// ♻️ Child-first closure keeps original and partially prepared bytes under the actual external recipient.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
  self.validate(cx)?;self.closing=true;let grant=cx.retained_grant();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;if cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(false)}
  if self.bytes.capacity()!=0{let bytes=self.bytes.capacity();drop(std::mem::take(&mut*self.bytes));cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if let Some(owner)=self.retirement.as_mut(){if !owner.terminal_is_empty(){let step=owner.step(RetainedCloneGrant{maximum_items:grant.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)),maximum_depth:grant.maximum_depth-1,..grant}).map_err(|error|{let progress=error.retained_progress();cx.consume_fuel(progress.copied_items as u64);let _=cx.consume_retained(progress);error})?;cx.consume_fuel(step.progress().copied_items as u64);cx.consume_retained(step.progress())?;return Ok(false)}drop(self.retirement.take());cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  let field=if self.prepared.is_some(){&mut*self.prepared}else{&mut*self.original};if let Some(source)=field.take(){match ControlledRetirement::new(source){Ok(owner)=>*self.retirement=Some(owner),Err((error,source))=>{*field=Some(source);return Err(error)}}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}Ok(true)
 }
}
impl Drop for OriginalJobSourceCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original job crossing source dropped before its paid guest acknowledgement and source closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
