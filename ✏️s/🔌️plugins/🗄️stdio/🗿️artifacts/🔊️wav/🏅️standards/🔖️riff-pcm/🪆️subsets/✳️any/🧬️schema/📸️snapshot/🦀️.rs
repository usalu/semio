//! 🧬️ WavSnapshot — typed primary `fmt `/`data` chunks plus the complete ordered RIFF
//! chunk sequence. Duplicate canonical chunks remain verbatim auxiliary chunks, so every admitted
//! recording can preserve chunk order and multiplicity through an edit.

/// 📦️ Owned by `wav`: the `fmt ` chunk's fields, typed. `ext` carries the extensible/non-PCM
/// tail (`cbSize` bytes) verbatim when present — `None` for the plain 16-byte PCM form. NO type
/// sharing with `avi` (both are RIFF-based but deliberately distinct vocabularies per the master
/// plan).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct WavFmt {
    pub audio_format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<Vec<u8>>,
}

impl Default for WavFmt {
    fn default() -> Self {
        Self { audio_format: 1, channels: 1, sample_rate: 44_100, byte_rate: 88_200, block_align: 2, bits_per_sample: 16, ext: None }
    }
}

/// 📦️ Owned by `wav`: the `data` chunk's samples, typed per `WavFmt`'s
/// `(audio_format, bits_per_sample)` — `Raw` is the honest fallback for anything this codec
/// doesn't interpret sample-by-sample (24-bit PCM, ADPCM, WAVE_FORMAT_EXTENSIBLE payloads, …).
/// 🏷️ Adjacently tagged (`tag`+`content`), not purely internally tagged — serde cannot serialize
/// an internally-tagged newtype variant wrapping a non-map type (`Vec<T>` here), the same
/// constraint already on record for `HtmlNode`/`JsonValue` elsewhere in this codebase.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum WavData {
    Pcm16(Vec<i16>),
    Pcm8(Vec<u8>),
    Float32(Vec<f32>),
    Raw(Vec<u8>),
}

impl Default for WavData {
    fn default() -> Self {
        WavData::Raw(Vec::new())
    }
}

fn wav_data_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(1, "kind", dsl::Shape::Enum(vec![("pcm16".into(), 0), ("pcm8".into(), 1), ("float32".into(), 2), ("raw".into(), 3)])),
            dsl::FieldSpec::new(2, "pcm16", <Vec<i16> as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "pcm8", <Vec<u8> as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "float32", <Vec<f32> as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "raw", <Vec<u8> as dsl::DslField>::shape()).optional(),
        ],
    )
}

impl dsl::DslField for WavData {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(wav_data_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        let (kind, id, value) = match self {
            Self::Pcm16(values) => (0, 2, dsl::DslField::to_value(values)),
            Self::Pcm8(values) => (1, 3, dsl::DslField::to_value(values)),
            Self::Float32(values) => (2, 4, dsl::DslField::to_value(values)),
            Self::Raw(values) => (3, 5, dsl::DslField::to_value(values)),
        };
        let mut record = dsl::RecordValue::default();
        record.fields.insert(1, dsl::FieldValue::Enum(kind));
        record.fields.insert(id, value);
        dsl::FieldValue::Record(record)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else {
            return Err("WAV samples require a typed record".into());
        };
        let Some(dsl::FieldValue::Enum(kind)) = record.get(1) else {
            return Err("WAV samples require a declared kind".into());
        };
        let id = kind.checked_add(2).filter(|id| *id <= 5).ok_or("WAV samples have an unknown kind")? as u16;
        for (other, value) in &record.fields {
            if *other != 1 && *other != id && !matches!(value, dsl::FieldValue::Absent) {
                return Err("WAV sample kind selects exactly one typed array".into());
            }
        }
        let values = record.get(id).ok_or("WAV selected sample array is absent")?;
        match kind {
            0 => Ok(Self::Pcm16(dsl::DslField::from_value(values)?)),
            1 => Ok(Self::Pcm8(dsl::DslField::from_value(values)?)),
            2 => Ok(Self::Float32(dsl::DslField::from_value(values)?)),
            3 => Ok(Self::Raw(dsl::DslField::from_value(values)?)),
            _ => Err("WAV samples have an unknown kind".into()),
        }
    }
}

/// 📦️ Owned by `wav`: any auxiliary or duplicate canonical RIFF chunk, retained byte-for-byte
/// together with its word-alignment pad byte.
pub(crate) fn is_zero_byte(value: &u8) -> bool {
    *value == 0
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RiffChunk {
    pub fourcc: String,
    #[value(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
    #[value(default, skip_serializing_if = "is_zero_byte")]
    pub pad_byte: u8,
}

/// 🧭️ One position in the top-level RIFF/WAVE chunk sequence. `Format` and `Samples`
/// reference the typed primary chunks; `Other` references `other_chunks[index]`. A duplicate
/// `fmt `/`data` chunk is deliberately an `Other` entry so its original payload survives exactly.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum WavChunkRef {
    Format,
    Samples,
    Other(u64),
}

fn wav_chunk_ref_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(1, "kind", dsl::Shape::Enum(vec![("format".into(), 0), ("samples".into(), 1), ("other".into(), 2)])), dsl::FieldSpec::new(2, "index", dsl::Shape::UInt).optional()])
}

impl dsl::DslField for WavChunkRef {
    fn shape() -> dsl::Shape {
        dsl::Shape::Record(wav_chunk_ref_spec)
    }
    fn to_value(&self) -> dsl::FieldValue {
        let mut record = dsl::RecordValue::default();
        let kind = match self {
            Self::Format => 0,
            Self::Samples => 1,
            Self::Other(index) => {
                record.fields.insert(2, dsl::FieldValue::UInt(*index));
                2
            }
        };
        record.fields.insert(1, dsl::FieldValue::Enum(kind));
        dsl::FieldValue::Record(record)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Record(record) = value else {
            return Err("WAV chunk reference requires a typed record".into());
        };
        if record.fields.keys().any(|id| ![1, 2].contains(id)) {
            return Err("WAV chunk reference contains an unknown field".into());
        }
        let index = record.get(2).filter(|value| !matches!(value, dsl::FieldValue::Absent));
        match (record.get(1), index) {
            (Some(dsl::FieldValue::Enum(0)), None) => Ok(Self::Format),
            (Some(dsl::FieldValue::Enum(1)), None) => Ok(Self::Samples),
            (Some(dsl::FieldValue::Enum(2)), Some(dsl::FieldValue::UInt(index))) => Ok(Self::Other(*index)),
            _ => Err("WAV chunk reference requires its exact declared kind and unsigned64 index".into()),
        }
    }
}

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_WAV_DOCUMENT_SCHEMA: &str = "stdio.wav";
pub const MAXIMUM_FMT_EXTENSION_BYTES: usize = u16::MAX as usize;
//#endregion 🔖️Ids

/// 🚫️ One WAV snapshot field that cannot be represented exactly on the RIFF wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WavSerializationIssue {
    pub code: &'static str,
    pub message: String,
    pub target: Vec<String>,
}

fn serialization_issue(code: &'static str, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> WavSerializationIssue {
    WavSerializationIssue { code, message: message.into(), target: target.into_iter().map(Into::into).collect() }
}

fn pad_is_representable(pad_byte: u8, payload_is_odd: bool, target: &'static str) -> Result<(), WavSerializationIssue> {
    if pad_byte != 0 && !payload_is_odd {
        return Err(serialization_issue("stdio.wav.serialization.invalid-pad-byte", format!("{target} is nonzero but its RIFF chunk payload has even length and carries no pad byte"), [target]));
    }
    Ok(())
}

/// 🧭️ Refuses snapshot states that cannot survive one exact RIFF/WAVE save and reopen.
pub fn validate_wav_serialization(snapshot: &WavSnapshot) -> Result<(), WavSerializationIssue> {
    let ext_len = snapshot.fmt.ext.as_ref().map_or(0, Vec::len);
    if ext_len > MAXIMUM_FMT_EXTENSION_BYTES {
        return Err(serialization_issue("stdio.wav.serialization.fmt-extension-too-large", format!("fmt.ext contains {ext_len} bytes; RIFF/WAVE cbSize can declare at most {MAXIMUM_FMT_EXTENSION_BYTES}"), ["fmt", "ext"]));
    }
    let fmt_payload_is_odd = snapshot.fmt.ext.as_ref().is_some_and(|ext| !ext.len().is_multiple_of(2));
    pad_is_representable(snapshot.fmt_pad_byte, fmt_payload_is_odd, "fmtPadByte")?;
    let data_payload_is_odd = match &snapshot.data {
        WavData::Pcm8(bytes) | WavData::Raw(bytes) => !bytes.len().is_multiple_of(2),
        WavData::Pcm16(_) | WavData::Float32(_) => false,
    };
    pad_is_representable(snapshot.data_pad_byte, data_payload_is_odd, "dataPadByte")?;
    for (index, chunk) in snapshot.other_chunks.iter().enumerate() {
        let bytes = chunk.fourcc.as_bytes();
        if bytes.len() != 4 || !bytes.iter().all(|byte| byte.is_ascii_graphic() || *byte == b' ') {
            return Err(serialization_issue("stdio.wav.serialization.invalid-fourcc", format!("otherChunks[{index}].fourcc must contain exactly four printable ASCII wire bytes"), ["otherChunks".to_string(), index.to_string(), "fourcc".to_string()]));
        }
        pad_is_representable(chunk.pad_byte, !chunk.data.len().is_multiple_of(2), "padByte").map_err(|mut issue| {
            issue.message = format!("otherChunks[{index}].padByte is nonzero but its RIFF chunk payload has even length and carries no pad byte");
            issue.target = vec!["otherChunks".into(), index.to_string(), "padByte".into()];
            issue
        })?;
    }
    Ok(())
}

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.wav")]
pub struct WavSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub fmt: WavFmt,
    #[state(artifact)]
    pub data: WavData,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "is_zero_byte")]
    pub fmt_pad_byte: u8,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "is_zero_byte")]
    pub data_pad_byte: u8,
    #[state(artifact)]
    #[value(default)]
    pub other_chunks: Vec<RiffChunk>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub chunk_order: Vec<WavChunkRef>,
}

impl Default for WavSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_WAV_DOCUMENT_SCHEMA.into(), fmt: WavFmt::default(), data: WavData::Pcm16(Vec::new()), fmt_pad_byte: 0, data_pad_byte: 0, other_chunks: Vec::new(), chunk_order: vec![WavChunkRef::Format, WavChunkRef::Samples] }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🎼️ Encodes every owned logical snapshot field independently of ordinary file syntax.
impl store::ArtifactDsl for WavSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_WAV_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 32 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for WavSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let raw = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;
#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
