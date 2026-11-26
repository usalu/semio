//! 🔌️ A selected guest handle retains only its actual Source-native leases.
use super::*;
use crate::actor_failure::{ActorFailureHome,ActorFailureCapsule};
use crate::{FrozenReceiptFrontier,GuestFrozenHistoryCapture};
use crate::original_unit_flow::ColdHeader;
use semio_framework_job::{OriginalNativeLease,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_plugin_host_wake::{GuestRelayAdmissionFailure,GuestRelayGuestPurpose,GuestRelayGuestSubloan};
use std::mem::size_of;
#[path="🪙️escrow.rs"]
mod escrow;
use escrow::ColdEscrow;
#[derive(Default)]
struct Snapshot{encoding:bool,page:bool,done:bool,failed:bool,fault_route:bool,failure:bool,fault_transferred:bool,execution:bool,purpose:Option<GuestRelayGuestPurpose>,wire_header:Option<usize>,frozen:Option<(FrozenReceiptFrontier,GuestRelayGuestSubloan)>,header:Option<(ColdHeader,bool)>,return_ready:bool,return_required:bool,draining_receipt:bool}
impl Snapshot{
 fn read(owner:&GuestRelayColdController,runtime:&GuestRuntimes,instance:&GuestInstance)->Result<Self,OriginalTurnFailure>{
  Ok(Self{encoding:owner.needs_encoding(),page:owner.needs_page(),done:owner.done(),failed:owner.failed(),
   fault_route:owner.failure_route(),failure:owner.has_failure(),fault_transferred:owner.failure_transferred(),
   execution:owner.requires_execution(runtime,instance)?,purpose:owner.needs_loan(),wire_header:owner.required_wire_header(),
   frozen:owner.frozen().zip(owner.loan()),header:owner.header().map(|header|(header,owner.scalar_receipt())),
   return_ready:owner.return_ready(),return_required:owner.return_required(),draining_receipt:owner.draining_receipt()})
 }
}
/// 🧬️ The parent retains this handle by value before any inference future can suspend.
pub struct PluginInstanceHandle{pub actor:RuntimeActorId,origin:GuestRelayNativeOrigin,home:Option<OriginalNativeLease<GuestRelayNativeHome>>,output:Option<OriginalNativeLease<GuestRelayOutputHome>>,fault:Option<OriginalNativeLease<GuestRelayFaultHome>>,guest_fault:Option<OriginalNativeLease<GuestRelayOutputHome>>,actor_failure:Option<OriginalNativeLease<ActorFailureHome>>,escrow:ColdEscrow}
impl PluginInstanceHandle{
 pub(super) fn from_admitted(actor:RuntimeActorId,origin:GuestRelayNativeOrigin,home:OriginalNativeLease<GuestRelayNativeHome>,output:OriginalNativeLease<GuestRelayOutputHome>,fault:OriginalNativeLease<GuestRelayFaultHome>,guest_fault:OriginalNativeLease<GuestRelayOutputHome>,actor_failure:OriginalNativeLease<ActorFailureHome>)->Self{let escrow=ColdEscrow::new(home.recipient_identity());Self{actor,origin,home:Some(home),output:Some(output),fault:Some(fault),guest_fault:Some(guest_fault),actor_failure:Some(actor_failure),escrow}}
 /// 💡️ A unit-return callback retains all produced owners in this handle or the caller's result slot.
 pub async fn infer<S:semio_framework_pack_json::JsonWriteSource>(&mut self,input:&S,driver:&GuestRelayWakeDriver,out:&mut Option<GuestRelayResult>)->Result<(),GuestRelayAdmissionFailure>{self.run("semio.infer",input,driver,out).await}
 /// ▶️ Each declared cold job uses the same finite Source and typed output custody.
 pub async fn run<S:semio_framework_pack_json::JsonWriteSource>(&mut self,kind:&'static str,input:&S,driver:&GuestRelayWakeDriver,out:&mut Option<GuestRelayResult>)->Result<(),GuestRelayAdmissionFailure>{let mut source=GuestRelayBorrowedJson(input);self.run_from(kind,&mut source,driver,out).await}
 /// 📚️ A Source provider lends each input view only within its paid receiving callback.
 pub async fn run_from(&mut self,kind:&'static str,input:&mut dyn GuestRelayJsonSource,driver:&GuestRelayWakeDriver,out:&mut Option<GuestRelayResult>)->Result<(),GuestRelayAdmissionFailure>{self.run_provider(kind,JobInput::Json(input),driver,out).await}
 /// 📄️ A binary provider lends its original bytes in each paid encoding slice.
 pub async fn run_bytes_from(&mut self,kind:&'static str,input:&mut dyn GuestRelayByteSource,driver:&GuestRelayWakeDriver,out:&mut Option<GuestRelayResult>)->Result<(),GuestRelayAdmissionFailure>{self.run_provider(kind,JobInput::Bytes(input),driver,out).await}
 async fn run_provider(&mut self,kind:&'static str,mut input:JobInput<'_>,driver:&GuestRelayWakeDriver,out:&mut Option<GuestRelayResult>)->Result<(),GuestRelayAdmissionFailure>{
  if out.is_some(){return Err(refused("inference result slot is already occupied").into())}if driver.operation_abandoned(){return Err(refused("abandoned operation cannot resume borrowed input").into())}let mut guard=AbandonOnDrop{driver,armed:true};
  let tracing=std::env::var_os("SEMIO_DEBUG_COLD_ACTION").is_some_and(|flag|flag=="1");
  let mut last_action=None;
  let mut occurrences=0u64;
  loop{
   let mut snapshot=None;
   let mut action=None;
   let mut epoch=None;
   driver.dispatch_operation_step(false,false,|cx|{with_controller(&mut self.home,cx,|runtime,instance,state,cx|{
    if state.controller.is_none(){if !pay(cx,size_of::<GuestRelayColdController>(),false)?{return Ok(())}state.controller=Some(GuestRelayColdController::new(1,kind)?)}
    let owner=state.controller.as_ref().unwrap();if owner.kind()!=kind{return Err(refused("cold job kind changed after admission").into())}if tracing{action=Some(owner.action());epoch=owner.loan().map(|loan|loan.budget().input.epoch);}snapshot=Some(Snapshot::read(owner,runtime,instance)?);Ok(())
   })?;Ok(())})?;
   let Some(state)=snapshot else{semio_framework_async::yield_once().await;continue};
   if let Some(action)=action{
    if last_action!=Some(action){last_action=Some(action);occurrences=0;}
    occurrences=occurrences.saturating_add(1);
    if occurrences.is_power_of_two(){eprintln!("[DEBUG] cold job={kind} action={} iteration={occurrences} epoch={epoch:?} loan={} execution={} frozen={} header={} returning={} draining={}",action.label(),state.purpose.is_some(),state.execution,state.frozen.is_some(),state.header.is_some(),state.return_ready,state.draining_receipt);}
   }
   if self.advance_escrow(driver,&state,false)?{semio_framework_async::yield_once().await;continue}
   if state.fault_route{let result=self.drive_actor_failure(driver,out).await;if result.is_ok(){guard.armed=false}return result}
   if state.fault_transferred{
    driver.dispatch_operation_step(false,true,|cx|{if let Some(fault)=GuestRelayFault::take_captured_step(&mut self.fault,cx)?{*out=Some(GuestRelayResult::Failure(fault))}Ok(())})?;if out.is_some(){guard.armed=false;return Ok(())}
   }else if state.failure{
    let fault=&mut self.fault;driver.dispatch_operation_step(false,true,|cx|{with_controller_pair(&mut self.home,fault,cx,|_,_,state,fault,cx|{let controller=state.controller.as_mut().ok_or_else(||refused("fault lost its native controller"))?;controller.take_failure_step(cx,|failure,cx|{fault.capture_turn_failure_step(failure,cx)?;Ok(())})?;Ok(())})?;Ok(())})?;
   }else if state.done{
    let target=if state.failed{&mut self.guest_fault}else{&mut self.output};driver.dispatch_operation_step(false,false,|cx|{if state.failed{if let Some(fault)=GuestRelayFault::take_guest_failure_step(target,cx)?{*out=Some(GuestRelayResult::Failure(fault))}}else if let Some(output)=GuestRelayOutput::take_sealed_step(target,cx)?{*out=Some(GuestRelayResult::Success(output))}Ok(())})?;if out.is_some(){guard.armed=false;return Ok(())}
   }else if let Some(purpose)=state.purpose{
    let header=state.wire_header.ok_or_else(||refused("fixed crossing has no wire header"))?;
    self.begin_loan(driver,purpose,header,false)?;
   }else if state.encoding{
    driver.dispatch_operation_step(false,false,|cx|{let home=self.home.as_mut().ok_or_else(||refused("encoding lost its native lease"))?;match &mut input{JobInput::Json(source)=>{source.with_home_json(home,cx,&mut |source,home,cx|{home.with_native(cx,|_,_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("encoding lost its native controller"))?.encode_step(source,cx)?;Ok(())})?;Ok(())})?;},JobInput::Bytes(source)=>{source.with_home_bytes(home,cx,&mut |source,home,cx|{home.with_native(cx,|_,_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("encoding lost its native controller"))?.encode_bytes_step(source,cx)?;Ok(())})?;Ok(())})?;}}Ok(())})?;
   }else if state.page{
    driver.dispatch_operation_step(false,false,|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("page lost its native controller"))?.begin_page_step(cx)?;Ok(())})?;Ok(())})?;
   }else{
    let target=if state.failed{&mut self.guest_fault}else{&mut self.output};let requires_execution=state.execution;
    driver.dispatch_operation_step(requires_execution,false,|cx|{with_controller_pair(&mut self.home,target,cx,|runtime,instance,state,output,cx|{
     let controller=state.controller.as_mut().ok_or_else(||refused("execution lost its native controller"))?;
     if requires_execution{if !controller.prepare_execution_step(cx)?{return Ok(())}let mut result=None;GuestRelayWakeDriver::execute_current_original(cx,|execution,cx|{result=Some(controller.advance_step(runtime,instance,output,Some(execution),cx));controller.execution_receipt()})?;result.ok_or_else(||refused("scoped execution did not run"))??;}else{controller.advance_step(runtime,instance,output,None,cx)?}Ok(())
    })?;Ok(())})?;
   }
   semio_framework_async::yield_once().await;
  }
 }


 /// 🧯️ Continues only a verified ActorFailure protocol on its reserved Infrastructure source.
 pub(super) async fn drive_actor_failure(&mut self,driver:&GuestRelayWakeDriver,out:&mut Option<GuestRelayResult>)->Result<(),GuestRelayAdmissionFailure>{
  if out.is_some(){return Err(refused("actor failure receiver is occupied").into())}
  loop{
   let mut snapshot=None;driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller(&mut self.home,cx,|runtime,instance,state,cx|{if !pay(cx,size_of::<Snapshot>(),true)?{return Ok(())}let owner=state.controller.as_ref().ok_or_else(||refused("actor failure lost its controller"))?;snapshot=Some(Snapshot::read(owner,runtime,instance)?);Ok(())})?;Ok(())}))?;
   let Some(state)=snapshot else{semio_framework_async::yield_once().await;continue};if self.advance_escrow(driver,&state,true)?{semio_framework_async::yield_once().await;continue}if !state.fault_route{return Err(refused("cleanup cannot publish ordinary job output").into())}
   if state.fault_transferred{
    driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{if let Some(fault)=GuestRelayFault::take_captured_step(&mut self.fault,cx)?{*out=Some(GuestRelayResult::Failure(fault))}Ok(())}))?;if out.is_some(){return Ok(())}
   }else if state.failure{
    let fault=&mut self.fault;driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller_pair(&mut self.home,fault,cx,|_,_,state,fault,cx|{let controller=state.controller.as_mut().ok_or_else(||refused("actor failure host fault lost its controller"))?;controller.take_failure_step(cx,|failure,cx|{fault.capture_turn_failure_step(failure,cx)?;Ok(())})?;Ok(())})?;Ok(())}))?;
   }else if state.done{
    let recipient=self.actor_failure.as_ref().ok_or_else(||refused("actor failure recipient is absent"))?.recipient_identity();driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{let mut permit=None;with_controller(&mut self.home,cx,|_,_,state,cx|{permit=state.controller.as_ref().ok_or_else(||refused("actor failure lost its controller"))?.failure_permit_step(recipient,cx)?;Ok(())})?;if let Some(permit)=permit{if let Some(fault)=ActorFailureCapsule::take_step(&mut self.actor_failure,&permit,cx)?{*out=Some(GuestRelayResult::Failure(GuestRelayFault::ActorFailure(fault)))}}Ok(())}))?;if out.is_some(){return Ok(())}
   }else if let Some(purpose)=state.purpose{
    if purpose!=GuestRelayGuestPurpose::Infrastructure{return Err(refused("actor failure cannot consume semantic stock").into())}
    let header=state.wire_header.ok_or_else(||refused("cleanup crossing has no wire header"))?;
    self.begin_loan(driver,purpose,header,true)?;
   }else if state.page{
    driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("actor failure page lost its controller"))?.begin_page_step(cx)?;Ok(())})?;Ok(())}))?;
   }else if state.encoding{return Err(refused("cleanup cannot encode a new input").into())}
   else{
    let recipient=self.actor_failure.as_ref().ok_or_else(||refused("actor failure recipient is absent"))?.recipient_identity();let requires_execution=state.execution;
    driver.dispatch_operation_step(requires_execution,true,|cx|cx.with_original_retirement(|cx|{with_controller_pair(&mut self.home,&mut self.actor_failure,cx,|runtime,instance,state,output,cx|{let controller=state.controller.as_mut().ok_or_else(||refused("actor failure execution lost its controller"))?;if requires_execution{if !controller.prepare_execution_step(cx)?{return Ok(())}let mut result=None;GuestRelayWakeDriver::execute_current_original(cx,|execution,cx|{result=Some(controller.advance_failure_step(runtime,instance,output,recipient,Some(execution),cx));controller.execution_receipt()})?;result.ok_or_else(||refused("actor failure scoped execution did not run"))??;}else{controller.advance_failure_step(runtime,instance,output,recipient,None,cx)?}Ok(())})?;Ok(())}))?;
   }
   semio_framework_async::yield_once().await;
  }
 }
 /// 🧾️ Settles a retained actual guest receipt before returning its producer for retirement.
 pub(super) fn settle_retirement_step(&mut self,driver:&GuestRelayWakeDriver)->Result<bool,GuestRelayAdmissionFailure>{
  if self.home.is_none(){return Ok(true)}
  let mut snapshot=None;
  driver.dispatch_operation_step(false,true,|cx|{
   with_controller(&mut self.home,cx,|runtime,instance,state,cx|{
    if !pay(cx,size_of::<Snapshot>(),true)?{return Ok(())}
    snapshot=Some(state.controller.as_ref().map(|owner|Snapshot::read(owner,runtime,instance)).transpose()?);
    Ok(())
   })?;
   Ok(())
  })?;
  let Some(state)=snapshot else{return Ok(false)};
  let Some(state)=state else{return Ok(true)};
  if state.failure{
   driver.dispatch_operation_step(false,true,|cx|{
    with_controller_pair(&mut self.home,&mut self.fault,cx,|_,_,state,fault,cx|{
     let controller=state.controller.as_mut().ok_or_else(||refused("retirement lost its native fault controller"))?;
     controller.take_failure_step(cx,|failure,cx|{fault.capture_turn_failure_step(failure,cx)?;Ok(())})?;
     Ok(())
    })?;
    Ok(())
   })?;
   return Ok(false)
  }
  if self.advance_escrow(driver,&state,true)?{return Ok(false)}
  if !state.draining_receipt{return Ok(true)}
  let execution=state.execution;
  driver.dispatch_operation_step(execution,true,|cx|{
   with_controller_pair(&mut self.home,&mut self.output,cx,|runtime,instance,state,output,cx|{
    let controller=state.controller.as_mut().ok_or_else(||refused("retirement lost its native controller"))?;
    if !execution{return controller.retire_receipt_step(runtime,instance,output,None,cx)}
    if !controller.prepare_execution_step(cx)?{return Ok(())}
    let mut result=None;
    GuestRelayWakeDriver::execute_current_original(cx,|execution,cx|{
     result=Some(controller.retire_receipt_step(runtime,instance,output,Some(execution),cx));
     controller.execution_receipt()
    })?;
    result.ok_or_else(||refused("retirement receipt execution did not run"))?
   })?;
   Ok(())
  })?;
  Ok(false)
 }
 fn begin_loan(&mut self,driver:&GuestRelayWakeDriver,purpose:GuestRelayGuestPurpose,header:usize,cleanup:bool)->Result<(),GuestRelayAdmissionFailure>{
  let home=self.home.as_mut().ok_or_else(||refused("cold job lost its native lease"))?;
  let Some(loan)=self.escrow.loan_step(driver,home,purpose,header,cleanup)? else{return Ok(())};
  let mut began=false;
  driver.dispatch_operation_step(false,cleanup,|cx|{
   with_controller(&mut self.home,cx,|_,_,state,cx|{
    began=state.controller.as_mut().ok_or_else(||refused("admission lost its native controller"))?.begin_loan_step(loan,cx)?;
    Ok(())
   })?;
   Ok(())
  })?;
  if began{self.escrow.began_loan();}
  Ok(())
 }
 fn advance_escrow(&mut self,driver:&GuestRelayWakeDriver,state:&Snapshot,cleanup:bool)->Result<bool,GuestRelayAdmissionFailure>{
  if let Some((frontier,loan))=state.frozen{
   let home=self.home.as_mut().ok_or_else(||refused("frozen receipt lost its native lease"))?;
   self.escrow.capture_step(driver,home,frontier,loan,cleanup)?;
   return Ok(true)
  }
  if let Some((header,scalar))=state.header.filter(|_|state.return_ready){
   let home=self.home.as_mut().ok_or_else(||refused("header handback lost its native lease"))?;
   let bound=self.escrow.handback_step(driver,home,header,scalar,cleanup)?;
   return Ok(bound||state.return_required)
  }
  Ok(false)
 }
 pub(super) fn retire_history_step(&mut self,driver:&GuestRelayWakeDriver)->Result<bool,GuestRelayAdmissionFailure>{self.escrow.retire_step(driver)}
 /// 🪪️ Identifies the exact native parent whose physical closure precedes a subsequent hop.
 pub fn recipient_identity(&self)->Result<semio_framework_job::OriginalNativeRecipientIdentity,ValueError>{Ok(self.home.as_ref().ok_or_else(||refused("native parent is already returned"))?.recipient_identity())}
 /// ♻️ Returns each genuine recipient lease to its original Source before parent closure.
 pub fn return_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.home.is_some(){native_admission::return_native(&mut self.home,cx)?;return Ok(false)}if self.output.is_some(){native_admission::return_native(&mut self.output,cx)?;return Ok(false)}if self.fault.is_some(){native_admission::return_native(&mut self.fault,cx)?;return Ok(false)}if self.guest_fault.is_some(){native_admission::return_native(&mut self.guest_fault,cx)?;return Ok(false)}if self.actor_failure.is_some(){native_admission::return_native(&mut self.actor_failure,cx)?;return Ok(false)}Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.home.is_none()&&self.output.is_none()&&self.fault.is_none()&&self.guest_fault.is_none()&&self.actor_failure.is_none()}
 pub fn origin(&self)->GuestRelayNativeOrigin{self.origin}
}
enum JobInput<'a>{Json(&'a mut dyn GuestRelayJsonSource),Bytes(&'a mut dyn GuestRelayByteSource)}
struct AbandonOnDrop<'a>{driver:&'a GuestRelayWakeDriver,armed:bool}
impl Drop for AbandonOnDrop<'_>{fn drop(&mut self){if self.armed{self.driver.abandon_operation()}}}
fn with_controller(home:&mut Option<OriginalNativeLease<GuestRelayNativeHome>>,cx:&mut StepContext<'_>,run:impl FnOnce(&GuestRuntimes,&mut GuestInstance,&mut GuestRelayNativeState,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
 let mut run=Some(run);let mut invoked=false;let borrowed=home.as_mut().ok_or_else(||refused("inference native lease is absent"))?.with_native(cx,|home,cx|{home.with_native(cx,|runtime,instance,_,state,cx|{invoked=true;run.take().ok_or_else(||refused("native origin called twice"))?(runtime,instance,state,cx)})?;Ok(())})?;Ok(borrowed&&invoked)
}
fn with_controller_pair<T:semio_framework_job::OriginalNativeRetirement>(home:&mut Option<OriginalNativeLease<GuestRelayNativeHome>>,other:&mut Option<OriginalNativeLease<T>>,cx:&mut StepContext<'_>,run:impl FnOnce(&GuestRuntimes,&mut GuestInstance,&mut GuestRelayNativeState,&mut T,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
 let mut run=Some(run);let mut invoked=false;let borrowed=home.as_mut().ok_or_else(||refused("inference native lease is absent"))?.with_native_pair(other.as_mut().ok_or_else(||refused("paired native lease is absent"))?,cx,|home,other,cx|{home.with_native(cx,|runtime,instance,_,state,cx|{invoked=true;run.take().ok_or_else(||refused("native origin called twice"))?(runtime,instance,state,other,cx)})?;Ok(())})?;Ok(borrowed&&invoked)
}
fn pay(cx:&mut StepContext<'_>,bytes:usize,retirement:bool)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if (if retirement{cx.retirement_should_yield()}else{cx.should_yield()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

impl std::fmt::Debug for PluginInstanceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginInstanceHandle").field("actor", &self.actor).finish()
    }
}
