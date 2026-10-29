//! ✏️ `mp4` editor (any) — `ArtifactEditor` surface built on the frozen
//! `MediaWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native media preview while retained Details actions publish typed artifact mutations; playback position remains ephemeral host state.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::mp4::modes::edit;
use crate::editor::mp4::modes::edit::windows::main;
use crate::standards::isobmff::subsets::any::schema::mutations::{insert_sample,remove_sample,Mp4Mutation};

use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Sample, Mp4Snapshot};
#[cfg(test)] use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Track};
use crate::{MP4_DIALECT, STDIO_MP4_DOCUMENT_SCHEMA};
use semio_framework_2d_compute::EngineHandles;
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
fn mp4_editor_example_snapshot(example_id: &str) -> Mp4Snapshot {
    if example_id == crate::examples::demo::ID {
        <Mp4Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        Mp4Snapshot::default()
    }
}
fn mp4_editor_command_id(command: &Mp4EditCommand) -> &'static str {
    if let Mp4EditCommand::EditSnapshot { event } = command {
        return event.action_id();
    }
    match command {
        Mp4EditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}
fn mp4_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Mp4EditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) {
        return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| Mp4EditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit"))));
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(Mp4EditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn mp4_editor_retained_extent(command: &Mp4EditCommand, _snapshot: &Mp4Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, Mp4EditCommand::SetActiveExample { .. }).then_some(1)
}
fn mp4_editor_retained_reduce(
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
        Mp4EditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&mp4_editor_example_snapshot(example_id), STDIO_MP4_DOCUMENT_SCHEMA)], ..Default::default() }),
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
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
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
    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) {
            return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx);
        }
        if !STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if mp4_editor_command_id(&original.command) != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into());
        }
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, mp4_editor_command_id, STDIO_MP4_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, mp4_editor_retained_reduce, mp4_editor_retained_extent))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, STDIO_MP4_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str {
        mp4_editor_command_id(command)
    }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        mp4_editor_command_from_action(action, args)
    }

    fn reference_birth()->Option<fn()->Result<Self::Snapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(Mp4Snapshot::default())
    })}

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_s_artifact_stdio_contract::media_export::playback_app_io::<crate::standards::isobmff::subsets::any::io::playback::Mp4PlaybackExport>())
    }

    fn build_media_export_job(request:&mut Option<semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<Self>>>,body:&mut Option<Box<dyn semio_framework_plugin::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if request.as_ref().is_some_and(|source|source.port!=semio_s_artifact_stdio_contract::media_export::PLAYBACK_PORT_ID||source.tool_id!=semio_s_artifact_stdio_contract::media_export::PLAYBACK_TOOL_ID){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"Stdio media source requires its exact published output port").into())}
        crate::standards::isobmff::subsets::any::io::playback::Mp4PlaybackExportJob::admit_original(request,body,cx)
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
            Mp4EditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&mp4_editor_example_snapshot(example_id), STDIO_MP4_DOCUMENT_SCHEMA)], ..Default::default() }),
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

//#endregion 🧪️Tests

//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_MP4_DOCUMENT_SCHEMA: &str = "stdio.mp4";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{STDIO_MP4_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.mp4".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Mp4", "Mp4"),
        source_format: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{STDIO_MP4_DOCUMENT_SCHEMA};
pub use crate::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;
pub use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

pub fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<Mp4Snapshot, Mp4Mutation>(STDIO_MP4_DOCUMENT_SCHEMA);
    codec.extension = "semio";
    codec
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{artifact_kind};
use crate::{native_codec};

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.mp4.v1", artifact: "mp4", kind: artifact_kind, codec: native_codec }]
}
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{native_codecs};
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_MP4_DOCUMENT_SCHEMA};
use crate::{artifact_kind};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
pub use crate::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;
pub use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE g5) — replaces
/// the side-effecting `crate::standards::isobmff::engine::register()` call the plugin
/// root used to make imperatively. `.composers(...)` reaches this standard's ENGINE-level
/// `io_registry` (below `⚙️engine`, distinct from this file's own `🚪️DerivedIoRegistry` shadow, whose
/// `entries()` returns `&[&ComposerEntry]` — the wrong type for `.composers()`, which wants
/// `&'static [ComposerEntry]`) by its fully qualified path, per the silent-rebind hazard this ticket
/// calls out. `.document_codec_bare::<Mp4Snapshot, Mp4Mutation>(...)` folds in what
/// `subsets::any::io::register()`'s `store::os_store::codec_registry::register_document_codec(store::artifact_codec::<..>())`
/// call did — mp4 is a headless stdio artifact with no `ArtifactApp` to bind `.document_codec::<A>()`
/// to.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("mp4", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::standards::isobmff::subsets::any::schema::inferences::mp4_artifact_inference_descriptor()])
        .composers(crate::standards::isobmff::subsets::any::io::io_registry::entries())
        .document_codec_bare::<Mp4Snapshot, Mp4Mutation>(STDIO_MP4_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") })
        .try_build()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems9

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "mp4", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems10

//#region 🧩️ArtifactItems11
pub mod artifact_items_11 {
use crate::{artifact_kind};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const MP4_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp4", standard: StandardId("isobmff"), subset: SubsetId("*") };
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems11
