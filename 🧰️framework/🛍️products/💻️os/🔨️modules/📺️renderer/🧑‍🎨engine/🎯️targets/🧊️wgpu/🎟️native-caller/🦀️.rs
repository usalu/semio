//! 🎟️ Each initiating native event retains one paid source and its immutable original clock.
use semio_framework_actor::{OriginalActorTurnWork,RetainedTurnInput};
use semio_framework_job::{Generation,OperationId,OriginalTurnFailure,StepBudget,StepContext};
use semio_framework_actor::retained_turn::context::{OriginalActorAdmission,OriginalActorCall,OriginalActorContextError};
use semio_framework_plugin_guest_contract::original_actor_context::{OriginalActorEntryFailure,OriginalActorGuestEntry};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_trace::allocate_operation_id;

/// 📋️ The program authors one event's immutable work, deadline duration and physical currencies.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct NativeFrameRootPolicyV1{pub fuel:u64,pub maximum_elapsed_microseconds:u64,pub grant:RetainedCloneGrant,pub retirement_reserve:NativeFrameRetirementReserveV1}
/// 🪙️ Initially reserved stock pays physical return after semantic work stops, without replenishment.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct NativeFrameRetirementReserveV1{pub fuel:u64,pub grant:RetainedCloneGrant}
impl NativeFrameRootPolicyV1{
 /// 🪟️ The program authors one complete event lifetime separately from its one-item callback loan.
 pub const fn initiating_event()->Self{Self{fuel:67_108_864,maximum_elapsed_microseconds:30_000_000,grant:RetainedCloneGrant{maximum_items:33_554_432,maximum_copy_bytes:536870912,maximum_capacity_bytes:268435456,maximum_release_bytes:536870912,maximum_depth:256},retirement_reserve:NativeFrameRetirementReserveV1{fuel:8_388_608,grant:RetainedCloneGrant{maximum_items:4_194_304,maximum_copy_bytes:8388608,maximum_capacity_bytes:4194304,maximum_release_bytes:268435456,maximum_depth:256}}}}
 fn validate(self)->Result<(),ValueError>{let reserve=self.retirement_reserve;let stock=RetainedCloneProgress{copied_items:reserve.grant.maximum_items,copied_bytes:reserve.grant.maximum_copy_bytes,retained_capacity_bytes:reserve.grant.maximum_capacity_bytes,released_bytes:reserve.grant.maximum_release_bytes};if self.maximum_elapsed_microseconds==0||reserve.fuel>self.fuel||!stock.fits(self.grant)||reserve.grant.maximum_depth>self.grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native event requires a finite positive duration and an initially funded retirement reserve"))}Ok(())}
 fn business_grant(self)->RetainedCloneGrant{let reserve=self.retirement_reserve.grant;RetainedCloneGrant{maximum_items:self.grant.maximum_items-reserve.maximum_items,maximum_copy_bytes:self.grant.maximum_copy_bytes-reserve.maximum_copy_bytes,maximum_capacity_bytes:self.grant.maximum_capacity_bytes-reserve.maximum_capacity_bytes,maximum_release_bytes:self.grant.maximum_release_bytes-reserve.maximum_release_bytes,..self.grant}}
}
/// 🏛️ One actual event retains its original operation and generation across presentation changes.
pub struct NativeFrameCaller{entry:OriginalActorGuestEntry,operation:OperationId,generation:Generation,clock:fn()->Option<u64>,clock_binding:u64,policy:NativeFrameRootPolicyV1,deadline:u64,last_clock:u64,fuel:u64,epoch:u64,recipient:RetainedCloneProgress,ready:bool,closed:bool}
impl NativeFrameCaller{
 /// 🌱️ Inline construction accepts explicit program identity and policy before any native root backing birth.
 pub fn new(operation:OperationId,generation:Generation,clock:fn()->Option<u64>,policy:NativeFrameRootPolicyV1)->Result<Self,ValueError>{
  if operation.0==0||generation.0==0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"native event source requires its actual operation and generation"))}
  policy.validate()?;
  let now=clock().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native root requires its supplied lifetime clock"))?;let deadline=now.checked_add(policy.maximum_elapsed_microseconds).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native root absolute deadline overflow"))?;
  Ok(Self{entry:OriginalActorGuestEntry::new(),operation,generation,clock,clock_binding:allocate_operation_id().0,policy,deadline,last_clock:now,fuel:policy.fuel,epoch:0,recipient:Default::default(),ready:false,closed:false})
 }
 fn remaining(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.policy.grant.maximum_items.saturating_sub(self.recipient.copied_items),maximum_copy_bytes:self.policy.grant.maximum_copy_bytes.saturating_sub(self.recipient.copied_bytes),maximum_capacity_bytes:self.policy.grant.maximum_capacity_bytes.saturating_sub(self.recipient.retained_capacity_bytes),maximum_release_bytes:self.policy.grant.maximum_release_bytes.saturating_sub(self.recipient.released_bytes),..self.policy.grant}}
 fn issue(&mut self,_retirement:bool)->Result<(RetainedTurnInput,StepBudget),ValueError>{
  if self.closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native root already reached terminal closure"))}
  let now=(self.clock)().filter(|now|*now>=self.last_clock).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native root lost its monotone original lifetime clock"))?;self.last_clock=now;
  let epoch=self.epoch.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native root issued epoch overflow"))?;self.epoch=epoch;
  let fuel=self.fuel;let input=RetainedTurnInput{operation:self.operation.0,generation:self.generation.0,epoch,work:OriginalActorTurnWork{clock_binding:self.clock_binding,absolute_deadline_us:self.deadline,fuel},grant:self.remaining()};Ok((input,StepBudget::new(fuel,self.deadline,input.grant)))
 }
 fn remaining_business(&self)->RetainedCloneGrant{let grant=self.policy.business_grant();RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(self.recipient.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(self.recipient.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(self.recipient.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(self.recipient.released_bytes),..grant}}
 fn permits(&self,budget:StepBudget)->Result<bool,ValueError>{let remaining=self.remaining_business();Ok(budget.fuel>self.policy.retirement_reserve.fuel&&self.last_clock<self.deadline&&remaining.maximum_items!=0&&remaining.maximum_depth!=0)}
 fn permits_retirement(&self,budget:StepBudget)->bool{let remaining=self.remaining();budget.fuel!=0&&remaining.maximum_items!=0&&remaining.maximum_depth!=0}
 fn receive_raw(&mut self,spent:RetainedCloneProgress,work:u64)->Result<(),ValueError>{self.recipient=self.recipient.checked_add(spent)?;self.fuel=self.fuel.saturating_sub(work);if !self.recipient.fits(self.policy.grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native original lifetime recipient exceeds its immutable stock").with_retained_progress(spent))}Ok(())}
 fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}
 pub fn fuel_remaining(&self)->u64{self.fuel}
 pub fn retained_remaining(&self)->RetainedCloneGrant{self.remaining()}
 pub fn retained_spent(&self)->RetainedCloneProgress{self.recipient}
 pub fn deadline_us(&self)->u64{self.deadline}
 pub fn original_clock_binding(&self)->u64{self.clock_binding}
 pub fn observe_original_clock(&self,binding:u64)->Option<u64>{if self.closed||binding!=self.clock_binding{return None}(self.clock)().filter(|now|*now>=self.last_clock)}
 pub fn ready(&self)->bool{self.ready}
 /// 🪴️ Binding, cancellation, ledger and source births yield separately under actual issued root receipts.
 pub fn admit_step(&mut self)->Result<OriginalActorCall<OriginalActorAdmission,ValueError>,ValueError>{let(mut input,budget)=self.issue(false)?;input.grant=self.remaining();if !self.permits(budget)?{return Ok(OriginalActorCall{input,receipt_spent:Default::default(),work_units_spent:0,result:Ok(OriginalActorAdmission{ready:false,progress:Default::default()})})}let call=self.entry.admit_step(input,self.remaining_business(),self.clock);self.receive_raw(call.receipt_spent,call.work_units_spent)?;if let Ok(admission)=&call.result{self.ready=admission.ready}Ok(call)}
 /// ▶️ The body borrows the exact source, cumulative recipient under its immutable stock ceiling; the actual context exposes only remaining currencies.
 pub fn dispatch_step<T:Copy>(&mut self,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<OriginalActorCall<T,OriginalActorEntryFailure>,ValueError>{let(input,budget)=self.issue(false)?;if !self.permits(budget)?{return Ok(Self::denied(input))}let before=self.recipient;let fuel=self.fuel.saturating_sub(self.policy.retirement_reserve.fuel);let business=self.remaining_business();let mut call=self.entry.dispatch_value_step(input,budget,&mut self.recipient,self.clock,|cx|cx.with_original_work_loan(fuel,self.deadline,|cx|cx.with_retained_loan(business,produce)));call.receipt_spent=Self::delta(before,self.recipient);self.fuel=self.fuel.saturating_sub(call.work_units_spent);Ok(call)}
 /// ♻️ Original cleanup remains available after semantic failure and original cancellation.
 pub fn dispatch_cleanup_step<T>(&mut self,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<OriginalActorCall<T,OriginalActorEntryFailure>,ValueError>{let(input,budget)=self.issue(true)?;if !self.permits_retirement(budget){return Ok(Self::denied(input))}let before=self.recipient;let mut call=self.entry.dispatch_cleanup_step(input,budget,&mut self.recipient,self.clock,produce);call.receipt_spent=Self::delta(before,self.recipient);self.fuel=self.fuel.saturating_sub(call.work_units_spent);Ok(call)}
 fn denied<T>(input:RetainedTurnInput)->OriginalActorCall<T,OriginalActorEntryFailure>{OriginalActorCall{input,receipt_spent:Default::default(),work_units_spent:0,result:Err(OriginalActorEntryFailure::Authority(ValueError::literal(ValueRefusalKind::WorkLimit,"original native root callback has no supplied work or depth")))}}
 pub fn original_failure(&self)->Option<&OriginalTurnFailure>{self.entry.original_job_refusal()}
 pub fn original_cleanup_failure(&self)->Option<&OriginalTurnFailure>{self.entry.original_cleanup_refusal()}
 /// 🧾️ The same original entry closes each retained owner before terminal native root removal.
 pub fn close_step(&mut self)->Result<OriginalActorCall<RetainedCloneStep,OriginalActorContextError>,ValueError>{let(mut input,budget)=self.issue(true)?;input.grant=self.remaining();if !self.permits_retirement(budget)&&!self.entry.terminal_is_empty(){return Ok(OriginalActorCall{input,receipt_spent:Default::default(),work_units_spent:0,result:Ok(RetainedCloneStep::Progress(Default::default()))})}let call=self.entry.close_step(input,self.remaining());self.receive_raw(call.receipt_spent,call.work_units_spent)?;if matches!(&call.result,Ok(RetainedCloneStep::Complete(_)))&&self.entry.terminal_is_empty(){self.ready=false;self.closed=true}Ok(call)}
 pub fn terminal_is_empty(&self)->bool{self.entry.terminal_is_empty()}
 pub fn original_loans_are_returned(&self)->bool{self.entry.original_loans_are_returned()}
}

/// 🔔️ An actual initiating event creates a source only after the previous source physically closes.
pub struct NativeFrameEventIssuer{source:Option<NativeFrameCaller>,clock:fn()->Option<u64>,policy:NativeFrameRootPolicyV1,finishing:bool}
impl NativeFrameEventIssuer{
 /// 🌄️ The window retains only immutable policy before asynchronous boot produces its first event.
 pub fn new(clock:fn()->Option<u64>,policy:NativeFrameRootPolicyV1)->Result<Self,ValueError>{policy.validate()?;Ok(Self{source:None,clock,policy,finishing:false})}
 /// 📯️ Continuation notifications retain the current source, receipt, deadline, fuel and identity.
 pub fn observe_event(&mut self)->Result<bool,ValueError>{if self.source.is_some(){return Ok(false)}self.source=Some(NativeFrameCaller::new(semio_framework_trace::allocate_operation_id(),Generation(1),self.clock,self.policy)?);Ok(true)}
 fn source_mut(&mut self)->Result<&mut NativeFrameCaller,ValueError>{self.source.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native callback has no observed initiating event"))}
 pub fn ready(&self)->bool{self.source.as_ref().is_some_and(NativeFrameCaller::ready)}
 pub fn finishing(&self)->bool{self.finishing}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 pub fn deadline_us(&self)->Option<u64>{self.source.as_ref().map(NativeFrameCaller::deadline_us)}
 pub fn original_clock_binding(&self)->Option<u64>{self.source.as_ref().map(NativeFrameCaller::original_clock_binding)}
 pub fn fuel_remaining(&self)->u64{self.source.as_ref().map_or(0,NativeFrameCaller::fuel_remaining)}
 pub fn retained_spent(&self)->RetainedCloneProgress{self.source.as_ref().map_or(Default::default(),NativeFrameCaller::retained_spent)}
 pub fn admit_step(&mut self)->Result<OriginalActorCall<OriginalActorAdmission,ValueError>,ValueError>{self.source_mut()?.admit_step()}
 pub fn dispatch_step<T:Copy>(&mut self,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<OriginalActorCall<T,OriginalActorEntryFailure>,ValueError>{self.source_mut()?.dispatch_step(produce)}
 pub fn dispatch_cleanup_step<T>(&mut self,produce:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<OriginalActorCall<T,OriginalActorEntryFailure>,ValueError>{self.source_mut()?.dispatch_cleanup_step(produce)}
 /// 🏁️ Quiescent clients return all genuine source loans before the same caller pays physical close.
 pub fn finish_event_step(&mut self)->Result<bool,ValueError>{let Some(source)=self.source.as_mut()else{return Ok(true)};if !source.original_loans_are_returned(){return Ok(false)}self.finishing=true;let closed=source.close_step()?;if matches!(closed.result,Ok(RetainedCloneStep::Complete(_)))&&source.terminal_is_empty(){drop(self.source.take());self.finishing=false;return Ok(true)}Ok(false)}
}
