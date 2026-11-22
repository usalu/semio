//! 🎛️ The outer owner keeps its actual Driver outside every Source-native instance and yields only after complete handback.
use super::*;
use semio_framework_actor::ActorId;
use semio_framework_job::{OriginalNativeLease,OriginalNativeRecipientIdentity,OriginalNativeRetiredWitness,OriginalNativeRetirement};
use semio_plugin_host_post_turn_relay::{GuestRelayFaultHome,NativeHomeAdmission,NativeInstanceHome,GuestRelayNativeOrigin,with_bootstrap_history_parts,attest_bootstrap_header_return,with_frozen_history_parts,attest_header_return,attest_actor_failure_if_needed,FrozenReceiptFrontier,GuestFrozenHistoryCapture};
use semio_plugin_host_wake::{GuestRelayWakeDriver,GuestRelayAdmissionCall,GuestRelayAdmissionFailure,GuestRelayGuestSubloan,GuestRelayGuestPurpose,GuestIssuedHistoryHome};
use std::mem::ManuallyDrop;
use super::event_json::PollPage;
use semio_framework_kernel::Event;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GuestPollSessionStatus{Pending,Ready,Delivered,Complete,Closed,Refused}
pub struct GuestPollSession<T:NativeReceive<DslValue>+RetireOwned+Send+Sync+'static,R:GuestPollFailureReceiving>{
 driver:ManuallyDrop<GuestRelayWakeDriver>,admission:ManuallyDrop<Option<NativeHomeAdmission<GuestPoll<T,R>>>>,unopened:ManuallyDrop<Option<GuestPoll<T,R>>>,producer:ManuallyDrop<Option<OriginalNativeLease<NativeInstanceHome<GuestPoll<T,R>>>>>,history_input:ManuallyDrop<Option<GuestIssuedHistoryHome>>,history:ManuallyDrop<Option<OriginalNativeLease<GuestIssuedHistoryHome>>>,fault_input:ManuallyDrop<Option<semio_plugin_host_actor_failure::ActorFailureHome>>,fault:ManuallyDrop<Option<OriginalNativeLease<semio_plugin_host_actor_failure::ActorFailureHome>>>,fault_id:Option<OriginalNativeRecipientIdentity>,failure:ManuallyDrop<Option<GuestRelayAdmissionFailure>>,cleanup_failure:ManuallyDrop<Option<GuestRelayAdmissionFailure>>,
 next_place:OriginalCallerReturnPlace,next_result:ManuallyDrop<Option<Result<bool,OriginalTurnFailure>>>,
 diagnostic: GuestRelayFaultHome,
 guest:Option<GuestRelayGuestSubloan>,request:Option<GuestPollGuestLoan>,frozen:Option<GuestPollFrozenKind>,producer_id:Option<OriginalNativeRecipientIdentity>,history_id:Option<OriginalNativeRecipientIdentity>,witness:Option<OriginalNativeRetiredWitness>,stage:u8,capture:GuestFrozenHistoryCapture,close_phase:u8,opened:bool,closing:bool,closed:bool,delivered:bool,limits:Option<semio_framework_kernel_identity::OwnedCorePollLimits>,
 trace:Option<PollProgressTrace>,
}
#[derive(Default)]
struct PollProgressTrace{driver_steps:usize,native_steps:usize,input_receipts:usize}
fn poll_trace_sample(steps:&mut usize)->Option<usize>{
 *steps=steps.saturating_add(1);
 steps.is_power_of_two().then_some(*steps)
}
impl<T:NativeReceive<DslValue>+RetireOwned+Send+Sync+'static,R:GuestPollFailureReceiving> GuestPollSession<T,R>{
 pub fn new(driver:GuestRelayWakeDriver,actor:ActorId,origin:GuestRelayNativeOrigin,policy:OriginalNativeDecodePolicy,limits:JsonReadLimits,receiver:R)->Self{
  let(admission,unopened,failure)=match NativeHomeAdmission::new(actor,origin,GuestPoll::new(policy,limits,receiver)){Ok(admission)=>(Some(admission),None,None),Err((state,error))=>(None,Some(state),Some(GuestRelayAdmissionFailure::from(error)))};
  Self{driver:ManuallyDrop::new(driver),admission:ManuallyDrop::new(admission),unopened:ManuallyDrop::new(unopened),producer:ManuallyDrop::new(None),history_input:ManuallyDrop::new(Some(GuestIssuedHistoryHome::empty())),history:ManuallyDrop::new(None),fault_input:ManuallyDrop::new(Some(semio_plugin_host_actor_failure::ActorFailureHome::empty())),fault:ManuallyDrop::new(None),fault_id:None,failure:ManuallyDrop::new(failure),cleanup_failure:ManuallyDrop::new(None),next_place:OriginalCallerReturnPlace::empty(),next_result:ManuallyDrop::new(None),diagnostic:GuestRelayFaultHome::empty(),guest:None,request:None,frozen:None,producer_id:None,history_id:None,witness:None,stage:0,capture:GuestFrozenHistoryCapture::default(),close_phase:0,opened:false,closing:false,closed:false,delivered:false,limits:None,trace:std::env::var_os("RUST_BACKTRACE").is_some().then(PollProgressTrace::default)}
 }
 #[cfg(test)]
 pub(super) fn fixture_guest_request(&self)->Option<(bool,OriginalJobBudget,GuestPollGuestLoan)>{
  Some((self.stage==4,self.guest?.budget(),self.request?))
 }
 #[cfg(test)]
 pub(super) fn fixture_execution_spent(&self)->u64{self.driver.original_execution_spent().unwrap()}
 #[cfg(test)]
 pub(super) fn fixture_drive_copy(&self)->usize{self.driver.original_operation_drive_grant().unwrap().unwrap().maximum_copy_bytes}
 #[cfg(test)]
 pub(super) fn fixture_drive_release(&self)->usize{self.driver.original_operation_drive_grant().unwrap().unwrap().maximum_release_bytes}
 #[cfg(test)]
 pub(super) fn fixture_guest_owner(&self)->OriginalNativeRecipientIdentity{self.guest.unwrap().producer_identity()}

 pub fn failure(&self)->Option<&GuestRelayAdmissionFailure>{self.failure.as_ref()}
 pub fn cleanup_failure(&self)->Option<&GuestRelayAdmissionFailure>{self.cleanup_failure.as_ref()}
 fn retain(&mut self,result:Result<bool,GuestRelayAdmissionFailure>,cleanup:bool)->bool{match result{Ok(progress)=>progress,Err(error)=>{if cleanup{assert!(self.cleanup_failure.is_none());*self.cleanup_failure=Some(error)}else{assert!(self.failure.is_none());*self.failure=Some(error)}false}}}
 pub fn open_step(&mut self)->GuestPollSessionStatus{
  if self.failure.is_some()||self.cleanup_failure.is_some(){return GuestPollSessionStatus::Refused}if self.opened{return GuestPollSessionStatus::Ready}if self.closing||self.closed{return GuestPollSessionStatus::Refused}
  if self.producer.is_none(){let admission=self.admission.as_mut().unwrap();let producer=&mut*self.producer;let result=self.driver.dispatch_admission_step(admission.requires_execution(),|execution,cx|{let mut call=admission.advance(execution,cx);if call.result.is_ok(){call.result=admission.take_step(producer,cx).map(|_|());}call});self.retain(result,false);if self.producer.is_some(){self.producer_id=self.producer.as_ref().map(OriginalNativeLease::recipient_identity)}return if self.failure.is_some(){GuestPollSessionStatus::Refused}else{GuestPollSessionStatus::Pending}}
  if self.history.is_none(){let input=&mut*self.history_input;let history=&mut*self.history;let result=self.driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:cx.admit_original_native(input,history).map(|_|()),execution:None});self.retain(result,false);if self.history.is_some(){self.history_id=self.history.as_ref().map(OriginalNativeLease::recipient_identity)}return if self.failure.is_some(){GuestPollSessionStatus::Refused}else{GuestPollSessionStatus::Pending}}
  if self.fault.is_none(){let input=&mut*self.fault_input;let fault=&mut*self.fault;let result=self.driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:cx.admit_original_native(input,fault).map(|_|()),execution:None});self.retain(result,false);if self.fault.is_some(){self.fault_id=self.fault.as_ref().map(OriginalNativeLease::recipient_identity)}return if self.failure.is_some(){GuestPollSessionStatus::Refused}else{GuestPollSessionStatus::Pending}}
  let result=(||{let(request,deadline)=self.driver.operation_request_limits()?;let policy=self.driver.policy()?;self.limits=Some(semio_framework_kernel_identity::OwnedCorePollLimits::try_from(PollRequestLimits{retained_policy:request.retained,work_fuel:request.work_units,fuel:policy.root.turn_execution_fuel,deadline_us:deadline,max_effects:policy.root.max_effects,max_patch_bytes:policy.root.max_patch_bytes,max_frames:policy.root.max_frames})?);Ok(true)})();if self.retain(result,false){self.opened=true;GuestPollSessionStatus::Ready}else{GuestPollSessionStatus::Refused}
 }
 /// 🔎️ The constructor's immutable origin is lent only under the actual producer Source callback.
 pub fn with_native_step(&mut self,receive:impl FnOnce(&GuestRuntimes,&mut GuestInstance,&CompiledHandle,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->GuestPollSessionStatus{if !self.opened||self.closing||self.closed||self.failure.is_some()||self.cleanup_failure.is_some(){return GuestPollSessionStatus::Refused}let producer=self.producer.as_mut().unwrap();let result=self.driver.dispatch_operation_step(false,false,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|runtime,instance,compiled,_,cx|receive(runtime,instance,compiled,cx))?;Ok(())})?;Ok(())});if self.retain(result,false){GuestPollSessionStatus::Ready}else if self.failure.is_some(){GuestPollSessionStatus::Refused}else{GuestPollSessionStatus::Pending}}

 /// 🧯️ Receives the producer's complete retained result under its original Source, including during cleanup.
 pub fn receive_state_step(&mut self,receive:impl FnOnce(&mut GuestPoll<T,R>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->GuestPollSessionStatus{
  if self.closed||self.producer.is_none()||self.cleanup_failure.is_some(){return GuestPollSessionStatus::Refused}
  let producer=self.producer.as_mut().unwrap();let result=self.driver.dispatch_operation_step(false,true,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|_,_,_,poll,cx|receive(poll,cx))?;Ok(())})?;Ok(())});
  if self.retain(result,true){GuestPollSessionStatus::Ready}else if self.cleanup_failure.is_some(){GuestPollSessionStatus::Refused}else{GuestPollSessionStatus::Pending}
 }
 pub fn take_failure_step(&mut self,cleanup:bool,receive:impl FnOnce(&mut Option<GuestRelayAdmissionFailure>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,GuestRelayAdmissionFailure>{let failure=if cleanup{&mut*self.cleanup_failure}else{&mut*self.failure};let mut taken=false;self.driver.dispatch_operation_step(false,true,|cx|{receive(failure,cx)?;taken=failure.is_none();Ok(())})?;Ok(taken)}
}

/// 📥️ The actual session constructs immutable request DATA; it is not a callback grant or Source identity.
pub struct GuestPollSessionInput<'a,S:JsonWriteSource+?Sized>{events:&'a S,pages:[PollPage<'a>;2],limits:semio_framework_kernel_identity::OwnedCorePollLimits,operation:(u64,u64),_pinned:std::marker::PhantomPinned}
impl<S:JsonWriteSource+?Sized> JsonWriteSource for GuestPollSessionInput<'_,S>{
 fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError>{use semio_framework_pack_json::JsonWriteNode as N;if path.is_empty(){return Ok(N::Object(1))}if path[0]!=0{return Err(ValueError::invariant("Poll request tag path is absent"))}if path.len()==1{return Ok(N::Object(4))}match path[1]{0=>self.events.node_at_path(&path[2..]),1|2=>self.pages[path[1]-1].node(&path[2..]),3=>super::event_json::limits_node(&self.limits,&path[2..]),_=>Err(ValueError::invariant("Poll request field path is absent"))}}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty(){return if index==0{Ok("Poll")}else{Err(ValueError::invariant("Poll request tag is absent"))}}if path[0]!=0{return Err(ValueError::invariant("Poll request tag path is absent"))}if path.len()==1{return ["events","command_page","cold_pair_page","limits"].get(index).copied().ok_or_else(||ValueError::invariant("Poll request field is absent"))}match path[1]{0=>self.events.object_key_at_path(&path[2..],index),1|2=>self.pages[path[1]-1].key(&path[2..],index),_=>Err(ValueError::invariant("Poll request object path is absent"))}}
}

impl<T:NativeReceive<DslValue>+RetireOwned+Send+Sync+'static,R:GuestPollFailureReceiving> GuestPollSession<T,R>{
 /// 🔁️ Captures and settles the actual next-command reset before releasing another request to the resident guest.
 pub fn next_input_step(&mut self)->GuestPollSessionStatus{
  if self.failure.is_some()||self.cleanup_failure.is_some()||!self.opened||self.closing||self.closed{return GuestPollSessionStatus::Refused}if !self.delivered&&self.next_result.is_none()&&!self.next_place.is_admitted(){return GuestPollSessionStatus::Ready}if self.stage!=0||self.guest.is_some()||self.request.is_some()||self.frozen.is_some(){return GuestPollSessionStatus::Pending}
  let producer=self.producer.as_mut().unwrap();let place=&mut self.next_place;let result=&mut*self.next_result;let delivered=&mut self.delivered;let mut ready=false;
  let turn=self.driver.dispatch_operation_step(false,false,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|_,_,_,poll,cx|{
   if place.is_admitted()||result.is_some(){settle_next_round(place,result,delivered,cx)?;ready=!*delivered;return Ok(())}
   let address=(result as*mut Option<Result<bool,OriginalTurnFailure>>)as usize;let bytes=std::mem::size_of_val(result);if !place.admit_sized(cx,address,bytes)?{return Ok(())}if !place.preissue_capture(cx,address,bytes)?{return Ok(())}*result=Some(poll.begin_next_step(cx));Ok(())
  })?;Ok(())})?;Ok(())});self.retain(turn,false);if self.failure.is_some(){GuestPollSessionStatus::Refused}else if ready{GuestPollSessionStatus::Ready}else{GuestPollSessionStatus::Pending}
 }
 pub fn input<'a,S:JsonWriteSource+?Sized>(&self,events:&'a S)->Result<GuestPollSessionInput<'a,S>,ValueError>{if !self.opened{return Err(ValueError::invariant("Poll input requires its opened session"))}let policy=self.driver.policy()?;Ok(GuestPollSessionInput{events,pages:[PollPage::Empty;2],limits:self.limits.ok_or_else(||ValueError::invariant("Poll input lost its authenticated limits"))?,operation:(policy.root.operation,policy.root.generation),_pinned:std::marker::PhantomPinned})}

 /// 📨️ Drives an ordinary event batch or one dedicated transport page using the existing Source.
 pub fn event_input<'a>(&self, source: &'a EventJsonSource<'a>, page: Option<&'a Event>) -> Result<GuestPollSessionInput<'a, EventJsonSource<'a>>, ValueError> {
  let mut input = self.input(source)?;
  match page {
   Some(Event::CommandIngressPage { cursor, bytes }) => input.pages[0] = PollPage::Command(cursor, bytes.as_slice()),
   Some(Event::ColdDocumentPairPage(page)) => input.pages[1] = PollPage::Cold(page),
   Some(_) => return Err(ValueError::invariant("Poll dedicated page is not a command or cold pair page")),
   None => {}
  }
  Ok(input)
 }
 fn with_frozen(&mut self,cleanup:bool,receive:impl for<'a,'b> FnOnce(semio_framework_job::OriginalNativeBorrowed<'a,GuestIssuedHistoryHome>,semio_plugin_host_guest_runtime::GuestFrozenEscrowBorrow<'b>,&mut GuestPoll<T,R>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->bool{
  let frontier=if self.frozen==Some(GuestPollFrozenKind::Input){FrozenReceiptFrontier::Unit}else{FrozenReceiptFrontier::Scalar};let history=self.history.as_mut().unwrap();let producer=self.producer.as_mut().unwrap();let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{with_frozen_history_parts(history,producer,frontier,cx,receive)?;Ok(())});self.retain(result,cleanup)
 }
 fn acknowledge(&mut self,cleanup:bool)->Option<GuestPollSessionStatus>{
  let producer=self.producer.as_mut().unwrap();let diagnostic=&mut self.diagnostic;let mut status=None;let mut request=None;let mut frozen=None;let mut awaiting_control=false;let mut result=self.driver.dispatch_operation_step(false,cleanup,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|_,_,_,poll,cx|{
   let actor_failure=poll.journal_identity.is_none()&&poll.actor_failure.is_some();
   if !cleanup&&(actor_failure||poll.scalar_progress().is_some_and(scalar_failed)||poll.failed&&poll.decoding_outcome().is_some()){
    if std::env::var_os("RUST_BACKTRACE").is_some(){
     eprintln!("[DEBUG] poll refusal in {:?}: actor={}, transport={:?}, input={:?}, scalar={:?}, decoding={}",poll.phase,poll.actor_failure.is_some(),poll.transport_failure(),poll.input_failure(),poll.scalar_progress().map(|progress|&progress.result),poll.decoding_outcome().is_some());
    }
    status=Some(GuestPollSessionStatus::Refused);return Ok(())
   }
   if poll.transport_failure().is_some(){
    poll.take_transport_failure_step(cx,|failure,_,cx|receive_poll_turn_failure_step(diagnostic,failure,cleanup,cx))?;
    status=Some(GuestPollSessionStatus::Ready);return Ok(())
   }
   if poll.input_failure().is_some(){
    poll.take_input_failure_step(cx,|failure,_,cx|receive_poll_turn_failure_step(diagnostic,failure,cleanup,cx))?;
    status=Some(GuestPollSessionStatus::Ready);return Ok(())
   }
   if cleanup{
    if poll.scalar_progress().is_some_and(scalar_failed){
     poll.take_scalar_failure_step(cx,|failure,_,cx|retire_scalar_failure_step(diagnostic,failure,cx))?;
     status=Some(GuestPollSessionStatus::Ready);return Ok(())
    }
    if actor_failure{return Err(invalid("Poll still owns its original actor failure reference"))}
   }
   if poll.input_progress_pending(){awaiting_control=!poll.acknowledge_input_progress_step(cx)?;return Ok(())}if poll.scalar_progress().is_some(){poll.acknowledge_scalar_progress_step(cx)?;return Ok(())}if poll.scalar_result().is_some(){poll.acknowledge_scalar_result_step(cx)?;return Ok(())}if poll.raw_word().is_some(){poll.take_raw_word_step(cx)?;return Ok(())}
   if poll.fixed_receipt().is_some(){poll.acknowledge_fixed_step(cx)?;return Ok(())}
   if poll.outcome.is_some()||poll.settlement.is_some(){poll.retire_outcome_step(cx)?;return Ok(())}
   if poll.terminal_is_empty(){status=Some(GuestPollSessionStatus::Complete);return Ok(())}
   let Some(kind)=poll.frozen_kind_step(cx)? else { awaiting_control=true;return Ok(()) };
   frozen=kind;
   if cleanup&&((matches!(poll.phase,Phase::Encode|Phase::Boot|Phase::Poll)&&!poll.scalar_started)||(poll.phase==Phase::Input&&poll.unit.needs_admission_budget()))&&!poll.frozen{status=Some(GuestPollSessionStatus::Pending);return Ok(())}
   if frozen.is_none()&&self.guest.is_none(){let Some(next)=poll.next_guest_subloan_step(cx)? else { return Ok(()) };request=next;}
   status=Some(GuestPollSessionStatus::Pending);Ok(())
  })?;Ok(())})?;Ok(())});
  if result.is_ok()&&awaiting_control&&!cleanup&&self.guest.is_some(){
   result=result.and_then(|progress|{
    if self.driver.original_operation_drive_grant()?.is_some_and(|grant|grant.maximum_copy_bytes<control_bytes()){
     return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"ordinary Poll copy budget cannot fund its acknowledgment frame").into())
    }
    Ok(progress)
   });
  }
  self.retain(result,cleanup);if let Some(frozen)=frozen{self.frozen=Some(frozen);self.stage=if matches!(frozen,GuestPollFrozenKind::Boot|GuestPollFrozenKind::CloseContext){9}else{10};}else if let Some(request)=request{self.request=Some(request);self.stage=1;}status
 }
 fn advance_native(&mut self,input:Option<&dyn JsonWriteSource>,cleanup:bool)->Option<GuestPollStatus>{
  let sample=self.trace.as_mut().and_then(|trace|poll_trace_sample(&mut trace.native_steps));
  let mut frontier=None;
  let mut returned_frame_demand=None;
  let producer=self.producer.as_mut().unwrap();let budget=self.guest.map(|loan|loan.budget());let mut status=None;let result=self.driver.dispatch_operation_step(true,cleanup,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|runtime,instance,_,poll,cx|{
   if sample.is_some(){frontier=Some((poll.phase,poll.scalar_started,poll.frozen,poll.awaiting_input_budget));}
   if poll.awaiting_input_budget{
    if cleanup&&poll.retire_interrupted_guest_step(runtime,instance,budget,cx)? { return Ok(()) }
    if let Some(budget)=budget{poll.repeat_input_step(runtime,instance,budget,cx)?;}
    return Ok(())
   }
   if !poll.prepare_step(runtime,instance,cx)?{return Ok(())}
   if poll.requires_execution(runtime,instance)?{
    let mut body=None;
    let settlement=GuestRelayWakeDriver::execute_current_original(cx,|execution,cx|{
     body=Some(match input{Some(input)=>poll.advance_step(runtime,instance,input,budget,Some(execution),cx),None=>poll.advance_retirement_step(runtime,instance,budget,Some(execution),cx)});
     poll.execution_receipt()
    });
    if (settlement.is_err() || matches!(body, Some(Err(_) | Ok(GuestPollStatus::Refused)))) && std::env::var_os("RUST_BACKTRACE").is_some(){
     eprintln!("[DEBUG] poll VM body: {body:?}, input failure: {:?}, scalar failure: {:?}, settlement: {settlement:?}",poll.input_failure(),poll.scalar_progress().and_then(|progress|progress.result.as_ref().err()));
    }
    settlement?;
    status=Some(body.ok_or_else(||invalid("Poll VM callback did not execute"))??);
    if !cleanup&&budget.is_some()&&status==Some(GuestPollStatus::Pending){
     returned_frame_demand=runtime.original_returned_frame_demand(instance,cx)?;
    }
   }else{
    let progress=match input{Some(input)=>poll.advance_step(runtime,instance,input,budget,None,cx)?,None=>poll.advance_retirement_step(runtime,instance,budget,None,cx)?};
    if !cleanup&&progress==GuestPollStatus::Refused&&std::env::var_os("RUST_BACKTRACE").is_some(){
     eprintln!("[DEBUG] poll non-VM refusal in {:?}: actor={}, input={:?}, scalar={:?}, fixed={}",poll.phase,poll.actor_failure.is_some(),poll.input_failure(),poll.scalar_progress().map(|progress|&progress.result),poll.fixed_receipt().is_some());
     if poll.input_failure().is_some(){
      if let Some(diagnostics)=instance.guest_diagnostics_text(){
       if let Some((_,detail))=diagnostics.rsplit_once("[DEBUG] unresolved original input receipt"){
        for line in detail.lines().skip(1).take(2){eprintln!("[DEBUG] poll guest Input: {line}");}
       }
      }
     }
    }
    status=Some(progress);
   }
   if !cleanup&&status==Some(GuestPollStatus::Refused)&&poll.input_failure().is_some(){
    status=Some(GuestPollStatus::Pending);
   }
   Ok(())
  })?;Ok(())})?;Ok(())});
  if let Some(step)=sample{
   eprintln!("[DEBUG] poll native progress: step {step}, frontier {frontier:?}, guest loan epoch {:?}, VM fuel spent {:?}, status {status:?}",budget.map(|loan|loan.input.epoch),self.driver.original_execution_spent());
  }
  let result=result.and_then(|progress|{
   if let Some(demand)=returned_frame_demand{
    if self.driver.original_operation_drive_grant()?.is_some_and(|grant|!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)){
     return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"ordinary Poll budget cannot retire its returned Core frame").into())
    }
   }
   Ok(progress)
  });
  self.retain(result,cleanup);status
 }
 /// ▶️ One bounded step finishes its Source callback and every actual VM receipt before returning.
 pub fn drive_step<S:JsonWriteSource+?Sized>(&mut self,input:std::pin::Pin<&GuestPollSessionInput<'_,S>>,receive:&mut impl FnMut(&T,&mut R,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>)->GuestPollSessionStatus{
  if self.failure.is_some()||self.cleanup_failure.is_some(){return GuestPollSessionStatus::Refused}if !self.opened{return self.open_step()}if self.closing||self.closed{return GuestPollSessionStatus::Refused}
  if self.next_place.is_admitted()||self.next_result.is_some(){return GuestPollSessionStatus::Pending}let input=input.get_ref();match self.driver.policy(){Ok(policy)if input.operation==(policy.root.operation,policy.root.generation)=>{},Ok(_)=>{self.retain(Err(ValueError::invariant("Poll input belongs to another actual session").into()),false);return GuestPollSessionStatus::Refused},Err(error)=>{self.retain(Err(error.into()),false);return GuestPollSessionStatus::Refused}}
  self.drive_inner(Some(input),false,receive)
 }
 fn drive_inner(&mut self,input:Option<&dyn JsonWriteSource>,cleanup:bool,receive:&mut impl FnMut(&T,&mut R,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>)->GuestPollSessionStatus{
  if let Some(step)=self.trace.as_mut().and_then(|trace|poll_trace_sample(&mut trace.driver_steps)){
   eprintln!("[DEBUG] poll driver progress: step {step}, stage {}, receipt {:?}, frozen {:?}, retirement {cleanup}, VM fuel spent {:?}",self.stage,self.capture,self.frozen,self.driver.original_execution_spent());
  }
  let mut complete=false;
  match self.stage{
   0=>{match self.acknowledge(cleanup){Some(GuestPollSessionStatus::Pending) if self.stage==0=>self.stage=4,Some(GuestPollSessionStatus::Pending|GuestPollSessionStatus::Ready)|None=>{},Some(status)=>return status}},
   1=>{let history=self.history.as_mut().unwrap();let mut ready=false;let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{history.with_native_borrow(cx,|guard,cx|{ready=GuestIssuedHistoryHome::reserve_step(guard,cx)?;Ok(())})?;Ok(())});self.retain(result,cleanup);if ready{self.stage=2}},
   2=>{let request=self.request.unwrap();let producer=self.producer.as_mut().unwrap();let result=if cleanup{self.driver.admit_cleanup_guest_subloan(producer,request.header_bytes)}else{self.driver.admit_guest_subloan(producer,match request.purpose{GuestPollGuestPurpose::Infrastructure=>GuestRelayGuestPurpose::Infrastructure,GuestPollGuestPurpose::Semantic=>GuestRelayGuestPurpose::Semantic},request.header_bytes)};match result{Ok(Some(loan))=>{self.guest=Some(loan);self.stage=3},Ok(None)=>{},Err(error)=>{self.retain(Err(error),cleanup);}}},
   3=>{let history=self.history.as_mut().unwrap();let producer=self.producer.as_mut().unwrap();let loan=self.guest.unwrap();let mut ready=false;let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{history.with_native_pair_borrow(producer,cx,|history,producer,cx|{ready=GuestIssuedHistoryHome::commit_issued_step(history,&producer,loan,cx)?;Ok(())})?;Ok(())});self.retain(result,cleanup);if ready{self.stage=4}},
   4=>{
    self.stage=0;
    match self.advance_native(input,cleanup){
     Some(GuestPollStatus::Output)=>{
      let producer=self.producer.as_mut().unwrap();let mut delivered=false;
      let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|_,_,_,poll,cx|{
       if cleanup{poll.discard_cancelled_step(cx)?;}else{
        delivered=poll.deliver_step(cx,|value,receiver,cx|receive(value,receiver,cx))?;
       }Ok(())
      })?;Ok(())})?;Ok(())});
      self.retain(result,cleanup);
      if delivered{self.delivered=true;return GuestPollSessionStatus::Delivered}
     },
     Some(GuestPollStatus::Refused) if !cleanup=>return GuestPollSessionStatus::Refused,
     _=>{}
    }
   },
   8=>{let producer=self.producer.as_mut().unwrap();let mut ready=false;let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|runtime,instance,_,poll,cx|{ready=poll.resume_bootstrap_step(runtime,instance,cx)?;Ok(())})?;Ok(())})?;Ok(())});self.retain(result,cleanup);if ready{self.guest=None;self.request=None;self.stage=29}},
   9=>{let loan=self.guest.unwrap();let history=self.history.as_mut().unwrap();let producer=self.producer.as_mut().unwrap();let mut selected=None;let mut ready=false;let result=self.driver.dispatch_escrow_reconciliation_step(cleanup,|cx,mut issuer|{selected=with_bootstrap_history_parts(history,producer,cx,|history,frozen,_,cx|{ready=issuer.apply_bootstrap(history,&frozen,loan,cx)?;Ok(())})?;Ok(())});self.retain(result,cleanup);if selected==Some(false){self.stage=10}else if ready{self.stage=28}},
   28=>{let history=self.history.as_mut().unwrap();let producer=self.producer.as_mut().unwrap();let mut ready=false;let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{with_bootstrap_history_parts(history,producer,cx,|history,frozen,poll,cx|{GuestIssuedHistoryHome::with_bootstrap_call_step(history,&frozen,cx,|call,identity,cx|{ready=poll.accept_bootstrap_step(call,identity,cx)?;Ok(())})?;Ok(())})?;Ok(())});self.retain(result,cleanup);if ready{self.stage=8}},
   10=>{
    let frontier=if self.frozen==Some(GuestPollFrozenKind::Input){FrozenReceiptFrontier::Unit}else{FrozenReceiptFrontier::Scalar};
    let result=self.capture.advance(&self.driver,self.history.as_mut().unwrap(),self.producer.as_mut().unwrap(),frontier,self.guest.unwrap(),cleanup);
    if self.retain(result,cleanup){self.capture=GuestFrozenHistoryCapture::default();self.guest=None;self.request=None;self.stage=26}
   },
   26=>{
    let mut ready=false;let mut input_receipt=None;
    let trace_input=self.trace.is_some()&&self.frozen==Some(GuestPollFrozenKind::Input);
    self.with_frozen(cleanup,|history,frozen,poll,cx|{GuestIssuedHistoryHome::with_journaled_receipt_step(history,&frozen,cx,|view,cx|{
     ready=poll.accept_journaled_step(&view,cx)?;
     if ready&&trace_input{input_receipt=poll.fixed_receipt().copied();}
     Ok(())
    })?;Ok(())});
    if let Some(call)=input_receipt{
     if let Some(sample)=self.trace.as_mut().and_then(|trace|poll_trace_sample(&mut trace.input_receipts)){
      eprintln!("[DEBUG] Input SRE1 settled: receipt {sample}, epoch {}, result {:?}, guest work {}",call.input.epoch,call.kind,call.work_units_spent);
     }
    }
    if ready{self.stage=27}
   },
   27=>{let producer=self.producer.as_mut().unwrap();let scalar=self.frozen!=Some(GuestPollFrozenKind::Input);let mut ready=false;let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{let mut captured=false;producer.with_native_borrow(cx,|loan,cx|{captured=attest_actor_failure_if_needed(loan,scalar,cx,|poll|poll.needs_actor_failure_reference(),|poll|&mut poll.actor_failure)?;Ok(())})?;if !captured{return Ok(())}producer.with_native(cx,|home,cx|{home.with_native(cx,|runtime,instance,_,poll,cx|{if poll.fixed_receipt().is_some(){poll.acknowledge_fixed_step(cx)?;return Ok(())}ready=poll.resume_journaled_step(runtime,instance,cx)?;Ok(())})?;Ok(())})?;Ok(())});self.retain(result,cleanup);if ready{self.stage=29}},
   29=>{
    match self.acknowledge(cleanup){
     Some(GuestPollSessionStatus::Refused)=>return GuestPollSessionStatus::Refused,
     Some(GuestPollSessionStatus::Ready)=>return GuestPollSessionStatus::Pending,
     _=>{}
    }
    if self.failure.is_some()||self.cleanup_failure.is_some(){return GuestPollSessionStatus::Refused}
    let scalar=self.frozen!=Some(GuestPollFrozenKind::Input);
    let producer=self.producer.as_mut().unwrap();
    let mut returned=false;
    let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{
     producer.with_native_borrow(cx,|mut guard,cx|{
      let mut identity=None;
      let mut bootstrap=None;
      guard.value_mut().with_native(cx,|_,_,_,poll,_|{
       returned=poll.header_returned.is_some()||poll.bootstrap_returned.is_some();
       if !scalar || poll.scalar_started {
        identity=poll.header_identity();
        bootstrap=poll.bootstrap_identity;
       }
       Ok(())
      })?;
      if !returned {
       if let Some(identity)=bootstrap{returned=attest_bootstrap_header_return(guard,identity,cx,|poll|&mut poll.bootstrap_returned)?;}
       else if let Some(identity)=identity{returned=attest_header_return(guard,scalar,identity,cx,|poll|&mut poll.header_returned)?;}
      }
      Ok(())
     })?;
     Ok(())
    });
    self.retain(result,cleanup);
    if self.failure.is_some() || self.cleanup_failure.is_some(){return GuestPollSessionStatus::Refused}
    if returned{self.stage=30}else{self.advance_native(input,cleanup);}
   },
   30=>{let history=self.history.as_mut().unwrap();let producer=self.producer.as_mut().unwrap();let mut ready=false;let result=self.driver.dispatch_operation_step(false,cleanup,|cx|{history.with_native_pair_borrow(producer,cx,|history,mut producer,cx|{producer.value_mut().with_native(cx,|_,_,_,poll,cx|{if poll.bootstrap_identity.is_some(){ready=GuestIssuedHistoryHome::receive_bootstrap_header_return_step(history,&mut poll.bootstrap_returned,cx)?;if ready{ready=poll.finish_bootstrap_header_return_step(cx)?;}}else{ready=GuestIssuedHistoryHome::receive_header_return_step(history,&mut poll.header_returned,cx)?;if ready{ready=poll.finish_header_return_step(cx)?;}}Ok(())})?;Ok(())})?;Ok(())});self.retain(result,cleanup);if ready{self.frozen=None;self.stage=0}},
   _=>{self.retain(Err(ValueError::invariant("Poll session changed its bounded frontier").into()),cleanup);complete=true;}
  }
  if self.failure.is_some()||self.cleanup_failure.is_some()||complete{GuestPollSessionStatus::Refused}else{GuestPollSessionStatus::Pending}
 }
 pub async fn drive_from<S:JsonWriteSource+?Sized>(&mut self,events:&S,mut receive:impl FnMut(&T,&mut R,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>)->GuestPollSessionStatus{
  loop{match self.open_step(){GuestPollSessionStatus::Ready=>break,GuestPollSessionStatus::Refused=>return GuestPollSessionStatus::Refused,_=>semio_framework_async::yield_once().await}}
  let input=match self.input(events){Ok(input)=>input,Err(error)=>{self.retain(Err(error.into()),false);return GuestPollSessionStatus::Refused}};let input=std::pin::pin!(input);
  loop{let status=self.drive_step(input.as_ref(),&mut receive);if matches!(status,GuestPollSessionStatus::Complete|GuestPollSessionStatus::Refused|GuestPollSessionStatus::Closed){return status}semio_framework_async::yield_once().await;}
 }
}

impl<T:NativeReceive<DslValue>+RetireOwned+Send+Sync+'static,R:GuestPollFailureReceiving> GuestPollSession<T,R>{
 pub fn close_step(&mut self,receive:&mut impl FnMut(&T,&mut R,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>)->GuestPollSessionStatus{
  if self.closed{return GuestPollSessionStatus::Closed}self.closing=true;self.driver.abandon_operation();
  if self.cleanup_failure.is_some(){return GuestPollSessionStatus::Refused}
  if self.failure.is_some() || !self.diagnostic.terminal_is_empty() {
   let failure=&mut *self.failure;let diagnostic=&mut self.diagnostic;
   let result=self.driver.dispatch_operation_step(false,true,|cx|{
    if diagnostic.terminal_is_empty() { diagnostic.capture_step(failure,cx)?; }
    else { diagnostic.close_on_source(cx)?; }
    Ok(())
   });
   self.retain(result,true);
   return if self.cleanup_failure.is_some(){GuestPollSessionStatus::Refused}else{GuestPollSessionStatus::Pending};
  }
  let result=self.close_slice(receive);self.retain(result,true);
  if self.failure.is_some()||self.cleanup_failure.is_some(){GuestPollSessionStatus::Refused}else if self.closed{GuestPollSessionStatus::Closed}else{GuestPollSessionStatus::Pending}
 }
 fn close_slice(&mut self,receive:&mut impl FnMut(&T,&mut R,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>)->Result<bool,GuestRelayAdmissionFailure>{
  if !self.driver.admit_root_step()?{return Ok(false)}
  match self.close_phase{
   0=>{
    if self.next_place.is_admitted()||self.next_result.is_some(){let place=&mut self.next_place;let result=&mut*self.next_result;let delivered=&mut self.delivered;self.driver.dispatch_operation_step(false,true,|cx|{settle_next_round(place,result,delivered,cx)?;Ok(())})?;return Ok(false)}
    if let Some(admission)=self.admission.as_mut(){let mut done=false;self.driver.dispatch_cleanup_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:(||{done=admission.close_step(cx)?;Ok(())})(),execution:None})?;if !done{return Ok(false)}}
    if let Some(state)=self.unopened.as_mut(){let mut done=false;self.driver.dispatch_cleanup_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:(||{done=state.close_unopened(cx)?;Ok(())})(),execution:None})?;if !done{return Ok(false)}}
    self.close_phase=if self.producer.is_some()&&self.history.is_some(){1}else{4};
   },
   1=>{match self.drive_inner(None,true,receive){
    GuestPollSessionStatus::Complete=>{
     let producer=self.producer.as_ref().unwrap();
     self.close_phase=if self.driver.pending_guest_subloan(producer)?.is_some(){4}else{2};
    },
    _=>{}
   }},
   2=>{let mut ready=false;let producer=self.producer.as_mut().unwrap();self.driver.dispatch_operation_step(false,true,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|_,_,_,poll,cx|{ready=poll.request_core_retirement_step(cx)?;Ok(())})?;Ok(())})?;Ok(())})?;if ready{self.stage=0;self.close_phase=3}},
   3=>{if self.drive_inner(None,true,receive)==GuestPollSessionStatus::Complete{let mut ready=false;let producer=self.producer.as_mut().unwrap();self.driver.dispatch_operation_step(false,true,|cx|{producer.with_native(cx,|home,cx|{home.with_native(cx,|_,_,_,poll,_|{ready=poll.core_quiescent&&poll.terminal_is_empty();Ok(())})?;Ok(())})?;Ok(())})?;if ready{self.close_phase=4}}},
   4=>{
    if self.producer.is_some(){let slot=&mut*self.producer;self.driver.dispatch_operation_step(false,true,|cx|{OriginalNativeLease::return_slot_step(slot,cx)?;Ok(())})?;return Ok(false)}
    if self.history.is_some(){let slot=&mut*self.history;self.driver.dispatch_operation_step(false,true,|cx|{OriginalNativeLease::return_slot_step(slot,cx)?;Ok(())})?;return Ok(false)}
    if self.fault.is_some(){let slot=&mut*self.fault;self.driver.dispatch_operation_step(false,true,|cx|{OriginalNativeLease::return_slot_step(slot,cx)?;Ok(())})?;return Ok(false)}
    self.close_phase=5;
   },
   5=>{if let Some(producer)=self.producer_id{let mut done=false;self.driver.dispatch_operation_step(true,true,|cx|{done=cx.close_original_native_recipient_step(producer)?;Ok(())})?;if !done{return Ok(false)}}self.close_phase=11},
   11=>{if let Some(fault)=self.fault_id{let mut done=false;self.driver.dispatch_operation_step(false,true,|cx|{done=cx.close_original_native_recipient_step(fault)?;Ok(())})?;if !done{return Ok(false)}}self.close_phase=6},
   6=>{if let Some(producer)=self.producer_id{let witness=&mut self.witness;self.driver.dispatch_return_birth_close_step(|cx|{if witness.is_none(){if let Some(proof)=cx.original_native_retired_witness(producer)?{if session_metadata(cx,std::mem::size_of_val(witness))?{*witness=Some(proof)}}}Ok(())})?;if self.witness.is_none(){return Ok(false)}}self.close_phase=7},
   7=>{if let(Some(history),Some(producer),Some(witness))=(self.history_id,self.producer_id,self.witness.as_ref()){let mut ready=false;self.driver.dispatch_escrow_reconciliation_step(true,|cx,mut issuer|{cx.with_original_native_root_step::<GuestIssuedHistoryHome>(history,|home,cx|{ready=issuer.retire_closed_history_step(home,producer,witness,cx)?;Ok(())})?;Ok(())})?;if !ready{return Ok(false)}}self.close_phase=8},
   8=>{if let Some(history)=self.history_id{let mut done=false;self.driver.dispatch_operation_step(false,true,|cx|{done=cx.close_original_native_recipient_step(history)?;Ok(())})?;if !done{return Ok(false)}}self.close_phase=9},
   9=>{
    let mut cleared=false;self.driver.dispatch_operation_step(false,true,|cx|{let bytes=std::mem::size_of_val(&self.admission)+std::mem::size_of_val(&self.unopened)+std::mem::size_of_val(&self.history_input)+std::mem::size_of_val(&self.fault_input)+std::mem::size_of_val(&self.guest)+std::mem::size_of_val(&self.request)+std::mem::size_of_val(&self.frozen)+std::mem::size_of_val(&self.witness)+std::mem::size_of_val(&self.limits)+std::mem::size_of::<u8>();if session_metadata(cx,bytes)?{self.admission.take();self.unopened.take();self.history_input.take();self.fault_input.take();self.guest=None;self.request=None;self.frozen=None;self.witness=None;self.limits=None;self.stage=0;cleared=true;}Ok(())})?;if cleared{self.close_phase=10}
   },
   10=>{if self.driver.close_root_step()?{self.closed=true;return Ok(true)}},
   _=>return Err(ValueError::invariant("Poll cleanup changed its retained frontier").into())
  }Ok(false)
 }
 pub async fn close(&mut self,mut receive:impl FnMut(&T,&mut R,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>)->GuestPollSessionStatus{
  let trace=std::env::var_os("RUST_BACKTRACE").is_some();
  let mut previous=None;
  let mut attempts=0usize;
  loop{
   let status=self.close_step(&mut receive);
   attempts=attempts.saturating_add(1);
   if trace && (previous!=Some(self.close_phase)||attempts.is_power_of_two()&&attempts>=64){
    previous=Some(self.close_phase);
    eprintln!("[DEBUG] poll close frontier: {}.{} in {attempts} steps, diagnostic empty {}, guest held {}, remaining {:?}, status {status:?}",self.close_phase,self.stage,self.diagnostic.terminal_is_empty(),self.guest.is_some(),self.driver.original_remaining());
   }
   if matches!(status,GuestPollSessionStatus::Closed|GuestPollSessionStatus::Refused){return status}
   semio_framework_async::yield_once().await
  }
 }
 pub fn terminal_is_empty(&self)->bool{self.next_result.is_none()&&self.next_place.terminal_is_empty()&&self.closed&&self.producer.is_none()&&self.history.is_none()&&self.fault.is_none()&&self.fault_input.is_none()&&self.admission.is_none()&&self.unopened.is_none()&&self.history_input.is_none()&&self.failure.is_none()&&self.cleanup_failure.is_none()&&self.diagnostic.terminal_is_empty()}
}
impl<T:NativeReceive<DslValue>+RetireOwned+Send+Sync+'static,R:GuestPollFailureReceiving> Drop for GuestPollSession<T,R>{
 fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Poll session requires actual Source retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.driver);ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.unopened);ManuallyDrop::drop(&mut self.producer);ManuallyDrop::drop(&mut self.history_input);ManuallyDrop::drop(&mut self.history);ManuallyDrop::drop(&mut self.fault_input);ManuallyDrop::drop(&mut self.fault);ManuallyDrop::drop(&mut self.failure);ManuallyDrop::drop(&mut self.cleanup_failure);}}}
}
fn session_metadata(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn receive_poll_turn_failure_step(diagnostic:&mut GuestRelayFaultHome,failure:&mut Option<OriginalTurnFailure>,cleanup:bool,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
 if cleanup{return diagnostic.capture_turn_failure_step(failure,cx).map(|_|(()))}
 match failure.take(){Some(error)=>Err(error),None=>Ok(())}
}
fn retire_scalar_failure_step(diagnostic:&mut GuestRelayFaultHome,failure:&mut Option<OwnedOriginalCallProgress>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
 let Some(progress)=failure.take()else{return Ok(())};
 if let Err(error)=progress.result{
  let mut original=Some(error);
  let captured=diagnostic.capture_turn_failure_step(&mut original,cx);
  if let Some(error)=original{*failure=Some(OwnedOriginalCallProgress{result:Err(error),execution:progress.execution});}
  captured?;
 }
 Ok(())
}

fn settle_next_round(place:&mut OriginalCallerReturnPlace,result:&mut Option<Result<bool,OriginalTurnFailure>>,delivered:&mut bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if place.is_admitted(){place.settle_return(cx)?;return Ok(false)}if result.is_none(){return Ok(true)}let bytes=2*std::mem::size_of_val(result)+std::mem::size_of::<bool>();if !session_metadata(cx,bytes)?{return Ok(false)}match result.take().unwrap(){Ok(true)=>*delivered=false,Ok(false)=>{},Err(error)=>return Err(error)}Ok(true)
}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_actor::RetainedTurnInput;
 use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
 use semio_framework_value::RetainedCloneGrant;

 #[test]
 fn close_hands_back_the_original_scalar_cause_only_when_paid(){
  fn clock()->Option<u64>{Some(1000)}
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️failure-close.json")).unwrap();
  let grant=RetainedCloneGrant{maximum_items:128,maximum_copy_bytes:16384,maximum_capacity_bytes:16384,maximum_release_bytes:16384,maximum_depth:64};
  let close_grant=RetainedCloneGrant{maximum_copy_bytes:262144,maximum_capacity_bytes:262144,maximum_release_bytes:262144,..grant};
  let stock=RetainedCloneGrant{
   maximum_copy_bytes:grant.maximum_copy_bytes+close_grant.maximum_copy_bytes+OriginalJobFixtureCaller::birth_copy_bytes(),
   maximum_capacity_bytes:grant.maximum_capacity_bytes+close_grant.maximum_capacity_bytes,
   maximum_release_bytes:grant.maximum_release_bytes+close_grant.maximum_release_bytes,
   ..grant
  };
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(153),Generation(1),stock,clock,1_000_000,1_000_000);
  let receipt=caller.turn(grant,|cx|{
   cx.bind_original_source_epoch(1)?;
   let budget=OriginalGuestExecutionBudget{input:RetainedTurnInput::borrow_original(cx)?,fuel:16,deadline_us:cx.deadline_us()};
   Ok(OriginalGuestExecutionReceipt{budget,spent_fuel:fixture["spentFuel"].as_u64(),submitted:true})
  }).unwrap();
  let error=invalid("original scalar cause").with_work_refusal(ValueError::literal(ValueRefusalKind::WorkLimit,"original scalar work refusal")).unwrap();
  let mut failure=Some(OwnedOriginalCallProgress{result:Err(error),execution:Some(receipt)});
  let mut diagnostic=GuestRelayFaultHome::empty();
  let observed=|failure:&Option<OwnedOriginalCallProgress>,diagnostic:&GuestRelayFaultHome|serde_json::json!({"producerRetained":failure.is_some(),"diagnosticRetained":diagnostic.has_failure()});
  let mut epoch=2;
  caller.cancel().cancel_now();
  caller.turn(RetainedCloneGrant{maximum_copy_bytes:0,..grant},|cx|{
   cx.bind_original_source_epoch(epoch)?;
   cx.with_original_retirement(|cx|retire_scalar_failure_step(&mut diagnostic,&mut failure,cx))
  }).unwrap();
  assert_eq!(observed(&failure,&diagnostic),fixture["unpaid"]);
  assert_eq!(failure.as_ref().unwrap().execution,Some(receipt));
  epoch+=1;
  caller.turn(grant,|cx|{
   cx.bind_original_source_epoch(epoch)?;
   cx.with_original_retirement(|cx|{
    retire_scalar_failure_step(&mut diagnostic,&mut failure,cx)?;
    diagnostic.with_failure(cx,|failure,_|{
     let primary=failure.primary.as_ref().unwrap();
     assert_eq!(Some(primary.primary.message()),fixture["failure"].as_str());
     assert_eq!(Some(primary.work_refusal().unwrap().message.as_ref()),fixture["workRefusal"].as_str());
     Ok(())
    })
   })
  }).unwrap();
  assert_eq!(observed(&failure,&diagnostic),fixture["paid"]);
  for _ in 0..128{
   epoch+=1;
   if caller.turn(close_grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|diagnostic.close_on_source(cx))}).unwrap(){break;}
  }
  assert!(diagnostic.terminal_is_empty(),"diagnostic retirement stalled with stock {:?}, work {}, last effects {:?}",caller.remaining_grant(),caller.fuel_remaining(),caller.receipt());
  assert_eq!(observed(&failure,&diagnostic),fixture["retired"]);
  caller.retire(grant);
 }

 #[test]
 fn ordinary_poll_observation_transfers_the_typed_refusal(){
  fn clock()->Option<u64>{Some(1000)}
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️failure-close.json")).unwrap();
  let grant=RetainedCloneGrant{maximum_items:128,maximum_copy_bytes:16384,maximum_capacity_bytes:16384,maximum_release_bytes:16384,maximum_depth:64};
  let stock=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes+OriginalJobFixtureCaller::birth_copy_bytes(),..grant};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(154),Generation(1),stock,clock,1_000_000,1_000_000);
  let mut diagnostic=GuestRelayFaultHome::empty();
  for (index,kind) in [ValueRefusalKind::WorkLimit,ValueRefusalKind::Canceled].into_iter().enumerate(){
   let mut failure=Some(ValueError::literal(kind,"unframed original input").into());
   caller.turn(grant,|cx|{
    cx.bind_original_source_epoch(index as u64+1)?;
    let error=receive_poll_turn_failure_step(&mut diagnostic,&mut failure,false,cx).unwrap_err();
    let original=error.primary_value().unwrap();
    let observed=serde_json::json!({"kind":format!("{:?}",original.kind),"message":original.message,"producerRetained":failure.is_some()});
    assert_eq!(observed,fixture["ordinaryRefusals"][index]);
    Ok(())
   }).unwrap();
  }
  assert!(diagnostic.terminal_is_empty());
  caller.retire(grant);
 }
}
