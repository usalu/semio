//! 🎫️ Caller-owned admission prepays the operation transfer on its actual finite Source.
use super::*;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_plugin_host_wake::GuestRelayAdmissionFailure;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Unreserved,ReceiptPending,Ready,Consumed,Closed}
/// 🧳️ This value owns the driver and both failure slots before any async work begins.
pub struct GuestRelayOperationAdmission{driver:Option<GuestRelayWakeDriver>,actor:RuntimeActorId,origin:GuestRelayNativeOrigin,phase:Phase,failure:ManuallyDrop<Option<GuestRelayAdmissionFailure>>,cleanup_failure:ManuallyDrop<Option<GuestRelayAdmissionFailure>>}
impl GuestRelayOperationAdmission{
 pub fn new(driver:GuestRelayWakeDriver,actor:RuntimeActorId,origin:GuestRelayNativeOrigin)->Self{Self{driver:Some(driver),actor,origin,phase:Phase::Unreserved,failure:ManuallyDrop::new(None),cleanup_failure:ManuallyDrop::new(None)}}
 /// 🎟️ The actual Source pays the future control and failure slots; the driver stays borrowed until settlement ends.
 pub fn reserve_step(&mut self)->GuestRelayOperationStatus{let Some(driver)=self.driver.as_ref()else{return if self.phase==Phase::Consumed{GuestRelayOperationStatus::Complete}else{GuestRelayOperationStatus::Failed}};reserve(driver,self.origin,&mut self.phase,&mut*self.failure)}
 /// 📤️ Only a settled paid reservation can move the driver into the caller-owned operation slot.
 pub fn take_step(&mut self,out:&mut Option<GuestRelayOperation>)->GuestRelayOperationStatus{if self.failure.is_some(){return GuestRelayOperationStatus::Failed}if self.phase!=Phase::Ready{return GuestRelayOperationStatus::Pending}if out.is_some(){*self.failure=Some(refused().into());if let Some(driver)=self.driver.as_ref(){driver.abandon_operation()}return GuestRelayOperationStatus::Failed}let Some(driver)=self.driver.take()else{*self.failure=Some(refused().into());return GuestRelayOperationStatus::Failed};*out=Some(GuestRelayOperation::from_reserved(driver,self.actor,self.origin));self.phase=Phase::Consumed;GuestRelayOperationStatus::Complete}
 /// ⏳️ The caller owns this admission and output before the first suspension.
 pub async fn admit(&mut self,out:&mut Option<GuestRelayOperation>)->GuestRelayOperationStatus{
  let Some(driver)=self.driver.as_ref()else{return if self.phase==Phase::Consumed{GuestRelayOperationStatus::Complete}else{GuestRelayOperationStatus::Failed}};let mut guard=Guard{driver,armed:true};
  loop{match reserve(driver,self.origin,&mut self.phase,&mut*self.failure){GuestRelayOperationStatus::Pending=>semio_framework_async::yield_once().await,GuestRelayOperationStatus::Failed=>return GuestRelayOperationStatus::Failed,GuestRelayOperationStatus::Complete=>break}}
  guard.armed=false;drop(guard);self.take_step(out)
 }
 /// 👁️ Borrows one complete failure only within a paid receiving callback.
 pub fn with_failure_step(&self,cleanup:bool,cx:&mut StepContext<'_>,receive:impl FnOnce(&GuestRelayAdmissionFailure,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{let slot=if cleanup{&*self.cleanup_failure}else{&*self.failure};let Some(failure)=slot.as_ref()else{return Ok(false)};let grant=cx.retained_grant();let bytes=size_of::<&GuestRelayAdmissionFailure>();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);receive(failure,cx)?;Ok(true)}
 /// 🧯️ A paid fault home receives either whole admission or independent cleanup failure.
 pub fn capture_failure_step(&mut self,cleanup:bool,home:&mut GuestRelayFaultHome,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let slot=if cleanup{&mut*self.cleanup_failure}else{&mut*self.failure};if slot.is_none(){return Ok(true)}home.capture_step(slot,cx)}
 /// ♻️ A refused or abandoned admission closes the same retained driver after its failure is preserved.
 pub async fn close(&mut self)->GuestRelayOperationStatus{if self.phase==Phase::Consumed||self.phase==Phase::Closed{return GuestRelayOperationStatus::Complete}if self.failure.is_some()||self.cleanup_failure.is_some(){return GuestRelayOperationStatus::Failed}let Some(driver)=self.driver.as_ref()else{return GuestRelayOperationStatus::Failed};match driver.close_operation().await{Ok(())=>{self.phase=Phase::Closed;GuestRelayOperationStatus::Complete},Err(error)=>{*self.cleanup_failure=Some(error);GuestRelayOperationStatus::Failed}}}
 pub fn terminal_is_empty(&self)->bool{matches!(self.phase,Phase::Consumed|Phase::Closed)&&self.failure.is_none()&&self.cleanup_failure.is_none()}
}
impl Drop for GuestRelayOperationAdmission{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"operation admission requires transfer or Source closure");if self.failure.is_none(){unsafe{ManuallyDrop::drop(&mut self.failure)}}if self.cleanup_failure.is_none(){unsafe{ManuallyDrop::drop(&mut self.cleanup_failure)}}}}
fn reserve(driver:&GuestRelayWakeDriver,origin:GuestRelayNativeOrigin,phase:&mut Phase,failure:&mut Option<GuestRelayAdmissionFailure>)->GuestRelayOperationStatus{
 if failure.is_some(){return GuestRelayOperationStatus::Failed}if *phase==Phase::Ready{return GuestRelayOperationStatus::Complete}if *phase!=Phase::Unreserved||origin.cache_identity==0||driver.operation_abandoned(){*failure=Some(refused().into());driver.abandon_operation();return GuestRelayOperationStatus::Failed}
 let result=driver.dispatch_operation_step(false,false,|cx|{let bytes=size_of::<GuestRelayOperation>()+size_of::<Option<GuestRelayOperation>>()+size_of::<Phase>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);*phase=Phase::ReceiptPending;Ok(())});
 match result{Ok(true)if *phase==Phase::ReceiptPending=>{*phase=Phase::Ready;GuestRelayOperationStatus::Complete},Ok(_)=>GuestRelayOperationStatus::Pending,Err(error)=>{*failure=Some(error);driver.abandon_operation();GuestRelayOperationStatus::Failed}}
}
struct Guard<'a>{driver:&'a GuestRelayWakeDriver,armed:bool}
impl Drop for Guard<'_>{fn drop(&mut self){if self.armed{self.driver.abandon_operation()}}}
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"operation admission changed its paid transfer")}
