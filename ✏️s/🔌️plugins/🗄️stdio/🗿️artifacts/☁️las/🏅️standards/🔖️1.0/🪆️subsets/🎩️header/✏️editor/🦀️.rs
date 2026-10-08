//! ✏️ LAS editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `LasAnyEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::las::modes::edit;
use crate::editor::las::modes::edit::windows::main;
use crate::standards::v1_0::subsets::any::schema::mutations::{net_mutations, LasMutation};

use crate::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot;
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
pub const LAS_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };
pub const LAS_ANY_DOCUMENT_SCHEMA: &str = "stdio.las";
//#endregion 🔖️Dialect

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum LasAnyEditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for LasAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = LAS_ANY_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "LasAnyEditCommand", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "LasAnyEditCommand", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };

const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const LAS_ANY_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.las.tool-command.v1";
const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;

fn lasAnyEditor_example_snapshot(example_id: &str) -> LasSnapshot {
    if example_id == crate::examples::demo::ID {
        <LasSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        LasSnapshot::default()
    }
}

fn lasAnyEditor_command_id(command: &LasAnyEditCommand) -> &'static str {
    if let LasAnyEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command {
        LasAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}

fn lasAnyEditor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<LasAnyEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| LasAnyEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(LasAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}

fn lasAnyEditor_retained_extent(command: &LasAnyEditCommand, _snapshot: &LasSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, LasAnyEditCommand::SetActiveExample { .. }).then_some(1)
}

fn lasAnyEditor_retained_reduce(
    command: &LasAnyEditCommand,
    _snapshot: &LasSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<LasAnyEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<LasMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        LasAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
            effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&lasAnyEditor_example_snapshot(example_id), LAS_ANY_DOCUMENT_SCHEMA)],
            ..Default::default()
        }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}

struct LasAnyEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}

impl LasAnyEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for LasAnyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<LasAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<LasAnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for LasAnyEditorExampleFactory {
    type Owner = EditorApp<LasAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = LAS_ANY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct LasAnyEditor;

impl ArtifactEditor for LasAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = LasSnapshot;
    type Mutation = LasMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = LasAnyEditCommand;

    const DIALECT: Dialect = LAS_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LAS_ANY_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<LasAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/✏️editor/🦀️.rs",
        controller: "s.stdio.las@1.0/*#editor",
        artifact_schema: "stdio.las",
        factory: "LasAnyEditorExampleFactory",
        factory_type: LasAnyEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(LasAnyEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if !LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if lasAnyEditor_command_id(&request.command) != request.tool_id {
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
            lasAnyEditor_command_id,
            LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lasAnyEditor_retained_reduce, lasAnyEditor_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn command_id(command: &Self::Command) -> &'static str { lasAnyEditor_command_id(command) }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> { lasAnyEditor_command_from_action(action, args) }

    fn initial_snapshot() -> LasSnapshot {
        LasSnapshot::default()
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
            LasAnyEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            LasAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&lasAnyEditor_example_snapshot(example_id), LAS_ANY_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.las@1.0/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for LasAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { LasAnyEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        editing::snapshot_edit_net_exact(event, snapshot, net_mutations)
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_las_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(LAS_ANY_DIALECT).document(["stdio", "las"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::LAS_ANY_EDIT_MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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
