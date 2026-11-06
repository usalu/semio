//! 🌱️ Generic native Home admission retains all partial owners before any VM startup.
use super::*;
use semio_framework_job::{OriginalNativeLease,OriginalNativeRetirement,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_plugin_host_wake::GuestRelayAdmissionCall;
use std::mem::size_of;
/// 🧳️ The caller owns this admission and its receiving slot across every suspended startup turn.
pub struct NativeHomeAdmission<T:NativeInstanceRetirement>{actor:RuntimeActorId,input:Option<NativeInstanceHome<T>>,lease:Option<OriginalNativeLease<NativeInstanceHome<T>>>,ready:bool,requires_execution:bool,closing:bool}
impl<T:NativeInstanceRetirement> NativeHomeAdmission<T>{
 pub fn new(actor:RuntimeActorId,origin:GuestRelayNativeOrigin,state:T)->Result<Self,(T,ValueError)>{if origin.cache_identity==0{return Err((state,refused()))}Ok(Self{actor,input:Some(NativeInstanceHome::opening(origin,state)),lease:None,ready:false,requires_execution:false,closing:false})}
 pub fn requires_execution(&self)->bool{self.requires_execution}
 /// ▶️ One paid callback births the native recipient before advancing its actual guest startup.
 pub fn advance(&mut self,execution:Option<semio_framework_actor::OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->GuestRelayAdmissionCall{
  let mut call=GuestRelayAdmissionCall{result:Ok(()),execution:execution.map(|budget|semio_framework_actor::OriginalGuestExecutionReceipt{budget,spent_fuel:Some(0),submitted:false})};
  call.result=(||{if self.closing||self.requires_execution!=execution.is_some(){return Err(refused().into())}if self.lease.is_none(){cx.admit_original_native(&mut self.input,&mut self.lease)?;return Ok(())}let mut result=None;let borrowed=self.lease.as_mut().unwrap().with_native(cx,|home,cx|{let next=home.open_step(self.actor,execution,cx);self.requires_execution=home.opening_requires_execution();let ready=home.instance_is_open();result=Some((next,ready));Ok(())})?;if borrowed{let(next,ready)=result.ok_or_else(refused)?;call.execution=next.execution;next.result?;if ready&&pay(cx,size_of::<bool>(),false)?{self.ready=true}}Ok(())})();call
 }
 /// 📤️ Transfers only an opened Home into its caller-owned lease slot.
 pub fn take_step(&mut self,out:&mut Option<OriginalNativeLease<NativeInstanceHome<T>>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if out.is_some(){return Err(refused().into())}if !self.ready||self.input.is_some()||self.lease.is_none(){return Ok(false)}if !pay(cx,size_of::<Option<OriginalNativeLease<NativeInstanceHome<T>>>>()*2+size_of::<bool>(),false)?{return Ok(false)}*out=self.lease.take();self.ready=false;Ok(true)}
 /// 🧹️ Returns partial admission without losing its actual Source owner.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.closing{if !pay(cx,size_of::<bool>(),true)?{return Ok(false)}self.closing=true}if let Some(lease)=self.lease.as_mut(){if !lease.return_step(cx)?||!pay(cx,size_of::<Option<OriginalNativeLease<NativeInstanceHome<T>>>>(),true)?{return Ok(false)}self.lease.take();return Ok(false)}if let Some(home)=self.input.as_mut(){if !home.close_on_source(cx)?||!pay(cx,size_of::<Option<NativeInstanceHome<T>>>(),true)?{return Ok(false)}self.input.take();return Ok(false)}if self.ready||self.requires_execution{if !pay(cx,2*size_of::<bool>(),true)?{return Ok(false)}self.ready=false;self.requires_execution=false;return Ok(false)}Ok(true)}
 pub fn terminal_is_empty(&self)->bool{self.input.is_none()&&self.lease.is_none()&&!self.ready&&!self.requires_execution}
}
impl<T:NativeInstanceRetirement> Drop for NativeHomeAdmission<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native Home admission requires Source-funded transfer or retirement")}}
fn pay(cx:&mut StepContext<'_>,bytes:usize,retirement:bool)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if (if retirement{cx.retirement_should_yield()}else{cx.should_yield()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native Home admission changed its actual Source phase")}
