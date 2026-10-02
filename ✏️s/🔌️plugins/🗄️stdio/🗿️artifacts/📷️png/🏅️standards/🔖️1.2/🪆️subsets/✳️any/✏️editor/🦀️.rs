//! ✏️ `png` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::png::modes::edit;
use crate::editor::png::modes::edit::windows::main;
use crate::standards::v1_2::subsets::any::schema::mutations::{PatchPixelsMutation, PngMutation, ReplacePixelsMutation};
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
use semio_framework_plugin::Dialect;
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

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum PngNativeEditCommand {
    PatchPixelRegion(patch_pixel_region::PatchPixelRegion),
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
}

impl protocol::OpBinary for PngNativeEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_PNG_DOCUMENT_SCHEMA_NATIVE_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "png-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "png-edit-command", offset: 0, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(PngNativeEditCommand, [semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, patch_pixel_region::ACTION_ID]);
pub type PngEditCommand = editing::SnapshotEditingCommand<PngNativeEditCommand>;
//#endregion 🔖️Command


const STDIO_PNG_DOCUMENT_SCHEMA_NATIVE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, patch_pixel_region::ACTION_ID];
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
        PngNativeEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}
fn pngEditor_command_id(command: &PngEditCommand) -> &'static str {
    editing::snapshot_editing_command_id(command, pngEditor_native_command_id)
}
fn pngEditor_native_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<PngNativeEditCommand, Fault> {
    match action {
        patch_pixel_region::ACTION_ID => Ok(PngNativeEditCommand::PatchPixelRegion(patch_pixel_region::PatchPixelRegion::from_action(args)?)),
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(PngNativeEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not a PNG editor action"))),
    }
}
fn pngEditor_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<PngEditCommand, Fault> {
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
fn pngEditor_pixel_index(segment: &str, len: usize, insertion: bool) -> Result<usize, Fault> {
    if insertion && segment == "-" { return Ok(len); }
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(pngEditor_edit_fault("stdio.png.invalid-pixel-path", format!("pixel index '{segment}' is not canonical")));
    }
    let index = segment.parse::<usize>().map_err(|error| pngEditor_edit_fault("stdio.png.invalid-pixel-path", error.to_string()))?;
    if index > len || (!insertion && index == len) { return Err(pngEditor_edit_fault("stdio.png.pixel-index-out-of-range", format!("pixel index {index} is outside 0..{len}"))); }
    Ok(index)
}
fn pngEditor_pixel_value(value: &dsl::DslValue) -> Result<u8, Fault> {
    let dsl::DslValue::Number(number) = value else { return Err(pngEditor_edit_fault("stdio.png.invalid-pixel-value", "pixel channel value must be an integer")); };
    let value = number.as_u64().ok_or_else(|| pngEditor_edit_fault("stdio.png.invalid-pixel-value", "pixel channel value must be unsigned"))?;
    u8::try_from(value).map_err(|_| pngEditor_edit_fault("stdio.png.invalid-pixel-value", "pixel channel value must be in 0..255"))
}
fn pngEditor_snapshot_edit(event: &editing::SnapshotEditEvent, snapshot: &PngSnapshot) -> Result<PngSnapshot, Fault> {
    use editing::SnapshotEditEvent;
    if let SnapshotEditEvent::ReplaceSource { source } = event {
        return editing::snapshot_from_edit_source(source).map_err(|error| pngEditor_edit_fault("stdio.png.invalid-source", error.to_string()));
    }
    if let SnapshotEditEvent::SetValue { path, value } = event {
        if path.is_empty() {
            return <PngSnapshot as dsl::FromValue>::from_value(value.clone()).map_err(|error| pngEditor_edit_fault("stdio.png.invalid-snapshot", error.to_string()));
        }
        if path == "/pixels" {
            let mut next = snapshot.clone();
            next.pixels = <Vec<u8> as dsl::FromValue>::from_value(value.clone()).map_err(|error| pngEditor_edit_fault("stdio.png.invalid-pixels", error.to_string()))?;
            return Ok(next);
        }
        if let Some(segment) = path.strip_prefix("/pixels/") {
            let mut next = snapshot.clone();
            let index = pngEditor_pixel_index(segment, next.pixels.len(), false)?;
            next.pixels[index] = pngEditor_pixel_value(value)?;
            return Ok(next);
        }
    }
    if let SnapshotEditEvent::InsertValue { path, value } = event {
        if let Some(segment) = path.strip_prefix("/pixels/") {
            let mut next = snapshot.clone();
            let index = pngEditor_pixel_index(segment, next.pixels.len(), true)?;
            next.pixels.insert(index, pngEditor_pixel_value(value)?);
            return Ok(next);
        }
    }
    if let SnapshotEditEvent::RemoveValue { path } = event {
        if let Some(segment) = path.strip_prefix("/pixels/") {
            let mut next = snapshot.clone();
            let index = pngEditor_pixel_index(segment, next.pixels.len(), false)?;
            next.pixels.remove(index);
            return Ok(next);
        }
    }
    if let SnapshotEditEvent::MoveValue { from, path } = event {
        if let (Some(from), Some(path)) = (from.strip_prefix("/pixels/"), path.strip_prefix("/pixels/")) {
            let mut next = snapshot.clone();
            let from = pngEditor_pixel_index(from, next.pixels.len(), false)?;
            let value = next.pixels.remove(from);
            let path = pngEditor_pixel_index(path, next.pixels.len(), true)?;
            next.pixels.insert(path, value);
            return Ok(next);
        }
    }
    let patch = editing::prepare_snapshot_patch(snapshot, event).map_err(|error| pngEditor_edit_fault(error.code, error.to_string()))?;
    editing::apply_snapshot_patch_for_dialect(snapshot, &patch, PNG_DIALECT, STDIO_PNG_DOCUMENT_SCHEMA).map_err(|error| pngEditor_edit_fault(error.code, error.to_string()))
}
fn pngEditor_metadata_mutation(next: &PngSnapshot, base: &PngSnapshot) -> Option<PngMutation> {
    let header_changed = next.width != base.width || next.height != base.height || next.bit_depth != base.bit_depth || next.color_type != base.color_type || next.interlace != base.interlace;
    let changes = [
        next.schema != base.schema,
        header_changed,
        next.plte != base.plte,
        next.trns != base.trns,
        next.gama != base.gama,
        next.chrm != base.chrm,
        next.srgb != base.srgb,
        next.phys != base.phys,
        next.time != base.time,
        next.bkgd != base.bkgd,
        next.text_chunks != base.text_chunks,
        next.pixels != base.pixels,
        next.chunk_order != base.chunk_order,
        next.unknown_chunks != base.unknown_chunks,
    ];
    if changes.into_iter().filter(|changed| *changed).count() != 1 { return None; }
    if header_changed { return Some(PngMutation::ChangeHeader(crate::schema::mutations::ChangeHeaderMutation { width: next.width, height: next.height, bit_depth: next.bit_depth, color_type: next.color_type, interlace: next.interlace })); }
    macro_rules! one_field {
        ($field:ident, $variant:ident, $payload:ident) => {
            if next.$field != base.$field { return Some(PngMutation::$variant(crate::schema::mutations::$payload { $field: next.$field.clone() })); }
        };
    }
    one_field!(plte, ReplacePalette, ReplacePaletteMutation);
    one_field!(trns, ChangeTransparency, ChangeTransparencyMutation);
    one_field!(gama, ChangeGamma, ChangeGammaMutation);
    one_field!(chrm, ChangeChromaticities, ChangeChromaticitiesMutation);
    one_field!(srgb, ChangeSrgbIntent, ChangeSrgbIntentMutation);
    one_field!(phys, ChangePhysicalDims, ChangePhysicalDimsMutation);
    one_field!(time, ChangeTimestamp, ChangeTimestampMutation);
    one_field!(bkgd, ChangeBackground, ChangeBackgroundMutation);
    if next.text_chunks != base.text_chunks {
        if next.text_chunks.len() == base.text_chunks.len() {
            let mut changed = next.text_chunks.iter().zip(&base.text_chunks).enumerate().filter(|(_, (next, base))| next != base);
            if let Some((index, (chunk, _))) = changed.next() {
                if changed.next().is_none() { return Some(PngMutation::ReplaceTextChunk(crate::schema::mutations::ReplaceTextChunkMutation { index, chunk: chunk.clone() })); }
            }
        } else if next.text_chunks.len() == base.text_chunks.len() + 1 {
            let index = base.text_chunks.iter().zip(&next.text_chunks).position(|(base, next)| base != next).unwrap_or(base.text_chunks.len());
            if base.text_chunks[index..] == next.text_chunks[index + 1..] { return Some(PngMutation::InsertTextChunk(crate::schema::mutations::InsertTextChunkMutation { index, chunk: next.text_chunks[index].clone() })); }
        } else if base.text_chunks.len() == next.text_chunks.len() + 1 {
            let index = base.text_chunks.iter().zip(&next.text_chunks).position(|(base, next)| base != next).unwrap_or(next.text_chunks.len());
            if base.text_chunks[index + 1..] == next.text_chunks[index..] { return Some(PngMutation::RemoveTextChunk(crate::schema::mutations::RemoveTextChunkMutation { index })); }
        }
    }
    if next.pixels != base.pixels { return Some(PngMutation::ReplacePixels(ReplacePixelsMutation { pixels: next.pixels.clone() })); }
    if next.unknown_chunks != base.unknown_chunks {
        if next.unknown_chunks.len() == base.unknown_chunks.len() + 1 {
            let index = base.unknown_chunks.iter().zip(&next.unknown_chunks).position(|(base, next)| base != next).unwrap_or(base.unknown_chunks.len());
            if base.unknown_chunks[index..] == next.unknown_chunks[index + 1..] { return Some(PngMutation::InsertUnknownChunk(crate::schema::mutations::InsertUnknownChunkMutation { index, chunk: next.unknown_chunks[index].clone() })); }
        } else if base.unknown_chunks.len() == next.unknown_chunks.len() + 1 {
            let index = base.unknown_chunks.iter().zip(&next.unknown_chunks).position(|(base, next)| base != next).unwrap_or(next.unknown_chunks.len());
            if base.unknown_chunks[index + 1..] == next.unknown_chunks[index..] { return Some(PngMutation::RemoveUnknownChunk(crate::schema::mutations::RemoveUnknownChunkMutation { index })); }
        }
    }
    None
}
struct PngDetailsProvider<'a> {
    snapshot: &'a PngSnapshot,
    metadata: editing::DslSnapshotDetailsProvider<'static, PngSnapshot>,
    pixels: &'a [u8],
}
impl<'a> PngDetailsProvider<'a> {
    fn new(snapshot: &'a PngSnapshot) -> Self {
        let metadata = PngSnapshot {
            schema: snapshot.schema.clone(),
            width: snapshot.width,
            height: snapshot.height,
            bit_depth: snapshot.bit_depth,
            color_type: snapshot.color_type,
            interlace: snapshot.interlace,
            plte: snapshot.plte.clone(),
            trns: snapshot.trns.clone(),
            gama: snapshot.gama,
            chrm: snapshot.chrm,
            srgb: snapshot.srgb,
            phys: snapshot.phys,
            time: snapshot.time,
            bkgd: snapshot.bkgd.clone(),
            text_chunks: snapshot.text_chunks.clone(),
            pixels: Vec::new(),
            chunk_order: snapshot.chunk_order.clone(),
            unknown_chunks: snapshot.unknown_chunks.clone(),
        };
        Self { snapshot, metadata: editing::DslSnapshotDetailsProvider::from_value(dsl::ToValue::to_value(&metadata)), pixels: &snapshot.pixels }
    }
}
impl editing::SnapshotDetailsProvider for PngDetailsProvider<'_> {
    fn value(&self, path: &[editing::SnapshotDetailPathSegment]) -> Option<editing::SnapshotDetailValue> {
        match path {
            [editing::SnapshotDetailPathSegment::Key(key), editing::SnapshotDetailPathSegment::Index(index)] if key == "pixels" => self.pixels.get(*index).map(|value| editing::SnapshotDetailValue::Number(dsl::Number::UInt((*value).into()))),
            _ => editing::SnapshotDetailsProvider::value(&self.metadata, path),
        }
    }
    fn child_count(&self, path: &[editing::SnapshotDetailPathSegment]) -> usize {
        match path {
            [editing::SnapshotDetailPathSegment::Key(key)] if key == "pixels" => self.pixels.len(),
            _ => editing::SnapshotDetailsProvider::child_count(&self.metadata, path),
        }
    }
    fn object_key(&self, path: &[editing::SnapshotDetailPathSegment], index: usize) -> Option<String> {
        editing::SnapshotDetailsProvider::object_key(&self.metadata, path, index)
    }
    fn creation_template(&self, path: &[editing::SnapshotDetailPathSegment], collection_item: bool) -> Option<dsl::DslValue> {
        editing::SnapshotDetailsProvider::creation_template(&self.metadata, path, collection_item)
    }
    fn missing_property_templates(&self, path: &[editing::SnapshotDetailPathSegment]) -> Vec<(String, dsl::DslValue)> {
        editing::SnapshotDetailsProvider::missing_property_templates(&self.metadata, path)
    }
    fn allows_untyped_creation(&self, path: &[editing::SnapshotDetailPathSegment]) -> bool {
        editing::SnapshotDetailsProvider::allows_untyped_creation(&self.metadata, path)
    }
    fn enum_values(&self, path: &[editing::SnapshotDetailPathSegment]) -> Vec<dsl::DslValue> {
        editing::SnapshotDetailsProvider::enum_values(&self.metadata, path)
    }
    fn has_source(&self) -> bool { self.snapshot.pixels.len() <= 512 * 1_024 }
    fn source(&self) -> Option<String> {
        let source = editing::snapshot_edit_source(self.snapshot);
        editing::snapshot_edit_source_is_admitted(&source).then_some(source)
    }
}
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

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.png@1.2/*#editor";
        let mut proofs = vec![ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngEditorExampleFactory", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "stdio.png", ToolExecutionContract::bounded_first_step(STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<PngEditor>, PngEditorExampleFactory>()];
        proofs.push(ArtifactBoundedFirstStepProof::new::<EditorApp<PngEditor>>(OWNER_FILE, CONTROLLER, "PngPixelRegionFactory", patch_pixel_region::ACTION_ID, "stdio.png", pngEditor_pixel_region_execution_contract()).with_factory_type::<EditorApp<PngEditor>, PngPixelRegionFactory>());
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, "stdio.png"));
        proofs
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(PngEditorExampleFactory::new(registry.controller_id()))?;
        registry.register(PngPixelRegionFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if request.tool_id == patch_pixel_region::ACTION_ID {
            if pngEditor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-png-pixel-region-tool-mismatch")); }
            let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
            let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, pngEditor_command_id, patch_pixel_region::MAXIMUM_RAW_BYTES, patch_pixel_region::CAPACITY.work_items(), Box::new(patch_pixel_region::PatchPixelRegionWork::default()))?;
            return Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        if !STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if pngEditor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, pngEditor_command_id, STDIO_PNG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, pngEditor_retained_reduce, pngEditor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_PNG_DOCUMENT_SCHEMA, operation, generation))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-png-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn command_id(command: &Self::Command) -> &'static str { pngEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> { pngEditor_command_from_action(action, args) }

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
            PngEditCommand::Edit(event) => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details_provider(&PngDetailsProvider::new(doc.snapshot), view_state.locale, "s.stdio.png@1.2/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
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
        let pixel_mutation = match event {
            editing::SnapshotEditEvent::SetValue { path, value } if path == "/pixels" => Some(PngMutation::ReplacePixels(ReplacePixelsMutation {
                pixels: <Vec<u8> as dsl::FromValue>::from_value(value.clone()).map_err(|error| pngEditor_edit_fault("stdio.png.invalid-pixels", error.to_string()))?,
            })),
            editing::SnapshotEditEvent::SetValue { path, value } if path.starts_with("/pixels/") => {
                let index = pngEditor_pixel_index(&path[8..], snapshot.pixels.len(), false)?;
                Some(PngMutation::PatchPixels(PatchPixelsMutation { index: index as u64, remove_count: 1, pixels: vec![pngEditor_pixel_value(value)?], move_to: None }))
            }
            editing::SnapshotEditEvent::InsertValue { path, value } if path.starts_with("/pixels/") => {
                let index = pngEditor_pixel_index(&path[8..], snapshot.pixels.len(), true)?;
                Some(PngMutation::PatchPixels(PatchPixelsMutation { index: index as u64, remove_count: 0, pixels: vec![pngEditor_pixel_value(value)?], move_to: None }))
            }
            editing::SnapshotEditEvent::RemoveValue { path } if path.starts_with("/pixels/") => {
                let index = pngEditor_pixel_index(&path[8..], snapshot.pixels.len(), false)?;
                Some(PngMutation::PatchPixels(PatchPixelsMutation { index: index as u64, remove_count: 1, pixels: Vec::new(), move_to: None }))
            }
            editing::SnapshotEditEvent::MoveValue { from, path } if from.starts_with("/pixels/") && path.starts_with("/pixels/") => {
                let from = pngEditor_pixel_index(&from[8..], snapshot.pixels.len(), false)?;
                let path = pngEditor_pixel_index(&path[8..], snapshot.pixels.len() - 1, true)?;
                Some(PngMutation::PatchPixels(PatchPixelsMutation { index: from as u64, remove_count: 0, pixels: Vec::new(), move_to: Some(path as u64) }))
            }
            _ => None,
        };
        if let Some(mutation) = pixel_mutation {
            return Ok(Emit { artifact_mutations: vec![mutation], description: Some("Edit PNG pixels".into()), ..Default::default() });
        }
        let next = pngEditor_snapshot_edit(event, snapshot)?;
        if let Some(mutation) = pngEditor_metadata_mutation(&next, snapshot) {
            return Ok(Emit { artifact_mutations: vec![mutation], description: Some("Edit PNG details".into()), ..Default::default() });
        }
        editing::snapshot_edit_patch(event, snapshot, |patch| PngMutation::PatchSnapshot(crate::schema::mutations::PatchSnapshot { patch }))
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
