//! 📖️ Viewer app: the app wrapper that turns an `ArtifactViewer` into an `ArtifactApp`. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

/// 🗄️👁️ Adapter binding an `ArtifactViewer` author into the runtime `ArtifactApp` contract
/// (contract §2.1/§2.2). Draft-lane and clipboard/genesis/import methods are left to
/// `ArtifactApp`'s own defaults (empty/no-op) since `ArtifactViewer` never declares them — a
/// viewer never originates or imports a document. `handle` is the one real seam: it converts
/// `V::handle`'s `ViewEmit` into an `Emit` whose `artifact_mutations`/`draft_mutations` are always
/// empty BY CONSTRUCTION, built solely from `ViewEmit`'s three fields (contract §2.2) — never from
/// a caller-supplied mutation vector, so a hand-written runtime impl cannot smuggle one in either.
pub struct ViewerApp<V: ArtifactViewer> {
    /// 🪪️ See `EditorApp::id` — same placeholder/derivation split applies here.
    pub(super) id: String,
    _viewer: std::marker::PhantomData<V>,
}

impl<V: ArtifactViewer> Default for ViewerApp<V> {
    fn default() -> Self {
        Self { id: surface_app_id(&V::DIALECT.into(), V::ROLE), _viewer: std::marker::PhantomData }
    }
}

impl<V: ArtifactViewer> ViewerApp<V> {
    /// 🪪️ The real derived surface id — see `EditorApp::surface_id`.
    pub async fn surface_id(&self) -> &str {
        &self.id
    }
}

impl<V: ArtifactViewer> ArtifactApp for ViewerApp<V> {
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { V::owned_mutation_batch_birth_bytes() }
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<Self>)->Option<usize>{V::completion_retirement_birth_bytes(value)}
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<Self>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{V::admit_completion_retirement(value,grant)}

    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, RetainedCloneProgress)>, ValueError> { V::admit_owned_mutation_batch(values, grant) }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        V::build_envelope_decode_owner_bundle()
    }

    fn build_document_store_initialization_job(
        envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: ActorId,
    ) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        V::build_document_store_initialization_job(envelope, operation, generation, actor)
    }

    fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{V::document_store_owners_source_demands()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        V::build_document_store_owners(grant)
    }

    fn config_store_owners_source_demands()->Result<RetirementDemand,ValueError>{V::config_store_owners_source_demands()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        V::build_config_store_owners(grant)
    }

    fn build_document_store_disposer() -> ArtifactDisposal<ArtifactStore<Self::Snapshot, Self::Mutation>> {
        V::build_document_store_disposer()
    }

    fn build_config_store_disposer() -> ArtifactDisposal<ConfigStore<Self::Config, Self::ConfigMutation>> {
        V::build_config_store_disposer()
    }

    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        V::build_presence_store_disposer()
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        V::build_transient_store_disposer()
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        V::build_presence_peer_retirement_factory()
    }

    fn draft_store_owners_source_demands()->Result<RetirementDemand,ValueError>{store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<Self::Draft,Self::DraftMutation>()}
    fn build_draft_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<NoDraft, NoDraftMutation>(grant))
    }

    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    const DIALECT: Dialect = V::DIALECT;
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        V::child_restore_projection(snapshot)
    }
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, ValueError> {
        V::genesis_child_pack(snapshot, slot, child_id)
    }
    const APP_ID: &'static str = "surface";
    const ROLE: AppRole = V::ROLE;
    const DOCUMENT_SCHEMA: &'static str = V::DOCUMENT_SCHEMA;

    /// 🪪️ See `EditorApp::instance_id` — same placeholder-override reasoning applies here.
    async fn instance_id(&self) -> &str {
        self.surface_id().await
    }
    fn mounted_job_maintenance_phase(instance_id: u32) -> Option<MountedOwnerPhaseV1> { V::mounted_job_maintenance_phase(instance_id) }
    fn mounted_job_maintenance_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { V::mounted_job_maintenance_demands(instance_id, body) }
    fn mounted_job_close_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { V::mounted_job_close_demands(instance_id, body) }
    fn mounted_job_maintenance_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        V::mounted_job_maintenance_step(instance_id,cx,grant)
    }
    fn mounted_job_close_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        V::mounted_job_close_step(instance_id,cx,grant)
    }
    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        V::mounted_jobs_terminal_is_empty(instance_id)
    }
    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &WindowConfigOwnerRegistry, view: Option<&ViewModel>, grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
        V::mounted_job_prepare_snapshot_read(operation, snapshot, windows, view, grant)
    }
    fn register_window_transient_owners(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        V::register_window_transient_owners(registry)
    }
    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        V::register_window_config_owners(registry)
    }
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        V::bounded_first_step_tool_proofs()
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        V::register_tool_job_factories(registry)
    }
    fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        V::build_tool_run_job(request)
    }
    fn build_retargetable_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault> {
        V::build_retargetable_tool_run_job(request)
    }
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        V::retained_window_transient_target(command)
    }
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        V::interaction_scope(verb, domains)
    }
    fn operation_progress_scope() -> UiDirtyScope {
        V::operation_progress_scope()
    }
    fn host_event(event: &HostEvent) -> Option<Self::Command> {
        V::host_event(event)
    }
    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        V::build_instance_operation_owner()
    }
    fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        V::build_tool_job(request,output,cx)
    }
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        V::build_reserved_tool_job(request)
    }
    fn build_media_export_job(request:&mut Option<ArtifactMediaExportJobRequest<Self>>,body:&mut Option<Box<dyn semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        V::build_media_export_job(request,body,cx)
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        V::build_config_store_one_item_preparation_factory()
    }
    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        V::build_presence_store_one_item_preparation_factory()
    }
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        V::build_transient_store_one_item_preparation_factory()
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        V::build_presence_local_root_retirement_factory()
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        V::build_transient_local_root_retirement_factory()
    }
    type Snapshot = V::Snapshot;
    type Mutation = V::Mutation;
    type Config = V::Config;
    type ConfigMutation = V::ConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = V::Presence;
    type PresenceMutation = V::PresenceMutation;
    type Transient = V::Transient;
    type TransientMutation = V::TransientMutation;
    type Command = V::Command;

    async fn ephemeral(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, presence: &PresenceView<'_, Self::Presence>, transient: &TransientView<'_, Self::Transient>) -> EphemeralEmit<Self> {
        let (presence_mutations, transient_mutations) = V::ephemeral(command, doc, cfg, presence, transient);
        EphemeralEmit { presence: presence_mutations, transient: transient_mutations, window_transient: Vec::new() }
    }
    async fn config_schema() -> &'static str {
        V::config_schema()
    }
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        V::fault_notices()
    }
    async fn initial_snapshot() -> Self::Snapshot {
        V::initial_snapshot()
    }
    async fn initial_config() -> Self::Config {
        V::initial_config()
    }
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation> {
        let view_emit = V::handle(command, doc, cfg, interaction, view_state, engines)?;
        Ok(Emit { config_mutations: view_emit.config_mutations, effects: view_emit.effects, ui_scope: view_emit.ui_dirty, ..Default::default() })
    }
    async fn command_id(command: &Self::Command) -> &'static str {
        V::command_id(command)
    }
    async fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        V::command_from_action(action, args, original_fault)
    }
    async fn config_spec() -> ConfigSpec {
        V::config_spec()
    }
    async fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, ValueError> {
        V::interaction_topology(doc, cfg)
    }
    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        V::render(body_key, doc, cfg, view_state)
    }
    async fn render_with_request_context(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult<ComponentTree> {
        V::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction,cx)
    }
    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
        V::pending_effects(owner, doc, cfg, view)
    }
    async fn window_engagements(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        V::window_engagements(doc, cfg, view_state)
    }
    async fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        V::window_engagements_with_request_context(doc, cfg, view_state, transient, interaction)
    }
    async fn window_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        V::window_measures(doc, cfg, view_state)
    }
    async fn window_measures_with_request_context(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        V::window_measures_with_request_context(doc, cfg, view_state, interaction)
    }
    async fn tool_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        V::tool_measures(doc, cfg, view_state)
    }
    async fn app_catalogue_json() -> String {
        V::app_catalogue_json()
    }
    async fn context_menu(request: &ContextMenuRequest, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        V::context_menu(request, doc, cfg, view_state, registry)
    }
    async fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        V::context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)
    }
    async fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        V::app_schema()
    }
    async fn io() -> Option<AppIo> {
        V::io()
    }
    async fn media_ports() -> Vec<MediaPortSpec> {
        V::media_ports().await
    }
    async fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
        V::export_media(port, doc)
    }
    async fn export_media_with_request_context(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, Self::Snapshot>, transient: &TransientView<'_, Self::Transient>) -> Result<Media, MediaError> {
        V::export_media_with_request_context(owner, port, doc, transient)
    }
    async fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        V::media_fingerprint(port, doc)
    }
}
