//! ✏️ Semio Presentation editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `SemioPresentationEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::modes::edit;
use crate::editor::modes::edit::windows::main;
use crate::schema::mutations::SemioPresentationMutation;
use crate::schema::snapshot::SemioPresentationSnapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
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
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Dialect
pub const SEMIO_PRESENTATION_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };
pub const SEMIO_PRESENTATION_DOCUMENT_SCHEMA: &str = "s.stdio.semio.presentation";
//#endregion 🔖️Dialect

//#region 🔖️Command
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum SemioPresentationEditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for SemioPresentationEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = SEMIO_PRESENTATION_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "SemioPresentationEditCommand", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "SemioPresentationEditCommand", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };

const SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const SEMIO_PRESENTATION_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "s.stdio.semio.presentation.tool-command.v1";
const SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;

fn semio_presentation_editor_example_snapshot(example_id: &str) -> SemioPresentationSnapshot {
    if example_id == semio_s_artifact_stdio_semio_base_examples::examples::demo::ID {
        <SemioPresentationSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_stdio_semio_base_examples::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        SemioPresentationSnapshot::default()
    }
}

fn semio_presentation_editor_command_id(command: &SemioPresentationEditCommand) -> &'static str {
    if let SemioPresentationEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command {
        SemioPresentationEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}

fn semio_presentation_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<SemioPresentationEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| SemioPresentationEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(SemioPresentationEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}

fn semio_presentation_editor_retained_extent(command: &SemioPresentationEditCommand, _snapshot: &SemioPresentationSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, SemioPresentationEditCommand::SetActiveExample { .. }).then_some(1)
}

fn semio_presentation_editor_retained_reduce(
    command: &SemioPresentationEditCommand,
    _snapshot: &SemioPresentationSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SemioPresentationEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<SemioPresentationMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        SemioPresentationEditCommand::SetActiveExample { example_id } => Ok(Emit {
            effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&semio_presentation_editor_example_snapshot(example_id), SEMIO_PRESENTATION_DOCUMENT_SCHEMA)],
            ..Default::default()
        }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}

struct SemioPresentationEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}

impl SemioPresentationEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for SemioPresentationEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<SemioPresentationEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<SemioPresentationEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
}

impl ArtifactOwnedToolJobFactory for SemioPresentationEditorExampleFactory {
    type Owner = EditorApp<SemioPresentationEditor>;
    const TOOL_IDS: &'static [&'static str] = SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_PRESENTATION_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
pub use semio_s_artifact_stdio_semio_presentation_examples::editor::edit_rules;

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct SemioPresentationEditor;

impl ArtifactEditor for SemioPresentationEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_stdio_semio_base_examples::examples::demo::source()]
    }
    type Snapshot = SemioPresentationSnapshot;
    type Mutation = SemioPresentationMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SemioPresentationEditCommand;

    const DIALECT: Dialect = SEMIO_PRESENTATION_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SEMIO_PRESENTATION_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<SemioPresentationEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🦀️.rs",
        controller: "s.stdio.semio@v1/presentation#editor",
        artifact_schema: "s.stdio.semio.presentation",
        factory: "SemioPresentationEditorExampleFactory",
        factory_type: SemioPresentationEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(SemioPresentationEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx); }
        if !SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if semio_presentation_editor_command_id(&original.command) != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into());
        }
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, semio_presentation_editor_command_id, SEMIO_PRESENTATION_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_presentation_editor_retained_reduce, semio_presentation_editor_retained_extent))
    }

    semio_s_artifact_stdio_contract::snapshot_editing_publication_authority!();

    fn command_id(command: &Self::Command) -> &'static str { semio_presentation_editor_command_id(command) }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref(); semio_presentation_editor_command_from_action(action, args) }

    fn initial_snapshot() -> SemioPresentationSnapshot {
        SemioPresentationSnapshot::default()
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
            SemioPresentationEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            SemioPresentationEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&semio_presentation_editor_example_snapshot(example_id), SEMIO_PRESENTATION_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.semio@v1/presentation#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for SemioPresentationEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { SemioPresentationEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &edit_rules::EDIT_RULES
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_semio_presentation_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(SEMIO_PRESENTATION_DIALECT)
        .document(["stdio", "semio"])
        .icon_id("box")
        .mode_def(edit::definition())
        .default_mode_id(edit::SEMIO_PRESENTATION_EDIT_MODE_ID)
        .window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition())
        .default_layout(edit::layout())
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(semio_s_artifact_stdio_semio_base_examples::examples::demo::ID, semio_s_artifact_stdio_semio_base_examples::examples::demo::label())], semio_s_artifact_stdio_semio_base_examples::examples::demo::ID))
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
