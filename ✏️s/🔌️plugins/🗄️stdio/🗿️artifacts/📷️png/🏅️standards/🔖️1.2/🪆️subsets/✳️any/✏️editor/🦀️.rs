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
use semio_framework_2d::compute::EngineHandles;

use semio_s_artifact_stdio_contract::editing;

#[path = "🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🦀️.rs"]
pub(crate) mod patch_pixel_region;
#[path = "🎭️modes/✏️edit/🎮️commands/🎨️paint-native-region/🦀️.rs"]
pub(crate) mod paint_native_region;

#[path="🧬️publication/🦀️.rs"]
pub(crate) mod publication;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
fn pngEditor_example_snapshot(example_id: &str) -> PngSnapshot {
    if example_id == crate::examples::demo::ID { <PngSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { PngSnapshot::default() }
}
fn pngEditor_native_command_id(command: &PngNativeEditCommand) -> &'static str {
    match command {
        PngNativeEditCommand::PatchPixelRegion(_) => patch_pixel_region::ACTION_ID,
        PngNativeEditCommand::PaintNativeRegion(command) => paint_native_region::action_id(command.paint.profile),
        PngNativeEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}
fn pngEditor_command_id(command: &PngEditCommand) -> &'static str {
    editing::snapshot_editing_command_id(command, pngEditor_native_command_id)
}
fn pngEditor_native_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<PngNativeEditCommand, Fault> {
    match action {
        patch_pixel_region::ACTION_ID => Ok(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion::from_action(args)?)),
        action if paint_native_region::ACTION_IDS.contains(&action) => Ok(PngNativeEditCommand::PaintNativeRegion(paint_native_region::PaintNativeRegion::from_action(action, args)?)),
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(PngNativeEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not a PNG editor action"))),
    }
}
fn pngEditor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<PngEditCommand, Fault> {
    editing::snapshot_editing_command_from_action(action, args, pngEditor_native_command_from_action)
}
fn pngEditor_retained_extent(command: &PngEditCommand, _snapshot: &PngSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, PngEditCommand::Native(PngNativeEditCommand::SetActiveExample { .. })).then_some(1)
}
fn pngEditor_retained_reduce(command: &PngEditCommand, _snapshot: &PngSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PngEditor>>>, _operation: &AppOperationContext) -> Result<Emit<PngMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        PngEditCommand::Native(PngNativeEditCommand::SetActiveExample { example_id }) => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&pngEditor_example_snapshot(example_id), STDIO_PNG_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn pngEditor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
fn pngEditor_snapshot_edit(event: &editing::SnapshotEditEvent, snapshot: &PngSnapshot) -> Result<PngSnapshot, Fault> {
    if let editing::SnapshotEditEvent::ReplaceSource { source } = event {
        return editing::snapshot_from_edit_source(source).map_err(|error| pngEditor_edit_fault("stdio.png.invalid-source", error.to_string()));
    }
    let patch = editing::prepare_snapshot_patch(snapshot, event).map_err(|error| pngEditor_edit_fault(error.code, error.to_string()))?;
    editing::apply_snapshot_patch_for_dialect(snapshot, &patch, PNG_DIALECT, STDIO_PNG_DOCUMENT_SCHEMA).map_err(|error| pngEditor_edit_fault(error.code, error.to_string()))
}

fn pngEditor_metadata_mutation(_next: &PngSnapshot, _base: &PngSnapshot) -> Option<PngMutation> {
    None
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
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for PngEditorExampleFactory {
    type Owner = EditorApp<PngEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PNG_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
const STDIO_PNG_PIXEL_REGION_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: patch_pixel_region::ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] };
const fn pngEditor_pixel_region_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(patch_pixel_region::MAXIMUM_RAW_BYTES, 64, patch_pixel_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}
struct PngPixelRegionFactory { keys: Vec<ToolFactoryKey> }
impl PngPixelRegionFactory {
    fn new(controller_id: &str) -> Self { Self { keys: vec![ToolFactoryKey::new(controller_id, patch_pixel_region::ACTION_ID)] } }
}
impl ToolJobFactory for PngPixelRegionFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<PngEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<PngEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { patch_pixel_region::PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { pngEditor_pixel_region_execution_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > patch_pixel_region::MAXIMUM_RAW_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("PNG pixel region rejects oversized wire or checkpoint owner"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
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
const fn pngEditor_native_region_execution_contract() -> ToolExecutionContract {
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
    fn execution_contract(&self) -> ToolExecutionContract { pngEditor_native_region_execution_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > paint_native_region::MAXIMUM_RAW_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("PNG native region rejects oversized wire or checkpoint owner"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
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

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(PngMutation::SetSnapshot(crate::schema::mutations::SetSnapshot { snapshot }))
    }

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.png@1.2/*#editor";
        let mut proofs = vec![ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngEditorExampleFactory", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "stdio.png", ToolExecutionContract::bounded_first_step(STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<PngEditor>, PngEditorExampleFactory>()];
        proofs.push(ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngPixelRegionFactory", patch_pixel_region::ACTION_ID, "stdio.png", pngEditor_pixel_region_execution_contract()).with_factory_type::<EditorApp<PngEditor>, PngPixelRegionFactory>());
        for action_id in paint_native_region::ACTION_IDS {
            proofs.push(ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngNativeRegionFactory", *action_id, "stdio.png", pngEditor_native_region_execution_contract()).with_factory_type::<EditorApp<PngEditor>, PngNativeRegionFactory>());
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
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if request.tool_id == patch_pixel_region::ACTION_ID {
            if pngEditor_command_id(&request.command) != request.tool_id { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-png-pixel-region-tool-mismatch")); }
            let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
            let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, pngEditor_command_id, patch_pixel_region::MAXIMUM_RAW_BYTES, patch_pixel_region::CAPACITY.work_items(), Box::new(patch_pixel_region::PatchPixelRegionWork::default()))?;
            return Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        if paint_native_region::ACTION_IDS.contains(&request.tool_id.as_str()) {
            if pngEditor_command_id(&request.command) != request.tool_id { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-png-native-region-tool-mismatch")); }
            let tool_id = match request.tool_id.as_str() {
                paint_native_region::INDEXED_ACTION_ID => paint_native_region::INDEXED_ACTION_ID,
                paint_native_region::GRAYSCALE_ACTION_ID => paint_native_region::GRAYSCALE_ACTION_ID,
                paint_native_region::GRAYSCALE_ALPHA_ACTION_ID => paint_native_region::GRAYSCALE_ALPHA_ACTION_ID,
                paint_native_region::RGB_ACTION_ID => paint_native_region::RGB_ACTION_ID,
                paint_native_region::RGBA_ACTION_ID => paint_native_region::RGBA_ACTION_ID,
                _ => unreachable!("checked native PNG action roster"),
            };
            let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
            let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, pngEditor_command_id, paint_native_region::MAXIMUM_RAW_BYTES, paint_native_region::CAPACITY.work_items(), Box::new(paint_native_region::PaintNativeRegionWork::new(tool_id)))?;
            return Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        if !STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if pngEditor_command_id(&request.command) != request.tool_id { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, pngEditor_command_id, STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, pngEditor_retained_reduce, pngEditor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_PNG_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(publication::factory())
    }
    fn command_id(command: &Self::Command) -> &'static str { pngEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> { pngEditor_command_from_action(action, args) }

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
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&pngEditor_example_snapshot(example_id), STDIO_PNG_DOCUMENT_SCHEMA)],
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
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let next = pngEditor_snapshot_edit(event, snapshot)?;
        if let Some(mutation) = pngEditor_metadata_mutation(&next, snapshot) {
            return Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() });
        }
        editing::snapshot_edit_patch(event, snapshot, |patch| PngMutation::PatchSnapshot(crate::schema::mutations::PatchSnapshot { patch }), Some(|snapshot| PngMutation::SetSnapshot(crate::schema::mutations::SetSnapshot { snapshot })))
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
