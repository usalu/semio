//! 🎞️ Media exports and segmented downloads.
use semio_framework_plugin_decl_media::ArtifactOutputChunks;
use semio_framework_plugin_decl_media::ArtifactMediaExportResult;
use semio_framework_plugin_decl_media::ArtifactMediaExportCompletion;
use semio_framework_plugin_decl_media::ArtifactMediaExportCredit;
use semio_framework_plugin_decl_media::ArtifactMediaExportHandle;
use semio_framework_plugin_decl_media::ArtifactMediaExportPoll;
use semio_framework::{Fault, FaultCode, FaultOrigin};
/// 🎞️ The `Media`/`MediaPayload`/`MediaFingerprint`/`MediaError` value vocabulary backing
/// `ArtifactApp::{media_ports, export_media, import_media, media_fingerprint}` — re-exported so
/// implementers never need a direct `semio-framework-core` dependency just to satisfy this trait.
use semio_framework::MediaError;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use crate::original_rejection_lifecycle;
use crate::original_worker_lifecycle;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJob;
use semio_framework_plugin_operation::ToolCancellationLease;
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveMediaExportCloseStage {
    Live,
    Worker,
    DisposeJob,
    DrainChunks,
    DrainCompletion,
    Lease,
    Complete,
}

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct MediaExportOwnedParts{
 pub handle:ArtifactMediaExportHandle,pub port:String,pub retained_result:Option<Result<ArtifactMediaExportResult,MediaError>>,pub output_credit:ArtifactMediaExportCredit,pub output_chunks:ArtifactOutputChunks,pub completion:ArtifactMediaExportCompletion,
}
#[doc(hidden)]
pub struct ActiveMediaExport{
 pub parts:std::mem::ManuallyDrop<Option<MediaExportOwnedParts>>,
 pub parts_retirement:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<MediaExportOwnedParts>>>,
 pub operation:semio_framework_job::Operation,pub contract:semio_framework::ToolExecutionContract,
 pub session:Option<semio_framework_job_worker::MountedWorkerJobSession<ArtifactReservedToolJob>>,pub session_rejected:Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<ArtifactReservedToolJob>>,
 pub outcome_pending:bool,pub resume_pending:bool,
 pub lease:std::mem::ManuallyDrop<Option<ToolCancellationLease>>,
 pub cancellation_retirement:Option<Box<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>>,
 pub checkpoint_available:bool,pub applied_progress:u64,pub download_pending:bool,pub close_stage:ActiveMediaExportCloseStage,
}
impl std::ops::Deref for ActiveMediaExport{type Target=MediaExportOwnedParts;fn deref(&self)->&Self::Target{self.parts.as_ref().expect("live media operation retains its original publication parts")}}
impl std::ops::DerefMut for ActiveMediaExport{fn deref_mut(&mut self)->&mut Self::Target{self.parts.as_mut().expect("live media operation retains its original publication parts")}}

impl ActiveMediaExport {
    pub fn running_poll(&self) -> ArtifactMediaExportPoll {
        ArtifactMediaExportPoll::Running { applied_progress: self.applied_progress, checkpoint_available: self.checkpoint_available }
    }

    pub fn retire_nonterminal_outcome(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let grant=cx.retained_grant();let demand=RetirementDemand{copy_bytes:if self.resume_pending{size_of::<bool>()}else{0},depth:1,..Default::default()};
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|self.retire_nonterminal_outcome_funded(grant).map(|step|(step,false)).map_err(Into::into))
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
    pub fn retirement_demands(&self, body: usize,canonical:&semio_framework_plugin_operation::ToolCancellationHandle) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if self.close_stage == ActiveMediaExportCloseStage::Live { return Ok(RetirementDemand { copy_bytes: size_of::<ActiveMediaExportCloseStage>() + size_of::<bool>(), ..item }); }
        if let Some(rejected) = self.session_rejected.as_ref() {
            if rejected.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(rejected), ..item }); }
            return rejected.retirement_demands(body);
        }
        if let Some(session) = self.session.as_ref() {
            if self.outcome_pending || !session.terminal_is_empty() { return match session.retirement_demands(body) {
                Ok(demand) => Ok(demand),
                Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => Ok(item),
                Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => Err(error),
            }; }
            if session.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(session), ..item }); }
            return match session.retirement_demands(body) {
                Ok(demand) => Ok(demand),
                Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => Ok(item),
                Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => Err(error),
            };
        }
        if let Some(owner)=self.parts_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(item)}else{owner.selected_demand(body)}}
        if self.parts.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<MediaExportOwnedParts>(),..item})}
        if self.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),..item})}
        if let Some(owner)=self.cancellation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),..item})}else{owner.retirement_demands(canonical,body)}}
        Ok(item)
    }

    pub fn terminal_is_empty(&self)->Result<bool,Fault>{Ok(self.close_stage==ActiveMediaExportCloseStage::Complete&&self.session.is_none()&&self.session_rejected.is_none()&&!self.outcome_pending&&!self.resume_pending&&self.parts.is_none()&&self.parts_retirement.is_none()&&self.lease.is_none()&&self.cancellation_retirement.is_none())}

    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant,canonical:&semio_framework_plugin_operation::ToolCancellationHandle) -> Result<PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
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
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"media original worker quote awaits its handback"}),Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(error.into())};
            return original_worker_lifecycle(cx,demand,grant,"media original mounted worker awaits its handback",|cx|{let step=session.close_step(cx)?;Ok((step,session.terminal_is_empty()))});
        }
        if let Some(owner)=self.parts_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.parts_retirement.take());return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
            let demand=owner.selected_demand(grant.maximum_copy_bytes)?;
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|Ok((PluginLifecycleStep::Progress(owner.step(grant)?.progress()),false)));
        }
        if self.parts.is_some(){
            let demand=RetirementDemand{copy_bytes:size_of::<MediaExportOwnedParts>(),depth:1,..Default::default()};
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,_|{
                let value=self.parts.take().expect("original media parts remain held before admitted control");match semio_framework_value::retirement::controlled::ControlledRetirement::new(value){Ok(owner)=>*self.parts_retirement=Some(owner),Err((error,value))=>{*self.parts=Some(value);return Err(error.into())}}
                Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))
            });
        }
        if let Some(owner)=self.cancellation_retirement.as_ref(){
            let terminal=owner.terminal_is_empty();let demand=if terminal{RetirementDemand{release_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()}}else{owner.retirement_demands(canonical,grant.maximum_copy_bytes)?};
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{let progress=if terminal{semio_framework_plugin_operation::original_lease_cursor_release(&mut self.cancellation_retirement,grant)?}else{self.cancellation_retirement.as_mut().expect("quoted original cancellation cursor").close_step(canonical,grant)?.progress()};Ok((PluginLifecycleStep::Progress(progress),false))});
        }
        if self.lease.is_some(){
            let demand=RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()};
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{let(owner,progress)=semio_framework_plugin_operation::original_lease_cursor_admit(&mut self.lease,grant)?;self.cancellation_retirement=owner;Ok((PluginLifecycleStep::Progress(progress),false))});
        }
        self.close_stage=ActiveMediaExportCloseStage::Complete;Ok(PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
}

/// 📥️ Terminal output retains partial metadata and actual received roots until paid publication.
#[doc(hidden)]
pub struct MediaDownloadPreparation{
 pub operation_id:u64,pub source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation,usize),
 pub document_identity:(usize,usize),pub mime_identity:(usize,usize),pub chunks_identity:usize,pub coordinates:(u32,semio_framework_job::RevisionId,semio_framework_job::Generation),
 pub buffers:std::mem::ManuallyDrop<Option<[Vec<u8>;2]>>,pub buffers_close:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<[Vec<u8>;2]>>>,pub stage:usize,
 pub handle:std::mem::ManuallyDrop<Option<ArtifactMediaExportHandle>>,pub handle_close:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactMediaExportHandle>>>,
 pub output:std::mem::ManuallyDrop<Option<semio_framework_plugin_decl_media::ArtifactDownloadOutput>>,pub output_close:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin_decl_media::ArtifactDownloadOutput>>>,pub published:bool,
}
impl MediaDownloadPreparation{
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.output.is_some()||self.output_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.output,&self.output_close,body)}
  if self.handle.is_some()||self.handle_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.handle,&self.handle_close,body)}
  if self.buffers.is_some()||self.buffers_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.buffers,&self.buffers_close,body)}
  Ok(RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
  if self.output.is_some()||self.output_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.output,&mut self.output_close,grant).map(Some)}
  if self.handle.is_some()||self.handle_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.handle,&mut self.handle_close,grant).map(Some)}
  if self.buffers.is_some()||self.buffers_close.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.buffers,&mut self.buffers_close,grant).map(Some)}Ok(None)
 }
}

/// ♻️ Complete segmented output fields retain their actual controlled physical close owner.
#[doc(hidden)]
pub struct MediaDownloadRetirement{pub original:std::mem::ManuallyDrop<Option<semio_framework_plugin_decl_media::ArtifactDownloadOutput>>,pub active:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_plugin_decl_media::ArtifactDownloadOutput>>>}
impl MediaDownloadRetirement{
 pub fn pending(output:semio_framework_plugin_decl_media::ArtifactDownloadOutput)->Self{Self{original:std::mem::ManuallyDrop::new(Some(output)),active:std::mem::ManuallyDrop::new(None)}}
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.original,&self.active,body)}
 pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{let demand=self.retirement_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.original,&mut self.active,grant).map(|step|(PluginLifecycleStep::Progress(step.progress()),self.terminal_is_empty())).map_err(Into::into))}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.active.is_none()}
}
impl Drop for MediaDownloadRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"segmented output requires its complete paid original physical return")}}
