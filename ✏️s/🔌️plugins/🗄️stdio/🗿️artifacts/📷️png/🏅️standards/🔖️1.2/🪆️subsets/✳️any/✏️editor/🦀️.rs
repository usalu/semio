//! ✏️ `png` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::png::modes::edit;
use crate::editor::png::modes::edit::windows::main;
use crate::standards::v1_2::subsets::any::schema::mutations::PngMutation;
use crate::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;
use crate::{PNG_DIALECT, STDIO_PNG_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactBoundedFirstStepProof;
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
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_2d_compute::EngineHandles;

use semio_s_artifact_stdio_contract::editing;

#[path = "🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🦀️.rs"]
pub(crate) mod patch_pixel_region;
#[path = "🎭️modes/✏️edit/🎮️commands/🎨️paint-native-region/🦀️.rs"]
pub(crate) mod paint_native_region;


pub(crate) use semio_s_artifact_stdio_png_editor::editor::png::publication;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum PngNativeEditCommand {
    PatchPixelRegion(patch_pixel_region::PatchPixelRegion),
    PaintNativeRegion(paint_native_region::PaintNativeRegion),
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
}

impl protocol::OpBinary for PngNativeEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_PNG_DOCUMENT_SCHEMA_NATIVE_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "png-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "png-edit-command", offset: 0, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(PngNativeEditCommand, [
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    patch_pixel_region::ACTION_ID,
    paint_native_region::INDEXED_ACTION_ID,
    paint_native_region::GRAYSCALE_ACTION_ID,
    paint_native_region::GRAYSCALE_ALPHA_ACTION_ID,
    paint_native_region::RGB_ACTION_ID,
    paint_native_region::RGBA_ACTION_ID
]);
pub type PngEditCommand = editing::SnapshotEditingCommand<PngNativeEditCommand>;
//#endregion 🔖️Command


const STDIO_PNG_DOCUMENT_SCHEMA_NATIVE_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    patch_pixel_region::ACTION_ID,
    paint_native_region::INDEXED_ACTION_ID,
    paint_native_region::GRAYSCALE_ACTION_ID,
    paint_native_region::GRAYSCALE_ALPHA_ACTION_ID,
    paint_native_region::RGB_ACTION_ID,
    paint_native_region::RGBA_ACTION_ID,
];
const STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.png.tool-command.v1";
const STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn png_editor_example_snapshot(example_id: &str) -> PngSnapshot {
    if example_id == crate::examples::demo::ID { <PngSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { PngSnapshot::default() }
}
fn png_editor_native_command_id(command: &PngNativeEditCommand) -> &'static str {
    match command {
        PngNativeEditCommand::PatchPixelRegion(_) => patch_pixel_region::ACTION_ID,
        PngNativeEditCommand::PaintNativeRegion(command) => paint_native_region::action_id(command.paint.profile),
        PngNativeEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}
pub fn png_editor_command_id(command: &PngEditCommand) -> &'static str {
    editing::snapshot_editing_command_id(command, png_editor_native_command_id)
}
fn png_editor_native_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<PngNativeEditCommand, Fault> {
    match action {
        patch_pixel_region::ACTION_ID => Ok(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion::from_action(args)?)),
        action if paint_native_region::ACTION_IDS.contains(&action) => Ok(PngNativeEditCommand::PaintNativeRegion(paint_native_region::PaintNativeRegion::from_action(action, args)?)),
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(PngNativeEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not a PNG editor action"))),
    }
}
pub fn png_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<PngEditCommand, Fault> {
    editing::snapshot_editing_command_from_action(action, args, png_editor_native_command_from_action)
}
fn png_editor_retained_extent(command: &PngEditCommand, _snapshot: &PngSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, PngEditCommand::Native(PngNativeEditCommand::SetActiveExample { .. })).then_some(1)
}
fn png_editor_retained_reduce(command: &PngEditCommand, _snapshot: &PngSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PngEditor>>>, _operation: &AppOperationContext) -> Result<Emit<PngMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        PngEditCommand::Native(PngNativeEditCommand::SetActiveExample { example_id }) => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&png_editor_example_snapshot(example_id), STDIO_PNG_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn png_editor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
type PngDetailsProvider = editing::DslSnapshotDetailsProvider<'static, PngSnapshot>;

struct PngEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl PngEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for PngEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<PngEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<PngEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for PngEditorExampleFactory {
    type Owner = EditorApp<PngEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PNG_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
const STDIO_PNG_PIXEL_REGION_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: patch_pixel_region::ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] };
const fn png_editor_pixel_region_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(patch_pixel_region::MAXIMUM_RAW_BYTES, 64, patch_pixel_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}
pub struct PngPixelRegionFactory { keys: Vec<ToolFactoryKey> }
impl PngPixelRegionFactory {
    fn new(controller_id: &str) -> Self { Self { keys: vec![ToolFactoryKey::new(controller_id, patch_pixel_region::ACTION_ID)] } }
}
impl ToolJobFactory for PngPixelRegionFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<PngEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<PngEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { patch_pixel_region::PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { png_editor_pixel_region_execution_contract() }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: patch_pixel_region::MAXIMUM_RAW_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for PngPixelRegionFactory {
    type Owner = EditorApp<PngEditor>;
    const TOOL_IDS: &'static [&'static str] = &[patch_pixel_region::ACTION_ID];
    const DOCUMENT_SCHEMA: &'static str = STDIO_PNG_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_PNG_PIXEL_REGION_CONTRACT];
}
const STDIO_PNG_NATIVE_REGION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: paint_native_region::INDEXED_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: paint_native_region::GRAYSCALE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: paint_native_region::GRAYSCALE_ALPHA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: paint_native_region::RGB_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: paint_native_region::RGBA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];
const fn png_editor_native_region_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(paint_native_region::MAXIMUM_RAW_BYTES, 64, paint_native_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}
struct PngNativeRegionFactory { keys: Vec<ToolFactoryKey> }
impl PngNativeRegionFactory {
    fn new(controller_id: &str) -> Self { Self { keys: paint_native_region::ACTION_IDS.iter().map(|id| ToolFactoryKey::new(controller_id, *id)).collect() } }
}
impl ToolJobFactory for PngNativeRegionFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<PngEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<PngEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { paint_native_region::PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { png_editor_native_region_execution_contract() }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: paint_native_region::MAXIMUM_RAW_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for PngNativeRegionFactory {
    type Owner = EditorApp<PngEditor>;
    const TOOL_IDS: &'static [&'static str] = paint_native_region::ACTION_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PNG_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = STDIO_PNG_NATIVE_REGION_CONTRACTS;
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct PngEditor;

impl ArtifactEditor for PngEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = PngSnapshot;
    type Mutation = PngMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = PngEditCommand;

    const DIALECT: Dialect = PNG_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PNG_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.png@1.2", extension: ".png", media_type: "image/png", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        crate::standards::v1_2::subsets::any::io::encode_png(snapshot).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        crate::standards::v1_2::subsets::any::io::decode_png(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.png@1.2/*#editor";
        let mut proofs = vec![ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngEditorExampleFactory", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "stdio.png", ToolExecutionContract::bounded_first_step(STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<PngEditor>, PngEditorExampleFactory>()];
        proofs.push(ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngPixelRegionFactory", patch_pixel_region::ACTION_ID, "stdio.png", png_editor_pixel_region_execution_contract()).with_factory_type::<EditorApp<PngEditor>, PngPixelRegionFactory>());
        for action_id in paint_native_region::ACTION_IDS {
            proofs.push(ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngNativeRegionFactory", *action_id, "stdio.png", png_editor_native_region_execution_contract()).with_factory_type::<EditorApp<PngEditor>, PngNativeRegionFactory>());
        }
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, "stdio.png"));
        proofs
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(PngEditorExampleFactory::new(registry.controller_id()))?;
        registry.register(PngPixelRegionFactory::new(registry.controller_id()))?;
        registry.register(PngNativeRegionFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx); }
        if original.tool_id == patch_pixel_region::ACTION_ID {
            if png_editor_command_id(&original.command) != original.tool_id { return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-png-pixel-region-tool-mismatch").into()); }
            
            return semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, png_editor_command_id, patch_pixel_region::MAXIMUM_RAW_BYTES, patch_pixel_region::CAPACITY.work_items(), false, |request| patch_pixel_region::PatchPixelRegionWork::default());
        }
        if paint_native_region::ACTION_IDS.contains(&original.tool_id.as_str()) {
            if png_editor_command_id(&original.command) != original.tool_id { return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-png-native-region-tool-mismatch").into()); }
            let tool_id = match original.tool_id.as_str() {
                paint_native_region::INDEXED_ACTION_ID => paint_native_region::INDEXED_ACTION_ID,
                paint_native_region::GRAYSCALE_ACTION_ID => paint_native_region::GRAYSCALE_ACTION_ID,
                paint_native_region::GRAYSCALE_ALPHA_ACTION_ID => paint_native_region::GRAYSCALE_ALPHA_ACTION_ID,
                paint_native_region::RGB_ACTION_ID => paint_native_region::RGB_ACTION_ID,
                paint_native_region::RGBA_ACTION_ID => paint_native_region::RGBA_ACTION_ID,
                _ => unreachable!("checked native PNG action roster"),
            };
            
            return semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, png_editor_command_id, paint_native_region::MAXIMUM_RAW_BYTES, paint_native_region::CAPACITY.work_items(), false, |request| paint_native_region::PaintNativeRegionWork::new(tool_id));
        }
        if !STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) { return Ok(false); }
        if png_editor_command_id(&original.command) != original.tool_id { return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into()); }
        
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, png_editor_command_id, STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, png_editor_retained_reduce, png_editor_retained_extent))
    }
    fn build_document_store_initialization_job(envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, STDIO_PNG_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(publication::factory())
    }
    fn command_id(command: &Self::Command) -> &'static str { png_editor_command_id(command) }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref(); png_editor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        crate::standards::v1_2::subsets::any::schema::blank_png_snapshot()
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
            PngEditCommand::Native(PngNativeEditCommand::SetActiveExample { example_id }) => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&png_editor_example_snapshot(example_id), STDIO_PNG_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
            PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(_)) => Err(Fault::from("PNG pixel-region editing requires the cancellable retained route")),
            PngEditCommand::Native(PngNativeEditCommand::PaintNativeRegion(_)) => Err(Fault::from("PNG native-region editing requires the cancellable retained route")),
            PngEditCommand::Edit(event) => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details_provider_revisioned(
                &PngDetailsProvider::from_value(semio_framework_value::ToValue::to_value(doc.snapshot)),
                semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?,
                view_state.locale,
                "s.stdio.png@1.2/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

impl editing::SnapshotEditingEditor for PngEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { PngEditCommand::Edit(event) => Some(event), PngEditCommand::Native(_) => None }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &crate::editor::png::edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        use crate::schema::mutations::{replace_samples, PngMutation};
        use crate::schema::snapshot::PngRegion;
        let editing::SnapshotEditEvent::SetValue { path, value: semio_framework_value::DslValue::Number(number) } = event else { return Ok(None) };
        let Some(index) = path.strip_prefix("/image/samples/").and_then(|rest| rest.parse::<usize>().ok()) else { return Ok(None) };
        let image = &snapshot.image;
        let spp = image.color_type.samples_per_pixel();
        let sample = u16::try_from(number.as_u64().ok_or_else(|| Fault::from("a PNG sample is an unsigned integer"))?).map_err(|_| Fault::from("a PNG sample is a 16-bit unsigned integer"))?;
        let pixel = index / spp;
        let (x, y) = ((pixel % image.width as usize) as u32, (pixel / image.width as usize) as u32);
        let region = PngRegion { x, y, width: 1, height: 1 };
        let mut samples = image.region_samples(region).ok_or_else(|| Fault::from(format!("sample {index} is outside the image")))?;
        samples[index % spp] = sample;
        Ok(Some(vec![PngMutation::ReplaceSamples(replace_samples::ReplaceSamples { region, samples })]))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_png_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(PNG_DIALECT).document(["semio", "png"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_with(patch_pixel_region::action())
        .action_describe(patch_pixel_region::ACTION_ID, LocalizedLabel::native("Paints a checked solid RGBA color into a rectangular region and records exact undo history.", "Malt eine geprüfte RGBA-Farbe in einen rechteckigen Bereich und speichert einen exakten Rückgängig-Verlauf."))
        .action_interactive_job(patch_pixel_region::ACTION_ID, InteractiveJobClassification::Migrated);
    let builder = paint_native_region::actions().into_iter().fold(builder, |builder, action| {
        let action_id = action.id.clone();
        builder.action_with(action)
            .action_describe(action_id.clone(), LocalizedLabel::native("Paints exact native PNG samples without converting the source color profile, precision, packing, or interlace.", "Malt exakte native PNG-Abtastwerte, ohne Farbprofil, Präzision, Packung oder Interlace der Quelle umzuwandeln."))
            .action_interactive_job(action_id, InteractiveJobClassification::Migrated)
    });
    editing::snapshot_edit_actions().into_iter().fold(builder, |builder, action| {
        let action_id = action.id.clone();
        builder.action_with(action).action_interactive_job(action_id, InteractiveJobClassification::Migrated)
    }).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{STDIO_PNG_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.png".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Png", "Png"),
        source_format: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{artifact_kind};
use crate::{native_codec};

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.png.v1", artifact: "png", kind: artifact_kind, codec: native_codec }]
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
use crate::{STDIO_PNG_DOCUMENT_SCHEMA};
use crate::{artifact_kind};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
pub use crate::schema::mutations::PngMutation;
pub use crate::schema::snapshot::PngSnapshot;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::standards::v1_2::subsets::any::io::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// row — png has no baseline/tiny/basic subset here, just the single ✳️any entry) by its FULLY
/// QUALIFIED path, never the bare `io_registry::entries()` shortcut that would silently rebind to
/// this file's own shadowing `io_registry` module below (repo-wide "silent rebind" hazard this
/// ticket tracks — that module returns `&[&ComposerEntry]`, a different type, and is left in place
/// as orphaned dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). Unlike
/// `🖼️tiff`/`📸️jpg`/`🎨️svg`, png's `register()` never registered a subset validator (no baseline
/// subset here) and never called `register_schema_specs()` — nothing left uncovered. `⚙️engine`
/// itself is untouched — this only REFERENCES what it already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("png", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor()])
        .composers(crate::standards::v1_2::subsets::any::io::facets::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") })
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — moved
/// here verbatim from `⚙️engine::register_pilot_languages` (same 5-role Document/Ops/Diff/Pack/Spr
/// shape every stdio artifact uses), leaked to a `&'static` slice since `dsl::passthrough_hooks`
/// isn't `const fn`, mirroring the `🗒️note`/`🔋️model` exemplars' own helper of the same shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png",
                    extension: Some("png"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1_2::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1_2::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1_2::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1_2::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1_2::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1_2::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.spr"),
                },
            ]
        })
        .as_slice()
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
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "png", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
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
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems11

//#region 🧩️ArtifactItems12
pub mod artifact_items_12 {
use crate::{pilot_languages};
use crate::{artifact_kind};
use crate::{STDIO_PNG_DOCUMENT_SCHEMA};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
pub use crate::schema::mutations::PngMutation;
pub use crate::schema::snapshot::PngSnapshot;

//#region 🔖️ImperativeRegister
/// 🌉️ Relocated from `⚙️engine::register` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `declaration()` above is what stdio's
/// own plugin-host boot path now uses, but external plugins (`🖨️raster`'s own
/// `ensure_stdio_semio_and_png_registered`) call this imperative entry point directly for
/// standalone `cargo test` runs that never execute the declarative plugin-host boot. Behavior
/// preserved verbatim — only the module path changed (`engine::register` → `png::register`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::io_registry::register();
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor()).expect("schema descriptor publication");
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor()).expect("schema descriptor publication");
    for lang in pilot_languages() {
        semio_framework_dsl::register_language(*lang);
    }
    semio_framework_plugin::io::register_native_snapshot_codec(Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") }, store::artifact_codec::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA)).expect("static Stdio registration must be available and conflict-free");
}
//#endregion 🔖️ImperativeRegister
}
//#endregion 🧩️ArtifactItems12
