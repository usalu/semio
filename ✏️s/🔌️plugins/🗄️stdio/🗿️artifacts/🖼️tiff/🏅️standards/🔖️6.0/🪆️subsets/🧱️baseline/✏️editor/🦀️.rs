//! ✏️ `tiff` editor (baseline) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::tiff_baseline::modes::edit;
use crate::editor::tiff_baseline::modes::edit::windows::main;
use crate::standards::v6_0::subsets::baseline::schema::mutations::TiffBaselineMutation;
use crate::standards::v6_0::subsets::baseline::schema::snapshot::TiffSnapshot;
use crate::{STDIO_TIFF_DOCUMENT_SCHEMA, TIFF_BASELINE_DIALECT};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_plugin::ArtifactEditor;
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
use semio_framework_2d::compute::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum TiffBaselineEditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for TiffBaselineEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_TIFF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "tiff_baseline-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "tiff_baseline-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_TIFF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.tiff.tool-command.v1";
const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn tiffBaselineEditor_example_snapshot(example_id: &str) -> TiffSnapshot {
    if example_id == crate::examples::demo::ID { <TiffSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { TiffSnapshot::default() }
}
fn tiffBaselineEditor_command_id(command: &TiffBaselineEditCommand) -> &'static str {
    if let TiffBaselineEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command { TiffBaselineEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, _ => "other" }
}
fn tiffBaselineEditor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<TiffBaselineEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| TiffBaselineEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(TiffBaselineEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn tiffBaselineEditor_retained_extent(command: &TiffBaselineEditCommand, _snapshot: &TiffSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, TiffBaselineEditCommand::SetActiveExample { .. }).then_some(1)
}
fn tiffBaselineEditor_retained_reduce(command: &TiffBaselineEditCommand, _snapshot: &TiffSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TiffBaselineEditor>>>, _operation: &AppOperationContext) -> Result<Emit<TiffBaselineMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        TiffBaselineEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&tiffBaselineEditor_example_snapshot(example_id), STDIO_TIFF_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct TiffBaselineEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl TiffBaselineEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for TiffBaselineEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TiffBaselineEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TiffBaselineEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for TiffBaselineEditorExampleFactory {
    type Owner = EditorApp<TiffBaselineEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TIFF_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TiffBaselineEditor;

impl ArtifactEditor for TiffBaselineEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = TiffSnapshot;
    type Mutation = TiffBaselineMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TiffBaselineEditCommand;

    const DIALECT: Dialect = TIFF_BASELINE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TIFF_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<TiffBaselineEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs",
        controller: "s.stdio.tiff@6.0/baseline#editor",
        artifact_schema: "stdio.tiff",
        factory: "TiffBaselineEditorExampleFactory",
        factory_type: TiffBaselineEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(TiffBaselineEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if !STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if tiffBaselineEditor_command_id(&request.command) != request.tool_id { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone(), retained: request.retained };
        let payload = ArtifactRetainedCommandPayload::new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, tiffBaselineEditor_command_id, STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, tiffBaselineEditor_retained_reduce, tiffBaselineEditor_retained_extent)));
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_TIFF_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str { tiffBaselineEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> { tiffBaselineEditor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        crate::standards::v6_0::subsets::document::schema::blank_tiff_snapshot()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            TiffBaselineEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            TiffBaselineEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&tiffBaselineEditor_example_snapshot(example_id), STDIO_TIFF_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.tiff@6.0/baseline#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


fn tiffBaselineEditor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}

impl editing::SnapshotEditingEditor for TiffBaselineEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { TiffBaselineEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &crate::editor::tiff_baseline::edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        use crate::standards::v6_0::subsets::baseline::schema::mutations::{set_bits_per_sample::SetBitsPerSample, set_photometric_interpretation::SetPhotometricInterpretation};
        use crate::schema::snapshot::{TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_PHOTOMETRIC};
        let path = match event {
            editing::SnapshotEditEvent::SetValue { path, .. } | editing::SnapshotEditEvent::InsertValue { path, .. } | editing::SnapshotEditEvent::RemoveValue { path } => path,
            _ => return Ok(None),
        };
        let segments: Vec<&str> = path.split('/').skip(1).collect();
        let ["ifds", "0", "entries", entry, ..] = segments.as_slice() else { return Ok(None) };
        let Some(row) = entry.parse::<usize>().ok().and_then(|index| snapshot.ifds.first().and_then(|page| page.entries.get(index))) else { return Ok(None) };
        if row.tag != TAG_PHOTOMETRIC && row.tag != TAG_BITS_PER_SAMPLE {
            return Ok(None);
        }
        let edited = editing::edited_subtree(&semio_framework_value::ToValue::to_value(row), &format!("/ifds/0/entries/{entry}"), event).map_err(|error| tiffBaselineEditor_edit_fault(error.code, error.to_string()))?;
        let edited = <TiffTag as semio_framework_value::FromValue>::from_value(edited).map_err(|error| tiffBaselineEditor_edit_fault("stdio.tiff.invalid-tag", error.to_string()))?;
        let TiffValues::Short(words) = edited.values else { return Err(tiffBaselineEditor_edit_fault("stdio.tiff.baseline-short-words", "the baseline interpretation tags hold SHORT words")) };
        Ok(Some(vec![match (row.tag, words.as_slice()) {
            (TAG_PHOTOMETRIC, [photometric]) => TiffBaselineMutation::SetPhotometricInterpretation(SetPhotometricInterpretation { photometric: *photometric }),
            (TAG_BITS_PER_SAMPLE, _) => TiffBaselineMutation::SetBitsPerSample(SetBitsPerSample { bits: words }),
            _ => return Err(tiffBaselineEditor_edit_fault("stdio.tiff.baseline-photometric-single", "the photometric interpretation holds exactly one SHORT word")),
        }]))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_tiff_baseline_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(TIFF_BASELINE_DIALECT).document(["semio", "tiff"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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
