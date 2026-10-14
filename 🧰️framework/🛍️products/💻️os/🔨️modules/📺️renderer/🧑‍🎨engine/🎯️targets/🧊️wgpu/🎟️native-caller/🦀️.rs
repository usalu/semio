//! 🎟️ The native window root explicitly issues callbacks on one paid source and lifetime clock.
use semio_framework_actor::RetainedTurnInput;
use semio_framework_job::{Generation,OperationId,OriginalTurnFailure,StepBudget,StepContext};
use semio_framework_actor::retained_turn::context::{OriginalActorAdmission,OriginalActorCall,OriginalActorContextError};
use semio_framework_plugin_interpreter::original_actor_context::{OriginalActorEntryFailure,OriginalActorGuestEntry};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};

/// 📋️ The program authors one lifetime's immutable work, deadline duration and physical currencies.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct NativeFrameRootPolicyV1{pub fuel:u64,pub maximum_elapsed_microseconds:u64,pub grant:RetainedCloneGrant}
/// 🏛️ One window lifetime retains its original operation and generation across presentation changes.
pub struct NativeFrameCaller{entry:OriginalActorGuestEntry,operation:OperationId,generation:Generation,clock:fn()->Option<u64>,policy:NativeFrameRootPolicyV1,deadline:u64,last_clock:u64,fuel:u64,epoch:u64,recipient:RetainedCloneProgress,ready:bool,closed:bool}
impl NativeFrameCaller{
 /// 🌱️ Inline construction accepts explicit program identity and policy before any native root backing birth.
 pub fn new(operation:OperationId,generation:Generation,clock:fn()->Option<u64>,policy:NativeFrameRootPolicyV1)->Result<Self,ValueError>{
  RetainedTurnInput{operation:operation.0,generation:generation.0,epoch:1,grant:policy.grant}.validate()?;
  if policy.maximum_elapsed_microseconds==0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native root turn duration must be finite and positive"))}
  let now=clock().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native root requires its supplied lifetime clock"))?;let deadline=now.checked_add(policy.maximum_elapsed_microseconds).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native root absolute deadline overflow"))?;
  Ok(Self{entry:OriginalActorGuestEntry::new(),operation,generation,clock,policy,deadline,last_clock:now,fuel:policy.fuel,epoch:0,recipient:Default::default(),ready:false,closed:false})
 }
 fn remaining(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.policy.grant.maximum_items.saturating_sub(self.recipient.copied_items),maximum_copy_bytes:self.policy.grant.maximum_copy_bytes.saturating_sub(self.recipient.copied_bytes),maximum_capacity_bytes:self.policy.grant.maximum_capacity_bytes.saturating_sub(self.recipient.retained_capacity_bytes),maximum_release_bytes:self.policy.grant.maximum_release_bytes.saturating_sub(self.recipient.released_bytes),..self.policy.grant}}
 fn issue(&mut self)->Result<(RetainedTurnInput,StepBudget),ValueError>{
  if self.closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native root already reached terminal closure"))}
  let now=(self.clock)().filter(|now|*now>=self.last_clock).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native root lost its monotone original lifetime clock"))?;self.last_clock=now;
  let epoch=self.epoch.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native root issued epoch overflow"))?;self.epoch=epoch;
  let input=RetainedTurnInput{operation:self.operation.0,generation:self.generation.0,epoch,grant:self.policy.grant};Ok((input,StepBudget::new(self.fuel,self.deadline,input.grant)))
 }
 fn permits(&self,budget:StepBudget)->Result<bool,ValueError>{let remaining=self.remaining();Ok(budget.fuel!=0&&self.last_clock<self.deadline&&remaining.maximum_items!=0&&remaining.maximum_depth!=0)}
 fn receive_raw(&mut self,spent:RetainedCloneProgress,work:u64)->Result<(),ValueError>{self.recipient=self.recipient.checked_add(spent)?;self.fuel=self.fuel.saturating_sub(work);if !self.recipient.fits(self.policy.grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native original lifetime recipient exceeds its immutable stock").with_retained_progress(spent))}Ok(())}
 fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}
 pub fn fuel_remaining(&self)->u64{self.fuel}
 pub fn retained_remaining(&self)->RetainedCloneGrant{self.remaining()}
 pub fn retained_spent(&self)->RetainedCloneProgress{self.recipient}
 pub fn deadline_us(&self)->u64{self.deadline}
 pub fn ready(&self)->bool{self.ready}
 /// 🪴️ Binding, cancellation, ledger and source births yield separately under actual issued root receipts.
 pub fn admit_step(&mut self)->Result<OriginalActorCall<OriginalActorAdmission,ValueError>,ValueError>{let(mut input,budget)=self.issue()?;input.grant=self.remaining();if !self.permits(budget)?{return Ok(OriginalActorCall{input,receipt_spent:Default::default(),work_units_spent:0,result:Ok(OriginalActorAdmission{ready:false,progress:Default::default()})})}let mut call=self.entry.admit_step(input);call.work_units_spent=1;self.receive_raw(call.receipt_spent,call.work_units_spent)?;if let Ok(admission)=&call.result{self.ready=admission.ready}Ok(call)}
 /// ▶️ The body borrows the exact source, cumulative recipient under its immutable stock ceiling; the actual context exposes only remaining currencies.
 pub fn dispatch_step<T>(&mut self,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<OriginalActorCall<T,OriginalActorEntryFailure>,ValueError>{let(input,budget)=self.issue()?;if !self.permits(budget)?{return Ok(Self::denied(input))}let before=self.recipient;let mut call=self.entry.dispatch_value_step(input,budget,&mut self.recipient,self.clock,produce);call.receipt_spent=Self::delta(before,self.recipient);self.fuel=self.fuel.saturating_sub(call.work_units_spent);Ok(call)}
 /// ♻️ Original cleanup remains available after semantic failure and original cancellation.
 pub fn dispatch_cleanup_step<T>(&mut self,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<OriginalActorCall<T,OriginalActorEntryFailure>,ValueError>{let(input,budget)=self.issue()?;if !self.permits(budget)?{return Ok(Self::denied(input))}let before=self.recipient;let mut call=self.entry.dispatch_cleanup_step(input,budget,&mut self.recipient,self.clock,produce);call.receipt_spent=Self::delta(before,self.recipient);self.fuel=self.fuel.saturating_sub(call.work_units_spent);Ok(call)}
 fn denied<T>(input:RetainedTurnInput)->OriginalActorCall<T,OriginalActorEntryFailure>{OriginalActorCall{input,receipt_spent:Default::default(),work_units_spent:0,result:Err(OriginalActorEntryFailure::Authority(ValueError::literal(ValueRefusalKind::WorkLimit,"original native root callback has no supplied work or depth")))}}
 pub fn original_failure(&self)->Option<&OriginalTurnFailure>{self.entry.original_job_refusal()}
 pub fn original_cleanup_failure(&self)->Option<&OriginalTurnFailure>{self.entry.original_cleanup_refusal()}
 /// 🧾️ The same original entry closes each retained owner before terminal native root removal.
 pub fn close_step(&mut self)->Result<OriginalActorCall<RetainedCloneStep,OriginalActorContextError>,ValueError>{let(mut input,budget)=self.issue()?;input.grant=self.remaining();if !self.permits(budget)?&&!self.entry.terminal_is_empty(){return Ok(OriginalActorCall{input,receipt_spent:Default::default(),work_units_spent:0,result:Ok(RetainedCloneStep::Progress(Default::default()))})}let mut call=self.entry.close_step(input);call.work_units_spent=u64::from(call.receipt_spent!=Default::default());self.receive_raw(call.receipt_spent,call.work_units_spent)?;if matches!(&call.result,Ok(RetainedCloneStep::Complete(_)))&&self.entry.terminal_is_empty(){self.ready=false;self.closed=true}Ok(call)}
 pub fn terminal_is_empty(&self)->bool{self.entry.terminal_is_empty()}
}
