//! 🤝️ Semantic result borrows leave original payload headers and physical pages inside their job.
pub use semio_framework_job_payload::RetainedJobPayload;
pub use semio_framework_trace::{OperationId,Generation};
pub use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
#[path="../🎟️descriptor/🦀️.rs"] mod descriptor;
pub use descriptor::{JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView,JobOutcomeDescriptorSlot};
/// 🧭️ Original semantic admission uses the caller’s retained wallet and work recipient.
pub trait JobOutcomeContext{fn retained_grant(&self)->RetainedCloneGrant;fn should_yield(&self)->bool;fn retirement_should_yield(&self)->bool;fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>;fn consume_fuel(&mut self,work:u64);fn operation(&self)->OperationId;fn generation(&self)->Generation;fn original_retirement_active(&self)->bool;}
use semio_framework_value_retained_clone::RetirementDemand;

/// 🎟️ See `README.md`.
#[derive(Debug)]
pub struct JobOutcomeAdmission { progress:RetainedCloneProgress,operation:OperationId,generation:Generation }
impl JobOutcomeAdmission {pub fn progress(&self)->RetainedCloneProgress{self.progress}pub fn operation(&self)->OperationId{self.operation}pub fn generation(&self)->Generation{self.generation}}

/// 📬️ See `README.md`.
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
 fn admit(cx:&mut impl JobOutcomeContext)->Result<Option<JobOutcomeAdmission>,ValueError>{Self::admit_with_cursor(cx,0)}
 fn admit_with_cursor(cx:&mut impl JobOutcomeContext,cursor_copy_bytes:usize)->Result<Option<JobOutcomeAdmission>,ValueError>{Self::admit_cursor_mode(cx,cursor_copy_bytes,false)}
 fn admit_cursor_mode(cx:&mut impl JobOutcomeContext,cursor_copy_bytes:usize,retiring:bool)->Result<Option<JobOutcomeAdmission>,ValueError>{let mut demand=Self::admission_demand();demand.copy_bytes=demand.copy_bytes.checked_add(cursor_copy_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original outcome cursor copy extent overflow"))?;let grant=cx.retained_grant();if (if retiring{cx.retirement_should_yield()}else{cx.should_yield()})||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};let received=cx.consume_retained(progress);cx.consume_fuel(1);received?;Ok(Some(JobOutcomeAdmission{progress,operation:cx.operation(),generation:cx.generation()}))}
 pub fn admit_yield(cx:&mut impl JobOutcomeContext)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Yield{admission}))}
 pub fn admit_preview(cx:&mut impl JobOutcomeContext,payload:&'a RetainedJobPayload)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::PreviewReady{payload,admission}))}
 /// 📤️ See `README.md`.
 pub fn admit_preview_with_cursor(cx:&mut impl JobOutcomeContext,payload:&'a RetainedJobPayload,cursor_copy_bytes:usize)->Result<Option<Self>,ValueError>{Ok(Self::admit_with_cursor(cx,cursor_copy_bytes)?.map(|admission|Self::PreviewReady{payload,admission}))}
 pub fn admit_checkpoint(cx:&mut impl JobOutcomeContext,state:&'a RetainedJobPayload,applied_progress:u64)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::CheckpointReady{state,applied_progress,admission}))}
 pub fn admit_complete(cx:&mut impl JobOutcomeContext,state:Option<&'a RetainedJobPayload>,output:Option<&'a RetainedJobPayload>)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Complete{state,output,admission}))}
 pub fn admit_cancelled(cx:&mut impl JobOutcomeContext)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Cancelled{admission}))}
 pub fn admit_fault(cx:&mut impl JobOutcomeContext,detail:&'a RetainedJobPayload)->Result<Option<Self>,ValueError>{Ok(Self::admit(cx)?.map(|admission|Self::Fault{detail,admission}))}
 /// 📤️ See `README.md`.
 pub fn admit_fault_with_cursor(cx:&mut impl JobOutcomeContext,detail:&'a RetainedJobPayload,cursor_copy_bytes:usize)->Result<Option<Self>,ValueError>{Ok(Self::admit_with_cursor(cx,cursor_copy_bytes)?.map(|admission|Self::Fault{detail,admission}))}
 pub fn admit_fault_with_cursor_retiring(cx:&mut impl JobOutcomeContext,detail:&'a RetainedJobPayload,cursor_copy_bytes:usize)->Result<Option<Self>,ValueError>{if !cx.original_retirement_active()||detail.stream()!=semio_framework_job_payload::JobPayloadStream::Fault{return Err(ValueError::invariant("fault outcome requires original retirement authority and fault stream"))}Ok(Self::admit_cursor_mode(cx,cursor_copy_bytes,true)?.map(|admission|Self::Fault{detail,admission}))}
 /// ⚠️ See `README.md`.
 pub fn admit_original_fault(operation:OperationId,generation:Generation,grant:RetainedCloneGrant,recipient:&mut RetainedCloneProgress,detail:&'a RetainedJobPayload)->Result<Option<Self>,ValueError>{if !recipient.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fault recipient exceeds its received wallet").with_retained_progress(*recipient))}let demand=Self::admission_demand();if recipient.copied_items==grant.maximum_items||grant.maximum_copy_bytes-recipient.copied_bytes<demand.copy_bytes||grant.maximum_capacity_bytes-recipient.retained_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes-recipient.released_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};*recipient=recipient.checked_add(progress)?;Ok(Some(Self::Fault{detail,admission:JobOutcomeAdmission{progress,operation,generation}}))}

 pub fn admission(&self)->&JobOutcomeAdmission{match self{Self::Yield{admission}|Self::PreviewReady{admission,..}|Self::CheckpointReady{admission,..}|Self::Complete{admission,..}|Self::Cancelled{admission}|Self::Fault{admission,..}=>admission}}
 pub fn is_terminal(&self)->bool{matches!(self,Self::Complete{..}|Self::Cancelled{..}|Self::Fault{..})}
 /// 🧲️ See `README.md`.
 pub fn into_descriptor(self)->JobOutcomeDescriptor{match self{Self::Yield{admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Yield,admission,[None,None]),Self::PreviewReady{payload,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::PreviewReady,admission,[Some(payload),None]),Self::CheckpointReady{state,applied_progress,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::CheckpointReady{applied_progress},admission,[Some(state),None]),Self::Complete{state,output,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Complete,admission,[state,output]),Self::Cancelled{admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Cancelled,admission,[None,None]),Self::Fault{detail,admission}=>JobOutcomeDescriptor::from_original(JobOutcomeKind::Fault,admission,[Some(detail),None])}}
}



