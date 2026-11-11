//! 📦️ A caller-owned operation retains admission, native handle, driver and result before suspension.
use super::*;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_plugin_host_wake::GuestRelayAdmissionFailure;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GuestRelayOperationStatus{Pending,Complete,Failed}
/// 🧳️ The embedding owner keeps this value until the same finite Source has physically closed.
struct NextJob{admission:PluginInstanceHandleAdmission,handle:Option<PluginInstanceHandle>,result:Option<GuestRelayResult>,previous:semio_framework_job::OriginalNativeRecipientIdentity,retired:bool}
pub struct GuestRelayOperation{driver:GuestRelayWakeDriver,admission:PluginInstanceHandleAdmission,handle:Option<PluginInstanceHandle>,result:Option<GuestRelayResult>,next:Option<NextJob>,failure:ManuallyDrop<Option<GuestRelayAdmissionFailure>>,cleanup_failure:ManuallyDrop<Option<GuestRelayAdmissionFailure>>,closed:bool}
impl GuestRelayOperation{
 pub(super) fn from_reserved(driver:GuestRelayWakeDriver,actor:RuntimeActorId,origin:GuestRelayNativeOrigin)->Self{Self{driver,admission:PluginInstanceHandleAdmission::new(actor,origin),handle:None,result:None,next:None,failure:ManuallyDrop::new(None),cleanup_failure:ManuallyDrop::new(None),closed:false}}
 pub fn result_parts(&mut self)->(&GuestRelayWakeDriver,&mut Option<GuestRelayResult>){(&self.driver,&mut self.result)}
 /// 🔤️ Drives a JSON job while its source and all output slots stay in caller custody.
 pub async fn run_from(&mut self,kind:&'static str,input:&mut dyn GuestRelayJsonSource)->GuestRelayOperationStatus{if self.failure.is_some(){return GuestRelayOperationStatus::Failed}match self.run_json_inner(kind,input).await{Ok(())=>GuestRelayOperationStatus::Complete,Err(error)=>{*self.failure=Some(error);GuestRelayOperationStatus::Failed}}}
 async fn run_json_inner(&mut self,kind:&'static str,input:&mut dyn GuestRelayJsonSource)->Result<(),GuestRelayAdmissionFailure>{
  if self.closed||self.driver.operation_abandoned()||self.result.is_some(){return Err(refused().into())}let mut guard=AdmissionGuard{driver:&self.driver,armed:true};
  while self.handle.is_none(){let requires_execution=self.admission.requires_execution();self.driver.dispatch_admission_step(requires_execution,|execution,cx|self.admission.advance(execution,cx))?;self.driver.dispatch_operation_step(false,false,|cx|{self.admission.take_born_step(&mut self.handle,cx)?;Ok(())})?;if self.handle.is_none(){semio_framework_async::yield_once().await}}
  self.handle.as_mut().unwrap().run_from(kind,input,&self.driver,&mut self.result).await?;guard.armed=false;Ok(())
 }
 /// 📄️ Drives a binary job without detaching or copying its original provider outside paid turns.
 pub async fn run_bytes_from(&mut self,kind:&'static str,input:&mut dyn GuestRelayByteSource)->GuestRelayOperationStatus{if self.failure.is_some(){return GuestRelayOperationStatus::Failed}match self.run_bytes_inner(kind,input).await{Ok(())=>GuestRelayOperationStatus::Complete,Err(error)=>{*self.failure=Some(error);GuestRelayOperationStatus::Failed}}}
 async fn run_bytes_inner(&mut self,kind:&'static str,input:&mut dyn GuestRelayByteSource)->Result<(),GuestRelayAdmissionFailure>{
  if self.closed||self.driver.operation_abandoned()||self.result.is_some(){return Err(refused().into())}let mut guard=AdmissionGuard{driver:&self.driver,armed:true};
  while self.handle.is_none(){let requires_execution=self.admission.requires_execution();self.driver.dispatch_admission_step(requires_execution,|execution,cx|self.admission.advance(execution,cx))?;self.driver.dispatch_operation_step(false,false,|cx|{self.admission.take_born_step(&mut self.handle,cx)?;Ok(())})?;if self.handle.is_none(){semio_framework_async::yield_once().await}}
  self.handle.as_mut().unwrap().run_bytes_from(kind,input,&self.driver,&mut self.result).await?;guard.armed=false;Ok(())
 }
 /// ➡️ Runs the next binary hop under the same Source while the preceding output remains leased.
 pub async fn run_next_bytes(&mut self,actor:RuntimeActorId,origin:GuestRelayNativeOrigin,kind:&'static str)->GuestRelayOperationStatus{if self.failure.is_some(){return GuestRelayOperationStatus::Failed}match self.run_next(actor,origin,kind,None).await{Ok(())=>GuestRelayOperationStatus::Complete,Err(error)=>{*self.failure=Some(error);GuestRelayOperationStatus::Failed}}}
 /// 🔄️ Transforms a borrowed previous output into a JSON view on the same Source.
 pub async fn run_next_json(&mut self,actor:RuntimeActorId,origin:GuestRelayNativeOrigin,kind:&'static str,transform:&mut dyn GuestRelayJsonTransform)->GuestRelayOperationStatus{if self.failure.is_some(){return GuestRelayOperationStatus::Failed}match self.run_next(actor,origin,kind,Some(transform)).await{Ok(())=>GuestRelayOperationStatus::Complete,Err(error)=>{*self.failure=Some(error);GuestRelayOperationStatus::Failed}}}
 async fn run_next(&mut self,actor:RuntimeActorId,origin:GuestRelayNativeOrigin,kind:&'static str,mut transform:Option<&mut dyn GuestRelayJsonTransform>)->Result<(),GuestRelayAdmissionFailure>{
  if self.closed||self.driver.operation_abandoned()||origin.cache_identity==0||!matches!(&self.result,Some(GuestRelayResult::Success(_))){return Err(refused().into())}let mut guard=AdmissionGuard{driver:&self.driver,armed:true};
  while self.next.is_none(){self.driver.dispatch_operation_step(false,false,|cx|{if !pay_normal(cx,size_of::<NextJob>())?{return Ok(())}let previous=self.handle.as_ref().ok_or_else(refused)?.recipient_identity()?;self.next=Some(NextJob{admission:PluginInstanceHandleAdmission::new(actor,origin),handle:None,result:None,previous,retired:false});Ok(())})?;if self.next.is_none(){semio_framework_async::yield_once().await}}
  while self.handle.is_some(){self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{let handle=self.handle.as_mut().unwrap();if handle.return_step(cx)?&&pay(cx,size_of::<Option<PluginInstanceHandle>>())?{self.handle.take();}Ok(())}))?;if self.handle.is_some(){semio_framework_async::yield_once().await}}
  while !self.next.as_ref().unwrap().retired{self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{cx.close_original_native_step()?;let next=self.next.as_mut().unwrap();if let Some(proof)=cx.original_native_recipient_closed(next.previous)?{if !proof.proves(next.previous,cx){return Err(refused().into())}if pay(cx,size_of::<bool>())?{next.retired=true}}Ok(())}))?;if !self.next.as_ref().unwrap().retired{semio_framework_async::yield_once().await}}
  while self.next.as_ref().unwrap().handle.is_none(){let next=self.next.as_mut().unwrap();let requires_execution=next.admission.requires_execution();self.driver.dispatch_admission_step(requires_execution,|execution,cx|next.admission.advance(execution,cx))?;self.driver.dispatch_operation_step(false,false,|cx|{next.admission.take_born_step(&mut next.handle,cx)?;Ok(())})?;if next.handle.is_none(){self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{cx.close_original_native_step()?;Ok(())}))?;semio_framework_async::yield_once().await}}
  {let next=self.next.as_mut().unwrap();let Some(GuestRelayResult::Success(input))=self.result.as_mut()else{return Err(refused().into())};if let Some(transform)=transform.as_deref_mut(){let mut source=GuestRelayTransformedOutput{output:input,transform};next.handle.as_mut().unwrap().run_from(kind,&mut source,&self.driver,&mut next.result).await?;}else{next.handle.as_mut().unwrap().run_bytes_from(kind,input,&self.driver,&mut next.result).await?;}}
  while self.result.is_some(){self.driver.dispatch_operation_step(false,false,|cx|cx.with_original_retirement(|cx|{let result=self.result.as_mut().unwrap();if result.return_step(cx)?&&pay(cx,size_of::<Option<GuestRelayResult>>())?{self.result.take();}Ok(())}))?;if self.result.is_some(){semio_framework_async::yield_once().await}}
  loop{let mut moved=false;self.driver.dispatch_operation_step(false,false,|cx|{let next=self.next.as_mut().unwrap();if !pay_normal(cx,size_of::<Option<GuestRelayResult>>()+size_of::<Option<PluginInstanceHandle>>()+size_of::<PluginInstanceHandleAdmission>()+size_of::<Option<NextJob>>())?{return Ok(())}self.result=next.result.take();self.handle=next.handle.take();std::mem::swap(&mut self.admission,&mut next.admission);self.next.take();moved=true;Ok(())})?;if moved{break}semio_framework_async::yield_once().await}
  guard.armed=false;Ok(())
 }
 /// 🧹️ One paid slice returns result, handle and partial admission before the original Source closes.
 fn close_contents_step_inner(&mut self)->Result<bool,GuestRelayAdmissionFailure>{
  if self.closed{return Ok(true)}if !self.driver.admit_root_step().map_err(GuestRelayAdmissionFailure::from)?{return Ok(false)}
  if let Some(next)=self.next.as_mut(){
   if next.result.is_some(){self.driver.dispatch_operation_step(false,true,|cx|{let result=next.result.as_mut().unwrap();if result.return_step(cx)?&&pay(cx,size_of::<Option<GuestRelayResult>>())?{next.result.take();}Ok(())})?;return Ok(false)}
   if next.handle.is_some(){if !next.handle.as_mut().unwrap().settle_retirement_step(&self.driver)?{return Ok(false)}self.driver.dispatch_operation_step(false,true,|cx|{let handle=next.handle.as_mut().unwrap();if handle.return_step(cx)?&&pay(cx,size_of::<Option<PluginInstanceHandle>>())?{next.handle.take();}Ok(())})?;return Ok(false)}
   if !next.admission.terminal_is_empty(){self.driver.dispatch_cleanup_admission_step(false,|_,cx|semio_plugin_host_wake::GuestRelayAdmissionCall{result:next.admission.close_step(cx).map(|_|()),execution:None})?;return Ok(false)}
   self.driver.dispatch_operation_step(false,true,|cx|{if pay(cx,size_of::<Option<NextJob>>())?{self.next.take();}Ok(())})?;return Ok(false)
  }
  if self.result.is_some(){self.driver.dispatch_operation_step(false,true,|cx|{let result=self.result.as_mut().unwrap();if result.return_step(cx)?&&pay(cx,size_of::<Option<GuestRelayResult>>())?{self.result.take();}Ok(())})?;return Ok(false)}
  if self.handle.is_some(){if !self.handle.as_mut().unwrap().settle_retirement_step(&self.driver)?{return Ok(false)}self.driver.dispatch_operation_step(false,true,|cx|{let handle=self.handle.as_mut().unwrap();if handle.return_step(cx)?&&pay(cx,size_of::<Option<PluginInstanceHandle>>())?{self.handle.take();}Ok(())})?;return Ok(false)}
  if !self.admission.terminal_is_empty(){self.driver.dispatch_cleanup_admission_step(false,|_,cx|semio_plugin_host_wake::GuestRelayAdmissionCall{result:self.admission.close_step(cx).map(|_|()),execution:None})?;return Ok(false)}Ok(true)
 }

 /// 🧯️ Retains a verified ActorFailure capsule in the original caller-owned result slot.
 pub async fn recover_actor_failure(&mut self)->GuestRelayOperationStatus{
  if self.closed||self.failure.is_some()||self.cleanup_failure.is_some(){return GuestRelayOperationStatus::Failed}
  let result=if let Some(next)=self.next.as_mut(){match next.handle.as_mut(){Some(handle)=>handle.drive_actor_failure(&self.driver,&mut next.result).await,None=>Err(refused().into())}}else{match self.handle.as_mut(){Some(handle)=>handle.drive_actor_failure(&self.driver,&mut self.result).await,None=>Err(refused().into())}};
  match result{Ok(())=>GuestRelayOperationStatus::Complete,Err(error)=>{*self.cleanup_failure=Some(error);GuestRelayOperationStatus::Failed}}
 }
 /// 👁️ Borrows the next hop's retained result without moving it out of its parent.
 pub fn next_result_parts(&mut self)->Option<(&GuestRelayWakeDriver,&mut Option<GuestRelayResult>)>{let next=self.next.as_mut()?;Some((&self.driver,&mut next.result))}
 /// ♻️ Retires the complete original Source after its caller-owned recipients have returned.
 pub async fn close(&mut self)->GuestRelayOperationStatus{if self.failure.is_some()||self.cleanup_failure.is_some(){return GuestRelayOperationStatus::Failed}match self.close_inner().await{Ok(())=>GuestRelayOperationStatus::Complete,Err(error)=>{*self.cleanup_failure=Some(error);GuestRelayOperationStatus::Failed}}}
 async fn close_inner(&mut self)->Result<(),GuestRelayAdmissionFailure>{while !self.close_contents_step_inner()?{semio_framework_async::yield_once().await}self.driver.close_operation().await?;self.closed=true;Ok(())}
 /// 🧯️ A failed operation is captured, read and retired under its own driver before the owner closes.
 pub async fn retire_refusal(&mut self,read:&mut dyn FnMut(&GuestRelayAdmissionFailure,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<(),GuestRelayAdmissionFailure>{super::operation_admission::retire_failure(&self.driver,&mut*self.failure,read).await?;super::operation_admission::retire_failure(&self.driver,&mut*self.cleanup_failure,read).await}
 /// 👁️ Borrows one complete failure only within a paid receiving callback.
 pub fn with_failure_step(&self,cleanup:bool,cx:&mut StepContext<'_>,receive:impl FnOnce(&GuestRelayAdmissionFailure,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{let slot=if cleanup{&*self.cleanup_failure}else{&*self.failure};let Some(failure)=slot.as_ref()else{return Ok(false)};if !pay(cx,size_of::<&GuestRelayAdmissionFailure>())?{return Ok(false)}receive(failure,cx)?;Ok(true)}
 /// 🧯️ A paid receiver captures either complete body failure or independent cleanup failure.
 pub fn capture_failure_step(&mut self,cleanup:bool,home:&mut GuestRelayFaultHome,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let slot=if cleanup{&mut*self.cleanup_failure}else{&mut*self.failure};if slot.is_none(){return Ok(true)}home.capture_step(slot,cx)}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.result.is_none()&&self.handle.is_none()&&self.next.is_none()&&self.failure.is_none()&&self.cleanup_failure.is_none()&&self.admission.terminal_is_empty()}
}
impl Drop for GuestRelayOperation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"operation owner requires complete Source retirement");if self.failure.is_none(){unsafe{ManuallyDrop::drop(&mut self.failure)}}if self.cleanup_failure.is_none(){unsafe{ManuallyDrop::drop(&mut self.cleanup_failure)}}}}
struct AdmissionGuard<'a>{driver:&'a GuestRelayWakeDriver,armed:bool}
impl Drop for AdmissionGuard<'_>{fn drop(&mut self){if self.armed{self.driver.abandon_operation()}}}
fn pay_normal(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn pay(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"operation owner cannot restart or replace its result")}
