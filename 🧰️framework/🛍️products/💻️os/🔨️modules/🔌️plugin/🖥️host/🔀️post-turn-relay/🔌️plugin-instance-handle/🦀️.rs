//! 🔌️ A selected guest handle retains only its actual Source-native leases.
use super::*;
use crate::actor_failure::{ActorFailureHome,ActorFailureCapsule};
use semio_framework_job::{OriginalNativeLease,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_plugin_host_wake::{GuestRelayAdmissionFailure,GuestRelayGuestPurpose,GuestRelayGuestSubloan};
use semio_framework_actor::RetainedTurnInput;
use std::mem::size_of;
#[derive(Default)]
struct Snapshot{encoding:bool,page:bool,done:bool,failed:bool,fault_route:bool,failure:bool,fault_transferred:bool,execution:bool,purpose:Option<GuestRelayGuestPurpose>,wire_header:Option<usize>,receipt:Option<(GuestRelayGuestSubloan,RetainedTurnInput,RetainedCloneProgress,u64)>}
/// 🧬️ The parent retains this handle by value before any inference future can suspend.
pub struct PluginInstanceHandle{pub actor:RuntimeActorId,origin:GuestRelayNativeOrigin,home:Option<OriginalNativeLease<GuestRelayNativeHome>>,output:Option<OriginalNativeLease<GuestRelayOutputHome>>,fault:Option<OriginalNativeLease<GuestRelayFaultHome>>,guest_fault:Option<OriginalNativeLease<GuestRelayOutputHome>>,actor_failure:Option<OriginalNativeLease<ActorFailureHome>>}
impl PluginInstanceHandle{
 pub(super) fn from_admitted(actor:RuntimeActorId,origin:GuestRelayNativeOrigin,home:OriginalNativeLease<GuestRelayNativeHome>,output:OriginalNativeLease<GuestRelayOutputHome>,fault:OriginalNativeLease<GuestRelayFaultHome>,guest_fault:OriginalNativeLease<GuestRelayOutputHome>,actor_failure:OriginalNativeLease<ActorFailureHome>)->Self{Self{actor,origin,home:Some(home),output:Some(output),fault:Some(fault),guest_fault:Some(guest_fault),actor_failure:Some(actor_failure)}}
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
  loop{
   let mut snapshot=None;
   driver.dispatch_operation_step(false,false,|cx|{with_controller(&mut self.home,cx,|runtime,instance,state,cx|{
    if state.controller.is_none(){if !pay(cx,size_of::<GuestRelayColdController>(),false)?{return Ok(())}state.controller=Some(GuestRelayColdController::new(1,kind)?)}
    let owner=state.controller.as_ref().unwrap();if owner.kind()!=kind{return Err(refused("cold job kind changed after admission").into())}snapshot=Some(Snapshot{encoding:owner.needs_encoding(),page:owner.needs_page(),done:owner.done(),failed:owner.failed(),fault_route:owner.failure_route(),failure:owner.has_failure(),fault_transferred:owner.failure_transferred(),execution:owner.requires_execution(runtime,instance)?,purpose:owner.needs_loan(),wire_header:owner.required_wire_header(),receipt:owner.receipt()});Ok(())
   })?;Ok(())})?;
   let Some(state)=snapshot else{semio_framework_async::yield_once().await;continue};
   if state.fault_route{let result=self.drive_actor_failure(driver,out).await;if result.is_ok(){guard.armed=false}return result}
   if state.fault_transferred{
    driver.dispatch_operation_step(false,true,|cx|{if let Some(fault)=GuestRelayFault::take_captured_step(&mut self.fault,cx)?{*out=Some(GuestRelayResult::Failure(fault))}Ok(())})?;if out.is_some(){guard.armed=false;return Ok(())}
   }else if state.failure{
    let fault=&mut self.fault;driver.dispatch_operation_step(false,true,|cx|{with_controller_pair(&mut self.home,fault,cx,|_,_,state,fault,cx|{let controller=state.controller.as_mut().ok_or_else(||refused("fault lost its native controller"))?;controller.take_failure_step(cx,|failure,cx|{fault.capture_turn_failure_step(failure,cx)?;Ok(())})?;Ok(())})?;Ok(())})?;
   }else if state.done{
    let target=if state.failed{&mut self.guest_fault}else{&mut self.output};driver.dispatch_operation_step(false,false,|cx|{if state.failed{if let Some(fault)=GuestRelayFault::take_guest_failure_step(target,cx)?{*out=Some(GuestRelayResult::Failure(fault))}}else if let Some(output)=GuestRelayOutput::take_sealed_step(target,cx)?{*out=Some(GuestRelayResult::Success(output))}Ok(())})?;if out.is_some(){guard.armed=false;return Ok(())}
   }else if let Some((loan,original,spent,work))=state.receipt{
    let proof=driver.receive_guest_subloan_witness(loan,original,spent,work).map_err(GuestRelayAdmissionFailure::from)?;let producer=self.home.as_ref().ok_or_else(||refused("settled producer is absent"))?.recipient_identity();driver.dispatch_operation_step(false,false,|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("settlement lost its native controller"))?.settled_step(proof,producer,cx)?;Ok(())})?;Ok(())})?;
   }else if let Some(purpose)=state.purpose{
    let home=self.home.as_mut().ok_or_else(||refused("inference lost its native lease"))?;let loan=match driver.pending_guest_subloan(home)?{Some(loan)=>Some(loan),None=>driver.admit_guest_subloan(home,purpose,state.wire_header.ok_or_else(||refused("fixed crossing has no wire header"))?)?};if let Some(loan)=loan{driver.dispatch_operation_step(false,false,|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("admission lost its native controller"))?.begin_loan_step(loan,cx)?;Ok(())})?;Ok(())})?;}
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
   let mut snapshot=None;driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller(&mut self.home,cx,|runtime,instance,state,cx|{if !pay(cx,size_of::<Snapshot>(),true)?{return Ok(())}let owner=state.controller.as_ref().ok_or_else(||refused("actor failure lost its controller"))?;snapshot=Some(Snapshot{encoding:owner.needs_encoding(),page:owner.needs_page(),done:owner.done(),failed:owner.failed(),fault_route:owner.failure_route(),failure:owner.has_failure(),fault_transferred:owner.failure_transferred(),execution:owner.requires_execution(runtime,instance)?,purpose:owner.needs_loan(),wire_header:owner.required_wire_header(),receipt:owner.receipt()});Ok(())})?;Ok(())}))?;
   let Some(state)=snapshot else{semio_framework_async::yield_once().await;continue};if !state.fault_route{return Err(refused("cleanup cannot publish ordinary job output").into())}
   if state.fault_transferred{
    driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{if let Some(fault)=GuestRelayFault::take_captured_step(&mut self.fault,cx)?{*out=Some(GuestRelayResult::Failure(fault))}Ok(())}))?;if out.is_some(){return Ok(())}
   }else if state.failure{
    let fault=&mut self.fault;driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller_pair(&mut self.home,fault,cx,|_,_,state,fault,cx|{let controller=state.controller.as_mut().ok_or_else(||refused("actor failure host fault lost its controller"))?;controller.take_failure_step(cx,|failure,cx|{fault.capture_turn_failure_step(failure,cx)?;Ok(())})?;Ok(())})?;Ok(())}))?;
   }else if state.done{
    let recipient=self.actor_failure.as_ref().ok_or_else(||refused("actor failure recipient is absent"))?.recipient_identity();driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{let mut permit=None;with_controller(&mut self.home,cx,|_,_,state,cx|{permit=state.controller.as_ref().ok_or_else(||refused("actor failure lost its controller"))?.failure_permit_step(recipient,cx)?;Ok(())})?;if let Some(permit)=permit{if let Some(fault)=ActorFailureCapsule::take_step(&mut self.actor_failure,&permit,cx)?{*out=Some(GuestRelayResult::Failure(GuestRelayFault::ActorFailure(fault)))}}Ok(())}))?;if out.is_some(){return Ok(())}
   }else if let Some((loan,input,spent,work))=state.receipt{
    let proof=driver.receive_guest_subloan_witness(loan,input,spent,work).map_err(GuestRelayAdmissionFailure::from)?;let producer=self.home.as_ref().ok_or_else(||refused("settled producer is absent"))?.recipient_identity();driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("actor failure settlement lost its controller"))?.settled_step(proof,producer,cx)?;Ok(())})?;Ok(())}))?;
   }else if let Some(purpose)=state.purpose{
    if purpose!=GuestRelayGuestPurpose::Infrastructure{return Err(refused("actor failure cannot consume semantic stock").into())}let home=self.home.as_mut().ok_or_else(||refused("actor failure producer is absent"))?;let loan=match driver.pending_guest_subloan(home)?{Some(loan)=>Some(loan),None=>driver.admit_cleanup_guest_subloan(home,state.wire_header.ok_or_else(||refused("cleanup crossing has no wire header"))?)?};if let Some(loan)=loan{if loan.purpose()!=GuestRelayGuestPurpose::Infrastructure{return Err(refused("actor failure retained another purpose").into())}driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{state.controller.as_mut().ok_or_else(||refused("actor failure admission lost its controller"))?.begin_loan_step(loan,cx)?;Ok(())})?;Ok(())}))?;}
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
  if self.home.is_none(){return Ok(true)}let mut snapshot=None;driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{if !pay(cx,size_of::<Snapshot>(),true)?{return Ok(())}snapshot=Some(state.controller.as_ref().and_then(|controller|controller.receipt()));Ok(())})?;Ok(())}))?;
  let Some(receipt)=snapshot else{return Ok(false)};let Some((loan,input,spent,work))=receipt else{return Ok(true)};let proof=driver.receive_guest_subloan_witness(loan,input,spent,work).map_err(GuestRelayAdmissionFailure::from)?;let producer=self.home.as_ref().ok_or_else(||refused("settled producer is absent"))?.recipient_identity();
  let mut settled=false;driver.dispatch_operation_step(false,true,|cx|cx.with_original_retirement(|cx|{with_controller(&mut self.home,cx,|_,_,state,cx|{settled=state.controller.as_mut().ok_or_else(||refused("retirement lost its controller"))?.settled_step(proof,producer,cx)?;Ok(())})?;Ok(())}))?;Ok(settled)
 }
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
