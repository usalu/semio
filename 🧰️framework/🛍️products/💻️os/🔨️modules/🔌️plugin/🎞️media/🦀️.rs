//! 🎞️ Media exports and segmented downloads.
use crate::media_contract::ArtifactOutputChunks;
use crate::media_contract::ArtifactMediaExportResult;
use crate::media_contract::ArtifactMediaExportCompletion;
use crate::media_contract::ArtifactMediaExportCredit;
use crate::media_contract::ArtifactMediaExportHandle;
use crate::media_contract::ArtifactMediaExportPoll;
use semio_framework::{Fault, FaultCode, FaultOrigin};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::MediaForm;
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
use semio_framework::MediaError;
/// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
/// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
/// without a direct `semio-framework-core` dependency.
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::MediaClass;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use crate::bounded_ownership::PluginLifecycleStep;
use crate::document_ingress::original_rejection_lifecycle;
use crate::document_ingress::original_worker_lifecycle;
use crate::tool_contract::ArtifactReservedToolJob;
use crate::sdk_fault::plugin_sdk_fault;
use crate::tool_cancellation::ToolCancellationLease;
#[cfg(test)]
use crate::store_lifecycle::ArtifactSnapshotDisposer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActiveMediaExportCloseStage {
    Live,
    Worker,
    DisposeJob,
    DrainChunks,
    DrainCompletion,
    Lease,
    Complete,
}

#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct MediaExportOwnedParts{
 pub(crate) handle:ArtifactMediaExportHandle,pub(crate) port:String,pub(crate) retained_result:Option<Result<ArtifactMediaExportResult,MediaError>>,pub(crate) output_credit:ArtifactMediaExportCredit,pub(crate) output_chunks:ArtifactOutputChunks,pub(crate) completion:ArtifactMediaExportCompletion,
}
pub(crate) struct ActiveMediaExport{
 pub(crate) parts:std::mem::ManuallyDrop<Option<MediaExportOwnedParts>>,
 pub(crate) parts_retirement:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<MediaExportOwnedParts>>>,
 pub(crate) operation:semio_framework_job::Operation,pub(crate) contract:semio_framework::ToolExecutionContract,
 pub(crate) session:Option<semio_framework_job::MountedWorkerJobSession<semio_framework::ErasedToolJob>>,pub(crate) session_rejected:Option<crate::reserved_job_session::RefusedWorkerSource<semio_framework::ErasedToolJob>>,pub(crate) closing_job:Option<ArtifactReservedToolJob>,
 pub(crate) outcome_pending:bool,pub(crate) resume_pending:bool,
 pub(crate) lease:std::mem::ManuallyDrop<Option<ToolCancellationLease>>,
 pub(crate) cancellation_retirement:Option<Box<crate::typed_operation::ToolCancellationLeaseRetirement>>,
 pub(crate) checkpoint_available:bool,pub(crate) applied_progress:u64,pub(crate) close_stage:ActiveMediaExportCloseStage,
}
impl std::ops::Deref for ActiveMediaExport{type Target=MediaExportOwnedParts;fn deref(&self)->&Self::Target{self.parts.as_ref().expect("live media operation retains its original publication parts")}}
impl std::ops::DerefMut for ActiveMediaExport{fn deref_mut(&mut self)->&mut Self::Target{self.parts.as_mut().expect("live media operation retains its original publication parts")}}

impl ActiveMediaExport {
    pub(crate) fn running_poll(&self) -> ArtifactMediaExportPoll {
        ArtifactMediaExportPoll::Running { applied_progress: self.applied_progress, checkpoint_available: self.checkpoint_available }
    }

    pub(crate) fn retire_nonterminal_outcome(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let grant=cx.retained_grant();let demand=RetirementDemand{copy_bytes:if self.resume_pending{size_of::<bool>()}else{0},depth:1,..Default::default()};
        crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|self.retire_nonterminal_outcome_funded(grant).map(|step|(step,false)).map_err(Into::into))
    }
    fn retire_nonterminal_outcome_funded(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.outcome_pending {
            let session = self.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-session-missing"), "media outcome acknowledgement lost its mounted session"))?;
            let step = session.acknowledge_checked_out_outcome(grant);
            if matches!(step, RetainedCloneStep::Complete(_)) { self.outcome_pending = false; self.resume_pending = true; }
            return Ok(PluginLifecycleStep::retained(step, false));
        }
        if self.resume_pending {
            if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<bool>()||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            let session = self.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-session-missing"), "media outcome handback lost its mounted session"))?;
            if session.resume().is_err(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"media outcome handback awaits its original mounted session"});}
            self.resume_pending=false;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}));
        }
        Ok(PluginLifecycleStep::Progress(Default::default()))
    }
    /// 📏️ Quotes the exact next [`Self::close_step`] turn, one owner at a time in the same order.
    pub(crate) fn retirement_demands(&self, body: usize,canonical:&crate::tool_cancellation::ToolCancellationHandle) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if self.close_stage == ActiveMediaExportCloseStage::Live { return Ok(RetirementDemand { copy_bytes: size_of::<ActiveMediaExportCloseStage>() + size_of::<bool>(), ..item }); }
        if let Some(rejected) = self.session_rejected.as_ref() {
            if rejected.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(rejected), ..item }); }
            return rejected.retirement_demands(body);
        }
        if let Some(session) = self.session.as_ref() {
            if self.outcome_pending || !session.terminal_is_empty() { return match session.retirement_demands(body) {
                Ok(demand) => Ok(demand),
                Err(semio_framework_job::WorkerJobDemandError::Contention(_)) => Ok(item),
                Err(semio_framework_job::WorkerJobDemandError::Refused(error)) => Err(error),
            }; }
            if session.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(session), ..item }); }
            return match session.retirement_demands(body) {
                Ok(demand) => Ok(demand),
                Err(semio_framework_job::WorkerJobDemandError::Contention(_)) => Ok(item),
                Err(semio_framework_job::WorkerJobDemandError::Refused(error)) => Err(error),
            };
        }
        if let Some(job) = self.closing_job.as_ref() {
            if !semio_framework_job::InteractiveJob::terminal_is_empty(job) {
                return Ok(RetirementDemand { copy_bytes: semio_framework_job::InteractiveJob::next_close_copy_byte_demand(job)?, capacity_bytes: semio_framework_job::InteractiveJob::next_close_capacity_byte_demand(job, body)?, release_bytes: semio_framework_job::InteractiveJob::next_close_release_byte_demand(job)?, depth: semio_framework_job::InteractiveJob::next_close_depth_demand(job)? });
            }
        }
        if self.closing_job.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ArtifactReservedToolJob>>(),..item})}
        if let Some(owner)=self.parts_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(item)}else{owner.selected_demand(body)}}
        if self.parts.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<MediaExportOwnedParts>(),..item})}
        if self.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<crate::typed_operation::ToolCancellationLeaseRetirement>(),..item})}
        if let Some(owner)=self.cancellation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<crate::typed_operation::ToolCancellationLeaseRetirement>(),..item})}else{owner.retirement_demands(canonical,body)}}
        Ok(item)
    }

    pub(crate) fn terminal_is_empty(&self)->Result<bool,Fault>{Ok(self.close_stage==ActiveMediaExportCloseStage::Complete&&self.session.is_none()&&self.session_rejected.is_none()&&!self.outcome_pending&&self.closing_job.is_none()&&!self.resume_pending&&self.parts.is_none()&&self.parts_retirement.is_none()&&self.lease.is_none()&&self.cancellation_retirement.is_none())}

    pub(crate) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant,canonical:&crate::tool_cancellation::ToolCancellationHandle) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if self.close_stage==ActiveMediaExportCloseStage::Live{
            let bytes=size_of::<ActiveMediaExportCloseStage>()+size_of::<bool>();
            if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            if let Some(lease)=self.lease.as_ref(){lease.cancel();}if let Some(session)=self.session.as_mut(){session.begin_close();}
            self.resume_pending=false;self.outcome_pending=false;self.close_stage=ActiveMediaExportCloseStage::Worker;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
        }
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.terminal_is_empty(){let bytes=size_of_val(rejected);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}drop(self.session_rejected.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));}
            let demand=rejected.retirement_demands(grant.maximum_copy_bytes)?;
            return original_rejection_lifecycle(cx,demand,grant,"media original rejection awaits worker handback",|cx|{let step=rejected.close_step(cx)?;Ok((step,rejected.terminal_is_empty()))});
        }
        if let Some(session)=self.session.as_mut(){
            if session.terminal_is_empty(){let bytes=size_of_val(session);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}drop(self.session.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));}
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(semio_framework_job::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"media original worker quote awaits its handback"}),Err(semio_framework_job::WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            return original_worker_lifecycle(cx,demand,grant,"media original mounted worker awaits its handback",|cx|{let step=session.close_step(cx)?;Ok((step,session.terminal_is_empty()))});
        }
        if let Some(job)=self.closing_job.as_mut(){
            if !semio_framework_job::InteractiveJob::terminal_is_empty(job){
                let demand=RetirementDemand{copy_bytes:semio_framework_job::InteractiveJob::next_close_copy_byte_demand(job)?,capacity_bytes:semio_framework_job::InteractiveJob::next_close_capacity_byte_demand(job,grant.maximum_copy_bytes)?,release_bytes:semio_framework_job::InteractiveJob::next_close_release_byte_demand(job)?,depth:semio_framework_job::InteractiveJob::next_close_depth_demand(job)?};
                return original_rejection_lifecycle(cx,demand,grant,"media original reserved producer awaits its handback",|cx|{let step=semio_framework_job::InteractiveJob::close_step(job,cx)?;Ok((step,semio_framework_job::InteractiveJob::terminal_is_empty(job)))});
            }
        }
        if self.closing_job.is_some(){let bytes=size_of::<Option<ArtifactReservedToolJob>>();if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()))}drop(self.closing_job.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))}
        if let Some(owner)=self.parts_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.parts_retirement.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
            let demand=owner.selected_demand(grant.maximum_copy_bytes)?;
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|Ok((PluginLifecycleStep::Progress(owner.step(grant)?.progress()),false)));
        }
        if self.parts.is_some(){
            let demand=RetirementDemand{copy_bytes:size_of::<MediaExportOwnedParts>(),depth:1,..Default::default()};
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,_|{
                let value=self.parts.take().expect("original media parts remain held before admitted control");match semio_framework_value::retirement::controlled::ControlledRetirement::new(value){Ok(owner)=>*self.parts_retirement=Some(owner),Err((error,value))=>{*self.parts=Some(value);return Err(error.into())}}
                Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))
            });
        }
        if let Some(owner)=self.cancellation_retirement.as_ref(){
            let terminal=owner.terminal_is_empty();let demand=if terminal{RetirementDemand{release_bytes:size_of::<crate::typed_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()}}else{owner.retirement_demands(canonical,grant.maximum_copy_bytes)?};
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{let progress=if terminal{crate::typed_operation::original_lease_cursor_release(&mut self.cancellation_retirement,grant)?}else{self.cancellation_retirement.as_mut().expect("quoted original cancellation cursor").close_step(canonical,grant)?.progress()};Ok((PluginLifecycleStep::Progress(progress),false))});
        }
        if self.lease.is_some(){
            let demand=RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<crate::typed_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()};
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{let(owner,progress)=crate::typed_operation::original_lease_cursor_admit(&mut self.lease,grant)?;self.cancellation_retirement=owner;Ok((PluginLifecycleStep::Progress(progress),false))});
        }
        self.close_stage=ActiveMediaExportCloseStage::Complete;Ok(PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
}
