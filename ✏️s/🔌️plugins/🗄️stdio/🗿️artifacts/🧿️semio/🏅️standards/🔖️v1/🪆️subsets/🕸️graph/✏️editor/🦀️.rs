//! ✏️ Semio Graph editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioGraphEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::semio_graph::modes::edit;
use crate::editor::semio_graph::modes::edit::windows::main;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane,  ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec, EditorApp, InteractiveJobClassification,
};
use store::EngineHandles;

//#region 🔖️Dialect
pub const SEMIO_GRAPH_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("graph") };
pub const SEMIO_GRAPH_DOCUMENT_SCHEMA: &str = "s.stdio.semio.graph";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The Main window declares the shared `MeshWindowKit::editable_window_kind()`'s
/// `set-vertex` action (contract §2.6), but this subset's own `🧬️schema/🧬️mutations` declares
/// no by-index "replace"/"set" op that action could honestly back today (only insert/remove and
/// whole-document `SetSnapshot`) — per this ticket's explicit allowance, the editor still exists
/// with a MINIMAL command set: the window really advertises the action, `handle` is a real dispatch
/// (not `unreachable!()`) that is a no-op today, rather than inventing a mutation the schema does
/// not have. Report, don't invent.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
pub enum SemioGraphEditCommand {
    #[default]
    SetVertex,
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
}

impl protocol::OpBinary for SemioGraphEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "SemioGraphEditCommand", offset: 0, detail: error.to_string() })?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "SemioGraphEditCommand", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };

const SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "s.stdio.semio.graph.tool-command.v1";
const SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;

fn semioGraphEditor_example_snapshot(example_id: &str) -> SemioGraphSnapshot {
    if example_id == crate::examples::demo::ID {
        <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        SemioGraphSnapshot::default()
    }
}

fn semioGraphEditor_command_id(command: &SemioGraphEditCommand) -> &'static str {
    match command {
        SemioGraphEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}

fn semioGraphEditor_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<SemioGraphEditCommand, Fault> {
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(SemioGraphEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}

fn semioGraphEditor_retained_extent(command: &SemioGraphEditCommand, _snapshot: &SemioGraphSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, SemioGraphEditCommand::SetActiveExample { .. }).then_some(1)
}

fn semioGraphEditor_retained_reduce(
    command: &SemioGraphEditCommand,
    _snapshot: &SemioGraphSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SemioGraphEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<SemioGraphMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        SemioGraphEditCommand::SetActiveExample { example_id } => Ok(Emit {
            effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&semioGraphEditor_example_snapshot(example_id), SEMIO_GRAPH_DOCUMENT_SCHEMA)],
            description: Some(format!("Load example {example_id}")),
            ..Default::default()
        }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}

struct SemioGraphEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}

impl SemioGraphEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for SemioGraphEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<SemioGraphEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<SemioGraphEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for SemioGraphEditorExampleFactory {
    type Owner = EditorApp<SemioGraphEditor>;
    const TOOL_IDS: &'static [&'static str] = SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_GRAPH_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<SemioGraphEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🦀️.rs",
        controller: "s.stdio.semio@v1/*#editor",
        artifact_schema: "s.stdio.semio.graph",
        factory: "SemioGraphEditorExampleFactory",
        factory_type: SemioGraphEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(SemioGraphEditorExampleFactory::new(registry.controller_id()))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if !SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if semioGraphEditor_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-example-tool-mismatch"));
        }
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
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
            semioGraphEditor_command_id,
            SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semioGraphEditor_retained_reduce, semioGraphEditor_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, SEMIO_GRAPH_DOCUMENT_SCHEMA, operation, generation))
    }

    fn command_id(command: &Self::Command) -> &'static str { semioGraphEditor_command_id(command) }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> { semioGraphEditor_command_from_action(action, args) }

    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[SEMIO_GRAPH_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioGraphEditor;

impl ArtifactEditor for SemioGraphEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = SemioGraphSnapshot;
    type Mutation = SemioGraphMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SemioGraphEditCommand;

    const DIALECT: Dialect = SEMIO_GRAPH_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_GRAPH_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SemioGraphSnapshot {
        SemioGraphSnapshot::default()
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
            SemioGraphEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&semioGraphEditor_example_snapshot(example_id), SEMIO_GRAPH_DOCUMENT_SCHEMA)],
                description: Some(format!("Load example {example_id}")),
                ..Default::default()
            }),
            _ => Ok(Emit::default()),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_graph_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SEMIO_GRAPH_DIALECT).document(["stdio", "semio"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::SEMIO_GRAPH_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
