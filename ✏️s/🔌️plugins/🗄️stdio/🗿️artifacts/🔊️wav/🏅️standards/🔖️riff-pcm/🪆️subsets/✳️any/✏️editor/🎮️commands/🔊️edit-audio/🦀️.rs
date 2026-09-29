//! 🔊️ Revision-guarded natural WAV sample, frame, channel, and format editing.

use super::{WavEditCommand, WavEditor};
use crate::standards::riff_pcm::subsets::any::schema::mutations::{patch_data, set_fmt, WavMutation};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{WavData, WavFmt, WavSnapshot};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, ArgSchema, EditorApp, Emit, Fault, FaultCode, FaultOrigin, LocalizedLabel};
use semio_s_artifact_stdio_contract::editing::RetainedBytesCopy;

pub const SET_SAMPLE_ACTION_ID: &str = "set-cell";
pub const INSERT_FRAME_ACTION_ID: &str = "insert-frame";
pub const INSERT_CHANNEL_ACTION_ID: &str = "insert-channel";
pub const SET_SAMPLE_RATE_ACTION_ID: &str = "set-sample-rate";
pub const PAYLOAD_SCHEMA: &str = "s.stdio.wav.command.edit-audio.v1";
pub const SCHEMA: &str = include_str!("🧬️schema/🔣️.json");
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
pub const PATCH_PAYLOAD_BYTES: usize = 16_384;
pub const CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(128);
pub const TOOL_IDS: &[&str] = &[
    SET_SAMPLE_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    INSERT_FRAME_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    INSERT_CHANNEL_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
    SET_SAMPLE_RATE_ACTION_ID,
];

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum EditAudio {
    SetSample { frame: u32, channel: u32, revision: String, value: String },
    AppendFrame { revision: String },
    InsertFrame { frame: u32, revision: String },
    RemoveFrame { frame: u32, revision: String },
    AppendChannel { revision: String },
    InsertChannel { channel: u32, revision: String },
    RemoveChannel { channel: u32, revision: String },
    SetSampleRate { revision: String, value: String },
}

fn fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

pub fn from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<EditAudio, Fault> {
    let revision = || semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision");
    match action {
        SET_SAMPLE_ACTION_ID => {
            let edit = semio_s_artifact_stdio_contract::window_kit_revisioned_cell_edit(args)?;
            Ok(EditAudio::SetSample { frame: edit.row, channel: edit.column, revision: edit.revision, value: edit.value })
        }
        semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID => Ok(EditAudio::AppendFrame { revision: revision()? }),
        INSERT_FRAME_ACTION_ID => Ok(EditAudio::InsertFrame { frame: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "row")?, revision: revision()? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID => Ok(EditAudio::RemoveFrame { frame: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "row")?, revision: revision()? }),
        semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID => Ok(EditAudio::AppendChannel { revision: revision()? }),
        INSERT_CHANNEL_ACTION_ID => Ok(EditAudio::InsertChannel { channel: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")?, revision: revision()? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID => Ok(EditAudio::RemoveChannel { channel: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")?, revision: revision()? }),
        SET_SAMPLE_RATE_ACTION_ID => Ok(EditAudio::SetSampleRate { revision: revision()?, value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")? }),
        _ => Err(fault("stdio.wav.audio-action", format!("unknown WAV audio action {action}"))),
    }
}

pub fn action_id(command: &EditAudio) -> &'static str {
    match command {
        EditAudio::SetSample { .. } => SET_SAMPLE_ACTION_ID,
        EditAudio::AppendFrame { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
        EditAudio::InsertFrame { .. } => INSERT_FRAME_ACTION_ID,
        EditAudio::RemoveFrame { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
        EditAudio::AppendChannel { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
        EditAudio::InsertChannel { .. } => INSERT_CHANNEL_ACTION_ID,
        EditAudio::RemoveChannel { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
        EditAudio::SetSampleRate { .. } => SET_SAMPLE_RATE_ACTION_ID,
    }
}

fn revision(command: &EditAudio) -> &str {
    match command {
        EditAudio::SetSample { revision, .. }
        | EditAudio::AppendFrame { revision }
        | EditAudio::InsertFrame { revision, .. }
        | EditAudio::RemoveFrame { revision, .. }
        | EditAudio::AppendChannel { revision }
        | EditAudio::InsertChannel { revision, .. }
        | EditAudio::RemoveChannel { revision, .. }
        | EditAudio::SetSampleRate { revision, .. } => revision,
    }
}

fn sample_len(data: &WavData) -> usize {
    match data {
        WavData::Pcm16(values) => values.len(),
        WavData::Pcm8(values) | WavData::Raw(values) => values.len(),
        WavData::Float32(values) => values.len(),
    }
}

fn sample_bytes(data: &WavData) -> Option<usize> {
    match data {
        WavData::Pcm8(_) => Some(1),
        WavData::Pcm16(_) => Some(2),
        WavData::Float32(_) => Some(4),
        WavData::Raw(_) => None,
    }
}

fn empty(data: &WavData) -> WavData {
    match data {
        WavData::Pcm8(_) => WavData::Pcm8(Vec::new()),
        WavData::Pcm16(_) => WavData::Pcm16(Vec::new()),
        WavData::Float32(_) => WavData::Float32(Vec::new()),
        WavData::Raw(_) => WavData::Raw(Vec::new()),
    }
}

fn zeros(data: &WavData, len: usize) -> WavData {
    match data {
        WavData::Pcm8(_) => WavData::Pcm8(vec![128; len]),
        WavData::Pcm16(_) => WavData::Pcm16(vec![0; len]),
        WavData::Float32(_) => WavData::Float32(vec![0.0; len]),
        WavData::Raw(_) => WavData::Raw(Vec::new()),
    }
}

#[derive(Clone, Copy, Debug)]
enum SampleValue {
    Pcm8(u8),
    Pcm16(i16),
    Float32(f32),
}

impl SampleValue {
    fn data(self) -> WavData {
        match self {
            Self::Pcm8(value) => WavData::Pcm8(vec![value]),
            Self::Pcm16(value) => WavData::Pcm16(vec![value]),
            Self::Float32(value) => WavData::Float32(vec![value]),
        }
    }
}

fn one_sample(data: &WavData, value: &str) -> Result<SampleValue, Fault> {
    match data {
        WavData::Pcm8(_) => value.parse::<u8>().map(SampleValue::Pcm8).map_err(|_| fault("stdio.wav.sample-range", "PCM8 sample must be an integer from 0 through 255")),
        WavData::Pcm16(_) => value.parse::<i16>().map(SampleValue::Pcm16).map_err(|_| fault("stdio.wav.sample-range", "PCM16 sample must be an integer from -32768 through 32767")),
        WavData::Float32(_) => {
            let value = value.parse::<f32>().map_err(|_| fault("stdio.wav.sample-range", "Float32 sample must be numeric"))?;
            value.is_finite().then_some(SampleValue::Float32(value)).ok_or_else(|| fault("stdio.wav.sample-range", "Float32 sample must be finite"))
        }
        WavData::Raw(_) => Err(fault("stdio.wav.raw-samples", "Raw WAV payloads do not have a safe frame or channel interpretation")),
    }
}

fn rewrite_channels(data: &WavData, start_frame: usize, frame_count: usize, old_channels: usize, channel: usize, insert: bool) -> WavData {
    macro_rules! rewrite {
        ($values:expr, $zero:expr, $variant:ident) => {{
            let source = &$values[start_frame * old_channels..(start_frame + frame_count) * old_channels];
            let mut output = Vec::with_capacity(if insert { frame_count * (old_channels + 1) } else { frame_count * (old_channels - 1) });
            for frame in source.chunks_exact(old_channels) {
                if insert {
                    output.extend_from_slice(&frame[..channel]);
                    output.push($zero);
                    output.extend_from_slice(&frame[channel..]);
                } else {
                    output.extend_from_slice(&frame[..channel]);
                    output.extend_from_slice(&frame[channel + 1..]);
                }
            }
            WavData::$variant(output)
        }};
    }
    match data {
        WavData::Pcm8(values) => rewrite!(values, 128, Pcm8),
        WavData::Pcm16(values) => rewrite!(values, 0, Pcm16),
        WavData::Float32(values) => rewrite!(values, 0.0, Float32),
        WavData::Raw(_) => WavData::Raw(Vec::new()),
    }
}

pub(crate) fn coherent_audio(snapshot: &WavSnapshot) -> Result<(usize, usize, usize), Fault> {
    let bytes = sample_bytes(&snapshot.data).ok_or_else(|| fault("stdio.wav.raw-samples", "Raw WAV payloads remain editable in Details but cannot be addressed as sample frames"))?;
    let channels = usize::from(snapshot.fmt.channels);
    if channels == 0 {
        return Err(fault("stdio.wav.zero-channels", "WAV channel count must be positive"));
    }
    let len = sample_len(&snapshot.data);
    if len % channels != 0 {
        return Err(fault("stdio.wav.partial-frame", format!("WAV has {len} samples, which is not divisible by {channels} channels")));
    }
    let expected_format = match &snapshot.data {
        WavData::Float32(_) => 3,
        WavData::Pcm8(_) | WavData::Pcm16(_) => 1,
        WavData::Raw(_) => unreachable!(),
    };
    let expected_bits = (bytes * 8) as u16;
    let expected_block_align = channels.checked_mul(bytes).and_then(|value| u16::try_from(value).ok()).ok_or_else(|| fault("stdio.wav.block-align-overflow", "WAV channel layout exceeds the u16 block alignment"))?;
    let expected_byte_rate =
        snapshot.fmt.sample_rate.checked_mul(u32::from(expected_block_align)).filter(|_| snapshot.fmt.sample_rate > 0).ok_or_else(|| fault("stdio.wav.byte-rate-overflow", "WAV sample rate and channel layout exceed the u32 byte rate"))?;
    if snapshot.fmt.audio_format != expected_format || snapshot.fmt.bits_per_sample != expected_bits || snapshot.fmt.block_align != expected_block_align || snapshot.fmt.byte_rate != expected_byte_rate {
        return Err(fault("stdio.wav.format-data-mismatch", "Natural sample editing requires format metadata matching the typed data and channel layout"));
    }
    Ok((channels, len / channels, bytes))
}

#[derive(Clone, Copy, Debug)]
struct FormatUpdate {
    channels: u16,
    sample_rate: u32,
    block_align: u16,
    byte_rate: u32,
}

fn updated_format(channels: usize, sample_bytes: usize, sample_rate: u32) -> Result<FormatUpdate, Fault> {
    if sample_rate == 0 {
        return Err(fault("stdio.wav.sample-rate", "Sample rate must be a positive integer in Hz"));
    }
    let block_align = channels.checked_mul(sample_bytes).and_then(|value| u16::try_from(value).ok()).ok_or_else(|| fault("stdio.wav.block-align-overflow", "WAV channel layout exceeds the u16 block alignment"))?;
    let byte_rate = sample_rate.checked_mul(u32::from(block_align)).ok_or_else(|| fault("stdio.wav.byte-rate-overflow", "WAV sample rate and channel layout exceed the u32 byte rate"))?;
    Ok(FormatUpdate { channels: u16::try_from(channels).map_err(|_| fault("stdio.wav.channel-overflow", "WAV channel count exceeds u16"))?, sample_rate, block_align, byte_rate })
}

fn build_fmt(source: &WavFmt, update: FormatUpdate, ext: Option<Vec<u8>>) -> WavFmt {
    WavFmt { audio_format: source.audio_format, channels: update.channels, sample_rate: update.sample_rate, byte_rate: update.byte_rate, block_align: update.block_align, bits_per_sample: source.bits_per_sample, ext }
}

#[derive(Clone, Debug)]
enum PlanKind {
    SetSample { index: usize, value: SampleValue },
    InsertSamples { index: usize, count: usize, chunk: usize },
    RemoveSamples { index: usize, count: usize, chunk: usize },
    RewriteChannels { channel: usize, old_channels: usize, frame_count: usize, frames_per_chunk: usize, chunks: usize, insert: bool, format: FormatUpdate },
    SetFormat(FormatUpdate),
}

#[derive(Clone, Debug)]
struct AudioPlan {
    kind: PlanKind,
    items: usize,
}

impl AudioPlan {
    fn new(command: &EditAudio, snapshot: &WavSnapshot) -> Result<Self, Fault> {
        let (channels, frames, bytes) = coherent_audio(snapshot)?;
        let samples_per_chunk = (PATCH_PAYLOAD_BYTES / bytes).max(1);
        let kind = match command {
            EditAudio::SetSample { frame, channel, value, .. } => {
                let frame = *frame as usize;
                let channel = *channel as usize;
                if frame >= frames || channel >= channels {
                    return Err(fault("stdio.wav.sample-stale", format!("Sample {frame},{channel} is outside {frames} frames and {channels} channels")));
                }
                PlanKind::SetSample { index: frame * channels + channel, value: one_sample(&snapshot.data, value)? }
            }
            EditAudio::AppendFrame { .. } => PlanKind::InsertSamples { index: frames * channels, count: channels, chunk: samples_per_chunk },
            EditAudio::InsertFrame { frame, .. } => {
                let frame = *frame as usize;
                if frame > frames {
                    return Err(fault("stdio.wav.frame-stale", format!("Frame {frame} is outside insertion range 0..={frames}")));
                }
                PlanKind::InsertSamples { index: frame * channels, count: channels, chunk: samples_per_chunk }
            }
            EditAudio::RemoveFrame { frame, .. } => {
                let frame = *frame as usize;
                if frame >= frames {
                    return Err(fault("stdio.wav.frame-stale", format!("Frame {frame} no longer exists")));
                }
                PlanKind::RemoveSamples { index: frame * channels, count: channels, chunk: samples_per_chunk }
            }
            EditAudio::AppendChannel { .. } | EditAudio::InsertChannel { .. } | EditAudio::RemoveChannel { .. } => {
                let (channel, insert) = match command {
                    EditAudio::AppendChannel { .. } => (channels, true),
                    EditAudio::InsertChannel { channel, .. } => (*channel as usize, true),
                    EditAudio::RemoveChannel { channel, .. } => (*channel as usize, false),
                    _ => unreachable!(),
                };
                if (insert && channel > channels) || (!insert && channel >= channels) {
                    return Err(fault("stdio.wav.channel-stale", format!("Channel {channel} is outside the current {channels}-channel layout")));
                }
                if !insert && channels == 1 {
                    return Err(fault("stdio.wav.last-channel", "A WAV document must retain at least one channel"));
                }
                let new_channels = if insert { channels + 1 } else { channels - 1 };
                let widest = channels.max(new_channels);
                if widest.checked_mul(bytes).is_none_or(|frame_bytes| frame_bytes > PATCH_PAYLOAD_BYTES) {
                    return Err(fault("stdio.wav.frame-too-wide", format!("One transformed frame needs more than the {PATCH_PAYLOAD_BYTES}-byte retained patch envelope")));
                }
                let frames_per_chunk = samples_per_chunk / widest;
                let chunks = frames.div_ceil(frames_per_chunk);
                let format = updated_format(new_channels, bytes, snapshot.fmt.sample_rate)?;
                PlanKind::RewriteChannels { channel, old_channels: channels, frame_count: frames, frames_per_chunk, chunks, insert, format }
            }
            EditAudio::SetSampleRate { value, .. } => {
                let sample_rate = value.parse::<u32>().ok().filter(|value| *value > 0).ok_or_else(|| fault("stdio.wav.sample-rate", "Sample rate must be a positive integer in Hz"))?;
                PlanKind::SetFormat(updated_format(channels, bytes, sample_rate)?)
            }
        };
        let items = match &kind {
            PlanKind::SetSample { .. } | PlanKind::SetFormat(_) => 1,
            PlanKind::InsertSamples { count, chunk, .. } | PlanKind::RemoveSamples { count, chunk, .. } => count.div_ceil(*chunk),
            PlanKind::RewriteChannels { chunks, .. } => chunks + 1,
        };
        CAPACITY.rows_for_items(items).ok_or_else(|| fault("stdio.wav.audio-edit-too-large", format!("Audio edit needs {items} retained mutations; maximum is {}", CAPACITY.invertible_items())))?;
        Ok(Self { kind, items })
    }

    fn requires_format(&self) -> bool {
        matches!(self.kind, PlanKind::RewriteChannels { .. } | PlanKind::SetFormat(_))
    }

    fn mutation(&self, ordinal: usize, snapshot: &WavSnapshot, format_ext: &mut Option<Option<Vec<u8>>>) -> Option<WavMutation> {
        match &self.kind {
            PlanKind::SetSample { index, value } if ordinal == 0 => Some(WavMutation::PatchData(patch_data::PatchData { index: *index as u64, remove_count: 1, data: value.data(), move_to: None })),
            PlanKind::InsertSamples { index, count, chunk } => {
                let offset = ordinal * chunk;
                (offset < *count).then(|| WavMutation::PatchData(patch_data::PatchData { index: (*index + offset) as u64, remove_count: 0, data: zeros(&snapshot.data, (*chunk).min(*count - offset)), move_to: None }))
            }
            PlanKind::RemoveSamples { index, count, chunk } => {
                let offset = ordinal * chunk;
                (offset < *count).then(|| WavMutation::PatchData(patch_data::PatchData { index: *index as u64, remove_count: (*chunk).min(*count - offset) as u64, data: empty(&snapshot.data), move_to: None }))
            }
            PlanKind::RewriteChannels { channel, old_channels, frame_count, frames_per_chunk, chunks, insert, format } => {
                if ordinal == *chunks {
                    return format_ext.take().map(|ext| WavMutation::SetFmt(set_fmt::SetFmt { fmt: build_fmt(&snapshot.fmt, *format, ext) }));
                }
                if ordinal > *chunks {
                    return None;
                }
                let reverse_chunk = chunks - 1 - ordinal;
                let start_frame = reverse_chunk * frames_per_chunk;
                let local_frames = (*frames_per_chunk).min(frame_count - start_frame);
                let data = rewrite_channels(&snapshot.data, start_frame, local_frames, *old_channels, *channel, *insert);
                Some(WavMutation::PatchData(patch_data::PatchData { index: (start_frame * old_channels) as u64, remove_count: (local_frames * old_channels) as u64, data, move_to: None }))
            }
            PlanKind::SetFormat(format) if ordinal == 0 => format_ext.take().map(|ext| WavMutation::SetFmt(set_fmt::SetFmt { fmt: build_fmt(&snapshot.fmt, *format, ext) })),
            _ => None,
        }
    }
}

fn number_arg(id: &'static str, en: &'static str, de: &'static str, minimum: f64, maximum: f64) -> ActionArgDef {
    let mut argument = ActionArgDef::number(id, LocalizedLabel::native(en, de)).required();
    if let ArgSchema::Number { min, max, step, integer, .. } = &mut argument.schema {
        *min = Some(minimum);
        *max = Some(maximum);
        *step = Some(1.0);
        *integer = true;
    }
    argument
}

fn revision_arg() -> ActionArgDef {
    ActionArgDef::document_revision("revision", LocalizedLabel::native("Document revision", "Dokumentrevision"))
}

pub fn extra_actions() -> Vec<ActionDefinition> {
    [
        ActionDefinition::bounded_catalog(INSERT_FRAME_ACTION_ID, LocalizedLabel::native("Insert frame", "Frame einfügen"), ActionKind::Mutation)
            .with_args([number_arg("row", "Frame", "Frame", 0.0, u32::MAX as f64), revision_arg()])
            .in_palette(false),
        ActionDefinition::bounded_catalog(INSERT_CHANNEL_ACTION_ID, LocalizedLabel::native("Insert channel", "Kanal einfügen"), ActionKind::Mutation)
            .with_args([number_arg("column", "Channel", "Kanal", 0.0, u16::MAX as f64), revision_arg()])
            .in_palette(false),
        ActionDefinition::bounded_catalog(SET_SAMPLE_RATE_ACTION_ID, LocalizedLabel::native("Set sample rate", "Abtastrate setzen"), ActionKind::Mutation)
            .with_args([revision_arg(), ActionArgDef::text("value", LocalizedLabel::native("Sample rate (Hz)", "Abtastrate (Hz)")).required()])
            .in_palette(false),
    ]
    .map(|mut action| {
        action.semantics.execution.interactive_job = semio_framework_plugin::InteractiveJobClassification::Migrated;
        action
    })
    .into()
}

fn data_payload_len(data: &WavData) -> usize {
    match data {
        WavData::Pcm8(values) | WavData::Raw(values) => values.len(),
        WavData::Pcm16(values) => values.len() * std::mem::size_of::<i16>(),
        WavData::Float32(values) => values.len() * std::mem::size_of::<f32>(),
    }
}

fn retire_data_payload(data: &mut WavData, maximum_bytes: usize) -> usize {
    let retire = |len: usize, width: usize| len.min(maximum_bytes / width);
    match data {
        WavData::Pcm8(values) | WavData::Raw(values) => {
            let count = retire(values.len(), 1);
            values.truncate(values.len() - count);
            count
        }
        WavData::Pcm16(values) => {
            let count = retire(values.len(), std::mem::size_of::<i16>());
            values.truncate(values.len() - count);
            count * std::mem::size_of::<i16>()
        }
        WavData::Float32(values) => {
            let count = retire(values.len(), std::mem::size_of::<f32>());
            values.truncate(values.len() - count);
            count * std::mem::size_of::<f32>()
        }
    }
}

fn mutation_payload_len(mutation: &WavMutation) -> usize {
    match mutation {
        WavMutation::PatchData(patch_data::PatchData { data, .. }) => data_payload_len(data),
        WavMutation::SetFmt(set_fmt::SetFmt { fmt }) => fmt.ext.as_ref().map_or(0, Vec::len),
        _ => 0,
    }
}

fn retire_mutation_payload(mutation: &mut WavMutation, maximum_bytes: usize) -> usize {
    match mutation {
        WavMutation::PatchData(patch_data::PatchData { data, .. }) => retire_data_payload(data, maximum_bytes),
        WavMutation::SetFmt(set_fmt::SetFmt { fmt }) => {
            let Some(ext) = &mut fmt.ext else { return 0 };
            let count = maximum_bytes.min(ext.len());
            ext.truncate(ext.len() - count);
            count
        }
        _ => 0,
    }
}

#[derive(Default)]
pub struct EditAudioWork {
    tool_id: Option<&'static str>,
    plan: Option<AudioPlan>,
    format_ext_copy: RetainedBytesCopy,
    format_ext: Option<Option<Vec<u8>>>,
    cursor: usize,
    mutations: Vec<WavMutation>,
    complete: bool,
    closing: bool,
}

impl EditAudioWork {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id: Some(tool_id), ..Self::default() }
    }
}

impl ArtifactCommandWork<EditorApp<WavEditor>> for EditAudioWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id.unwrap_or(SET_SAMPLE_ACTION_ID)
    }

    fn extent(&self, command: &WavEditCommand, snapshot: &WavSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<WavEditor>>>) -> Option<usize> {
        let WavEditCommand::EditAudio(command) = command else { return None };
        let plan = AudioPlan::new(command, snapshot).ok()?;
        CAPACITY.rows_for_items(plan.items)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<WavEditor>>) -> Result<ArtifactCommandWorkStep<EditorApp<WavEditor>>, Fault> {
        if self.closing || self.complete {
            return Err(fault("stdio.wav.audio-work-closed", "WAV audio edit work is already closed"));
        }
        let WavEditCommand::EditAudio(command) = input.command else {
            return Err(fault("stdio.wav.audio-route-mismatch", "WAV audio edit work received another command"));
        };
        if action_id(command) != self.tool_id() {
            return Err(fault("stdio.wav.audio-tool-mismatch", "WAV audio command does not match its retained tool"));
        }
        if self.plan.is_none() {
            let canonical = semio_s_artifact_stdio_contract::window_kit_canonical_revision(input.operation.canonical_base_revision);
            if revision(command) != canonical {
                return Err(fault("stdio.wav.audio-conflict", "The WAV document changed before this audio edit was applied"));
            }
            self.plan = Some(AudioPlan::new(command, input.snapshot)?);
            return Ok(ArtifactCommandWorkStep::Progress { stage: "wav-audio-prepare", preview: br#"{"en":"Preparing audio edit","de":"Audiobearbeitung wird vorbereitet"}"# });
        }
        let plan = self.plan.as_ref().expect("audio plan prepared");
        if plan.requires_format() && self.format_ext.is_none() {
            match input.snapshot.fmt.ext.as_deref() {
                None => self.format_ext = Some(None),
                Some(source) => {
                    self.format_ext_copy.advance(source, PATCH_PAYLOAD_BYTES).map_err(|message| fault("stdio.wav.format-copy", message))?;
                    if self.format_ext_copy.is_complete() {
                        self.format_ext = Some(self.format_ext_copy.take());
                    }
                }
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "wav-audio-format", preview: br#"{"en":"Preserving audio format metadata","de":"Audioformat-Metadaten werden erhalten"}"# });
        }
        if self.cursor < plan.items {
            if let Some(mutation) = plan.mutation(self.cursor, input.snapshot, &mut self.format_ext) {
                self.mutations.push(mutation);
            }
            self.cursor += 1;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "wav-audio-edit", preview: br#"{"en":"Editing audio frames","de":"Audioframes werden bearbeitet"}"# });
        }
        self.complete = true;
        let mutations = std::mem::take(&mut self.mutations);
        Ok(ArtifactCommandWorkStep::Complete(Emit { description: (!mutations.is_empty()).then(|| action_id(command).to_string()), artifact_mutations: mutations, ..Default::default() }))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if let Some(mutation) = self.mutations.last_mut() {
            let released_bytes = retire_mutation_payload(mutation, maximum_bytes);
            if released_bytes > 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes };
            }
            if mutation_payload_len(mutation) > 0 || maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.mutations.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if !self.format_ext_copy.terminal_is_empty() {
            return self.format_ext_copy.close_step(maximum_items, maximum_bytes);
        }
        if let Some(Some(ext)) = &mut self.format_ext {
            if !ext.is_empty() {
                if maximum_bytes == 0 {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let released_bytes = maximum_bytes.min(ext.len());
                ext.truncate(ext.len() - released_bytes);
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes };
            }
        }
        if self.format_ext.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.format_ext = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.plan.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.plan = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.plan.is_none() && self.format_ext_copy.terminal_is_empty() && self.format_ext.is_none() && self.mutations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff};

    fn apply_all(base: &WavSnapshot, mutations: &[WavMutation]) -> WavSnapshot {
        mutations.iter().fold(base.clone(), |snapshot, mutation| MutationDiff::apply(<WavMutation as Mutation<WavSnapshot>>::diff(mutation, &snapshot).diff(), &snapshot).expect("planned mutation applies"))
    }

    #[test]
    fn channel_rewrite_is_chunked_reversible_and_format_coherent() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🎚️natural-edit/🔣️.json")).expect("neutral audio-edit fixture parses");
        let base: WavSnapshot = dsl::json::from_json_str(&serde_json::to_string(&fixture["before"]).expect("fixture before prints")).expect("fixture before decodes");
        let command = EditAudio::InsertChannel { channel: fixture["insertChannel"]["channel"].as_u64().expect("channel") as u32, revision: "rev".into() };
        let plan = AudioPlan::new(&command, &base).expect("channel insertion plans");
        let mut format_ext = Some(base.fmt.ext.clone());
        let mutations = (0..plan.items).filter_map(|ordinal| plan.mutation(ordinal, &base, &mut format_ext)).collect::<Vec<_>>();
        let mut edited = base.clone();
        let mut inverses = Vec::new();
        for mutation in &mutations {
            inverses.push(<WavMutation as Mutation<WavSnapshot>>::inverse(mutation, &edited));
            edited = apply_all(&edited, std::slice::from_ref(mutation));
        }
        let expected_data = fixture["insertChannel"]["expectedData"].as_array().expect("expected data").iter().map(|value| value.as_i64().expect("sample") as i16).collect::<Vec<_>>();
        assert_eq!(edited.data, WavData::Pcm16(expected_data.clone()));
        assert_eq!(
            (edited.fmt.channels, edited.fmt.block_align, edited.fmt.byte_rate),
            (
                fixture["insertChannel"]["expectedChannels"].as_u64().expect("channels") as u16,
                fixture["insertChannel"]["expectedBlockAlign"].as_u64().expect("block align") as u16,
                fixture["insertChannel"]["expectedByteRate"].as_u64().expect("byte rate") as u32,
            ),
        );
        let native = crate::standards::riff_pcm::subsets::any::io::try_encode_wav(&edited).expect("edited WAV encodes");
        let mut cursor = std::io::Cursor::new(&native);
        let riff = riff::Chunk::read(&mut cursor, 0).expect("independent RIFF oracle reads edited WAVE");
        let chunks = riff.iter(&mut cursor).collect::<Result<Vec<_>, _>>().expect("independent RIFF oracle walks edited chunks");
        let fmt = chunks.iter().find(|chunk| chunk.id().as_str() == "fmt ").expect("fmt chunk").read_contents(&mut cursor).expect("fmt payload");
        let data = chunks.iter().find(|chunk| chunk.id().as_str() == "data").expect("data chunk").read_contents(&mut cursor).expect("data payload");
        assert_eq!(u16::from_le_bytes([fmt[2], fmt[3]]), edited.fmt.channels);
        assert_eq!(u32::from_le_bytes([fmt[8], fmt[9], fmt[10], fmt[11]]), edited.fmt.byte_rate);
        assert_eq!(u16::from_le_bytes([fmt[12], fmt[13]]), edited.fmt.block_align);
        assert_eq!(&fmt[18..], &[170, 187, 204]);
        assert_eq!(data, expected_data.into_iter().flat_map(i16::to_le_bytes).collect::<Vec<_>>());
        assert_eq!(crate::standards::riff_pcm::subsets::any::io::decode_wav(&native).expect("edited WAV reopens"), edited);
        let mut restored = edited;
        for batch in inverses.into_iter().rev() {
            for inverse in batch {
                restored = apply_all(&restored, &[inverse]);
            }
        }
        assert_eq!(restored, base);
        let schema: serde_json::Value = serde_json::from_str(SCHEMA).expect("command schema parses");
        assert_eq!(schema["$id"], PAYLOAD_SCHEMA);
    }

    #[test]
    fn retained_channel_edit_reports_localized_progress_refuses_stale_and_retires_incrementally() {
        use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};

        let mut fmt = WavFmt::default();
        fmt.channels = 2;
        fmt.block_align = 4;
        fmt.byte_rate = fmt.sample_rate * u32::from(fmt.block_align);
        let snapshot = WavSnapshot { fmt, data: WavData::Pcm16(vec![7; 40_000]), ..WavSnapshot::default() };
        let config = semio_framework_plugin::NoConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let operation = semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "wav-audio-retained".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "authoring-seed-test".into() };
        let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision);
        let command = WavEditCommand::EditAudio(EditAudio::InsertChannel { channel: 1, revision });
        let input = ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
        let mut work = EditAudioWork::new(INSERT_CHANNEL_ACTION_ID);
        assert!(work.extent(&command, &snapshot, &interaction, None).is_some_and(|extent| extent > 2));
        for expected_stage in ["wav-audio-prepare", "wav-audio-format", "wav-audio-edit"] {
            let ArtifactCommandWorkStep::Progress { stage, preview } = work.step(&input).expect("retained audio step") else { panic!("audio edit must report progress") };
            assert_eq!(stage, expected_stage);
            let preview = std::str::from_utf8(preview).expect("localized progress");
            assert!(preview.contains("en") && preview.contains("de"));
        }
        work.begin_close();
        assert!(matches!(work.close_step(1, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(!work.terminal_is_empty());
        while !work.terminal_is_empty() {
            assert!(!matches!(work.close_step(1, PATCH_PAYLOAD_BYTES), InteractiveJobCloseStep::Blocked));
        }
        assert_eq!(snapshot.data, WavData::Pcm16(vec![7; 40_000]));

        let stale = WavEditCommand::EditAudio(EditAudio::InsertChannel { channel: 1, revision: "stale".into() });
        let stale_input = ArtifactCommandInputs { command: &stale, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
        assert!(EditAudioWork::new(INSERT_CHANNEL_ACTION_ID).step(&stale_input).is_err());
    }

    #[test]
    fn addressed_actions_require_exact_frame_channel_and_revision_arguments() {
        let revision = "0123456789abcdef";
        let insert_frame = dsl::DslValue::Object(vec![("row".into(), dsl::DslValue::uint(7)), ("revision".into(), dsl::DslValue::String(revision.into()))]);
        assert_eq!(from_action(INSERT_FRAME_ACTION_ID, Some(&insert_frame)).expect("addressed frame action"), EditAudio::InsertFrame { frame: 7, revision: revision.into() });
        let remove_channel = dsl::DslValue::Object(vec![("column".into(), dsl::DslValue::uint(3)), ("revision".into(), dsl::DslValue::String(revision.into()))]);
        assert_eq!(
            from_action(semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID, Some(&remove_channel)).expect("addressed channel action"),
            EditAudio::RemoveChannel { channel: 3, revision: revision.into() }
        );
        let sample = dsl::DslValue::Object(vec![
            ("row".into(), dsl::DslValue::uint(11)),
            ("column".into(), dsl::DslValue::uint(5)),
            ("revision".into(), dsl::DslValue::String(revision.into())),
            ("value".into(), dsl::DslValue::String("-8".into())),
        ]);
        assert_eq!(
            from_action(SET_SAMPLE_ACTION_ID, Some(&sample)).expect("addressed sample action"),
            EditAudio::SetSample { frame: 11, channel: 5, revision: revision.into(), value: "-8".into() }
        );
        assert!(extra_actions().iter().all(|action| !action.in_palette));
    }

    #[test]
    fn channel_rewrite_refuses_a_frame_wider_than_one_retained_patch() {
        let channels = PATCH_PAYLOAD_BYTES / std::mem::size_of::<i16>();
        assert!(channels < usize::from(u16::MAX));
        let block_align = u16::try_from(channels * std::mem::size_of::<i16>()).expect("bounded block alignment");
        let snapshot = WavSnapshot {
            fmt: WavFmt { channels: channels as u16, sample_rate: 1, byte_rate: u32::from(block_align), block_align, ..WavFmt::default() },
            data: WavData::Pcm16(vec![1; channels]),
            ..WavSnapshot::default()
        };
        let error = AudioPlan::new(&EditAudio::AppendChannel { revision: "r".into() }, &snapshot).expect_err("the inserted channel makes one frame wider than a retained patch");
        assert_eq!(error.code.0, "stdio.wav.frame-too-wide");
    }

    #[test]
    fn wide_frame_removal_pages_at_one_stable_index_and_is_exactly_reversible() {
        let channels = PATCH_PAYLOAD_BYTES / std::mem::size_of::<i16>() + 1;
        let block_align = u16::try_from(channels * std::mem::size_of::<i16>()).expect("bounded block alignment");
        let mut values = vec![1; channels];
        values.extend(vec![2; channels]);
        let base = WavSnapshot {
            fmt: WavFmt { channels: channels as u16, sample_rate: 1, byte_rate: u32::from(block_align), block_align, ..WavFmt::default() },
            data: WavData::Pcm16(values),
            ..WavSnapshot::default()
        };
        let plan = AudioPlan::new(&EditAudio::RemoveFrame { frame: 0, revision: "r".into() }, &base).expect("wide frame removal plans");
        assert_eq!(plan.items, 2);
        let mut format_ext = None;
        let mutations = (0..plan.items).filter_map(|ordinal| plan.mutation(ordinal, &base, &mut format_ext)).collect::<Vec<_>>();
        assert!(mutations.iter().all(|mutation| matches!(mutation, WavMutation::PatchData(patch_data::PatchData { index: 0, .. }))));
        let mut edited = base.clone();
        let mut inverses = Vec::new();
        for mutation in &mutations {
            inverses.push(<WavMutation as Mutation<WavSnapshot>>::inverse(mutation, &edited));
            edited = apply_all(&edited, std::slice::from_ref(mutation));
        }
        assert_eq!(edited.data, WavData::Pcm16(vec![2; channels]));
        for batch in inverses.into_iter().rev() {
            for inverse in batch {
                edited = apply_all(&edited, &[inverse]);
            }
        }
        assert_eq!(edited, base);
    }

    #[test]
    fn large_format_extension_is_copied_in_pages_and_cancelled_with_exact_grants() {
        use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};

        assert!(PATCH_PAYLOAD_BYTES + 17 <= u16::MAX as usize);
        let extension = (0..PATCH_PAYLOAD_BYTES + 17).map(|index| (index % 251) as u8).collect::<Vec<_>>();
        let snapshot = WavSnapshot { fmt: WavFmt { ext: Some(extension.clone()), ..WavFmt::default() }, data: WavData::Pcm16(vec![1, 2]), ..WavSnapshot::default() };
        let config = semio_framework_plugin::NoConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let operation = semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "wav-format-copy".into(), operation_id: 2, generation: 3, canonical_base_revision: [6; 32], authoring_seed: "authoring-seed-test".into() };
        let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision);
        let command = WavEditCommand::EditAudio(EditAudio::SetSampleRate { revision, value: "22050".into() });
        let input = ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };

        let mut completed = EditAudioWork::new(SET_SAMPLE_RATE_ACTION_ID);
        let mut format_steps = 0;
        let emitted = loop {
            match completed.step(&input).expect("format edit advances") {
                ArtifactCommandWorkStep::Progress { stage: "wav-audio-format", .. } => format_steps += 1,
                ArtifactCommandWorkStep::Progress { .. } => {}
                ArtifactCommandWorkStep::Complete(emit) => break emit,
                ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => panic!("unexpected format edit step"),
            }
        };
        assert!(format_steps >= 3, "reserve plus two byte pages are observable progress");
        let [WavMutation::SetFmt(set_fmt::SetFmt { fmt })] = emitted.artifact_mutations.as_slice() else { panic!("sample-rate edit emits one format mutation") };
        assert_eq!(fmt.ext.as_deref(), Some(extension.as_slice()));
        assert_eq!(fmt.sample_rate, 22_050);

        let mut cancelled = EditAudioWork::new(SET_SAMPLE_RATE_ACTION_ID);
        for _ in 0..3 {
            assert!(matches!(cancelled.step(&input).expect("partial format copy advances"), ArtifactCommandWorkStep::Progress { .. }));
        }
        cancelled.begin_close();
        assert_eq!(cancelled.close_step(1, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        while !cancelled.terminal_is_empty() {
            let step = cancelled.close_step(1, 1_024);
            if let InteractiveJobCloseStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 1);
                assert!(released_bytes <= 1_024);
            }
        }
        assert_eq!(snapshot.fmt.ext.as_deref(), Some(extension.as_slice()));
    }

    #[test]
    fn natural_audio_command_binary_round_trip_preserves_revision_and_selection() {
        let command = WavEditCommand::EditAudio(EditAudio::SetSample { frame: 3, channel: 1, revision: "0123456789abcdef".into(), value: "-27".into() });
        let bytes = <WavEditCommand as protocol::OpBinary>::encode_op(&command).expect("natural command encodes");
        assert_eq!(<WavEditCommand as protocol::OpBinary>::decode_op(&bytes).expect("natural command decodes"), command);
    }
}
