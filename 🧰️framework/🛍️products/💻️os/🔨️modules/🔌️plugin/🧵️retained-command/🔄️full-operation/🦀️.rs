//! 🔄️ Mounted typed command full operations and their job.
use semio_framework_value::DslValue;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::SchemaId;
use semio_framework::{Fault, FaultCode, FaultOrigin};
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::ArtifactRef;
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::{os_store::store_schema::ArtifactCommand, os_store::store_schema::HistoryLane, Mutation};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_ui_locale::LocalizedLabel;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_async::PublicationClaimHandle;
use std::mem::ManuallyDrop;
#[cfg(test)]
use crate::store_publication::ChildPublicationResultV1;
#[cfg(test)]
use crate::app_emission::ChildEmit;
#[cfg(test)]
use crate::command_admission::original_latest_key::ToolLatestWinsKeyCopy;
use crate::bounded_ownership::plugin_grant_funds;
use crate::app_contract::ArtifactApp;
use crate::app_emission::ActionMeta;
use crate::app_views::InteractionHoverState;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::artifact_instance::VcsArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::bounded_ownership::plugin_retirement_fault;
use crate::bounded_ownership::plugin_text_close;
use crate::bounded_ownership::plugin_text_retirement_demand;
use crate::bounded_ownership::plugin_turn_admitted;
use crate::bounded_ownership::plugin_typed_owner_demand;
use crate::bounded_ownership::plugin_typed_owner_step;
use crate::child_view::ChildContentView;
use crate::history_view::HistoryView;
use crate::media_contract::ArtifactOutputChunks;
use crate::app_views::PeerPresenceRoot;
use crate::tool_contract::ArtifactReservedToolJob;
use crate::tool_contract::ArtifactReservedToolJobState;
use crate::reserved_jobs::QualifiedBoundedFirstStepProof;
use crate::tool_contract::ToolOwnerWitness;
use crate::sdk_fault::plugin_sdk_fault;
use crate::store_publication::MountedTypedCommandFullOperationStage;
use crate::store_publication::MountedWorkerFaultCapture;
use crate::store_publication::PendingArtifactStorePublication;
use crate::store_publication::PendingChildGroupPublication;
use crate::store_publication::PendingChildGroupPublicationPhase;
use crate::store_publication::advance_original_fault_capture;
use crate::store_publication::original_fault_capture_demands;
use crate::store_publication::original_reserved_arc_frame_bytes;
use crate::tool_cancellation::ToolCancellationHandle;
use crate::tool_cancellation::ToolCancellationLease;
use crate::tool_cancellation::ToolCancellationState;
use crate::tool_cancellation::ToolDocumentCancellationScope;
use crate::tool_cancellation::ToolKeyedDocumentCancellationScope;
use crate::tool_contract::ArtifactBoundedToolFault;
use crate::tool_contract::ArtifactToolCompletion;
use crate::tool_contract::ArtifactToolCompletionValue;
use crate::tool_contract::ArtifactToolPublicationLane;
use crate::tool_contract::ArtifactToolRawInput;
use crate::tool_contract::BOUNDED_FIRST_STEP_FACTORY;
use crate::tool_contract::TYPED_OPERATION_FAULT_PAGE_BYTES;
use crate::tool_contract::TypedOperationResultLane;
use crate::tool_jobs::TypedOperationResultPage;
use crate::tool_jobs::TypedOperationResultToken;
use crate::tool_jobs::operation_progress;
#[cfg(test)]
use crate::window_kits::AppActionRegistry;
#[cfg(test)]
use crate::plugin_app::PluginApp;
#[cfg(test)]
use crate::app_emission::ChildEmitGenesis;
#[cfg(test)]
use crate::app_emission::Emit;
#[cfg(test)]
use crate::app_emission::EphemeralEmit;
#[cfg(test)]
use crate::app_testing::artifact_app_laws;
#[cfg(test)]
use crate::artifact_instance::MAINTENANCE_STAGES;
#[cfg(test)]
use crate::artifact_instance::typed_operation_document_is_fresh;
#[cfg(test)]
use crate::command_admission::ToolLatestWinsRegistry;
#[cfg(test)]
use crate::reserved_jobs::QualifiedToolProof;
#[cfg(test)]
use crate::reserved_jobs::plugin_job_yield_once;
#[cfg(test)]
use crate::tool_cancellation::ArtifactDocumentAuthority;
#[cfg(test)]
use crate::tool_cancellation::TOOL_CANCELLATION_SLOTS;
#[cfg(test)]
use crate::tool_cancellation::ToolOperationKey;
#[cfg(test)]
use crate::tool_contract::TYPED_OPERATION_FAULT_BYTES;
#[cfg(test)]
use crate::tool_jobs::TYPED_OPERATION_RESULT_PAGE_BYTES;



pub(crate) struct MountedTypedCommandFullOperation<A: ArtifactApp> {
    pub(crate) verb: String,
    pub(crate) meta: ActionMeta,
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub(crate) canonical_revision: [u8; 32],
    pub(crate) artifact_generation: u64,
    pub(crate) config_generation: u64,
    pub(crate) draft_generation: u64,
    pub(crate) presence_generation: u64,
    pub(crate) transient_generation: u64,
    pub(crate) window_config_authority: Option<super::window_config::WindowConfigAuthority>,
    pub(crate) window_transient_authority: Option<super::window_transient::WindowTransientAuthority>,
    pub(crate) publication_lanes: &'static [ArtifactToolPublicationLane],
    pub(crate) session: Option<semio_framework_job::MountedWorkerJobSession<semio_framework::ErasedToolJob>>,
    pub(crate) session_rejected: Option<crate::reserved_job_session::RefusedWorkerSource<semio_framework::ErasedToolJob>>,
    pub(crate) reserved_producer: Option<ArtifactReservedToolJob>,
    pub(crate) completion: Option<ArtifactToolCompletion<A>>,
    pub(crate) completion_retirement:Option<ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub(crate) publication_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub(crate) output_retirement:Option<ControlledRetirement<ArtifactOutputChunks>>,
    pub(crate) raw_input: Option<ArtifactToolRawInput>,
    pub(crate) output_chunks: Option<ArtifactOutputChunks>,
    pub(crate) cancellation_lease: Option<ToolCancellationLease>,
    pub(crate) identity: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    pub(crate) identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress,
    pub(crate) worker_semantic_pending: bool,
    pub(crate) worker_outcome_pending: bool,
    pub(crate) worker_fault_capture: Option<MountedWorkerFaultCapture>,
    pub(crate) terminal_seen: bool,
    pub(crate) worker_resume_pending: bool,
    pub(crate) publication: Option<ArtifactToolCompletionValue<A>>,
    pub(crate) pending_artifact_publication: Option<PendingArtifactStorePublication<A>>,
    pub(crate) pending_publication_outcome:PendingPublicationOutcome,
    pub(crate) pending_window_config_receipt:Option<store::os_store::presence::LaneItemReceipt>,
    pub(crate) cancellation_retirement:Option<Box<ToolCancellationLeaseRetirement>>,
    pub(crate) pending_child_publication: Option<PendingChildGroupPublication<A>>,
    pub(crate) owned_child_group: Option<u64>,
    pub(crate) owned_child_committed: bool,
    pub(crate) owned_child_result_pending: bool,
    pub(crate) captured_child_content: Option<std::sync::Arc<ChildContentView>>,
    pub(crate) captured_child_content_generation: u64,
    pub(crate) result_page: Option<TypedOperationResultPage>,
    pub(crate) result_page_presented: bool,
    pub(crate) result_sequence: u32,
    /// 🧮️ The publication ladder's OWN monotone progress: how many publication units actually
    /// moved this operation's pending store publication, counted by the store's checkpoint
    /// rather than by the unit having been spent. A healthy ladder that folds one mutation per
    /// unit raises it on every unit; an owner that cannot advance never raises it at all, which
    /// is exactly the term [`Self::typed_operation_stall_witness`]'s three facts were missing.
    pub(crate) publication_progress: u64,
    /// 🧮️ The last checkpoint the pending publication answered with, so a unit that repeats it
    /// verbatim is recognised as no movement at all instead of as progress.
    pub(crate) publication_checkpoint: Option<store::os_store::one_item::ArtifactStoreOneItemCheckpoint>,
    pub(crate) publication_ownership_progress: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub(crate) original_retirement_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub(crate) actor_capture: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
    pub(crate) publication_attempt: u8,
    pub(crate) ui_pending: bool,
    pub(crate) progress: Option<u64>,
    pub(crate) progress_pending: bool,
    pub(crate) user_cancel_requested: bool,
    pub(crate) published_artifact: bool,
    pub(crate) published_config: bool,
    /// 🪟 The per-window config lane published. A `View` whose only durable emission is this lane
    /// is session view state, and [`Self::record_settled_typed_operation_command`] leaves it out
    /// of the artifact command history — the same exception [`Self::dispatch_emit_inner`] makes.
    pub(crate) published_window_config: bool,
    pub(crate) command_logged: bool,
    pub(crate) interaction_revalidated: bool,
    pub(crate) terminal_fault: Option<ArtifactBoundedToolFault>,
    pub(crate) retained_close_fault: Option<Fault>,
    pub(crate) retained_close_fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub(crate) retained_close_fault_refusal:Option<ValueError>,
    pub(crate) stage: MountedTypedCommandFullOperationStage,
}

include!("../../♻️cancellation/📦️lease/🦀️.rs");

include!("../🪟️mounted/♻️frontier/🦀️.rs");


/// 🪢️ Returns the worker's original cancellation ancestor alias against the lease root the operation still owns, since that root can never become unique while the operation retains it.
fn return_session_cancel_alias(session: &semio_framework_job::MountedWorkerJobSession<semio_framework::ErasedToolJob>, lease: &Option<ToolCancellationLease>, grant: RetainedCloneGrant) -> Option<Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure>> {
    let lease = lease.as_ref()?;
    matches!(session.has_original_cancel_alias_witness(&lease.token), Ok(true)).then(|| match session.return_original_cancel_alias_step(&lease.token, grant) {
        Ok(Some(step)) => Ok(PluginLifecycleStep::Progress(step.progress())),
        Ok(None) | Err(semio_framework_job::WorkerJobDemandError::Contention(_)) => Ok(PluginLifecycleStep::AwaitingInput { reason: "typed operation original cancellation alias awaits its issuing root" }),
        Err(semio_framework_job::WorkerJobDemandError::Refused(error)) => Err(error.into()),
    })
}

impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
    /// 🚨️ The retained refusal of an owner that has no retirement path at all: neither a preparation that was refused at birth nor a refused fault close can ever succeed on retry, so the operation would otherwise stay pending without a word.
    pub(crate) fn retained_structural_refusal(&self) -> Option<String> {
        if let Some(fault) = self.retained_close_fault.as_ref() {
            return Some(format!("retains its original close fault: {}: {}", fault.code.0, fault.message));
        }
        let refusal = self.pending_artifact_publication.as_ref().and_then(|publication| publication.preparation_refusal()).or(self.retained_close_fault_refusal.as_ref())?;
        (refusal.kind == semio_framework_value::ValueRefusalKind::UnsupportedOwner).then(|| format!("is refused by an owner without a retirement path: {refusal}"))
    }

    /// 🎭️ Yields the actual actor capture receipt before admitting any durable mutation input.
    pub(crate) fn admit_actor_capture(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
        if self.actor_capture.is_some(){return Ok(false);}
        if let Some(receipt)=semio_framework_plugin_mounted_owner::actor_capture::admit_actor_capture(&mut self.actor_capture,&self.meta.actor,grant).map_err(|error|plugin_sdk_fault(error.to_string()))?{
            assert!(self.publication_ownership_progress.is_none(),"mounted actor cannot overwrite its original unreturned receipt");
            self.publication_ownership_progress=Some((grant,receipt));
        }
        Ok(true)
    }

    /// 🧮️ Records what the store answered for ONE publication unit. The checkpoint carries the
    /// store's own cursor, completed items and completed bytes; a unit that repeats the
    /// previous checkpoint verbatim moved nothing and raises no progress, so an owner that can
    /// never advance still runs its streak up to
    /// [`TYPED_OPERATION_STALL_FAULT_CEILING`], while a ladder folding one mutation per unit
    /// resets that streak every unit however many mutations the edit carries.
    pub(crate) fn note_publication_checkpoint(&mut self, checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint) {
        if self.publication_checkpoint != Some(checkpoint) {
            self.publication_checkpoint = Some(checkpoint);
            self.publication_progress = self.publication_progress.saturating_add(1);
        }
    }

    /// 🚫️ A rejected worker admission never ran the job, so no completion value will ever
    /// arrive: publication would spin on `completion.take() == None` for as long as the host
    /// keeps advancing it. The operation is terminal the moment its session is refused —
    /// cancel the lease so the next publication unit mints the exact fault page.
    pub(crate) fn drive_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        if self.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted worker requires its original source identity independently of its registry record").into())}
        let grant=cx.retained_grant();let demand=if cx.is_cancelled()&&self.session_rejected.as_ref().is_some_and(|source|source.admission_pending()){self.worker_retirement_demands(grant.maximum_copy_bytes)?}else{self.worker_step_demands(grant.maximum_copy_bytes)?};
        crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        if self.stage != MountedTypedCommandFullOperationStage::Worker {return Ok(PluginLifecycleStep::Blocked{reason:"typed command awaits its bounded revision-validated publication or ACK turn"});}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.admission_pending()&&!cx.is_cancelled(){
                let config=semio_framework_job::BatchDriveConfig{retained:grant,site:"plugin_framework_reserved_route",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let before=cx.retained_progress();
                if let Some(session)=rejected.admit_pending(config,cx)?{self.session=Some(session);drop(self.session_rejected.take());}
                let after=cx.retained_progress();
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}));
            }
            if rejected.terminal_is_empty(){
                let bytes=size_of_val(rejected);
                if grant.maximum_copy_bytes<bytes{return Ok(PluginLifecycleStep::Progress(Default::default()));}
                drop(self.session_rejected.take());self.stage=MountedTypedCommandFullOperationStage::Publishing;
                self.terminal_fault.get_or_insert_with(||ArtifactBoundedToolFault::from_fault(&Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.admission-capacity"),"typed operation worker session admission was refused by the process-wide session capacity")));
                if let Some(lease)=self.cancellation_lease.as_ref(){lease.cancel();}
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
            }
            let demand=rejected.retirement_demands(grant.maximum_copy_bytes)?;
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{let step=rejected.close_step(cx)?.admit(grant,rejected.terminal_is_empty())?;Ok((match step{
                semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>PluginLifecycleStep::Progress(progress),
                semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason:"typed operation rejected-session close awaits its worker handback"},

            },false))}).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        let session=self.session.as_mut().ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.typed-operation-session"),"typed operation lost its persistent worker session before publication"))?;
        if self.worker_fault_capture.is_some(){
            let detail=match session.checked_out_outcome(){
                Ok(Some(semio_framework_job::JobOutcomeView::Fault{detail,..}))=>detail,
                _=>return Ok(PluginLifecycleStep::Blocked{reason:"original bounded capture lost its checked-out fault descriptor"}),
            };
            let step=advance_original_fault_capture(&mut self.worker_fault_capture,detail,&mut self.terminal_fault,grant);
            if self.worker_fault_capture.is_none()&&self.terminal_fault.is_some(){self.finish_original_worker_fault_capture();}
            return Ok(step);
        }
        if self.worker_outcome_pending{
            let step=session.acknowledge_checked_out_outcome(grant);
            if matches!(step,RetainedCloneStep::Complete(_)){self.worker_outcome_pending=false;self.worker_resume_pending=true;}
            return Ok(PluginLifecycleStep::retained(step,false));
        }
        if self.worker_resume_pending{
            if self.terminal_seen{session.begin_close();}else if session.resume().is_err(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation outcome handback awaits its original worker session"});}
            self.worker_resume_pending=false;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        if self.terminal_seen{
            if session.terminal_is_empty(){
                drop(self.session.take());self.stage=MountedTypedCommandFullOperationStage::Publishing;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
            }
            if let Some(step)=return_session_cancel_alias(session,&self.cancellation_lease,grant){return step.map_err(semio_framework_job::OriginalTurnFailure::from);}
            let demand=match session.retirement_demands(grant.maximum_copy_bytes){
                Ok(demand)=>demand,
                Err(semio_framework_job::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation original worker close quote awaits its handback"}),
                Err(semio_framework_job::WorkerJobDemandError::Refused(error))=>return Err(error.into())
            };
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{let step=session.close_step(cx)?;Ok((match step{
                semio_framework_job::WorkerJobCloseStep::Pending{progress}if progress.fits(grant)=>PluginLifecycleStep::Progress(progress),
                semio_framework_job::WorkerJobCloseStep::Complete{progress}if progress.fits(grant)&&session.terminal_is_empty()=>PluginLifecycleStep::Progress(progress),
                semio_framework_job::WorkerJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted session close awaits its worker handback"},

                _=>return Err(plugin_sdk_fault("typed operation worker violated its original grant or terminal witness").with_retained_progress(step.progress()).into())
            },false))}).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.worker_semantic_pending{
            let outcome=match session.checked_out_outcome(){
                Ok(Some(original))=>original,
                Ok(None)=>{self.worker_semantic_pending=false;self.worker_resume_pending=true;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));},
                Err(_)=>return Ok(PluginLifecycleStep::Blocked{reason:"typed operation original checked-out descriptor cannot be borrowed"}),
            };
            if let semio_framework_job::JobOutcomeView::Fault{detail,..}=&outcome{
                let step=advance_original_fault_capture(&mut self.worker_fault_capture,detail,&mut self.terminal_fault,grant);
                if self.worker_fault_capture.is_none()&&self.terminal_fault.is_some(){self.finish_original_worker_fault_capture();}
                return Ok(step);
            }
            self.terminal_seen=outcome.is_terminal();
            match outcome{
                semio_framework_job::JobOutcomeView::CheckpointReady{applied_progress,..}=>{let next=self.progress.unwrap_or(0).max(applied_progress);self.progress_pending|=self.progress!=Some(next);self.progress=Some(next);},
                semio_framework_job::JobOutcomeView::Cancelled{..}=>if let Some(lease)=self.cancellation_lease.as_ref(){lease.cancel();},
                _=>{}
            }
            if self.terminal_seen&&self.progress.is_some(){self.progress_pending=true;}
            self.worker_semantic_pending=false;self.worker_outcome_pending=true;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        match session.pump_one_for_interactive_turn(cx){
            Ok(semio_framework_job::WorkerJobPoll::Outcome|semio_framework_job::WorkerJobPoll::Terminal)=>{
                let Some((issued,progress))=session.take_checked_out_retained_step_receipt()else{return Ok(PluginLifecycleStep::Blocked{reason:"typed operation original worker receipt awaits its sole recipient"});};
                self.worker_semantic_pending=true;
                if !progress.fits(issued)||!progress.fits(grant)||issued.maximum_items>grant.maximum_items||issued.maximum_copy_bytes>grant.maximum_copy_bytes||issued.maximum_capacity_bytes>grant.maximum_capacity_bytes||issued.maximum_release_bytes>grant.maximum_release_bytes||issued.maximum_depth>grant.maximum_depth{return Err(plugin_sdk_fault("typed operation original worker receipt exceeded its received turn").with_retained_progress(progress).into());}
                if matches!(session.checked_out_outcome(),Ok(None))&&session.resume().is_ok(){self.worker_semantic_pending=false;}
                Ok(PluginLifecycleStep::Progress(progress))
            }
            Ok(semio_framework_job::WorkerJobPoll::Submitted|semio_framework_job::WorkerJobPoll::Rejected)=>Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),
            Ok(_)=>Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted worker awaits its next outcome"}),
            Err(failure)=>Err(failure)

        }
    
            })()?;Ok((step,self.terminal_is_empty()))
        }).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    /// 🔔️ Exposes the captured canonical wire only after every original fault byte has been funded.
    fn finish_original_worker_fault_capture(&mut self) {
        self.terminal_seen=true;self.worker_semantic_pending=false;self.worker_outcome_pending=true;
        if self.progress.is_some(){self.progress_pending=true;}
        if let Some(lease)=self.cancellation_lease.as_ref(){lease.cancel();}
    }

    pub(crate) fn next_token(&self) -> TypedOperationResultToken {
        TypedOperationResultToken { receiver: self.meta.instance_id, operation: self.operation.operation.0, generation: self.operation.generation.0, sequence: self.result_sequence, attempt: 1 }
    }

    pub(crate) fn queue_page(&mut self, page: TypedOperationResultPage) -> Result<(), Fault> {
        if self.stage != MountedTypedCommandFullOperationStage::Publishing || self.result_page.is_some() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-state"), "typed operation can retain exactly one unacknowledged result page"));
        }
        self.result_page = Some(page);
        self.result_page_presented = false;
        self.stage = MountedTypedCommandFullOperationStage::AwaitingAck;
        Ok(())
    }

    pub(crate) fn reject_cancelled_publication(&mut self) -> Result<bool, Fault> {
        if self.stage != MountedTypedCommandFullOperationStage::Publishing {
            return Ok(false);
        }
        let lease = self.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("typed-operation publication lacks its exact cancellation lease"))?;
        if !lease.handle.publication_scope.is_cancelled() && !lease.document_claim.as_ref().is_some_and(|claim| claim.is_cancelled()) && !lease.publication_claim.is_cancelled() && !lease.token.is_cancelled_now() {
            return Ok(false);
        }
        if self.publication.is_none() {
            if let Some(completion) = self.completion.as_ref() {
                self.publication = completion.take()?;
            }
        }
        if let Some(pending) = self.pending_artifact_publication.as_mut() {
            pending.begin_close();
        }
        if let Some(pending) = self.pending_child_publication.as_mut() {
            pending.begin_close();
        }
        self.ui_pending = false;
        let page = match self.terminal_fault.as_ref() {
            Some(fault) => {
                let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
                let len = fault.framed_page_bytes(&mut framed);
                TypedOperationResultPage::try_new(self.next_token(), TypedOperationResultLane::Fault, &framed[..len])?
            }
            None => {
                let lane = operation_progress::cancellation_result_lane(self.user_cancel_requested, false);
                let detail: &[u8] = if lane == TypedOperationResultLane::Terminal { b"" } else { b"typed-operation cancelled before its next publication unit" };
                TypedOperationResultPage::try_new(self.next_token(), lane, detail)?
            }
        };
        self.queue_page(page)?;
        Ok(true)
    }

    pub(crate) fn take_result_page(&mut self) -> Option<TypedOperationResultPage> {
        if self.result_page_presented {
            return None;
        }
        self.result_page_presented = true;
        self.result_page.clone()
    }

    pub(crate) fn acknowledge_result_page(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault> {
        let Some(page) = self.result_page.as_ref() else { return Ok(false) };
        if page.token != token || self.stage != MountedTypedCommandFullOperationStage::AwaitingAck {
            return Ok(false);
        }
        let lane = page.lane;
        if matches!(
            lane,
            TypedOperationResultLane::Artifact
                | TypedOperationResultLane::Config
                | TypedOperationResultLane::Draft
                | TypedOperationResultLane::Presence
                | TypedOperationResultLane::Transient
                | TypedOperationResultLane::WindowConfig
                | TypedOperationResultLane::WindowTransient
        ) && !self.pending_artifact_publication.as_mut().is_some_and(PendingArtifactStorePublication::acknowledge)
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.artifact-publication-ack"), "artifact-store result ACK lost its exact pending publication authority"));
        }
        if lane == TypedOperationResultLane::Child && !self.owned_child_committed && !self.pending_child_publication.as_mut().is_some_and(PendingChildGroupPublication::acknowledge) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-publication-ack"), "child-group result ACK lost its exact committed publication authority"));
        }
        self.result_page = None;
        if lane == TypedOperationResultLane::Child && self.owned_child_committed { self.owned_child_result_pending = false; }
        self.result_page_presented = false;
        self.result_sequence = self.result_sequence.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-sequence"), "typed-operation result sequence exhausted"))?;
        self.stage = if matches!(lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault | TypedOperationResultLane::Download) {
            MountedTypedCommandFullOperationStage::Retiring
        } else {
            MountedTypedCommandFullOperationStage::Publishing
        };
        Ok(true)
    }

    /// 📏️ Quotes the exact next [`Self::drive_worker_step`] turn of a Worker-stage operation.
    pub(crate) fn worker_step_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if self.stage != MountedTypedCommandFullOperationStage::Worker { return Ok(Default::default()); }
        if let Some(source)=self.session_rejected.as_ref(){return if source.admission_pending(){source.admission_demands()}else{self.worker_retirement_demands(body)};}
        let Some(session) = self.session.as_ref() else { return Ok(item) };
        if self.worker_fault_capture.is_some() {
            return match session.checked_out_outcome() {
                Ok(Some(semio_framework_job::JobOutcomeView::Fault { detail, .. })) => original_fault_capture_demands(&self.worker_fault_capture, detail),
                _ => Ok(item),
            };
        }
        if self.worker_outcome_pending || (self.terminal_seen && !self.worker_resume_pending) { return self.worker_retirement_demands(body); }
        Ok(item)
    }

    fn worker_retirement_pending(&self) -> bool {
        self.session.is_some() || self.session_rejected.is_some() || self.worker_semantic_pending || self.worker_outcome_pending || self.worker_fault_capture.is_some()
    }

    /// 📏️ Quotes the exact next [`Self::retire_worker_step`] turn.
    fn worker_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if let Some(rejected) = self.session_rejected.as_ref() {
            if rejected.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(rejected), ..item }); }
            return rejected.retirement_demands(body);
        }
        if let Some(session) = self.session.as_ref() {
            if session.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(session), ..item }); }
            return match session.retirement_demands(body) {
                Ok(demand) => Ok(demand),
                Err(semio_framework_job::WorkerJobDemandError::Contention(_)) => Ok(item),
                Err(semio_framework_job::WorkerJobDemandError::Refused(error)) => Err(error),
            };
        }
        Ok(item)
    }

    /// ♻️ Closes the original mounted worker session, its refused admission and its inline capture markers under the unchanged grant.
    fn retire_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();let demand=self.worker_retirement_demands(grant.maximum_copy_bytes)?;
        crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if let Some(rejected) = self.session_rejected.as_mut() {
            if rejected.terminal_is_empty() {
                let bytes = size_of_val(rejected);
                drop(self.session_rejected.take());
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_bytes: bytes, ..item }));
            }
            let demand = rejected.retirement_demands(grant.maximum_copy_bytes)?;
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant| {
                let step = rejected.close_step(cx)?.admit(grant, rejected.terminal_is_empty())?;
                Ok((match step {
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => PluginLifecycleStep::Progress(progress),
                    semio_framework_job::InteractiveJobCloseStep::Blocked => PluginLifecycleStep::AwaitingInput { reason: "typed operation rejected-session close awaits its worker handback" },

                }, false))
            }).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if let Some(session) = self.session.as_mut() {
            if session.terminal_is_empty() {
                let bytes = size_of_val(session);
                drop(self.session.take());
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_bytes: bytes, ..item }));
            }
            session.begin_close();
            if let Some(step) = return_session_cancel_alias(session, &self.cancellation_lease, grant) { return step.map_err(semio_framework_job::OriginalTurnFailure::from); }
            let demand = match session.retirement_demands(grant.maximum_copy_bytes) {
                Ok(demand) => demand,
                Err(semio_framework_job::WorkerJobDemandError::Contention(_)) => return Ok(PluginLifecycleStep::AwaitingInput { reason: "typed operation original worker close quote awaits its handback" }),
                Err(semio_framework_job::WorkerJobDemandError::Refused(error)) => return Err(error.into()),
            };
            return crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant| {
                let step = session.close_step(cx)?;
                Ok((match step {
                    semio_framework_job::WorkerJobCloseStep::Pending { progress } if progress.fits(grant) => PluginLifecycleStep::Progress(progress),
                    semio_framework_job::WorkerJobCloseStep::Complete { progress } if progress.fits(grant) => PluginLifecycleStep::Progress(progress),
                    semio_framework_job::WorkerJobCloseStep::Blocked => PluginLifecycleStep::AwaitingInput { reason: "typed operation mounted session close awaits its worker handback" },

                    _ => return Err(plugin_sdk_fault("typed operation worker violated its original grant or terminal witness").with_retained_progress(step.progress()).into()),
                }, false))
            }).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        self.worker_fault_capture = None;
        self.worker_semantic_pending = false;
        self.worker_outcome_pending = false;
        Ok(PluginLifecycleStep::Progress(item))
    
            })()?;Ok((step,self.terminal_is_empty()))
        }).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    /// 📏️ Quotes the exact next [`Self::retirement_step`] turn, one original owner at a time in the same order.
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if self.close_fault_pending() { return self.close_fault_demands(body); }
        if self.worker_retirement_pending() { return self.worker_retirement_demands(body); }
        if self.pending_window_config_receipt.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<store::os_store::presence::LaneItemReceipt>(), ..item }); }
        if self.granted_retirement_ready() && self.granted_retirement_pending() { return self.granted_retirement_demands(body); }
        if self.publication_retirement.is_none() {
            if self.meta.view_state.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.meta.view_state); }
            if self.window_config_authority.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.window_config_authority); }
            if self.window_transient_authority.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.window_transient_authority); }
        }
        if self.captured_child_content.is_some() { return Ok(item); }
        if self.pending_publication_outcome.pending() || self.pending_publication_outcome.report.is_some() { return Ok(RetirementDemand { copy_bytes: self.pending_publication_outcome.copy_demand(), ..item }); }
        if self.verb.capacity() != 0 { return Ok(RetirementDemand { release_bytes: self.verb.capacity(), ..item }); }
        if self.meta.actor.has_owner() { return Ok(RetirementDemand { release_bytes: self.meta.actor.original_allocation_bytes(), ..item }); }
        Ok(Default::default())
    }

    /// 🎟️ Returns each original mounted owner under the caller's unchanged grant.
    pub(crate) fn retirement_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        let empty = RetainedCloneProgress::default();
        let item = RetainedCloneProgress { copied_items: 1, ..empty };
        if self.stage != MountedTypedCommandFullOperationStage::Retiring { return Ok(PluginLifecycleStep::Blocked { reason: "typed operation is not admitted for retirement" }); }
        if self.result_page.is_some() { return Ok(PluginLifecycleStep::Blocked { reason: "typed operation retains an unacknowledged result page" }); }
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if !plugin_grant_funds(demand, grant) { return Ok(PluginLifecycleStep::Progress(empty)); }
        if self.close_fault_pending(){return self.close_fault_step(grant).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(semio_framework_job::OriginalTurnFailure::from);}
        if self.worker_retirement_pending() { return self.retire_worker_step(cx); }
        if self.pending_window_config_receipt.take().is_some(){return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_bytes:size_of::<store::os_store::presence::LaneItemReceipt>(),..item}));}
        if self.granted_retirement_ready() && self.granted_retirement_pending() {
            let step = self.granted_retirement_step(cx,grant)?;
            semio_framework_value::retained_clone::admit_retained_clone_progress(grant, step.progress(), "typed operation granted retirement")?;
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        if self.publication_retirement.is_none() {
            if self.meta.view_state.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.meta.view_state, &mut self.publication_retirement, grant).map(|step| PluginLifecycleStep::Progress(step.progress())).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if self.window_config_authority.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.window_config_authority, &mut self.publication_retirement, grant).map(|step| PluginLifecycleStep::Progress(step.progress())).map_err(semio_framework_job::OriginalTurnFailure::from); }
            if self.window_transient_authority.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.window_transient_authority, &mut self.publication_retirement, grant).map(|step| PluginLifecycleStep::Progress(step.progress())).map_err(semio_framework_job::OriginalTurnFailure::from); }
        }
        if self.captured_child_content.take().is_some() { return Ok(PluginLifecycleStep::Progress(item)); }
        if self.pending_publication_outcome.pending() || self.pending_publication_outcome.report.is_some() {
            let bytes = self.pending_publication_outcome.copy_demand();
            self.pending_publication_outcome = PendingPublicationOutcome::new();
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_bytes: bytes, ..item }));
        }
        if self.verb.capacity() != 0 { let original = std::mem::take(&mut self.verb); let bytes = original.capacity(); drop(original); return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { released_bytes: bytes, ..item })); }
        if self.meta.actor.has_owner() {
            let original = std::mem::take(&mut self.meta.actor);
            return match original.close_original_lease(grant) {
                Ok((text, progress)) => { let released = text.as_ref().map_or(0, String::capacity); drop(text); Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { released_bytes: progress.released_bytes + released, ..progress })) }
                Err((error, original)) => { self.meta.actor = original; Err(error.into()) }
            };
        }
        if !self.terminal_is_empty() { return Ok(PluginLifecycleStep::Blocked { reason: "original mounted authority awaits its typed retirement issuer" }); }
        Ok(PluginLifecycleStep::Complete(empty))
    
            })()?;Ok((step,self.terminal_is_empty()))
        }).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    /// 📏️ Quotes cancellation handback separately from original owner retirement.
    pub(crate) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.stage == MountedTypedCommandFullOperationStage::Retiring { self.retirement_demands(body) }
        else { Ok(RetirementDemand { depth: 1, ..Default::default() }) }
    }

    /// 🎟️ Withdraws inline result custody before closing retained originals.
    pub(crate) fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();let demand=self.close_demands(grant.maximum_copy_bytes)?;
        crate::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !plugin_grant_funds(demand, grant) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if self.stage == MountedTypedCommandFullOperationStage::Retiring { return self.retirement_step(cx); }
        self.user_cancel_requested = true;
        self.result_page = None;
        self.result_page_presented = false;
        self.ui_pending = false;
        self.progress_pending = false;
        self.stage = MountedTypedCommandFullOperationStage::Retiring;
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    
            })()?;Ok((step,self.terminal_is_empty()))
        }).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.stage == MountedTypedCommandFullOperationStage::Retiring
            && self.session.is_none()
            && self.session_rejected.is_none()
            && !self.worker_semantic_pending
            && !self.worker_outcome_pending
            && self.worker_fault_capture.is_none()
            && self.publication.is_none()
            && self.pending_artifact_publication.is_none()
            && !self.pending_publication_outcome.pending() && self.pending_publication_outcome.report.is_none()
            && self.pending_child_publication.is_none()
            && self.owned_child_group.is_none()
            && self.captured_child_content.is_none()
            && self.result_page.is_none()
            && self.raw_input.is_none()
            && self.output_chunks.is_none()
            && self.reserved_producer.is_none()
            && self.completion.is_none()
            && self.completion_retirement.is_none()
            && self.publication_retirement.is_none()
            && self.output_retirement.is_none()
            && self.cancellation_lease.is_none() && self.cancellation_retirement.is_none() && self.pending_window_config_receipt.is_none()
            && self.actor_capture.is_none()
            && self.verb.capacity() == 0
            && !self.meta.actor.has_owner()
            && self.meta.view_state.is_none()
            && self.window_config_authority.is_none()
            && self.window_transient_authority.is_none()
            && self.retained_close_fault.is_none()
            && self.retained_close_fault_retirement.is_none()
            && self.retained_close_fault_refusal.is_none()
    }
}

pub(crate) fn bounded_json_items(value: Option<&DslValue>, maximum: usize) -> Result<usize, Fault> {
    let mut count = 1usize;
    let mut pending = value.into_iter().collect::<Vec<_>>();
    while let Some(value) = pending.pop() {
        count = count.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.decoded-limit"), "decoded item count overflowed"))?;
        if count > maximum {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.decoded-limit"), format!("decoded command contains {count} items; maximum is {maximum}")));
        }
        match value {
            DslValue::Array(values) => pending.extend(values),
            DslValue::Object(values) => {
                count = count.checked_add(values.len()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.decoded-limit"), "decoded item count overflowed"))?;
                if count > maximum {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.decoded-limit"), format!("decoded command contains {count} items; maximum is {maximum}")));
                }
                pending.extend(values.iter().map(|(_, value)| value));
            }
            _ => {}
        }
    }
    Ok(count)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TypedCommandFullOperationPhase {
    Prepare,
    Reducer,
    Fault,
}

pub(crate) struct TypedCommandFullOperationJob<A: ArtifactApp> {
    pub(crate) operation: Option<semio_framework_job::Operation>,
    pub(crate) parent_document_id: Option<String>,
    pub(crate) command: Option<std::sync::Arc<A::Command>>,
    pub(crate) snapshot: Option<std::sync::Arc<A::Snapshot>>,
    pub(crate) config: Option<std::sync::Arc<A::Config>>,
    pub(crate) history: Option<std::sync::Arc<HistoryView>>,
    pub(crate) children: Option<std::sync::Arc<ChildContentView>>,
    pub(crate) draft: Option<std::sync::Arc<A::Draft>>,
    pub(crate) interaction_state: Option<std::sync::Arc<protocol::InteractionState>>,
    pub(crate) interaction_hover: Option<std::sync::Arc<InteractionHoverState>>,
    pub(crate) peer_presence: Option<std::sync::Arc<PeerPresenceRoot>>,
    pub(crate) presence_local: Option<store::os_store::snapshot_read::SnapshotRead<A::Presence>>,
    pub(crate) presence_peers: Option<std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>>,
    pub(crate) transient: Option<std::sync::Arc<A::Transient>>,
    pub(crate) contract: semio_framework::ToolExecutionContract,
    pub(crate) decoded_items: usize,
    pub(crate) phase: TypedCommandFullOperationPhase,
    pub(crate) prepare_cursor: u8,
    pub(crate) reducer_cursor: u8,
    pub(crate) output: Option<ArtifactToolCompletion<A>>,
    pub(crate) state: [u8; 3],
    pub(crate) fault_message: &'static [u8],
    pub(crate) pending: Option<crate::tool_contract::ReservedOutcomeKind>,
    pub(crate) outcomes: crate::tool_contract::ReservedJobOutcomes,
    pub(crate) closing: bool,
}

impl<A: ArtifactApp> TypedCommandFullOperationJob<A> {
    /// 📏️ The next close turn: the parent id text, else one retained alias per turn.
    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.outcomes.terminal_is_empty() { return self.outcomes.close_demands(); }
        Ok(self.owner_close_demand())
    }

    fn owner_close_demand(&self) -> RetirementDemand {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if let Some(parent) = self.parent_document_id.as_ref() {
            let demand = plugin_text_retirement_demand(parent);
            return if demand == RetirementDemand::default() { item } else { demand };
        }
        let retained = self.command.is_some() || self.snapshot.is_some() || self.config.is_some() || self.history.is_some() || self.children.is_some() || self.draft.is_some() || self.interaction_state.is_some() || self.interaction_hover.is_some() || self.peer_presence.is_some() || self.presence_local.is_some() || self.presence_peers.is_some() || self.transient.is_some() || self.output.is_some();
        if retained||self.operation.is_some()||!self.closing||self.pending.is_some() { item } else { RetirementDemand::default() }
    }

    fn checkpoint(&mut self) {
        self.state = [self.phase as u8, self.prepare_cursor, self.reducer_cursor];
        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Checkpoint { applied_progress: u64::from(self.prepare_cursor).saturating_add(u64::from(self.reducer_cursor)) });
    }

    fn fault(&mut self, message: &'static [u8]) {
        self.phase = TypedCommandFullOperationPhase::Fault;
        self.fault_message = message;
        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Fault);
    }
}

impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use crate::tool_contract::ReservedOutcomeKind;
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.outcomes.is_delivered() {
            self.outcomes.retire(cx)?;
            return Ok(None);
        }
        if let Some(kind) = self.pending {
            let source: &[u8] = match kind {
                ReservedOutcomeKind::Preview => b"typed-command-prepare",
                ReservedOutcomeKind::Fault => self.fault_message,
                _ => &self.state,
            };
            let outcome = self.outcomes.advance(kind, source, cx)?;
            if outcome.is_some() {
                self.pending = None;
            }
            return Ok(outcome);
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.decoded_items > self.contract.max_decoded_items || self.contract.max_work_units_per_step == 0 || self.contract.max_output_bytes == 0 {
            self.fault(b"typed command admission exceeds its fixed item, work, or output cap");
            return Ok(None);
        }
        cx.consume_fuel(1);
        let operation = self.operation.expect("tool job factory assigns operation identity");
        debug_assert_eq!(operation.operation, cx.operation());
        match self.phase {
            TypedCommandFullOperationPhase::Prepare => {
                cx.set_stage("typed-command-prepare");
                let present = match self.prepare_cursor {
                    0 => self.parent_document_id.is_some(),
                    1 => self.command.is_some(),
                    2 => self.snapshot.is_some(),
                    3 => self.config.is_some(),
                    4 => self.history.is_some(),
                    5 => self.children.is_some(),
                    6 => self.draft.is_some(),
                    7 => self.interaction_state.is_some(),
                    8 => self.interaction_hover.is_some(),
                    9 => self.peer_presence.is_some(),
                    10 => self.presence_local.is_some(),
                    11 => self.presence_peers.is_some(),
                    12 => self.transient.is_some(),
                    _ => {
                        self.phase = TypedCommandFullOperationPhase::Reducer;
                        self.checkpoint();
                        return Ok(None);
                    }
                };
                if !present {
                    self.fault(b"typed command preparation root is absent");
                    return Ok(None);
                }
                self.prepare_cursor = self.prepare_cursor.saturating_add(1);
                if self.prepare_cursor == 1 {
                    self.pending = Some(ReservedOutcomeKind::Preview);
                } else {
                    self.checkpoint();
                }
            }
            TypedCommandFullOperationPhase::Reducer => {
                cx.set_stage("typed-command-reducer");
                self.reducer_cursor = self.reducer_cursor.saturating_add(1);
                self.fault(b"generic bounded proof has no resumable app-owned reducer job");
            }
            TypedCommandFullOperationPhase::Fault => self.fault(b"typed command remains faulted"),
        }
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep,close_original_retained_step};
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.close_demand()?;
        close_original_retained_step(cx,demand,|grant|{
            self.closing=true;self.pending=None;
            if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}
            let item=RetainedCloneProgress{copied_items:1,..Default::default()};
            if let Some(parent)=self.parent_document_id.as_mut(){return Ok(RetainedCloneStep::Progress(plugin_text_close(parent,grant).unwrap_or_else(||{self.parent_document_id=None;item})))}
            macro_rules! return_alias{($($field:ident),+)=>{$(if let Some(original)=self.$field.as_ref(){if std::sync::Arc::strong_count(original)<=1{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"typed command requires its original final owner retirement cursor"))}drop(self.$field.take());return Ok(RetainedCloneStep::Progress(item));})+}}
            return_alias!(command,snapshot,config,history,children,draft,interaction_state,interaction_hover,peer_presence,presence_peers,transient);
            if self.presence_local.is_some()||self.output.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"typed command requires its original retained completion or snapshot-read retirement cursor"))}
            self.operation=None;Ok(RetainedCloneStep::Complete(item))
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand()?.depth)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outcomes.terminal_is_empty() && self.owner_terminal_is_empty()
    }
}

impl<A: ArtifactApp> TypedCommandFullOperationJob<A> {
    fn owner_terminal_is_empty(&self) -> bool {
        self.parent_document_id.is_none()
            && self.command.is_none()
            && self.snapshot.is_none()
            && self.config.is_none()
            && self.history.is_none()
            && self.children.is_none()
            && self.draft.is_none()
            && self.interaction_state.is_none()
            && self.interaction_hover.is_none()
            && self.peer_presence.is_none()
            && self.presence_local.is_none()
            && self.presence_peers.is_none()
            && self.transient.is_none()
            && self.output.is_none()
            && self.operation.is_none()
    }
}

pub(crate) struct TypedCommandFullOperationJobFactory<A: ArtifactApp> {
    keys: Vec<semio_framework::ToolFactoryKey>,
    payload_schema_id: String,
    proof: QualifiedBoundedFirstStepProof,
    marker: std::marker::PhantomData<fn() -> A>,
}

impl<A: ArtifactApp> TypedCommandFullOperationJobFactory<A> {
    pub(crate) fn from_proof(proof: QualifiedBoundedFirstStepProof) -> Option<Self> {
        if proof.owner != ToolOwnerWitness::of::<A>() || proof.row.owner != proof.owner || proof.row.factory != BOUNDED_FIRST_STEP_FACTORY || proof.row.factory_type_id.is_some() || proof.row.factory_type_name.is_some() {
            return None;
        }
        let keys = vec![proof.key()];
        let payload_schema_id = proof.schema_id();
        Some(Self { keys, payload_schema_id, proof, marker: std::marker::PhantomData })
    }
}

impl<A: ArtifactApp> semio_framework::ToolJobFactory for TypedCommandFullOperationJobFactory<A> {
    type Payload = TypedCommandFullOperationJob<A>;
    type Job = TypedCommandFullOperationJob<A>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        &self.payload_schema_id
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        self.proof.contract()
    }

    fn create_job(&mut self, operation: semio_framework_job::Operation, mut payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        payload.operation = Some(operation);
        Ok(payload)
    }
}

#[cfg(test)]
pub(crate) async fn test_retained_keyed_dispatch<A: ArtifactApp + Default>(
    registry: AppActionRegistry,
    command: fn(&str, i32) -> A::Command,
    mutation: fn(i32) -> A::Mutation,
    observe: fn(&A::Snapshot) -> i32,
    clock: Option<fn() -> Option<u64>>,
    mounted_policy: crate::MountedOwnerPolicyV1,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
) {
    let fixture: Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
    let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)));
    for case in fixture["cases"].as_array().unwrap() {
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry.clone(), protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.test_tool_clock = clock;
        app.bind_instance_id(7).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None };
        let first = Box::new(command(case["firstTarget"].as_str().unwrap(), case["firstValue"].as_i64().unwrap() as i32));
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&first).unwrap();
        let admission = app.admit_command_wire("compositeEdit", &wire, 1).await.unwrap();
        assert!(matches!(admission.proof, QualifiedToolProof::AppOwned(_)));
        app.dispatch_typed_command_inner(first, admission, &meta, identity).await.unwrap();
        let first_id = *app.latest_wins_order.items.front().unwrap();
        assert!(!app.can_admit_typed_operation(first_id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64));
        for _ in 0..100_000 {
            if app.tool_operations.get(first_id).is_some() {
                break;
            }
            app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
            if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
            plugin_job_yield_once().await;
        }
        let first_token = app.tool_operations.get(first_id).expect("actual registered first worker").cancellation_lease.as_ref().unwrap().token.clone();
        let next = Box::new(command(case["nextTarget"].as_str().unwrap(), case["nextValue"].as_i64().unwrap() as i32));
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&next).unwrap();
        let admission = app.admit_command_wire("compositeEdit", &wire, 1).await.unwrap();
        app.dispatch_typed_command_inner(next, admission, &meta, identity).await.unwrap();
        let next_id = *app.latest_wins_order.items.front().unwrap();
        let old_key = app.latest_wins_commands.get(next_id).unwrap().lease.as_ref().unwrap().key.clone();
        app.store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation(13)], transaction: None }, identity).await.unwrap();
        let fresh_revision = app.store.content_revision_now();
        for _ in 0..100_000 {
            if app.tool_operations.get(next_id).is_some() {
                break;
            }
            app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
            if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
            plugin_job_yield_once().await;
        }
        let mounted = app.tool_operations.get(next_id).expect("accepted key creates actual fresh worker");
        assert_eq!(mounted.canonical_revision, fresh_revision);
        assert_eq!(mounted.operation.generation.0, fixture["rebase"]["afterGeneration"].as_u64().unwrap());
        let fresh_key = mounted.cancellation_lease.as_ref().unwrap().key.clone();
        assert_eq!(fresh_key.generation, mounted.operation.generation);
        assert!(!app.tool_cancellations.cancel(&old_key).unwrap());
        assert_eq!(serde_json::json!(first_token.is_cancelled_now()), case["firstCancelled"]);
        let expected = 13 + case["nextValue"].as_i64().unwrap() as i32;
        for _ in 0..100_000 {
            if observe(app.store.snapshot_root().as_ref()) == expected {
                break;
            }
            if app.tool_operations.get(next_id).unwrap().stage == MountedTypedCommandFullOperationStage::Worker {
                app.tool_operations.get_mut(next_id).unwrap().drive_worker_step(app.mounted_policy.maintenance).unwrap();
            } else {
                app.advance_typed_operation_publication_one(app.mounted_policy.maintenance).await.unwrap();
            }
            plugin_job_yield_once().await;
        }
        let state = app.tool_operations.get(next_id).unwrap();
        eprintln!(
            "[TRACE] registered keyed dispatch progress stage={:?} terminal_seen={} pending={} completion={} result={:?}",
            state.stage,
            state.terminal_seen,
            state.pending_artifact_publication.is_some(),
            state.publication.is_some(),
            state.result_page.as_ref().map(|page| String::from_utf8_lossy(&page.bytes[..page.len]))
        );
        assert_eq!(serde_json::json!(observe(app.store.snapshot_root().as_ref())), serde_json::json!(expected));
        assert!(app.tool_cancellations.cancel(&fresh_key).unwrap());
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            match fixture_close_receipt(&mut app, mounted_policy).unwrap() {
                PluginLifecycleStep::Progress(progress) => {
                    assert!(progress.copied_items <= 1);
                    assert!(progress.copied_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                    assert!(progress.fits(mounted_policy.close));
                }
                PluginLifecycleStep::Complete(progress) => {assert!(progress.fits(mounted_policy.close));break;},
                PluginLifecycleStep::AwaitingInput { reason } => panic!("registered keyed dispatch close awaited input: {reason}"),
                PluginLifecycleStep::Blocked { .. } => {}
            }
            plugin_job_yield_once().await;
        }
        assert!(app.close_terminal_is_empty());
        eprintln!("[TRACE] actual registered keyed dispatch {} preserved exact target supersession, rebased its worker, published count{expected}, and closed every retained owner", case["id"]);
    }
    for case in fixture["lostReservations"].as_array().unwrap() {
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry.clone(), protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(7).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None };
        let command = Box::new(command("aä🧵", 42));
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).unwrap();
        let admission = app.admit_command_wire("compositeEdit", &wire, 1).await.unwrap();
        app.dispatch_typed_command_inner(command, admission, &meta, identity).await.unwrap();
        let id = *app.latest_wins_order.items.front().unwrap();
        let pending = app.latest_wins_commands.get_mut(id).unwrap();
        let copy = pending.key_copy.take();
        let key = pending.key.take();
        pending.retired_key = Some(fixture["keyRetirement"]["text"].as_str().unwrap().into());
        let pointer=pending.retired_key.as_ref().unwrap().as_ptr();let capacity=pending.retired_key.as_ref().unwrap().capacity();
        for denied in[RetainedCloneGrant{maximum_items:0,..mounted_policy.close},RetainedCloneGrant{maximum_release_bytes:capacity-1,..mounted_policy.close},RetainedCloneGrant{maximum_depth:0,..mounted_policy.close}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pending.close_key_step(denied).unwrap());assert_eq!(step.progress().unwrap(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pending.retired_key.as_ref().unwrap().as_ptr(),pointer);}
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pending.close_key_step(mounted_policy.close).unwrap());assert_eq!(step.progress().unwrap().released_bytes,capacity);assert_eq!(step.progress().unwrap().copied_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(serde_json::json!(capacity),fixture["keyRetirement"]["utf8Bytes"]);
        pending.key_copy = copy;
        pending.key = key;
        let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let foreign = id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64;
        app.typed_operation_reservations[slot] = case["foreignReservation"].as_bool().unwrap().then_some(foreign);
        for _ in 0..8 {
            app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
            if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
            assert!(app.tool_operations.get(id).is_none());
            if case["foreignReservation"].as_bool().unwrap() {
                assert_eq!(app.typed_operation_reservations[slot], Some(foreign));
            }
        }
        app.typed_operation_reservations[slot] = None;
        for _ in 0..100_000 {
            if app.tool_operations.get(id).is_some() {
                break;
            }
            app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
            if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
        }
        let mounted = app.tool_operations.get_mut(id).expect("vacant rejected slot receives its retained terminal result");
        assert!(mounted.session.is_none());
        let page = mounted.result_page.as_ref().unwrap();
        assert_eq!(page.lane, TypedOperationResultLane::Fault);
        assert!(mounted.acknowledge_result_page(page.token).unwrap());
        assert_eq!(observe(app.store.snapshot_root().as_ref()), 0);
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            let _ = fixture_close_receipt(&mut app, mounted_policy).unwrap();
            plugin_job_yield_once().await;
        }
        assert!(app.close_terminal_is_empty());
        eprintln!("[TRACE] actual registered keyed dispatch {} preserved foreign reservation, retired seven UTF-8 bytes exactly, and delivered rejection after vacancy", case["id"]);
    }
}

/// 🎰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B0: typed-operation ingress pre-admits the exact slot
/// BEFORE it mints an operation id. Minting from the process-wide counter first and then testing that
/// one residue class of the direct-mapped 64-slot authorities refused unrelated live actions —
/// `suggestionsTick`, `setCamera`, `engagementAbort`, `translateSelection` all died on
/// it in the puzzle3d browser battery — whenever a live operation happened to share the class. With 63
/// classes held, sixty-four consecutive admissions must every time land on the one vacant slot (the
/// mint-then-test model visits every residue class across that span, so it fails here deterministically),
/// a real dispatch must succeed there, and only true saturation may refuse.
#[cfg(test)]
pub(crate) async fn test_typed_operation_slot_preadmission<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    const VACANT: usize = 11;
    let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
    app.bind_instance_id(7).await;
    let meta = ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None };
    for slot in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
        app.typed_operation_reservations[slot] = (slot != VACANT).then_some((slot + 3 * ARTIFACT_LIVE_OUTPUT_SLOTS) as u64);
    }
    for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
        let admitted = app.admit_typed_operation_slot().expect("one vacant residue class still pre-admits an exact operation slot");
        assert_eq!(admitted.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS, VACANT, "ingress must mint its operation id FOR the vacant slot instead of testing whichever slot the shared counter reached");
    }
    let first = Box::new(command("aä🧵", 42));
    let wire = <A::Command as ::protocol::OpBinary>::encode_op(&first).unwrap();
    let admission = app.admit_command_wire(verb, &wire, 1).await.unwrap();
    app.dispatch_typed_command_inner(first, admission, &meta, identity).await.expect("a typed action must dispatch while unrelated operations hold every other slot");
    let operation_id = *app.latest_wins_order.items.front().expect("the admitted operation owns the vacant slot");
    assert_eq!(operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS, VACANT);
    assert_eq!(app.typed_operation_reservations[VACANT], Some(operation_id));
    let saturated = Box::new(command("bö🧶", 43));
    let wire = <A::Command as ::protocol::OpBinary>::encode_op(&saturated).unwrap();
    let admission = app.admit_command_wire(verb, &wire, 1).await.unwrap();
    let fault = app.dispatch_typed_command_inner(saturated, admission, &meta, identity).await.expect_err("only true saturation of every fixed slot may refuse a typed action");
    assert_eq!(fault.code.0, "interactive-job.typed-operation-capacity");
    assert!(app.admit_typed_operation_slot().is_none());
    for slot in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
        if slot != VACANT {
            app.typed_operation_reservations[slot] = None;
        }
    }
    app.latest_wins_commands.get_mut(operation_id).expect("the pending keyed command").lease.as_ref().expect("pending cancellation lease").cancel();
    for _ in 0..100_000 {
        if app.tool_operations.get(operation_id).is_some() {
            break;
        }
        app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
            if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
        plugin_job_yield_once().await;
    }
    let mounted = app.tool_operations.get_mut(operation_id).expect("the cancelled pending command lands its retained terminal result");
    let page = mounted.result_page.as_ref().expect("terminal fault page");
    assert_eq!(page.lane, TypedOperationResultLane::Fault);
    let token = page.token;
    assert!(mounted.acknowledge_result_page(token).unwrap());
    for _ in 0..100_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        let _ = fixture_close_receipt(&mut app, mounted_policy).unwrap();
        plugin_job_yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
    eprintln!("[TRACE] actual typed ingress pre-admitted slot {VACANT} under 63 live foreign reservations and refused only at true saturation");
}

/// ♻️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8: every admitted typed-operation slot is RELEASED by
/// the very continuation that reports it runnable. Admission happens synchronously at ingress, but the
/// only driver that ever removed a `Retiring` operation from `tool_operations` was `maintenance_step`
/// stage 0 of [`MAINTENANCE_STAGES`] — one bounded unit per whole rotation, scheduled on the shared
/// live-cleanup job, i.e. once per ~24 reactor turns — while
/// [`VcsArtifactApp::advance_typed_operation_publication_one`] reported the same operation runnable and
/// advanced nothing for it. Any sustained action rate therefore fills all
/// [`ARTIFACT_LIVE_OUTPUT_SLOTS`] residue classes and every later typed action is refused with
/// `interactive-job.typed-operation-capacity` (puzzle3d browser battery 2026-09-11: 39 refusals across
/// `engagementAbort`, `setCamera`, `openVortexSuggestions`). This law drives ONLY the host's own
/// continuation — publication, result page, ACK, outbox drains — exactly as
/// `plugin_runtime::advance_typed_operation_output` does, with no maintenance call at all, and requires
/// the occupancy to return to zero after every single action.
#[cfg(test)]
pub(crate) async fn test_typed_operation_slot_retirement_under_storm<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    const ACTIONS: usize = 200;
    const HOST_CONTINUATIONS_PER_ACTION: usize = 512;
    const RECEIVER: u32 = 7;
    let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
    app.bind_instance_id(RECEIVER).await;
    let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
    let mut highest = 0;
    for index in 0..ACTIONS {
        let target = format!("storm-{index}");
        let payload = Box::new(command(&target, index as i32));
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&payload).unwrap();
        let admission = app.admit_command_wire(verb, &wire, 1).await.expect("every storm action retains its exact command admission");
        if let Err(fault) = app.dispatch_typed_command_inner(payload, admission, &meta, identity).await {
            panic!("action {index} was refused with '{}' ({}) after {highest} of {ARTIFACT_LIVE_OUTPUT_SLOTS} typed-operation slots were never released by the host continuation", fault.message, fault.code.0);
        }
        drive_host_typed_continuations(&mut app, RECEIVER, HOST_CONTINUATIONS_PER_ACTION, mounted_policy, identity).await;
        let live = app.live_typed_operation_slots();
        highest = highest.max(live);
        assert_eq!(live, 0, "action {index} {}; an admitted slot must have exactly one release path the same turn driver walks", released_slot_census(&app, HOST_CONTINUATIONS_PER_ACTION));
    }
    close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
    eprintln!("[TRACE] actual {ACTIONS} storm actions each released their typed-operation slot within {HOST_CONTINUATIONS_PER_ACTION} host continuations, peak occupancy {highest}");
}

/// ♻️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8: the slot release path is the SAME on every outcome an
/// admitted typed operation can reach. `settled` publishes and terminates normally; `replaced` admits
/// three commands on ONE latest-wins key with no continuation between them, so the FIFO retires the
/// displaced pendings; `cancelled` cancels the admitted command's own cancellation lease, which is the
/// path a refused or faulted operation also takes (it mounts a terminal `Fault` page instead of a
/// terminal publication). After each, the fixed slot array must be empty again.
#[cfg(test)]
pub(crate) async fn test_typed_operation_slot_release_on_every_outcome<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    const CONTINUATIONS: usize = 512;
    const RECEIVER: u32 = 7;
    let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
    app.bind_instance_id(RECEIVER).await;
    let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
    admit_fixture_typed_action(&mut app, verb, Box::new(command("settled", 1)), &meta, identity).await;
    drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity).await;
    assert_eq!(app.live_typed_operation_slots(), 0, "a settled operation {}", released_slot_census(&app, CONTINUATIONS));
    for value in 0..3 {
        admit_fixture_typed_action(&mut app, verb, Box::new(command("replaced", value)), &meta, identity).await;
    }
    drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity).await;
    assert_eq!(app.live_typed_operation_slots(), 0, "a latest-wins replacement {}", released_slot_census(&app, CONTINUATIONS));
    admit_fixture_typed_action(&mut app, verb, Box::new(command("cancelled", 4)), &meta, identity).await;
    let pending = *app.latest_wins_order.items.front().expect("the admitted keyed command owns its exact pending slot");
    app.latest_wins_commands.get_mut(pending).expect("the admitted keyed command").lease.as_ref().expect("pending cancellation lease").cancel();
    drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity).await;
    assert_eq!(app.live_typed_operation_slots(), 0, "a cancelled operation {}", released_slot_census(&app, CONTINUATIONS));
    close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
    eprintln!("[TRACE] actual settled, latest-wins-replaced and cancelled typed operations each released their exact slot within {CONTINUATIONS} host continuations");
}

/// 🎫️ Admits one typed action through the production ingress, failing with the refusal a saturated slot
/// array would have produced.
#[cfg(test)]
async fn admit_fixture_typed_action<A: ArtifactApp + Default>(app: &mut VcsArtifactApp<A>, verb: &str, command: Box<A::Command>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let wire = <A::Command as ::protocol::OpBinary>::encode_op(command.as_ref()).unwrap();
    let admission = app.admit_command_wire(verb, &wire, 1).await.expect("every fixture action retains its exact command admission");
    if let Err(fault) = app.dispatch_typed_command_inner(command, admission, meta, identity).await {
        panic!("typed ingress refused '{verb}' with '{}' ({})", fault.message, fault.code.0);
    }
}

/// 🔁️ The host's OWN typed-operation continuation, unit for unit: exactly what
/// `plugin_runtime::advance_typed_operation_output` drives per turn — one publication unit, the result
/// page for the bound receiver plus its mandatory ACK, then the effect, event, UI-scope and completion
/// outboxes. Deliberately calls NO `maintenance_step`: a slot the host cannot release on this path alone
/// is a leak, whatever the cooperative rotation would eventually have done about it.
#[cfg(test)]
async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt> {
    let mut ownership_receipts = Vec::new();
    artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture admission receipts");
    for _ in 0..continuations {
        if !app.has_runnable_typed_operations() {
            return ownership_receipts;
        }
        app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
        artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture continuation receipts");
        while let Some(page) = app.take_typed_operation_result_page(receiver) {
            assert!(app.acknowledge_typed_operation_result(page.token).expect("presented result page accepts its exact token"));
        }
        while app.take_typed_operation_effect().is_some() {}
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while app.take_typed_operation_completion().await.expect("one terminal completion witness").is_some() {}
        plugin_job_yield_once().await;
    }
    ownership_receipts
}

/// 🕰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16: the host's OWN call, with the ACK round trip the
/// reactor really has. `poll` routes an `Event::Message` acknowledgement at the TOP of a turn and only
/// then advances the operation, so the ACK a turn's presented page earns is submitted with the NEXT
/// continuation, never inside the turn that produced it. `stop_at_idle` picks which host call this is:
/// `false` is the renderer's `settlePluginTurn`, which also continues while it owes an acknowledgement;
/// `true` is every call that can only read the turn's STATUS — the eventless `drainTypedOperations` poll
/// and a `refresh-ui` settle whose surfaces are already published. Answers
/// `(terminal completions, continuations spent, first parked continuation)`, where "parked" is a
/// continuation that ended owing typed-operation work while the turn reported none runnable.
#[cfg(test)]
async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, stop_at_idle: bool, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> HostCallCensus {
    let mut ownership_receipts = Vec::new();
    artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original deferred admission receipts");
    let mut acknowledgements: Vec<TypedOperationResultToken> = Vec::new();
    let (mut completions, mut spent, mut parked) = (0usize, 0usize, None);
    let mut history_patches = 0usize;
    let mut terminal: Option<(TypedOperationResultLane, String)> = None;
    while spent < continuations {
        for token in std::mem::take(&mut acknowledgements) {
            app.acknowledge_typed_operation_result(token).expect("a presented result page accepts its exact token");
        }
        app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
        artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original deferred continuation receipts");
        while let Some(page) = app.take_typed_operation_result_page(receiver) {
            if matches!(page.lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault) {
                terminal = Some((page.lane, String::from_utf8_lossy(page.bytes()).into_owned()));
            }
            acknowledgements.push(page.token);
        }
        while app.take_typed_operation_effect().is_some() {}
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while let Some(completion) = app.take_typed_operation_completion().await.expect("one terminal completion witness") {
            completions += 1;
            history_patches += usize::from(completion.history_patch.is_some());
        }
        let runnable = app.has_runnable_typed_operations();
        if !runnable && app.has_pending_typed_operations() {
            parked.get_or_insert(spent);
        }
        spent += 1;
        if !runnable && (stop_at_idle || acknowledgements.is_empty()) {
            break;
        }
        plugin_job_yield_once().await;
    }
    HostCallCensus { ownership_receipts, completions, history_patches, spent, parked, terminal }
}

/// 🧾️ What one host call actually handed its caller: terminal completion witnesses, continuations spent,
/// the first continuation that ended owing typed-operation work while the turn reported none runnable,
/// and the terminal-lane page (`Terminal` or `Fault`) the call carried back, with its body. The body is
/// the difference between a ladder that PUBLISHED and one the fixture refused at preflight — ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B19 found wave B16's two laws green over the latter.
#[cfg(test)]
struct HostCallCensus {
    ownership_receipts: Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt>,
    completions: usize,
    history_patches: usize,
    spent: usize,
    parked: Option<usize>,
    terminal: Option<(TypedOperationResultLane, String)>,
}

/// 🛡️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: the non-vacuity guard every scheduling law over this
/// fixture owes. "No continuation was parked" and "nothing is left pending" are satisfied TRIVIALLY by a
/// ladder the app refused before it published anything — wave B19 found both of wave B16's laws green
/// over exactly that: the `KeyedTestApp`/`compositeEdit` emit was refused at
/// `TestCountOneItemPreparationFactory::preflight` and terminated on the `Fault` lane. A law about
/// scheduling a LANDING mutation must therefore state which lane its ladder earned.
#[cfg(test)]
fn assert_landing_terminal_lane(terminal: Option<&(TypedOperationResultLane, String)>, verb: &str, spent: usize) {
    let Some((lane, body)) = terminal else {
        panic!("'{verb}' reached no terminal lane at all within {spent} host continuations, so this law asserted nothing about a landing mutation");
    };
    assert_eq!(*lane, TypedOperationResultLane::Terminal, "'{verb}' terminated on the {lane:?} lane ({body:?}) within {spent} host continuations; a refused ladder publishes nothing, so this law would be vacuous over it");
}

/// ⏳️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16: a turn NEVER reports "no runnable typed operations"
/// while it still owes some. Only the folded status crosses the wire
/// (`⚛️reactor/🔄️turn/🦀️.rs:1171`, `:1229`), so a turn that answers `Idle` mid-ladder is invisible to
/// every host continuation that can read nothing else — the eventless `drainTypedOperations` poll and a
/// `refresh-ui` settle. [`PluginApp::has_runnable_typed_operations`] answered `false` for an
/// `AwaitingAck` operation whose page had been handed out, which is EVERY turn that publishes anything:
/// the operation was parked, and its edit landed on whatever host call happened to come next — measured
/// in the browser on wasm #46 as a one-command lag on every catalogue add, duplicate, delete and
/// volume-brush add (`📓️2026-09-11-wave-B14-settle-semantics.md` §6).
#[cfg(test)]
pub(crate) async fn test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    const CONTINUATIONS: usize = 512;
    const RECEIVER: u32 = 7;
    let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
    app.bind_instance_id(RECEIVER).await;
    let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
    admit_fixture_typed_action(&mut app, verb, Box::new(command("own-call", 1)), &meta, identity).await;
    let HostCallCensus { spent, parked, terminal, .. } = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, false, mounted_policy, identity).await;
    assert_eq!(parked, None, "continuation {parked:?} of {spent} ended owing typed-operation work while the turn reported none runnable; it {}", released_slot_census(&app, spent));
    assert!(!app.has_pending_typed_operations(), "the admitting call ended owing typed-operation work; it {}", released_slot_census(&app, spent));
    assert_landing_terminal_lane(terminal.as_ref(), verb, spent);
    close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
    eprintln!("[TRACE] actual one mutating typed operation kept its turn runnable through all {spent} continuations of the call that admitted it");
}

/// 🚰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16: a host call that can read nothing but the turn's
/// status still finishes the operation. The eventless `drainTypedOperations` poll and a `refresh-ui`
/// settle continue exactly while the guest answers `MoreWork`; a mounted operation that answers `Idle`
/// the moment it hands out a result page stops such a call at continuation 0 and stays parked, because
/// a presented page is deliberately never republished
/// (`presented result page must not be republished before an explicit retry deadline`). Only the turn's
/// own status can keep the acknowledgement coming, so the status is what this law pins.
#[cfg(test)]
pub(crate) async fn test_typed_operation_completes_under_a_status_only_host_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    const CONTINUATIONS: usize = 512;
    const RECEIVER: u32 = 7;
    let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
    app.bind_instance_id(RECEIVER).await;
    let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
    admit_fixture_typed_action(&mut app, verb, Box::new(command("drain-call", 2)), &meta, identity).await;
    let HostCallCensus { spent, parked, terminal, .. } = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, true, mounted_policy, identity).await;
    let drain = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, true, mounted_policy, identity).await;
    let drained = drain.spent;
    assert!(
        !app.has_pending_typed_operations(),
        "a status-only call of {spent} continuations (parked at {parked:?}) and one eventless drain call of {drained} left typed-operation work for the next command; it {}",
        released_slot_census(&app, spent + drained)
    );
    assert_landing_terminal_lane(terminal.as_ref().or(drain.terminal.as_ref()), verb, spent + drained);
    close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
    eprintln!("[TRACE] actual a status-only host call finished its typed operation in {spent} continuations, with {drained} drain continuations after it");
}

/// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B19: the host call that ADMITS a typed command is the
/// call that hands its TERMINAL lane back — and a terminal page on the `Terminal` lane owes exactly one
/// completion witness in the same call, because that witness is what carries the `history_patch` the
/// renderer turns into an `OperationCompleted` frame
/// (`🔌️PluginRuntime/🟦️.tsx` `subscribeOperationCompletions`). Browser reading on wasm #47: the
/// admitting call of `addObjectKind` came back `frameKinds:["Invocation","Ephemeral"]`,
/// `historyUpserts:0`, and `history patch applied … create-object` arrived inside the NEXT command's
/// settle — one command late, which is why every catalogue add, duplicate, delete and volume-brush add
/// reads unchanged at its own verdict (`📓️2026-09-12-wave-B19-mutation-lane-regression.md`).
///
/// The lane assertion is the second half of the law and the reason it is written as an equivalence: over
/// the `KeyedTestApp`/`compositeEdit` fixture wave B16's two laws use, the ladder is refused at
/// `TestCountOneItemPreparationFactory::preflight` and terminates on the `Fault` lane with no witness at
/// all, so those laws are green over a ladder that never published. This law names that refusal instead
/// of hiding it, and turns into the completion pin the moment the fixture reaches `Terminal`.
#[cfg(test)]
pub(crate) async fn test_typed_operation_lands_its_edit_inside_the_admitting_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    const CONTINUATIONS: usize = 512;
    const RECEIVER: u32 = 7;
    let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
    app.bind_instance_id(RECEIVER).await;
    let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
    admit_fixture_typed_action(&mut app, verb, Box::new(command("admitting-call", 3)), &meta, identity).await;
    let census = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, false, mounted_policy, identity).await;
    let (spent, parked) = (census.spent, census.parked);
    let Some((lane, body)) = census.terminal.clone() else {
        panic!("the admitting call of {spent} continuations (parked at {parked:?}) handed back no terminal-lane page for '{verb}' — the ladder finishes on a later host call; it {}", released_slot_census(&app, spent));
    };
    let expected = usize::from(lane == TypedOperationResultLane::Terminal);
    assert_eq!(
        census.completions,
        expected,
        "the admitting call of {spent} continuations carried back a {lane:?} page ({body:?}) but {} completion witness(es); a Terminal page owes exactly one, a Fault page none; it {}",
        census.completions,
        released_slot_census(&app, spent)
    );
    assert_eq!(
        census.history_patches,
        expected,
        "the admitting call of {spent} continuations handed back a {lane:?} page ({body:?}) and {} completion witness(es) but {} of them carried a history patch; the `create-object` row is what the renderer turns into the document frame, so a Terminal completion owes exactly one; it {}",
        census.completions,
        census.history_patches,
        released_slot_census(&app, spent)
    );
    close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
    eprintln!("[TRACE] actual the admitting call handed back the {lane:?} lane ({body:?}), {} completion witness(es) and {} history patch(es) within {spent} continuations", census.completions, census.history_patches);
}

/// 🎰️ The live slot census a leak report needs: how many classes are still occupied and, for each, WHICH
/// of the five authorities `typed_operation_slot_is_vacant` folds still owns it — a mounted operation and
/// its stage, a bare reservation, a pending latest-wins command, or a segmented download/closure. A census
/// that only listed mounted operations printed an empty list for the three other owners, which is the
/// shape a leaked reservation has (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21).
#[cfg(test)]
fn released_slot_census<A: ArtifactApp>(app: &VcsArtifactApp<A>, continuations: usize) -> String {
    let owners: Vec<String> = (0..ARTIFACT_LIVE_OUTPUT_SLOTS)
        .filter(|slot| !app.typed_operation_slot_is_vacant(*slot))
        .map(|slot| {
            let mounted = app.tool_operations.entry(slot).map(|(id, operation)| format!("mounted {id}:{:?}:presented={}", operation.stage, operation.result_page_presented));
            let reserved = app.typed_operation_reservations[slot].map(|id| format!("reserved {id}"));
            let pending = app.latest_wins_commands.entry(slot).map(|(id, _)| format!("latest-wins {id}"));
            let download = (!app.segmented_downloads.slot_is_vacant(slot)).then(|| "segmented-download".to_string());
            let closure = (!app.segmented_closures.slot_is_vacant(slot)).then(|| "segmented-closure".to_string());
            format!("{slot}:{}", [mounted, reserved, pending, download, closure].into_iter().flatten().collect::<Vec<String>>().join("+"))
        })
        .collect();
    format!("left {} of {ARTIFACT_LIVE_OUTPUT_SLOTS} typed-operation slots live after {continuations} host continuations {owners:?}", app.live_typed_operation_slots())
}

/// 🧹 Drains the fixture app's close ladder, which every `VcsArtifactApp` owes its retained store before
/// `Drop`. The cooperative rotation is walked FIRST and only here: these laws measure what the host
/// continuation alone releases, but the worker-session retirements a storm parks are PROCESS-global
/// (`semio_framework_job::worker_job_retirements_are_parked`, maintenance stage 23), so leaving them
/// behind pollutes every later test in the same binary — `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`
/// measures exactly that heap.
#[cfg(test)]
fn fixture_close_receipt<A:ArtifactApp>(app:&mut VcsArtifactApp<A>,mounted_policy:crate::MountedOwnerPolicyV1)->Result<PluginLifecycleStep,Fault>{
    let grant=mounted_policy.close;
    let demand=app.close_retirement_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?;
    crate::bounded_ownership::runtime_lifecycle_step(demand,grant,|grant|Ok((app.close_step(grant)?,app.close_terminal_is_empty())))
}

#[cfg(test)]
async fn close_fixture_app_to_terminal_emptiness<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, mounted_policy: crate::MountedOwnerPolicyV1) {
    for _ in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
        if !semio_framework_job::worker_job_retirements_are_parked() {
            break;
        }
        let _ = PluginApp::maintenance_step(app, mounted_policy.maintenance).unwrap();
        plugin_job_yield_once().await;
    }
    for _ in 0..1_000_000 {
        if app.close_terminal_is_empty() {
            break;
        }
        let _ = fixture_close_receipt(app, mounted_policy).unwrap();
        plugin_job_yield_once().await;
    }
    assert!(app.close_terminal_is_empty());
}

#[cfg(test)]
pub(crate) async fn test_retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = crate::publication_fixture::PublicationPresence, PresenceMutation = crate::publication_fixture::PublicationPresenceMutation> + Default>() {
    typed_command_full_operation_tests::retained_cancellation_publication_boundaries::<A>().await;
}

#[cfg(test)]
fn fixture_close_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant) -> PluginLifecycleStep {
    let _demand = app.close_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
    app.close_step(original_grant).unwrap()
}

#[cfg(test)]
fn fixture_maintenance_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant) -> PluginLifecycleStep {
    let _demand = app.maintenance_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
    app.maintenance_step(original_grant).unwrap()
}

#[cfg(test)]
pub(crate) async fn test_retained_latest_wins_slot_and_publication_fairness<A: ArtifactApp<Presence = crate::publication_fixture::PublicationPresence, PresenceMutation = crate::publication_fixture::PublicationPresenceMutation> + Default>() {
    typed_command_full_operation_tests::retained_latest_wins_slot_and_publication_fairness::<A>().await;
}

#[cfg(test)]
pub(crate) async fn test_retained_document_cancellation<A: ArtifactApp + Default>(
    factory: std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>,
    mutation: fn() -> A::Mutation,
    observe: fn(&A::Snapshot) -> i32,
) {
    typed_command_full_operation_tests::retained_document_cancellation::<A>(factory, mutation, observe).await;
}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs");
