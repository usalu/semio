//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.

use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep};
use semio_framework_plugin_decl_views::media_contract::{ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES, ArtifactDownloadOutput, ArtifactMediaExportHandle, ArtifactMediaExportPoll};
use semio_framework::{ActionKind, AppRole, Fault, FaultCode, FaultFrom, FaultOrigin, Media, MediaClass, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaType, MediaWireFormat, ViewModel};
use semio_framework_plugin_decl_emission::{ActionMeta, ChildEmit, Emit};
use semio_framework_plugin_document_media::{ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadState, MediaArtifact, copy_media_text_parts, document_archive_history_decoder, whole_document_media};
use semio_framework::kernel::{AppEvent, Effect, FRAMEWORK_RESERVED_JOB_KIND, FixedCommandPage, HistoryPatch, InvocationResult, UiDirtyScope};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_BUSY_CODE, DOCUMENT_LOAD_CHANGED_CODE, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_HISTORY_INVALID_CODE, DOCUMENT_LOAD_INCOMPLETE_CODE, DOCUMENT_LOAD_MEMBER_INVALID_CODE, DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, DOCUMENT_LOAD_TOO_LARGE_CODE, DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, EphemeralSnapshot, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption, NATURAL_FILE_PORT, TransactionPrepareOutcome, document_load_fault, media_projection_error, natural_file_input_media, whole_document_archive};
use store::os_store::store_schema::{ArtifactCommand, ArtifactEnvelope};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use semio_framework_plugin_decl_app::tool_contract::{ArtifactToolCompletionValue, ArtifactToolPublicContract, ToolOwnerWitness, TypedOperationResultLane};
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, InteractionView};
use std::collections::{BTreeMap, HashMap};
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_plugin_composition_publication::{ComposedGestureResult, MountedTypedCommandFullOperationStage};
use semio_framework_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, FRAMEWORK_HISTORY_BODY_KEY, WindowEngagement, WindowMeasure};
use semio_framework_value::{DslValue, RetirementDemand, ValueError};
use semio_framework_plugin_job::{EmitWire, PendingTransaction, TransactionProposalDraft, TypedOperationCompletion, TypedOperationResultPage, TypedOperationResultToken, TypedOperationUiProgress};
use semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitProgress;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use crate::{InstanceExtensions, VcsArtifactApp};
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework::manifest::CommandAddress as ManifestCommandAddress;
use std::mem::ManuallyDrop;
use semio_framework_plugin_instance_docs::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_mounted_owner::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
use protocol::OpText;
use semio_framework_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
use semio_framework_plugin_peer::{PeerRosterPublication, PresenceRosterAdmission, PresenceRosterOutcome};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_framework_plugin_window_transient::window_transient::TransientView;
use semio_framework_plugin_window_config::WindowConfigPack;
use semio_framework_plugin_window_transient::built_to_component_tree;
use store::os_store::document_materialize::create_document_envelope;
use semio_framework_plugin_instance_dispatch::dispatch_emit;
use semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job;
#[cfg(test)]
use semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job_counted;
use semio_framework_plugin_window_kits::{merge_ui_values, validate_ui_dispatch_classification};
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_instance_render as render;
use semio_framework_plugin_time_travel_panel::command_history_panel_lift::ui_history_panel;
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::TOOL_RUN_TRACE_DELTA_BYTES;

use semio_framework_plugin_instance_commands::commands_instance_root::framework_reserved_action_kind;
use semio_framework_plugin_instance_jobs::jobs_instance_root::OriginalRenderTreeIdentity;

#[path = "🔒️close-terminal/🦀️.rs"]
mod close_terminal;

#[path = "🧾️typed-operation-results/🦀️.rs"]
mod typed_operation_results;

#[path = "🕹️ephemeral-interaction/🦀️.rs"]
mod ephemeral_interaction;

#[path = "📨️action-dispatch/🦀️.rs"]
mod action_dispatch;

#[path = "🔀️transactions/🦀️.rs"]
mod transactions;

#[path = "🗃️document-load/🦀️.rs"]
mod document_load;

#[path = "🖼️render-surface/🦀️.rs"]
mod render_surface;

#[path = "🎞️media-transfer/🦀️.rs"]
mod media_transfer;

/// 🧪 Guest laws in dependent crates (puzzle 3d) settle Isolated reserved admits the host would.
pub async fn settle_framework_reserved_admission<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult) -> Result<InvocationResult,semio_framework_job::OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok(admitted);
    };
    let output=drive_framework_reserved_registry_job(job,input,app.mounted_policy.preparation)?;
    settle_framework_reserved_commit(app,job,Ok(output)).await.map_err(Into::into)
}

/// 🪜️ Most units one reserved commit may take in a settle law — one per composed child or undone
/// edit, well above anything a fixture composes.
pub const FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS: usize = 4_096;

/// 🪜️ Hands a finished reserved spawn-job to `app`'s commit queue and drives the commit one unit
/// per call — the same units the reactor's typed-operation continuation runs across turns — to its
/// outcome.
pub async fn settle_framework_reserved_commit<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, job: u64, output: Result<Vec<u8>, Fault>) -> Result<InvocationResult, Fault> {
    if !app.pending.admit_framework_reserved_commit(&mut app.history, job, output)? {
        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-missing"), "framework reserved spawn-job had no pending admit"));
    }
    for _ in 0..FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS {
        app.step_framework_reserved_commit().await?;
        if let Some(outcome) = app.pending.take_framework_reserved_commit_outcome() {
            return outcome;
        }
    }
    Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stall"), format!("framework reserved commit did not finish within {FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS} units")))
}

/// 🧪 Browser `driveSpawnedJob` contract: the isolated reserved dummy job is admitted from its original request and driven
/// by funded `step_job` turns to its terminal output, which is then settled through the reserved commit; the turn count is returned.
#[cfg(any(test, feature = "artifact-app-testing"))]
pub async fn drive_framework_reserved_spawn_like_host<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult) -> Result<(InvocationResult,u32),semio_framework_job::OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok((admitted, 0));
    };
    let (output, steps) = drive_framework_reserved_registry_job_counted(job, input, app.mounted_policy.preparation)?;
    let settled = settle_framework_reserved_commit(app, job, Ok(output)).await?;
    Ok((settled, steps))
}

/// 🪪️ A document loaded from its text or pack belongs to the app that loads it: a pack that names no dialect is stamped
/// with the app's own, exactly as every admission path (`try_adopt_completed`, the constructor) stamps it — without it the
/// first child restore faulted "child restore requires the parent's exact declared dialect".
fn loaded_with_app_dialect<A: ArtifactApp>(mut envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>) -> ArtifactEnvelope<A::Snapshot, A::Mutation> {
    if envelope.dialect.is_none() {
        envelope.dialect = Some(A::DIALECT.into());
    }
    envelope
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M> {
    fn admit_cancellation_issuer(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let parent=cx.original_source_parent()?;self.tools.tool_cancellations.admit_step(parent,cx)}

    fn mounted_owner_maintenance_phase(&self) -> Result<MountedOwnerPhaseV1, Fault> {
        let instance_id = self.tools.live_runtime_instance_id.ok_or_else(|| plugin_sdk_fault("mounted phase requires the live instance"))?;
        if A::mounted_jobs_terminal_is_empty(instance_id) { return Ok(MountedOwnerPhaseV1::Maintenance); }
        A::mounted_job_maintenance_phase(instance_id).ok_or_else(|| plugin_sdk_fault("retained mounted owner did not declare its preparation or maintenance phase"))
    }

    fn mounted_owner_maintenance_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let identity=turn.identity();if self.tools.live_runtime_instance_id!=Some(identity.instance_id)||!matches!(turn.phase(),MountedOwnerPhaseV1::Preparation|MountedOwnerPhaseV1::Maintenance){return Err(plugin_sdk_fault("mounted maintenance does not own this live instance and phase").into())}
        let step=turn.advance_lifecycle(|cx,loan|if !A::mounted_jobs_terminal_is_empty(identity.instance_id){let demand=A::mounted_job_maintenance_demands(identity.instance_id,loan.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,actual|Ok((A::mounted_job_maintenance_step(identity.instance_id,same,actual)?,true)))}else{self.maintenance_step(cx,loan)})?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.maintenance_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn mounted_owner_close_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let identity=turn.identity();if self.tools.live_runtime_instance_id!=Some(identity.instance_id)||turn.phase()!=MountedOwnerPhaseV1::Close{return Err(plugin_sdk_fault("mounted close does not own this live instance and phase").into())}
        let step=turn.advance_lifecycle(|cx,loan|if !A::mounted_jobs_terminal_is_empty(identity.instance_id){let demand=A::mounted_job_close_demands(identity.instance_id,loan.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,actual|Ok((A::mounted_job_close_step(identity.instance_id,same,actual)?,A::mounted_jobs_terminal_is_empty(identity.instance_id))))}else{self.close_step(cx,loan)})?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.close_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.constructor.has_constructor_receipt() { return Ok(Default::default()); }
        if !self.constructor.constructor_terminal_is_empty() { return self.stores.constructor_close_demands(&self.constructor, body); }
        self.close_ladder_demand(body)
    }

    fn maintenance_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.ladder_ref().maintenance_ladder_demands(body)
    }

    async fn bind_instance_id(&mut self, instance_id: u32) {
        self.tools.live_runtime_instance_id = Some(instance_id);
    }

    async fn bind_actor(&mut self, actor: &str) {
        assert_eq!(self.stores.store.local_actor_id().0.as_str(), actor, "opening actor must own the document store from construction");
        assert_eq!(self.stores.config_store.local_actor_id().0.as_str(), actor, "opening actor must own the config store from construction");
        assert_eq!(self.stores.draft_store.local_actor_id().0.as_str(), actor, "opening actor must own the draft store from construction");
        assert_eq!(self.stores.interaction_store.local_actor_id().0.as_str(), actor, "opening actor must own the interaction store from construction");
        assert_eq!(self.stores.window_config_store.local_actor_id().0.as_str(), actor, "opening actor must own the window config registry from construction");
    }

    /// 🎟️ Advances one genuine original owner under the complete caller grant.
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if self.constructor.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"original constructor receipt awaits its phase recipient"})}
        let demand=self.close_retirement_demands(loan.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,grant|{
            if !self.constructor.constructor_terminal_is_empty(){self.stores.close_store_constructor_unit(&mut self.constructor, grant)?;let progress=self.constructor.constructor_receipt().map_or(Default::default(),|(_,_,_,progress)|*progress);self.constructor.take_constructor_receipt();return Ok((PluginLifecycleStep::Progress(progress),false))}
            let step=self.close_ladder_step(same,grant)?;let terminal=self.close_terminal_is_empty();Ok((PluginLifecycleStep::retained(step,terminal),terminal))
        })
    }

    fn close_terminal_is_empty(&self) -> bool { self.close_terminal_is_empty() }

    /// 🎡️ Visits the live-maintenance units in fair order and forwards the unchanged grant to the first that releases something.
    fn maintenance_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        self.maintenance_ladder_step(cx,loan)
    }

    fn maintenance_terminal_is_empty(&self) -> bool {
        self.ladder_ref().maintenance_ladder_terminal_is_empty()
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

    async fn tool_public_contracts(&self) -> Vec<ArtifactToolPublicContract> { self.tool_public_contracts().await }

    async fn advance_typed_operation_publication(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if self.interaction.local_interaction_query_receipt.is_some(){return Ok(())}
        if self.constructor.has_constructor_receipt() { return Ok(()); }
        if self.constructor.store_constructor_is_pending() { ({ let __a0 = semio_framework_plugin_reactor_app::original_plugin_turn_grant(self.mounted_policy.preparation,retained); self.stores.drive_store_constructor_unit(&mut self.constructor, __a0) })?; return Ok(()); }
        if InstanceExtensions::holds_receipt(&self.time_travel, &self.tool_runs) || (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index|self.tools.tool_operations.entry(index).is_some_and(|(_,owner)|owner.publication_ownership_progress.is_some()||owner.original_retirement_receipt.is_some())){return Ok(());}
        if !self.pending.reserved_commits.is_empty() && self.pending.reserved_commit_outcome.is_none() {
            self.command_ctx().step_framework_reserved_commit().await?;
            return Ok(());
        }
        if InstanceExtensions::drive_turn(&mut self.extension_ctx(), &mut retained, identity, cx).await? {
            return Ok(());
        }
        if self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work()) {
            self.interaction.local_interaction_query_turn = !self.interaction.local_interaction_query_turn;
            if self.interaction.local_interaction_query_turn {
                self.interaction.advance_local_interaction_query_one(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained), semio_framework_plugin_interaction::query::LOCAL_INTERACTION_QUERY_PAGE_BYTES)?;
                return Ok(());
            }
        }
        self.typed_ctx().advance_typed_operation_publication_one(cx).await.map_err(Into::into)
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

    fn close_diagnostic(&self) -> String {
        self.ladder_ref().close_rung_name()
    }

    fn typed_operation_diagnostic(&self) -> String {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).filter_map(|index| {
            let (id, operation) = self.tools.tool_operations.entry(index)?;
            Some(format!("operation {id} '{}' stage={:?} session={} terminal={} refusal={}", operation.verb, operation.stage, operation.session.is_some(), operation.terminal_seen, operation.retained_structural_refusal().unwrap_or_else(|| "none".into())))
        }).collect::<Vec<_>>().join("; ")
    }

    fn has_pending_typed_operations(&self) -> bool {
        self.constructor.store_constructor_is_pending() || self.tools.operation_progress_retired
            || self.child_composition.derivable_follow_awaits_retirement(&self.stores)
            || InstanceExtensions::has_pending_work(&self.extension_ref())
            || !self.pending.reserved_commits.is_empty()
            || self.pending.reserved_commit_outcome.is_some()
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
            || !self.tools.tool_operations.is_empty()
            || self.ingest.has_runnable_artifact_envelope_decode_worker_step()
            || !self.tools.latest_wins_commands.is_empty()
            || self.tools.typed_effect_outbox.len() != 0
            || self.tools.typed_event_outbox.len() != 0
            || self.tools.typed_ui_outbox.len() != 0
            || self.tools.typed_completion_outbox.len() != 0
            || self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work())
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

    fn take_typed_operation_result_page(&mut self, receiver: u32,grant:RetainedCloneGrant) -> Result<(Option<TypedOperationResultPage> ,RetainedCloneProgress),Fault>{
        let selected = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (self.tools.typed_result_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.tools.tool_operations.entry(index).filter(|(_, operation)| operation.meta.instance_id == receiver && !operation.result_page_presented && operation.result_page.is_some()).map(|(id, _)| (index, *id))
        });
        let Some((index,operation_id))=selected else{return Ok((None,Default::default()))};let demand=self.typed_operation_result_page_demands(receiver)?;
        if !plugin_turn_admitted(demand,grant)?{return Ok((None,Default::default()))}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-std::mem::size_of::<usize>(),..grant};let(page,mut progress)=self.tools.tool_operations.get_mut(operation_id).expect("selected result owner remains mounted").take_result_page(child)?;
        if page.is_some(){self.tools.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        progress.tools.copied_bytes+=std::mem::size_of::<usize>()
    ;}Ok((page,progress))
    }

    fn acknowledge_typed_operation_result(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault> { self.acknowledge_typed_operation_result(token) }

    fn take_typed_operation_effect(&mut self) -> Option<Effect> {
        self.tools.typed_effect_outbox.pop()
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

    async fn take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault> { self.take_typed_operation_completion().await }

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

    async fn handle_action(&mut self, action: &str, args: &mut Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_action(action, args, &mut None, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        self.child_composition.follow_derivable_children(&mut self.stores, identity).await?;
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, action, result).await))
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

    async fn handle_action_invocation(&mut self, address: &semio_framework::manifest::ActionAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> { self.handle_action_invocation(address, source_args, original_fault, active_mode_id, meta, identity, cx).await }

    async fn handle_response_action(&mut self, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if !self.tool_machines.ingress.action_source_capture_ready(action, cx){return Ok(None)}
        let definition = self.tools.registry.app_command(action).await.ok_or_else(|| plugin_sdk_fault(format!("continuation command '{action}' is not owned by the receiving app")))?;
        validate_ui_dispatch_classification("receiving app", action, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        let Some(command) = self.tool_machines.ingress.receive_action_command(action, args, original_fault, cx).await? else { return Ok(None) };
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&self.stores, action, &wire, 1).await?;
        self.media.require_complete_tool_operation_pipeline(&admission)?;
        let log_generation_before = self.history.log_generation;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result = self.dispatch_ctx().dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await;
        self.host_events.invocation_kind = previous_kind;
        let result = result?;
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, action, result).await))
    }

    async fn handle_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_command(address, source_args, original_fault, active_mode_id, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, &address.command_id, result).await))
    }

    fn original_command_frame_pending(&self)->bool{self.tools.agent_lane_preview.is_some()}

    async fn handle_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_command_frame(command_bytes, meta, identity, cx).await?;
        match result{Some(result)=>Ok(Some(self.render_ctx().finish_recorded(log_generation_before,"typed-command",result).await)),None=>Ok(None)}
    }

    /// 🎯️ M1: resolves `A::command_from_intent(intent)` then dispatches through the SAME
    /// `dispatch_typed_command_inner` `dispatch_typed_command`/`dispatch_typed` use — kind
    /// discipline, the command log, undo grouping and `AsyncTask` spawning apply unchanged.
    async fn handle_intent_frame(&mut self, intent: &UiIntent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let verb = intent.action.name.as_str();
        if intent.action.version != 1 {
            return Err((Fault::new(FaultOrigin::App, FaultCode::new("app.intent.version-mismatch"), format!("intent action '{verb}' targets version {} but the intent bridge only resolves version-1 actions", intent.action.version))).into());
        }
        let _ = self.tools.qualified_tool_proof(&self.stores, verb)?;
        if !self.tool_machines.ingress.action_source_capture_ready(verb, cx){return Ok(None)}
        let mut merged = if self.tool_machines.ingress.action_source_pending(){None}else{merge_ui_values(intent.args.as_ref(), intent.input.as_ref()).await?};
        let Some(command)=self.tool_machines.ingress.receive_action_command(verb, &mut merged, &mut None, cx).await?else{return Ok(None)};
        self.dispatch_ctx().dispatch_typed(command, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes the task resolution's own
    /// `OpBinary`-encoded `A::Command` and re-enters `dispatch_typed` with it — see the trait
    /// method's own doc for why this is the applying lane and not `handle_command_frame`.
    async fn resume_task_command(&mut self, command_bytes: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let command = <A::Command as ::protocol::OpBinary>::decode_op(&command_bytes)
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.task-resume-command"), format!("an AsyncTask resolved TaskResolution::Command with bytes that are not this app's typed Command: {error}")))?;
        self.dispatch_ctx().dispatch_typed(command, meta, identity, cx).await
    }

    async fn resume_task_emit(&mut self, artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> { self.resume_task_emit(artifact_ops, config_ops, draft_ops, meta, identity, cx).await }

    async fn take_last_emit_wire(&mut self) -> Option<EmitWire> {
        self.pending.last_emit_wire.take()
    }

    async fn take_pending_presence(&mut self) -> Vec<PresenceUpdate> {
        std::mem::take(&mut self.roster.pending_presence)
    }

    async fn take_pending_transaction_proposal(&mut self) -> Option<TransactionProposalDraft> {
        self.txn.pending_transaction_proposal.take()
    }

    async fn transaction_prepare(&mut self, txn_id: &str, _mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome { self.transaction_prepare(txn_id, _mutation_id, payload, prepared_ops, prepared_child_ops, origin).await }

    async fn transaction_commit(&mut self, txn_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault> { self.transaction_commit(txn_id, meta, identity).await }

    async fn transaction_rollback(&mut self, txn_id: &str) -> Result<(), Fault> {
        match &self.txn.pending_transaction {
            Some(pending) if pending.txn_id == txn_id => {
                self.txn.pending_transaction = None;
                Ok(())
            }
            _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.rollback-unknown"), format!("transaction_rollback: no pending transaction matches txn_id {txn_id:?}"))),
        }
    }

    async fn transaction_undo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.child_composition.children.is_empty() {
            return self.dispatch_ctx().transaction_group_history("undo", group_id, identity).await;
        }
        if self.stores.store.tail_group_id().await.as_deref() != Some(group_id) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-foreign-tail"), format!("transaction_undo: this instance's tail edit does not belong to group {group_id:?}")));
        }
        self.stores.store.undo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-failed"), format!("{error:?}")))?;
        self.history.cache = None;
        Ok(())
    }

    async fn transaction_redo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.child_composition.children.is_empty() {
            return self.dispatch_ctx().transaction_group_history("redo", group_id, identity).await;
        }
        match self.stores.store.redo_tail().await {
            Some((_, Some(tail_group))) if tail_group == group_id => {
                self.stores.store.redo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.redo-failed"), format!("{error:?}")))?;
                self.history.cache = None;
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

    async fn hydrate_pure_head(&mut self,head:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> { self.hydrate_pure_head(head, cx).await }

    async fn ephemeral_snapshot(&self) -> EphemeralSnapshot { self.ephemeral_snapshot().await }

    fn reserve_presence_ingress(&mut self, seq: u64) -> Result<PresenceRosterAdmission, Fault> {
        let generation = self.roster.peer_roster_admission_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-generation"), "peer roster admission generation exhausted"))?;
        let slot = InstanceStores::<A>::peer_roster_slot(generation);
        if self.roster.peer_roster_reservations[slot].is_some()
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
        Ok(PresenceRosterAdmission { seq, generation, cancel: self.roster.peer_roster_scope.child_now() })
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

    async fn load_window_config_pack(&mut self, pack: WindowConfigPack) -> Result<(), Fault> {
        self.stores.window_config_store.load(pack).await
    }

    async fn load_child_pack(&mut self, slot: &str, child_id: &str, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.child_composition.open_child(&mut self.stores, slot, child_id, dialect, envelope_pack, identity).await?;
        self.history.cache = None;
        Ok(())
    }

    /// 🔢️ Sorted by `(slot, child_id)`: the map's iteration order is not stable, and a
    /// persisted child list that reshuffles between reads would make every save look like a
    /// change to anything diffing it.
    async fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, Fault> { self.child_composition.child_packs().await }

    async fn child_head_packs(&self) -> Result<Vec<protocol::ChildHeadPackEntry>, Fault> { self.child_composition.child_head_packs().await }

    fn begin_document_archive_load(&mut self, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> { self.begin_document_archive_load(operation, archive) }

    fn begin_document_archive_merge(&mut self, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        PluginApp::begin_document_archive_load(self, operation, archive)?;
        self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge admission changed before it was flagged"))?.merge = true;
        Ok(())
    }

    /// 📬️ Observes the original mounted load; its registered maintenance job drives retained work.
    async fn poll_document_archive_load(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
        if self.ingest.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::awaits_merge) {
            self.command_ctx().merge_document_archive(operation, identity, cx).await?;
        }
        self.ingest.document_archive_loads.get(operation).map(ActiveDocumentArchiveLoad::status).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive authority changed before status publication"))
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

    async fn document_archive(&self) -> Result<protocol::DocumentArchivePack, Fault> {
        let parent_generation = self.stores.store.generation_now();
        let child_generation = self.child_composition.child_content_generation;
        let parent = store::os_store::text_format::print_document_pack(self.stores.store.envelope()).await.map_err(|error| error.into_fault())?;
        let members = self.child_composition.archive_member_entries().await?;
        if self.stores.store.generation_now() != parent_generation || self.child_composition.child_content_generation != child_generation {
            return Err(document_load_fault(DOCUMENT_LOAD_CHANGED_CODE, "document archive authority changed during generation-fenced export"));
        }
        Ok(protocol::DocumentArchivePack { parent_pack: parent.pack, parent_spr: parent.spr, members })
    }

    async fn load_config_pack(&mut self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<(), Fault> {
        let parsed: store::os_store::text_format::ParsedDocumentText<A::Config, A::ConfigMutation> = store::os_store::text_format::parse_document_pack(&files.pack, &files.spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| error.into_fault())?;
        self.stores.config_store.reset(parsed.into_envelope()).await.map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    async fn dispatch_config_command(&mut self, command_bytes: &mut Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let Some(result)=self.dispatch_ctx().dispatch_config_command_inner(command_bytes,meta,identity,cx).await? else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before,"configCommand",result).await))
    }

    async fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let envelopes = protocol::decode_envelopes(mutations).map_err(|error| error.into_fault())?;
        let mut reports = Vec::with_capacity(envelopes.len());
        let generation = self.stores.store.generation();
        for envelope in envelopes {
            reports.push(self.stores.store.ingest_remote(envelope).await.map_err(|error| error.into_fault())?);
        }
        self.history.cache = None;
        if self.stores.store.generation() != generation {
            self.dispatch_ctx().deliver_base_moved(identity, cx).await?;
        }
        Ok(reports)
    }

    async fn ingest_operations_text(&mut self, operations_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut mutations: Vec<A::Mutation> = Vec::new();
        for line in operations_text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            mutations.push(A::Mutation::parse_op(line).map_err(|error| error.into_fault())?);
        }
        if mutations.is_empty() {
            return Ok(());
        }
        self.stores.store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    async fn document_text(&self) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
        store::os_store::text_format::print_document_text(self.stores.store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn document_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
        store::os_store::text_format::print_document_pack(self.stores.store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn attach_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.stores.store.attach_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.child_composition.announce_member_event_logs(&mut self.stores).await?;
        self.history.cache = None;
        Ok(())
    }

    async fn attach_hot_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.stores.store.attach_hot_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    async fn tick_backbone(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let generation = self.stores.store.generation();
        let reports = self.stores.store.tick_backbone_reports().await.map_err(|error| error.into_fault())?;
        if self.stores.store.generation() != generation {
            self.dispatch_ctx().deliver_base_moved(identity, cx).await?;
        }
        self.child_composition.follow_derivable_children(&mut self.stores, identity).await?;
        let folded_member_lanes = self.child_composition.fold_member_inbound(&mut self.stores, &mut self.history).await?;
        if !reports.is_empty() || folded_member_lanes != 0 {
            self.history.cache = None;
        }
        Ok(reports)
    }

    async fn detach_backbone(&mut self) -> Result<(), Fault> {
        self.stores.store.detach_backbone().map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<()>,semio_framework_job::OriginalTurnFailure> { self.render(body_key, snapshot_override_json, view_state, cx).await }

    fn original_render_tree_source<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<&'a mut Option<ComponentTree>,semio_framework_job::OriginalTurnFailure> { self.pending.original_render_tree_source(cx) }

    async fn window_engagements(&mut self, view_state: &ViewModel) -> HashMap<String, WindowEngagement> { self.window_engagements(view_state).await }

    async fn window_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> { self.window_measures(view_state).await }

    async fn tool_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> { self.tool_measures(view_state).await }

    async fn app_catalogue_json(&mut self) -> String {
        A::app_catalogue_json().await
    }

    async fn pending_effects(&mut self, view: Option<&ViewModel>) -> Vec<Effect> { self.pending_effects(view).await }

    async fn context_menu(&mut self, request: &ContextMenuRequest, view_state: &ViewModel) -> Vec<ContextMenuItemSpec> { self.context_menu(request, view_state).await }

    async fn produce_media(&mut self, port: &str, owner: &mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,semio_framework_plugin_decl_app::MediaArtifactOperationFailure> { self.produce_media(port, owner, cx).await }

    async fn prepare_media_import(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let source=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
        if let Some((_,original))=self.media.pending_media_import_permit.as_ref(){
            if *original!=source{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media import requires its retained original receiving source").into())}
            return Ok(!cx.should_yield()&&cx.retained_grant().maximum_items>0&&cx.retained_grant().maximum_depth>0)
        }
        let meta=ActionMeta{actor:self.stores.store.local_actor_id().0.clone(),instance_id:self.tools.live_runtime_instance_id.ok_or_else(||plugin_sdk_fault("media import requires its mounted live instance"))?,view_state:None};
        let Some(permit)=self.dispatch_ctx().admit_reserved_emit_producer("import-media",&meta,cx)?else{return Ok(false)};
        *self.media.pending_media_import_permit=Some((permit,source));Ok(false)
    }

    async fn consume_media(&mut self, port: &str, artifact: MediaArtifact, cx:&mut semio_framework_job::StepContext<'_>) -> Result<MediaConsumption, MediaArtifactError> { self.consume_media(port, artifact, cx).await }

    /// 🎞️ Batch-only adapter. Registered interactive outputs are driven through the same
    /// externally addressable submission API; this loop is not used by UI/workflow entry points.
    async fn export_media(&mut self,port:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Media>,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(None)}
        if self.tools.app_tool_registrations.keys().any(|verb|verb.strip_prefix("export-media:")==Some(port)){
            let operation_id=if let Some(id)=self.media.current_media_export{if self.media.media_exports.get(id).is_none_or(|active|active.port!=port){return self.media.retain_media_submission_refusal(MediaError::Payload(port.to_string(),"current export belongs to a different original port".into()), cx)}id}else{let Some(instance)=self.tools.live_runtime_instance_id else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media requires its mounted instance").into())};if !self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, port, instance, None, cx).await?{return Ok(None)}self.media.current_media_export.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media publication lost its live operation"))?};
            if self.media.media_export_preparation.is_some(){
                self.media.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media lost its original supplier"))?.validate(cx)?;
                let grant=cx.retained_grant();self.media.close_media_submission_refusal(&mut self.tools, cx, grant)?;return Ok(None)
            }
            return match self.media.receive_owned_media_poll(&mut self.stores, operation_id, cx).await?{None|Some(ArtifactMediaExportPoll::Running{..})=>Ok(None),Some(ArtifactMediaExportPoll::Complete(result))=>match result.into_batch_media(){Ok(media)=>Ok(Some(media)),Err(source)=>self.media.retain_media_submission_refusal(source, cx)},Some(ArtifactMediaExportPoll::Cancelled)=>self.media.retain_media_submission_refusal(MediaError::Payload(port.to_string(),"media export cancelled".into()), cx),Some(ArtifactMediaExportPoll::Failed(detail))=>self.media.retain_media_submission_refusal(MediaError::Payload(port.to_string(),detail), cx)};
        }
        self.render_ctx().refresh_cache().await?;
        let render_operation=self.tools.live_render_operation(&self.mounted_policy, &self.stores);let parent_document_id=self.stores.store.envelope().id.clone();
        let VcsArtifactApp{app:_,cache,child_content_root,transient_store,instance_operation_owner,..}=self;
        let(_,snapshot,config,history)=cache.as_ref().expect("cache refreshed above");
        let doc=ArtifactView::with_render_context(snapshot.as_ref(),history.as_ref(),ChildContentView::clone(child_content_root),render_operation,parent_document_id,None).await.with_instance(instance_operation_owner.clone());let _cfg=ConfigView{snapshot:config.as_ref(),window:None};let transient=transient_store.current_root();
        match A::export_media_with_request_context(instance_operation_owner,port,&doc,&TransientView{snapshot:transient.as_ref(),window:None}).await{Ok(media)=>Ok(Some(media)),Err(source)=>self.media.retain_media_submission_refusal(source, cx)}
    }

    async fn submit_media_export(&mut self,port:&str,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let Some(app_instance_id)=self.tools.live_runtime_instance_id else{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()), cx).map(|_|false)};
        self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, port, app_instance_id, Some(receiver), cx).await
    }

    async fn submit_media_export_checked(&mut self,port:&str,expected_parent_document_id:&str,expected_base_revision:u64,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let parent_document_id=self.stores.store.envelope().id.as_str();let content_revision=self.stores.store.content_revision();let base_revision=u64::from_be_bytes(content_revision[..8].try_into().expect("revision lane width"));
        if parent_document_id!=expected_parent_document_id||base_revision!=expected_base_revision{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"media export resource authority is stale".into()), cx).map(|_|false)}
        let Some(app_instance_id)=self.tools.live_runtime_instance_id else{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()), cx).map(|_|false)};
        self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, port, app_instance_id, Some(receiver), cx).await
    }

    async fn poll_media_export(&mut self,handle:semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<ArtifactMediaExportPoll>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        self.media.snapshot_retirements.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling lost its original source"))?.validate(cx)?;
        if receiver.is_some(){return Ok(true)}
        if self.media.media_exports.get(handle.operation_id.0).is_none_or(|active|active.handle.original_loan()!=handle){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling requires its exact original live handle").into())}
        let grant=cx.retained_grant();let bytes=size_of::<ArtifactMediaExportPoll>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let Some(poll)=self.media.receive_owned_media_poll(&mut self.stores, handle.operation_id.0, cx).await?else{return Ok(false)};
        if matches!(poll,ArtifactMediaExportPoll::Complete(_)){self.media.media_closures.get_mut(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"terminal media poll lost its original close owner"))?.download_pending=true}
        *receiver=Some(poll);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    async fn cancel_media_export(&mut self, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        self.media.cancel_owned_media_export(handle).await
    }

    async fn retain_media_export_result(&mut self,handle:semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandleLoan<'_>,source:&mut Option<ArtifactMediaExportPoll>,status:&mut Option<semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportPublishedStatus>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> { self.retain_media_export_result(handle, source, status, cx).await }

    async fn take_media_export_chunk(&mut self,handle:semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> { self.media.take_media_export_chunk(handle, receiver, terminal, cx).await }

    async fn take_segmented_download_chunk(&mut self, operation_id: u64) -> Result<Option<Vec<u8>>, Fault> { self.media.take_segmented_download_chunk(operation_id).await }

    async fn import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let log_generation_before = self.history.log_generation;
        let result = self.dispatch_ctx().dispatch_import_media(port, media, meta, cx).await?;
        Ok(self.render_ctx().finish_recorded(log_generation_before, &format!("import-media:{port}"), result).await)
    }

    async fn media_fingerprint(&mut self, port: &str) -> Result<MediaFingerprint, MediaError> {
        self.render_ctx().refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, child_content_root, instance_operation_owner, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, None).await.with_instance(instance_operation_owner.clone());
        let _cfg = ConfigView { snapshot: config.as_ref(), window: None };
        A::media_fingerprint(port, &doc).await
    }
}
