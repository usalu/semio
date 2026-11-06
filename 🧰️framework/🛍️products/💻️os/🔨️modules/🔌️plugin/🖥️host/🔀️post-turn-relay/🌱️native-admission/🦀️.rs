//! 🌱️ Canonical handles are born under the actual Source before their first Core allocation.
use super::*;
use crate::actor_failure::ActorFailureHome;
use semio_framework_job::{OriginalNativeLease,OriginalNativeRetirement,OriginalTurnFailure,StepContext};
use semio_framework_actor::OriginalGuestExecutionBudget;
use semio_plugin_host_wake::GuestRelayAdmissionCall;
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::size_of;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Home,Output,Fault,GuestFault,ActorFailure,Opening,Publish,Complete,Closing}
pub struct PluginInstanceHandleAdmission{actor:RuntimeActorId,origin:GuestRelayNativeOrigin,home_input:Option<GuestRelayNativeHome>,output_input:Option<GuestRelayOutputHome>,fault_input:Option<GuestRelayFaultHome>,guest_fault_input:Option<GuestRelayOutputHome>,actor_failure_input:Option<ActorFailureHome>,home:Option<OriginalNativeLease<GuestRelayNativeHome>>,output:Option<OriginalNativeLease<GuestRelayOutputHome>>,fault:Option<OriginalNativeLease<GuestRelayFaultHome>>,guest_fault:Option<OriginalNativeLease<GuestRelayOutputHome>>,actor_failure:Option<OriginalNativeLease<ActorFailureHome>>,ready:Option<PluginInstanceHandle>,phase:Phase,requires_execution:bool}
impl PluginInstanceHandleAdmission{
 pub fn new(actor:RuntimeActorId,origin:GuestRelayNativeOrigin)->Self{Self{actor,origin,home_input:Some(GuestRelayNativeHome::opening(origin,GuestRelayNativeState::default())),output_input:Some(GuestRelayOutputHome::empty()),fault_input:Some(GuestRelayFaultHome::empty()),guest_fault_input:Some(GuestRelayOutputHome::empty()),actor_failure_input:Some(ActorFailureHome::empty()),home:None,output:None,fault:None,guest_fault:None,actor_failure:None,ready:None,phase:Phase::Home,requires_execution:false}}
 pub fn requires_execution(&self)->bool{self.phase==Phase::Opening&&self.requires_execution}
 /// ▶️ Keeps every partially born owner in caller custody while a funded callback advances one physical phase.
 pub fn advance(&mut self,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->GuestRelayAdmissionCall{
  let mut receipt=execution.map(|budget|semio_framework_actor::OriginalGuestExecutionReceipt{budget,spent_fuel:Some(0),submitted:false});
  let result=(||{if self.phase==Phase::Closing{return Err(refused("closing native admission cannot restart a guest").into())}if self.requires_execution()!=execution.is_some(){return Err(refused("native admission execution differs from its actual cached phase").into())}
   match self.phase{
    Phase::Home=>{if cx.admit_original_native(&mut self.home_input,&mut self.home)?&&metadata(cx,size_of::<Phase>())?{self.phase=Phase::Output;}},
    Phase::Output=>{if cx.admit_original_native(&mut self.output_input,&mut self.output)?&&metadata(cx,size_of::<Phase>())?{self.phase=Phase::Fault;}},
    Phase::Fault=>{if cx.admit_original_native(&mut self.fault_input,&mut self.fault)?&&metadata(cx,size_of::<Phase>())?{self.phase=Phase::GuestFault;}},
    Phase::GuestFault=>{if cx.admit_original_native(&mut self.guest_fault_input,&mut self.guest_fault)?&&metadata(cx,size_of::<Phase>())?{self.phase=Phase::ActorFailure;}},
    Phase::ActorFailure=>{if cx.admit_original_native(&mut self.actor_failure_input,&mut self.actor_failure)?&&metadata(cx,size_of::<Phase>())?{self.phase=Phase::Opening;}},
    Phase::Opening=>{let mut opened=false;let mut call=None;let borrowed=self.home.as_mut().ok_or_else(||refused("native opening lost its actual Source lease"))?.with_native(cx,|home,cx|{let progress=home.open_step(self.actor,execution,cx);self.requires_execution=home.opening_requires_execution();opened=home.instance_is_open();call=Some(progress);Ok(())})?;if borrowed{let call=call.ok_or_else(||refused("native opening did not return its actual physical result"))?;receipt=call.execution;call.result?;if opened&&metadata(cx,size_of::<Phase>())?{self.phase=Phase::Publish;}}},
    Phase::Publish=>{if metadata(cx,size_of::<PluginInstanceHandle>()+size_of::<Phase>())?{self.ready=Some(PluginInstanceHandle::from_admitted(self.actor,self.origin,self.home.take().unwrap(),self.output.take().unwrap(),self.fault.take().unwrap(),self.guest_fault.take().unwrap(),self.actor_failure.take().unwrap()));self.phase=Phase::Complete;}},
    Phase::Complete=>{},Phase::Closing=>unreachable!()
   }Ok(())
  })();GuestRelayAdmissionCall{result,execution:receipt}
 }
 /// 📤️ Transfers the already born handle into its caller-owned slot under the same actual Source.
 pub fn take_born_step(&mut self,out:&mut Option<PluginInstanceHandle>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if out.is_some(){return Err(refused("native handle receiver is occupied").into())}if self.phase!=Phase::Complete||self.home_input.is_some()||self.output_input.is_some()||self.fault_input.is_some()||self.guest_fault_input.is_some()||self.actor_failure_input.is_some()||self.home.is_some()||self.output.is_some()||self.fault.is_some()||self.guest_fault.is_some()||self.actor_failure.is_some()||self.ready.is_none(){return Ok(false)}if !metadata(cx,size_of::<Option<PluginInstanceHandle>>()*2)?{return Ok(false)}*out=self.ready.take();Ok(true)}
 /// 🧹️ Returns each admitted alias before the Source retires its actual native payload; unadmitted empty inputs stay owned.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.phase=Phase::Closing;if let Some(handle)=self.ready.as_mut(){if !handle.return_step(cx)?{return Ok(false)}if !retirement_metadata(cx,size_of::<Option<PluginInstanceHandle>>())?{return Ok(false)}self.ready.take();return Ok(false)}
  if self.home.is_some(){return return_native(&mut self.home,cx).map(|_|false)}if self.output.is_some(){return return_native(&mut self.output,cx).map(|_|false)}
  if self.actor_failure.is_some(){return return_native(&mut self.actor_failure,cx).map(|_|false)}
  if let Some(home)=self.actor_failure_input.as_mut(){if !home.close_on_source(cx)?{return Ok(false)}if !retirement_metadata(cx,size_of::<Option<ActorFailureHome>>())?{return Ok(false)}self.actor_failure_input.take();return Ok(false)}
  if self.guest_fault.is_some(){return return_native(&mut self.guest_fault,cx).map(|_|false)}
  if let Some(home)=self.guest_fault_input.as_mut(){if !home.close_on_source(cx)?{return Ok(false)}if !retirement_metadata(cx,size_of::<Option<GuestRelayOutputHome>>())?{return Ok(false)}self.guest_fault_input.take();return Ok(false)}
  if self.fault.is_some(){return return_native(&mut self.fault,cx).map(|_|false)}
  if let Some(home)=self.fault_input.as_mut(){if !home.close_on_source(cx)?{return Ok(false)}if !retirement_metadata(cx,size_of::<Option<GuestRelayFaultHome>>())?{return Ok(false)}self.fault_input.take();return Ok(false)}
  if let Some(home)=self.home_input.as_mut(){if !home.close_on_source(cx)?{return Ok(false)}if !retirement_metadata(cx,size_of::<Option<GuestRelayNativeHome>>())?{return Ok(false)}self.home_input.take();return Ok(false)}
  if let Some(home)=self.output_input.as_mut(){if !home.close_on_source(cx)?{return Ok(false)}if !retirement_metadata(cx,size_of::<Option<GuestRelayOutputHome>>())?{return Ok(false)}self.output_input.take();return Ok(false)}Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.home_input.is_none()&&self.output_input.is_none()&&self.fault_input.is_none()&&self.guest_fault_input.is_none()&&self.actor_failure_input.is_none()&&self.home.is_none()&&self.output.is_none()&&self.fault.is_none()&&self.guest_fault.is_none()&&self.actor_failure.is_none()&&self.ready.is_none()}
}
pub(super) fn return_native<T:OriginalNativeRetirement>(native:&mut Option<OriginalNativeLease<T>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{OriginalNativeLease::return_slot_step(native,cx)}
fn metadata(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{if cx.should_yield(){return Ok(false)}pay(cx,bytes)}
fn retirement_metadata(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{if cx.retirement_should_yield(){return Ok(false)}pay(cx,bytes)}
fn pay(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,message)}
