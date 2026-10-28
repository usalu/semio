//! 🖥️ Drawing play app: the `DrawingPlayApp` editor, its body render and its `ArtifactEditor` implementation.

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️DrawingPlayApp
/// 🧪️ Drawing editor owner; durable content stays in the document while concrete Canvas windows own
/// persisted navigation and ephemeral engagement/trace progress.
pub struct DrawingPlayApp {
    arena_boot_fault: Option<&'static str>,
}

impl DrawingPlayApp {
    pub fn arena_boot_fault(&self) -> Option<&'static str> {
        self.arena_boot_fault.or_else(semio_s_artifact_draw_drawing_host_arena::drawing_mutation_arena_pool_fault)
    }
}

fn render_drawing_body(
    body_key: &str,
    doc: &ArtifactView<'_ ,DrawingSnapshot>,
    config: &DrawingCanvasWindowConfig,
    preview: &DrawingGesturePreview,
    selection: &[String],
    point_selection: &[String],
    view_state: &semio_framework_plugin::ViewModel,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let document=doc.snapshot;
    let labels = semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state);
    let active_utility = drawing_active_utility(view_state);
    let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
    let root = match body_key {
        DRAWING_PLAY_BODY_COMPOSITE => semio_s_artifact_draw_drawing_editor_interaction::geometry_session::with_visual(doc.render_operation(),|plan,revision,fresh|{let idle=DrawingGesturePreview::default();canvas_window::render(plan,revision,document,config,if fresh{preview}else{&idle},active_utility,if fresh{selection}else{&[]},if fresh{point_selection}else{&[]})}),
        DRAWING_PLAY_BODY_LAYERS => layers_panel::render(document, labels, &windows),
        DRAWING_PLAY_BODY_CATALOGUE => catalogue_panel::render(document, labels, &windows),
        DRAWING_PLAY_BODY_PROPERTIES => properties_panel::render(document, &[], labels, &windows),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("drawing.body.label", "the fixed Drawing unknown-body label exceeds its UI bound")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(root))
}

impl Default for DrawingPlayApp {
    fn default() -> Self {
        let arena_boot_fault = match semio_s_artifact_draw_drawing_host_arena::request_drawing_mutation_arena_pool() {
            semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::Fault(error) => Some(error),
            semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::Ready | semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::NotReady | semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::Contended => None,
        };
        Self { arena_boot_fault }
    }
}

impl ArtifactEditor for DrawingPlayApp {
    fn completion_retirement_birth_bytes(value:&semio_framework_plugin::ArtifactToolCompletionValue<semio_framework_plugin::EditorApp<Self>>)->Option<usize>{completion::birth(value)}
    fn admit_completion_retirement(value:&mut Option<semio_framework_plugin::ArtifactToolCompletionValue<semio_framework_plugin::EditorApp<Self>>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{completion::admit(value,grant)}
    /// 📢️ The localized notices of the retained gesture owner's refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        drawing_fault_notices()
    }

    type Snapshot = DrawingSnapshot;
    type Mutation = DrawingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = DrawingPresence;
    type PresenceMutation = DrawingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = DrawingCommand;

    const DIALECT: semio_framework_artifact_reference::Dialect = semio_s_artifact_draw_drawing_model::DRAWING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;

    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        protocol::InteractionTopology { domains: [(DRAWING_INTERACTION_DOMAIN.into(), semio_s_artifact_draw_drawing_editor_interaction::drawing_interaction_topology(doc.snapshot)),(DRAWING_POINT_DOMAIN.into(),semio_s_artifact_draw_drawing_editor_interaction::drawing_point_topology(doc.snapshot))].into() }

})())
}

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields;
    /// without it every live envelope load faults `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("drawing.child-projection"), error.to_string()))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_draw_drawing_host_arena::drawing_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_draw_drawing_host::owned::drawing_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    /// 👥️ Returned local reads retain the actual Draw presence fields and original Arc backing.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_draw_drawing_editor_interaction::presence::DrawingPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_draw_drawing_editor_interaction::presence::DrawingPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default Drawing presence is the exact empty terminal")))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(DrawingArtifactStorePreparationFactory::default()))
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        canvas_window::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        canvas_window::transient::register(registry)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        DrawingGestureProofs::bounded_first_step_tool_proofs().into_iter().chain(DrawingBoundedProofs::bounded_first_step_tool_proofs()).chain(DrawingPathProofs::bounded_first_step_tool_proofs()).chain(DrawingImageProofs::bounded_first_step_tool_proofs()).chain(DrawingExportProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn mounted_job_prepare_snapshot_read(operation:semio_framework_plugin::AppRenderOperationContext,snapshot:&Self::Snapshot)->bool{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::prepare(operation,snapshot)}
    fn mounted_job_maintenance_demands(instance:u32,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::maintenance_demands(instance,body)}
    fn mounted_job_maintenance_step(instance:u32,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::maintenance(instance,grant).map_err(semio_framework_plugin::PluginLifecycleFailure::from)}
    fn mounted_job_close_demands(instance:u32,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::close_demands(instance,body)}
    fn mounted_job_close_step(instance:u32,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::close(instance,grant).map_err(semio_framework_plugin::PluginLifecycleFailure::from)}
    fn mounted_jobs_terminal_is_empty(instance:u32)->bool{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::terminal_is_empty(instance)}
    fn pending_effects(_owner:&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,_view:Option<&semio_framework_plugin::ViewModel>)->Vec<semio_framework::kernel::Effect>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::reconcile(doc)}

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(DrawingInstanceOperationOwner::new())
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DrawingGestureOperationJobFactory::new(&controller))?;
        registry.register(DrawingBoundedCommandJobFactory::new(&controller))?;
        registry.register(DrawingPathCommandJobFactory::new(&controller))?;
        registry.register(DrawingImageCommandJobFactory::new(&controller))?;
        registry.register(DrawingExportCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        let tool_id = original.tool_id.as_str();
        if DRAWING_EXPORT_TOOL_IDS.contains(&tool_id) {return export_job::build(request, output, cx);}
        if DRAWING_IMAGE_TOOL_IDS.contains(&tool_id) {return image_job::build(request, output, cx);}
        if DRAWING_PATH_TOOL_IDS.contains(&tool_id) {return path_job::build(request, output, cx);}
        if DRAWING_BOUNDED_TOOL_IDS.contains(&tool_id) {return drawing_bounded_tool_job(request, output, cx);}
        if DRAWING_GESTURE_TOOL_IDS.contains(&tool_id) {return drawing_gesture_tool_job(request, output, cx);}
        Ok(false)
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_draw_drawing_editor_interaction::config::app_schema_descriptor())
    }

    fn reference_birth()->Option<fn()->DrawingSnapshot>{Some(||{
        semio_s_artifact_draw_drawing_model::default_drawing_document("empty", None)
    })}

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(drawing_io())
    }

    /// 🎞️ `vector:out` (see `drawing_vector_media`) plus the inherited `document:out` default (the pack
    /// of `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, DrawingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "vector:out" => drawing_vector_media(doc.snapshot),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn clipboard_media_type()->Option<MediaType>{Some(semio_s_artifact_draw_drawing_editor_interaction::clipboard::media_type())}

    fn copy_fragment(doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,interaction:&InteractionView<'_>)->Result<semio_framework_plugin::kernel::ClipboardFragment,semio_framework_plugin::kernel::ClipboardError>{
        semio_s_artifact_draw_drawing_editor_interaction::clipboard::copy(doc.snapshot,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids)
    }

    fn cut_operations(doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,interaction:&InteractionView<'_>)->Vec<DrawingMutation>{
        semio_s_artifact_draw_drawing_editor_interaction::clipboard::cut(doc.snapshot,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids).unwrap_or_default()
    }

    fn build_reserved_tool_job(request:semio_framework_plugin::ArtifactReservedToolJobRequest<semio_framework_plugin::EditorApp<Self>>)->Result<Option<semio_framework_plugin::ArtifactReservedToolJob>,Fault>{
        if !DRAWING_CLIPBOARD_RESERVED_IDS.contains(&request.tool_id.as_str()){return Ok(None);}
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(crate::clipboard_job::DrawingClipboardJob::new(request)?)))
    }

    /// 🏷️ `app_commands!`'s generated `command_id()`.
    fn command_id(command: &DrawingCommand) -> &'static str {
        command.command_id()
    }

    /// 🌉️ Shell `{action, args}` → `DrawingCommand` (see `args_bridge`); the trait default refuses
    /// every app action, which left the whole ribbon/palette dead in the React shell.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &DrawingCommand,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DrawingMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if DRAWING_GESTURE_TOOL_IDS.contains(&command.command_id()) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.retained-route"), "Drawing gesture commands are reachable only through their exact retained factory owner"));
        }
        let mut session = DrawingSession::new(view_state.map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility), doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str()));
        session.interaction.ids = interaction.selection(DRAWING_INTERACTION_DOMAIN).ids.clone();
        session.interaction.points = interaction.selection(DRAWING_POINT_DOMAIN).ids.clone();
        session.window_config = canvas_window::config::current(cfg);
        command.dispatch(doc, cfg, &mut session)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_drawing_body(body_key, doc, &canvas_window::config::current(cfg), &DrawingSession::default().preview(), &[], &[], view_state)
    }

    fn render_with_instance_operation_owner(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview = match doc.render_operation() {
            Some(operation) => owner
                .with_mut::<DrawingInstanceOperationOwner, _>(|owner| Ok(owner.preview_projection(semio_s_artifact_draw_drawing_editor_interaction::geometry_session::identity(operation), drawing_active_utility(view_state))))
                .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner", error.message))?
                .unwrap_or_default(),
            None => DrawingGesturePreview::default(),
        };
        render_drawing_body(body_key, doc, &canvas_window::config::current(cfg), &preview, &[], &[], view_state)
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key == DRAWING_PLAY_BODY_PROPERTIES {
            return properties_panel::render(doc.snapshot, &interaction.selection(DRAWING_INTERACTION_DOMAIN).ids, semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state), &semio_framework_plugin::TreeWindows::for_body(view_state, body_key)).map(semio_framework_plugin::built_to_component_tree);
        }
        let preview=match doc.render_operation() {
            Some(operation)=>owner.with_mut::<DrawingInstanceOperationOwner,_>(|owner|Ok(owner.preview_projection(semio_s_artifact_draw_drawing_editor_interaction::geometry_session::identity(operation),drawing_active_utility(view_state))))
                .map_err(|error|semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner",error.message))?.unwrap_or_default(),
            None=>DrawingGesturePreview::default(),
        };
        render_drawing_body(body_key,doc,&canvas_window::config::current(cfg),&preview,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids,&interaction.selection(DRAWING_POINT_DOMAIN).ids,view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        semio_s_artifact_draw_drawing_editor_interaction::status::engagements(doc.snapshot.layers.len(),0,String::new(),view_state)
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, DrawingSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        semio_s_artifact_draw_drawing_editor_interaction::status::engagements(doc.snapshot.layers.len(),interaction.selection(DRAWING_INTERACTION_DOMAIN).ids.len(),canvas_window::transient::current(transient).engagement_input,view_state)
    }
}
//#endregion 🔖️DrawingPlayApp
