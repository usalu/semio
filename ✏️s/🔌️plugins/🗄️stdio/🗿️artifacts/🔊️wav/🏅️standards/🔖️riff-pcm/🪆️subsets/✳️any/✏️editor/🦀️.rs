//! ✏️ `wav` editor (any) — `ArtifactEditor` surface built on the frozen
//! `MediaWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::wav::modes::edit;
use crate::editor::wav::modes::edit::windows::main;
use crate::standards::riff_pcm::subsets::any::schema::mutations::{patch_data, set_data, set_fmt, set_other_chunks, WavMutation};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{WavData, WavSnapshot};
use crate::{STDIO_WAV_DOCUMENT_SCHEMA, WAV_DIALECT};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{AppOperationContext, ArtifactBoundedFirstStepProof, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use store::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

#[path = "🎮️commands/🔊️edit-audio/🦀️.rs"]
pub(crate) mod edit_audio;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum WavEditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditAudio(edit_audio::EditAudio),
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for WavEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_WAV_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(pack::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = pack::parse_json_bytes(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "wav-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as dsl::FromValue>::from_value(pack::json_to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "wav-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


const STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_WAV_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    edit_audio::SET_SAMPLE_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    edit_audio::INSERT_FRAME_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    edit_audio::INSERT_CHANNEL_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
    edit_audio::SET_SAMPLE_RATE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.wav.tool-command.v1";
const STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn wavEditor_example_snapshot(example_id: &str) -> WavSnapshot {
    if example_id == crate::examples::demo::ID { <WavSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { WavSnapshot::default() }
}
fn wavEditor_command_id(command: &WavEditCommand) -> &'static str {
    match command {
        WavEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        WavEditCommand::EditAudio(command) => edit_audio::action_id(command),
        WavEditCommand::EditSnapshot { event } => event.action_id(),
    }
}
fn wavEditor_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<WavEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| WavEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(WavEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        action if edit_audio::TOOL_IDS.contains(&action) => Ok(WavEditCommand::EditAudio(edit_audio::from_action(action, args)?)),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn wavEditor_retained_extent(command: &WavEditCommand, _snapshot: &WavSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, WavEditCommand::SetActiveExample { .. }).then_some(1)
}
fn wavEditor_retained_reduce(command: &WavEditCommand, _snapshot: &WavSnapshot, _config: &NoConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<WavEditor>>>, _operation: &AppOperationContext) -> Result<Emit<WavMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        WavEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&wavEditor_example_snapshot(example_id), STDIO_WAV_DOCUMENT_SCHEMA)], description: Some(format!("Load example {example_id}")), ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn wavEditor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
fn wavEditor_sample_index(segment: &str, len: usize, insertion: bool) -> Result<usize, Fault> {
    if insertion && segment == "-" { return Ok(len); }
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(wavEditor_edit_fault("stdio.wav.invalid-sample-path", format!("sample index '{segment}' is not canonical")));
    }
    let index = segment.parse::<usize>().map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-sample-path", error.to_string()))?;
    if index > len || (!insertion && index == len) { return Err(wavEditor_edit_fault("stdio.wav.sample-index-out-of-range", format!("sample index {index} is outside 0..{len}"))); }
    Ok(index)
}
fn wavEditor_set_sample(data: &mut WavData, index: usize, value: &dsl::DslValue) -> Result<(), Fault> {
    let dsl::DslValue::Number(number) = value else { return Err(wavEditor_edit_fault("stdio.wav.invalid-sample", "sample must be numeric")); };
    match data {
        WavData::Pcm16(values) => values[index] = i16::try_from(number.as_i64().ok_or_else(|| wavEditor_edit_fault("stdio.wav.invalid-sample", "PCM16 sample must be an integer"))?).map_err(|_| wavEditor_edit_fault("stdio.wav.invalid-sample", "PCM16 sample is outside i16"))?,
        WavData::Pcm8(values) | WavData::Raw(values) => values[index] = u8::try_from(number.as_u64().ok_or_else(|| wavEditor_edit_fault("stdio.wav.invalid-sample", "byte sample must be an unsigned integer"))?).map_err(|_| wavEditor_edit_fault("stdio.wav.invalid-sample", "byte sample is outside u8"))?,
        WavData::Float32(values) => {
            let sample = number.as_f64();
            if !sample.is_finite() || sample < f32::MIN as f64 || sample > f32::MAX as f64 { return Err(wavEditor_edit_fault("stdio.wav.invalid-sample", "float sample is outside finite f32")); }
            values[index] = sample as f32;
        }
    }
    Ok(())
}
fn wavEditor_insert_sample(data: &mut WavData, index: usize, value: &dsl::DslValue) -> Result<(), Fault> {
    match data {
        WavData::Pcm16(values) => values.insert(index, 0),
        WavData::Pcm8(values) | WavData::Raw(values) => values.insert(index, 0),
        WavData::Float32(values) => values.insert(index, 0.0),
    }
    wavEditor_set_sample(data, index, value)
}
fn wavEditor_remove_sample(data: &mut WavData, index: usize) {
    match data { WavData::Pcm16(values) => { values.remove(index); }, WavData::Pcm8(values) | WavData::Raw(values) => { values.remove(index); }, WavData::Float32(values) => { values.remove(index); } }
}
fn wavEditor_move_sample(data: &mut WavData, from: usize, path: usize) {
    match data {
        WavData::Pcm16(values) => { let value = values.remove(from); values.insert(path, value); }
        WavData::Pcm8(values) | WavData::Raw(values) => { let value = values.remove(from); values.insert(path, value); }
        WavData::Float32(values) => { let value = values.remove(from); values.insert(path, value); }
    }
}
fn wavEditor_sample_len(data: &WavData) -> usize {
    match data { WavData::Pcm16(values) => values.len(), WavData::Pcm8(values) | WavData::Raw(values) => values.len(), WavData::Float32(values) => values.len() }
}
fn wavEditor_empty_data(data: &WavData) -> WavData {
    match data { WavData::Pcm16(_) => WavData::Pcm16(Vec::new()), WavData::Pcm8(_) => WavData::Pcm8(Vec::new()), WavData::Float32(_) => WavData::Float32(Vec::new()), WavData::Raw(_) => WavData::Raw(Vec::new()) }
}
fn wavEditor_single_sample(data: &WavData, value: &dsl::DslValue) -> Result<WavData, Fault> {
    let mut sample = wavEditor_empty_data(data);
    wavEditor_insert_sample(&mut sample, 0, value)?;
    Ok(sample)
}
fn wavEditor_direct_patch(event: &editing::SnapshotEditEvent, snapshot: &WavSnapshot) -> Result<Option<patch_data::PatchData>, Fault> {
    use editing::SnapshotEditEvent;
    let patch = match event {
        SnapshotEditEvent::SetValue { path, value } if path.starts_with("/data/value/") => {
            let index = wavEditor_sample_index(path.trim_start_matches("/data/value/"), wavEditor_sample_len(&snapshot.data), false)?;
            patch_data::PatchData { index: index as u64, remove_count: 1, data: wavEditor_single_sample(&snapshot.data, value)?, move_to: None }
        }
        SnapshotEditEvent::InsertValue { path, value } if path.starts_with("/data/value/") => {
            let index = wavEditor_sample_index(path.trim_start_matches("/data/value/"), wavEditor_sample_len(&snapshot.data), true)?;
            patch_data::PatchData { index: index as u64, remove_count: 0, data: wavEditor_single_sample(&snapshot.data, value)?, move_to: None }
        }
        SnapshotEditEvent::RemoveValue { path } if path.starts_with("/data/value/") => {
            let index = wavEditor_sample_index(path.trim_start_matches("/data/value/"), wavEditor_sample_len(&snapshot.data), false)?;
            patch_data::PatchData { index: index as u64, remove_count: 1, data: wavEditor_empty_data(&snapshot.data), move_to: None }
        }
        SnapshotEditEvent::MoveValue { from, path } if from.starts_with("/data/value/") && path.starts_with("/data/value/") => {
            let from = wavEditor_sample_index(from.trim_start_matches("/data/value/"), wavEditor_sample_len(&snapshot.data), false)?;
            let path = wavEditor_sample_index(path.trim_start_matches("/data/value/"), wavEditor_sample_len(&snapshot.data) - 1, true)?;
            patch_data::PatchData { index: from as u64, remove_count: 0, data: wavEditor_empty_data(&snapshot.data), move_to: Some(path as u64) }
        }
        _ => return Ok(None),
    };
    Ok(Some(patch))
}
fn wavEditor_snapshot_edit(event: &editing::SnapshotEditEvent, snapshot: &WavSnapshot) -> Result<WavSnapshot, Fault> {
    use editing::SnapshotEditEvent;
    if let SnapshotEditEvent::ReplaceSource { source } = event {
        return editing::snapshot_from_edit_source(source).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-source", error.to_string()));
    }
    if let SnapshotEditEvent::SetValue { path, value } = event {
        if path.is_empty() { return <WavSnapshot as dsl::FromValue>::from_value(value.clone()).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-snapshot", error.to_string())); }
        if path == "/data" {
            let mut next = snapshot.clone();
            next.data = <WavData as dsl::FromValue>::from_value(value.clone()).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-data", error.to_string()))?;
            return Ok(next);
        }
        if path == "/data/value" {
            let mut next = snapshot.clone();
            next.data = match &next.data {
                WavData::Pcm16(_) => WavData::Pcm16(<Vec<i16> as dsl::FromValue>::from_value(value.clone()).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-data", error.to_string()))?),
                WavData::Pcm8(_) => WavData::Pcm8(<Vec<u8> as dsl::FromValue>::from_value(value.clone()).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-data", error.to_string()))?),
                WavData::Float32(_) => WavData::Float32(<Vec<f32> as dsl::FromValue>::from_value(value.clone()).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-data", error.to_string()))?),
                WavData::Raw(_) => WavData::Raw(<Vec<u8> as dsl::FromValue>::from_value(value.clone()).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-data", error.to_string()))?),
            };
            return Ok(next);
        }
        if let Some(segment) = path.strip_prefix("/data/value/") {
            let mut next = snapshot.clone();
            let index = wavEditor_sample_index(segment, wavEditor_sample_len(&next.data), false)?;
            wavEditor_set_sample(&mut next.data, index, value)?;
            return Ok(next);
        }
    }
    if let SnapshotEditEvent::InsertValue { path, value } = event {
        if let Some(segment) = path.strip_prefix("/data/value/") {
            let mut next = snapshot.clone();
            let index = wavEditor_sample_index(segment, wavEditor_sample_len(&next.data), true)?;
            wavEditor_insert_sample(&mut next.data, index, value)?;
            return Ok(next);
        }
    }
    if let SnapshotEditEvent::RemoveValue { path } = event {
        if let Some(segment) = path.strip_prefix("/data/value/") {
            let mut next = snapshot.clone();
            let index = wavEditor_sample_index(segment, wavEditor_sample_len(&next.data), false)?;
            wavEditor_remove_sample(&mut next.data, index);
            return Ok(next);
        }
    }
    if let SnapshotEditEvent::MoveValue { from, path } = event {
        if let (Some(from), Some(path)) = (from.strip_prefix("/data/value/"), path.strip_prefix("/data/value/")) {
            let mut next = snapshot.clone();
            let from = wavEditor_sample_index(from, wavEditor_sample_len(&next.data), false)?;
            let path = wavEditor_sample_index(path, wavEditor_sample_len(&next.data) - 1, true)?;
            wavEditor_move_sample(&mut next.data, from, path);
            return Ok(next);
        }
    }
    let patch = editing::prepare_snapshot_patch(snapshot, event).map_err(|error| wavEditor_edit_fault(error.code, error.to_string()))?;
    editing::apply_snapshot_patch_for_dialect(snapshot, &patch, WAV_DIALECT, STDIO_WAV_DOCUMENT_SCHEMA).map_err(|error| wavEditor_edit_fault(error.code, error.to_string()))
}
struct WavDetailsProvider<'a> {
    snapshot: &'a WavSnapshot,
    metadata: editing::DslSnapshotDetailsProvider<'static, WavSnapshot>,
    data: &'a WavData,
}
impl<'a> WavDetailsProvider<'a> {
    fn new(snapshot: &'a WavSnapshot) -> Self {
        let mut metadata = snapshot.clone();
        metadata.data = wavEditor_empty_data(&snapshot.data);
        Self { snapshot, metadata: editing::DslSnapshotDetailsProvider::from_value(dsl::ToValue::to_value(&metadata)), data: &snapshot.data }
    }
}
impl editing::SnapshotDetailsProvider for WavDetailsProvider<'_> {
    fn value(&self, path: &[editing::SnapshotDetailPathSegment]) -> Option<editing::SnapshotDetailValue> {
        match path {
            [editing::SnapshotDetailPathSegment::Key(data), editing::SnapshotDetailPathSegment::Key(value), editing::SnapshotDetailPathSegment::Index(index)] if data == "data" && value == "value" => match self.data {
                WavData::Pcm16(values) => values.get(*index).map(|value| editing::SnapshotDetailValue::Number(dsl::Number::Int(i64::from(*value)))),
                WavData::Pcm8(values) | WavData::Raw(values) => values.get(*index).map(|value| editing::SnapshotDetailValue::Number(dsl::Number::UInt(u64::from(*value)))),
                WavData::Float32(values) => values.get(*index).map(|value| editing::SnapshotDetailValue::Number(dsl::Number::Float(f64::from(*value)))),
            },
            _ => editing::SnapshotDetailsProvider::value(&self.metadata, path),
        }
    }
    fn child_count(&self, path: &[editing::SnapshotDetailPathSegment]) -> usize {
        match path {
            [editing::SnapshotDetailPathSegment::Key(data), editing::SnapshotDetailPathSegment::Key(value)] if data == "data" && value == "value" => wavEditor_sample_len(self.data),
            _ => editing::SnapshotDetailsProvider::child_count(&self.metadata, path),
        }
    }
    fn object_key(&self, path: &[editing::SnapshotDetailPathSegment], index: usize) -> Option<String> { editing::SnapshotDetailsProvider::object_key(&self.metadata, path, index) }
    fn creation_template(&self, path: &[editing::SnapshotDetailPathSegment], collection_item: bool) -> Option<dsl::DslValue> { editing::SnapshotDetailsProvider::creation_template(&self.metadata, path, collection_item) }
    fn missing_property_templates(&self, path: &[editing::SnapshotDetailPathSegment]) -> Vec<(String, dsl::DslValue)> { editing::SnapshotDetailsProvider::missing_property_templates(&self.metadata, path) }
    fn allows_untyped_creation(&self, path: &[editing::SnapshotDetailPathSegment]) -> bool { editing::SnapshotDetailsProvider::allows_untyped_creation(&self.metadata, path) }
    fn enum_values(&self, path: &[editing::SnapshotDetailPathSegment]) -> Vec<dsl::DslValue> { editing::SnapshotDetailsProvider::enum_values(&self.metadata, path) }
    fn has_source(&self) -> bool { wavEditor_sample_len(self.data) <= 512 * 1_024 }
    fn source(&self) -> Option<String> {
        let source = editing::snapshot_edit_source(self.snapshot);
        editing::snapshot_edit_source_is_admitted(&source).then_some(source)
    }
}
struct WavEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl WavEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for WavEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<WavEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<WavEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for WavEditorExampleFactory {
    type Owner = EditorApp<WavEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_WAV_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
const WAV_AUDIO_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: edit_audio::SET_SAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: edit_audio::INSERT_FRAME_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: edit_audio::INSERT_CHANNEL_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: edit_audio::SET_SAMPLE_RATE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];
const fn wavEditor_audio_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(edit_audio::MAXIMUM_RAW_BYTES, 64, edit_audio::CAPACITY.work_items() as u64, 65_536, 7_500)
}
struct WavAudioFactory { keys: Vec<ToolFactoryKey> }
impl WavAudioFactory {
    fn new(controller_id: &str) -> Self { Self { keys: edit_audio::TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } }
}
impl ToolJobFactory for WavAudioFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<WavEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<WavEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { edit_audio::PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { wavEditor_audio_execution_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > edit_audio::MAXIMUM_RAW_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("WAV audio edit rejects oversized wire or checkpoint owner"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for WavAudioFactory {
    type Owner = EditorApp<WavEditor>;
    const TOOL_IDS: &'static [&'static str] = edit_audio::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_WAV_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = WAV_AUDIO_PUBLICATION_CONTRACTS;
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct WavEditor;

impl ArtifactEditor for WavEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = WavSnapshot;
    type Mutation = WavMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = WavEditCommand;

    const DIALECT: Dialect = WAV_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_WAV_DOCUMENT_SCHEMA;

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.wav@riff-pcm/*#editor";
        let mut proofs = vec![ArtifactBoundedFirstStepProof::new::<EditorApp<WavEditor>>(OWNER_FILE, CONTROLLER, "WavEditorExampleFactory", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "stdio.wav", ToolExecutionContract::bounded_first_step(STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<WavEditor>, WavEditorExampleFactory>()];
        proofs.extend(edit_audio::TOOL_IDS.iter().map(|tool_id| ArtifactBoundedFirstStepProof::new::<EditorApp<WavEditor>>(OWNER_FILE, CONTROLLER, "WavAudioFactory", *tool_id, "stdio.wav", wavEditor_audio_execution_contract()).with_factory_type::<EditorApp<WavEditor>, WavAudioFactory>()));
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, "stdio.wav"));
        proofs
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(WavEditorExampleFactory::new(registry.controller_id()))?;
        registry.register(WavAudioFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if edit_audio::TOOL_IDS.contains(&request.tool_id.as_str()) {
            if wavEditor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-wav-audio-tool-mismatch")); }
            let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
            let tool_id = wavEditor_command_id(&request.command);
            let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, wavEditor_command_id, edit_audio::MAXIMUM_RAW_BYTES, edit_audio::CAPACITY.work_items(), Box::new(edit_audio::EditAudioWork::new(tool_id)))?;
            return Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        if !STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if wavEditor_command_id(&request.command) != request.tool_id { return Err(Fault::from("stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, wavEditor_command_id, STDIO_WAV_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, wavEditor_retained_reduce, wavEditor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_WAV_DOCUMENT_SCHEMA, operation, generation))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-wav-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn command_id(command: &Self::Command) -> &'static str { wavEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> { wavEditor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        WavSnapshot::default()
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
            WavEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&wavEditor_example_snapshot(example_id), STDIO_WAV_DOCUMENT_SCHEMA)],
                description: Some(format!("Load example {example_id}")),
                ..Default::default()
            }),
            WavEditCommand::EditAudio(_) => Err(Fault::from("WAV natural audio editing requires the cancellable retained route")),
            WavEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision = doc.render_operation().map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision)).unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot));
                main::render_revisioned(doc.snapshot, &revision, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree)
            }
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details_provider(&WavDetailsProvider::new(doc.snapshot), view_state.locale, "s.stdio.wav@riff-pcm/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

impl editing::SnapshotEditingEditor for WavEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { WavEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(patch) = wavEditor_direct_patch(event, snapshot)? {
            return Ok(Emit { artifact_mutations: vec![WavMutation::PatchData(patch)], description: Some("Edit WAV samples".into()), ..Default::default() });
        }
        let next = wavEditor_snapshot_edit(event, snapshot)?;
        let mutation = match (
            next.fmt != snapshot.fmt,
            next.data != snapshot.data,
            next.fmt_pad_byte != snapshot.fmt_pad_byte,
            next.data_pad_byte != snapshot.data_pad_byte,
            next.other_chunks != snapshot.other_chunks,
            next.chunk_order != snapshot.chunk_order,
        ) {
            (true, false, false, false, false, false) => WavMutation::SetFmt(set_fmt::SetFmt { fmt: next.fmt }),
            (false, true, false, false, false, false) => WavMutation::SetData(set_data::SetData { data: next.data }),
            (false, false, false, false, true, false) => WavMutation::SetOtherChunks(set_other_chunks::SetOtherChunks { chunks: next.other_chunks }),
            _ => return editing::snapshot_edit_patch(event, snapshot, |patch| WavMutation::PatchSnapshot(crate::standards::riff_pcm::subsets::any::schema::mutations::patch_snapshot::PatchSnapshot { patch })),
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some("Edit WAV details".into()), ..Default::default() })
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_wav_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(WAV_DIALECT).document(["semio", "wav"]).icon_id("play").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
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
