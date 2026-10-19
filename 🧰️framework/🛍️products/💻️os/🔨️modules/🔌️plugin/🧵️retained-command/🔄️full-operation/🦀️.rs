//! 🔄️ Mounted typed command full operations and their job.
use semio_framework_plugin_operation::{ToolCancellationLeaseRetirement, original_lease_cursor_admit, original_lease_cursor_release};
use semio_framework_value::retirement::controlled::ControlledRetirement;
pub(crate) use semio_framework_actor::typed_operation_diagnostics as diagnostic_publication;

use semio_framework_value::DslValue;
use semio_framework::{Fault, FaultCode, FaultOrigin};
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::Mutation;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[cfg(test)]
use semio_framework_plugin_composition_publication::ChildPublicationResultV1;
#[cfg(test)]
use semio_framework_plugin_decl_emission::ChildEmit;
use semio_framework_plugin_ownership::bounded_ownership::{plugin_grant_funds, plugin_retirement_fault, plugin_turn_admitted};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJob;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use semio_framework_plugin_composition_publication::MountedWorkerFaultCapture;
use semio_framework_plugin_composition_publication::PendingArtifactStorePublication;
use semio_framework_plugin_composition_publication::PendingChildGroupPublication;
use semio_framework_plugin_composition_publication::PendingChildGroupPublicationPhase;
use semio_framework_plugin_composition_publication::advance_original_fault_capture;
use semio_framework_plugin_composition_publication::original_fault_capture_demands;
use semio_framework_plugin_composition_publication::original_reserved_arc_frame_bytes;
use semio_framework_plugin_operation::ToolCancellationHandle;
use semio_framework_plugin_operation::ToolCancellationLease;
use semio_framework_plugin_decl_app::tool_contract::ArtifactBoundedToolFault;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolRawInput;
use semio_framework_plugin_decl_app::tool_contract::TYPED_OPERATION_FAULT_PAGE_BYTES;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_framework_plugin_job::TypedOperationResultPage;
use semio_framework_plugin_job::TypedOperationResultToken;
#[cfg(test)]
use semio_framework_plugin_decl_emission::Emit;



#[doc(hidden)]
pub struct MountedTypedCommandFullOperation<A: ArtifactApp> {
    pub verb: String,
    pub meta: ActionMeta,
    pub operation: semio_framework_job::Operation,
    pub source_identity:semio_framework_job::OriginalSourceBinding,
    pub canonical_revision: [u8; 32],
    pub artifact_generation: u64,
    pub config_generation: u64,
    pub draft_generation: u64,
    pub presence_generation: u64,
    pub transient_generation: u64,
    pub window_config_authority: Option<semio_framework_plugin_window_config::WindowConfigAuthority>,
    pub window_transient_authority: Option<semio_framework_plugin_window_transient::window_transient::WindowTransientAuthority>,
    pub publication_lanes: &'static [ArtifactToolPublicationLane],
    /// 🔒️ The admitted tool's exact execution contract, which bounds each worker step to `max_work_units_per_step` work units and `max_step_micros` microseconds; a completed-output mount never runs a worker and retains none.
    pub contract: Option<semio_framework::ToolExecutionContract>,
    pub session: Option<semio_framework_job_worker::MountedWorkerJobSession<semio_framework::ErasedToolJob>>,
    pub session_rejected: Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<semio_framework::ErasedToolJob>>,
    pub(crate) producer_source: Option<semio_framework::action_bus::OriginalToolSourceBinding>,
    pub(crate) producer_request: Option<semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobRequest<A>>,
    pub(crate) producer_request_retirement: Option<ControlledRetirement<semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobRequest<A>>>,
    pub reserved_producer: Option<ArtifactReservedToolJob>,
    pub completion: Option<ArtifactToolCompletion<A>>,
    pub completion_retirement:Option<ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub publication_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub output_retirement:Option<ControlledRetirement<ArtifactOutputChunks>>,
    pub raw_input: Option<ArtifactToolRawInput>,
    pub output_chunks: Option<ArtifactOutputChunks>,
    pub cancellation_lease: Option<ToolCancellationLease>,
    pub identity: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    pub identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress,
    pub worker_semantic_pending: bool,
    pub worker_outcome_pending: bool,
    pub worker_fault_capture: Option<MountedWorkerFaultCapture>,
    pub terminal_seen: bool,
    pub worker_resume_pending: bool,
    pub publication: Option<ArtifactToolCompletionValue<A>>,
    pub pending_artifact_publication: Option<PendingArtifactStorePublication<A>>,
    pub pending_publication_outcome:PendingPublicationOutcome,
    pub pending_window_config_receipt:Option<store::os_store::presence::LaneItemReceipt>,
    pub cancellation_retirement:Option<Box<ToolCancellationLeaseRetirement>>,
    pub pending_child_publication: Option<PendingChildGroupPublication<A>>,
    pub owned_child_group: Option<u64>,
    pub owned_child_committed: bool,
    pub owned_child_result_pending: bool,
    pub captured_child_content: Option<std::sync::Arc<ChildContentView>>,
    pub captured_child_content_generation: u64,
    pub result_page: Option<TypedOperationResultPage>,
    pub diagnostics: Option<diagnostic_publication::OperationDiagnosticPublication>,
    pub result_page_presented: bool,
    pub result_sequence: u32,
    /// 🧮️ The publication ladder's OWN monotone progress: how many publication units actually
    /// moved this operation's pending store publication, counted by the store's checkpoint
    /// rather than by the unit having been spent. A healthy ladder that folds one mutation per
    /// unit raises it on every unit; an owner that cannot advance never raises it at all, which
    /// is exactly the term [`Self::typed_operation_stall_witness`]'s three facts were missing.
    pub publication_progress: u64,
    /// 🧮️ The last checkpoint the pending publication answered with, so a unit that repeats it
    /// verbatim is recognised as no movement at all instead of as progress.
    pub publication_checkpoint: Option<store::os_store::one_item::ArtifactStoreOneItemCheckpoint>,
    pub publication_ownership_progress: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub original_retirement_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub actor_capture: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
    pub publication_attempt: u8,
    pub ui_pending: bool,
    pub progress: Option<u64>,
    pub progress_pending: bool,
    pub user_cancel_requested: bool,
    pub published_artifact: bool,
    pub published_config: bool,
    /// 🪟 The per-window config lane published. A `View` whose only durable emission is this lane
    /// is session view state, and [`Self::record_settled_typed_operation_command`] leaves it out
    /// of the artifact command history — the same exception [`Self::dispatch_emit_inner`] makes.
    pub published_window_config: bool,
    pub command_logged: bool,
    pub interaction_revalidated: bool,
    pub terminal_fault: Option<ArtifactBoundedToolFault>,
    pub retained_close_fault: Option<Fault>,
    pub retained_close_fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub retained_close_fault_refusal:Option<ValueError>,
    pub stage: MountedTypedCommandFullOperationStage,
}

impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
 pub fn lease_retirement_ready(&self)->bool{self.stage==MountedTypedCommandFullOperationStage::Retiring&&self.session.is_none()&&self.producer_request.is_none()&&self.producer_request_retirement.is_none()&&self.session_rejected.is_none()&&!self.worker_semantic_pending&&!self.worker_outcome_pending&&self.worker_fault_capture.is_none()&&self.publication.is_none()&&self.publication_retirement.is_none()&&self.pending_artifact_publication.is_none()&&!self.pending_publication_outcome.pending()&&self.pending_child_publication.is_none()&&self.owned_child_group.is_none()&&self.captured_child_content.is_none()&&self.result_page.is_none()&&self.pending_window_config_receipt.is_none()&&self.raw_input.is_none()&&self.output_chunks.is_none()&&self.output_retirement.is_none()&&self.reserved_producer.is_none()&&self.completion.is_none()&&self.completion_retirement.is_none()&&self.meta.view_state.is_none()&&self.window_config_authority.is_none()&&self.window_transient_authority.is_none()&&self.actor_capture.is_none()&&self.verb.capacity()==0&&!self.meta.actor.has_owner()&&self.retained_close_fault.is_none()&&(self.cancellation_lease.is_some()||self.cancellation_retirement.is_some())}
 pub fn lease_retirement_demands(&self,canonical:&ToolCancellationHandle,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.cancellation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}else{owner.retirement_demands(canonical,body)};}Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}
 pub fn lease_retirement_step(&mut self,canonical:&ToolCancellationHandle,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.lease_retirement_demands(canonical,grant.maximum_copy_bytes)?;if !plugin_grant_funds(demand,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(owner)=self.cancellation_retirement.as_mut(){if owner.terminal_is_empty(){return original_lease_cursor_release(&mut self.cancellation_retirement,grant).map(RetainedCloneStep::Progress);}return owner.close_step(canonical,grant);}
  let(owner,progress)=original_lease_cursor_admit(&mut self.cancellation_lease,grant)?;self.cancellation_retirement=owner;Ok(RetainedCloneStep::Progress(progress))
 }
}

include!("../🪟️mounted/♻️frontier/🦀️.rs");


/// 🪢️ Returns the worker's original cancellation ancestor alias against the lease root the operation still owns, since that root can never become unique while the operation retains it.
fn return_session_cancel_alias(session: &semio_framework_job_worker::MountedWorkerJobSession<semio_framework::ErasedToolJob>, lease: &Option<ToolCancellationLease>, grant: RetainedCloneGrant) -> Option<Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure>> {
    let lease = lease.as_ref()?;
    matches!(session.has_original_cancel_alias_witness(&lease.token), Ok(true)).then(|| match session.return_original_cancel_alias_step(&lease.token, grant) {
        Ok(Some(step)) => Ok(PluginLifecycleStep::Progress(step.progress())),
        Ok(None) | Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => Ok(PluginLifecycleStep::AwaitingInput { reason: "typed operation original cancellation alias awaits its issuing root" }),
        Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => Err(error.into()),
    })
}

impl<A: ArtifactApp> MountedTypedCommandFullOperation<A> {
    /// 🚨️ The retained refusal of an owner that has no retirement path at all: neither a preparation that was refused at birth nor a refused fault close can ever succeed on retry, so the operation would otherwise stay pending without a word.
    pub fn retained_structural_refusal(&self) -> Option<String> {
        if let Some(fault) = self.retained_close_fault.as_ref() {
            return Some(format!("retains its original close fault: {}: {}", fault.code.0, fault.message));
        }
        let refusal = self.pending_artifact_publication.as_ref().and_then(|publication| publication.preparation_refusal()).or(self.retained_close_fault_refusal.as_ref())?;
        (refusal.kind == semio_framework_value::ValueRefusalKind::UnsupportedOwner).then(|| format!("is refused by an owner without a retirement path: {refusal}"))
    }

    /// 🎭️ Yields the actual actor capture receipt before admitting any durable mutation input.
    pub fn admit_actor_capture(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
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
    pub fn note_publication_checkpoint(&mut self, checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint) {
        if self.publication_checkpoint != Some(checkpoint) {
            self.publication_checkpoint = Some(checkpoint);
            self.publication_progress = self.publication_progress.saturating_add(1);
        }
    }

    /// 🚫️ A rejected worker admission never ran the job, so no completion value will ever
    /// arrive: publication would spin on `completion.take() == None` for as long as the host
    /// keeps advancing it. The operation is terminal the moment its session is refused —
    /// cancel the lease so the next publication unit mints the exact fault page.
    pub fn drive_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        self.validate_original_producer_source(cx)?;
        if self.stage != MountedTypedCommandFullOperationStage::Worker {return Ok(PluginLifecycleStep::Blocked{reason:"typed command awaits its bounded revision-validated publication or ACK turn"});}
        if !self.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted worker requires its original source identity independently of its registry record").into())}
        if (self.producer_request.is_some()||self.original_reserved_producer_pending())&&!self.user_cancel_requested&&!self.terminal_seen&&!cx.is_cancelled(){return self.drive_original_producer(cx);}
        let grant=cx.retained_grant();let demand=if cx.is_cancelled()&&(self.original_producer_pending()||self.session_rejected.as_ref().is_some_and(|source|source.admission_pending())){self.worker_retirement_demands(grant.maximum_copy_bytes)?}else{self.worker_step_demands(grant.maximum_copy_bytes)?};
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        if self.stage != MountedTypedCommandFullOperationStage::Worker {return Ok(PluginLifecycleStep::Blocked{reason:"typed command awaits its bounded revision-validated publication or ACK turn"});}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        let contract=self.contract.ok_or_else(||plugin_sdk_fault("typed operation worker requires its admitted execution contract"))?;
        if self.original_producer_pending(){return self.retire_original_producer(grant);}
        if let Some(rejected)=self.session_rejected.as_mut(){
            if rejected.admission_pending()&&!cx.is_cancelled()&&!self.user_cancel_requested&&!self.terminal_seen{
                let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_framework_reserved_route",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)};
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
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{let step=rejected.close_step(cx)?.admit(grant,rejected.terminal_is_empty())?;Ok((match step{
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
                Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>return Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation original worker close quote awaits its handback"}),
                Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(error.into())
            };
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{let step=session.close_step(cx)?;Ok((match step{
                semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{progress}if progress.fits(grant)=>PluginLifecycleStep::Progress(progress),
                semio_framework_job_worker_authority::WorkerJobCloseStep::Complete{progress}if progress.fits(grant)&&session.terminal_is_empty()=>PluginLifecycleStep::Progress(progress),
                semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked=>PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted session close awaits its worker handback"},

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
        let step_fuel=cx.fuel_remaining().min(contract.max_work_units_per_step);
        let step_deadline_us=cx.now_us().map_or(cx.deadline_us(),|now|now.saturating_add(u64::from(contract.max_step_micros)));
        match cx.with_original_work_loan(step_fuel,step_deadline_us,|same|session.pump_one_for_interactive_turn(same)){
            Ok(semio_framework_job_worker::WorkerJobPoll::Outcome|semio_framework_job_worker::WorkerJobPoll::Terminal)=>{
                let Some((issued,progress))=session.take_checked_out_retained_step_receipt()else{return Ok(PluginLifecycleStep::Blocked{reason:"typed operation original worker receipt awaits its sole recipient"});};
                self.worker_semantic_pending=true;
                if !progress.fits(issued)||!progress.fits(grant)||issued.maximum_items>grant.maximum_items||issued.maximum_copy_bytes>grant.maximum_copy_bytes||issued.maximum_capacity_bytes>grant.maximum_capacity_bytes||issued.maximum_release_bytes>grant.maximum_release_bytes||issued.maximum_depth>grant.maximum_depth{return Err(plugin_sdk_fault("typed operation original worker receipt exceeded its received turn").with_retained_progress(progress).into());}
                if matches!(session.checked_out_outcome(),Ok(None))&&session.resume().is_ok(){self.worker_semantic_pending=false;}
                Ok(PluginLifecycleStep::Progress(progress))
            }
            Ok(semio_framework_job_worker::WorkerJobPoll::Submitted|semio_framework_job_worker::WorkerJobPoll::Rejected)=>Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),
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

    pub fn next_token(&self) -> TypedOperationResultToken {
        TypedOperationResultToken { receiver: self.meta.instance_id, operation: self.operation.operation.0, generation: self.operation.generation.0, sequence: self.result_sequence, attempt: 1 }
    }

    pub fn queue_page(&mut self, page: TypedOperationResultPage) -> Result<(), Fault> {
        if self.stage != MountedTypedCommandFullOperationStage::Publishing || self.result_page.is_some() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-state"), "typed operation can retain exactly one unacknowledged result page"));
        }
        self.result_page = Some(page);
        self.result_page_presented = false;
        self.stage = MountedTypedCommandFullOperationStage::AwaitingAck;
        Ok(())
    }

    pub fn reject_cancelled_publication(&mut self) -> Result<bool, Fault> {
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
                let lane = semio_framework_plugin_decl_app::cancellation_result_lane(self.user_cancel_requested, false);
                let detail: &[u8] = if lane == TypedOperationResultLane::Terminal { b"" } else { b"typed-operation cancelled before its next publication unit" };
                TypedOperationResultPage::try_new(self.next_token(), lane, detail)?
            }
        };
        self.queue_page(page)?;
        Ok(true)
    }

    pub(crate) fn result_page_presentation_demands(&self)->Result<RetirementDemand,ValueError>{self.result_page.as_ref().map_or(Ok(Default::default()),|page|{let mut d=page.presentation_demand()?;d.copy_bytes+=size_of::<bool>();Ok(d)})}
    pub fn take_result_page(&mut self,grant:RetainedCloneGrant) -> Result<(Option<TypedOperationResultPage>,RetainedCloneProgress),Fault> {
        if self.result_page_presented { return Ok((None,Default::default())); }
        let demand=self.result_page_presentation_demands().map_err(plugin_retirement_fault)?;
        if !plugin_turn_admitted(demand,grant)?{return Ok((None,Default::default()));}
        let Some(page)=self.result_page.as_mut()else{return Ok((None,Default::default()))};
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<bool>(),..grant};
        let(result,mut progress)=page.present(child).map_err(plugin_retirement_fault)?;
        if result.is_some(){self.result_page_presented=true;progress.copied_bytes+=size_of::<bool>();}
        Ok((result,progress))
    }

    /// 🧪️ Supplies each production presentation turn with the fixture issuer's unchanged grant.
    #[cfg(test)]
    pub(crate) fn take_fixture_result_page(&mut self,grant:RetainedCloneGrant)->Option<TypedOperationResultPage>{
        for _ in 0..2{let((page,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.take_result_page(grant).expect("original fixture page presentation"));assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if page.is_some()||progress==RetainedCloneProgress::default(){return page;}}
        panic!("original fixture page sealing omitted its next projection");
    }
    /// 🧪️ Pays the original page retirement through the production close port after exact ACK.
    #[cfg(test)]
    pub(crate) fn close_fixture_original_page(&mut self,grant:RetainedCloneGrant){for _ in 0..100000{if !self.result_page_retirement_pending(){return;}let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.result_page_retirement_step(grant).expect("original fixture producer page close"));assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_ne!(p,Default::default(),"original fixture close grant funds the exact producer page");}panic!("original fixture producer page did not close");}
    /// ♻️ Exact ACK retains the original page until its funded physical closure.
    pub(crate) fn result_page_retirement_pending(&self)->bool{self.result_page.is_some()&&self.stage!=MountedTypedCommandFullOperationStage::AwaitingAck}
    pub(crate) fn result_page_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.result_page.as_ref().map_or(Ok(Default::default()),|page|{let mut d=page.next_close_demand(body.saturating_sub(size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>()))?;d.copy_bytes+=size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>();Ok(d)})}
    pub(crate) fn result_page_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,Fault>{
        if !self.result_page_retirement_pending(){return Ok(Default::default());}
        let demand=self.result_page_retirement_demands(grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;if !plugin_turn_admitted(demand,grant)?{return Ok(Default::default());}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<TypedOperationResultPage>>()-size_of::<bool>(),..grant};let page=self.result_page.as_mut().unwrap();let mut progress=page.close_step(child).map_err(plugin_retirement_fault)?.progress();
        if page.terminal_is_empty(){self.result_page=None;self.result_page_presented=false;progress.copied_bytes+=size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>();}
        Ok(progress)
    }

    pub fn acknowledge_result_page(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault> {
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
        if lane == TypedOperationResultLane::Diagnostic && !self.diagnostics.as_mut().is_some_and(|owner|owner.acknowledge(token)) {
            return Err(plugin_sdk_fault("diagnostic result ACK lost its exact original source authority"));
        }
        if lane == TypedOperationResultLane::Child && self.owned_child_committed { self.owned_child_result_pending = false; }
        self.result_sequence = self.result_sequence.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-sequence"), "typed-operation result sequence exhausted"))?;
        self.stage = if matches!(lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault | TypedOperationResultLane::Download) {
            MountedTypedCommandFullOperationStage::Retiring
        } else {
            MountedTypedCommandFullOperationStage::Publishing
        };
        if self.stage==MountedTypedCommandFullOperationStage::Retiring{if let Some(owner)=self.diagnostics.as_mut(){owner.begin_close();}}
        Ok(true)
    }

    /// 📏️ Quotes the exact next [`Self::drive_worker_step`] turn of a Worker-stage operation.
    pub fn worker_step_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if self.stage != MountedTypedCommandFullOperationStage::Worker { return Ok(Default::default()); }
        if self.original_producer_pending(){return if self.user_cancel_requested||self.terminal_seen{self.original_producer_retirement_demands(body)}else{Ok(item)};}
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
        self.original_producer_pending() || self.session.is_some() || self.session_rejected.is_some() || self.worker_semantic_pending || self.worker_outcome_pending || self.worker_fault_capture.is_some()
    }

    /// 📏️ Quotes the exact next [`Self::retire_worker_step`] turn.
    fn worker_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if self.original_producer_pending(){return self.original_producer_retirement_demands(body);}
        if let Some(rejected) = self.session_rejected.as_ref() {
            if rejected.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(rejected), ..item }); }
            return rejected.retirement_demands(body);
        }
        if let Some(session) = self.session.as_ref() {
            if session.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(session), ..item }); }
            return match session.retirement_demands(body) {
                Ok(demand) => Ok(demand),
                Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => Ok(item),
                Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => Err(error),
            };
        }
        Ok(item)
    }

    /// ♻️ Closes the original mounted worker session, its refused admission and its inline capture markers under the unchanged grant.
    fn retire_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        self.validate_original_producer_source(cx)?;
        let grant=cx.retained_grant();let demand=self.worker_retirement_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.original_producer_pending(){return self.retire_original_producer(grant);}
        if let Some(rejected) = self.session_rejected.as_mut() {
            if rejected.terminal_is_empty() {
                let bytes = size_of_val(rejected);
                drop(self.session_rejected.take());
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_bytes: bytes, ..item }));
            }
            let demand = rejected.retirement_demands(grant.maximum_copy_bytes)?;
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant| {
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
                Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => return Ok(PluginLifecycleStep::AwaitingInput { reason: "typed operation original worker close quote awaits its handback" }),
                Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => return Err(error.into()),
            };
            return semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant| {
                let step = session.close_step(cx)?;
                Ok((match step {
                    semio_framework_job_worker_authority::WorkerJobCloseStep::Pending { progress } if progress.fits(grant) => PluginLifecycleStep::Progress(progress),
                    semio_framework_job_worker_authority::WorkerJobCloseStep::Complete { progress } if progress.fits(grant) => PluginLifecycleStep::Progress(progress),
                    semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked => PluginLifecycleStep::AwaitingInput { reason: "typed operation mounted session close awaits its worker handback" },

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
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if self.result_page_retirement_pending(){return self.result_page_retirement_demands(body);}
        if self.close_fault_pending() { return self.close_fault_demands(body); }
        if self.worker_retirement_pending() { return self.worker_retirement_demands(body); }
        if let Some(owner)=self.diagnostics.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<diagnostic_publication::OperationDiagnosticPublication>>(),..item})}else{owner.next_demand(body)}}
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
    pub fn retirement_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        self.validate_original_producer_source(cx)?;
        let grant=cx.retained_grant();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        let empty = RetainedCloneProgress::default();
        let item = RetainedCloneProgress { copied_items: 1, ..empty };
        if self.stage != MountedTypedCommandFullOperationStage::Retiring { return Ok(PluginLifecycleStep::Blocked { reason: "typed operation is not admitted for retirement" }); }
        if self.result_page_retirement_pending(){return self.result_page_retirement_step(grant).map(PluginLifecycleStep::Progress);}
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
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.stage == MountedTypedCommandFullOperationStage::Retiring { self.retirement_demands(body) }
        else { Ok(RetirementDemand { depth: 1, ..Default::default() }) }
    }

    /// 🎟️ Withdraws inline result custody before closing retained originals.
    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();let demand=self.close_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            let step=(||->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !plugin_grant_funds(demand, grant) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if self.stage == MountedTypedCommandFullOperationStage::Retiring { return self.retirement_step(cx); }
        self.user_cancel_requested = true;
        if let Some(owner)=self.diagnostics.as_mut(){owner.begin_close();}
        self.result_page = None;
        self.result_page_presented = false;
        self.ui_pending = false;
        self.progress_pending = false;
        self.stage = MountedTypedCommandFullOperationStage::Retiring;
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    
            })()?;Ok((step,self.terminal_is_empty()))
        }).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.stage == MountedTypedCommandFullOperationStage::Retiring
            && self.session.is_none()
            && self.producer_request.is_none()
            && self.producer_request_retirement.is_none()
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
            && self.diagnostics.is_none()
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

#[doc(hidden)]
pub fn bounded_json_items(value: Option<&DslValue>, maximum: usize) -> Result<usize, Fault> {
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


























pub use semio_framework_plugin_job::lifted_full_operation_job::*;

include!("🏭️producer/🦀️.rs");

include!("🏭️producer/🦀️.rs");
