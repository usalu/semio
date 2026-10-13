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
    Complete,
}

pub(crate) struct ActiveMediaExport {
    pub(crate) handle: ArtifactMediaExportHandle,
    pub(crate) port: String,
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) contract: semio_framework::ToolExecutionContract,
    pub(crate) session: Option<semio_framework_job::MountedWorkerJobSession<semio_framework::ErasedToolJob>>,
    pub(crate) session_rejected: Option<crate::reserved_job_session::RefusedWorkerSource<semio_framework::ErasedToolJob>>,
    pub(crate) closing_job: Option<ArtifactReservedToolJob>,
    pub(crate) outcome_pending: bool,
    pub(crate) resume_pending: bool,
    pub(crate) retained_result: Option<Result<ArtifactMediaExportResult,MediaError>>,
    pub(crate) output_credit: ArtifactMediaExportCredit,
    pub(crate) output_chunks: ArtifactOutputChunks,
    pub(crate) completion: ArtifactMediaExportCompletion,
    pub(crate) lease: ToolCancellationLease,
    pub(crate) checkpoint_available: bool,
    pub(crate) applied_progress: u64,
    close_stage: ActiveMediaExportCloseStage,
}

impl ActiveMediaExport {
    pub(crate) fn running_poll(&self) -> ArtifactMediaExportPoll {
        ArtifactMediaExportPoll::Running { applied_progress: self.applied_progress, checkpoint_available: self.checkpoint_available }
    }

    pub(crate) fn retire_nonterminal_outcome(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
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
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
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
        Ok(item)
    }

    pub(crate) fn terminal_is_empty(&self) -> Result<bool, Fault> {
        if self.close_stage!=ActiveMediaExportCloseStage::Complete||self.session.is_some()||self.session_rejected.is_some()||self.outcome_pending||self.closing_job.is_some()||!self.retained_result.is_none()||self.resume_pending{return Ok(false);}
        Err(plugin_sdk_fault("original media result and completion owners require their retained physical authority"))
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if self.close_stage==ActiveMediaExportCloseStage::Live{
            let bytes=size_of::<ActiveMediaExportCloseStage>()+size_of::<bool>();
            if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
            self.lease.cancel();if let Some(session)=self.session.as_mut(){session.begin_close();}
            self.resume_pending=false;self.outcome_pending=false;self.close_stage=ActiveMediaExportCloseStage::Worker;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
        }
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.terminal_is_empty(){let bytes=size_of_val(rejected);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}drop(self.session_rejected.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));}
            let demand=rejected.retirement_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?;
            return original_rejection_lifecycle(demand,grant,"media original rejection awaits worker handback",|grant|{let step=rejected.close_step(grant);(step,rejected.terminal_is_empty())});
        }
        if let Some(session)=self.session.as_mut(){
            if session.terminal_is_empty(){let bytes=size_of_val(session);if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}drop(self.session.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));}
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(semio_framework_job::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"media original worker quote awaits its handback"}),Err(semio_framework_job::WorkerJobDemandError::Refused(error))=>return Err(plugin_sdk_fault(error.to_string()))};
            return original_worker_lifecycle(demand,grant,"media original mounted worker awaits its handback",|grant|{let step=session.close_step(grant);(step,session.terminal_is_empty())});
        }
        if let Some(job)=self.closing_job.as_mut(){
            if !semio_framework_job::InteractiveJob::terminal_is_empty(job){
                let demand=RetirementDemand{copy_bytes:semio_framework_job::InteractiveJob::next_close_copy_byte_demand(job).map_err(|error|plugin_sdk_fault(error.to_string()))?,capacity_bytes:semio_framework_job::InteractiveJob::next_close_capacity_byte_demand(job,grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?,release_bytes:semio_framework_job::InteractiveJob::next_close_release_byte_demand(job).map_err(|error|plugin_sdk_fault(error.to_string()))?,depth:semio_framework_job::InteractiveJob::next_close_depth_demand(job).map_err(|error|plugin_sdk_fault(error.to_string()))?};
                return original_rejection_lifecycle(demand,grant,"media original reserved producer awaits its handback",|grant|{let step=semio_framework_job::InteractiveJob::close_step(job,grant);(step,semio_framework_job::InteractiveJob::terminal_is_empty(job))});
            }
        }
        Ok(PluginLifecycleStep::AwaitingInput{reason:"original media result and completion owners require their retained physical authority"})
    }
}
