//! 🚪️ Original refused worker source retains all child, parameter, cancellation and cause ownership through paid close.
use semio_framework_job::{BatchJobParams,BatchJobSession,InteractiveJob,InteractiveJobCloseStep,MountedWorkerJobSession,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};

/// 🧾️ Keeps every original refused input until its actual supplied context funds physical retirement.
pub struct RefusedWorkerSource<J:InteractiveJob>{job:ManuallyDrop<Option<J>>,params:ManuallyDrop<Option<BatchJobParams>>,error:ManuallyDrop<Option<OriginalTurnFailure>>,cancel:Option<semio_framework_async::CancelTokenRetirement>,error_retirement:Option<ControlledRetirement<OriginalTurnFailure>>,closing:bool,pending:bool,original:Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>}
impl<J:InteractiveJob> RefusedWorkerSource<J>{
 /// 🎒️ Retains the exact dispatched producer before the initiating turn admits its worker storage.
 pub fn pending(job:J,operation:(semio_framework_job::OperationId,semio_framework_job::Generation))->Self{let mut source=refused(Some(job),None,None);source.pending=true;source.original=Some(operation);source}
 pub fn admission_pending(&self)->bool{self.pending&&!self.closing}
 pub fn admission_demands(&self)->Result<RetirementDemand,ValueError> where J:'static{if self.params.is_none(){Ok(RetirementDemand{depth:1,..Default::default()})}else{semio_framework_job::WorkerJobSession::<J>::owned_admission_demand()}}
 /// 🚪️ Admits the original parameter alias and session on distinct funded caller events.
  fn prepare_pending_params(&mut self,config:semio_framework_job::BatchDriveConfig,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.admission_pending(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker source is already closing or admitted").into())}
  let original=self.original.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"pending worker lost its original business operation"))?;
  if original.0!=cx.operation()||original.1!=cx.generation(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"pending worker requires its original business identity without relabeling").into())}
   if self.params.is_none(){let Some(cancel)=cx.admit_original_cancel_alias()?else{return Ok(false)};*self.params=Some(BatchJobParams{operation:cx.operation(),generation:cx.generation(),cancel,config,now_us:cx.clock_source()});return Ok(false)}
   Ok(true)
  }
  pub fn admit_pending(&mut self,config:semio_framework_job::BatchDriveConfig,cx:&mut StepContext<'_>)->Result<Option<MountedWorkerJobSession<J>>,OriginalTurnFailure> where J:'static{
   if !self.prepare_pending_params(config,cx)?{return Ok(None)}
  let admitted=MountedWorkerJobSession::try_admit_owned(&mut self.job,&mut self.params,cx)?;if let Some((session,_posted))=admitted{self.pending=false;return Ok(Some(session))}Ok(None)
 }
  pub fn admit_pending_batch(&mut self,config:semio_framework_job::BatchDriveConfig,cx:&mut StepContext<'_>)->Result<Option<BatchJobSession<J>>,OriginalTurnFailure> where J:'static{
   if !self.prepare_pending_params(config,cx)?{return Ok(None)}
   let admitted=BatchJobSession::try_admit_owned(&mut self.job,&mut self.params,cx)?;if let Some((session,_posted))=admitted{self.pending=false;return Ok(Some(session))}Ok(None)
  }
 pub fn error(&self)->Option<&OriginalTurnFailure>{self.error.as_ref()}
 fn job(&self)->Option<&J>{self.job.as_ref()}
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()&&self.params.is_none()&&self.cancel.is_none()&&self.error.is_none()&&self.error_retirement.is_none()}
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(job)=self.job.as_ref(){if !self.closing{return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}if job.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<J>>(),depth:1,..Default::default()})}return Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original refused job parent depth overflow"))?})}
  if self.params.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<BatchJobParams>>(),depth:1,..Default::default()})}
  if let Some(cancel)=self.cancel.as_ref(){return if cancel.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_async::CancelTokenRetirement>>(),depth:1,..Default::default()})}else{let mut demand=cancel.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original refused cancellation parent depth overflow"))?;Ok(demand)}}
  if let Some(error)=self.error_retirement.as_ref(){return if error.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<OriginalTurnFailure>>>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:error.next_copy_byte_demand()?,capacity_bytes:error.next_capacity_byte_demand(body)?,release_bytes:error.next_release_byte_demand()?,depth:error.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original refused error parent depth overflow"))?})}}
  Ok(RetirementDemand{copy_bytes:if self.error.is_some(){size_of::<Option<OriginalTurnFailure>>()}else{0},depth:usize::from(self.error.is_some()),..Default::default()})
 }
 fn posted(step:InteractiveJobCloseStep,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{let progress=step.progress();cx.consume_fuel(progress.copied_items as u64);cx.consume_retained(progress).map_err(|error|OriginalTurnFailure::from(error.with_retained_progress(progress)))?;Ok(step)}
 fn metadata(&self,bytes:usize,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Self::posted(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}}else{InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}},cx)}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  let idle=||InteractiveJobCloseStep::Pending{progress:Default::default()};if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0{return Ok(idle())}
  let mut demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(OriginalTurnFailure::from)?;let alias=self.cancel.as_ref().is_some_and(|cancel|cancel.is_original_alias_witness(cx.original_cancel_token()));if alias{demand=RetirementDemand{depth:2,..Default::default()}}
  if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original refused worker close exceeds caller depth").into())}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(idle())}
  if let Some(job)=self.job.as_mut(){if !self.closing{job.begin_close();self.closing=true;return self.metadata(demand.copy_bytes,cx)}if job.terminal_is_empty(){drop(self.job.take());return self.metadata(demand.copy_bytes,cx)}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=cx.with_retained_loan(child,|same|job.close_step(same))?;let step=step.admit(child,job.terminal_is_empty()).map_err(OriginalTurnFailure::from)?;return Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}
  if let Some(params)=self.params.take(){self.cancel=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));return self.metadata(demand.copy_bytes,cx)}
  if let Some(cancel)=self.cancel.as_mut(){if cancel.terminal_is_empty(){drop(self.cancel.take());return self.metadata(demand.copy_bytes,cx)}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let result=if alias{cancel.return_alias_step(cx.original_cancel_token(),child)}else{cancel.close_step(child)};let step=match result{Ok(step)=>step,Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>return Ok(InteractiveJobCloseStep::Blocked),Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx))};return Self::posted(InteractiveJobCloseStep::Pending{progress:step.progress()},cx)}
  if let Some(error)=self.error_retirement.as_mut(){if error.terminal_is_empty(){drop(self.error_retirement.take());return self.metadata(demand.copy_bytes,cx)}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=error.step(child).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,error.terminal_is_empty(),"original refused admission cause").map_err(OriginalTurnFailure::from)?;return Self::posted(InteractiveJobCloseStep::Pending{progress:step.progress()},cx)}
  if let Some(error)=self.error.take(){match ControlledRetirement::new(error){Ok(owner)=>{self.error_retirement=Some(owner);return self.metadata(demand.copy_bytes,cx)},Err((cause,error))=>{*self.error=Some(error);return Err(cause.into())}}}Ok(idle())
 }
}
impl<J:InteractiveJob> Drop for RefusedWorkerSource<J>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original refused worker source dropped before paid physical retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.job);ManuallyDrop::drop(&mut self.params);ManuallyDrop::drop(&mut self.error);}}}}
fn refused<J:InteractiveJob>(job:Option<J>,params:Option<BatchJobParams>,error:Option<OriginalTurnFailure>)->RefusedWorkerSource<J>{RefusedWorkerSource{job:ManuallyDrop::new(job),params:ManuallyDrop::new(params),error:ManuallyDrop::new(error),cancel:None,error_retirement:None,closing:false,pending:false,original:None}}

macro_rules! admit_session {
    ($session:ident, $job:expr, $params:expr, $cx:expr) => {{
        let mut job = Some($job);
        let mut params = Some($params);
        match $session::try_admit_owned(&mut job, &mut params, $cx) {
            Ok(Some((session, _admission))) => Ok(session),
            Ok(None) => Err(refused(job, params, None)),
            Err(error) => Err(refused(job, params, Some(OriginalTurnFailure::from(error)))),
        }
    }};
}

/// 🚪️ Admits one mounted session under the caller's own preparation grant.
pub fn admit_mounted<J: InteractiveJob + 'static>(job: J, params: BatchJobParams, cx: &mut StepContext<'_>) -> Result<MountedWorkerJobSession<J>, RefusedWorkerSource<J>> {
    admit_session!(MountedWorkerJobSession, job, params, cx)
}

/// 🚪️ Admits one caller-driven batch session under the caller's own preparation grant.
pub fn admit_batch<J: InteractiveJob + 'static>(job: J, params: BatchJobParams, cx: &mut StepContext<'_>) -> Result<BatchJobSession<J>, RefusedWorkerSource<J>> {
    admit_session!(BatchJobSession, job, params, cx)
}

#[cfg(test)]
#[path="🫴️refusal/🧪️tests/🦀️.rs"]
mod original_refused_source_tests;
