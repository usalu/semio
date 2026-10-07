//! 💾️ Binary representation codec surface for `s.stdio.semio.audio` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::audio::schema::snapshot::*;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `🌊️flow`'s/`🔺️mesh`'s/`🖼️image`'s own upgraded
/// `ArtifactPack` uses) backing the real `ArtifactPack` below — replaces the old
/// `serde_json::to_vec`-in-envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f32_le(reader: &mut store::ByteReader<'_>) -> Result<f32, String> {
    let bytes = reader.read_bytes(4).map_err(|e| e.to_string())?;
    let arr: [u8; 4] = bytes.try_into().map_err(|_| "f32 read: truncated".to_string())?;
    Ok(f32::from_le_bytes(arr))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn format_tag(f: SemioAudioFormat) -> u8 {
    match f {
        SemioAudioFormat::Pcm8 => 0,
        SemioAudioFormat::Pcm16 => 1,
        SemioAudioFormat::Pcm24 => 2,
        SemioAudioFormat::Pcm32 => 3,
        SemioAudioFormat::Float32 => 4,
        SemioAudioFormat::Float64 => 5,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn format_from_tag(tag: u8) -> Result<SemioAudioFormat, String> {
    match tag {
        0 => Ok(SemioAudioFormat::Pcm8),
        1 => Ok(SemioAudioFormat::Pcm16),
        2 => Ok(SemioAudioFormat::Pcm24),
        3 => Ok(SemioAudioFormat::Pcm32),
        4 => Ok(SemioAudioFormat::Float32),
        5 => Ok(SemioAudioFormat::Float64),
        other => Err(format!("unsupported audio format tag {other}")),
    }
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 + real fixed `sample_rate` (`u32` LE) +
/// `audio_format` (`u8` tag) — all genuinely, individually protocol-walkable, matching the real
/// `📡️.protocol.semio` header/segment fields exactly — then `channels` (varint count +
/// per-channel varint sample count + real 4-byte LE `f32` samples, no hex/text detour) and `tags`
/// (varint count + per-entry varint-length-prefixed `key`/`value` UTF-8) as the honest opaque
/// `payload` tail (`protocol-array-of-records` gap — `channels`/`tags` are homogeneous
/// variable-length repeated records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_audio_snapshot_binary(s: &SemioAudioSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    out.extend_from_slice(&s.sample_rate.to_le_bytes());
    out.push(format_tag(s.format));
    store::pack_rt::write_varint_u64(&mut out, s.channels.len() as u64);
    for c in &s.channels {
        store::pack_rt::write_varint_u64(&mut out, c.samples.len() as u64);
        for sample in &c.samples {
            out.extend_from_slice(&sample.to_le_bytes());
        }
    }
    store::pack_rt::write_varint_u64(&mut out, s.tags.len() as u64);
    for tag in &s.tags {
        write_str_lp(&mut out, &tag.key);
        write_str_lp(&mut out, &tag.value);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_audio_snapshot_binary(bytes: &[u8]) -> Result<SemioAudioSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let sample_rate = reader.read_u32_le().map_err(|e| e.to_string())?;
    let audio_format = format_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let channel_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut channels = Vec::with_capacity(channel_count as usize);
    for _ in 0..channel_count {
        let sample_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut samples = Vec::with_capacity(sample_count as usize);
        for _ in 0..sample_count {
            samples.push(read_f32_le(&mut reader)?);
        }
        channels.push(SemioAudioChannel { samples });
    }
    let tag_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut tags = Vec::with_capacity(tag_count as usize);
    for _ in 0..tag_count {
        let key = read_str_lp(&mut reader)?;
        let value = read_str_lp(&mut reader)?;
        tags.push(SemioAudioTag { key, value });
    }
    Ok(SemioAudioSnapshot { schema, sample_rate, format: audio_format, channels, tags })
}

impl store::ArtifactPack for SemioAudioSnapshot {

    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_audio_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_audio_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;
