//! 🖊️ Editor app: the app wrapper that turns an `ArtifactEditor` into an `ArtifactApp`. See [`super`].

use super::*;
use semio_framework_2d_compute::EngineHandles;

/// ✏️ Binds the authored editor and its original derived surface identity.
pub struct EditorApp<E:ArtifactEditor>{pub(super) id:String,_editor:std::marker::PhantomData<E>}

impl<E:ArtifactEditor> Default for EditorApp<E>{
    fn default()->Self{Self{id:surface_app_id(&E::DIALECT.into(),E::ROLE),_editor:std::marker::PhantomData}}
}

impl<E:ArtifactEditor> EditorApp<E>{
    /// 🪪️ Borrows the actual derived surface identity used by the owning stores.
    pub async fn surface_id(&self)->&str{&self.id}
}

impl<E: ArtifactEditor> ArtifactApp for EditorApp<E> {
    fn owned_mutation_batch_birth_bytes() -> Option<usize> { E::owned_mutation_batch_birth_bytes() }
    fn completion_retirement_birth_bytes(value:&ArtifactToolCompletionValue<Self>)->Option<usize>{E::completion_retirement_birth_bytes(value)}
    fn admit_completion_retirement(value:&mut Option<ArtifactToolCompletionValue<Self>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{E::admit_completion_retirement(value,grant)}

    fn admit_owned_mutation_batch(values: &mut Option<Vec<Self::Mutation>>, grant: RetainedCloneGrant) -> Result<Option<(store::MemberStoreOwnedBatch, RetainedCloneProgress)>, ValueError> { E::admit_owned_mutation_batch(values, grant) }

    const DIALECT: Dialect = E::DIALECT;
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        E::child_restore_projection(snapshot)
    }
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, ValueError> {
        E::genesis_child_pack(snapshot, slot, child_id)
    }
    const APP_ID: &'static str = "surface";
    const ROLE: AppRole = E::ROLE;
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = E::REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY;
    const DOCUMENT_SCHEMA: &'static str = E::DOCUMENT_SCHEMA;

    /// 🪪️ Overrides the `APP_ID` placeholder default — every ownership check must see the real
    /// derived surface id (`self.id`, see struct doc), matching what `ShellHost` addresses.
    async fn instance_id(&self) -> &str {
        self.surface_id().await
    }
    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        E::bounded_first_step_tool_proofs()
    }
    fn mounted_job_maintenance_phase(instance_id: u32) -> Option<MountedOwnerPhaseV1> { E::mounted_job_maintenance_phase(instance_id) }
    fn mounted_job_maintenance_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { E::mounted_job_maintenance_demands(instance_id, body) }
    fn mounted_job_close_demands(instance_id: u32, body: usize) -> Result<RetirementDemand, ValueError> { E::mounted_job_close_demands(instance_id, body) }
    fn mounted_job_maintenance_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        E::mounted_job_maintenance_step(instance_id,cx,grant)
    }
    fn mounted_job_close_step(instance_id: u32,cx:&mut semio_framework_job::StepContext<'_>,grant: RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        E::mounted_job_close_step(instance_id,cx,grant)
    }
    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        E::mounted_jobs_terminal_is_empty(instance_id)
    }
    fn mounted_job_prepare_snapshot_read(operation: AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &WindowConfigOwnerRegistry, view: Option<&ViewModel>, grant: RetainedCloneGrant) -> crate::MountedSnapshotReadPreparation {
        E::mounted_job_prepare_snapshot_read(operation, snapshot, windows, view, grant)
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        E::register_tool_job_factories(registry)
    }
    fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        E::build_tool_run_job(request)
    }
    fn build_retargetable_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault> {
        E::build_retargetable_tool_run_job(request)
    }
    fn register_window_transient_owners(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        E::register_window_transient_owners(registry)
    }
    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        E::register_window_config_owners(registry)
    }
    fn retained_window_transient_target(command: &Self::Command) -> Option<(&str, &'static str)> {
        E::retained_window_transient_target(command)
    }
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        E::interaction_scope(verb, domains)
    }
    fn operation_progress_scope() -> UiDirtyScope {
        E::operation_progress_scope()
    }
    fn host_event(event: &HostEvent) -> Option<Self::Command> {
        E::host_event(event)
    }
    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        E::build_instance_operation_owner()
    }
    fn build_tool_job(request:&mut Option<ArtifactOwnedToolJobRequest<Self>>,output:&mut Option<semio_framework::ErasedToolJob>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        E::build_tool_job(request,output,cx)
    }
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<Self>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        E::build_reserved_tool_job(request)
    }
    fn build_media_export_job(request:&mut Option<ArtifactMediaExportJobRequest<Self>>,body:&mut Option<Box<dyn semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        E::build_media_export_job(request,body,cx)
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        E::build_envelope_decode_owner_bundle()
    }
    fn document_store_owners_source_demands()->Result<RetirementDemand,ValueError>{E::document_store_owners_source_demands()}
    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        E::build_document_store_owners(grant)
    }
    fn config_store_owners_source_demands()->Result<RetirementDemand,ValueError>{E::config_store_owners_source_demands()}
    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        E::build_config_store_owners(grant)
    }
    fn draft_store_owners_source_demands()->Result<RetirementDemand,ValueError>{E::draft_store_owners_source_demands()}
    fn build_draft_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        E::build_draft_store_owners(grant)
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        E::build_artifact_store_one_item_preparation_factory()
    }
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        E::build_config_store_one_item_preparation_factory()
    }
    fn build_draft_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Draft, Self::DraftMutation>>> {
        E::build_draft_store_one_item_preparation_factory()
    }
    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        E::build_presence_store_one_item_preparation_factory()
    }
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        E::build_transient_store_one_item_preparation_factory()
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        E::build_presence_local_root_retirement_factory()
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        E::build_presence_peer_retirement_factory()
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        E::build_transient_local_root_retirement_factory()
    }
    fn build_document_store_initialization_job(
        envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: ActorId,
    ) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        E::build_document_store_initialization_job(envelope, operation, generation, actor)
    }
    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        E::validate_document_store_publication(operation, generation, live_generation)
    }
    fn build_document_store_disposer() -> ArtifactDisposal<ArtifactStore<Self::Snapshot, Self::Mutation>> {
        E::build_document_store_disposer()
    }
    fn build_config_store_disposer() -> ArtifactDisposal<ConfigStore<Self::Config, Self::ConfigMutation>> {
        E::build_config_store_disposer()
    }
    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        E::build_draft_store_disposer()
    }
    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        E::build_presence_store_disposer()
    }
    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        E::build_transient_store_disposer()
    }
    type Snapshot = E::Snapshot;
    type Mutation = E::Mutation;
    type Config = E::Config;
    type ConfigMutation = E::ConfigMutation;
    type Draft = E::Draft;
    type DraftMutation = E::DraftMutation;
    type Presence = E::Presence;
    type PresenceMutation = E::PresenceMutation;
    type Transient = E::Transient;
    type TransientMutation = E::TransientMutation;
    type Command = E::Command;

    async fn ephemeral(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, presence: &PresenceView<'_, Self::Presence>, transient: &TransientView<'_, Self::Transient>) -> EphemeralEmit<Self> {
        let (presence_mutations, transient_mutations) = E::ephemeral(command, doc, cfg, presence, transient);
        EphemeralEmit { presence: presence_mutations, transient: transient_mutations, window_transient: Vec::new() }
    }
    async fn config_schema() -> &'static str {
        E::config_schema()
    }
    fn catalogue_example_document(example_id: &str) -> Option<Result<String, Fault>> {
        let mut examples = E::examples();
        if examples.is_empty() {
            examples = subset_examples(&E::DIALECT);
        }
        if examples.is_empty() {
            return None;
        }
        if example_id.is_empty() {
            return Some(Ok(<E::Snapshot as pack::snapshot::ArtifactDsl>::print_dsl(&E::initial_snapshot())));
        }
        Some(match examples.into_iter().find(|source| source.id() == example_id) {
            Some(source) => Ok(source.document()),
            None => Err(Fault::new(FaultOrigin::App, FaultCode::new("app.example.unknown"), format!("no registered example '{example_id}'"))),
        })
    }
    async fn initial_snapshot() -> Self::Snapshot {
        E::initial_snapshot()
    }
    async fn initial_config() -> Self::Config {
        E::initial_config()
    }
    fn host_configuration_mutation(action: &str, args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        E::host_configuration_mutation(action, args)
    }
    fn typing_fold(net: &[Self::Mutation], next: &[Self::Mutation]) -> semio_framework_tool_machine::TypingFold<Self::Mutation> {
        E::typing_fold(net, next)
    }
    fn entity_label(snapshot: &Self::Snapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        E::entity_label(snapshot, kinds, id)
    }
    fn selection_reference_id(kinds: &[String], row: &str) -> Option<String> {
        E::selection_reference_id(kinds, row)
    }
    fn tool_intent_kinds(tool: &str) -> &'static [&'static str] {
        E::tool_intent_kinds(tool)
    }
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        E::fault_notices()
    }
    async fn initial_draft() -> Self::Draft {
        E::initial_draft()
    }
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        draft: &DraftView<'_, Self::Draft>,
        engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation> {
        E::handle(command, doc, cfg, interaction, view_state, draft, engines)
    }
    async fn command_id(command: &Self::Command) -> &'static str {
        E::command_id(command)
    }
    async fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        E::command_from_action(action, args, original_fault)
    }
    async fn agent_target_revision(action: &str, args: &DslValue, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        E::agent_target_revision(action, args, doc)
    }

    async fn config_spec() -> ConfigSpec {
        E::config_spec()
    }
    async fn clipboard_media_type() -> Option<MediaType> {
        E::clipboard_media_type()
    }
    async fn clipboard_accepts() -> Vec<MediaType> {
        E::clipboard_accepts()
    }
    async fn copy_fragment(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        E::copy_fragment(doc, cfg, interaction)
    }
    async fn cut_operations(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, interaction: &InteractionView<'_>) -> Vec<Self::Mutation> {
        E::cut_operations(doc, cfg, interaction)
    }
    async fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, ValueError> {
        E::interaction_topology(doc, cfg)
    }
    async fn paste_operations(doc: &ArtifactView<'_, Self::Snapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Self::Mutation>, ClipboardError> {
        E::paste_operations(doc, fragment, placement)
    }
    async fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &crate::ArtifactPendingView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view: Option<&ViewModel>) -> Vec<Effect> {
        E::pending_effects(owner, doc, cfg, view)
    }
    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        E::render(body_key, doc, cfg, view_state)
    }
    async fn render_with_instance_operation_owner(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
    ) -> UiAssemblyResult<ComponentTree> {
        E::render_with_instance_operation_owner(owner, body_key, doc, cfg, view_state)
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
        E::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction,cx)
    }
    async fn window_engagements(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        E::window_engagements(doc, cfg, view_state)
    }
    async fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        E::window_engagements_with_request_context(doc, cfg, view_state, transient, interaction)
    }
    async fn window_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        E::window_measures(doc, cfg, view_state)
    }
    async fn window_measures_with_request_context(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        E::window_measures_with_request_context(doc, cfg, view_state, interaction)
    }
    async fn tool_measures(doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        E::tool_measures(doc, cfg, view_state)
    }
    async fn app_catalogue_json() -> String {
        E::app_catalogue_json()
    }
    async fn context_menu(request: &ContextMenuRequest, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        E::context_menu(request, doc, cfg, view_state, registry)
    }
    async fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        E::context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)
    }
    async fn genesis() -> Vec<Self::Mutation> {
        E::genesis()
    }
    async fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        E::app_schema()
    }
    async fn io() -> Option<AppIo> {
        E::io()
    }
    async fn natural_file_codec() -> Option<NaturalFileCodec> {
        E::natural_file_codec()
    }
    async fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, MediaError> {
        E::encode_natural_file(snapshot)
    }
    async fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, MediaError> {
        E::decode_natural_file(bytes)
    }
    async fn media_ports() -> Vec<MediaPortSpec> {
        E::media_ports().await
    }
    async fn export_media(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Media, MediaError> {
        E::export_media(port, doc)
    }
    async fn export_media_with_request_context(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, Self::Snapshot>, transient: &TransientView<'_, Self::Transient>) -> Result<Media, MediaError> {
        E::export_media_with_request_context(owner, port, doc, transient)
    }
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Self::Snapshot>) -> ArtifactMutationOutcome<Self::Mutation, Self::ConfigMutation, Self::DraftMutation, MediaError> {
        E::import_media(port, media, doc)
    }
    async fn media_fingerprint(port: &str, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<MediaFingerprint, MediaError> {
        E::media_fingerprint(port, doc)
    }
}
