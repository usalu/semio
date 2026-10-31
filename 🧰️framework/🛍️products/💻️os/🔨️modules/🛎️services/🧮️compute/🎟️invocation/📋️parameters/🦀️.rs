//! 📋️ Compute parameters retain the actual supplier across separately funded metadata, cancellation alias and publication.
use semio_framework_async::{CancelToken,CancelTokenRetirement,CancelTokenRetirementError};
use semio_framework_job::{StepContext,OriginalTurnFailure,OperationId,Generation};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};

pub struct ComputeJobParamsPreparation{operation:OperationId,generation:Generation,parent:usize,clock:fn()->Option<u64>,config:BatchDriveConfig,cancel:ManuallyDrop<Option<CancelToken>>,retirement:ManuallyDrop<Option<CancelTokenRetirement>>,published:bool,closing:bool}
impl ComputeJobParamsPreparation{
 pub fn admit(config:BatchDriveConfig,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
  let parent=cx.original_source_parent()?.source_identity()?;let grant=cx.retained_grant();let copy=size_of::<Self>();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None)}
  if config.retained.maximum_items>grant.maximum_items||config.retained.maximum_copy_bytes>grant.maximum_copy_bytes||config.retained.maximum_capacity_bytes>grant.maximum_capacity_bytes||config.retained.maximum_release_bytes>grant.maximum_release_bytes||config.retained.maximum_depth>grant.maximum_depth||config.fuel_per_step>cx.fuel_remaining(){return Err(refusal("original compute configuration exceeds its supplied parent").into())}
  let operation=cx.operation();let generation=cx.generation();let clock=cx.clock_source();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);Ok(Some(Self{operation,generation,parent,clock,config,cancel:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),published:false,closing:false}))
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{if self.operation!=cx.operation()||self.generation!=cx.generation()||self.parent!=cx.original_source_parent()?.source_identity()?||!std::ptr::fn_addr_eq(self.clock,cx.clock_source()){return Err(refusal("original compute parameter supplier changed"))}Ok(())}
 pub fn advance(&mut self,params:&mut Option<BatchJobParams>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.closing{return Ok(false)}if self.published{return Ok(params.is_some())}if params.is_some(){return Err(refusal("original compute parameters already have an owned destination").into())}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  if self.cancel.is_none(){cx.admit_original_cancel_alias(&mut self.cancel)?;return Ok(false)}
  let grant=cx.retained_grant();let copied=0;if grant.maximum_items==0||grant.maximum_copy_bytes<copied||grant.maximum_depth==0{return Ok(false)}*params=Some(BatchJobParams{operation:self.operation,generation:self.generation,cancel:self.cancel.take().unwrap(),config:self.config,now_us:self.clock});self.published=true;post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if self.terminal_is_empty(){return Ok(true)}let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if !self.closing{self.closing=true;post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false)}
  if self.retirement.is_none(){*self.retirement=Some(CancelTokenRetirement::from_token(self.cancel.take().unwrap()));post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false)}
  let owner=self.retirement.as_mut().unwrap();if owner.terminal_is_empty(){drop(self.retirement.take());post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(self.terminal_is_empty())}
  let turn=owner.return_alias_step(cx.original_cancel_token(),grant);let result=match turn.result{Ok(step)=>Ok(step),Err(CancelTokenRetirementError::Refused(error))=>Err(error),Err(CancelTokenRetirementError::Blocked(_))=>Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default()))};semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work:turn.work})?;Ok(false)
 }

 pub fn terminal_is_empty(&self)->bool{self.cancel.is_none()&&self.retirement.is_none()}
}
impl Drop for ComputeJobParamsPreparation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"compute parameter preparation abandoned its original cancellation alias")}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

fn post(cx:&mut StepContext<'_>,progress:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok(semio_framework_value::RetainedCloneStep::Progress(progress)),work:1})?;Ok(())}
