//! 🧩️ Play app: `Block3dPlayApp`, the `ArtifactEditor` the framework mounts, with its dispatch, render and media bodies.

use super::*;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Block3dPlayApp
/// 🧪️ B1: unit struct — every former `RefCell` field now lives in `crate::editor::block3d::config::
/// Block3dConfig`, written through `Block3dConfigMutation`s.
#[derive(Default)]
pub struct Block3dPlayApp;

impl ArtifactEditor for Block3dPlayApp {
    type Snapshot = Block3dSnapshot;
    type Mutation = Block3dMutation;
    type Config = Block3dConfig;
    type ConfigMutation = Block3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_s_artifact_block_3d_editor_presence::Block3dPresence;
    type PresenceMutation = semio_s_artifact_block_3d_editor_presence::Block3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Block3dCommand;

    const DIALECT: Dialect = BLOCK3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_3D_SCHEMA;

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<crate::store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<crate::store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<crate::store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn crate::store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn crate::store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<crate::store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(
            semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default())
                .expect("default Block3d presence is the exact empty terminal"),
        ))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<crate::store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Block3dArtifactStorePreparationFactory))
    }

    /// 📬️ Required by the Config publication lane: 11 of the 22 retained tools are config-only, and
    /// `VcsArtifactApp` rejects any tool whose declared lane has no one-item preparation factory with
    /// `interactive-job.publication-authority-missing`.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn crate::store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Block3dConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Block3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.block.block3d@1/*#editor",
        artifact_schema: "block.3d",
        factory: "Block3dRetainedCommandJobFactory",
        factory_type: Block3dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "patchObjectKind",
            "addRepresentation",
            "removeRepresentation",
            "addVortexKind",
            "removeVortexKind",
            "addVortex",
            "removeVortex",
            "setActiveExample",
            "edit",
            "setActiveRepresentation",
            "setWindowRepresentations",
            "toggleWindowRepresentation",
            "setWindowArrangement",
            "setWindowSpacing",
            "setBrushVortexKind",
            "setBrushRadius",
            "setBrushFlip",
            "worldSurfaceHover",
            "worldSurfaceLeave",
            "worldSurfacePlace",
            "setCamera",
            "patchRepresentation"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(crate::Block3dRetainedCommandJobFactory::new(&controller_id))
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<Block3dWorldWindowTransientOwner>()
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BLOCK3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || block3d_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "block3d-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = match tool_id {
            "worldSurfaceHover" | "worldSurfaceLeave" | "worldSurfacePlace" => Box::new(crate::Block3dWindowPreviewWork::new(tool_id)),
            _ => Box::new(BoundedArtifactCommandWork::new(tool_id, block3d_retained_reduce, block3d_retained_extent)),
        };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, snapshot_read:request.snapshot_read, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
            Block3dCommand::command_id,
            BLOCK3D_RETAINED_RAW_BYTES,
            BLOCK3D_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🚀️ Boots on the `hexagonal-cut-concrete-forest-left` fixture instead of the empty document —
    /// the `World3d` window renders `representations[].mesh_url`, so an empty boot document painted an
    /// empty scene until a client dispatched `setActiveExample`. See `dsl::block3d_boot_snapshot`.
    fn initial_snapshot() -> Block3dSnapshot {
        semio_s_artifact_block_3d_io::text::snapshot::block3d_boot_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(block3d_io())
    }

    fn command_id(command: &Block3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block3dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        let args = args.as_ref();
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "patchObjectKind" => Ok(Block3dCommand::PatchObjectKind(patch_object_kind::PatchObjectKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            "addRepresentation" => Ok(Block3dCommand::AddRepresentation(add_representation::AddRepresentation {})),
            "removeRepresentation" => Ok(Block3dCommand::RemoveRepresentation(remove_representation::RemoveRepresentation { id: str_field("id").unwrap_or_default() })),
            "addVortexKind" => Ok(Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {})),
            "removeVortexKind" => Ok(Block3dCommand::RemoveVortexKind(remove_vortex_kind::RemoveVortexKind { id: str_field("id").unwrap_or_default() })),
            "addVortex" => Ok(Block3dCommand::AddVortex(add_vortex::AddVortex {})),
            "removeVortex" => Ok(Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: str_field("id").unwrap_or_default() })),
            "setActiveExample" => Ok(Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() })),
            "edit" => Ok(Block3dCommand::Edit(edit::Edit { text: str_field("text").unwrap_or_default() })),
            "setActiveRepresentation" => Ok(Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: str_field("representationId").or_else(|| str_field("representation_id")) })),
            "setWindowRepresentations" => {
                let rep = str_field("value").or_else(|| str_field("representationId"));
                let representation_ids = rep.filter(|id| !id.is_empty()).map(|id| vec![id]).unwrap_or_default();
                Ok(Block3dCommand::SetWindowRepresentations(set_window_representations::SetWindowRepresentations { window_id: window_id_from_args(args), representation_ids }))
            }
            "toggleWindowRepresentation" => Ok(Block3dCommand::ToggleWindowRepresentation(toggle_window_representation::ToggleWindowRepresentation {
                window_id: window_id_from_args(args),
                representation_id: str_field("representationId").unwrap_or_default(),
                visible: args.and_then(|value| value.get("visible")).and_then(Value::as_bool).unwrap_or(true),
            })),
            "setWindowArrangement" => Ok(Block3dCommand::SetWindowArrangement(set_window_arrangement::SetWindowArrangement { window_id: window_id_from_args(args), arrangement: str_field("value").unwrap_or_else(|| "overlap".into()) })),
            "setWindowSpacing" => Ok(Block3dCommand::SetWindowSpacing(set_window_spacing::SetWindowSpacing { window_id: window_id_from_args(args), spacing: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(8.0) })),
            "setBrushVortexKind" => Ok(Block3dCommand::SetBrushVortexKind(set_brush_vortex_kind::SetBrushVortexKind { vortex_kind_id: str_field("value").or_else(|| str_field("vortexKindId")) })),
            "setBrushRadius" => Ok(Block3dCommand::SetBrushRadius(set_brush_radius::SetBrushRadius { radius: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(0.3) })),
            "setBrushFlip" => Ok(Block3dCommand::SetBrushFlip(set_brush_flip::SetBrushFlip { flip: args.and_then(|value| value.get("flip")).and_then(Value::as_bool).unwrap_or(false) })),
            "worldSurfaceHover" => Ok(Block3dCommand::HoverSurface(hover_surface::HoverSurface {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            })),
            "worldSurfaceLeave" => Ok(Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})),
            "worldSurfacePlace" => Ok(Block3dCommand::PlaceVortex(place_vortex::PlaceVortex {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            })),
            "patchRepresentation" => {
                Ok(Block3dCommand::PatchRepresentation(patch_representation::PatchRepresentation { id: str_field("id").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() }))
            }
            // 🩹️ Forward-fix, not a preserved behavior: pre-migration `command_from_action` had NO arm
            // for the manifest-declared `setCamera` view action at all (fell through to the reserved-
            // action error) — a real gap, the `Block3dCommand::SetCamera` variant was only reachable via
            // direct `dispatch_typed`/binary `OpBinary`. `assert_declared_actions_bridge_to_commands`
            // requires every declared action to bridge, so this parses the camera pose from
            // `{position,target,zoom}` args the same shape `BlockCamera3d` serializes to.
            "setCamera" => Ok(Block3dCommand::SetCamera(set_camera::SetCamera {
                camera: BlockCamera3d {
                    position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                    target: f64_vec3_field(args, "target").unwrap_or([0.0, 0.0, 0.0]),
                    zoom: args.and_then(|value| value.get("zoom")).and_then(Value::as_f64).unwrap_or(1.0),
                },
            })),
            other => Err(Fault::new(FaultOrigin::App, FaultCode::new("block3d.unhandled-action"), format!("action '{other}' is not a framework-reserved action — surface actions are dispatched exclusively through the typed command channel"))),
        }
    }

    fn handle(
        command: &Block3dCommand,
        doc: &ArtifactView<'_, Block3dSnapshot>,
        cfg: &ConfigView<'_, Block3dConfig>,
        _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Block3dMutation, Block3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain's
    /// `HierarchyProvider::Topology` — representations (`surface` granularity) and rim-vortex
    /// templates (`vortex` granularity) as a flat forest (no real structural parent between them, but
    /// declaring `Topology` rather than `Flat` lets `validate_state` prune stale selection/hover ids
    /// the moment a representation or vortex is removed — see `HierarchyProvider::Flat`'s doc comment
    /// on why `Flat` domains are never auto-pruned).
    fn interaction_topology(doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let mut ordered: Vec<TopologyNode> = Vec::new();
        for representation in &doc.snapshot.representations {
            ordered.push(TopologyNode { id: format!("surface:{}", representation.id), granularity: BLOCK3D_GRANULARITY_SURFACE.into(), parent: None });
        }
        for vortex in &doc.snapshot.vortices {
            ordered.push(TopologyNode { id: format!("vortex:{}", vortex.id), granularity: BLOCK3D_GRANULARITY_VORTEX.into(), parent: None });
        }
        let mut domains = BTreeMap::new();
        domains.insert(BLOCK3D_INTERACTION_VORTEX.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    fn window_measures(doc: &ArtifactView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        let labels = block3d_labels(view_state);
        let mut measures = HashMap::new();
        measures.insert(BLOCK3D_DEFAULT_WINDOW_ID.into(), world::window_measures(doc.snapshot, cfg.snapshot, BLOCK3D_DEFAULT_WINDOW_ID, labels));
        measures
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = block3d_labels(view_state);
        let active_representation_id = cfg.snapshot.active_representation_id.as_deref();
        let node = match body_key {
            world::BLOCK3D_BODY_WORLD => world::render(
                doc.snapshot,
                cfg.snapshot,
                block3d_render_window_id(view_state),
                view_state.active_utility_id.as_deref().unwrap_or(BLOCK3D_UTILITY_SELECT),
                None,
            )?,
            document_panel::BLOCK3D_BODY_ARTIFACT => document_panel::render(doc.snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, document_panel::BLOCK3D_BODY_ARTIFACT))?,
            inspection_panel::BLOCK3D_BODY_INSPECTOR => inspection_panel::render(doc.snapshot, active_representation_id, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::BLOCK3D_BODY_INSPECTOR))?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block3d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Block3dSnapshot>,
        cfg: &ConfigView<'_, Block3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key == world::BLOCK3D_BODY_WORLD {
            let preview = transient.window::<Block3dWorldWindowTransientOwner>().and_then(|window| window.brush_preview.as_ref());
            let root = world::render(
                doc.snapshot,
                cfg.snapshot,
                block3d_render_window_id(view_state),
                view_state.active_utility_id.as_deref().unwrap_or(BLOCK3D_UTILITY_SELECT),
                preview,
            )?;
            return Ok(semio_framework_plugin::built_to_component_tree(root));
        }
        Self::render(body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    /// 🌉️ The flagship seam: `puzzle3d_catalog_fragment`'s first real caller. Wraps the block-3d
    /// document's puzzle3d-shaped catalog fragment as a `kit.catalog`-schema `Media` value for the
    /// `"catalog:out"` port declared in `block3d_io`. `wanted_tags` should come from `cfg.wanted_tags`
    /// but `ArtifactEditor::export_media`'s landed signature doesn't thread `ConfigView` through yet —
    /// see `Block3dConfig::wanted_tags`'s doc — so this always resolves the active representation with
    /// an empty (all-tags) filter until that lands. Falls through to the default whole-document pack
    /// export for every other port (`"artifact:out"`).
    fn export_media(port: &str, doc: &ArtifactView<'_, Block3dSnapshot>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `ArtifactEditor::export_media`'s default `"artifact:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this surface.
            if port != "artifact:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type }, |io| io.artifact_media_type);
            let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: crate::store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = semio_s_artifact_block_3d_io_write::puzzle_catalog::puzzle3d_catalog_fragment(doc.snapshot, &[]);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block3dPlayApp
