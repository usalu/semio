//! 💾️ Binary representation codec surface for `stdio.semio.text` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::text::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers every other real semio codec in this standard uses).
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
pub(crate) fn mark_kind_tag(k: SemioTextMarkKind) -> u8 {
    match k {
        SemioTextMarkKind::Bold => 0,
        SemioTextMarkKind::Italic => 1,
        SemioTextMarkKind::Code => 2,
        SemioTextMarkKind::Link => 3,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn mark_kind_from_tag(tag: u8) -> Result<SemioTextMarkKind, String> {
    match tag {
        0 => Ok(SemioTextMarkKind::Bold),
        1 => Ok(SemioTextMarkKind::Italic),
        2 => Ok(SemioTextMarkKind::Code),
        3 => Ok(SemioTextMarkKind::Link),
        other => Err(format!("unsupported mark kind tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_mark(out: &mut Vec<u8>, m: &SemioTextMark) {
    out.push(mark_kind_tag(m.kind));
    write_str_lp(out, &m.href);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_mark(reader: &mut store::ByteReader<'_>) -> Result<SemioTextMark, String> {
    let kind = mark_kind_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let href = read_str_lp(reader)?;
    Ok(SemioTextMark { kind, href })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_run(out: &mut Vec<u8>, r: &SemioTextRun) {
    write_str_lp(out, &r.language);
    write_str_lp(out, &r.content);
    store::pack_rt::write_varint_u64(out, r.marks.len() as u64);
    for m in &r.marks {
        write_mark(out, m);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_run(reader: &mut store::ByteReader<'_>) -> Result<SemioTextRun, String> {
    let language = read_str_lp(reader)?;
    let content = read_str_lp(reader)?;
    let mark_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut marks = Vec::with_capacity(mark_count as usize);
    for _ in 0..mark_count {
        marks.push(read_mark(reader)?);
    }
    Ok(SemioTextRun { language, content, marks })
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 — both genuinely, individually
/// protocol-walkable, matching `📡️.protocol.semio`'s header/segment fields exactly —
/// then `runs` (varint count + per-run language/content/marks) as the honest opaque `payload`
/// tail (`protocol-array-of-records` gap — homogeneous, variable-length repeated records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_text_snapshot_binary(s: &SemioTextSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.runs.len() as u64);
    for r in &s.runs {
        write_run(&mut out, r);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_text_snapshot_binary(bytes: &[u8]) -> Result<SemioTextSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let run_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut runs = Vec::with_capacity(run_count as usize);
    for _ in 0..run_count {
        runs.push(read_run(&mut reader)?);
    }
    Ok(SemioTextSnapshot { schema, runs })
}

impl store::ArtifactPack for SemioTextSnapshot {

    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_text_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_text_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::text::schema::snapshot::*;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::text::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioTextSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_text_pack(snapshot: &SemioTextSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioTextSnapshot`] — the inverse of
/// [`encode_semio_text_pack`], reading `../../🖼️assets/📃️note/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_text_pack(bytes: &[u8]) -> Result<SemioTextSnapshot, String> {
    <SemioTextSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
