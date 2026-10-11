//! ✏️ `mp4` editor (any) — `ArtifactEditor` surface built on the frozen
//! `MediaWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native media preview while retained Details actions publish typed artifact mutations; playback position remains ephemeral host state.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::mp4::modes::edit;
use crate::editor::mp4::modes::edit::windows::main;
use crate::standards::isobmff::subsets::any::schema::mutations::{insert_sample,insert_track,remove_sample,remove_track,set_ftyp,set_sample_sync,set_track_codec,Mp4Mutation};

use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Sample, Mp4Snapshot, Mp4Track};
use crate::{MP4_DIALECT, STDIO_MP4_DOCUMENT_SCHEMA};
use semio_framework_2d::compute::EngineHandles;
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
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_ui_locale::Label;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum Mp4EditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample {
        example_id: String,
    },
    EditSnapshot {
        event: editing::SnapshotEditEvent,
    },
}

impl protocol::OpBinary for Mp4EditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_MP4_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "mp4-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "mp4-edit-command", offset: 0, detail: error.to_string() })
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
    if example_id == crate::examples::demo::ID {
        <Mp4Snapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        Mp4Snapshot::default()
    }
}
fn mp4Editor_command_id(command: &Mp4EditCommand) -> &'static str {
    if let Mp4EditCommand::EditSnapshot { event } = command {
        return event.action_id();
    }
    match command {
        Mp4EditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}
fn mp4Editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Mp4EditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) {
        return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| Mp4EditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit"))));
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(Mp4EditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn mp4Editor_retained_extent(command: &Mp4EditCommand, _snapshot: &Mp4Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, Mp4EditCommand::SetActiveExample { .. }).then_some(1)
}
fn mp4Editor_retained_reduce(
    command: &Mp4EditCommand,
    _snapshot: &Mp4Snapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Mp4Editor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<Mp4Mutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        Mp4EditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&mp4Editor_example_snapshot(example_id), STDIO_MP4_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct Mp4EditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}
impl Mp4EditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl ToolJobFactory for Mp4EditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Mp4Editor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Mp4Editor>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint));
        }
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

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.mp4@isobmff", extension: ".mp4", media_type: "video/mp4", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        let bytes = crate::standards::isobmff::subsets::any::io::encode_mp4(snapshot);
        let reopened = crate::standards::isobmff::subsets::any::io::decode_mp4(&bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:natural".into(), error))?;
        if reopened == *snapshot {
            Ok(bytes)
        } else {
            Err(semio_framework_plugin::MediaError::Payload("artifact:natural".into(), "MP4 snapshot is not exactly representable".into()))
        }
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        let snapshot = crate::standards::isobmff::subsets::any::io::decode_mp4(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:natural".into(), error))?;
        if crate::standards::isobmff::subsets::any::io::encode_mp4(&snapshot).as_slice() == bytes {
            Ok(snapshot)
        } else {
            Err(semio_framework_plugin::MediaError::Payload("artifact:natural".into(), "MP4 input is outside the exact ISO-BMFF subset".into()))
        }
    }

    fn import_media(port: &str, media: &semio_framework_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

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
        editing::register_snapshot_edit_tool_factory::<Self>(registry)?;
        registry.register(crate::standards::isobmff::subsets::any::io::playback::Mp4MediaExportJobFactory::<EditorApp<Self>>::new(registry.controller_id()))
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) {
            return editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if mp4Editor_command_id(&request.command) != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-example-tool-mismatch"));
        }
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
            retained: request.retained,
        };
        let payload = ArtifactRetainedCommandPayload::new(
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
            mp4Editor_command_id,
            STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, mp4Editor_retained_reduce, mp4Editor_retained_extent)),
        );
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_MP4_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str {
        mp4Editor_command_id(command)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        mp4Editor_command_from_action(action, args)
    }

    fn initial_snapshot() -> Self::Snapshot {
        Mp4Snapshot::default()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_s_artifact_stdio_contract::media_export::playback_app_io::<crate::standards::isobmff::subsets::any::io::playback::Mp4PlaybackExport>())
    }

    fn build_media_export_job(request: semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ArtifactReservedToolJob>, Fault> {
        if request.port != semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID || request.tool_id != semio_s_artifact_stdio_contract::media_export::PLAYBACK_TOOL_ID {
            return Ok(None);
        }
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(crate::standards::isobmff::subsets::any::io::playback::Mp4PlaybackExportJob::new(request)?)))
    }

    fn build_snapshot_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactSnapshotDisposer<Self::Snapshot>>> {
        Some(Box::new(semio_s_artifact_stdio_contract::media_export::RetireOwnedSnapshotDisposer::<Mp4Snapshot>::default()))
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
            Mp4EditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&mp4Editor_example_snapshot(example_id), STDIO_MP4_DOCUMENT_SCHEMA)], ..Default::default() }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale, semio_s_artifact_stdio_contract::media_transport_resource(doc, "s.stdio.mp4@isobmff/*#editor")).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.mp4@isobmff/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY))
                .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

impl editing::SnapshotEditingEditor for Mp4Editor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command {
            Mp4EditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &crate::editor::mp4::edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        let path = match event {
            editing::SnapshotEditEvent::SetValue { path, .. } | editing::SnapshotEditEvent::InsertValue { path, .. } | editing::SnapshotEditEvent::RemoveValue { path } | editing::SnapshotEditEvent::MoveValue { path, .. } | editing::SnapshotEditEvent::RenameKey { path, .. } => path,
            editing::SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
        };
        let segments: Vec<&str> = path.split('/').skip(1).collect();
        let ["tracks", track, "samples", sample, field, ..] = segments.as_slice() else { return Ok(None) };
        if *field == "sync" {
            return Ok(None);
        }
        let fail = |message: String| Fault::from(message);
        let (track_index, index) = (track.parse::<usize>().map_err(|e| fail(e.to_string()))?, sample.parse::<usize>().map_err(|e| fail(e.to_string()))?);
        let current = snapshot.tracks.get(track_index).and_then(|track| track.samples.get(index)).ok_or_else(|| fail(format!("sample {index} of track {track_index} does not exist")))?;
        let edited = editing::edited_subtree(&semio_framework_value::ToValue::to_value(current), &format!("/tracks/{track_index}/samples/{index}"), event).map_err(|error| fail(error.to_string()))?;
        let sample = <Mp4Sample as semio_framework_value::FromValue>::from_value(edited).map_err(|error| fail(error.to_string()))?;
        Ok(Some(vec![Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index, index }), Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index, index, sample })]))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_mp4_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(MP4_DIALECT)
        .document(["semio", "mp4"])
        .icon_id("play")
        .mode_def(edit::definition())
        .default_mode_id(edit::MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(editing::snapshot_details_window_definition())
        .default_layout(edit::layout())
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
