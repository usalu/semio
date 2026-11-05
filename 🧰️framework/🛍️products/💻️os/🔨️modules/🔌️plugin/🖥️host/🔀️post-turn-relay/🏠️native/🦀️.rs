//! 🏠️ The admitting Source owns the real instance; immutable compiled/runtime parents are borrowed from their original cache.
use super::*;
use semio_framework_job::{OriginalNativeRetirement,OriginalTurnFailure,StepContext};
use semio_framework_plugin_host_guest_runtime::CompiledHandle;
pub type GuestRelayNativeResolver=fn([u8;32],&mut dyn FnMut(&GuestRuntimes,&CompiledHandle)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>;
/// 🪪️ The selected immutable cache admits a borrowed lookup for its exact content identity.
#[derive(Clone,Copy)]
pub struct GuestRelayNativeOrigin{pub package_hash:[u8;32],pub resolve:GuestRelayNativeResolver}
/// 🧹️ A native receiver returns its real unit and response owners before the containing instance closes.
pub trait NativeInstanceRetirement:Send+'static{fn close_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,compiled:&CompiledHandle,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;fn close_unopened(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;fn terminal_is_empty(&self)->bool;}
/// 🧳️ One canonical Source root owns the mutable instance and its typed receiver state.
pub struct NativeInstanceHome<T:NativeInstanceRetirement>{origin:GuestRelayNativeOrigin,opening:Option<semio_framework_plugin_host_guest_runtime::GuestActorOpening>,instance:Option<GuestInstance>,state:T}
impl<T:NativeInstanceRetirement> NativeInstanceHome<T>{
 pub fn new(origin:GuestRelayNativeOrigin,instance:GuestInstance,state:T)->Self{Self{origin,opening:None,instance:Some(instance),state}}
 /// 🌱️ Admits the actual parent and receiver before phased startup can create Core resources.
 pub fn opening(origin:GuestRelayNativeOrigin,state:T)->Self{Self{origin,opening:None,instance:None,state}}
 pub fn instance_is_open(&self)->bool{self.instance.is_some()}
 pub fn opening_requires_execution(&self)->bool{self.opening.as_ref().is_some_and(|opening|!opening.ready()&&opening.requires_execution())}
 /// 🎟️ Keeps every actual partial startup owner in this original recipient before returning its receipt.
 pub fn open_step(&mut self,actor:RuntimeActorId,execution:Option<semio_framework_actor::OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->semio_framework_plugin_host_wake::GuestRelayAdmissionCall{
  use semio_framework_plugin_host_guest_runtime::OwnedActorStartupStatus;
  let mut call=semio_framework_plugin_host_wake::GuestRelayAdmissionCall{result:Ok(()),execution:execution.map(|budget|semio_framework_actor::OriginalGuestExecutionReceipt{budget,spent_fuel:Some(0),submitted:false})};
  if self.instance.is_some(){return call}let expected=self.origin.package_hash;let mut invoked=false;let available=(self.origin.resolve)(expected,&mut|runtime,compiled|{if invoked||compiled.package_hash!=expected{return Err(native_refusal("native opening requires its one exact immutable origin callback").into())}invoked=true;
   call.result=(||{if let Some(opening)=self.opening.as_mut(){if opening.ready(){if execution.is_some(){return Err(native_refusal("native startup transfer cannot consume a VM authorization").into())}let input=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;self.instance=runtime.take_original_actor_step(&mut self.opening,compiled,input,cx)?;}else{if execution.is_some(){call.execution=None;}let reply=runtime.advance_original_actor_step(opening,compiled,execution,cx)?;call.execution=reply.execution;reply.result.and_then(|status|match status{OwnedActorStartupStatus::Pending|OwnedActorStartupStatus::Ready=>Ok(()),OwnedActorStartupStatus::Cancelled=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"native startup observed its actual cancellation").into()),OwnedActorStartupStatus::Deadline=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"native startup reached its actual deadline").into()),OwnedActorStartupStatus::ChannelMismatch{..}=>Err(native_refusal("native startup returned a different component channel").into())})?;}}else{if execution.is_some(){return Err(native_refusal("native startup birth cannot consume a VM authorization").into())}self.opening=runtime.begin_original_actor_step(compiled,actor,cx)?;}Ok(())})();Ok(())});
  match available{Ok(true)if invoked=>{},Ok(false)if!invoked=>{},Ok(_)=>call.result=Err(native_refusal("native opening availability differs from its actual callback").into()),Err(error)=>call.result=Err(error)}call
 }
 /// 🔎️ Borrows the actual instance and its verified immutable parents without transferring native custody.
 pub fn with_instance(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&GuestRuntimes,&mut GuestInstance,&CompiledHandle,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{self.with_native(cx,|runtime,instance,compiled,state,cx|{if !state.terminal_is_empty(){return Err(native_refusal("native instance still retains its typed receiver state").into())}run(runtime,instance,compiled,cx)})}
 pub fn with_native(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&GuestRuntimes,&mut GuestInstance,&CompiledHandle,&mut T,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  let mut run=Some(run);let instance=self.instance.as_mut().ok_or_else(||native_refusal("native parent instance has physically closed"))?;let state=&mut self.state;let expected=self.origin.package_hash;let mut invoked=false;
  let available=(self.origin.resolve)(expected,&mut |runtime,compiled|{if compiled.package_hash!=expected{return Err(native_refusal("native immutable origin returned another compiled identity").into())}let run=run.take().ok_or_else(||native_refusal("native immutable origin invoked more than one borrowed callback"))?;invoked=true;run(runtime,instance,compiled,state,cx)})?;
  if available!=invoked{return Err(native_refusal("native immutable origin availability differs from its actual callback").into())}Ok(available)
 }
}
impl<T:NativeInstanceRetirement> OriginalNativeRetirement for NativeInstanceHome<T>{
 fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.opening.is_some(){let expected=self.origin.package_hash;let mut closed=false;let available=(self.origin.resolve)(expected,&mut|runtime,compiled|{if compiled.package_hash!=expected{return Err(native_refusal("native startup cleanup changed its immutable origin").into())}closed=runtime.close_original_actor_step(&mut self.opening,compiled,cx)?;Ok(())})?;return Ok(available&&closed&&self.terminal_is_empty())}if self.instance.is_none(){return self.state.close_unopened(cx)}let mut closed=false;
  if !self.with_native(cx,|runtime,instance,compiled,state,cx|{if !state.terminal_is_empty(){state.close_step(runtime,instance,compiled,cx)?;}else{closed=runtime.close_original_instance_step(instance,compiled,cx)?;}Ok(())})?{return Ok(false)}
  if closed{self.instance.take();}Ok(closed)
 }
 fn terminal_is_empty(&self)->bool{self.opening.is_none()&&self.instance.is_none()&&self.state.terminal_is_empty()}
}
impl<T:NativeInstanceRetirement> Drop for NativeInstanceHome<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native instance parent requires genuine Source-funded physical closure")}}
#[derive(Default)]
pub struct GuestRelayNativeState{pub(super) response:Option<GuestRelayNativeResponse>}
impl NativeInstanceRetirement for GuestRelayNativeState{
 fn close_step(&mut self,_:&GuestRuntimes,_:&mut GuestInstance,_:&CompiledHandle,_:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.response.is_some(){return Err(native_refusal("cold receiver still retains its whole guest response").into())}Ok(true)}
 fn close_unopened(&mut self,_:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.response.is_some(){return Err(native_refusal("unopened cold receiver cannot own a guest response").into())}Ok(true)}
 fn terminal_is_empty(&self)->bool{self.response.is_none()}
}
pub type GuestRelayNativeHome=NativeInstanceHome<GuestRelayNativeState>;
fn native_refusal(message:&'static str)->semio_framework_value::ValueError{semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,message)}
