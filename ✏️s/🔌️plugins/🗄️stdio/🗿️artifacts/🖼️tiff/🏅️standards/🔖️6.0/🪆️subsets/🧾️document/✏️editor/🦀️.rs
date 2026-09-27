//! ✏️ `tiff` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::tiff_any::modes::edit;
use crate::editor::tiff_any::modes::edit::windows::main;
use crate::standards::v6_0::subsets::document::schema::mutations::{ChangeByteOrderMutation, InsertIfdMutation, RemoveIfdMutation, RemoveTagMutation, ReplacePixelsMutation, ReplaceTagMutation, set_snapshot as snapshot_edit_set_snapshot, TiffMutation};
use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffIfd, TiffSnapshot, TiffTag};
use crate::{STDIO_TIFF_DOCUMENT_SCHEMA, TIFF_ANY_DIALECT};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{AppOperationContext, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use store::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum TiffAnyEditCommand {
    SetPixelRegion { pixels: Vec<u8> },
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for TiffAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_TIFF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "tiff_any-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "tiff_any-edit-command", offset: 0, detail: error.to_string() })
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
fn tiffAnyEditor_example_snapshot(example_id: &str) -> TiffSnapshot {
    if example_id == crate::examples::demo::ID { <TiffSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { TiffSnapshot::default() }
}
fn tiffAnyEditor_command_id(command: &TiffAnyEditCommand) -> &'static str {
    if let TiffAnyEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command { TiffAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, _ => "other" }
}
fn tiffAnyEditor_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<TiffAnyEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| TiffAnyEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(TiffAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn tiffAnyEditor_retained_extent(command: &TiffAnyEditCommand, _snapshot: &TiffSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, TiffAnyEditCommand::SetActiveExample { .. }).then_some(1)
}
fn tiffAnyEditor_retained_reduce(command: &TiffAnyEditCommand, _snapshot: &TiffSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TiffAnyEditor>>>, _operation: &AppOperationContext) -> Result<Emit<TiffMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        TiffAnyEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&tiffAnyEditor_example_snapshot(example_id), STDIO_TIFF_DOCUMENT_SCHEMA)], description: Some(format!("Load example {example_id}")), ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn tiffAnyEditor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
fn tiffAnyEditor_index(segment: &str, len: usize, insertion: bool) -> Result<usize, Fault> {
    if insertion && segment == "-" { return Ok(len); }
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) { return Err(tiffAnyEditor_edit_fault("stdio.tiff.invalid-index", format!("'{segment}' is not a canonical index"))); }
    let index = segment.parse::<usize>().map_err(|error| tiffAnyEditor_edit_fault("stdio.tiff.invalid-index", error.to_string()))?;
    if index > len || (!insertion && index == len) { return Err(tiffAnyEditor_edit_fault("stdio.tiff.index-out-of-range", format!("index {index} is outside 0..{len}"))); }
    Ok(index)
}
fn tiffAnyEditor_ifd_path(path: &str) -> Option<(&str, Option<&str>)> {
    let rest = path.strip_prefix("/ifds/")?;
    Some(rest.split_once('/').map_or((rest, None), |(index, suffix)| (index, Some(suffix))))
}
fn tiffAnyEditor_entry_path(path: &str) -> Option<(&str, &str, Option<&str>)> {
    let (ifd, suffix) = tiffAnyEditor_ifd_path(path)?;
    let rest = suffix?.strip_prefix("entries/")?;
    Some(rest.split_once('/').map_or((ifd, rest, None), |(entry, suffix)| (ifd, entry, Some(suffix))))
}
fn tiffAnyEditor_direct_mutation(event: &editing::SnapshotEditEvent, snapshot: &TiffSnapshot) -> Result<Option<TiffMutation>, Fault> {
    match event {
        editing::SnapshotEditEvent::SetValue { path, value } if path == "/pixels" => {
            let pixels = <Vec<u8> as dsl::FromValue>::from_value(value.clone()).map_err(|error| tiffAnyEditor_edit_fault("stdio.tiff.invalid-pixels", error.to_string()))?;
            return Ok(Some(TiffMutation::ReplacePixels(ReplacePixelsMutation { pixels })));
        }
        editing::SnapshotEditEvent::InsertValue { path, value } => {
            if let Some((ifd, None)) = tiffAnyEditor_ifd_path(path) {
                let index = tiffAnyEditor_index(ifd, snapshot.ifds.len(), true)?;
                let ifd = <TiffIfd as dsl::FromValue>::from_value(value.clone()).map_err(|error| tiffAnyEditor_edit_fault("stdio.tiff.invalid-ifd", error.to_string()))?;
                return Ok(Some(TiffMutation::InsertIfd(InsertIfdMutation { index, ifd })));
            }
            if let Some((ifd, _, None)) = tiffAnyEditor_entry_path(path) {
                let ifd_index = tiffAnyEditor_index(ifd, snapshot.ifds.len(), false)?;
                let tag = <TiffTag as dsl::FromValue>::from_value(value.clone()).map_err(|error| tiffAnyEditor_edit_fault("stdio.tiff.invalid-tag", error.to_string()))?;
                return Ok(Some(TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index, tag: tag.tag, kind: tag.kind, values: tag.values })));
            }
        }
        editing::SnapshotEditEvent::RemoveValue { path } => {
            if let Some((ifd, None)) = tiffAnyEditor_ifd_path(path) {
                let index = tiffAnyEditor_index(ifd, snapshot.ifds.len(), false)?;
                return Ok(Some(TiffMutation::RemoveIfd(RemoveIfdMutation { index })));
            }
            if let Some((ifd, entry, None)) = tiffAnyEditor_entry_path(path) {
                let ifd_index = tiffAnyEditor_index(ifd, snapshot.ifds.len(), false)?;
                let entry = tiffAnyEditor_index(entry, snapshot.ifds[ifd_index].entries.len(), false)?;
                return Ok(Some(TiffMutation::RemoveTag(RemoveTagMutation { ifd_index, tag: snapshot.ifds[ifd_index].entries[entry].tag })));
            }
        }
        _ => {}
    }
    Ok(None)
}
fn tiffAnyEditor_bounded_edit(event: &editing::SnapshotEditEvent, snapshot: &TiffSnapshot) -> Result<TiffSnapshot, Fault> {
    let patch = editing::prepare_snapshot_patch(snapshot, event).map_err(|error| tiffAnyEditor_edit_fault(error.code, error.to_string()))?;
    editing::apply_snapshot_patch_for_dialect(snapshot, &patch, TIFF_ANY_DIALECT, STDIO_TIFF_DOCUMENT_SCHEMA).map_err(|error| tiffAnyEditor_edit_fault(error.code, error.to_string()))
}
fn tiffAnyEditor_compact_mutation(event: &editing::SnapshotEditEvent, next: TiffSnapshot, base: &TiffSnapshot) -> TiffMutation {
    let path = match event { editing::SnapshotEditEvent::SetValue { path, .. } => path.as_str(), _ => return TiffMutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot: next }) };
    if path == "/byteOrder" { return TiffMutation::ChangeByteOrder(ChangeByteOrderMutation { byte_order: next.byte_order }); }
    if let Some((ifd, entry, _)) = tiffAnyEditor_entry_path(path) {
        if let Ok(ifd_index) = tiffAnyEditor_index(ifd, base.ifds.len(), false) {
            if let Ok(entry) = tiffAnyEditor_index(entry, next.ifds[ifd_index].entries.len(), false) {
                let tag = next.ifds[ifd_index].entries[entry].clone();
                return TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index, tag: tag.tag, kind: tag.kind, values: tag.values });
            }
        }
    }
    TiffMutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot: next })
}
struct TiffAnyEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl TiffAnyEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for TiffAnyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TiffAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TiffAnyEditor>>;
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
impl ArtifactOwnedToolJobFactory for TiffAnyEditorExampleFactory {
    type Owner = EditorApp<TiffAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TIFF_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TiffAnyEditor;

impl ArtifactEditor for TiffAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = TiffSnapshot;
    type Mutation = TiffMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TiffAnyEditCommand;

    const DIALECT: Dialect = TIFF_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TIFF_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<TiffAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🦀️.rs",
        controller: "s.stdio.tiff@6.0/*#editor",
        artifact_schema: "stdio.tiff",
        factory: "TiffAnyEditorExampleFactory",
        factory_type: TiffAnyEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(TiffAnyEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if !STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if tiffAnyEditor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, tiffAnyEditor_command_id, STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, tiffAnyEditor_retained_reduce, tiffAnyEditor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_TIFF_DOCUMENT_SCHEMA, operation, generation))
    }
    fn command_id(command: &Self::Command) -> &'static str { tiffAnyEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> { tiffAnyEditor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        TiffSnapshot::default()
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
            TiffAnyEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            TiffAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&tiffAnyEditor_example_snapshot(example_id), STDIO_TIFF_DOCUMENT_SCHEMA)],
                description: Some(format!("Load example {example_id}")),
                ..Default::default()
            }),
            TiffAnyEditCommand::SetPixelRegion { pixels } => Ok(Emit::mutations(vec![TiffMutation::ReplacePixels(crate::schema::mutations::ReplacePixelsMutation { pixels: pixels.clone() })])),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc.snapshot, view_state.locale, "s.stdio.tiff@6.0/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for TiffAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { TiffAnyEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(mutation) = tiffAnyEditor_direct_mutation(event, snapshot)? {
            return Ok(Emit { artifact_mutations: vec![mutation], description: Some("Edit TIFF structure".into()), ..Default::default() });
        }
        let next = tiffAnyEditor_bounded_edit(event, snapshot)?;
        let mutation = tiffAnyEditor_compact_mutation(event, next, snapshot);
        if !matches!(mutation, TiffMutation::SetSnapshot(_)) {
            return Ok(Emit { artifact_mutations: vec![mutation], description: Some("Edit TIFF details".into()), ..Default::default() });
        }
        editing::snapshot_edit_patch(event, snapshot, |patch| TiffMutation::PatchSnapshot(crate::standards::v6_0::subsets::document::schema::mutations::patch_snapshot::PatchSnapshot { patch }))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_tiff_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(TIFF_ANY_DIALECT).document(["semio", "tiff"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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
