//! ✏️ glTF editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `GltfAnyEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::gltf::modes::edit;
use crate::editor::gltf::modes::edit::windows::main;
use crate::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::GltfMutation;
use crate::standards::v2_0::subsets::any::schema::mutations as kinds;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_2d::compute::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Dialect
pub const GLTF_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };
pub const GLTF_ANY_DOCUMENT_SCHEMA: &str = "stdio.gltf";
//#endregion 🔖️Dialect

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum GltfAnyEditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for GltfAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = GLTF_ANY_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "GltfAnyEditCommand", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "GltfAnyEditCommand", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };

const GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const GLTF_ANY_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.gltf.tool-command.v1";
const GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;

fn gltfAnyEditor_example_snapshot(example_id: &str) -> GltfSnapshot {
    if example_id == crate::examples::demo::ID {
        <GltfSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        GltfSnapshot::default()
    }
}

fn gltfAnyEditor_command_id(command: &GltfAnyEditCommand) -> &'static str {
    if let GltfAnyEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command {
        GltfAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}

fn gltfAnyEditor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<GltfAnyEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| GltfAnyEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(GltfAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}

fn gltfAnyEditor_retained_extent(command: &GltfAnyEditCommand, _snapshot: &GltfSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, GltfAnyEditCommand::SetActiveExample { .. }).then_some(1)
}

fn gltfAnyEditor_retained_reduce(
    command: &GltfAnyEditCommand,
    _snapshot: &GltfSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<GltfAnyEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<GltfMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        GltfAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
            effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&gltfAnyEditor_example_snapshot(example_id), GLTF_ANY_DOCUMENT_SCHEMA)],
            ..Default::default()
        }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}

struct GltfAnyEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}

impl GltfAnyEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for GltfAnyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<GltfAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<GltfAnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for GltfAnyEditorExampleFactory {
    type Owner = EditorApp<GltfAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GLTF_ANY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct GltfAnyEditor;

impl ArtifactEditor for GltfAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = GltfSnapshot;
    type Mutation = GltfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = GltfAnyEditCommand;

    const DIALECT: Dialect = GLTF_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GLTF_ANY_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<GltfAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🦀️.rs",
        controller: "s.stdio.gltf@2.0/*#editor",
        artifact_schema: "stdio.gltf",
        factory: "GltfAnyEditorExampleFactory",
        factory_type: GltfAnyEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(GltfAnyEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if !GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if gltfAnyEditor_command_id(&request.command) != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-example-tool-mismatch"));
        }
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            gltfAnyEditor_command_id,
            GLTF_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, gltfAnyEditor_retained_reduce, gltfAnyEditor_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn command_id(command: &Self::Command) -> &'static str { gltfAnyEditor_command_id(command) }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> { gltfAnyEditor_command_from_action(action, args) }

    fn initial_snapshot() -> GltfSnapshot {
        GltfSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            GltfAnyEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            GltfAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&gltfAnyEditor_example_snapshot(example_id), GLTF_ANY_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.gltf@2.0/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for GltfAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { GltfAnyEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let expected = <Self as editing::SnapshotEditingEditor>::snapshot_edit_expected(event, snapshot)?;
        Ok(Emit { artifact_mutations: concrete_snapshot_edit(snapshot, &expected)?, ..Default::default() })
    }
}


//#region 🔖️ConcreteEdit
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unsupported_edit(what: &str) -> Fault {
    Fault::from(format!("snapshot-edit.unsupported: the edit changes {what}, which no glTF mutation kind expresses"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn same_members<T: Ord + Clone>(left: &[T], right: &[T]) -> bool {
    let (mut left, mut right) = (left.to_vec(), right.to_vec());
    left.sort();
    right.sort();
    left == right
}

/// 🧬️ The concrete glTF mutation kinds that carry `base` onto `expected`: field-level setters, binds and reorders only. A
/// structural difference (entries gained or lost, a reference list that changed members) is refused rather than approximated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn concrete_snapshot_edit(base: &GltfSnapshot, expected: &GltfSnapshot) -> Result<Vec<GltfMutation>, Fault> {
    let (before, after) = (&base.document, &expected.document);
    if base.schema != expected.schema || base.buffers != expected.buffers || base.source_form != expected.source_form {
        return Err(unsupported_edit("the container"));
    }
    let lengths = |d: &crate::standards::v2_0::subsets::any::schema::snapshot::GltfDocument| [d.scenes.len(), d.nodes.len(), d.meshes.len(), d.accessors.len(), d.buffer_views.len(), d.buffers.len(), d.materials.len(), d.textures.len(), d.images.len(), d.samplers.len(), d.skins.len(), d.animations.len(), d.cameras.len()];
    if lengths(before) != lengths(after) || before.accessors != after.accessors || before.buffer_views != after.buffer_views || before.buffers != after.buffers || before.textures != after.textures || before.images != after.images || before.samplers != after.samplers || before.skins != after.skins || before.animations != after.animations || before.cameras != after.cameras {
        return Err(unsupported_edit("the document collections"));
    }
    let mut rows = Vec::new();
    if before.asset.version != after.asset.version {
        rows.push(kinds::change_asset_version::mutation(kinds::change_asset_version::GltfChangeAssetVersionPayload { version: after.asset.version.clone() }));
    }
    if (before.asset.generator.as_ref(), before.asset.copyright.as_ref(), before.asset.min_version.as_ref()) != (after.asset.generator.as_ref(), after.asset.copyright.as_ref(), after.asset.min_version.as_ref()) {
        rows.push(kinds::change_asset_descriptive_metadata::mutation(kinds::change_asset_descriptive_metadata::GltfChangeAssetDescriptiveMetadataPayload { generator: after.asset.generator.clone(), copyright: after.asset.copyright.clone(), min_version: after.asset.min_version.clone() }));
    }
    if before.asset.extensions != after.asset.extensions {
        rows.push(kinds::change_asset_extension_data::mutation(kinds::change_asset_extension_data::GltfChangeAssetExtensionDataPayload { data: after.asset.extensions.clone() }));
    }
    if before.asset.extras != after.asset.extras {
        rows.push(kinds::change_asset_extra_data::mutation(kinds::change_asset_extra_data::GltfChangeAssetExtraDataPayload { data: after.asset.extras.clone() }));
    }
    if before.extensions != after.extensions {
        rows.push(kinds::change_document_extension_data::mutation(kinds::change_document_extension_data::GltfChangeDocumentExtensionDataPayload { data: after.extensions.clone() }));
    }
    if before.extras != after.extras {
        rows.push(kinds::change_document_extra_data::mutation(kinds::change_document_extra_data::GltfChangeDocumentExtraDataPayload { data: after.extras.clone() }));
    }
    if before.scene != after.scene {
        rows.push(match after.scene {
            Some(scene) => kinds::bind_default_scene::mutation(kinds::bind_default_scene::GltfBindDefaultScenePayload { scene }),
            None => kinds::unbind_default_scene::mutation(kinds::unbind_default_scene::GltfUnbindDefaultScenePayload {}),
        });
    }
    for (name, left, right) in [("extensionsUsed", &before.extensions_used, &after.extensions_used), ("extensionsRequired", &before.extensions_required, &after.extensions_required)] {
        if left != right {
            if !same_members(left, right) {
                return Err(unsupported_edit(name));
            }
            rows.push(if name == "extensionsUsed" {
                kinds::reorder_used_extensions::mutation(kinds::reorder_used_extensions::GltfReorderUsedExtensionsPayload { order: right.clone() })
            } else {
                kinds::reorder_required_extensions::mutation(kinds::reorder_required_extensions::GltfReorderRequiredExtensionsPayload { order: right.clone() })
            });
        }
    }
    for (index, (old, new)) in before.scenes.iter().zip(&after.scenes).enumerate() {
        let mut rest = old.clone();
        rest.name = new.name.clone();
        rest.extras = new.extras.clone();
        rest.extensions = new.extensions.clone();
        rest.nodes = new.nodes.clone();
        if &rest != new || (old.nodes != new.nodes && !same_members(&old.nodes, &new.nodes)) {
            return Err(unsupported_edit("a scene"));
        }
        if old.name != new.name {
            rows.push(kinds::change_scene_name::mutation(kinds::change_scene_name::GltfChangeSceneNamePayload { scene: index, value: new.name.clone() }));
        }
        if old.extras != new.extras {
            rows.push(kinds::change_scene_extra_data::mutation(kinds::change_scene_extra_data::GltfChangeSceneExtraDataPayload { scene: index, data: kinds::change_scene_extra_data::presence(&new.extras) }));
        }
        if old.extensions != new.extensions {
            rows.push(kinds::change_scene_extension_data::mutation(kinds::change_scene_extension_data::GltfChangeSceneExtensionDataPayload { scene: index, data: kinds::change_scene_extension_data::presence(&new.extensions) }));
        }
        if old.nodes != new.nodes {
            rows.push(kinds::reorder_scene_root_nodes::mutation(kinds::reorder_scene_root_nodes::GltfReorderSceneRootNodesPayload { scene: index, order: new.nodes.clone() }));
        }
    }
    for (index, (old, new)) in before.nodes.iter().zip(&after.nodes).enumerate() {
        let mut rest = old.clone();
        rest.name = new.name.clone();
        rest.extras = new.extras.clone();
        rest.extensions = new.extensions.clone();
        rest.weights = new.weights.clone();
        (rest.mesh, rest.camera, rest.skin) = (new.mesh, new.camera, new.skin);
        (rest.matrix, rest.translation, rest.rotation, rest.scale) = (new.matrix, new.translation, new.rotation, new.scale);
        rest.children = new.children.clone();
        if &rest != new || (old.children != new.children && !same_members(&old.children, &new.children)) {
            return Err(unsupported_edit("a node"));
        }
        if old.mesh != new.mesh {
            rows.push(match new.mesh {
                Some(mesh) => kinds::bind_node_mesh::mutation(kinds::bind_node_mesh::GltfBindNodeMeshPayload { node: index, mesh }),
                None => kinds::unbind_node_mesh::mutation(kinds::unbind_node_mesh::GltfUnbindNodeMeshPayload { node: index }),
            });
        }
        if old.camera != new.camera {
            rows.push(match new.camera {
                Some(camera) => kinds::bind_node_camera::mutation(kinds::bind_node_camera::GltfBindNodeCameraPayload { node: index, camera }),
                None => kinds::unbind_node_camera::mutation(kinds::unbind_node_camera::GltfUnbindNodeCameraPayload { node: index }),
            });
        }
        if old.skin != new.skin {
            rows.push(match new.skin {
                Some(skin) => kinds::bind_node_skin::mutation(kinds::bind_node_skin::GltfBindNodeSkinPayload { node: index, skin }),
                None => kinds::unbind_node_skin::mutation(kinds::unbind_node_skin::GltfUnbindNodeSkinPayload { node: index }),
            });
        }
        if old.weights != new.weights {
            rows.push(kinds::change_node_morph_weights::mutation(kinds::change_node_morph_weights::GltfChangeNodeMorphWeightsPayload { node: index, weights: new.weights.clone() }));
        }
        if (old.matrix, old.translation, old.rotation, old.scale) != (new.matrix, new.translation, new.rotation, new.scale) {
            let transform = match new.matrix {
                Some(_) if new.translation.is_some() || new.rotation.is_some() || new.scale.is_some() => return Err(unsupported_edit("a node transform holding both a matrix and translation/rotation/scale")),
                Some(matrix) => kinds::change_node_transform::GltfNodeTransform::Matrix { matrix },
                None => kinds::change_node_transform::GltfNodeTransform::Trs { translation: new.translation, rotation: new.rotation, scale: new.scale },
            };
            rows.push(kinds::change_node_transform::mutation(kinds::change_node_transform::GltfTransformNodePayload { node: index, transform }));
        }
        if old.name != new.name {
            rows.push(kinds::change_node_name::mutation(kinds::change_node_name::GltfChangeNodeNamePayload { node: u32::try_from(index).map_err(|_| unsupported_edit("a node index past u32"))?, value: new.name.clone() }));
        }
        if old.extras != new.extras {
            rows.push(kinds::change_node_extra_data::mutation(kinds::change_node_extra_data::GltfChangeNodeExtraDataPayload { node: index, data: kinds::change_node_extra_data::presence(&new.extras) }));
        }
        if old.extensions != new.extensions {
            rows.push(kinds::change_node_extension_data::mutation(kinds::change_node_extension_data::GltfChangeNodeExtensionDataPayload { node: index, data: kinds::change_node_extension_data::presence(&new.extensions) }));
        }
        if old.children != new.children {
            rows.push(kinds::reorder_node_children::mutation(kinds::reorder_node_children::GltfReorderNodeChildrenPayload { parent: index, order: new.children.clone() }));
        }
    }
    for (index, (old, new)) in before.meshes.iter().zip(&after.meshes).enumerate() {
        if old.primitives.len() != new.primitives.len() {
            return Err(unsupported_edit("a mesh primitive list"));
        }
        let mut rest = old.clone();
        rest.name = new.name.clone();
        rest.extras = new.extras.clone();
        rest.extensions = new.extensions.clone();
        rest.weights = new.weights.clone();
        for (slot, replacement) in rest.primitives.iter_mut().zip(&new.primitives) {
            (slot.mode, slot.material, slot.indices) = (replacement.mode, replacement.material, replacement.indices);
            slot.extras = replacement.extras.clone();
            slot.extensions = replacement.extensions.clone();
        }
        if &rest != new {
            return Err(unsupported_edit("a mesh"));
        }
        if old.name != new.name {
            rows.push(kinds::change_mesh_name::mutation(kinds::change_mesh_name::GltfChangeMeshNamePayload { mesh: index, value: new.name.clone() }));
        }
        if old.extras != new.extras {
            rows.push(kinds::change_mesh_extra_data::mutation(kinds::change_mesh_extra_data::GltfChangeMeshExtraDataPayload { mesh: index, data: kinds::change_mesh_extra_data::presence(&new.extras) }));
        }
        if old.extensions != new.extensions {
            rows.push(kinds::change_mesh_extension_data::mutation(kinds::change_mesh_extension_data::GltfChangeMeshExtensionDataPayload { mesh: index, data: kinds::change_mesh_extension_data::presence(&new.extensions) }));
        }
        if old.weights != new.weights {
            rows.push(kinds::change_mesh_morph_weights::mutation(kinds::change_mesh_morph_weights::GltfChangeMeshMorphWeightsPayload { mesh: index, weights: new.weights.clone() }));
        }
        for (primitive, (old, new)) in old.primitives.iter().zip(&new.primitives).enumerate() {
            if old.mode != new.mode {
                rows.push(kinds::change_primitive_topology_mode::mutation(kinds::change_primitive_topology_mode::GltfChangePrimitiveTopologyModePayload { mesh: index, primitive, mode: new.mode }));
            }
            if old.material != new.material {
                rows.push(match new.material {
                    Some(material) => kinds::bind_primitive_material::mutation(kinds::bind_primitive_material::GltfBindPrimitiveMaterialPayload { mesh: index, primitive, material }),
                    None => kinds::unbind_primitive_material::mutation(kinds::unbind_primitive_material::GltfUnbindPrimitiveMaterialPayload { mesh: index, primitive }),
                });
            }
            if old.indices != new.indices {
                rows.push(match new.indices {
                    Some(accessor) => kinds::bind_primitive_indices::mutation(kinds::bind_primitive_indices::GltfBindPrimitiveIndicesPayload { mesh: index, primitive, accessor }),
                    None => kinds::unbind_primitive_indices::mutation(kinds::unbind_primitive_indices::GltfUnbindPrimitiveIndicesPayload { mesh: index, primitive }),
                });
            }
            if old.extras != new.extras {
                rows.push(kinds::change_primitive_extra_data::mutation(kinds::change_primitive_extra_data::GltfChangePrimitiveExtraDataPayload { mesh: index, primitive, data: kinds::change_primitive_extra_data::presence(&new.extras) }));
            }
            if old.extensions != new.extensions {
                rows.push(kinds::change_primitive_extension_data::mutation(kinds::change_primitive_extension_data::GltfChangePrimitiveExtensionDataPayload { mesh: index, primitive, data: kinds::change_primitive_extension_data::presence(&new.extensions) }));
            }
        }
    }
    for (index, (old, new)) in before.materials.iter().zip(&after.materials).enumerate() {
        let mut rest = old.clone();
        (rest.alpha_mode, rest.double_sided) = (new.alpha_mode, new.double_sided);
        if &rest != new {
            return Err(unsupported_edit("a material"));
        }
        if old.alpha_mode != new.alpha_mode {
            rows.push(kinds::change_material_alpha_mode::mutation(kinds::change_material_alpha_mode::GltfChangeMaterialAlphaModePayload { material: index, alpha_mode: new.alpha_mode }));
        }
        if old.double_sided != new.double_sided {
            rows.push(kinds::change_material_double_sided::mutation(kinds::change_material_double_sided::GltfChangeMaterialDoubleSidedPayload { material: index, double_sided: new.double_sided }));
        }
    }
    Ok(rows)
}
//#endregion 🔖️ConcreteEdit

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_gltf_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(GLTF_ANY_DIALECT).document(["stdio", "gltf"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::GLTF_ANY_EDIT_MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        ;
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
