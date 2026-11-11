//! ✏️ `gif` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::gif_87a::modes::edit;
use crate::editor::gif_87a::modes::edit::windows::main;
use crate::standards::v87a::subsets::any::schema::mutations::{edit_rules, GifMutation};

use crate::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;
use crate::{GIF_87A_DIALECT, STDIO_GIF_DOCUMENT_SCHEMA};
use semio_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactOwnedToolJobFactory;
use semio_plugin::ArtifactBoundedFirstStepProof;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactStoreInitializationJob;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactToolPublicationContract;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::EditorApp;
use semio_plugin::InteractiveJobClassification;
use semio_plugin::ToolExecutionContract;
use semio_plugin::ToolFactoryKey;
use semio_plugin::ToolJobFactory;
use semio_plugin::ToolJobFactoryError;
use semio_plugin::ToolOperationSpec;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_ui_locale::Label;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::NoPresence;
use semio_plugin::NoPresenceMutation;
use semio_plugin::NoTransient;
use semio_plugin::NoTransientMutation;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

#[path = "🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs"]
pub(crate) mod paint_region;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum Gif87aEditCommand {
    PaintRegion(paint_region::PaintRegionCommand),
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
    paint_region::ACTION_ID,
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
    match command { Gif87aEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Gif87aEditCommand::PaintRegion(_) => paint_region::ACTION_ID,
        _ => "other" }
}
fn gif87a_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Gif87aEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| Gif87aEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        paint_region::ACTION_ID => Ok(Gif87aEditCommand::PaintRegion(paint_region::PaintRegionCommand::from_action(args)?)),
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(Gif87aEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn gif87a_editor_retained_extent(command: &Gif87aEditCommand, _snapshot: &GifSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, Gif87aEditCommand::SetActiveExample { .. }).then_some(1)
}
fn gif87a_editor_retained_reduce(command: &Gif87aEditCommand, _snapshot: &GifSnapshot, _config: &NoConfig, _history: &semio_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_plugin::app::InteractionHoverState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gif87aEditor>>>, _operation: &AppOperationContext) -> Result<Emit<GifMutation, NoConfigMutation, NoDraftMutation>, Fault> {
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
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for Gif87aEditorExampleFactory {
    type Owner = EditorApp<Gif87aEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
const STDIO_PAINT_REGION_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: paint_region::ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] };
const fn gif87a_editor_paint_region_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(paint_region::MAXIMUM_RAW_BYTES, 64, paint_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}
struct Gif87aPaintRegionFactory { keys: Vec<ToolFactoryKey> }
impl Gif87aPaintRegionFactory {
    fn new(controller_id: &str) -> Self { Self { keys: paint_region::TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } }
}
impl ToolJobFactory for Gif87aPaintRegionFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Gif87aEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Gif87aEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { paint_region::PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { gif87a_editor_paint_region_execution_contract() }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: paint_region::MAXIMUM_RAW_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for Gif87aPaintRegionFactory {
    type Owner = EditorApp<Gif87aEditor>;
    const TOOL_IDS: &'static [&'static str] = paint_region::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_PAINT_REGION_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Gif87aEditor;

impl ArtifactEditor for Gif87aEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_plugin::ExampleSource> {
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

    fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {
        Some(semio_plugin::NaturalFileCodec { format_kind: "s.stdio.gif@87a", extension: ".gif", media_type: "image/gif", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_plugin::MediaError> {
        crate::standards::v87a::subsets::any::io::encode_gif(snapshot).map_err(|error| semio_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_plugin::MediaError> {
        crate::standards::v87a::subsets::any::io::decode_gif(bytes).map_err(|error| semio_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))
    }

    /// 📂️ Opening a natural file or a document pack is the whole-document LOAD (genesis path), never a history mutation.
    fn import_media(port: &str, media: &semio_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.gif@87a/*#editor";
        let mut proofs = vec![
            ArtifactBoundedFirstStepProof::new::<EditorApp<Gif87aEditor>>(OWNER_FILE, CONTROLLER, "Gif87aEditorExampleFactory", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "stdio.gif", ToolExecutionContract::bounded_first_step(STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<Gif87aEditor>, Gif87aEditorExampleFactory>(),
            ArtifactBoundedFirstStepProof::new::<EditorApp<Gif87aEditor>>(OWNER_FILE, CONTROLLER, "Gif87aPaintRegionFactory", paint_region::ACTION_ID, "stdio.gif", gif87a_editor_paint_region_execution_contract()).with_factory_type::<EditorApp<Gif87aEditor>, Gif87aPaintRegionFactory>(),
        ];
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, "stdio.gif"));
        proofs
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(Gif87aEditorExampleFactory::new(registry.controller_id()))?;
        registry.register(Gif87aPaintRegionFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, _construction:&mut semio_plugin::__job::OriginalConstructionAdmission,cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx); }
        if original.tool_id == paint_region::ACTION_ID {
            if gif87a_editor_command_id(&original.command) != original.tool_id { return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-paint-region-tool-mismatch").into()); }
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, gif87a_editor_command_id, paint_region::MAXIMUM_RAW_BYTES, paint_region::CAPACITY.work_items(), false, |request| paint_region::PaintRegionWork::new());
        }
        if !STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) { return Ok(false); }
        if gif87a_editor_command_id(&original.command) != original.tool_id { return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into()); }
        semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, gif87a_editor_command_id, STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, gif87a_editor_retained_reduce, gif87a_editor_retained_extent))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_plugin::bounded_document_store_initialization_job::<_,_,::semio_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, STDIO_GIF_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str { gif87a_editor_command_id(command) }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref(); gif87a_editor_command_from_action(action, args) }

    fn reference_birth()->Option<fn()->Result<Self::Snapshot,semio_plugin::Fault>>{Some(||{
        Ok(crate::standards::v87a::subsets::any::schema::blank_gif_snapshot())
    })}

    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            Gif87aEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            Gif87aEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&gif87a_editor_example_snapshot(example_id), STDIO_GIF_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
            Gif87aEditCommand::PaintRegion(_) => Err(Fault::from("region painting requires the cancellable retained route")),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.gif@87a/*#editor", &semio_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
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
pub fn create_gif_87a_editor() -> semio_plugin::AppDefinition {
    let builder = Editor::builder(GIF_87A_DIALECT).document(["semio", "gif"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Main")).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        ;
    let builder = builder.action_with(paint_region::action())
        .action_describe(paint_region::ACTION_ID, semio_ui_locale::LocalizedLabel::native("Paints a checked solid color or palette index into a rectangular region and records exact undo history.", "Malt eine geprüfte Volltonfarbe oder einen Palettenindex in einen rechteckigen Bereich und speichert einen exakten Rückgängig-Verlauf."))
        .action_interactive_job(paint_region::ACTION_ID, InteractiveJobClassification::Migrated);
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

pub fn definition() -> Result<semio_plugin::ArtifactDefinition, semio_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn formats() -> Result<Vec<semio_plugin::io::FormatDescriptor>, semio_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{artifact_kind, STDIO_GIF_DOCUMENT_SCHEMA, STDIO_GIF89A_DOCUMENT_SCHEMA};

/// 💾️ Durable native 87a document codec; its pack schema hash is the SHA-256 of the 87a binary snapshot protocol.
pub fn native_codec_87a() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<crate::standards::v87a::subsets::any::schema::snapshot::GifSnapshot, crate::standards::v87a::subsets::any::schema::mutations::GifMutation>(STDIO_GIF_DOCUMENT_SCHEMA);
    codec.extension = "gif";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("../🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

/// 💾️ Durable native 89a document codec; its pack schema hash is the SHA-256 of the 89a binary snapshot protocol.
pub fn native_codec_89a() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<crate::schema::snapshot::GifSnapshot, crate::schema::mutations::GifMutation>(STDIO_GIF89A_DOCUMENT_SCHEMA);
    codec.extension = "gif";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

/// 🧷️ The native factories the schema-owned codec bindings resolve, one per GIF standard.
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![
        semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.gif.87a.v1", artifact: "gif", kind: artifact_kind, codec: native_codec_87a },
        semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.gif.89a.v1", artifact: "gif", kind: artifact_kind, codec: native_codec_89a },
    ]
}
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_GIF_DOCUMENT_SCHEMA};
use {semio_plugin::ArtifactKindSpec, semio_framework_artifact_reference::Dialect, semio_plugin::MediaClass, semio_plugin::MediaForm, semio_plugin::MediaType, semio_plugin::OsMediaCapability, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};
pub use crate::schema::mutations::GifMutation;
pub use crate::schema::snapshot::GifSnapshot;
pub use crate::schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("gif", definition()?, declaration)
}

/// 🧾️ The runtime `gif` declares for both standards (`87a`, `89a`, independently versioned ids): schemas, format, inference
/// descriptors, composers and document codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_plugin::ArtifactDefinition) -> Result<semio_plugin::ArtifactDeclaration, semio_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_plugin::ArtifactDeclaration::builder(definition)
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
        label: semio_ui_locale::LocalizedLabel::native("Gif", "Gif"),
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
    DECLARED.get_or_init(||{semio_plugin::PluginBuilder::<_, semio_plugin::app::NoPluginApp>::new("stdio").label("GIF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(declaration(definition().unwrap()).unwrap()).try_build().unwrap();});
}
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems9
