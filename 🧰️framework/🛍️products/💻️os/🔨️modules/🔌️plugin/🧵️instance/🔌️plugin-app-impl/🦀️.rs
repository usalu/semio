//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.

use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep, plugin_retirement_fault, plugin_turn_admitted};
use semio_plugin_decl_media::{ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES, ArtifactDownloadOutput, ArtifactMediaExportHandle, ArtifactMediaExportPoll};
use semio_plugin_decl_media::original_media::{RawMediaPicker, issue::pending_picker_effect, runtime::RawLiveDocument};
use semio_framework::{ActionKind, AppRole, Fault, FaultCode, FaultFrom, FaultOrigin, Media, MediaClass, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaType, MediaWireFormat, ViewModel};
use semio_plugin_decl_emission::{ActionMeta, ChildEmit, Emit};
use semio_plugin_document_media::{ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadState, MediaArtifact, copy_media_text_parts, document_archive_history_decoder, whole_document_media};
use semio_framework::kernel::{AppEvent, Effect, FRAMEWORK_RESERVED_JOB_KIND, FixedCommandPage, HistoryPatch, InvocationResult, UiDirtyScope};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_BUSY_CODE, DOCUMENT_LOAD_CHANGED_CODE, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_HISTORY_INVALID_CODE, DOCUMENT_LOAD_INCOMPLETE_CODE, DOCUMENT_LOAD_MEMBER_INVALID_CODE, DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, DOCUMENT_LOAD_TOO_LARGE_CODE, DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, EphemeralSnapshot, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption, NATURAL_FILE_PORT, TransactionPrepareOutcome, document_load_fault, media_projection_error, natural_file_input_media, whole_document_archive};
use store::os_store::store_schema::{ArtifactCommand, ArtifactEnvelope};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_plugin_store_lifecycle::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use semio_plugin_decl_app::tool_contract::{ArtifactToolCompletionValue, ArtifactToolPublicContract, ToolOwnerWitness, TypedOperationResultLane};
use semio_plugin_decl_views::{ArtifactView, ConfigView, InteractionView};
use std::collections::{BTreeMap, HashMap};
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_ui_runtime::ComponentTree;
use semio_plugin_composition_publication::{ComposedGestureResult, MountedTypedCommandFullOperationStage};
use semio_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, FRAMEWORK_HISTORY_BODY_KEY, WindowEngagement, WindowMeasure};
use semio_framework_value::{DslValue, RetirementDemand, ValueError};
use semio_plugin_job::{EmitWire, PendingTransaction, TransactionProposalDraft, TypedOperationCompletion, TypedOperationResultPage, TypedOperationResultToken, TypedOperationUiProgress};
use semio_plugin_job::reserved_jobs::FrameworkReservedCommitProgress;
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use crate::{InstanceExtensions, VcsArtifactApp};
use semio_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework::manifest::CommandAddress as ManifestCommandAddress;
use std::mem::ManuallyDrop;
use semio_plugin_instance_docs::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, ReferenceSpaceMember, SpaceMember};
use semio_plugin_mounted_owner::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
use protocol::OpText;
use semio_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
use semio_plugin_peer::{PeerRosterPublication, PresenceRosterAdmission, PresenceRosterOutcome};
use semio_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_plugin_window_transient::window_transient::TransientView;
use semio_plugin_window_config::WindowConfigPack;
use semio_plugin_window_transient::built_to_component_tree;
use store::os_store::document_materialize::create_document_envelope;
use semio_plugin_instance_dispatch::dispatch_emit;
use semio_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job_counted;
use semio_plugin_window_kits::{merge_ui_values, validate_ui_dispatch_classification};
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_instance_render as render;
use semio_plugin_time_travel_panel::command_history_panel_lift::ui_history_panel;
use semio_ui_contract::*;
use semio_plugin_tool_run::TOOL_RUN_TRACE_DELTA_BYTES;

use semio_plugin_instance_commands::commands_instance_root::framework_reserved_action_kind;
use semio_plugin_instance_jobs::jobs_instance_root::OriginalRenderTreeIdentity;

#[path = "🔒️close-terminal/🦀️.rs"]
mod close_terminal;

#[path = "🕹️ephemeral-interaction/🦀️.rs"]
mod ephemeral_interaction;

#[path = "🎞️media-transfer/🦀️.rs"]
mod media_transfer;

#[path = "🔖️reserved-mount/🦀️.rs"]
mod reserved_mount;

#[path = "🎟️picker-publication/🦀️.rs"]
mod picker_publication;
#[path = "🎟️natural-publication/🦀️.rs"]
mod natural_publication;

/// 🧪 Guest laws in dependent crates (puzzle 3d) settle Isolated reserved admits the host would.
pub async fn settle_framework_reserved_admission<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult,cx:&mut StepContext<'_>) -> Result<InvocationResult,OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok(admitted);
    };
    let output=drive_framework_reserved_registry_job(job,input,cx)?;
    settle_framework_reserved_commit(app,job,Ok(output),cx).await.map_err(Into::into)
}

/// 🪜️ Most units one reserved commit may take in a settle law — one per composed child or undone
/// edit, well above anything a fixture composes.
pub const FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS: usize = 4_096;

/// 🔖️ Hands a finished reserved spawn-job to `app`'s commit queue and drives the commit one unit
/// per call — the same units the reactor's typed-operation continuation runs across turns — to its
/// outcome.
pub async fn settle_framework_reserved_commit<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, job: u64, output: Result<Vec<u8>, Fault>,cx:&mut StepContext<'_>) -> Result<InvocationResult,OriginalTurnFailure> {
    if !app.pending.admit_framework_reserved_commit(&mut app.history, job, output)? {
        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-missing"), "framework reserved spawn-job had no pending admit").into());
    }
    for _ in 0..FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS {
        app.command_ctx().step_framework_reserved_commit(cx).await?;
        if let Some(outcome) = app.pending.take_framework_reserved_commit_outcome() {
            return outcome.map_err(Into::into);
        }
    }
    Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stall"), format!("framework reserved commit did not finish within {FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS} units")).into())
}

/// 💼️ Browser `driveSpawnedJob` contract: the isolated reserved dummy job is admitted from its original request and driven
/// by funded `step_job` turns to its terminal output, which is then settled through the reserved commit; the turn count is returned.
#[cfg(any(test, feature = "artifact-app-testing"))]
pub async fn drive_framework_reserved_spawn_like_host<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult,cx:&mut StepContext<'_>) -> Result<(InvocationResult,u32),OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok((admitted, 0));
    };
    let (output, steps) = drive_framework_reserved_registry_job_counted(job, input, cx)?;
    let settled = settle_framework_reserved_commit(app, job, Ok(output),cx).await?;
    Ok((settled, steps))
}

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M> {
    fn admit_cancellation_issuer(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let parent=cx.original_source_parent()?;self.tools.tool_cancellations.admit_step(parent,cx)}

    fn birth_identity(&self)->Option<(u32,String)>{self.tools.live_runtime_instance_id.map(|id|(id,self.stores.store.local_actor_id().0.to_string()))}

    fn mounted_owner_maintenance_phase(&self) -> Result<MountedOwnerPhaseV1, Fault> {
        let instance_id = self.tools.live_runtime_instance_id.ok_or_else(|| plugin_sdk_fault("mounted phase requires the live instance"))?;
        if self.ingest.document_blocks_pending_effects() {
            return Ok(MountedOwnerPhaseV1::Maintenance);
        }
        Ok(A::mounted_job_maintenance_phase(instance_id).unwrap_or(MountedOwnerPhaseV1::Maintenance))
    }

    fn mounted_owner_maintenance_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let identity=turn.identity();if self.tools.live_runtime_instance_id!=Some(identity.instance_id)||!matches!(turn.phase(),MountedOwnerPhaseV1::Preparation|MountedOwnerPhaseV1::Maintenance){return Err(plugin_sdk_fault("mounted maintenance does not own this live instance and phase").into())}
        let step = turn.advance_covering_lifecycle(PluginLifecycleStep::with_progress, |cx, loan| {
            if self.ingest.document_blocks_pending_effects()
                || A::mounted_job_maintenance_phase(identity.instance_id) != Some(MountedOwnerPhaseV1::Preparation)
            {
                return self.maintenance_step(cx, loan);
            }
            let demand = A::mounted_job_maintenance_demands(identity.instance_id, loan.maximum_copy_bytes)?;
            semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx, loan, demand, |same, actual| {
                Ok((A::mounted_job_maintenance_step(identity.instance_id, same, actual)?, true))
            })
        })?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.maintenance_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn mounted_owner_close_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let identity=turn.identity();if self.tools.live_runtime_instance_id!=Some(identity.instance_id)||turn.phase()!=MountedOwnerPhaseV1::Close{return Err(plugin_sdk_fault("mounted close does not own this live instance and phase").into())}
        let step = turn.advance_close_batch(PluginLifecycleStep::with_progress, |cx, loan| {
            if A::mounted_jobs_terminal_is_empty(identity.instance_id) {
                return self.close_step(cx, loan);
            }
            let demand = A::mounted_job_close_demands(identity.instance_id, loan.maximum_copy_bytes)?;
            semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx, loan, demand, |same, actual| {
                Ok((A::mounted_job_close_step(identity.instance_id, same, actual)?, A::mounted_jobs_terminal_is_empty(identity.instance_id)))
            })
        })?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.close_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner)=self.original_controller_close.as_ref().filter(|owner|!(owner.terminal)(&self.app)){return (owner.demands)(&self.app,body)}
        if self.constructor.has_constructor_receipt() { return Ok(Default::default()); }
        if !self.constructor.constructor_terminal_is_empty() { return self.stores.constructor_close_demands(&self.constructor, body); }
        if let Some(source) = self.media.natural_source.as_ref().filter(|_| self.natural_source_can_close()) {
            return source.close_demands(body);
        }
        self.close_ladder_demand(body)
    }

    fn maintenance_retirement_demands(&self, body: usize,maximum_work_units:u64) -> Result<RetirementDemand, ValueError> {
        self.ladder_ref().maintenance_ladder_demands(body,maximum_work_units)
    }

    fn maintenance_source_close_demands(&self,body:usize,cx:&StepContext<'_>)->Result<RetirementDemand,ValueError>{
        if semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::owns_birth(&self.pending.reserved_preparation,&self.pending.reserved_birth,cx){return semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::slot_close_demands(&self.pending.reserved_preparation,&self.pending.reserved_birth,body)}
        if self.owns_original_reserved_mount(cx){return self.reserved_mount_close_demands(body)}
        if self.owns_original_reserved_permit(cx){return if self.reserved_permit_waits(cx){Ok(Default::default())}else{self.media.original_tool_lease_close_demands(&self.tools,body)}}
        if self.host_event_routing_ref().owns_original_host_event_source(cx){return self.host_event_routing_ref().original_host_event_close_demands(body)}
        if self.command_lease_release_ready(cx){return Ok(Default::default())}
        if !self.host_event_routing_ref().owns_original_command_scope(cx)&&self.ingress.owns_original_action_ingress(cx){return self.ingress.original_action_ingress_close_demands(body,cx)}
        if self.tools.original_tool_tag_capture.is_some()||self.tools.original_tool_tag_retirement.is_some()||self.tools.original_scrub_dispatch.is_some()||self.tools.original_press_settlement.is_some()||self.tools.original_typing_settlement.is_some()||self.tools.original_tool_follow.is_some(){return self.render_ref().tool_overlay_close_demand(body)}
        if self.original_typed_tag.is_some()||!self.original_typed_tag_close.is_empty(){return self.original_typed_tag_close_demands(body)}
        if self.original_typed_command.pending(){return semio_plugin_instance_dispatch::dispatch_runtime_command::original_typed_source_close_demands(&self.original_typed_command,body)}
        if self.original_typing.pending(){return self.original_typing.close_demands()}
        if self.original_emission.pending(){return self.original_emission.close_demands()}
        if self.host_events.routes.cache_busy(){return self.host_events.routes.capture_close_demands(body)}
        if self.host_events.delivering||!self.host_events.action_delivery.terminal_is_empty(){return self.host_event_routing_ref().action_host_event_close_demands(body)}
        Ok(Default::default())
    }
    fn maintenance_source_close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let demand=self.maintenance_source_close_demands(loan.maximum_copy_bytes,cx)?;
        semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,actual|{
            if self.maintenance_source_terminal_is_empty(same){return Ok((PluginLifecycleStep::Complete(Default::default()),true))}
            if self.command_lease_release_ready(same){
                self.host_event_routing_ctx().release_original_command_scope(same);
                let terminal=self.maintenance_source_terminal_is_empty(same);
                return Ok((if terminal{PluginLifecycleStep::Complete(Default::default())}else{PluginLifecycleStep::Progress(Default::default())},terminal))
            }
            let step=if semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::owns_birth(&self.pending.reserved_preparation,&self.pending.reserved_birth,same){let before=same.retained_progress();semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::close_slot(&mut self.pending.reserved_preparation,&mut self.pending.reserved_birth,&*self.tools.tool_jobs,same)?;semio_framework_value::RetainedCloneStep::Progress(semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,same.retained_progress())?)}else if self.owns_original_reserved_mount(same){self.close_original_reserved_mount(same)?}else if self.owns_original_reserved_permit(same){if self.reserved_permit_waits(same){semio_framework_value::RetainedCloneStep::Progress(Default::default())}else{self.media.close_original_tool_lease(&mut self.tools,same,actual)?}}else if self.host_event_routing_ref().owns_original_host_event_source(same){self.host_event_routing_ctx().close_original_host_event_source(same)?}else if !self.host_event_routing_ref().owns_original_command_scope(same)&&self.ingress.owns_original_action_ingress(same){let before=same.retained_progress();self.ingress.close_original_action_ingress(same,actual)?;semio_framework_value::RetainedCloneStep::Progress(semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,same.retained_progress())?)}else if self.tools.original_tool_tag_capture.is_some()||self.tools.original_tool_tag_retirement.is_some()||self.tools.original_scrub_dispatch.is_some()||self.tools.original_press_settlement.is_some()||self.tools.original_typing_settlement.is_some()||self.tools.original_tool_follow.is_some(){self.render_ctx().tool_overlay_retirement_step(same,actual,true)?}else if self.original_typed_tag.is_some()||!self.original_typed_tag_close.is_empty(){self.close_original_typed_tag(same)?}else{let before=same.retained_progress();if self.original_typed_command.pending(){semio_plugin_instance_dispatch::dispatch_runtime_command::close_original_typed_source(&mut self.original_typed_command,&mut self.tools.typed_operation_reservations,&mut self.host_events.host_event_operations,same)?;}else if self.original_typing.pending(){self.original_typing.close_step(same)?;}else if self.original_emission.pending(){self.original_emission.close_step(same)?;}else if self.host_events.routes.cache_busy(){self.host_events.routes.settle_capture(same)?;}else{self.host_event_routing_ctx().close_action_host_events(same)?;}semio_framework_value::RetainedCloneStep::Progress(semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,same.retained_progress())?)};
            let terminal=self.maintenance_source_terminal_is_empty(same);Ok((if terminal{PluginLifecycleStep::Complete(step.progress())}else{PluginLifecycleStep::Progress(step.progress())},terminal))
        })
    }
    fn maintenance_source_terminal_is_empty(&self,cx:&StepContext<'_>)->bool{!semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::owns_birth(&self.pending.reserved_preparation,&self.pending.reserved_birth,cx)&&!self.owns_original_reserved_mount(cx)&&!self.owns_original_reserved_permit(cx)&&!self.host_event_routing_ref().owns_original_host_event_source(cx)&&!self.host_event_routing_ref().owns_original_command_scope(cx)&&!self.host_event_routing_ref().awaits_original_command_scope(cx)&&!self.ingress.owns_original_action_ingress(cx)}


    async fn bind_instance_id(&mut self, instance_id: u32) {
        self.tools.live_runtime_instance_id = Some(instance_id);
    }

    fn bind_original_opening(&mut self,instance_id:u32,actor:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.stores.store.local_actor_id().0.as_str()!=actor||self.stores.config_store.local_actor_id().0.as_str()!=actor||self.stores.draft_store.local_actor_id().0.as_str()!=actor||self.stores.interaction_store.local_actor_id().0.as_str()!=actor||self.stores.window_config_store.local_actor_id().0.as_str()!=actor{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original opening actor differs from its admitted Store and Window sources").into())}
        if self.tools.live_runtime_instance_id==Some(instance_id){return Ok(true)}if self.tools.live_runtime_instance_id.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original opening instance identity changed after binding").into())}
        let p=RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<u32>>(),..Default::default()};if !cx.is_cancelled()&&!cx.should_yield()&&cx.fuel_remaining()>=1&&p.fits(cx.retained_grant()){cx.consume_retained(p)?;cx.consume_fuel(1);self.tools.live_runtime_instance_id=Some(instance_id);}Ok(false)
    }

    async fn bind_actor(&mut self, actor: &str) {
        assert_eq!(self.stores.store.local_actor_id().0.as_str(), actor, "opening actor must own the document store from construction");
        assert_eq!(self.stores.config_store.local_actor_id().0.as_str(), actor, "opening actor must own the config store from construction");
        assert_eq!(self.stores.draft_store.local_actor_id().0.as_str(), actor, "opening actor must own the draft store from construction");
        assert_eq!(self.stores.interaction_store.local_actor_id().0.as_str(), actor, "opening actor must own the interaction store from construction");
        assert_eq!(self.stores.window_config_store.local_actor_id().0.as_str(), actor, "opening actor must own the window config registry from construction");
    }

    /// 🎟️ Advances one genuine original owner under the complete caller grant.
    fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if self.constructor.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"original constructor receipt awaits its phase recipient"})}
        if self.media.natural_source.is_some() && self.natural_source_can_close()
            && self.original_controller_close.as_ref().is_none_or(|owner| (owner.terminal)(&self.app))
            && self.constructor.constructor_terminal_is_empty()
        {
            return semio_plugin_instance_natural_publication::NaturalFilePublication::close_mounted(
                &mut self.media.natural_source, cx, loan,
            );
        }
        if self.original_controller_close.as_ref().is_none_or(|owner|(owner.terminal)(&self.app))&&self.constructor.constructor_terminal_is_empty()&&self.close_ladder_quoted_retirement_selected(){let step=self.close_ladder_step(cx,loan)?;return Ok(PluginLifecycleStep::retained(step,self.close_terminal_is_empty()))}
        let demand=self.close_retirement_demands(loan.maximum_copy_bytes)?;
        if self.original_controller_close.as_ref().is_none_or(|owner|(owner.terminal)(&self.app))&&!self.constructor.constructor_terminal_is_empty(){return semio_plugin_ownership::bounded_ownership::original_lifecycle_effect_step(cx,loan,demand,|_,effect|{self.stores.close_store_constructor_unit(&mut self.constructor, effect)?;let progress=self.constructor.constructor_receipt().map_or(Default::default(),|(_,_,_,progress)|*progress);self.constructor.take_constructor_receipt();Ok((PluginLifecycleStep::Progress(progress),false))})}
        semio_plugin_document_media::original_composed_lifecycle_step(cx,loan,demand,|same,grant|{
            if let Some(owner)=self.original_controller_close.as_ref().filter(|owner|!(owner.terminal)(&self.app)){let before=same.retained_progress();(owner.step)(&mut self.app,same)?;let p=semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,same.retained_progress())?;return Ok((PluginLifecycleStep::Progress(p),false))}
            let step=self.close_ladder_step(same,grant)?;let terminal=self.close_terminal_is_empty();Ok((PluginLifecycleStep::retained(step,terminal),terminal))
        })
    }

    fn close_terminal_is_empty(&self) -> bool { self.close_terminal_is_empty() }

    /// 🎡️ Visits the live-maintenance units in fair order and forwards the unchanged grant to the first that releases something.
    fn maintenance_step(&mut self, cx:&mut StepContext<'_>, loan:RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        self.maintenance_ladder_step(cx,loan)
    }

    fn maintenance_terminal_is_empty(&self) -> bool {
        self.ladder_ref().maintenance_ladder_terminal_is_empty()
    }

    fn maintenance_quiescent_under_live_preview(&self) -> bool {
        self.ladder_ref().maintenance_ladder_quiescent_under_live_preview()
    }

    fn maintenance_under_pressure(&self) -> bool {
        self.ladder_ref().maintenance_ladder_under_pressure()
    }

    async fn app_id(&self) -> &str {
        self.app.instance_id().await
    }

    async fn artifact_schema(&self) -> &str {
        A::DOCUMENT_SCHEMA
    }

    async fn tool_owner_witness(&self) -> ToolOwnerWitness {
        ToolOwnerWitness::of::<A>()
    }

    async fn tool_public_contracts(&self) -> Vec<ArtifactToolPublicContract> { self.tools.bounded_contracts().cloned().chain(self.tools.framework_registrations().chain(self.tools.app_registrations()).map(|(_,row)|ArtifactToolPublicContract {
        owner: row.owner, controller_id: row.key.controller_id.clone(), tool_id: row.key.tool_id.clone(), schema_id: row.schema_id.clone(),
        max_raw_wire_bytes: row.contract.max_raw_wire_bytes, publication_lanes: row.publication_lanes,
    })).collect() }

    async fn advance_typed_operation_publication(&mut self, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<(), semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if self.interaction.local_interaction_query_receipt.is_some(){return Ok(())}
        if self.constructor.has_constructor_receipt() { return Ok(()); }
        if self.constructor.store_constructor_is_pending() { ({ let __a0 = semio_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation,retained); self.stores.drive_store_constructor_unit(&mut self.constructor, cx, __a0) })?; return Ok(()); }
        if InstanceExtensions::holds_receipt(&self.time_travel, &self.tool_runs) || self.tools.tool_operations.entries_from(0).any(|(_, (_, owner))| owner.publication_ownership_progress.is_some() || owner.original_retirement_receipt.is_some()){return Ok(());}
        if !self.pending.reserved_commits.is_empty() && self.pending.reserved_commit_outcome.is_none() {
            self.command_ctx().step_framework_reserved_commit(cx).await?;
            return Ok(());
        }
        if !self.host_events.delivering&&(self.original_host_event_command.pending()||self.host_events.routes.runnable(self.shared_command_source_pending()||self.ingress.action_source_pending()))&&self.host_events.routes.take_turn(){
            if !self.host_event_routing_ctx().claim_original_host_event_scope(cx)?{return Ok(())}
            let mut source=std::mem::take(&mut self.original_host_event_command);
            let stepped=self.dispatch_ctx().background_host_event_step(&mut source,identity,cx).await;
            self.original_host_event_command=source;
            return stepped.map_err(Into::into)
        }
        let command_source_pending = self.shared_command_source_pending() || self.ingress.action_source_pending();
        if !self.host_events.resumes.runnable() {
            self.original_command_scope.release_idle(cx, command_source_pending);
        }
        if cx.is_cancelled() && self.host_events.resumes.runnable() && !self.host_events.resumes.in_flight() {
            self.host_events.resumes.plan(true, self.original_typed_command.pending(), self.host_events.delivering);
            return Ok(());
        }
        if self.host_events.resumes.runnable()&&(self.host_events.resumes.in_flight()||!command_source_pending)&&self.host_events.resumes.take_turn(){
            if !self.host_event_routing_ctx().claim_original_command_scope(cx)?{return Ok(())}
            let mut source = std::mem::take(&mut self.original_typed_command);
            let stepped = self.dispatch_ctx().task_resume_step(&mut source, identity, cx).await;
            self.original_typed_command = source;
            if stepped.is_err()||!self.host_events.resumes.in_flight()&&!self.original_typed_command.pending(){self.original_command_scope.release(cx);}
            return stepped.map_err(Into::into);
        }
        if InstanceExtensions::drive_turn(&mut self.extension_ctx(), &mut retained, identity, cx).await? {
            return Ok(());
        }
        if self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work()) {
            self.interaction.local_interaction_query_turn = !self.interaction.local_interaction_query_turn;
            if self.interaction.local_interaction_query_turn {
                self.interaction.advance_local_interaction_query_one(cx, store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained), semio_plugin_interaction::query::LOCAL_INTERACTION_QUERY_PAGE_BYTES)?;
                return Ok(());
            }
        }
        if self.ingest.has_pending_mounted_genesis_replacements() {
            self.maintenance.mounted_genesis_turn=!self.maintenance.mounted_genesis_turn;
            if self.maintenance.mounted_genesis_turn {
                let grant=semio_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.maintenance,cx.retained_grant());
                self.ladder_ctx().drive_store_replacement_jobs(cx,grant,false)?;
                return Ok(());
            }
        }
        self.typed_ctx().advance_typed_operation_publication_turn(identity,cx).await.map_err(Into::into)
    }

    fn take_typed_operation_preparation_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)> { self.tools.take_typed_operation_preparation_receipt() }

    fn take_typed_operation_retirement_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)> {
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{let Some((_,owner))=self.tools.tool_operations.entry_mut(index)else{continue;};if let Some((grant,progress))=owner.original_retirement_receipt.take(){return Some((owner.operation.operation,grant,progress));}}None
    }

    fn take_history_command_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> {
        self.time_travel.take_command_receipt()
    }

    fn take_tool_run_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> {
        self.tool_runs.take_original_receipt()
    }

    fn take_local_interaction_query_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.interaction.local_interaction_query_receipt.take()}

    fn store_constructor_is_pending(&self) -> bool { self.constructor.store_constructor_is_pending() }

    fn take_constructor_receipt(&mut self) -> Option<(ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)> { self.constructor.take_constructor_receipt() }

    fn tool_run_presence(&self) -> Option<protocol::PresenceToolRun> {
        self.tool_runs.presence()
    }

    fn tool_run_trace_delta(&self, cursor: Option<ToolRunTraceCursor>) -> Option<String> {
        self.tool_runs.trace_delta(cursor, TOOL_RUN_TRACE_DELTA_BYTES)
    }

    fn typed_operation_refusal(&self) -> Option<String> {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| {
            let (_, operation) = self.tools.tool_operations.entry(index)?;
            operation.retained_structural_refusal().map(|reason| format!("typed operation '{}' {reason}", operation.verb))
        })
    }

    fn document_load_diagnostic(&self, grant: RetainedCloneGrant) -> String { self.ladder_ref().document_load_diagnostic(grant) }

    fn close_diagnostic(&self) -> String {
        let ladder = self.ladder_ref();
        if matches!(ladder.close_rung(), semio_plugin_instance_ladders::ladders_close_ladder::CloseRung::OwnedStores) {
            let lane = semio_plugin_instance_ladders::ladders_lifecycle_stores::ORIGINAL_STORE_LANES
                .get(usize::from(self.close.close_owned_stage)).copied().unwrap_or("complete");
            let window = &self.stores.window_config_store;
            return format!("OwnedStores {lane} advance={} window registry={}/{} preview={} actor={}/{}", self.close.close_owned_advance_ready, window.owners.len(), window.birth.terminal_is_empty(), window.preview_active_epoch.is_some(), window.actor.is_some(), window.actor_retirement.is_some());
        }
        if matches!(ladder.close_rung(), semio_plugin_instance_ladders::ladders_close_ladder::CloseRung::StoreReplacement) {
            if let Some((operation, replacement)) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| self.ingest.store_replacement_jobs.entry(index)) {
                let worker = replacement.session.as_ref().map(|session| session.poll());
                return format!("StoreReplacement operation={operation} state={:?} worker={worker:?} target={:?} outcome={} resume={}", replacement.state, replacement.terminal_target, replacement.outcome_pending, replacement.resume_pending);
            }
        }
        if matches!(ladder.close_rung(), semio_plugin_instance_ladders::ladders_close_ladder::CloseRung::DocumentArchive) {
            if let Some((operation, load)) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| self.ingest.document_archive_loads.entry(index)) {
                let hydration = load.hydration.as_ref().map(|owner| (owner.phase, owner.remaining_owner(), owner.requires_original_close()));
                return format!("DocumentArchive operation={operation} state={:?} phase={:?} target={:?} hydration={hydration:?} history={} retained={}", load.state, load.phase, load.terminal_target, load.history.is_some(), load.retained.is_some());
            }
        }
        if matches!(ladder.close_rung(), semio_plugin_instance_ladders::ladders_close_ladder::CloseRung::TimeTravel) {
            #[cfg(feature = "artifact-app-testing")]
            return format!("TimeTravel residue={:?} {}", self.time_travel.residue(), self.time_travel.original_close_diagnostic());
            #[cfg(not(feature = "artifact-app-testing"))]
            return format!("TimeTravel residue={:?}", self.time_travel.residue());
        }
        ladder.close_rung_name()
    }

    fn typed_operation_diagnostic(&self) -> String {
        let operations = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).filter_map(|index| {
            let (id, operation) = self.tools.tool_operations.entry(index)?;
            Some(format!("op {id} '{}' {:?} page={}/{} s={} rej={}/{} cancel={} term={} sem={} out={} res={} fc={} prod={} ref={} wait={:?}", operation.verb, operation.stage, operation.result_page.is_some(), operation.result_page_presented, operation.session.is_some(), operation.session_rejected.is_some(), operation.session_rejected.as_ref().is_some_and(|rejected| rejected.admission_pending()), operation.user_cancel_requested, operation.terminal_seen, operation.worker_semantic_pending, operation.worker_outcome_pending, operation.worker_resume_pending, operation.worker_fault_capture.is_some(), operation.producer_request.is_some(), operation.retained_structural_refusal().unwrap_or_else(|| "none".into()), semio_plugin_retained_command::typed_operation::last_publication_wait(*id)))
        }).collect::<Vec<_>>().join("; ");
        let lw=(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|i|self.tools.latest_wins_commands.entry(i)).map(|(id,p)|format!(" lw{id}:ctx/lookup/accepted/closing/key={:?}",(p.context_ready,p.lookup_started,p.accepted,p.closing,p.key_copy.as_ref().map(|k|(k.part,k.prefix,k.offset))))).unwrap_or_default();
        let command = format!("command idle={} shared pending={} route cache={} route delivery={} action delivery={} original emission={} original typed command={}", self.original_command_scope.is_idle(), self.shared_command_source_pending(), self.host_events.routes.cache_busy(), self.host_events.routes.delivering, !self.host_events.action_delivery.terminal_is_empty(), self.original_emission.pending(), self.original_typed_command.pending());
        format!("{operations}{lw} | {command} | gates resumes={} delivering={} typing={} emission_host={} emission_input={} typed_input={} typed_receipt={} typed_meta={} typed_production={} action_source={} | lw={} ob={}/{}/{}/{} pr={} fa={} q={}", self.host_events.resumes.in_flight(), self.host_events.delivering, self.original_typing.pending(), self.original_emission.host_active(), self.original_emission.input.is_some(), self.original_typed_command.input_ready(), self.original_typed_command.receipt.is_some(), self.original_typed_command.metadata.is_some(), self.original_typed_command.production_pending(), self.ingress.action_source_pending(), self.tools.latest_wins_commands.len(), self.tools.typed_effect_outbox.len(), self.tools.typed_event_outbox.len(), self.tools.typed_ui_outbox.len(), self.tools.typed_completion_outbox.len(), self.tools.operation_progress_retired, self.child_composition.derivable_follow_awaits_retirement(&self.stores), self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work()))
    }

    fn has_pending_typed_operations(&self) -> bool {
        self.constructor.store_constructor_is_pending() || self.tools.operation_progress_retired
            || self.child_composition.derivable_follow_awaits_retirement(&self.stores)
            || InstanceExtensions::has_pending_work(&self.extension_ref())
            || !self.pending.reserved_commits.is_empty()
            || self.pending.reserved_commit_outcome.is_some()
            || self.host_events.resumes.pending()
            || self.host_events.routes.pending() || self.original_host_event_command.pending()
            || !self.tools.tool_operations.is_empty()
            || !self.tools.latest_wins_commands.is_empty()
            || self.tools.typed_effect_outbox.len() != 0
            || self.tools.typed_event_outbox.len() != 0
            || self.tools.typed_ui_outbox.len() != 0
            || self.tools.typed_completion_outbox.len() != 0
            || self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work())
    }

    fn live_typed_operation_slots(&self) -> usize {
        VcsArtifactApp::<A, M>::live_typed_operation_slots(self)
    }

    fn has_runnable_typed_operations(&self) -> bool {
        self.constructor.store_constructor_is_pending() || self.tools.operation_progress_retired
            || self.child_composition.derivable_follow_awaits_retirement(&self.stores)
            || InstanceExtensions::has_pending_work(&self.extension_ref())
            || !self.pending.reserved_commits.is_empty()
            || self.pending.reserved_commit_outcome.is_some()
            || self.host_events.resumes.pending()
            || self.host_events.routes.pending() || self.original_host_event_command.pending()
            || self.tools.has_runnable_tool_operation()
            || self.ingest.has_runnable_artifact_envelope_decode_worker_step()
            || !self.tools.latest_wins_commands.is_empty()
            || self.tools.typed_effect_outbox.len() != 0
            || self.tools.typed_event_outbox.len() != 0
            || self.tools.typed_ui_outbox.len() != 0
            || self.tools.typed_completion_outbox.len() != 0
            || self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work())
            || InstanceExtensions::holds_receipt(&self.time_travel, &self.tool_runs)
    }

    fn begin_local_interaction_query(&mut self, request_id: u64, query_generation: u64) -> Option<protocol::LocalInteractionQueryReply> { self.begin_local_interaction_query(request_id, query_generation) }

    fn acknowledge_local_interaction_query(&mut self, token: &protocol::LocalInteractionQueryToken) -> bool {
        self.interaction.local_interaction_query.as_mut().is_some_and(|query| query.acknowledge(token))
    }

    fn cancel_local_interaction_query(&mut self, token: &protocol::LocalInteractionQueryToken) -> bool {
        self.interaction.local_interaction_query.as_mut().is_some_and(|query| query.cancel_authorized(token))
    }

    fn take_local_interaction_query_reply(&mut self) -> Option<protocol::LocalInteractionQueryReply> {
        let query = self.interaction.local_interaction_query.as_mut()?;
        let reply = query.take_reply();
        if query.terminal_is_empty() {
            self.interaction.local_interaction_query = None;
        }
        reply
    }

    fn publish_local_interaction_query_reply(&mut self, frames: &mut Vec<Vec<u8>>, maximum_frames: usize) -> bool { self.publish_local_interaction_query_reply(frames, maximum_frames) }

    fn typed_operation_result_page_demands(&self, receiver: u32) -> Result<RetirementDemand, Fault> {
        let Some((_, operation_id)) = self.selected_typed_result_page(receiver) else { return Ok(Default::default()) };
        let mut demand = self.tools.tool_operations.get(operation_id).expect("selected result owner remains mounted").result_page_presentation_demands().map_err(plugin_retirement_fault)?;
        demand.copy_bytes = demand.copy_bytes.checked_add(std::mem::size_of::<usize>()).ok_or_else(|| plugin_sdk_fault("original result page cursor quote overflows"))?;
        Ok(demand)
    }

    fn take_typed_operation_result_page(&mut self, receiver: u32,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<(Option<TypedOperationResultPage> ,RetainedCloneProgress),Fault>{
        let grant=effect.grant();let selected = self.selected_typed_result_page(receiver);
        let Some((index,operation_id))=selected else{return Ok((None,Default::default()))};let demand=self.typed_operation_result_page_demands(receiver)?;
        if !plugin_turn_admitted(demand,grant)?{return Ok((None,Default::default()))}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-std::mem::size_of::<usize>(),..grant};let(page,mut progress)=self.tools.tool_operations.get_mut(operation_id).expect("selected result owner remains mounted").take_result_page(&mut effect.nested(child))?;
        if page.is_some(){self.tools.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        progress.copied_bytes+=std::mem::size_of::<usize>()
    ;}Ok((page,progress))
    }

    fn acknowledge_typed_operation_result(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault> { self.members_ctx().acknowledge_typed_operation_result(token) }

    fn take_typed_operation_effect(&mut self) -> Option<Effect> {
        if self.tools.typed_effect_outbox.items.front().is_some_and(pending_picker_effect) {
            return None;
        }
        self.tools.typed_effect_outbox.pop()
    }

    fn original_picker_effect(&mut self) -> Option<&mut Effect> {
        self.tools.typed_effect_outbox.items.front_mut().filter(|effect| pending_picker_effect(effect))
    }

    fn discard_original_picker_effect(&mut self) {
        if self.original_picker_effect().is_some() {
            drop(self.tools.typed_effect_outbox.pop());
        }
    }

    fn take_typed_operation_event(&mut self) -> Option<AppEvent> {
        self.tools.typed_event_outbox.pop()
    }

    fn take_typed_operation_ui_scope(&mut self) -> Option<UiDirtyScope> {
        self.tools.typed_ui_outbox.pop().or_else(|| self.tools.take_operation_progress_scope())
    }

    fn take_typed_operation_ui_progress(&mut self) -> Option<TypedOperationUiProgress> {
        let ui_scope = match self.tools.typed_ui_outbox.pop().or_else(|| self.tools.take_operation_progress_scope()) {
            Some(scope) => scope,
            None if self.time_travel.has_patch() => UiDirtyScope::None,
            None => return None,
        };
        Some(TypedOperationUiProgress { ui_scope, leftover: self.tools.typed_inline_interaction_leftover.take(), history_patch: self.time_travel.take_patch() })
    }

    async fn take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault> { self.members_ctx().take_typed_operation_completion().await }

    fn take_typed_operation_composed_result(&mut self) -> Option<ComposedGestureResult> {
        self.tools.typed_composed_outbox.pop()
    }

    async fn begin_dispatch_report(&mut self) {
        self.pending.reset_dispatch_report(&mut self.stores).await;
    }

    async fn dispatch_report(&self) -> &protocol::DispatchReport {
        &self.pending.dispatch_report
    }

    async fn set_merge_policy(&mut self, policy: protocol::MergePolicy) {
        self.pending.apply_merge_policy(&mut self.stores, policy).await;
    }

    async fn resolve_conflict(&mut self, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, Fault> {
        self.pending.resolve_store_conflict(&mut self.stores, &mut self.history, conflict_id, resolution).await
    }

    async fn open_conflicts(&self) -> Vec<protocol::Conflict> {
        self.stores.current_open_conflicts().await
    }

    async fn handle_action(&mut self, action: &str, args: &mut Option<DslValue>, meta: &mut Option<ActionMeta>, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<Option<InvocationResult>, OriginalTurnFailure> {
        let outcome:Result<Option<InvocationResult>,OriginalTurnFailure>=async{
        if !self.host_event_routing_ctx().capture_original_host_event_route(meta.as_ref(),cx)?{return Ok(None)}
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_action(action, args, &mut None, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        self.child_composition.follow_derivable_children(&mut self.stores, identity).await?;
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, action, result).await))
        }.await;
        self.settle_command_lease(&outcome,cx);
        outcome
    }

    fn admit_reserved_spawned_job(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
        self.pending.admit_framework_reserved_commit(&mut self.history, job, output)
    }

    fn take_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
        self.pending.take_framework_reserved_commit_outcome()
    }

    fn reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.pending.framework_reserved_commit_progress()
    }

    async fn handle_action_invocation(&mut self, address: &semio_framework::manifest::ActionAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &mut Option<ActionMeta>, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<Option<InvocationResult>,OriginalTurnFailure> {
        let outcome:Result<Option<InvocationResult>,OriginalTurnFailure>=async{
        if !self.host_event_routing_ctx().capture_original_host_event_route(meta.as_ref(),cx)?{return Ok(None)}  self.members_ctx().handle_action_invocation(address, source_args, original_fault, active_mode_id, meta, identity, cx).await
        }.await;
        self.settle_command_lease(&outcome,cx);
        outcome
    }

    async fn handle_response_action(&mut self, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta:&mut Option<ActionMeta>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut StepContext<'_>) -> Result<Option<InvocationResult>, OriginalTurnFailure> {
        let outcome:Result<Option<InvocationResult>,OriginalTurnFailure>=async{
        if !self.host_event_routing_ctx().capture_original_host_event_route(meta.as_ref(),cx)?{return Ok(None)}
        if self.original_typing.pending(){match self.command_ctx().capture_original_direct_tool_tag(action,args.as_ref(),meta,identity,cx).await?{None=>return Ok(None),Some(true)=>{self.command_ctx().original_emission.admit_empty(action,meta,cx)?;return Ok(None)},Some(false)=>{}}}
        if self.original_emission.input.is_some()||self.tools.original_emission_admission.is_some()||self.tools.original_emission_receipt.is_some(){
            if self.original_typed_command.pending()&&self.original_typed_command.tool_tag_dispatch==Some(true){cx.with_original_retirement(|cx|self.original_typed_command.close_step(cx))?;return Ok(None)}
            return self.command_ctx().drive_original_emission(cx).await
        }
        if self.original_typed_command.tool_tag_dispatch==Some(true){self.command_ctx().original_emission.admit_empty(action,meta,cx)?;return Ok(None)}
        if self.original_typed_command.input_ready()||self.original_typed_command.receipt.is_some()||self.original_typed_command.metadata.is_some(){return self.command_ctx().drive_original_direct_typed_command(meta,identity,cx).await}
        let definition = self.tools.registry.app_command(action).await.ok_or_else(|| plugin_sdk_fault(format!("continuation command '{action}' is not owned by the receiving app")))?;
        validate_ui_dispatch_classification("receiving app", action, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        match self.command_ctx().capture_original_direct_tool_tag(action,args.as_ref(),meta,identity,cx).await?{None=>return Ok(None),Some(true)=>{self.command_ctx().original_emission.admit_empty(action,meta,cx)?;return Ok(None)},Some(false)=>{}}

        if !self.command_ctx().prepare_original_typed_action(action,args,original_fault,cx).await?{return Ok(None)}
        self.media.require_complete_tool_operation_pipeline(self.original_typed_command.admission.as_ref().unwrap())?;
        let log_generation_before = self.history.log_generation;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result=self.command_ctx().drive_original_direct_typed_command(meta,identity,cx).await;
        self.host_events.invocation_kind = previous_kind;
        let Some(result)=result?else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, action, result).await))
        }.await;
        self.settle_command_lease(&outcome,cx);
        outcome
    }

    async fn handle_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &mut Option<ActionMeta>, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<Option<InvocationResult>,OriginalTurnFailure> {
        let outcome:Result<Option<InvocationResult>,OriginalTurnFailure>=async{
        if !self.host_event_routing_ctx().capture_original_host_event_route(meta.as_ref(),cx)?{return Ok(None)}
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_command(address, source_args, original_fault, active_mode_id, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, &address.command_id, result).await))
        }.await;
        self.settle_command_lease(&outcome,cx);
        outcome
    }

    fn original_command_frame_pending(&self)->bool{self.tools.agent_lane_preview.is_some()}

    async fn handle_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<Option<InvocationResult>,OriginalTurnFailure> {
        let outcome:Result<Option<InvocationResult>,OriginalTurnFailure>=async{
        if !self.host_event_routing_ctx().capture_original_host_event_route(Some(meta),cx)?{return Ok(None)}
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_command_frame(command_bytes, meta, identity, cx).await?;
        match result{Some(result)=>Ok(Some(self.render_ctx().finish_recorded(log_generation_before,"typed-command",result).await)),None=>Ok(None)}
        }.await;
        self.settle_command_lease(&outcome,cx);
        outcome
    }

    /// 🎯️ The retained intent source donates its original converted arguments and selected metadata to the same declared native command ingress.
    async fn handle_intent_frame(&mut self,intent:&UiIntent,args:&mut Option<DslValue>,original_fault:&mut Option<Fault>,meta:&mut Option<ActionMeta>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut StepContext<'_>)->Result<Option<InvocationResult>,OriginalTurnFailure>{
        let outcome:Result<Option<InvocationResult>,OriginalTurnFailure>=async{
        if !self.host_event_routing_ctx().capture_original_host_event_route(meta.as_ref(),cx)?{return Ok(None)}
        if intent.action.version!=1{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"intent action version must match the declared native command producer").into())}
        if self.host_events.resumes.in_flight(){return Ok(None)}
        let mut context=self.command_ctx();
        if context.original_typing.pending(){match context.capture_original_direct_tool_tag(intent.action.name.as_str(),args.as_ref(),meta,identity,cx).await?{None=>return Ok(None),Some(true)=>{context.original_emission.admit_empty(intent.action.name.as_str(),meta,cx)?;return Ok(None)},Some(false)=>{}}}
        if context.original_emission.input.is_some()||context.tools.original_emission_admission.is_some()||context.tools.original_emission_receipt.is_some(){
            if context.original_typed_command.pending()&&context.original_typed_command.tool_tag_dispatch==Some(true){cx.with_original_retirement(|cx|context.original_typed_command.close_step(cx))?;return Ok(None)}
            return context.drive_original_emission(cx).await
        }
        if context.original_typed_command.tool_tag_dispatch==Some(true){context.original_emission.admit_empty(intent.action.name.as_str(),meta,cx)?;return Ok(None)}
        if context.original_typed_command.input_ready()||context.original_typed_command.receipt.is_some()||context.original_typed_command.metadata.is_some(){return context.drive_original_direct_typed_command(meta,identity,cx).await}
        {
            match context.capture_original_direct_tool_tag(intent.action.name.as_str(),args.as_ref(),meta,identity,cx).await?{None=>return Ok(None),Some(true)=>{context.original_emission.admit_empty(intent.action.name.as_str(),meta,cx)?;return Ok(None)},Some(false)=>{}}

        }
        if !context.prepare_original_typed_action(intent.action.name.as_str(),args,original_fault,cx).await?{return Ok(None)}
        context.drive_original_direct_typed_command(meta,identity,cx).await
        }.await;
        self.settle_command_lease(&outcome,cx);
        outcome
    }

    /// 🧵️ Validates and queues the task's binary command through the original typed source.
    fn queue_task_resume_command(&mut self, command_bytes: Vec<u8>, meta: &ActionMeta) -> Result<(), Fault> {
        <A::Command as ::protocol::OpBinary>::decode_op(&command_bytes).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.task-resume-command"), format!("an AsyncTask resolved TaskResolution::Command with bytes that are not this app's typed Command: {error}")))?;
        self.host_events.resumes.push(command_bytes, meta.clone())
    }

    fn take_task_resume_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> { self.host_events.resumes.take_outcome() }

    async fn resume_task_emit(&mut self, artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<InvocationResult, Fault> { self.members_ctx().resume_task_emit(artifact_ops, config_ops, draft_ops, meta, identity, cx).await }

    async fn take_last_emit_wire(&mut self) -> Option<EmitWire> {
        self.pending.last_emit_wire.take()
    }

    async fn take_pending_presence(&mut self) -> Vec<PresenceUpdate> {
        std::mem::take(&mut self.roster.pending_presence)
    }

    async fn take_pending_transaction_proposal(&mut self) -> Option<TransactionProposalDraft> {
        self.txn.pending_transaction_proposal.take()
    }

    async fn transaction_prepare(&mut self, txn_id: &str, _mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome { self.members_ctx().transaction_prepare(txn_id, _mutation_id, payload, prepared_ops, prepared_child_ops, origin).await }

    async fn transaction_commit(&mut self, txn_id: &str, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, Fault> { self.members_ctx().transaction_commit(txn_id, meta, identity).await }

    async fn transaction_rollback(&mut self, txn_id: &str) -> Result<(), Fault> {
        match &self.txn.pending_transaction {
            Some(pending) if pending.txn_id == txn_id => {
                self.txn.pending_transaction = None;
                Ok(())
            }
            _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.rollback-unknown"), format!("transaction_rollback: no pending transaction matches txn_id {txn_id:?}"))),
        }
    }

    async fn transaction_undo(&mut self, group_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.child_composition.children.is_empty() {
            return self.dispatch_ctx().transaction_group_history("undo", group_id, identity).await;
        }
        if self.stores.store.tail_group_id().await.as_deref() != Some(group_id) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-foreign-tail"), format!("transaction_undo: this instance's tail edit does not belong to group {group_id:?}")));
        }
        self.stores.store.undo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-failed"), format!("{error:?}")))?;
        self.history.invalidate_cache();
        Ok(())
    }

    async fn transaction_redo(&mut self, group_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.child_composition.children.is_empty() {
            return self.dispatch_ctx().transaction_group_history("redo", group_id, identity).await;
        }
        match self.stores.store.redo_tail().await {
            Some((_, Some(tail_group))) if tail_group == group_id => {
                self.stores.store.redo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.redo-failed"), format!("{error:?}")))?;
                self.history.invalidate_cache();
                Ok(())
            }
            _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.redo-foreign-tail"), format!("transaction_redo: this instance's redo-tail edit does not belong to group {group_id:?}"))),
        }
    }

    async fn history_snapshot(&mut self) -> Result<HistoryPatch, Fault> {
        if self.time_travel.history_unavailable() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), "this head-only pure evaluation holds no history; use a live instance"));
        }
        self.render_ctx().history_patch(true).await
    }

    async fn hydrate_pure_head(&mut self,head:&[u8],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure> { self.members_ctx().hydrate_pure_head(head, cx).await }

    async fn ephemeral_snapshot(&self) -> EphemeralSnapshot { self.ephemeral_snapshot().await }

    fn reserve_presence_ingress(&mut self, seq: u64) -> Result<PresenceRosterAdmission, Fault> {
        let generation = self.roster.peer_roster_admission_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-generation"), "peer roster admission generation exhausted"))?;
        let slot = InstanceStores::<A>::peer_roster_slot(generation);
        if self.roster.peer_presence.is_none()
            || self.roster.peer_roster_reservations[slot].is_some()
            || !self.roster.peer_roster_publications.can_insert(generation)
            || !self.roster.peer_roster_outcomes.can_insert(generation)
            || !self.roster.peer_presence_retirements.can_insert(generation)
            || !self.roster.presence_peer_retirements.can_insert(generation)
            || !self.stores.presence_store.peer_retirement_available()
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-saturated"), "peer roster ingress, outcome, or retirement authority is saturated before payload decode"));
        }
        self.roster.peer_roster_reservations[slot] = Some((generation, seq));
        self.roster.peer_roster_admission_generation = generation;
        Ok(PresenceRosterAdmission { seq, generation, cancel: self.roster.peer_roster_scope().child_now() })
    }

    fn admit_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: protocol::PresenceCommandCursor, now_ms: i64) { self.roster.admit_presence_ingress(admission, command, now_ms) }

    fn push_presence_ingress(&mut self, generation: u64, page_index: u32, page: FixedCommandPage) -> Result<(), (Fault, FixedCommandPage)> {
        let Some(publication) = self.roster.peer_roster_publications.get_mut(generation) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-owner"), "Presence page targets no retained publication owner"), page));
        };
        publication.push_page(page_index, page)
    }

    fn reject_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: FixedCommandPage, fault: Fault) { self.roster.reject_presence_ingress(admission, command, fault) }

    fn take_presence_outcome(&mut self) -> Option<PresenceRosterOutcome> {
        let generation = self.roster.peer_roster_outcome_generation.checked_add(1)?;
        let outcome = self.roster.peer_roster_outcomes.remove(generation)?;
        self.roster.peer_roster_outcome_generation = generation;
        Some(outcome)
    }

    async fn config_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
        store::os_store::text_format::print_document_pack(self.stores.config_store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn window_config_packs(&self) -> Result<Vec<WindowConfigPack>, Fault> {
        self.stores.window_config_store.packs().await
    }

    fn advance_window_config_load(&mut self, load: &mut semio_plugin_window_config::WindowConfigCommandLoad, entry: &mut protocol::WindowConfigPackEntry, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        load.advance(&mut self.stores.window_config_store, entry, cx)
    }

    async fn load_child_pack(&mut self, slot: &str, child_id: &str, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.child_composition.open_child(&mut self.stores, slot, child_id, dialect, envelope_pack, identity).await?;
        self.history.invalidate_cache();
        Ok(())
    }

    /// 🔢️ Sorted by `(slot, child_id)`: the map's iteration order is not stable, and a
    /// persisted child list that reshuffles between reads would make every save look like a
    /// change to anything diffing it.
    async fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, Fault> { self.child_composition.child_packs().await }

    async fn child_head_packs(&self) -> Result<Vec<protocol::ChildHeadPackEntry>, Fault> { self.child_composition.child_head_packs().await }

    fn validate_folder_archive_target(&self, document: &str, schema: &str) -> bool {
        self.stores.store.envelope().id == document && A::DOCUMENT_SCHEMA == schema
    }

    fn begin_paged_document_archive_load(
        &mut self, operation: u64,
        archive: &mut Option<protocol::channel::archive_ingress::VerifiedDocumentArchiveParent>,
        cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        if !self.ingest.document_archive_loads.can_insert(operation) {
            return Err(plugin_sdk_fault("paged archive operation is already live or its registry slot is busy").into());
        }
        let Some(source) = archive.as_ref() else { return Err(plugin_sdk_fault("paged archive admission lost its verified original").into()); };
        let capacity = size_of::<protocol::DocumentArchivePack>();
        let charge = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>>() + capacity,
            retained_capacity_bytes: capacity, ..Default::default()
        };
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0 || !charge.fits(cx.retained_grant()) {
            return Ok(false);
        }
        let history = document_archive_history_decoder(source.spr_len()).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let load = ActiveDocumentArchiveLoad::new_paged(operation, archive.take().expect("checked verified archive pages"), history);
        self.ingest.document_archive_loads.insert_admitted(operation, load);
        cx.consume_retained(charge)?;
        cx.consume_fuel(1);
        Ok(true)
    }

    fn begin_document_archive_load(&mut self,operation:u64,archive:&mut protocol::DocumentArchivePack,admission:&mut semio_plugin_document_media::OriginalDocumentArchiveAdmission,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.members_ctx().begin_document_archive_load(operation,archive,admission,false,cx)}
    fn begin_document_archive_merge(&mut self,operation:u64,archive:&mut protocol::DocumentArchivePack,admission:&mut semio_plugin_document_media::OriginalDocumentArchiveAdmission,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.members_ctx().begin_document_archive_load(operation,archive,admission,true,cx)}

    fn document_archive_maintenance_is_runnable(&self, stock: RetainedCloneGrant) -> Result<bool, ValueError> {
        self.ladder_ref().document_archive_maintenance_is_runnable(stock)
    }

    async fn poll_document_archive_load(&mut self, operation: u64, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
        if self.ingest.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::awaits_merge) {
            self.command_ctx().merge_document_archive(operation, identity, cx).await?;
        }
        self.ingest.document_archive_status(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive authority changed before status publication"))
    }

    fn cancel_document_archive_load(&mut self, operation: u64) -> Result<(), Fault> { self.ingest.cancel_document_archive_load(operation) }

    fn acknowledge_document_archive_load(&mut self, operation: u64) -> Result<(), Fault> { self.ingest.acknowledge_document_archive_load(operation) }

    fn document_load_archive(&self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<protocol::DocumentArchivePack, Fault> {
        let Effect::LoadDocument { pack, spr } = self.stores.stamp_load_document_identity(Effect::LoadDocument { pack: files.pack.clone(), spr: files.spr.clone() })? else {
            return Err(plugin_sdk_fault("the identity stamp answered something other than a document load"));
        };
        Ok(protocol::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() })
    }

    fn document_identity(&self) -> Option<String> {
        let id = &self.stores.store.envelope().id;
        (1..=512).contains(&id.chars().count()).then(|| id.clone())
    }

    fn document_base_revision(&self) -> Result<u64, Fault> {
        Ok(u64::from_be_bytes(self.stores.store.content_revision_now()[..8].try_into().expect("revision lane width")))
    }

    async fn document_archive(&self) -> Result<protocol::DocumentArchivePack, Fault> { semio_plugin_instance_members::document_archive(&self.stores, &self.child_composition).await }

    async fn load_config_pack(&mut self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<(), Fault> {
        let parsed: store::os_store::text_format::ParsedDocumentText<A::Config, A::ConfigMutation> = store::os_store::text_format::parse_document_pack(&files.pack, &files.spr, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| error.into_fault())?;
        self.stores.config_store.reset(parsed.into_envelope()).await.map_err(|error| error.into_fault())?;
        self.history.invalidate_cache();
        Ok(())
    }

    async fn dispatch_config_command(&mut self, command_bytes: &mut Vec<u8>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<Option<InvocationResult>, OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let Some(result)=self.dispatch_ctx().dispatch_config_command_inner(command_bytes,meta,identity,cx).await? else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before,"configCommand",result).await))
    }

    async fn ingest_operations(&mut self, mutations: &[u8], identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let envelopes = protocol::decode_envelopes(mutations).map_err(|error| error.into_fault())?;
        let mut reports = Vec::with_capacity(envelopes.len());
        let generation = self.stores.store.generation();
        for envelope in envelopes {
            reports.push(self.stores.store.ingest_remote(envelope).await.map_err(|error| error.into_fault())?);
        }
        self.history.invalidate_cache();
        if self.stores.store.generation() != generation {
            self.dispatch_ctx().queue_base_moved();
        }
        Ok(reports)
    }

    async fn ingest_operations_text(&mut self, operations_text: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut mutations: Vec<A::Mutation> = Vec::new();
        for line in operations_text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            mutations.push(A::Mutation::parse_op(line).map_err(|error| error.into_fault())?);
        }
        if mutations.is_empty() {
            return Ok(());
        }
        self.stores.store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        self.history.invalidate_cache();
        Ok(())
    }

    async fn document_text(&self) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
        let print = <A::Snapshot as store::os_store::serialization::ArtifactStoreSnapshot>::reference_snapshot_codec().and_then(|codec| codec.print_dsl).ok_or_else(|| plugin_sdk_fault("the snapshot owner declares no reference DSL printer"))?;
        store::os_store::text_format::print_document_text_with(self.stores.store.envelope(), print).await.map_err(|error| error.into_fault())
    }

    async fn document_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
        store::os_store::text_format::print_document_pack(self.stores.store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn attach_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.stores.store.attach_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.child_composition.announce_member_event_logs(&mut self.stores).await?;
        self.history.invalidate_cache();
        Ok(())
    }

    async fn attach_hot_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.stores.store.attach_hot_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.history.invalidate_cache();
        Ok(())
    }

    async fn tick_backbone(&mut self, identity: &mut EntityIdentityAuthority<'_>, cx:&mut StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let generation = self.stores.store.generation();
        let reports = self.stores.store.tick_backbone_reports().await.map_err(|error| error.into_fault())?;
        if self.stores.store.generation() != generation {
            self.dispatch_ctx().queue_base_moved();
        }
        self.child_composition.follow_derivable_children(&mut self.stores, identity).await?;
        let folded_member_lanes = self.child_composition.fold_member_inbound(&mut self.stores, &mut self.history).await?;
        if !reports.is_empty() || folded_member_lanes != 0 {
            self.history.invalidate_cache();
        }
        Ok(reports)
    }

    async fn advance_original_atomic_backbone(
        &mut self, inbox: &mut store::os_store::backbone::atomic_transport::AtomicBackboneInbox,
        original: &mut Option<Vec<u8>>, binding: [u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<Option<protocol::MergeReport>, OriginalTurnFailure> {
        let before = self.stores.store.generation();
        let report = semio_plugin_reactor_app::atomic_document::fold_atomic_inbox(&mut self.stores.store, inbox, original, binding, cx).await?;
        if self.stores.store.generation() != before {
            self.dispatch_ctx().queue_base_moved();
            self.history.invalidate_cache();
        }
        Ok(report)
    }

    fn original_atomic_outbound_pending(&self) -> bool {
        self.stores.store.original_document_outbound_pending()
    }

    fn advance_original_atomic_outbound(
        &mut self, delivery: &mut Option<store::os_store::backbone::atomic_transport::AtomicBackboneDelivery>,
        binding: [u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        semio_plugin_reactor_app::atomic_document::drive_document_outbox(cx, |effect| self.stores.store.advance_bound_document_outbound(binding, delivery, effect))
    }

    async fn detach_backbone(&mut self) -> Result<(), Fault> {
        self.stores.store.detach_backbone().map_err(|error| error.into_fault())?;
        self.history.invalidate_cache();
        Ok(())
    }

    fn render_demand(&self,body_key:&str,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError>{if self.pending.pending_render_failure.is_some()||self.pending.pending_render_failure_place.is_some()||self.pending.pending_render_compound_place.is_some()||self.pending.pending_render_tree.is_some(){return self.pending.render_failure_demands(maximum_body_bytes);}if A::uses_render_work(body_key){if let Some(work)=self.pending.pending_render_work.as_ref(){return work.query_demand(|work|A::render_work_demand(work,maximum_body_bytes));}}Ok(Default::default())}
    async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut StepContext<'_>) -> Result<Option<()>,OriginalTurnFailure> { self.members_ctx().render(body_key, snapshot_override_json, view_state, cx).await }

    fn original_render_tree_source<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<&'a mut Option<ComponentTree>,OriginalTurnFailure> { self.pending.original_render_tree_source(cx) }
    fn staged_action_matches(&self,action:&str,body:&str,view:&ViewModel)->bool{view.active_window_kind_id.as_deref().is_some_and(|kind|self.tools.registry.staged_action_matches(kind,action,body))}
    fn render_work_request_owned(&mut self,cx:&StepContext<'_>)->bool{self.render_ctx().render_request_owned(cx)}
    fn receive_render_work_request(&mut self,body_key:&str,request:&mut Option<semio_framework::action_staging::StagedFormRequest>,view:&ViewModel,cx:&mut StepContext<'_>)->Result<Option<()>,OriginalTurnFailure>{self.members_ctx().receive_render_request(body_key,request,view,cx)}
    fn close_render_work_request(&mut self,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{let grant=cx.retained_grant();cx.with_original_retirement(|cx|{self.pending.close_render_failure(cx,grant).map_err(OriginalTurnFailure::from)?;Ok(self.pending.render_failure_terminal_is_empty().then_some(true))})}

    async fn window_engagements(&mut self, view_state: &ViewModel) -> HashMap<String, WindowEngagement> { self.render_ctx().window_engagements(view_state).await }

    async fn window_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> { self.render_ctx().window_measures(view_state).await }

    async fn tool_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> { self.render_ctx().tool_measures(view_state).await }

    async fn app_catalogue_json(&mut self) -> String {
        A::app_catalogue_json().await
    }

    fn pending_effects_waits_for_document(&self) -> bool { self.ingest.document_blocks_pending_effects() }

    fn input_refusal(&self, verb: &str) -> Option<Fault> { self.ingest.document_loading_refusal(&*self.tools, verb, InstanceExtensions::admitted_while_loading(verb)) }

    async fn pending_effects(&mut self, view: Option<&ViewModel>, cx: &mut StepContext<'_>) -> Result<Option<Vec<Effect>>, OriginalTurnFailure> { self.render_ctx().pending_effects(view, cx).await }

    async fn context_menu(&mut self, request: &ContextMenuRequest, view_state: &ViewModel) -> Vec<ContextMenuItemSpec> { self.render_ctx().context_menu(request, view_state).await }

    async fn produce_media(&mut self, port: &str, owner: &mut semio_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut StepContext<'_>) -> Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure> { self.members_ctx().produce_media(port, owner, cx).await }

    fn original_media_live_document(&self) -> Option<RawLiveDocument<'_>> {
        Some((&self.stores.store.envelope().id, self.stores.store.content_revision()))
    }

    fn natural_source_capability(&self) -> Option<(semio_plugin_decl_media::original_media::natural_source_gate::NaturalSourceDeclaration<'static>, u32)> {
        semio_plugin_instance_natural_publication::natural_source_capability::<A>()
    }

    fn natural_source_format_closed(&self) -> bool { self.media.natural_source.is_none() }

    async fn advance_natural_source_format(
        &mut self, source: &mut semio_plugin_decl_media::original_media::natural_source::NaturalFileSourceGuest, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        semio_plugin_instance_natural_publication::advance_natural_publication(self, source, cx).await
    }

    fn original_media_format_closed(&self) -> bool {
        self.media.original_picker.is_none()
    }

    fn observe_original_media_commit(&mut self, picker: &mut RawMediaPicker, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        self.observe_picker_commit(picker, cx)
    }

    async fn advance_original_media_format(
        &mut self, picker: &mut RawMediaPicker, _identity: &mut EntityIdentityAuthority<'_>, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        self.advance_picker_publication(picker, cx).await
    }

    fn accepts_original_structured_media(&self,port:&str,schema:&str,media_type:&semio_framework::MediaType)->bool{A::accepts_original_structured_media(port,schema,media_type)}
    fn original_natural_file_media_type(&self,port:&str,format_kind:&str)->Option<semio_framework::MediaType>{
        if port!=semio_framework::NATURAL_FILE_PORT_ID{return None}
        let codec=::semio_framework_async::poll::resolve_ready(A::natural_file_codec())?;
        if codec.format_kind!=format_kind{return None}
        Some(::semio_framework_async::poll::resolve_ready(A::io()).map_or(semio_framework::MediaType{class:semio_framework::MediaClass::Data,form:semio_framework::MediaForm::Value},|io|io.artifact_media_type))
    }
    async fn prepare_media_import(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure> { self.prepare_media_import(cx).await }
    async fn advance_original_media_import(&mut self, port: &str, media: &mut Option<Media>, meta: &ActionMeta, progress: &mut semio_plugin_reactor_app::OriginalMediaImport, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> { self.advance_original_media_import(port, media, meta, progress, cx).await }

    async fn consume_media(&mut self,port:&str,artifact:&mut Option<MediaArtifact>,cx:&mut StepContext<'_>)->Result<Option<MediaConsumption>,semio_plugin_decl_app::MediaArtifactOperationFailure>{self.members_ctx().consume_media(port,artifact,cx).await}

    /// 🎞️ Batch-only adapter. Registered interactive outputs are driven through the same
    /// externally addressable submission API; this loop is not used by UI/workflow entry points.
    async fn export_media(&mut self,port:&str,cx:&mut StepContext<'_>)->Result<Option<Media>,OriginalTurnFailure>{self.members_ctx().export_media(port,cx).await}

    async fn submit_media_export(&mut self,port:&str,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure> { self.submit_media_export(port, receiver, cx).await }

    async fn submit_media_export_checked(&mut self,port:&str,expected_parent_document_id:&str,expected_base_revision:u64,checkpoint_id:Option<&str>,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure> { self.submit_media_export_checked(port, expected_parent_document_id, expected_base_revision, checkpoint_id, receiver, cx).await }

    async fn poll_media_export(&mut self,handle:semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<ArtifactMediaExportPoll>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure> { self.poll_media_export(handle, receiver, cx).await }

    async fn cancel_media_export(&mut self, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        self.media.cancel_owned_media_export(handle).await
    }

    async fn retain_media_export_result(&mut self,handle:semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,source:&mut Option<ArtifactMediaExportPoll>,status:&mut Option<semio_plugin_decl_media::ArtifactMediaExportPublishedStatus>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure> { self.media.retain_media_export_result(handle, source, status, cx).await }

    async fn take_media_export_chunk(&mut self,handle:semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure> { self.media.take_media_export_chunk(handle, receiver, terminal, cx).await }

    async fn take_segmented_download_chunk(&mut self, operation_id: u64) -> Result<Option<Vec<u8>>, Fault> { self.media.take_segmented_download_chunk(operation_id).await }

    async fn import_media(&mut self,port:&str,media:&mut Option<Media>,meta:&ActionMeta,cx:&mut StepContext<'_>)->Result<Option<InvocationResult>,OriginalTurnFailure> { self.import_media(port, media, meta, cx).await }

    async fn media_fingerprint(&mut self, port: &str) -> Result<MediaFingerprint, MediaError> { self.media_fingerprint(port).await }
}
