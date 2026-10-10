//! 🤝️ Semantic result borrows leave original payload headers and physical pages inside their job.
use super::*;
use semio_framework_value::RetirementDemand;

/// 🎟️ A semantic loan's actual receipt cannot be manufactured or cloned by a consumer.
#[derive(Debug)]
pub struct JobOutcomeAdmission { progress:RetainedCloneProgress,operation:OperationId,generation:Generation }
impl JobOutcomeAdmission {pub fn progress(&self)->RetainedCloneProgress{self.progress}}

/// 📬️ Every reference borrows the original job for the complete consumer lifetime.
#[derive(Debug)]
pub enum JobOutcomeBorrow<'a> {
 Yield { admission:JobOutcomeAdmission },
 PreviewReady { payload:&'a RetainedJobPayload,admission:JobOutcomeAdmission },
 CheckpointReady { state:&'a RetainedJobPayload,applied_progress:u64,admission:JobOutcomeAdmission },
 Complete { state:Option<&'a RetainedJobPayload>,output:Option<&'a RetainedJobPayload>,admission:JobOutcomeAdmission },
 Cancelled { admission:JobOutcomeAdmission },
 Fault { detail:&'a RetainedJobPayload,admission:JobOutcomeAdmission },
}
impl<'a> JobOutcomeBorrow<'a> {
 pub fn admission_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 fn admit(cx:&mut StepContext<'_>)->Result<Option<JobOutcomeAdmission>,ValueError>{let demand=Self::admission_demand();let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};cx.consume_retained(progress)?;Ok(Some(JobOutcomeAdmission{progress,operation:cx.operation(),generation:cx.generation()}))}
 pub fn admit_yield(cx:&mut StepContext<'_>)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Yield{admission}))}
 pub fn admit_preview(cx:&mut StepContext<'_>,payload:&'a RetainedJobPayload)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::PreviewReady{payload,admission}))}
 pub fn admit_checkpoint(cx:&mut StepContext<'_>,state:&'a RetainedJobPayload,applied_progress:u64)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::CheckpointReady{state,applied_progress,admission}))}
 pub fn admit_complete(cx:&mut StepContext<'_>,state:Option<&'a RetainedJobPayload>,output:Option<&'a RetainedJobPayload>)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Complete{state,output,admission}))}
 pub fn admit_cancelled(cx:&mut StepContext<'_>)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Cancelled{admission}))}
 pub fn admit_fault(cx:&mut StepContext<'_>,detail:&'a RetainedJobPayload)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Fault{detail,admission}))}
 /// ⚠️ Binds the same pre-admitted fault to its original caller wallet even when no temporal step can be entered.
 pub fn admit_original_fault(operation:OperationId,generation:Generation,grant:RetainedCloneGrant,recipient:&mut RetainedCloneProgress,detail:&'a RetainedJobPayload)->Result<Option<Self>,ValueError>{if !recipient.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fault recipient exceeds its received wallet").with_retained_progress(*recipient))}let demand=Self::admission_demand();if recipient.copied_items==grant.maximum_items||grant.maximum_copy_bytes-recipient.copied_bytes<demand.copy_bytes||grant.maximum_capacity_bytes-recipient.retained_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes-recipient.released_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};*recipient=recipient.checked_add(progress)?;Ok(Some(Self::Fault{detail,admission:JobOutcomeAdmission{progress,operation,generation}}))}

 pub fn admission(&self)->&JobOutcomeAdmission{match self{Self::Yield{admission}|Self::PreviewReady{admission,..}|Self::CheckpointReady{admission,..}|Self::Complete{admission,..}|Self::Cancelled{admission}|Self::Fault{admission,..}=>admission}}
 pub fn is_terminal(&self)->bool{matches!(self,Self::Complete{..}|Self::Cancelled{..}|Self::Fault{..})}
 /// 🎟️ Transfers only the original paid semantic admission and native reference witnesses into exclusive worker custody.
 pub fn into_descriptor(self)->JobOutcomeDescriptor{match self{Self::Yield{admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Yield,admission,[None,None]),Self::PreviewReady{payload,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::PreviewReady,admission,[Some(payload),None]),Self::CheckpointReady{state,applied_progress,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::CheckpointReady{applied_progress},admission,[Some(state),None]),Self::Complete{state,output,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Complete,admission,[state,output]),Self::Cancelled{admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Cancelled,admission,[None,None]),Self::Fault{detail,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Fault,admission,[Some(detail),None])}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// ▶️ Drives the actual retained producer using an original caller context and its same receipt.
pub fn drive_step<'a,J:InteractiveJob+?Sized>(job:&'a mut J,cx:&mut StepContext<'_>,site:&'static str,stage:InteractiveStage,callback_verdict:&mut Option<semio_framework_trace::CallbackVerdict>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{
 *callback_verdict=None;let demand=JobOutcomeBorrow::admission_demand();let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}if cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx)}if cx.fuel_remaining()==0||cx.latest_us().map(|now|now>=cx.deadline_us()).unwrap_or_else(||cx.deadline_exceeded()){return Ok(None)}let before=cx.retained_progress();let original=cx.retained_grant();let watchdog=Watchdog::start_at(site,cx.operation(),cx.generation(),stage,cx.latest_us().or_else(||cx.now_us()));let result=job.step(cx);let after=cx.retained_progress();let end=(cx.now_us)();let mut clock=cx.clock.get();clock.begin(end);cx.clock.set(clock);*callback_verdict=Some(watchdog.finish_at(end));let result=result?;let (Some(copied_items),Some(copied_bytes),Some(retained_capacity_bytes),Some(released_bytes))=(after.copied_items.checked_sub(before.copied_items),after.copied_bytes.checked_sub(before.copied_bytes),after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes),after.released_bytes.checked_sub(before.released_bytes))else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained semantic step decreased its original receipt"))};let delta=RetainedCloneProgress{copied_items,copied_bytes,retained_capacity_bytes,released_bytes};if !delta.fits(original){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained semantic step exceeds its original context").with_retained_progress(delta))}if let Some(outcome)=result.as_ref(){let admission=outcome.admission();if admission.operation!=cx.operation()||admission.generation!=cx.generation()||admission.progress.copied_items>delta.copied_items||admission.progress.copied_bytes>delta.copied_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained semantic outcome lacks its original current receipt").with_retained_progress(delta))}let stage=match outcome{JobOutcomeBorrow::Yield{..}=>None,JobOutcomeBorrow::PreviewReady{..}=>Some(semio_framework_trace::TraceStage::PreviewPublished),JobOutcomeBorrow::CheckpointReady{..}=>Some(semio_framework_trace::TraceStage::Checkpoint),JobOutcomeBorrow::Complete{..}=>Some(semio_framework_trace::TraceStage::Committed),JobOutcomeBorrow::Cancelled{..}=>Some(semio_framework_trace::TraceStage::Cancelled),JobOutcomeBorrow::Fault{..}=>Some(semio_framework_trace::TraceStage::Failed)};if let Some(stage)=stage{semio_framework_trace::record_trace_event_at(cx.operation(),cx.generation(),stage,end);}}Ok(result)
}
