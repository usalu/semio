//! 🪪️ Identity-free per-operation caps authorize distinct authenticated requests, never renewal of an existing source.
#[path="🪙️guest/🦀️.rs"]
mod guest;
pub use guest::{GuestRelayGuestSubloan,GuestRelayGuestPurpose,GuestRelayGuestSettlement,GuestEscrowReconcile,GuestIssuedHistoryHome,GuestJournaledReceipt,FrozenCaptureProgress,SreValidationProgress,HeaderValidationProgress};
pub(super) use guest::GuestLedger;
use super::{GuestRelayDriverPolicy,GuestRelayWakeDriver,intersect};
use semio_framework_value::{RetainedCloneGrant,ValueError,ValueRefusalKind};
use semio_framework_value_derive::{FromValue,ToValue};

#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct GuestRelayOperationCaps {
 pub cleanup:GuestRelayOperationReserve,pub transit:GuestRelayOperationTransit,pub infrastructure:GuestRelayOperationInfrastructure,
 pub turn_fuel:u64,pub operation_fuel:u64,pub turn_execution_fuel:u64,pub operation_execution_fuel:u64,pub maximum_turn_microseconds:u64,pub maximum_lifetime_microseconds:u64,
 pub max_effects:u32,pub max_patch_bytes:u32,pub max_frames:u32,
 pub admission:RetainedCloneGrant,pub drive:RetainedCloneGrant,pub wake:RetainedCloneGrant,pub operation:RetainedCloneGrant,
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct GuestRelayOperationRequest {pub work_units:u64,pub maximum_elapsed_microseconds:u64,pub retained:RetainedCloneGrant}
impl GuestRelayDriverPolicy {
 /// 📤️ Projects explicit per-operation caps without treating a configured lifetime identity as request funding.
 pub fn operation_caps(self,cleanup:GuestRelayOperationReserve,transit:GuestRelayOperationTransit,infrastructure:GuestRelayOperationInfrastructure)->GuestRelayOperationCaps {
  let root=self.root;GuestRelayOperationCaps{cleanup,transit,infrastructure,turn_fuel:root.turn_fuel,operation_fuel:root.operation_fuel,turn_execution_fuel:root.turn_execution_fuel,operation_execution_fuel:root.operation_execution_fuel,maximum_turn_microseconds:root.maximum_turn_microseconds,maximum_lifetime_microseconds:root.maximum_lifetime_microseconds,max_effects:root.max_effects,max_patch_bytes:root.max_patch_bytes,max_frames:root.max_frames,admission:self.admission,drive:self.drive,wake:self.wake,operation:self.operation}
 }
}
impl GuestRelayOperationCaps {
 /// 📏️ The request bounds guest logical work and elapsed time; the host keeps its independently declared finite physical stock.
 pub fn bind_request(self,request:GuestRelayOperationRequest)->Result<Self,ValueError>{
  if request.work_units==0||request.maximum_elapsed_microseconds==0{return Err(invalid("positive requested operation work and elapsed duration required"))}
  self.cleanup.validate()?;self.transit.validate()?;self.infrastructure.validate()?;if self.infrastructure.maximum_cleanup_microseconds>=self.cleanup.maximum_microseconds{return Err(invalid("guest infrastructure cleanup must leave host cleanup time"))}
  consumable_remaining(consumable_remaining(consumable_remaining(self.operation,self.cleanup.retained)?,self.transit.retained)?,self.infrastructure.retained)?;let elapsed=self.maximum_lifetime_microseconds.min(request.maximum_elapsed_microseconds);
  if elapsed<=self.cleanup.maximum_microseconds||self.cleanup.work_fuel.checked_add(self.transit.work_fuel).and_then(|fuel|fuel.checked_add(self.infrastructure.work_fuel)).is_none_or(|minimum|minimum>=self.operation_fuel)||self.cleanup.execution_fuel>=self.operation_execution_fuel{return Err(invalid("cleanup reserve leaves no requested semantic authority"))}
  Ok(Self{maximum_turn_microseconds:self.maximum_turn_microseconds.min(elapsed),maximum_lifetime_microseconds:elapsed,..self})
 }
 /// 🧮️ Publishes the effective semantic ceilings without exposing or spending cleanup currency.
 pub fn admitted_request(self,request:GuestRelayOperationRequest)->Result<GuestRelayOperationRequest,ValueError>{
  let bounded=self.bind_request(request)?;Ok(GuestRelayOperationRequest{work_units:request.work_units.min(bounded.operation_fuel-self.cleanup.work_fuel-self.transit.work_fuel-self.infrastructure.work_fuel),maximum_elapsed_microseconds:bounded.maximum_lifetime_microseconds,retained:intersect(consumable_remaining(consumable_remaining(consumable_remaining(bounded.operation,self.cleanup.retained)?,self.transit.retained)?,self.infrastructure.retained)?,request.retained)})
 }
 /// 🌱️ Births one finite source at authenticated request admission using the real native clock and cancellation.
 pub fn issue(self,request:GuestRelayOperationRequest,clock:fn()->Option<u64>,cancelled:Box<dyn Fn()->bool+Send+Sync>)->Result<GuestRelayWakeDriver,ValueError>{
  let caps=self.bind_request(request)?;let effective=self.admitted_request(request)?;
  let root=super::super::GuestRelayRootPolicy{operation:semio_framework_trace::allocate_operation_id().0,generation:semio_framework_trace::allocate_operation_id().0,turn_fuel:caps.turn_fuel,operation_fuel:caps.operation_fuel,turn_execution_fuel:caps.turn_execution_fuel,operation_execution_fuel:caps.operation_execution_fuel,maximum_turn_microseconds:caps.maximum_turn_microseconds,maximum_lifetime_microseconds:caps.maximum_lifetime_microseconds,max_effects:caps.max_effects,max_patch_bytes:caps.max_patch_bytes,max_frames:caps.max_frames};
  GuestRelayWakeDriver::new(GuestRelayDriverPolicy{root,admission:caps.admission,drive:caps.drive,wake:caps.wake,operation:caps.operation},clock,cancelled).map(|driver|{driver.original.try_lock().expect("new operation is unique").operation=Some(OperationBudget{request:effective,reserve:caps.cleanup,transit:caps.transit,infrastructure:caps.infrastructure,guest:guest::GuestLedger::default(),cleanup:false,closed:false});driver})
 }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct GuestRelayOperationReserve {pub retained:RetainedCloneGrant,pub work_fuel:u64,pub execution_fuel:u64,pub maximum_microseconds:u64}
impl GuestRelayOperationReserve {
 /// 🧾️ Cleanup is independently declared before any semantic work receives authority.
 pub fn validate(self)->Result<(),ValueError>{if self.work_fuel==0||self.execution_fuel==0||self.maximum_microseconds==0||self.retained.maximum_items==0||self.retained.maximum_depth==0{return Err(invalid("explicit positive operation cleanup reserve required"))}Ok(())}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct GuestRelayOperationTransit{pub retained:RetainedCloneGrant,pub work_fuel:u64}
impl GuestRelayOperationTransit{
 /// 🛣️ Keeps a separately declared host transport minimum beside guest and cleanup authority.
 pub fn validate(self)->Result<(),ValueError>{if self.work_fuel==0||self.retained.maximum_items==0||self.retained.maximum_depth==0{return Err(invalid("explicit positive host transport minimum required"))}Ok(())}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct GuestRelayOperationInfrastructure{pub retained:RetainedCloneGrant,pub work_fuel:u64,pub maximum_cleanup_microseconds:u64}
impl GuestRelayOperationInfrastructure{
 /// 🏗️ Guest bootstrap and close receive explicit authority distinct from transport and client work.
 pub fn validate(self)->Result<(),ValueError>{if self.work_fuel==0||self.maximum_cleanup_microseconds==0||self.retained.maximum_items==0||self.retained.maximum_depth==0{return Err(invalid("explicit positive guest infrastructure scope required"))}Ok(())}
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn consumable_remaining(total:RetainedCloneGrant,reserve:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{
 if reserve.maximum_depth>total.maximum_depth{return Err(invalid("cleanup depth exceeds the operation ceiling"))}
 Ok(RetainedCloneGrant{maximum_items:total.maximum_items.checked_sub(reserve.maximum_items).ok_or_else(||invalid("cleanup items exceed operation currency"))?,maximum_copy_bytes:total.maximum_copy_bytes.checked_sub(reserve.maximum_copy_bytes).ok_or_else(||invalid("cleanup copy exceeds operation currency"))?,maximum_capacity_bytes:total.maximum_capacity_bytes.checked_sub(reserve.maximum_capacity_bytes).ok_or_else(||invalid("cleanup capacity exceeds operation currency"))?,maximum_release_bytes:total.maximum_release_bytes.checked_sub(reserve.maximum_release_bytes).ok_or_else(||invalid("cleanup release exceeds operation currency"))?,maximum_depth:total.maximum_depth})
}

#[cfg(test)]
include!("🧪️tests/🦀️.rs");

pub(super) struct OperationBudget {pub request:GuestRelayOperationRequest,pub reserve:GuestRelayOperationReserve,pub transit:GuestRelayOperationTransit,pub infrastructure:GuestRelayOperationInfrastructure,pub guest:guest::GuestLedger,pub cleanup:bool,pub closed:bool}
pub struct GuestRelayAdmissionCall {pub result:Result<(),semio_framework_job::OriginalTurnFailure>,pub execution:Option<semio_framework_actor::OriginalGuestExecutionReceipt>}
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value",send_only)]
pub struct GuestRelayAdmissionFailure {pub closure:Option<semio_framework_actor::retained_context::OriginalActorContextError>,pub primary:Option<semio_framework_job::OriginalTurnFailure>,pub settlement:Option<semio_framework_job::OriginalTurnFailure>}
impl GuestRelayAdmissionFailure{pub fn retained_progress(&self)->RetainedCloneProgress{self.primary.as_ref().map(semio_framework_job::OriginalTurnFailure::retained_progress).or_else(||self.closure.as_ref().map(semio_framework_actor::retained_context::OriginalActorContextError::retained_progress)).or_else(||self.settlement.as_ref().map(semio_framework_job::OriginalTurnFailure::retained_progress)).unwrap_or_default()}}
impl From<ValueError> for GuestRelayAdmissionFailure{fn from(error:ValueError)->Self{Self::from(semio_framework_job::OriginalTurnFailure::from(error))}}
impl From<semio_framework_job::OriginalTurnFailure> for GuestRelayAdmissionFailure {fn from(primary:semio_framework_job::OriginalTurnFailure)->Self{Self{closure:None,primary:Some(primary),settlement:None}}}
fn spent(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,message)}
pub(super) fn physical_policy(original:&super::WakeDriverState,policy:RetainedCloneGrant,retirement:bool)->Result<RetainedCloneGrant,ValueError>{
 let Some(operation)=&original.operation else{return Ok(policy)};
 if operation.closed{return Err(invalid("operation source is already closed"))};
 let available=operation.guest.physical_remaining(original.treasury.original_remaining())?;if retirement||operation.cleanup{return Ok(intersect(policy,available))}
 let deadline=original.root.remaining_work().absolute_deadline_us.saturating_sub(operation.reserve.maximum_microseconds);
 if original.root.clock_source()().is_none_or(|now|now>=deadline){return Err(spent("semantic deadline exhausted with original cleanup window retained"))}
 let available=consumable_remaining(available,operation.reserve.retained)?;
 let policy=intersect(policy,available);
 if policy.maximum_items==0{return Err(spent("semantic operation currency exhausted with cleanup reserve retained"))}
 Ok(policy)
}
pub(super) fn scope_budget(original:&super::WakeDriverState,mut budget:semio_framework_job::StepBudget,retirement:bool)->Result<semio_framework_job::StepBudget,ValueError>{
 let Some(operation)=&original.operation else{return Ok(budget)};
 let available=operation.guest.physical_fuel(original.root.fuel_remaining())?;budget.fuel=budget.fuel.min(available);if retirement||operation.cleanup{return Ok(budget)}
 let fuel=available.checked_sub(operation.reserve.work_fuel).ok_or_else(||spent("semantic work exhausted with cleanup fuel retained"))?;
 if fuel==0{return Err(spent("semantic work exhausted with cleanup fuel retained"))}
 budget.fuel=budget.fuel.min(fuel);budget.deadline_us=budget.deadline_us.checked_sub(operation.reserve.maximum_microseconds).ok_or_else(||invalid("cleanup duration exceeds the original deadline"))?;Ok(budget)
}
impl GuestRelayWakeDriver {
 /// 🎚️ Request ceilings and the actual semantic deadline are DATA, never a minted Source callback.
 pub fn operation_request_limits(&self)->Result<(GuestRelayOperationRequest,u64),ValueError>{let original=self.original.try_lock().map_err(|_|invalid("request limits Source is borrowed"))?;let operation=original.operation.as_ref().ok_or_else(||invalid("request limits require their actual operation"))?;Ok((operation.request,original.root.remaining_work().absolute_deadline_us.saturating_sub(operation.reserve.maximum_microseconds)))}
 /// 🌱️ Runs one synchronous original admission slice and returns every actual receipt before yielding.
 pub fn dispatch_admission_step(&self,requires_execution:bool,produce:impl FnOnce(Option<semio_framework_actor::OriginalGuestExecutionBudget>,&mut semio_framework_job::StepContext<'_>)->GuestRelayAdmissionCall)->Result<bool,GuestRelayAdmissionFailure>{self.dispatch_original_admission_step(requires_execution,false,produce)}
 /// 🧹️ Uses the same Source cleanup reserve and retirement context without admitting another source.
 pub fn dispatch_cleanup_admission_step(&self,requires_execution:bool,produce:impl FnOnce(Option<semio_framework_actor::OriginalGuestExecutionBudget>,&mut semio_framework_job::StepContext<'_>)->GuestRelayAdmissionCall)->Result<bool,GuestRelayAdmissionFailure>{self.dispatch_original_admission_step(requires_execution,true,produce)}
 fn dispatch_original_admission_step(&self,requires_execution:bool,cleanup:bool,produce:impl FnOnce(Option<semio_framework_actor::OriginalGuestExecutionBudget>,&mut semio_framework_job::StepContext<'_>)->GuestRelayAdmissionCall)->Result<bool,GuestRelayAdmissionFailure>{
  if cleanup{let original=self.original.try_lock().map_err(|_|GuestRelayAdmissionFailure::from(invalid("cleanup original source is borrowed")))?;if !original.root.ready(){return Err(GuestRelayAdmissionFailure::from(invalid("cleanup requires an already admitted original source")))}}else if !self.admit_root_step().map_err(GuestRelayAdmissionFailure::from)?{return Ok(false)}
  let Some(input)=(if cleanup{self.begin_actor_cleanup_turn()}else{self.begin_actor_turn(None)}).map_err(|error|GuestRelayAdmissionFailure::from(semio_framework_job::OriginalTurnFailure::from(error)))?else{return Ok(false)};
  let execution=if requires_execution{match if cleanup{self.checkout_actor_cleanup_execution_budget(input)}else{self.checkout_actor_execution_budget(input)}{Ok(execution)=>Some(execution),Err(error)=>return Err(GuestRelayAdmissionFailure{closure:None,primary:Some(error.into()),settlement:self.finish_actor_step(input).err()})}}else{None};
  let callback=|cx:&mut semio_framework_job::StepContext<'_>|Ok(produce(execution,cx));let call=match if cleanup{self.dispatch_actor_cleanup_step(input,callback)}else{self.dispatch_actor_step(input,callback)}{Ok(Some(call))=>call,Ok(None)=>return Err(GuestRelayAdmissionFailure{closure:None,primary:Some(invalid("issued admission context became unavailable").into()),settlement:self.finish_actor_step(input).err()}),Err(error)=>return Err(GuestRelayAdmissionFailure{closure:None,primary:Some(error),settlement:self.finish_actor_step(input).err()})};
  if requires_execution&&call.execution.is_none(){return Err(GuestRelayAdmissionFailure{closure:None,primary:call.result.err(),settlement:Some(invalid("admission retains its missing original execution receipt").into())})}
  if let Some(receipt)=call.execution{if let Err(error)=self.receive_actor_execution_receipt(input,receipt){return Err(GuestRelayAdmissionFailure{closure:None,primary:call.result.err(),settlement:Some(error.into())})}}
  let settled=self.finish_actor_step(input);
  match(call.result,settled){(Ok(()),Ok(_))=>Ok(true),(result,settlement)=>Err(GuestRelayAdmissionFailure{closure:None,primary:result.err(),settlement:settlement.err()})}
 }
 /// 🔁️ Settles one unit-return slice of the actual operation before control can yield or lose a produced owner.
 pub fn dispatch_operation_step(&self,execution:bool,cleanup:bool,produce:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>)->Result<bool,GuestRelayAdmissionFailure>{
  if !cleanup&&!self.admit_root_step()?{return Ok(false)}
  let generation={let original=self.original.try_lock().map_err(|_|invalid("operation dispatch source is borrowed"))?;let operation=original.operation.as_ref().ok_or_else(||invalid("actual per-operation Source required"))?;if operation.closed||operation.cleanup&&!cleanup||cleanup&&!original.root.ready(){return Err(invalid("operation dispatch cannot restart semantics or birth a cleanup Source").into())}original.root.policy().generation};
  let result=match(execution,cleanup){(true,true)=>self.dispatch_execution_cleanup_step(0,generation,produce),(true,false)=>self.dispatch_execution_step(0,generation,produce),(false,true)=>self.dispatch_cleanup_step(0,generation,produce),(false,false)=>self.dispatch_step(0,generation,produce)};
  let held=match self.original.try_lock(){Ok(original)=>matches!(original.checkout.as_ref(),Some(super::DriverCheckout::Context{slot:0,generation:actual,..})if*actual==generation),Err(_)=>return Err(GuestRelayAdmissionFailure{closure:None,primary:result.err(),settlement:Some(invalid("operation handback source is borrowed").into())})};
  let settlement=if held{self.finish_turn(0,generation).map(|_|()).map_err(Into::into)}else{Ok(())};match(result,settlement){(Ok(ready),Ok(()))=>Ok(ready.is_some()),(result,settlement)=>Err(GuestRelayAdmissionFailure{closure:None,primary:result.err(),settlement:settlement.err()})}
 }
 fn dispatch_guest_reconcile_step(&self,cleanup:bool,produce:impl FnOnce(&mut semio_framework_job::StepContext<'_>,&mut GuestLedger)->Result<(),semio_framework_job::OriginalTurnFailure>)->Result<bool,GuestRelayAdmissionFailure>{
  let generation={let original=self.original.try_lock().map_err(|_|invalid("guest reconciliation source is borrowed"))?;let operation=original.operation.as_ref().ok_or_else(||invalid("guest reconciliation requires its operation"))?;if operation.closed||operation.cleanup&&!cleanup||cleanup&&!original.root.ready(){return Err(invalid("guest reconciliation cannot restart its Source").into())}let(items,bytes,work)=GuestIssuedHistoryHome::ledger_commit_minimum();let(close_items,close_bytes,_)=GuestIssuedHistoryHome::closed_retirement_minimum();let items=items.max(close_items);let drive=original.treasury.policy().drive;let copy=bytes.max(GuestIssuedHistoryHome::journal_claim_minimum_bytes()).max(GuestIssuedHistoryHome::journal_snapshot_minimum_bytes()).max(GuestIssuedHistoryHome::frozen_identity_minimum_bytes()).max(GuestIssuedHistoryHome::header_handback_transfer_minimum_bytes()).max(GuestIssuedHistoryHome::issued_loan_minimum_bytes()).max(GuestIssuedHistoryHome::frozen_retirement_minimum_bytes()).max(close_bytes);if drive.maximum_items<items||drive.maximum_copy_bytes<copy||drive.maximum_depth==0||original.root.policy().turn_fuel<work{return Err(invalid("operation policy cannot fund escrow reconciliation minimum").into())}original.root.policy().generation};
  let result=self.dispatch_original_with_guest_step(0,generation,cleanup,false,|cx,ledger|produce(cx,ledger.ok_or_else(||invalid("guest reconciliation lost its actual ledger"))?));
  let held=match self.original.try_lock(){Ok(original)=>matches!(original.checkout.as_ref(),Some(super::DriverCheckout::Context{slot:0,generation:actual,..})if*actual==generation),Err(_)=>return Err(GuestRelayAdmissionFailure{closure:None,primary:result.err(),settlement:Some(invalid("guest reconciliation handback source is borrowed").into())})};
  let settlement=if held{self.finish_turn(0,generation).map(|_|()).map_err(Into::into)}else{Ok(())};match(result,settlement){(Ok(ready),Ok(()))=>Ok(ready.is_some()),(result,settlement)=>Err(GuestRelayAdmissionFailure{closure:None,primary:result.err(),settlement:settlement.err()})}
 }
 /// ♻️ Closes the same finite operation source; lack of funded progress remains an owned refusal.
 pub async fn close_operation(&self)->Result<(),GuestRelayAdmissionFailure>{
  {let mut original=self.original.try_lock().map_err(|_|invalid("operation close source is borrowed"))?;let Some(operation)=original.operation.as_mut()else{return Err(invalid("per-operation source required for operation close").into())};operation.cleanup=true;}
  loop{
   let before={let mut original=self.original.try_lock().map_err(|_|invalid("operation close source is borrowed"))?;if original.root.terminal_is_empty(){if original.operation.as_ref().is_some_and(|operation|operation.guest.escrow_pending()){return Err(invalid("operation Source cannot close while guest escrow remains").into())}if let Some(operation)=original.operation.as_mut(){operation.closed=true;}return Ok(())}(original.treasury.original_remaining(),original.root.fuel_remaining(),original.checkout.is_some(),original.root.remaining_work().absolute_deadline_us,original.root.clock_source())};
   if self.close_root_step()?{let mut original=self.original.try_lock().map_err(|_|invalid("operation close source is borrowed"))?;if let Some(operation)=original.operation.as_mut(){operation.closed=true;}return Ok(())}
   let after={let original=self.original.try_lock().map_err(|_|invalid("operation close source is borrowed"))?;(original.treasury.original_remaining(),original.root.fuel_remaining(),original.checkout.is_some())};
   if !before.2&&!after.2&&before.0==after.0&&before.1==after.1{return Err(spent("operation cleanup made no funded progress; source remains retained").into())}
   if after.2&&(before.4)().is_none_or(|now|now>=before.3){return Err(spent("operation cleanup still owns an outstanding receipt at its original deadline").into())}
   semio_framework_async::yield_once().await;
  }
 }
}

pub(super) struct ExecutionCredit {pub budget:semio_framework_actor::OriginalGuestExecutionBudget,pub claimed:bool,pub receipt:Option<semio_framework_actor::OriginalGuestExecutionReceipt>}
pub(super) fn execution_fuel(original:&super::WakeDriverState,retirement:bool)->Result<u64,ValueError>{
 Ok(match &original.operation{Some(operation)if!retirement&&!operation.cleanup=>original.root.execution_fuel_remaining().saturating_sub(operation.reserve.execution_fuel),_=>u64::MAX})
}
impl GuestRelayWakeDriver {
 /// 🎟️ Lends one sealed VM authorization of this exact live callback and retains its actual receipt.
 fn execute_original(&self,cx:&mut semio_framework_job::StepContext<'_>,produce:impl FnOnce(semio_framework_actor::OriginalGuestExecutionBudget,&mut semio_framework_job::StepContext<'_>)->Option<semio_framework_actor::OriginalGuestExecutionReceipt>)->Result<(),semio_framework_job::OriginalTurnFailure>{
  let input=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;
  let mut state=self.execution.try_lock().map_err(|_|invalid("original VM crossing is exclusively borrowed"))?;
  let credit=state.as_mut().ok_or_else(||invalid("original callback has no issued VM credit"))?;
  if credit.claimed{return Err(invalid("original VM credit was already claimed by its callback").into())}
  let budget=semio_framework_actor::OriginalGuestExecutionBudget{input,fuel:credit.budget.fuel,deadline_us:cx.deadline_us()};budget.validate_narrowing(credit.budget)?;credit.claimed=true;let receipt=produce(budget,cx);credit.receipt=receipt;let receipt=receipt.ok_or_else(||invalid("submitted VM crossing retains its missing actual receipt"))?;receipt.validate_for(budget)?;Ok(())
 }
 /// 🔎️ The request owner removes refusal custody only after the actual source is empty.
 pub fn operation_terminal_is_empty(&self)->Result<bool,ValueError>{let original=self.original.try_lock().map_err(|_|invalid("operation source is borrowed"))?;Ok(original.root.terminal_is_empty()&&original.checkout.is_none()&&!original.operation.as_ref().is_some_and(|operation|operation.guest.escrow_pending())&&self.execution.try_lock().map_err(|_|invalid("original VM credit is borrowed"))?.is_none())}
}

thread_local!{static CURRENT_EXECUTION_DRIVER:std::cell::Cell<(*const GuestRelayWakeDriver,usize)>=const{std::cell::Cell::new((std::ptr::null(),0))};}
struct ExecutionScope{previous:(*const GuestRelayWakeDriver,usize)}
impl Drop for ExecutionScope{fn drop(&mut self){CURRENT_EXECUTION_DRIVER.with(|current|current.set(self.previous));}}
fn current_execution_driver()->*const GuestRelayWakeDriver{CURRENT_EXECUTION_DRIVER.with(|current|current.get().0)}
pub(super) fn with_execution_scope<R>(driver:&GuestRelayWakeDriver,context:usize,produce:impl FnOnce()->R)->R{
 let scope=ExecutionScope{previous:CURRENT_EXECUTION_DRIVER.with(|current|current.replace((driver as*const _,context)))};
 let result=produce();drop(scope);result
}
impl GuestRelayWakeDriver{
 /// 🔌️ Borrows the VM port lent only to this synchronous original callback on this thread.
 pub fn execute_current_original(cx:&mut semio_framework_job::StepContext<'_>,produce:impl FnOnce(semio_framework_actor::OriginalGuestExecutionBudget,&mut semio_framework_job::StepContext<'_>)->Option<semio_framework_actor::OriginalGuestExecutionReceipt>)->Result<(),semio_framework_job::OriginalTurnFailure>{
  let driver=current_execution_driver();
  if driver.is_null()||CURRENT_EXECUTION_DRIVER.with(|current|current.get().1)!=cx as*mut _ as usize{return Err(invalid("original callback has no scoped VM execution port").into())}
  unsafe{(&*driver).execute_original(cx,produce)}
 }
}
