//! ✏️ `jpg` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::jpg_any::modes::edit;
use crate::editor::jpg_any::modes::edit::windows::main;
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::{ChangeReEncodeQualityMutation, ChangeRestartIntervalMutation, JpgMutation, ReplaceHuffmanTableMutation, ReplacePixelsMutation, ReplaceQuantTableMutation};
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot;
use crate::{JPG_ANY_DIALECT, STDIO_JPG_DOCUMENT_SCHEMA};
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
use semio_framework_plugin::Dialect;
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum JpgAnyEditCommand {
    SetPixelRegion {
        pixels: Vec<u8>,
    },
    /// 🎬️ Navbar example picker payload.
    SetActiveExample {
        example_id: String,
    },
    EditSnapshot {
        event: editing::SnapshotEditEvent,
    },
}

impl protocol::OpBinary for JpgAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_JPG_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "jpg_any-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "jpg_any-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_JPG_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.jpg.tool-command.v1";
const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn jpgAnyEditor_example_snapshot(example_id: &str) -> JpgSnapshot {
    if example_id == crate::examples::demo::ID {
        <JpgSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        JpgSnapshot::default()
    }
}
fn jpgAnyEditor_command_id(command: &JpgAnyEditCommand) -> &'static str {
    if let JpgAnyEditCommand::EditSnapshot { event } = command {
        return event.action_id();
    }
    match command {
        JpgAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}
fn jpgAnyEditor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<JpgAnyEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) {
        return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| JpgAnyEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit"))));
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(JpgAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn jpgAnyEditor_retained_extent(command: &JpgAnyEditCommand, _snapshot: &JpgSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, JpgAnyEditCommand::SetActiveExample { .. }).then_some(1)
}
fn jpgAnyEditor_retained_reduce(
    command: &JpgAnyEditCommand,
    _snapshot: &JpgSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<JpgAnyEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<JpgMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        JpgAnyEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&jpgAnyEditor_example_snapshot(example_id), STDIO_JPG_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn jpgAnyEditor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
fn jpgAnyEditor_index(path: &str, prefix: &str, len: usize) -> Option<usize> {
    let segment = path.strip_prefix(prefix)?.split('/').next()?;
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    segment.parse::<usize>().ok().filter(|index| *index < len)
}
fn jpgAnyEditor_bounded_edit(event: &editing::SnapshotEditEvent, snapshot: &JpgSnapshot) -> Result<JpgSnapshot, Fault> {
    let patch = editing::prepare_snapshot_patch(snapshot, event).map_err(|error| jpgAnyEditor_edit_fault(error.code, error.to_string()))?;
    editing::apply_snapshot_patch_for_dialect(snapshot, &patch, JPG_ANY_DIALECT, STDIO_JPG_DOCUMENT_SCHEMA).map_err(|error| jpgAnyEditor_edit_fault(error.code, error.to_string()))
}
/// 🎯️ The domain leaf exactly as granular as a set of one whole field or table — re-encode quality, restart interval, the pixel
/// raster, one quantization or Huffman table — else `None`, and the edit publishes as a path-scoped patch (design §19.3: the JFIF
/// header leaf carries every header field, so a set of one of them is a patch).
fn jpgAnyEditor_compact_mutation(event: &editing::SnapshotEditEvent, next: &JpgSnapshot) -> Option<JpgMutation> {
    let editing::SnapshotEditEvent::SetValue { path, .. } = event else { return None };
    let table = |prefix: &str, len: usize| path.strip_prefix(prefix).filter(|rest| !rest.contains('/')).and_then(|_| jpgAnyEditor_index(path, prefix, len));
    match path.as_str() {
        "/reEncodeQuality" => Some(JpgMutation::ChangeReEncodeQuality(ChangeReEncodeQualityMutation { quality: next.re_encode_quality })),
        "/restartInterval" => Some(JpgMutation::ChangeRestartInterval(ChangeRestartIntervalMutation { restart_interval: next.restart_interval })),
        "/pixels" => Some(JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels: next.pixels.clone() })),
        _ => match (table("/quantTables/", next.quant_tables.len()), table("/huffmanTables/", next.huffman_tables.len())) {
            (Some(index), _) => Some(JpgMutation::ReplaceQuantTable(ReplaceQuantTableMutation { table: next.quant_tables[index].clone() })),
            (_, Some(index)) => Some(JpgMutation::ReplaceHuffmanTable(ReplaceHuffmanTableMutation { table: next.huffman_tables[index].clone() })),
            _ => None,
        },
    }
}
struct JpgAnyEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}
impl JpgAnyEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl ToolJobFactory for JpgAnyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<JpgAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<JpgAnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)
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
        if input.declared_bytes() > STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for JpgAnyEditorExampleFactory {
    type Owner = EditorApp<JpgAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JPG_DOCUMENT_SCHEMA;

    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JpgAnyEditor;

impl ArtifactEditor for JpgAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = JpgSnapshot;
    type Mutation = JpgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JpgAnyEditCommand;

    const DIALECT: Dialect = JPG_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JPG_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.jpg@jfif-1.01", extension: ".jpg", media_type: "image/jpeg", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        crate::standards::v_jfif_1_01::subsets::document::io::encode_jpg(snapshot).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        if !bytes.ends_with(&[0xff, 0xd9]) {
            return Err(semio_framework_plugin::MediaError::Payload("artifact:natural".into(), "JPEG input must end at its EOI marker".into()));
        }
        crate::standards::v_jfif_1_01::subsets::document::io::decode_jpg(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(JpgMutation::SetSnapshot(crate::standards::v_jfif_1_01::subsets::document::schema::mutations::set_snapshot::SetSnapshot { snapshot }))
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<JpgAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/🦀️.rs",
        controller: "s.stdio.jpg@jfif-1.01/*#editor",
        artifact_schema: "stdio.jpg",
        factory: "JpgAnyEditorExampleFactory",
        factory_type: JpgAnyEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(JpgAnyEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) {
            return editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if jpgAnyEditor_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-example-tool-mismatch"));
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
            jpgAnyEditor_command_id,
            STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, jpgAnyEditor_retained_reduce, jpgAnyEditor_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_JPG_DOCUMENT_SCHEMA, operation, generation))
    }
    fn command_id(command: &Self::Command) -> &'static str {
        jpgAnyEditor_command_id(command)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        jpgAnyEditor_command_from_action(action, args)
    }

    fn initial_snapshot() -> Self::Snapshot {
        crate::standards::v_jfif_1_01::subsets::document::schema::blank_jpg_snapshot()
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
            JpgAnyEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            JpgAnyEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&jpgAnyEditor_example_snapshot(example_id), STDIO_JPG_DOCUMENT_SCHEMA)], ..Default::default() }),
            JpgAnyEditCommand::SetPixelRegion { pixels } => Ok(Emit::mutations(vec![JpgMutation::ReplacePixels(crate::schema::mutations::ReplacePixelsMutation { pixels: pixels.clone() })])),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.jpg@jfif-1.01/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY))
                .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

impl editing::SnapshotEditingEditor for JpgAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command {
            JpgAnyEditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let next = jpgAnyEditor_bounded_edit(event, snapshot)?;
        if let Some(mutation) = jpgAnyEditor_compact_mutation(event, &next) {
            return Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() });
        }
        editing::snapshot_edit_patch(
            event,
            snapshot,
            |patch| JpgMutation::PatchSnapshot(crate::standards::v_jfif_1_01::subsets::document::schema::mutations::patch_snapshot::PatchSnapshot { patch }),
            Some(|snapshot| JpgMutation::SetSnapshot(crate::standards::v_jfif_1_01::subsets::document::schema::mutations::set_snapshot::SetSnapshot { snapshot })),
        )
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_jpg_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(JPG_ANY_DIALECT)
        .document(["semio", "jpg"])
        .icon_id("image")
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
