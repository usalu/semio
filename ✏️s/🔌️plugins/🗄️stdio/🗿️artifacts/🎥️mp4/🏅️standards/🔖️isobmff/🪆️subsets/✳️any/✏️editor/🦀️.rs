//! ✏️ `mp4` editor (any) — `ArtifactEditor` surface built on the frozen
//! `MediaWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native media preview while retained Details actions publish typed artifact mutations; playback position remains ephemeral host state.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::mp4::modes::edit;
use crate::editor::mp4::modes::edit::windows::main;
use crate::standards::isobmff::subsets::any::schema::mutations::{insert_sample, insert_track, remove_sample, remove_track, set_ftyp, set_sample_sync, set_snapshot as snapshot_edit_set_snapshot, set_track_codec, set_track_dimensions, Mp4Mutation};
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Sample, Mp4Snapshot, Mp4Track};
use crate::{MP4_DIALECT, STDIO_MP4_DOCUMENT_SCHEMA};
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
use semio_framework_plugin::Dialect;
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Mp4EditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for Mp4EditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_MP4_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "mp4-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "mp4-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


const STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_MP4_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.mp4.tool-command.v1";
const STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn mp4Editor_example_snapshot(example_id: &str) -> Mp4Snapshot {
    if example_id == crate::examples::demo::ID { <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { Mp4Snapshot::default() }
}
fn mp4Editor_command_id(command: &Mp4EditCommand) -> &'static str {
    if let Mp4EditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command { Mp4EditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, _ => "other" }
}
fn mp4Editor_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Mp4EditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| Mp4EditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(Mp4EditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn mp4Editor_retained_extent(command: &Mp4EditCommand, _snapshot: &Mp4Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, Mp4EditCommand::SetActiveExample { .. }).then_some(1)
}
fn mp4Editor_retained_reduce(command: &Mp4EditCommand, _snapshot: &Mp4Snapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Mp4Editor>>>, _operation: &AppOperationContext) -> Result<Emit<Mp4Mutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        Mp4EditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&mp4Editor_example_snapshot(example_id), STDIO_MP4_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn mp4Editor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
fn mp4Editor_index(segment: &str, len: usize, insertion: bool) -> Result<usize, Fault> {
    if insertion && segment == "-" { return Ok(len); }
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(mp4Editor_edit_fault("stdio.mp4.invalid-index", format!("'{segment}' is not a canonical array index")));
    }
    let index = segment.parse::<usize>().map_err(|error| mp4Editor_edit_fault("stdio.mp4.invalid-index", error.to_string()))?;
    if index > len || (!insertion && index == len) { return Err(mp4Editor_edit_fault("stdio.mp4.index-out-of-range", format!("index {index} is outside 0..{len}"))); }
    Ok(index)
}
fn mp4Editor_track_path(path: &str) -> Option<(&str, Option<&str>)> {
    let rest = path.strip_prefix("/tracks/")?;
    Some(rest.split_once('/').map_or((rest, None), |(index, suffix)| (index, Some(suffix))))
}
fn mp4Editor_sample_path(path: &str) -> Option<(&str, &str, Option<&str>)> {
    let (track, suffix) = mp4Editor_track_path(path)?;
    let rest = suffix?.strip_prefix("samples/")?;
    Some(rest.split_once('/').map_or((track, rest, None), |(sample, suffix)| (track, sample, Some(suffix))))
}
fn mp4Editor_direct_structural_mutation(event: &editing::SnapshotEditEvent, snapshot: &Mp4Snapshot) -> Result<Option<Mp4Mutation>, Fault> {
    match event {
        editing::SnapshotEditEvent::InsertValue { path, value } => {
            if let Some((track, None)) = mp4Editor_track_path(path) {
                let index = mp4Editor_index(track, snapshot.tracks.len(), true)?;
                let track = <Mp4Track as dsl::FromValue>::from_value(value.clone()).map_err(|error| mp4Editor_edit_fault("stdio.mp4.invalid-track", error.to_string()))?;
                return Ok(Some(Mp4Mutation::InsertTrack(insert_track::InsertTrack { index, track })));
            }
            if let Some((track, sample, None)) = mp4Editor_sample_path(path) {
                let track_index = mp4Editor_index(track, snapshot.tracks.len(), false)?;
                let index = mp4Editor_index(sample, snapshot.tracks[track_index].samples.len(), true)?;
                let sample = <Mp4Sample as dsl::FromValue>::from_value(value.clone()).map_err(|error| mp4Editor_edit_fault("stdio.mp4.invalid-sample", error.to_string()))?;
                return Ok(Some(Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index, index, sample })));
            }
        }
        editing::SnapshotEditEvent::RemoveValue { path } => {
            if let Some((track, None)) = mp4Editor_track_path(path) {
                let index = mp4Editor_index(track, snapshot.tracks.len(), false)?;
                return Ok(Some(Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index })));
            }
            if let Some((track, sample, None)) = mp4Editor_sample_path(path) {
                let track_index = mp4Editor_index(track, snapshot.tracks.len(), false)?;
                let index = mp4Editor_index(sample, snapshot.tracks[track_index].samples.len(), false)?;
                return Ok(Some(Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index, index })));
            }
        }
        _ => {}
    }
    Ok(None)
}
fn mp4Editor_bounded_edit(event: &editing::SnapshotEditEvent, snapshot: &Mp4Snapshot) -> Result<Mp4Snapshot, Fault> {
    let patch = editing::prepare_snapshot_patch(snapshot, event).map_err(|error| mp4Editor_edit_fault(error.code, error.to_string()))?;
    editing::apply_snapshot_patch_for_dialect(snapshot, &patch, MP4_DIALECT, STDIO_MP4_DOCUMENT_SCHEMA).map_err(|error| mp4Editor_edit_fault(error.code, error.to_string()))
}
fn mp4Editor_compact_mutation(event: &editing::SnapshotEditEvent, next: Mp4Snapshot, base: &Mp4Snapshot) -> Mp4Mutation {
    if matches!(event, editing::SnapshotEditEvent::SetValue { path, .. } if path == "/ftyp" || path.starts_with("/ftyp/")) {
        return Mp4Mutation::SetFtyp(set_ftyp::SetFtyp { ftyp: next.ftyp });
    }
    if let editing::SnapshotEditEvent::SetValue { path, .. } = event {
        if let Some((track, Some(field))) = mp4Editor_track_path(path) {
            if let Ok(track_index) = mp4Editor_index(track, base.tracks.len(), false) {
                if field == "width" || field == "height" {
                    let track = &next.tracks[track_index];
                    return Mp4Mutation::SetTrackDimensions(set_track_dimensions::SetTrackDimensions { track_index, width: track.width, height: track.height });
                }
                if field == "codec" || field.starts_with("codec/") {
                    return Mp4Mutation::SetTrackCodec(set_track_codec::SetTrackCodec { track_index, codec: next.tracks[track_index].codec.clone() });
                }
            }
        }
        if let Some((track, sample, Some("sync"))) = mp4Editor_sample_path(path) {
            if let Ok(track_index) = mp4Editor_index(track, base.tracks.len(), false) {
                if let Ok(index) = mp4Editor_index(sample, base.tracks[track_index].samples.len(), false) {
                    if let Some(sample) = next.tracks.get(track_index).and_then(|track| track.samples.get(index)) {
                        return Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index, index, sync: sample.sync });
                    }
                }
            }
        }
    }
    Mp4Mutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot: next })
}
struct Mp4EditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl Mp4EditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for Mp4EditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Mp4Editor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Mp4Editor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for Mp4EditorExampleFactory {
    type Owner = EditorApp<Mp4Editor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Mp4Editor;

impl ArtifactEditor for Mp4Editor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = Mp4Snapshot;
    type Mutation = Mp4Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Mp4EditCommand;

    const DIALECT: Dialect = MP4_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MP4_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<Mp4Editor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.mp4@isobmff/*#editor",
        artifact_schema: "stdio.mp4",
        factory: "Mp4EditorExampleFactory",
        factory_type: Mp4EditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(Mp4EditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if !STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if mp4Editor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, mp4Editor_command_id, STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, mp4Editor_retained_reduce, mp4Editor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_MP4_DOCUMENT_SCHEMA, operation, generation))
    }
    fn command_id(command: &Self::Command) -> &'static str { mp4Editor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> { mp4Editor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        Mp4Snapshot::default()
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
            Mp4EditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            Mp4EditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&mp4Editor_example_snapshot(example_id), STDIO_MP4_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale, semio_s_artifact_stdio_contract::media_transport_resource(doc, "s.stdio.mp4@isobmff/*#editor")).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc.snapshot, view_state.locale, "s.stdio.mp4@isobmff/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for Mp4Editor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { Mp4EditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(mutation) = mp4Editor_direct_structural_mutation(event, snapshot)? {
            return Ok(Emit { artifact_mutations: vec![mutation], description: Some("Edit MP4 structure".into()), ..Default::default() });
        }
        let next = mp4Editor_bounded_edit(event, snapshot)?;
        let mutation = mp4Editor_compact_mutation(event, next, snapshot);
        if !matches!(mutation, Mp4Mutation::SetSnapshot(_)) {
            return Ok(Emit { artifact_mutations: vec![mutation], description: Some("Edit MP4 details".into()), ..Default::default() });
        }
        editing::snapshot_edit_patch(event, snapshot, |patch| Mp4Mutation::PatchSnapshot(crate::standards::isobmff::subsets::any::schema::mutations::patch_snapshot::PatchSnapshot { patch }))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_mp4_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(MP4_DIALECT).document(["semio", "mp4"]).icon_id("play").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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
