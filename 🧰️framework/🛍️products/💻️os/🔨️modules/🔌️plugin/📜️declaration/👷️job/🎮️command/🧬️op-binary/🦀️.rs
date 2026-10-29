//! 🧬️ Any typed command whose schema derives `OpBinary` becomes an original command producer: the declared action bridge (or an already born command) is encoded into its exact sealed wire across funded turns.
use semio_framework::{Fault,FaultCode,FaultOrigin,action_bus::{RetainedToolWireInput,ToolWirePage,TOOL_WIRE_PAGE_BYTES}};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{DslValue,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind,retained_clone::RetainedCloneClose,retirement::RetireOwned};
use std::{mem::{ManuallyDrop,size_of},future::Future,task::Poll};
use super::{ArtifactOriginalCommandProducer,ArtifactOriginalCommandProducerFactory};
use protocol::OpBinary;
/// 📏️ The largest OpBinary wire a producer may declare; the registered tool contract narrows it at admission.
pub const RUNTIME_COMMAND_WIRE_MAXIMUM:usize=65_536;
/// 🌉️ The app's declared bridge from an action id and its arguments to its typed command.
pub type OriginalCommandBirth<C>=fn(&str,&mut Option<DslValue>,&mut Option<Fault>)->Result<C,Fault>;
const BIRTH:u8=0;const ENCODE:u8=1;const WIRE:u8=2;const PAGES:u8=3;const SEAL:u8=4;const RELEASE:u8=5;const READY:u8=6;const TAKEN:u8=7;
fn broken()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"command producer lost its original command, encoding or wire")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn original(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🧳️ The original command box, its one OpBinary encoding and the wire pages stay here until the typed source takes command and wire together.
struct OpBinaryProducer<C>{binding:OriginalSourceBinding,decoder:(u64,u64),body:usize,birth:Option<OriginalCommandBirth<C>>,command:ManuallyDrop<Option<Box<C>>>,bytes:ManuallyDrop<Option<Vec<u8>>>,input:ManuallyDrop<Option<RetainedToolWireInput>>,offset:usize,phase:u8,closing:bool,retirement:RetainedCloneClose}
impl<C:RetireOwned+OpBinary+Send+Sync+'static> OpBinaryProducer<C>{
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(broken().into())}Ok(())}
 fn length(&self)->Result<usize,ValueError>{self.bytes.as_ref().map(Vec::len).ok_or_else(broken)}
 fn release_quote(&self)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){self.retirement.demands(self.body)}else if self.bytes.is_some(){self.retirement.next_owner_demands::<Vec<u8>>(true,&None,self.body)}else{Ok(Default::default())}}
 fn close_quote(&self)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(header::<bool>())}if !self.retirement.is_empty(){return self.retirement.demands(self.body)}if self.bytes.is_some(){return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,self.body)}if self.command.is_some(){return self.retirement.next_owner_demands::<Box<C>>(true,&None,self.body)}if self.input.is_some(){return self.retirement.next_owner_demands::<RetainedToolWireInput>(true,&None,self.body)}Ok(Default::default())}
 fn step_demands(&self)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_quote()}Ok(match self.phase{
  BIRTH=>RetirementDemand{copy_bytes:2*size_of::<Option<Box<C>>>(),capacity_bytes:size_of::<C>(),depth:1,..Default::default()},
  ENCODE=>RetirementDemand{copy_bytes:size_of::<Option<Vec<u8>>>(),capacity_bytes:RUNTIME_COMMAND_WIRE_MAXIMUM,depth:1,..Default::default()},
  WIRE=>RetainedToolWireInput::original_birth_demands(self.length()?,RUNTIME_COMMAND_WIRE_MAXIMUM)?,
  PAGES=>RetirementDemand{copy_bytes:3*size_of::<ToolWirePage>()+(self.length()?-self.offset).min(TOOL_WIRE_PAGE_BYTES),depth:1,..Default::default()},
  SEAL=>header::<u8>(),RELEASE=>self.release_quote()?,_=>Default::default()})}
 fn release_bytes(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let d=self.release_quote()?;close_original_retained_step(cx,d,|g|{if !self.retirement.is_empty(){return self.retirement.select(self.body)?.step(g)}if self.bytes.is_some(){return Ok(self.retirement.begin_granted(&mut self.bytes,g)?.unwrap())}Ok(RetainedCloneStep::Complete(Default::default()))})?;Ok(())}
}
impl<C:RetireOwned+OpBinary+Send+Sync+'static> ArtifactOriginalCommandProducer<C> for OpBinaryProducer<C>{
 fn demands(&self,_action:&str,_args:&Option<DslValue>,_fault:&Option<Fault>,_body:usize)->Result<RetirementDemand,ValueError>{self.step_demands()}
 fn advance(&mut self,action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing||(fault.is_some()&&self.phase!=BIRTH){return Err(broken().into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"command production was cancelled").into())}if self.ready(){return Ok(true)}
  let d=self.step_demands()?;
  match self.phase{
   BIRTH=>{let birth=self.birth.ok_or_else(broken)?;let mut refused=None;let result=admit_original_construction(cx,&mut self.command,original(d),|_|match birth(action,args,fault){Ok(command)=>Ok((Box::new(command),progress(d))),Err(fault)=>{refused=Some(fault);Err(broken())}});if let Some(fault)=refused{return Err(fault.into())}result?;if self.command.is_some(){self.phase=ENCODE}},
   ENCODE=>{admit_original_construction(cx,&mut self.bytes,original(d),|_|{let mut bytes=OpBinary::encode_op(&**self.command.as_ref().ok_or_else(broken)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;if bytes.len()>RUNTIME_COMMAND_WIRE_MAXIMUM{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"command wire exceeds its declared maximum"))}bytes.shrink_to_fit();let retained=progress(RetirementDemand{capacity_bytes:bytes.capacity(),..d});Ok((bytes,retained))})?;if self.bytes.is_some(){self.phase=WIRE}},
   WIRE=>{let length=self.length()?;admit_original_construction(cx,&mut self.input,original(d),|_|RetainedToolWireInput::admit_original(length,RUNTIME_COMMAND_WIRE_MAXIMUM,grant(d))?.ok_or_else(broken))?;if self.input.is_some(){self.phase=PAGES}},
   PAGES=>{let length=self.length()?;if self.offset==length{self.phase=SEAL;return Ok(false)}let count=(length-self.offset).min(TOOL_WIRE_PAGE_BYTES);let mut done=None;admit_original_construction(cx,&mut done,original(d),|_|{let page=ToolWirePage::try_copy_from(&self.bytes.as_ref().ok_or_else(broken)?[self.offset..self.offset+count]).map_err(|_|broken())?;self.input.as_mut().ok_or_else(broken)?.admit_page(page).map_err(|_|broken())?;self.offset+=count;Ok(((),progress(d)))})?;},
   SEAL=>{let mut done=None;admit_original_construction(cx,&mut done,original(d),|_|{self.input.as_mut().ok_or_else(broken)?.seal().map_err(|_|broken())?;Ok(((),progress(d)))})?;self.phase=RELEASE},
   RELEASE=>{self.release_bytes(cx)?;if self.bytes.is_none()&&self.retirement.is_empty(){self.phase=READY}},
   _=>return Err(broken().into()),
  }
  Ok(self.ready())
 }
 fn ready(&self)->bool{!self.closing&&self.phase==READY&&self.command.is_some()&&self.input.as_ref().is_some_and(RetainedToolWireInput::original_is_sealed)&&self.bytes.is_none()&&self.retirement.is_empty()}
 fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(broken())}Ok(header::<(Option<Box<C>>,Option<Box<C>>,Option<RetainedToolWireInput>,Option<RetainedToolWireInput>,u8)>())}
 fn take_into(&mut self,command:&mut Option<Box<C>>,wire:&mut Option<RetainedToolWireInput>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if command.is_some()||wire.is_some(){return Err(broken().into())}let d=self.take_demands()?;let mut done=None;admit_original_construction(cx,&mut done,original(d),|_|{*command=self.command.take();*wire=self.input.take();self.phase=TAKEN;Ok(((),progress(d)))})?;Ok(self.phase==TAKEN)}
 fn close_demands(&self,_body:usize)->Result<RetirementDemand,ValueError>{self.close_quote()}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.terminal_is_empty(){return Ok(true)}
  if !self.closing{let d=header::<bool>();let mut done=None;close_original_retained_step(cx,d,|_|{self.closing=true;done=Some(());Ok(RetainedCloneStep::Progress(progress(d)))})?;return Ok(false)}
  let d=self.close_quote()?;
  close_original_retained_step(cx,d,|g|{if !self.retirement.is_empty(){return self.retirement.select(self.body)?.step(g)}if self.bytes.is_some(){return Ok(self.retirement.begin_granted(&mut self.bytes,g)?.unwrap())}if self.command.is_some(){return Ok(self.retirement.begin_granted(&mut self.command,g)?.unwrap())}if self.input.is_some(){return Ok(self.retirement.begin_granted(&mut self.input,g)?.unwrap())}Ok(RetainedCloneStep::Complete(Default::default()))})?;
  Ok(self.terminal_is_empty())
 }
 fn terminal_is_empty(&self)->bool{self.command.is_none()&&self.bytes.is_none()&&self.input.is_none()&&self.retirement.is_empty()}
}
impl<C> Drop for OpBinaryProducer<C>{fn drop(&mut self){assert!(std::thread::panicking()||(self.command.is_none()&&self.bytes.is_none()&&self.input.is_none()&&self.retirement.is_empty()),"command producer requires funded closure")}}
/// 🏭️ Births one producer into its slot: `make` supplies an already born command (runtime-originated), `birth` the declared action bridge; exactly one is expected.
pub fn admit_op_binary_producer<C:RetireOwned+OpBinary+Send+Sync+'static>(make:impl FnOnce()->Option<Box<C>>,birth:Option<OriginalCommandBirth<C>>,cx:&mut StepContext<'_>,output:&mut Option<Box<dyn ArtifactOriginalCommandProducer<C>>>)->Result<bool,OriginalTurnFailure>{
 let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let d=producer_birth_demands::<C>();
 admit_original_construction(cx,output,original(d),|control|{let command=make();if command.is_none()&&birth.is_none(){return Err(broken())}let phase=if command.is_some(){ENCODE}else{BIRTH};let decoder=(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(broken)?);Ok((Box::new(OpBinaryProducer::<C>{binding,decoder,body,birth,command:ManuallyDrop::new(command),bytes:ManuallyDrop::new(None),input:ManuallyDrop::new(None),offset:0,phase,closing:false,retirement:Default::default()})as Box<dyn ArtifactOriginalCommandProducer<C>>,progress(d)))})
}
fn producer_birth_demands<C:RetireOwned+OpBinary+Send+Sync+'static>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<OpBinaryProducer<C>>(),capacity_bytes:size_of::<OpBinaryProducer<C>>(),depth:1,..Default::default()}}
/// 🧬️ The generic factory every typed command gets from its declared action bridge, so no plugin hand-writes a producer.
pub struct OriginalActionCommandBridge<C>(pub OriginalCommandBirth<C>);
impl<C:RetireOwned+OpBinary+Send+Sync+'static> ArtifactOriginalCommandProducerFactory<C> for OriginalActionCommandBridge<C>{
 fn birth_demands(&self)->Result<RetirementDemand,ValueError>{Ok(producer_birth_demands::<C>())}
 fn admit(&self,cx:&mut StepContext<'_>,output:&mut Option<Box<dyn ArtifactOriginalCommandProducer<C>>>)->Result<bool,OriginalTurnFailure>{admit_op_binary_producer(||None,Some(self.0),cx,output)}
}
fn command_from_declared_action<A:crate::ArtifactApp>(action:&str,args:&mut Option<DslValue>,fault:&mut Option<Fault>)->Result<A::Command,Fault>{
 let mut future=std::pin::pin!(A::command_from_action(action,args,fault));
 match future.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())){Poll::Ready(result)=>result,Poll::Pending=>Err(Fault::new(FaultOrigin::Framework,FaultCode::new("action.command-suspended"),"a declared action-to-command bridge must complete without suspending"))}
}
/// 🧬️ The framework default producer of an app: its declared `command_from_action` bridge feeding the generic OpBinary wire encoder.
pub fn declared_action_command_bridge<A:crate::ArtifactApp>()->Option<&'static dyn ArtifactOriginalCommandProducerFactory<A::Command>>{Some(&OriginalActionCommandBridge::<A::Command>(command_from_declared_action::<A>))}
