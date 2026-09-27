//! ✏️ `avi` editor (any) — `ArtifactEditor` surface built on the frozen
//! `MediaWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native media preview while retained Details actions publish typed artifact mutations; playback position remains ephemeral host state.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::avi::modes::edit;
use crate::editor::avi::modes::edit::windows::main;
use crate::standards::v1_0::subsets::any::schema::mutations::{set_snapshot as snapshot_edit_set_snapshot, AviMutation};
use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::{AVI_DIALECT, STDIO_AVI_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{AppOperationContext, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use store::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum AviEditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for AviEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_AVI_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "avi-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "avi-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


const STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_AVI_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.avi.tool-command.v1";
const STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn aviEditor_example_snapshot(example_id: &str) -> AviSnapshot {
    if example_id == crate::examples::demo::ID { <AviSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { AviSnapshot::default() }
}
fn aviEditor_command_id(command: &AviEditCommand) -> &'static str {
    if let AviEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command { AviEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, _ => "other" }
}
fn aviEditor_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<AviEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| AviEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(AviEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn aviEditor_retained_extent(command: &AviEditCommand, _snapshot: &AviSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, AviEditCommand::SetActiveExample { .. }).then_some(1)
}
fn aviEditor_retained_reduce(command: &AviEditCommand, _snapshot: &AviSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<AviEditor>>>, _operation: &AppOperationContext) -> Result<Emit<AviMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        AviEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&aviEditor_example_snapshot(example_id), STDIO_AVI_DOCUMENT_SCHEMA)], description: Some(format!("Load example {example_id}")), ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct AviEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl AviEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for AviEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<AviEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<AviEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for AviEditorExampleFactory {
    type Owner = EditorApp<AviEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct AviEditor;

impl ArtifactEditor for AviEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = AviSnapshot;
    type Mutation = AviMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = AviEditCommand;

    const DIALECT: Dialect = AVI_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<AviEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/✏️editor/🦀️.rs",
        controller: "s.stdio.avi@1.0/*#editor",
        artifact_schema: "stdio.avi",
        factory: "AviEditorExampleFactory",
        factory_type: AviEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(AviEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if !STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if aviEditor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, aviEditor_command_id, STDIO_AVI_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, aviEditor_retained_reduce, aviEditor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_AVI_DOCUMENT_SCHEMA, operation, generation))
    }
    fn command_id(command: &Self::Command) -> &'static str { aviEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> { aviEditor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        AviSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            AviEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            AviEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&aviEditor_example_snapshot(example_id), STDIO_AVI_DOCUMENT_SCHEMA)],
                description: Some(format!("Load example {example_id}")),
                ..Default::default()
            }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc.snapshot, view_state.locale, "s.stdio.avi@1.0/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for AviEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { AviEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_is_admitted(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> bool {
        editing::snapshot_edit_value_is_admitted(event, snapshot)
    }
    fn snapshot_edit_emit(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| AviMutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot: snapshot }))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_avi_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(AVI_DIALECT).document(["semio", "avi"]).icon_id("play").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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
