//! 🎟️ Guest calls borrow the existing paid Core and authentic physical context throughout their retained machine phases.
use super::{OwnedRuntime,OwnedGuestInstance};
use semio_framework_plugin_interpreter::{OwnedSemioExport,ExportKind,Value,CoreOriginalInvocationStatus};
use semio_framework_actor::{RetainedTurnInput,OriginalGuestExecutionBudget,OriginalGuestExecutionReceipt};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::size_of;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Opening,Driving,Capturing,Closing,Ready,Aborting}
/// 🪢️ The inline call frontier keeps its exact original input and arguments until its actual machine is funded closed.
pub struct OwnedOriginalCall{export:OwnedSemioExport,arguments:[Value;semio_framework_actor::OriginalJobBudget::SCALAR_ARGUMENT_COUNT],count:usize,input:RetainedTurnInput,phase:Phase,result:Option<Value>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OwnedOriginalCallStatus{Pending,Ready,Cancelled,Deadline}
pub struct OwnedOriginalCallProgress{pub result:Result<OwnedOriginalCallStatus,OriginalTurnFailure>,pub execution:Option<OriginalGuestExecutionReceipt>}
#[derive(Clone,Copy)]
pub struct OwnedOriginalCallResult{pub input:RetainedTurnInput,pub value:Option<Value>}
impl OwnedGuestInstance{
 pub fn original_call_requires_execution(&self)->bool{self.original_call.as_ref().is_some_and(|owner|owner.phase==Phase::Driving)}
 pub fn original_call_is_pending(&self)->bool{self.original_call.is_some()}
}
impl OwnedRuntime{
 pub fn begin_original_call_step(&self,state:&mut OwnedGuestInstance,export:OwnedSemioExport,arguments:&[Value],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  validate(state,cx)?;if !std::ptr::fn_addr_eq(self.original_clock,cx.clock_source()){return Err(refused("original guest call requires its actual embedding clock").into())}if state.original_call.is_some(){return Err(refused("original guest call still owns its preceding machine").into())}if arguments.len()>semio_framework_actor::OriginalJobBudget::SCALAR_ARGUMENT_COUNT{return Err(refused("original guest call exceeds its declared inline argument extent").into())}if cx.is_cancelled()||!metadata(cx,size_of::<OwnedOriginalCall>())?{return Ok(false)}let mut values=[Value::I64(0);semio_framework_actor::OriginalJobBudget::SCALAR_ARGUMENT_COUNT];values[..arguments.len()].copy_from_slice(arguments);let input=RetainedTurnInput::borrow_original(cx)?;state.original_call=Some(OwnedOriginalCall{export,arguments:values,count:arguments.len(),input,phase:Phase::Opening,result:None});Ok(true)
 }
 pub fn advance_original_call_step(&self,state:&mut OwnedGuestInstance,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->Result<OwnedOriginalCallProgress,OriginalTurnFailure>{
  validate(state,cx)?;let owner=state.original_call.as_mut().ok_or_else(||refused("original guest call has no admitted frontier"))?;if owner.phase==Phase::Aborting{return Err(refused("original guest call cannot resume during its actual cleanup").into())}if execution.is_some()!=(owner.phase==Phase::Driving){return Err(refused("original guest execution authorization differs from its actual machine phase").into())}let module=state.artifact.original_actor_module();let core=state.original_core.as_mut().unwrap();
  if let Some(execution)=execution{let call=core.advance_scalar(module,execution,cx)?;let result=match call.result{Ok(CoreOriginalInvocationStatus::Complete)=>{if metadata(cx,size_of::<Phase>())?{owner.phase=Phase::Capturing;}Ok(OwnedOriginalCallStatus::Pending)},Ok(CoreOriginalInvocationStatus::Pending)=>Ok(OwnedOriginalCallStatus::Pending),Ok(CoreOriginalInvocationStatus::Cancelled)=>Ok(OwnedOriginalCallStatus::Cancelled),Ok(CoreOriginalInvocationStatus::Deadline)=>Ok(OwnedOriginalCallStatus::Deadline),Err(failure)=>Err(failure)};return Ok(OwnedOriginalCallProgress{result,execution:Some(call.execution)})}
  let status=if cx.is_cancelled(){OwnedOriginalCallStatus::Cancelled}else if cx.deadline_exceeded(){OwnedOriginalCallStatus::Deadline}else{match owner.phase{
   Phase::Opening=>{let function=match module.export(owner.export.core_name()){Some(ExportKind::Function(function))=>*function,_=>return Err(refused("original guest call has no actual defined export").into())};if metadata(cx,size_of::<Phase>())?&&core.begin_function_step(module,function,&owner.arguments[..owner.count],cx)?{owner.phase=Phase::Driving;}OwnedOriginalCallStatus::Pending},
   Phase::Capturing=>{let value=match core.invocation_values(){Some([])=>None,Some([value])=>Some(*value),_=>return Err(refused("original guest call requires its declared scalar result extent").into())};if metadata(cx,size_of::<Option<Value>>()+size_of::<Phase>())?{owner.result=value;owner.phase=Phase::Closing;}OwnedOriginalCallStatus::Pending},
   Phase::Closing=>{if core.close_invocation_step(module,cx)?&&metadata(cx,size_of::<Phase>())?{owner.phase=Phase::Ready;OwnedOriginalCallStatus::Ready}else{OwnedOriginalCallStatus::Pending}},
   Phase::Ready=>OwnedOriginalCallStatus::Ready,
   Phase::Driving|Phase::Aborting=>unreachable!()
  }};Ok(OwnedOriginalCallProgress{result:Ok(status),execution:None})
 }
 pub fn take_original_call_result_step(&self,state:&mut OwnedGuestInstance,cx:&mut StepContext<'_>)->Result<Option<OwnedOriginalCallResult>,OriginalTurnFailure>{validate(state,cx)?;let owner=state.original_call.as_ref().ok_or_else(||refused("original guest result has no retained call"))?;if owner.phase!=Phase::Ready{return Ok(None)}if !metadata(cx,size_of::<OwnedOriginalCall>())?{return Ok(None)}let owner=state.original_call.take().unwrap();Ok(Some(OwnedOriginalCallResult{input:owner.input,value:owner.result}))}
 pub fn close_original_call_step(&self,state:&mut OwnedGuestInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if state.original_call.is_none(){return Ok(true)}validate(state,cx)?;let owner=state.original_call.as_mut().unwrap();if !metadata(cx,size_of::<Phase>())?{return Ok(false)}owner.phase=Phase::Aborting;if !state.original_core.as_mut().unwrap().close_invocation_step(state.artifact.original_actor_module(),cx)?{return Ok(false)}if !metadata(cx,size_of::<OwnedOriginalCall>())?{return Ok(false)}state.original_call.take();Ok(true)
 }
}
fn validate(state:&OwnedGuestInstance,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{state.original_source.as_ref().ok_or_else(||refused("original guest call requires its retained genuine source"))?.validate_context(cx)?;let epoch=cx.original_source_epoch().ok_or_else(||refused("original guest call requires its actual issued epoch"))?;let input=state.original_startup_input.ok_or_else(||refused("original guest call requires its actual final startup input"))?;if epoch<input.epoch||state.original_call.as_ref().is_some_and(|owner|epoch<owner.input.epoch){return Err(refused("original guest call precedes its actual initiating input").into())}state.original_core.as_ref().ok_or_else(||refused("original guest call has no actual paid Core"))?.validate_original_context(state.artifact.original_actor_module(),cx)}
fn metadata(cx:&mut StepContext<'_>,copy:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
