//! 💾️ Binary representation grammar surface for `stdio.semio.video` (snapshot): real
//! varint-length-prefixed binary pack frame — `format u8` + `schema` (length-prefixed UTF-8) real
//! and fully described, `streams` an opaque trailing payload (`protocol-array-of-records` gap;
//! video wave, replacing the old `serde_json::to_vec`-in-envelope shortcut) — actual encode/decode
//! lives on `SemioVideoSnapshot`'s `store::ArtifactPack` impl in the facet root
//! `🦀️.rs`; this leaf carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::video::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers flow's/mesh's upgraded facets reuse) backing the real
/// `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope shortcut.
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
pub(crate) fn kind_tag(k: SemioVideoStreamKind) -> u8 {
    match k {
        SemioVideoStreamKind::Video => 0,
        SemioVideoStreamKind::Audio => 1,
        SemioVideoStreamKind::Subtitle => 2,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn kind_from_tag(t: u8) -> Result<SemioVideoStreamKind, String> {
    match t {
        0 => Ok(SemioVideoStreamKind::Video),
        1 => Ok(SemioVideoStreamKind::Audio),
        2 => Ok(SemioVideoStreamKind::Subtitle),
        other => Err(format!("stream kind: bad tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_video_snapshot_binary(s: &SemioVideoSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.streams.len() as u64);
    for stream in &s.streams {
        out.push(kind_tag(stream.kind));
        write_str_lp(&mut out, &stream.codec);
        out.extend_from_slice(&stream.width.to_le_bytes());
        out.extend_from_slice(&stream.height.to_le_bytes());
        out.extend_from_slice(&stream.rate.num.to_le_bytes());
        out.extend_from_slice(&stream.rate.den.to_le_bytes());
        store::pack_rt::write_varint_u64(&mut out, stream.samples.len() as u64);
        for sample in &stream.samples {
            out.extend_from_slice(&sample.pts.to_le_bytes());
            out.push(if sample.key { 1 } else { 0 });
            write_bytes_lp(&mut out, &sample.data);
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_video_snapshot_binary(bytes: &[u8]) -> Result<SemioVideoSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let stream_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut streams = Vec::with_capacity(stream_count as usize);
    for _ in 0..stream_count {
        let kind = kind_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
        let codec = read_str_lp(&mut reader)?;
        let width = reader.read_u32_le().map_err(|e| e.to_string())?;
        let height = reader.read_u32_le().map_err(|e| e.to_string())?;
        let num = i64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "rate.num: truncated".to_string())?);
        let den = i64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "rate.den: truncated".to_string())?);
        let sample_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut samples = Vec::with_capacity(sample_count as usize);
        for _ in 0..sample_count {
            let pts = reader.read_u64_le().map_err(|e| e.to_string())?;
            let key = match reader.read_u8().map_err(|e| e.to_string())? {
                0 => false,
                1 => true,
                other => return Err(format!("sample key: bad tag {other}")),
            };
            let data = read_bytes_lp(&mut reader)?;
            samples.push(SemioVideoSample { pts, key, data });
        }
        streams.push(SemioVideoStream { kind, codec, width, height, rate: SemioRational { num, den }, samples });
    }
    Ok(SemioVideoSnapshot { schema, streams })
}

impl store::ArtifactPack for SemioVideoSnapshot {

    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_video_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_video_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;
