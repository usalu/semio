//! ✏️ `gif` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::gif_87a::modes::edit;
use crate::editor::gif_87a::modes::edit::windows::main;
use crate::standards::v87a::subsets::any::schema::mutations::{edit_rules, set_image_pixels, GifMutation};

use crate::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
use crate::{GIF_87A_DIALECT, STDIO_GIF_DOCUMENT_SCHEMA};
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
pub enum Gif87aEditCommand {
    SetPixelRegion { indices: Vec<u8> },
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for Gif87aEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_GIF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_87a-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_87a-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


const STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_GIF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.gif.tool-command.v1";
const STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn gif87a_editor_example_snapshot(example_id: &str) -> GifSnapshot {
    if example_id == crate::examples::demo::ID { <GifSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { GifSnapshot::default() }
}
fn gif87a_editor_command_id(command: &Gif87aEditCommand) -> &'static str {
    if let Gif87aEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command { Gif87aEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, _ => "other" }
}
fn gif87a_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Gif87aEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| Gif87aEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(Gif87aEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn gif87a_editor_retained_extent(command: &Gif87aEditCommand, _snapshot: &GifSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, Gif87aEditCommand::SetActiveExample { .. }).then_some(1)
}
fn gif87a_editor_retained_reduce(command: &Gif87aEditCommand, _snapshot: &GifSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gif87aEditor>>>, _operation: &AppOperationContext) -> Result<Emit<GifMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        Gif87aEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&gif87a_editor_example_snapshot(example_id), STDIO_GIF_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct Gif87aEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl Gif87aEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for Gif87aEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Gif87aEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Gif87aEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for Gif87aEditorExampleFactory {
    type Owner = EditorApp<Gif87aEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Gif87aEditor;

impl ArtifactEditor for Gif87aEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = GifSnapshot;
    type Mutation = GifMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Gif87aEditCommand;

    const DIALECT: Dialect = GIF_87A_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF_DOCUMENT_SCHEMA;

    /// 📂️ Opening a natural file or a document pack is the whole-document LOAD (genesis path), never a history mutation.
    fn import_media(port: &str, media: &semio_framework_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<Gif87aEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.gif@87a/*#editor",
        artifact_schema: "stdio.gif",
        factory: "Gif87aEditorExampleFactory",
        factory_type: Gif87aEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(Gif87aEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if !STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if gif87a_editor_command_id(&request.command) != request.tool_id { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone(), retained: request.retained };
        let payload = ArtifactRetainedCommandPayload::new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, snapshot_read:request.snapshot_read, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, gif87a_editor_command_id, STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, gif87a_editor_retained_reduce, gif87a_editor_retained_extent)));
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, STDIO_GIF_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str { gif87a_editor_command_id(command) }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref(); gif87a_editor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        crate::standards::v87a::subsets::any::schema::blank_gif_snapshot()
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
            Gif87aEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            Gif87aEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&gif87a_editor_example_snapshot(example_id), STDIO_GIF_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
            Gif87aEditCommand::SetPixelRegion { indices } => Ok(Emit::mutations(vec![GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: indices.clone() })])),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.gif@87a/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for Gif87aEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { Gif87aEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        edit_rules::special(event, snapshot).map_err(|error| Fault::from(error.to_string()))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_gif_87a_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(GIF_87A_DIALECT).document(["semio", "gif"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_GIF_DOCUMENT_SCHEMA};
use {semio_framework_plugin::ArtifactKindSpec, semio_framework_artifact_reference::Dialect, semio_framework_plugin::MediaClass, semio_framework_plugin::MediaForm, semio_framework_plugin::MediaType, semio_framework_plugin::OsMediaCapability, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};
pub use crate::schema::mutations::GifMutation;
pub use crate::schema::snapshot::GifSnapshot;
pub use crate::schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("gif", definition()?, declaration)
}

/// 🧾️ The runtime `gif` declares for both standards (`87a`, `89a`, independently versioned ids): schemas, format, inference
/// descriptors, composers and document codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::standards::v87a::subsets::any::schema::gif_artifact_schema_descriptor())
        .schemas([crate::standards::v89a::subsets::any::schema::gif_artifact_schema_descriptor()])
        .formats(formats)
        .inferences([crate::standards::v87a::subsets::any::schema::inferences::gif_artifact_inference_descriptor(), crate::standards::v89a::subsets::any::schema::inferences::gif89a_artifact_inference_descriptor()])
        .composers(crate::standards::v87a::subsets::any::io::io_registry::entries())
        .composers(crate::standards::v89a::subsets::any::io::io_registry::entries())
        .document_codec_bare::<crate::standards::v87a::subsets::any::schema::snapshot::GifSnapshot, crate::standards::v87a::subsets::any::schema::mutations::GifMutation>(STDIO_GIF_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") })
        .document_codec_bare::<GifSnapshot, GifMutation>(STDIO_GIF89A_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") })
        .try_build()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.gif".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Gif", "Gif"),
        source_format: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "gif", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems8

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
use crate::{artifact_kind, declaration};
use crate::{definition};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const GIF_87A_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
pub const GIF_89A_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") };

#[cfg(any(test, feature = "testkit"))]
pub fn register_sqlite_test_declaration(){
    static DECLARED:std::sync::OnceLock<()>=std::sync::OnceLock::new();
    DECLARED.get_or_init(||{semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("stdio").label("GIF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(declaration(definition().unwrap()).unwrap()).try_build().unwrap();});
}
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems9
