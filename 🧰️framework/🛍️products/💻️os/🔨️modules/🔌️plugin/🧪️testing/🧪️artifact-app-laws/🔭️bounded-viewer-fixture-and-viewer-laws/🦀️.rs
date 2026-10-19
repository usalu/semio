//! 🔭️ Bounded viewer fixture and viewer laws: the bounded viewer fixture with its artifact viewer implementation, the editor command reduction and the viewer mutation, dialect and dependency assertions. See [`super`].

use super::*;

#[doc(hidden)]
pub struct BoundedViewerFixture<V: ArtifactViewer>(std::marker::PhantomData<V>);

impl<V: ArtifactViewer> Default for BoundedViewerFixture<V> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<V> ArtifactViewer for BoundedViewerFixture<V>
where
    V: ArtifactViewer<Presence = semio_framework_plugin_decl_views::NoPresence, PresenceMutation = semio_framework_plugin_decl_views::NoPresenceMutation, Transient = semio_framework_plugin_decl_views::NoTransient, TransientMutation = semio_framework_plugin_decl_views::NoTransientMutation>,
{
    fn document_store_owners_source_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{V::document_store_owners_source_demands()}
fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        V::build_document_store_owners(grant).or_else(|| Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Snapshot, Self::Mutation>(grant)))
    }

    fn config_store_owners_source_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{V::config_store_owners_source_demands()}
fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        V::build_config_store_owners(grant).or_else(|| Some(store::os_store::document_store_owners::bounded_artifact_store_owners::<Self::Config, Self::ConfigMutation>(grant)))
    }

    fn build_document_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ArtifactStore<Self::Snapshot, Self::Mutation>> {
        V::build_document_store_disposer().or_else(|| Some(semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>()))
    }

    fn build_config_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        V::build_config_store_disposer().or_else(|| Some(semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>()))
    }

    fn build_presence_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        V::build_presence_store_disposer().or_else(|| Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_store_disposer()))
    }

    fn build_transient_store_disposer() -> semio_framework_plugin_ownership::store_lifecycle::ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        V::build_transient_store_disposer().or_else(|| Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_store_disposer()))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        V::build_presence_peer_retirement_factory().or_else(|| Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_peer_retirement_factory()))
    }

    const ROLE: semio_framework::AppRole = V::ROLE;
    const DIALECT: semio_framework_artifact_reference::Dialect = V::DIALECT;
    const DOCUMENT_SCHEMA: &'static str = V::DOCUMENT_SCHEMA;
    /// 🧩️ The viewer's own roster: the fixture forwards `genesis_child_pack`, and a derived
    /// child's dialect must resolve through the roster that declares it, or construction
    /// panics at `seed_genesis_children` (energy/gis viewers derive `s.stdio.semio` members).
    type Members = V::Members;
    type Snapshot = V::Snapshot;
    type Mutation = V::Mutation;
    type Config = V::Config;
    type ConfigMutation = V::ConfigMutation;
    type Presence = semio_framework_plugin_decl_views::NoPresence;
    type PresenceMutation = semio_framework_plugin_decl_views::NoPresenceMutation;
    type Transient = semio_framework_plugin_decl_views::NoTransient;
    type TransientMutation = semio_framework_plugin_decl_views::NoTransientMutation;
    type Command = V::Command;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework::Fault> {
        V::child_restore_projection(snapshot)
    }
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        V::genesis_child_pack(snapshot, slot, child_id)
    }

    fn mounted_job_maintenance_phase(instance_id: u32) -> Option<semio_framework_plugin_mounted_owner::MountedOwnerPhaseV1> { V::mounted_job_maintenance_phase(instance_id) }
    fn mounted_job_maintenance_demands(instance_id: u32, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { V::mounted_job_maintenance_demands(instance_id, body) }
    fn mounted_job_close_demands(instance_id: u32, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { V::mounted_job_close_demands(instance_id, body) }
    fn mounted_job_maintenance_step(instance_id: u32, cx: &mut semio_framework_job::StepContext<'_>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        V::mounted_job_maintenance_step(instance_id, cx, grant)
    }

    fn mounted_job_close_step(instance_id: u32, cx: &mut semio_framework_job::StepContext<'_>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        V::mounted_job_close_step(instance_id, cx, grant)
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        V::mounted_jobs_terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: semio_framework_plugin_decl_views::AppRenderOperationContext, snapshot: &Self::Snapshot, windows: &semio_framework_plugin_window_config::WindowConfigOwnerRegistry, view: Option<&semio_framework::ViewModel>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_plugin_decl_views::MountedSnapshotReadPreparation {
            V::mounted_job_prepare_snapshot_read(operation, snapshot, windows, view, grant)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry) -> Result<(), semio_framework::Fault> {
        V::register_window_transient_owners(registry)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin_window_config::WindowConfigOwnerRegistry) -> Result<(), semio_framework::Fault> {
        V::register_window_config_owners(registry)
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
        V::build_presence_local_root_retirement_factory().or_else(|| Some(semio_framework_plugin_decl_views::presence_retirement::no_presence_local_root_retirement_factory()))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        V::build_transient_local_root_retirement_factory().or_else(|| Some(semio_framework_plugin_decl_views::transient_retirement::no_transient_local_root_retirement_factory()))
    }

    fn ephemeral(
        command: &Self::Command,
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
        presence: &semio_framework_plugin_decl_views::PresenceView<'_, Self::Presence>,
        transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        V::ephemeral(command, doc, cfg, presence, transient)
    }

    fn config_schema() -> &'static str {
        V::config_schema()
    }

    fn initial_snapshot() -> Self::Snapshot {
        V::initial_snapshot()
    }

    fn initial_config() -> Self::Config {
        V::initial_config()
    }

    fn handle(
        command: &Self::Command,
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
        interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
        view_state: Option<&semio_framework::ViewModel>,
        engines: &semio_framework_2d_compute::EngineHandles,
    ) -> Result<semio_framework_plugin_decl_emission::surface_contract::ViewEmit<Self::ConfigMutation>, semio_framework::Fault> {
        V::handle(command, doc, cfg, interaction, view_state, engines)
    }

    fn command_id(command: &Self::Command) -> &'static str {
        V::command_id(command)
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework::Fault>) -> Result<Self::Command, semio_framework::Fault> {
        V::command_from_action(action, args, original_fault)
    }

    fn config_spec() -> semio_framework::ConfigSpec {
        V::config_spec()
    }

    fn interaction_topology(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
        V::interaction_topology(doc, cfg)
    }

    fn render(body_key: &str, doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> semio_framework_plugin_interaction::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        V::render(body_key, doc, cfg, view_state)
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
        view_state: &semio_framework::ViewModel,
        transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
        interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult<semio_framework_ui_runtime::ComponentTree> {
        V::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction,cx)
    }

    fn pending_effects(owner: &semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle, doc: &semio_framework_plugin_decl_views::ArtifactPendingView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view: Option<&semio_framework::ViewModel>) -> Vec<semio_framework::kernel::Effect> {
        V::pending_effects(owner, doc, cfg, view)
    }

    fn window_engagements(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> std::collections::HashMap<String, semio_framework_ui_layout::layout::WindowEngagement> {
        V::window_engagements(doc, cfg, view_state)
    }

    fn window_engagements_with_request_context(
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
        view_state: &semio_framework::ViewModel,
        transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
        interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
    ) -> std::collections::HashMap<String, semio_framework_ui_layout::layout::WindowEngagement> {
        V::window_engagements_with_request_context(doc, cfg, view_state, transient, interaction)
    }

    fn window_measures(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
        V::window_measures(doc, cfg, view_state)
    }

    fn window_measures_with_request_context(
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
        view_state: &semio_framework::ViewModel,
        interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
    ) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
        V::window_measures_with_request_context(doc, cfg, view_state, interaction)
    }

    fn tool_measures(doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>, cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>, view_state: &semio_framework::ViewModel) -> std::collections::HashMap<String, Vec<semio_framework_ui_layout::layout::WindowMeasure>> {
        V::tool_measures(doc, cfg, view_state)
    }

    fn app_catalogue_json() -> String {
        V::app_catalogue_json()
    }

    fn context_menu(
        request: &semio_framework_ui_layout::layout::ContextMenuRequest,
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
        view_state: &semio_framework::ViewModel,
        registry: &AppActionRegistry,
    ) -> Vec<semio_framework_ui_layout::layout::ContextMenuItemSpec> {
        V::context_menu(request, doc, cfg, view_state, registry)
    }

    fn context_menu_with_request_context(
        request: &semio_framework_ui_layout::layout::ContextMenuRequest,
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        cfg: &semio_framework_plugin_decl_views::ConfigView<'_, Self::Config>,
        view_state: &semio_framework::ViewModel,
        interaction: &semio_framework_plugin_decl_views::InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<semio_framework_ui_layout::layout::ContextMenuItemSpec> {
        V::context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        V::app_schema()
    }

    fn io() -> Option<semio_framework::AppIo> {
        V::io()
    }

    async fn media_ports() -> Vec<semio_framework::MediaPortSpec> {
        V::media_ports().await
    }

    fn export_media(port: &str, doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>) -> Result<semio_framework::Media, semio_framework::MediaError> {
        V::export_media(port, doc)
    }

    fn export_media_with_request_context(
        owner: &semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle,
        port: &str,
        doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>,
        transient: &semio_framework_plugin_window_transient::window_transient::TransientView<'_, Self::Transient>,
    ) -> Result<semio_framework::Media, semio_framework::MediaError> {
        V::export_media_with_request_context(owner, port, doc, transient)
    }

    fn media_fingerprint(port: &str, doc: &semio_framework_plugin_decl_views::ArtifactView<'_, Self::Snapshot>) -> Result<semio_framework::MediaFingerprint, semio_framework::MediaError> {
        V::media_fingerprint(port, doc)
    }
}

/// 🧪️ Invokes an editor's direct command reducer with empty local interaction and history.
pub fn reduce_editor_command<E: ArtifactEditor>(command: &E::Command, snapshot: &E::Snapshot) -> Result<semio_framework_plugin_decl_emission::Emit<E::Mutation, E::ConfigMutation, E::DraftMutation>, semio_framework::Fault> {
    let history = semio_framework_plugin_decl_views::history_view::HistoryView::empty();
    let config = E::initial_config();
    let draft = E::initial_draft();
    let state = protocol::InteractionState::default();
    let hover = semio_framework_plugin_decl_views::InteractionHoverState::new();
    let peers = semio_framework_plugin_decl_views::PeerPresenceRoot::empty();
    E::handle(
        command,
        &semio_framework_plugin_decl_views::ArtifactView::new(snapshot, &history),
        &semio_framework_plugin_decl_views::ConfigView { snapshot: &config, window: None },
        &semio_framework_plugin_decl_views::InteractionView::new(&state, &hover, &peers, semio_framework_plugin_interaction::interaction_topology::empty_draft_references()),
        None,
        &semio_framework_plugin_decl_views::DraftView { snapshot: &draft },
        &semio_framework_2d_compute::EngineHandles::empty(),
    )
}

/// 👁️ Exercises the production ViewerApp adapter with a representative viewer command.
/// The adapter must emit neither document nor draft mutations. A refusal emits nothing and so
/// satisfies the law — a retained-route viewer refuses a direct `handle` by design. Command
/// registration and admission are separate runtime contracts; a generic viewer fixture has no
/// declared factory for its author-defined default command.
///
/// 🧩️ Over the viewer's OWN roster (`V::Members`), never `new_viewer`'s `NoMembers`: the
/// fixture forwards `genesis_child_pack`, so a viewer deriving composed children would
/// otherwise refuse construction at `seed_genesis_children`.
pub async fn assert_viewer_never_mutates<V>(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
where
    V: ArtifactViewer<Presence = semio_framework_plugin_decl_views::NoPresence, PresenceMutation = semio_framework_plugin_decl_views::NoPresenceMutation, Transient = semio_framework_plugin_decl_views::NoTransient, TransientMutation = semio_framework_plugin_decl_views::NoTransientMutation>,
    V::Command: Default,
{
    let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=async{
    use super::super::{ArtifactView, ConfigView, DraftView, InteractionHoverState, InteractionView, NoDraft, PeerPresenceRoot};
    let mut app: VcsArtifactApp<ViewerApp<BoundedViewerFixture<V>>, V::Members> = VcsArtifactApp::new(ViewerApp::<BoundedViewerFixture<V>>::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    app.render_ctx().refresh_cache().await.expect("viewer fixture cache");
    let result = {
        let (_, snapshot, config, history) = app.history.cache.as_ref().expect("viewer fixture cache populated");
        let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
        let cfg = ConfigView { snapshot: config.as_ref(), window: None };
        let draft = NoDraft::default();
        let draft = DraftView { snapshot: &draft };
        let state = protocol::InteractionState::default();
        let hover = InteractionHoverState::new();
        let peers = PeerPresenceRoot::empty();
        let interaction = InteractionView::new(&state, &hover, &peers, semio_framework_plugin_interaction::interaction_topology::empty_draft_references());
        ViewerApp::<V>::handle(&V::Command::default(), &doc, &cfg, &interaction, None, &draft, &semio_framework_2d_compute::EngineHandles::empty()).await.map(|emit| (emit.artifact_mutations.is_empty(), emit.draft_mutations.is_empty()))
    };
    close_registered_fixture_app(&mut app, mounted_policy,cx).expect("registered fixture original close");
    if let Ok((artifact_is_empty, draft_is_empty)) = result {
        assert!(artifact_is_empty, "a viewer must never emit document mutations");
        assert!(draft_is_empty, "a viewer must never emit draft mutations");
    }

    }.await;
    let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
    result
}

/// ✏️👁️ Contract §2.5 helper 2/3 — an editor and its viewer over the same subset must resolve
/// to one `Dialect` coordinate, or `surface_app_id` would mint two different canonical ids
/// (`…#editor` / `…#viewer`) for what the contract treats as one subset's two surfaces.
pub async fn assert_editor_and_viewer_share_dialect<E: ArtifactEditor, V: ArtifactViewer>() {
    assert_eq!(E::DIALECT, V::DIALECT, "an editor and its viewer over the same subset must share one Dialect coordinate");
}

/// 🔗️ Contract freeze §3 — every surface a plugin contributes for a FOREIGN artifact kind must
/// be backed by a declared dependency on that kind's owner, or the host's `AppRouter` excludes
/// the whole plugin (`surface.contribution-not-permitted`). Asserts over an already-assembled
/// manifest so a bundle that borrows other plugins' apps proves the property from its own test,
/// not only from the assembly gate (ticket 26/09/05/S-END-TO-END lane H).
// 🚫️async: pure predicate over already-assembled manifest data, no suspension point — see R9.
pub fn assert_surface_dependencies_declared(manifest: &semio_framework::PluginManifest) {
    let breaches = semio_framework_plugin_decl_registry::artifact_registration::surface_dependency_breaches(manifest);
    assert!(breaches.is_empty(), "plugin `{}` contributes surfaces without the matching dependency: {}", manifest.plugin_id, breaches.join("; "));
}

/// 👁️ Contract §2.5 helper 3/3 — the viewer twin of `new_app::<A: ArtifactApp>()`. `ViewerApp<V>`
/// already satisfies the runtime `ArtifactApp` bound (the SDK adapter, contract §2.1), so this
/// is a thin rename over the existing generic harness rather than new machinery.
pub async fn new_viewer<V: ArtifactViewer>(actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<ViewerApp<V>> {
    new_app::<ViewerApp<V>>(actor, mounted_policy, identity).await
}
