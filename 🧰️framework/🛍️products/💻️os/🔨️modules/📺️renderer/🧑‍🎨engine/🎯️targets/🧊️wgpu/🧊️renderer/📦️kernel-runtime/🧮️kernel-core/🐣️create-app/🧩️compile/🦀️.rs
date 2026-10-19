//! 🧩️ One sealed native compiler future borrows stable original inputs and retains its result until paid transfer.
use super::{SLOTS,Slot};
use super::super::super::{OriginalKernelTaskProducer,OriginalKernelTaskInput};
use semio_framework_actor::PackageHash;
use semio_framework_plugin_host::{GuestRuntime,GuestRuntimes,PackageRef,CompiledHandle};
use semio_framework_plugin_guest_contract::PluginHostError;
use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,OriginalSourceBinding,OriginalConstructionDemand,OriginalTurnFailure,StepContext,OriginalSuppliedJobBytes,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,admit_original_construction,close_original_retained_step};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,RetainedCloneStep,RetirementDemand};
use std::{future::Future,mem::{ManuallyDrop,size_of},pin::Pin,sync::{Arc,atomic::Ordering},task::{Context,Poll,RawWaker,RawWakerVTable,Waker}};

const _:()=assert!(super::CAPACITY<=64);
type CompileOutput=Result<CompiledHandle,PluginHostError>;
pub(super) struct Inputs{runtime:Option<Arc<GuestRuntimes>>,package:Option<PackageRef>,bytes:Option<OriginalSuppliedJobBytes>}
impl Inputs{pub(super) const fn empty()->Self{Self{runtime:None,package:None,bytes:None}}pub(super) fn terminal(&self)->bool{self.runtime.is_none()&&self.package.is_none()&&self.bytes.is_none()}}
pub(super) struct Output{result:ManuallyDrop<Option<CompileOutput>>,panic:ManuallyDrop<Option<Box<dyn std::any::Any+Send>>>,prepared:bool}
impl Output{pub(super) const fn empty()->Self{Self{result:ManuallyDrop::new(None),panic:ManuallyDrop::new(None),prepared:false}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(super) enum Status{Pending,Ready,BackendRefusal,Retiring,Closed}
fn refusal(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,text)}

#[derive(Clone,Copy)]
struct Ticket{index:usize,generation:u64}
impl Ticket{
 fn slot(self)->&'static Slot{let slot=&SLOTS[self.index];assert_eq!(slot.generation.load(Ordering::Acquire),self.generation,"compiler ticket retains its exact original request");slot}
 fn inputs(self)->&'static Inputs{unsafe{&*self.slot().compiler_inputs.get()}}
 fn output(self)->&'static Output{unsafe{&*self.slot().compiler_output.get()}}
 fn inputs_mut(self)->&'static mut Inputs{unsafe{&mut*self.slot().compiler_inputs.get()}}
 fn output_mut(self)->&'static mut Output{unsafe{&mut*self.slot().compiler_output.get()}}
 fn return_runtime(self,witness:&Arc<GuestRuntimes>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{let input=self.inputs();let Some(runtime)=input.runtime.as_ref()else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};if !Arc::ptr_eq(runtime,witness){return Err(refusal("compiler runtime alias requires its actual still-live supplier").into())}let demand=RetirementDemand{copy_bytes:size_of::<Option<Arc<GuestRuntimes>>>(),depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{self.inputs_mut().runtime.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})}
 fn close_input(self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.output().panic.is_some()||(self.output().result.is_some()&&!self.output().prepared){return Err(refusal("compiler result retains its defining backend retirement owner").into())}let input=self.inputs_mut();if input.runtime.is_some(){return Err(refusal("compiler runtime alias awaits its actual supplier witness").into())}
  if let Some(package)=input.package.as_ref(){let demand=RetirementDemand{copy_bytes:size_of::<Option<PackageRef>>(),release_bytes:package.package.0.capacity(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{input.package.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})}
  if input.bytes.is_some(){let demand=RetirementDemand{copy_bytes:size_of::<Option<OriginalSuppliedJobBytes>>(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{input.bytes.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})}Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
 }
 fn raw(self)->RawWaker{RawWaker::new(std::ptr::without_provenance(((self.generation as usize)<<6)|self.index),&WAKE)}
}
unsafe fn clone_wake(data:*const())->RawWaker{RawWaker::new(data,&WAKE)}
unsafe fn notify(data:*const()){let word=data.addr();let(index,generation)=(word&63,(word>>6)as u64);let slot=&SLOTS[index];if slot.generation.load(Ordering::Acquire)==generation{slot.compiler_wake.store(true,Ordering::Release);super::wake_compiler();}}
unsafe fn drop_wake(_: *const()){}
static WAKE:RawWakerVTable=RawWakerVTable::new(clone_wake,notify,notify,drop_wake);

struct InputTicket{ticket:Ticket}
impl OriginalKernelTaskInput for InputTicket{fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.ticket.close_input(cx)}fn terminal_is_empty(&self)->bool{self.ticket.inputs().terminal()&&(self.ticket.output().result.is_none()||self.ticket.output().prepared)&&self.ticket.output().panic.is_none()}}
struct CompileJob<F>{future:ManuallyDrop<Option<F>>,ticket:Ticket,finished:bool,started:bool,poisoned:bool,closing:bool}
impl<F:Future<Output=CompileOutput>+Send+'static> CompileJob<F>{
 fn poll(&mut self,cx:&mut StepContext<'_>)->Result<Status,OriginalTurnFailure>{
  if self.finished{return Ok(self.status())}if self.closing{return Err(refusal("pending compiler backend keeps its original future during retirement").into())}if cx.is_cancelled()||cx.should_yield(){return Ok(Status::Pending)}cx.consume_fuel(1);let waker=unsafe{Waker::from_raw(self.ticket.raw())};let mut context=Context::from_waker(&waker);self.started=true;let future=unsafe{Pin::new_unchecked(self.future.as_mut().unwrap())};
  match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||future.poll(&mut context))){Ok(Poll::Pending)=>Ok(Status::Pending),Ok(Poll::Ready(result))=>{*self.ticket.output_mut().result=Some(result);self.finished=true;Ok(self.status())},Err(payload)=>{*self.ticket.output_mut().panic=Some(payload);self.poisoned=true;self.finished=true;Ok(Status::BackendRefusal)}}
 } fn status(&self)->Status{if self.ticket.output().panic.is_some(){return Status::BackendRefusal}match self.ticket.output().result.as_ref(){Some(Ok(_))=>Status::Ready,Some(Err(_))=>Status::BackendRefusal,None if self.finished=>Status::Retiring,None=>Status::Pending}}
}
impl<F:Future<Output=CompileOutput>+Send+'static> InteractiveJob for CompileJob<F>{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{self.poll(cx)?;Ok(None)}
 fn borrow_outcome<'a>(&'a self,_descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{Err(refusal("native compiler transfers its typed host result rather than a guest outcome"))}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.closing=true;if self.future.is_some(){if self.poisoned||(!self.finished&&self.started){return Err(refusal("pending or poisoned compiler future requires its defining backend close before release").into())}let demand=RetirementDemand{copy_bytes:size_of::<Option<F>>(),depth:1,..Default::default()};return close_original_retained_step(cx,demand,|_|{unsafe{ManuallyDrop::drop(&mut self.future)}self.future=ManuallyDrop::new(None);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})}self.ticket.close_input(cx)
 }
 fn terminal_is_empty(&self)->bool{self.future.is_none()&&self.ticket.inputs().terminal()&&(self.ticket.output().result.is_none()||self.ticket.output().prepared)&&self.ticket.output().panic.is_none()}
}

trait Task:Send{fn poll(&mut self,cx:&mut StepContext<'_>)->Result<Status,OriginalTurnFailure>;fn status(&self)->Status;fn prepare(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;fn close(&mut self,witness:Option<&Arc<GuestRuntimes>>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;fn terminal(&self)->bool;fn bytes(&self)->usize;fn can_close(&self)->bool;}
struct Frame<F:Future<Output=CompileOutput>+Send+'static>{producer:OriginalKernelTaskProducer<InputTicket,CompileJob<F>>,ticket:Ticket,binding:OriginalSourceBinding,factory:fn(&'static GuestRuntimes,&'static PackageRef,&'static[u8])->F}
impl<F:Future<Output=CompileOutput>+Send+'static> Task for Frame<F>{
 fn poll(&mut self,cx:&mut StepContext<'_>)->Result<Status,OriginalTurnFailure>{if !self.producer.committed(){let factory=self.factory;let before=(cx.retained_progress(),cx.fuel_remaining());self.producer.admit(cx,OriginalConstructionDemand{items:1,copy_bytes:size_of::<CompileJob<F>>(),capacity_bytes:0,release_bytes:0,depth:1},|input,_|{let fields=input.ticket.inputs();let future=factory(fields.runtime.as_ref().unwrap(),fields.package.as_ref().unwrap(),fields.bytes.as_ref().unwrap().bytes());(CompileJob{future:ManuallyDrop::new(Some(future)),ticket:input.ticket,finished:false,started:false,poisoned:false,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<CompileJob<F>>(),..Default::default()})})?;if before==(cx.retained_progress(),cx.fuel_remaining()){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"compiler ingress has insufficient unchanged original stock or work").into())}self.ticket.slot().compiler_wake.store(true,Ordering::Release);return Ok(Status::Pending)}self.producer.run(cx,|job,cx|job.poll(cx))}
 fn status(&self)->Status{self.producer.candidate().map_or(Status::Pending,CompileJob::status)}
 fn prepare(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.producer.run(cx,|job,cx|{if job.status()!=Status::Ready{return Ok(false)}let mut header=None;admit_original_construction(cx,&mut header,OriginalConstructionDemand{items:1,copy_bytes:size_of::<CompileOutput>()+size_of::<bool>(),capacity_bytes:0,release_bytes:0,depth:1},|_|Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<CompileOutput>()+size_of::<bool>(),..Default::default()})))})}
 fn close(&mut self,witness:Option<&Arc<GuestRuntimes>>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{if !self.binding.matches(cx){return Err(refusal("compiler frame close refused another complete original source").into())}self.producer.begin_close();let inputs_unborrowed=self.producer.candidate().is_none_or(|job|job.future.is_none());if inputs_unborrowed&&(self.ticket.output().result.is_none()||self.ticket.output().prepared)&&self.ticket.output().panic.is_none()&&self.ticket.inputs().runtime.is_some(){if let Some(witness)=witness{return self.ticket.return_runtime(witness,cx)}}self.producer.close_step(cx)}
 fn terminal(&self)->bool{self.producer.terminal_is_empty()}
 fn bytes(&self)->usize{size_of::<Frame<F>>()}
 fn can_close(&self)->bool{self.producer.terminal_is_empty()||self.producer.candidate().is_some_and(|job|!job.poisoned&&(!job.started||job.finished)&&job.future.is_some())||((self.ticket.output().result.is_none()||self.ticket.output().prepared)&&self.ticket.output().panic.is_none()&&self.ticket.inputs().runtime.is_none())}
}

pub(super) struct CompilerTask{holder:ManuallyDrop<Option<Box<dyn Task>>>,ticket:Ticket,runtime_address:usize,hash:PackageHash,pub(super) committed:bool}
impl CompilerTask{
 pub(super) fn matches(&self,runtime:&Arc<GuestRuntimes>,hash:PackageHash,bytes:&Option<OriginalSuppliedJobBytes>)->bool{self.runtime_address==Arc::as_ptr(runtime).addr()&&self.hash==hash&&bytes.is_none()}
 pub(super) fn poll(&mut self,cx:&mut StepContext<'_>)->Result<Status,OriginalTurnFailure>{if !self.committed{return Err(refusal("unreceipted compiler frame cannot execute").into())}self.holder.as_mut().unwrap().poll(cx)}
 pub(super) fn status(&self)->Status{if self.ticket.output().panic.is_some(){return Status::BackendRefusal}if self.ticket.output().result.is_some(){match self.ticket.output().result.as_ref().unwrap(){Ok(_)=>Status::Ready,Err(_)=>Status::BackendRefusal}}else{self.holder.as_ref().map_or(Status::Closed,|frame|frame.status())}}
 pub(super) fn prepare(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.holder.as_mut().unwrap().prepare(cx)}
 pub(super) fn take_prepared(&mut self)->CompiledHandle{let output=self.ticket.output_mut();assert!(output.prepared&&self.holder.is_none(),"compiler result requires actual frame close and original transfer receipt");match output.result.take().expect("receipted compiler result remains owned before publication"){Ok(compiled)=>compiled,Err(_)=>unreachable!("only Ready compiler results are published")}}
 pub(super) fn close(&mut self,witness:Option<&Arc<GuestRuntimes>>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{let Some(frame)=self.holder.as_mut()else{return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})};if !frame.terminal(){return frame.close(witness,cx)}let demand=RetirementDemand{copy_bytes:size_of::<Option<Self>>(),release_bytes:frame.bytes(),depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{self.holder.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})}
 pub(super) fn frame_closed(&self)->bool{self.holder.is_none()}
 pub(super) fn prepare_committed(&mut self){self.ticket.output_mut().prepared=true;}
 pub(super) fn prepared(&self)->bool{self.ticket.output().prepared}
 pub(super) fn can_close(&self)->bool{self.holder.as_ref().is_some_and(|frame|frame.can_close())}
 pub(super) fn terminal(&self)->bool{self.holder.is_none()&&self.ticket.output().result.is_none()&&self.ticket.output().panic.is_none()}
 pub(super) fn describe(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{if let Some(payload)=self.ticket.output().panic.as_ref(){if let Some(text)=payload.downcast_ref::<String>(){return write!(f,"compiler backend panic: {text}")}if let Some(text)=payload.downcast_ref::<&'static str>(){return write!(f,"compiler backend panic: {text}")}return write!(f,"retained compiler backend panic {:?}",payload.as_ref().type_id())}if let Some(Err(error))=self.ticket.output().result.as_ref(){return write!(f,"{error}")}write!(f,"original compiler {:?}",self.status())}
}
impl Drop for CompilerTask{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal(),"compiler task retains its actual frame, future, result and source until paid close");}}

fn guest_future(runtime:&'static GuestRuntimes,package:&'static PackageRef,bytes:&'static[u8])->impl Future<Output=CompileOutput>+Send{runtime.compile(package,bytes)}
pub(super) fn admit(index:usize,generation:u64,runtime:&Arc<GuestRuntimes>,hash:PackageHash,bytes:&mut Option<OriginalSuppliedJobBytes>,output:&mut Option<CompilerTask>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{admit_frame(index,generation,runtime,hash,bytes,output,cx,guest_future)}
fn admit_frame<F:Future<Output=CompileOutput>+Send+'static>(index:usize,generation:u64,runtime:&Arc<GuestRuntimes>,hash:PackageHash,bytes:&mut Option<OriginalSuppliedJobBytes>,output:&mut Option<CompilerTask>,cx:&mut StepContext<'_>,factory:fn(&'static GuestRuntimes,&'static PackageRef,&'static[u8])->F)->Result<bool,OriginalTurnFailure>{
 if generation>(usize::MAX>>6)as u64{return Err(refusal("compiler fixed wake generation exhausted its platform ticket").into())}let ticket=Ticket{index,generation};let slot=ticket.slot();if output.is_some()||!ticket.inputs().terminal()||ticket.output().result.is_some()||ticket.output().panic.is_some()||bytes.is_none(){return Err(refusal("compiler admission requires original input and vacant exact source slots").into())}let fields=unsafe{&*slot.fields.get()};let plugin=fields.plugin.as_ref().unwrap();let length=plugin.len();let capacity=size_of::<Frame<F>>().checked_add(length).ok_or_else(||refusal("compiler frame quote overflow"))?;let binding=OriginalSourceBinding::borrow(cx);let copy_bytes=size_of::<Frame<F>>().checked_add(size_of::<Inputs>()).and_then(|value|value.checked_add(size_of::<CompilerTask>())).and_then(|value|value.checked_add(length)).ok_or_else(||refusal("compiler frame input copy quote overflow"))?;let demand=OriginalConstructionDemand{items:1,copy_bytes,capacity_bytes:capacity,release_bytes:0,depth:1};
 admit_original_construction(cx,output,demand,|_|{let mut name=String::new();name.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"compiler package identity allocation refused"))?;name.push_str(plugin);let capacity=name.capacity()+size_of::<Frame<F>>();*ticket.inputs_mut()=Inputs{runtime:Some(Arc::clone(runtime)),package:Some(PackageRef{package:semio_framework_actor::PackageId(name),hash}),bytes:bytes.take()};let frame=Box::new(Frame{producer:OriginalKernelTaskProducer::from_binding(InputTicket{ticket},binding),ticket,binding,factory});slot.compiler_wake.store(true,Ordering::Release);Ok((CompilerTask{holder:ManuallyDrop::new(Some(frame)),ticket,runtime_address:Arc::as_ptr(runtime).addr(),hash,committed:false},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
