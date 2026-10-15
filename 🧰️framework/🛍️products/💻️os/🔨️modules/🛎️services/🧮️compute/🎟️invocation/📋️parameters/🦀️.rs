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
  if self.cancel.is_none(){let Some(cancel)=cx.admit_original_cancel_alias()?else{return Ok(false)};*self.cancel=Some(cancel);cx.consume_fuel(1);return Ok(false)}
  let grant=cx.retained_grant();let copied=size_of::<Option<BatchJobParams>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copied||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()})?;cx.consume_fuel(1);*params=Some(BatchJobParams{operation:self.operation,generation:self.generation,cancel:self.cancel.take().unwrap(),config:self.config,now_us:self.clock});self.published=true;Ok(true)
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;self.closing=true;let grant=cx.retained_grant();if self.terminal_is_empty(){return Ok(true)}if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if self.retirement.is_none(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);*self.retirement=Some(CancelTokenRetirement::from_token(self.cancel.take().unwrap()));return Ok(false)}
  let owner=self.retirement.as_mut().unwrap();if owner.terminal_is_empty(){cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);drop(self.retirement.take());return Ok(self.terminal_is_empty())}
  let step=owner.return_alias_step(cx.original_cancel_token(),grant).map_err(|error|match error{CancelTokenRetirementError::Refused(error)=>OriginalTurnFailure::debit_original(error,cx),CancelTokenRetirementError::Blocked(_)=>refusal("original compute cancellation alias is still borrowed").into()})?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);Ok(false)
 }
 pub fn terminal_is_empty(&self)->bool{self.cancel.is_none()&&self.retirement.is_none()}
}
impl Drop for ComputeJobParamsPreparation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"compute parameter preparation abandoned its original cancellation alias")}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
