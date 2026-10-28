//! 📥️ Original response and intent inputs remain owned through deferred dispatch and paid source return.
use std::cell::{Cell, RefCell};
use crate::{DIRTY_INTENT_INSTANCE_CAPACITY, DirtyIntentBatch, collect_original_publication_receipts, native_close_key, retain_refused_publication, route_exchange_output};
use semio_framework::kernel::{Effect, Event};
use semio_framework_plugin_reactor_tasks::REGISTRY;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_framework_plugin_reactor_lifecycle::instance_lifetime;
use semio_framework_plugin_actor_host::requests;
use semio_framework_ui_contract as ui_contract;
use semio_framework_job::{OriginalCallerSourceLoan, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;
#[path="auxiliary.rs"]
pub mod auxiliary;

#[doc(hidden)]
pub struct ReceivingSource{pub identity:(semio_framework_job::OperationId,semio_framework_job::Generation),pub parent:usize,pub recipient:usize,pub epoch:Cell<Option<u64>>,pub clock:fn()->Option<u64>,observed:Cell<Option<u64>>,deadline:u64,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,returning_loan:bool}
impl ReceivingSource{
 pub fn new(cx:&StepContext<'_>)->Result<Self,OriginalTurnFailure>{Ok(Self{identity:(cx.operation(),cx.generation()),parent:cx.original_source_parent()?.source_identity()?,recipient:cx.original_recipient_identity(),epoch:Cell::new(cx.original_source_epoch()),clock:cx.clock_source(),observed:Cell::new(cx.latest_us()),deadline:cx.deadline_us(),loan:ManuallyDrop::new(None),returning_loan:false})}
 pub fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let epoch=cx.original_source_epoch();let now=cx.now_us();
  if self.identity!=(cx.operation(),cx.generation())||self.parent!=cx.original_source_parent()?.source_identity()?||self.recipient!=cx.original_recipient_identity()||!std::ptr::fn_addr_eq(self.clock,cx.clock_source())||self.deadline!=cx.deadline_us()||self.epoch.get().is_some()!=epoch.is_some()||matches!((self.epoch.get(),epoch),(Some(before),Some(after))if after<before)||now.is_none()||matches!((self.observed.get(),now),(Some(before),Some(after))if after<before){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiving continuation requires its initiating source recipient clock and work lifetime").into())}
  self.epoch.set(epoch);self.observed.set(now);
  if !self.returning_loan{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())
 }
 pub fn admit(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.loan.is_none()&&cx.is_cancelled(){return Ok(true)}if self.returning_loan||self.loan.is_some(){return Ok(true)}*self.loan=cx.original_source_parent()?.admit_loan(cx)?;Ok(false)}
 pub fn close(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;let Some(loan)=self.loan.as_mut()else{return Ok(true)};self.returning_loan=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}Ok(false)}
}
#[doc(hidden)]
pub fn metadata(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
type ResponseFields=(Option<String>,Option<semio_framework_value::DslValue>,Option<semio_framework::Fault>);
#[doc(hidden)]
#[path="📦️completed/🦀️.rs"]
pub mod completed;
#[path="staged.rs"]
pub mod staged;
#[doc(hidden)]
pub struct RetainedResponse{pub metadata:ManuallyDrop<Option<Box<semio_framework_plugin_runtime_ops::ops_typed_operations::OriginalResponseActionSource>>>,pub instance:Option<u32>,pub key:Option<instance_lifetime::NativeCloseKey>,pub witness:ReceivingSource,pub request:Option<semio_framework::kernel::RequestId>,pub input:ManuallyDrop<Option<completed::CompletedInput>>,pub fields:ManuallyDrop<Option<ResponseFields>>,pub request_result:ManuallyDrop<Option<Result<Vec<u8>,semio_framework::Fault>>>,pub request_closing:ManuallyDrop<Option<ControlledRetirement<Result<Vec<u8>,semio_framework::Fault>>>>,pub closing:ManuallyDrop<Option<ControlledRetirement<ResponseFields>>>,pub output:ManuallyDrop<Option<semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput>>,pub failure:ManuallyDrop<Option<OriginalTurnFailure>>,pub finished:bool}
impl RetainedResponse{
 pub async fn step<PA:semio_framework_plugin_reactor_app::PluginApp>(&mut self,runtime:&semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut StepContext<'_>)->Result<Option<semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput>,OriginalTurnFailure>{
  if !self.witness.admit(cx)?||cx.should_yield(){return Ok(None)}
  if self.finished{if let Some(source)=self.metadata.as_mut(){if !source.terminal_is_empty(){source.close_step(runtime,cx).await?;return Ok(None)}let d=semio_framework_plugin_runtime_ops::ops_typed_operations::OriginalResponseActionSource::release_demand();semio_framework_job::close_original_retained_step(cx,d,|_|{self.metadata.take();Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()}))})?;return Ok(None)}}
  if self.finished&&self.output.is_some(){if !metadata(cx)?{return Ok(None)}return Ok(self.output.take())}
  if let Some(input)=self.input.as_mut(){
   if self.finished||cx.is_cancelled(){self.finished=true;if !input.close_step(cx)?{return Ok(None)}if !metadata(cx)?{return Ok(None)}self.input.take();return Ok(None)}
   if self.fields.is_none()&&self.request_result.is_none(){match input.step(cx){Ok(Some(completed::CompletedResult::Response(fields)))=>{*self.fields=Some(fields);return Ok(None)},Ok(Some(completed::CompletedResult::Request(result)))=>{*self.request_result=Some(result);return Ok(None)},Ok(None)=>return Ok(None),Err(failure)=>{*self.failure=Some(failure);self.finished=true;return Ok(None)}}}
  }else if self.request.is_some()&&!self.finished{return Ok(None)}
  if !self.finished{
   if self.instance.is_none_or(|instance|native_close_key(runtime,instance).ok()!=self.key)||cx.is_cancelled(){self.finished=true;return Ok(None)}
   if self.request_result.is_some(){if !metadata(cx)?{return Ok(None)}REGISTRY.with(|registry|registry.resolve_original(self.request.unwrap(),&mut self.request_result));self.finished=true;return Ok(None)}
   if self.metadata.is_none(){semio_framework_plugin_runtime_ops::ops_typed_operations::OriginalResponseActionSource::admit(self.instance.unwrap(),cx,&mut self.metadata)?;return Ok(None)}
   let(action,args,fault)=self.fields.as_mut().unwrap();
   match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_dispatch_response_action(runtime,self.instance.unwrap(),action.as_deref().unwrap(),args,fault,self.metadata.as_mut().unwrap(),identity,cx).await{
    Ok(None)=>return Ok(None),Ok(Some(mut output))=>{if let Err(refusal)=collect_original_publication_receipts(&mut output,cx){*self.failure=Some(retain_refused_publication(self.instance.unwrap(),output,refusal,cx));}else{*self.output=Some(output)}self.finished=true;return Ok(None)},Err(failure)=>{*self.failure=Some(failure);self.finished=true;return Ok(None)}
   }
  }
  if let Some(owner)=self.request_closing.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(None)}if !metadata(cx)?{return Ok(None)}self.request_closing.take();return Ok(None)}
  if self.request_result.is_some(){if !metadata(cx)?{return Ok(None)}let result=self.request_result.take().unwrap();match ControlledRetirement::new(result){Ok(owner)=>*self.request_closing=Some(owner),Err((failure,result))=>{*self.request_result=Some(result);return Err(failure.into())}}return Ok(None)}
  if let Some(owner)=self.closing.as_mut(){if !owner.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?,|grant|owner.step(grant))?;return Ok(None)}if !metadata(cx)?{return Ok(None)}self.closing.take();return Ok(None)}
  if self.fields.is_some(){if !metadata(cx)?{return Ok(None)}let fields=self.fields.take().unwrap();match ControlledRetirement::new(fields){Ok(owner)=>*self.closing=Some(owner),Err((failure,fields))=>{*self.fields=Some(fields);return Err(failure.into())}}return Ok(None)}
  if !self.witness.close(cx)?{return Ok(None)}
  if !metadata(cx)?{return Ok(None)}if let Some(request)=self.request.take(){REGISTRY.with(|registry|registry.release_completed(request));}if let Some(failure)=self.failure.take(){return Err(failure)}Ok(self.output.take())
 }
 pub fn terminal(&self)->bool{self.finished&&self.metadata.is_none()&&self.request.is_none()&&self.input.is_none()&&self.fields.is_none()&&self.request_result.is_none()&&self.request_closing.is_none()&&self.closing.is_none()&&self.witness.loan.is_none()&&self.output.is_none()&&self.failure.is_none()}
}
impl Drop for RetainedResponse{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal(),"original response requires source retirement before terminal drop")}}
thread_local!{static RESPONSES:RefCell<[Option<RetainedResponse>;requests::REQUEST_SLOTS]>=const{RefCell::new([const{None};requests::REQUEST_SLOTS])};static RESPONSE_CURSOR:Cell<usize>=const{Cell::new(0)};}
#[doc(hidden)]
pub fn retain_completed<PA:semio_framework_plugin_reactor_app::PluginApp>(runtime:&semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,donor:&mut Option<Event>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 let Some(Event::Completed{req,..})=donor.as_ref()else{return Ok(false)};let req=*req;
 let instance=REGISTRY.with(|registry|registry.continuation_instance(req).or_else(||registry.pending_instance(req)));
 let key=instance.and_then(|instance|native_close_key(runtime,instance).ok());
 RESPONSES.with(|slots|{
  let mut slots=slots.borrow_mut();
  let index=if let Some(index)=slots.iter().position(|slot|slot.as_ref().is_some_and(|owner|owner.request==Some(req)&&owner.witness.identity==(cx.operation(),cx.generation()))){index}else{
   let Some(index)=slots.iter().position(Option::is_none)else{return Ok(false)};
   let witness=ReceivingSource::new(cx)?;if !metadata(cx)?{return Ok(false)}
   slots[index]=Some(RetainedResponse{metadata:ManuallyDrop::new(None),instance,key,witness,request:Some(req),input:ManuallyDrop::new(None),fields:ManuallyDrop::new(None),request_result:ManuallyDrop::new(None),request_closing:ManuallyDrop::new(None),closing:ManuallyDrop::new(None),output:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),finished:false});return Ok(false)
  };
  let owner=slots[index].as_mut().unwrap();
  if owner.input.is_some(){return Ok(false)}
  if !owner.witness.admit(cx)?||!metadata(cx)?{return Ok(false)}
  let continuation=REGISTRY.with(|registry|registry.take_continuation(req));
  let partial=if continuation.is_none(){REGISTRY.with(|registry|registry.take_pending_partial(req))}else{None};
  let event=donor.take().expect("borrowed Completed input remains resident through admission");
  *owner.input=Some(match continuation{Some(continuation)=>completed::CompletedInput::new(event,continuation),None=>completed::CompletedInput::new_request(event,partial.unwrap_or_default())});owner.finished=cx.is_cancelled()||key.is_none();Ok(true)
 })
}
#[doc(hidden)]
pub async fn advance_responses<PA:semio_framework_plugin_reactor_app::PluginApp>(runtime:&semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,effects:&mut Vec<Effect>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let start=RESPONSE_CURSOR.get();let selected=RESPONSES.with(|slots|{let mut slots=slots.borrow_mut();(0..requests::REQUEST_SLOTS).map(|offset|(start+offset)%requests::REQUEST_SLOTS).find_map(|index|slots[index].take().map(|owner|(index,owner)))});let Some((index,mut owner))=selected else{return Ok(())};let result=match owner.step(runtime,identity,cx).await{Ok(Some(output))=>{if let Err(failure)=route_exchange_output(owner.instance.expect("only a live response owns publication"),output,effects,cx){assert!(owner.failure.is_none(),"original response retains one refused output handback");*owner.failure=Some(failure);}Ok(())},Ok(None)=>Ok(()),Err(failure)=>Err(failure)};RESPONSE_CURSOR.set((index+1)%requests::REQUEST_SLOTS);if !owner.terminal(){RESPONSES.with(|slots|slots.borrow_mut()[index]=Some(owner))}result}
#[doc(hidden)]
pub fn response_pending()->bool{RESPONSES.with(|slots|slots.borrow().iter().any(Option::is_some))}
pub(super) fn response_key_empty(key:instance_lifetime::NativeCloseKey)->bool{RESPONSES.with(|slots|slots.borrow().iter().flatten().all(|owner|owner.key!=Some(key)))}

struct RetainedIntents{pub witness:ReceivingSource,pub batches:ManuallyDrop<ui_contract::UiFixedList<DirtyIntentBatch,DIRTY_INTENT_INSTANCE_CAPACITY>>,pub current:ManuallyDrop<Option<DirtyIntentBatch>>,pub value:ManuallyDrop<Option<ui_contract::UiValueRetirement>>,pub outputs:ManuallyDrop<[Option<(u32,semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput)>;DIRTY_INTENT_INSTANCE_CAPACITY]>,output_count:usize,current_key:Option<instance_lifetime::NativeCloseKey>,dirty_cursor:usize,pub intent_source:ManuallyDrop<Option<Box<semio_framework_plugin_runtime_ops::ops_typed_operations::OriginalIntentActionSource>>>,intent_done:bool,pub failure:ManuallyDrop<Option<OriginalTurnFailure>>,pub closing:bool}
impl RetainedIntents{
 async fn close_intent_source<PA:semio_framework_plugin_reactor_app::PluginApp>(&mut self,runtime:&semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,advance:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let Some(source)=self.intent_source.as_mut()else{return Ok(true)};let intent=self.current.as_mut().and_then(|batch|batch.intents.get_mut(self.dirty_cursor)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original intent continuation lost its actual batch cell"))?;if !source.terminal_is_empty(){source.close_step(runtime,intent,cx).await?;return Ok(false)}let mut d=semio_framework_plugin_runtime_ops::ops_typed_operations::OriginalIntentActionSource::release_demands();d.copy_bytes=std::mem::size_of::<(usize,bool)>();semio_framework_job::close_original_retained_step(cx,d,|_|{self.intent_source.take();if advance{self.dirty_cursor+=1;self.intent_done=false;}Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(false)}
 async fn close_current<PA:semio_framework_plugin_reactor_app::PluginApp>(&mut self,runtime:&semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.close_intent_source(runtime,false,cx).await?{return Ok(false)}
  let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if let Some(value)=self.value.as_mut(){if value.terminal_is_empty(){if !metadata(cx)?{return Ok(false)}self.value.take();return Ok(false)}if grant.maximum_copy_bytes==0{return Ok(false)}let step=value.close_step(1,grant.maximum_copy_bytes).map_err(|reason|ValueError::literal(ValueRefusalKind::InvariantViolated,reason))?;if step.progressed{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:step.released_bytes,..Default::default()})?;cx.consume_fuel(1)}return Ok(false)}
  let Some(batch)=self.current.as_mut()else{return Ok(true)};
  if let Some(intent)=batch.intents.last_mut(){if let Some(value)=intent.args.take().or_else(||intent.input.take()){*self.value=Some(ui_contract::UiValueRetirement::new(value));metadata(cx)?;return Ok(false)}if grant.maximum_copy_bytes<std::mem::size_of::<ui_contract::UiIntent>()||grant.maximum_depth<batch.intents.next_pop_depth_demand().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent pop depth quote refused"))?{return Ok(false)}batch.intents.pop();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<ui_contract::UiIntent>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if !batch.intents.terminal_is_empty(){let release=batch.intents.next_release_allocation_bytes().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent backing release quote refused"))?;if grant.maximum_release_bytes<release||grant.maximum_depth<batch.intents.next_release_depth_demand().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent backing depth quote refused"))?{return Ok(false)}let step=batch.intents.release_empty_page(grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent backing release refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:step.released_allocation_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if !metadata(cx)?{return Ok(false)}self.current.take();self.current_key=None;self.dirty_cursor=0;Ok(true)
 }
 pub async fn step<PA:semio_framework_plugin_reactor_app::PluginApp>(&mut self,runtime:&semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.witness.admit(cx)?||cx.should_yield(){return Ok(false)}
  if self.current.is_some(){
   if !self.closing{
    let instance=self.current.as_ref().unwrap().instance;
    if native_close_key(runtime,instance).ok()!=self.current_key||cx.is_cancelled(){self.closing=true;return Ok(false)}
    if self.intent_done{self.close_intent_source(runtime,true,cx).await?;return Ok(false)}
    if self.dirty_cursor==self.current.as_ref().unwrap().intents.len(){if metadata(cx)?{self.closing=true;}return Ok(false)}
    let intent=self.current.as_mut().unwrap().intents.get_mut(self.dirty_cursor).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original intent batch cursor changed"))?;
    if self.intent_source.is_none(){semio_framework_plugin_runtime_ops::ops_typed_operations::OriginalIntentActionSource::admit(instance,intent,cx,&mut self.intent_source)?;return Ok(false)}
    match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_dispatch_intent(runtime,instance,intent,self.intent_source.as_mut().unwrap(),identity,cx).await{Ok(None)=>return Ok(false),Ok(Some(mut output))=>{if let Err(refusal)=collect_original_publication_receipts(&mut output,cx){*self.failure=Some(retain_refused_publication(instance,output,refusal,cx));self.closing=true;}else{self.outputs[self.output_count]=Some((instance,output));self.output_count+=1;self.intent_done=true;}return Ok(false)},Err(failure)=>{*self.failure=Some(failure);self.closing=true;return Ok(false)}}
   }
   if !self.close_current(runtime,cx).await?{return Ok(false)}self.closing=self.failure.is_some()||cx.is_cancelled();return Ok(false)
  }
  if !self.batches.is_empty(){let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<std::mem::size_of::<DirtyIntentBatch>()||grant.maximum_depth<self.batches.next_pop_depth_demand().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent batch pop quote refused"))?{return Ok(false)}*self.current=self.batches.pop();self.current_key=self.current.as_ref().and_then(|batch|native_close_key(runtime,batch.instance).ok());cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<DirtyIntentBatch>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  if !self.batches.terminal_is_empty(){let grant=cx.retained_grant();let release=self.batches.next_release_allocation_bytes().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent batch release quote refused"))?;if grant.maximum_release_bytes<release||grant.maximum_depth<self.batches.next_release_depth_demand().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent batch release depth refused"))?{return Ok(false)}let step=self.batches.release_empty_page(grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"intent batch release refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:step.released_allocation_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  self.witness.close(cx)
 }
 pub fn terminal(&self)->bool{self.batches.terminal_is_empty()&&self.current.is_none()&&self.value.is_none()&&self.intent_source.is_none()&&self.witness.loan.is_none()&&self.outputs.iter().all(Option::is_none)&&self.failure.is_none()}
}
impl Drop for RetainedIntents{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal(),"original intent batch requires complete input and outcome retirement")}}
thread_local!{static INTENTS:RefCell<Option<RetainedIntents>>=const{RefCell::new(None)};}
pub(super) fn retain_intents(source:&mut ui_contract::UiFixedList<DirtyIntentBatch,DIRTY_INTENT_INSTANCE_CAPACITY>,cx:&StepContext<'_>)->Result<bool,OriginalTurnFailure>{if source.terminal_is_empty(){return Ok(true)}INTENTS.with(|slot|{let mut slot=slot.borrow_mut();if slot.is_some(){return Ok(false)}let witness=ReceivingSource::new(cx)?;*slot=Some(RetainedIntents{witness,batches:ManuallyDrop::new(std::mem::replace(source,ui_contract::UiFixedList::empty())),current:ManuallyDrop::new(None),value:ManuallyDrop::new(None),outputs:ManuallyDrop::new([const{None};DIRTY_INTENT_INSTANCE_CAPACITY]),output_count:0,current_key:None,dirty_cursor:0,intent_source:ManuallyDrop::new(None),intent_done:false,failure:ManuallyDrop::new(None),closing:false});Ok(true)})}
pub(super) async fn advance_intents<PA:semio_framework_plugin_reactor_app::PluginApp>(runtime:&semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,effects:&mut Vec<Effect>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let Some(mut owner)=INTENTS.with(|slot|slot.borrow_mut().take())else{return Ok(())};let result=owner.step(runtime,identity,cx).await;for index in 0..owner.outputs.len(){if let Some((instance,output))=owner.outputs[index].take(){if let Err(failure)=route_exchange_output(instance,output,effects,cx){INTENTS.with(|slot|*slot.borrow_mut()=Some(owner));return Err(failure)}}}owner.output_count=0;match result{Ok(true)=>match owner.failure.take(){Some(failure)=>Err(failure),None=>Ok(())},result=>{INTENTS.with(|slot|*slot.borrow_mut()=Some(owner));result.map(|_|())}}}
pub(super) fn intent_key_empty(key:instance_lifetime::NativeCloseKey)->bool{INTENTS.with(|slot|slot.borrow().as_ref().is_none_or(|owner|owner.batches.iter().all(|batch|batch.instance!=key.instance())&&owner.current.as_ref().is_none_or(|batch|batch.instance!=key.instance())&&owner.outputs.iter().flatten().all(|(instance,_)|*instance!=key.instance())))}
pub(super) fn intent_pending()->bool{INTENTS.with(|slot|slot.borrow().is_some())}

