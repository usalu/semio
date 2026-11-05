//! ✏️ `gif` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::gif_89a::modes::edit;
use crate::editor::gif_89a::modes::edit::windows::main;
use crate::standards::v89a::subsets::any::schema::mutations::{edit_rules, GifMutation};

use crate::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use crate::{GIF_89A_DIALECT, STDIO_GIF89A_DOCUMENT_SCHEMA};
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
pub enum Gif89aEditCommand {
    PaintRegion(paint_region::PaintRegionCommand),
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for Gif89aEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_GIF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_89a-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "gif_89a-edit-command", offset: 0, detail: error.to_string() })
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
fn gif89a_editor_example_snapshot(example_id: &str) -> GifSnapshot {
    if example_id == crate::examples::demo::ID { <GifSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { GifSnapshot::default() }
}
fn gif89a_editor_command_id(command: &Gif89aEditCommand) -> &'static str {
    if let Gif89aEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command { Gif89aEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Gif89aEditCommand::PaintRegion(_) => paint_region::ACTION_ID,
        _ => "other" }
}
fn gif89a_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Gif89aEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| Gif89aEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        paint_region::ACTION_ID => Ok(Gif89aEditCommand::PaintRegion(paint_region::PaintRegionCommand::from_action(args)?)),
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(Gif89aEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn gif89a_editor_retained_extent(command: &Gif89aEditCommand, _snapshot: &GifSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, Gif89aEditCommand::SetActiveExample { .. }).then_some(1)
}
fn gif89a_editor_retained_reduce(command: &Gif89aEditCommand, _snapshot: &GifSnapshot, _config: &NoConfig, _history: &semio_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_plugin::app::InteractionHoverState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gif89aEditor>>>, _operation: &AppOperationContext) -> Result<Emit<GifMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        Gif89aEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&gif89a_editor_example_snapshot(example_id), STDIO_GIF89A_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct Gif89aEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl Gif89aEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for Gif89aEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Gif89aEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Gif89aEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for Gif89aEditorExampleFactory {
    type Owner = EditorApp<Gif89aEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF89A_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
const STDIO_PAINT_REGION_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: paint_region::ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] };
const fn gif89a_editor_paint_region_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(paint_region::MAXIMUM_RAW_BYTES, 64, paint_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}
struct Gif89aPaintRegionFactory { keys: Vec<ToolFactoryKey> }
impl Gif89aPaintRegionFactory {
    fn new(controller_id: &str) -> Self { Self { keys: paint_region::TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } }
}
impl ToolJobFactory for Gif89aPaintRegionFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Gif89aEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Gif89aEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { paint_region::PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { gif89a_editor_paint_region_execution_contract() }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: paint_region::MAXIMUM_RAW_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for Gif89aPaintRegionFactory {
    type Owner = EditorApp<Gif89aEditor>;
    const TOOL_IDS: &'static [&'static str] = paint_region::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF89A_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_PAINT_REGION_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Gif89aEditor;

impl ArtifactEditor for Gif89aEditor {
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
    type Command = Gif89aEditCommand;

    const DIALECT: Dialect = GIF_89A_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_GIF89A_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {
        Some(semio_plugin::NaturalFileCodec { format_kind: "s.stdio.gif@89a", extension: ".gif", media_type: "image/gif", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_plugin::MediaError> {
        crate::standards::v89a::subsets::any::io::encode_gif(snapshot).map_err(|error| semio_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_plugin::MediaError> {
        crate::standards::v89a::subsets::any::io::decode_gif(bytes).map_err(|error| semio_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))
    }

    /// 📂️ Opening a natural file or a document pack is the whole-document LOAD (genesis path), never a history mutation.
    fn import_media(port: &str, media: &semio_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.gif@89a/*#editor";
        let mut proofs = vec![
            ArtifactBoundedFirstStepProof::new::<EditorApp<Gif89aEditor>>(OWNER_FILE, CONTROLLER, "Gif89aEditorExampleFactory", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "stdio.gif.89a", ToolExecutionContract::bounded_first_step(STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<Gif89aEditor>, Gif89aEditorExampleFactory>(),
            ArtifactBoundedFirstStepProof::new::<EditorApp<Gif89aEditor>>(OWNER_FILE, CONTROLLER, "Gif89aPaintRegionFactory", paint_region::ACTION_ID, "stdio.gif.89a", gif89a_editor_paint_region_execution_contract()).with_factory_type::<EditorApp<Gif89aEditor>, Gif89aPaintRegionFactory>(),
        ];
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, "stdio.gif.89a"));
        proofs
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(Gif89aEditorExampleFactory::new(registry.controller_id()))?;
        registry.register(Gif89aPaintRegionFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx); }
        if original.tool_id == paint_region::ACTION_ID {
            if gif89a_editor_command_id(&original.command) != original.tool_id { return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-paint-region-tool-mismatch").into()); }
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, gif89a_editor_command_id, paint_region::MAXIMUM_RAW_BYTES, paint_region::CAPACITY.work_items(), false, |request| paint_region::PaintRegionWork::new());
        }
        if !STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) { return Ok(false); }
        if gif89a_editor_command_id(&original.command) != original.tool_id { return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into()); }
        semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, gif89a_editor_command_id, STDIO_GIF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, gif89a_editor_retained_reduce, gif89a_editor_retained_extent))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_plugin::bounded_document_store_initialization_job::<_,_,::semio_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, STDIO_GIF89A_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str { gif89a_editor_command_id(command) }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref(); gif89a_editor_command_from_action(action, args) }

    fn reference_birth()->Option<fn()->Result<Self::Snapshot,semio_plugin::Fault>>{Some(||{
        Ok(crate::standards::v89a::subsets::any::schema::blank_gif_snapshot())
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
            Gif89aEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            Gif89aEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&gif89a_editor_example_snapshot(example_id), STDIO_GIF89A_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
            Gif89aEditCommand::PaintRegion(_) => Err(Fault::from("region painting requires the cancellable retained route")),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.gif@89a/*#editor", &semio_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for Gif89aEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { Gif89aEditCommand::EditSnapshot { event } => Some(event), _ => None }
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
pub fn create_gif_89a_editor() -> semio_plugin::AppDefinition {
    let builder = Editor::builder(GIF_89A_DIALECT).document(["semio", "gif"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Main")).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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
