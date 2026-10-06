//! 💾️ Binary representation codec surface for `stdio.semio.image` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `🌊️flow`'s/`🔺️mesh`'s own upgraded `ArtifactPack` uses)
/// backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope
/// shortcut.
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
pub(crate) fn colorspace_tag(c: SemioColorspace) -> u8 {
    match c {
        SemioColorspace::Rgb => 0,
        SemioColorspace::Rgba => 1,
        SemioColorspace::Grayscale => 2,
        SemioColorspace::GrayscaleAlpha => 3,
        SemioColorspace::Indexed => 4,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn colorspace_from_tag(tag: u8) -> Result<SemioColorspace, String> {
    match tag {
        0 => Ok(SemioColorspace::Rgb),
        1 => Ok(SemioColorspace::Rgba),
        2 => Ok(SemioColorspace::Grayscale),
        3 => Ok(SemioColorspace::GrayscaleAlpha),
        4 => Ok(SemioColorspace::Indexed),
        other => Err(format!("unsupported colorspace tag {other}")),
    }
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 + real fixed-width `width`/`height`
/// (`u32` LE) + `colorspace` (`u8` tag) + `bit_depth` (`u8`) — all genuinely, individually
/// protocol-walkable, matching the real `📡️.protocol.semio` header/segment fields
/// exactly — then `icc` (presence `u8` + optional varint-length-prefixed bytes), `frames`
/// (varint count + per-frame `delay_ms`/`rgba8`), and `metadata` (varint count + per-entry
/// `key`/`value`) as the honest opaque `payload` tail (`protocol-array-of-records` gap — `frames`/
/// `metadata` are homogeneous variable-length repeated records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_image_snapshot_binary(s: &SemioImageSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    out.extend_from_slice(&s.width.to_le_bytes());
    out.extend_from_slice(&s.height.to_le_bytes());
    out.push(colorspace_tag(s.colorspace));
    out.push(s.bit_depth);
    match &s.icc {
        Some(bytes) => {
            out.push(1);
            write_bytes_lp(&mut out, bytes);
        }
        None => out.push(0),
    }
    store::pack_rt::write_varint_u64(&mut out, s.frames.len() as u64);
    for f in &s.frames {
        out.extend_from_slice(&f.delay_ms.to_le_bytes());
        write_bytes_lp(&mut out, &f.rgba8);
    }
    store::pack_rt::write_varint_u64(&mut out, s.metadata.len() as u64);
    for entry in &s.metadata {
        write_str_lp(&mut out, &entry.key);
        write_str_lp(&mut out, &entry.value);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_image_snapshot_binary(bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let width = reader.read_u32_le().map_err(|e| e.to_string())?;
    let height = reader.read_u32_le().map_err(|e| e.to_string())?;
    let colorspace = colorspace_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let bit_depth = reader.read_u8().map_err(|e| e.to_string())?;
    let icc = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(read_bytes_lp(&mut reader)?),
        other => return Err(format!("unsupported icc presence tag {other}")),
    };
    let frame_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut frames = Vec::with_capacity(frame_count as usize);
    for _ in 0..frame_count {
        let delay_ms = reader.read_u32_le().map_err(|e| e.to_string())?;
        let rgba8 = read_bytes_lp(&mut reader)?;
        frames.push(SemioImageFrame { delay_ms, rgba8 });
    }
    let metadata_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut metadata = Vec::with_capacity(metadata_count as usize);
    for _ in 0..metadata_count {
        let key = read_str_lp(&mut reader)?;
        let value = read_str_lp(&mut reader)?;
        metadata.push(SemioImageMetadataEntry { key, value });
    }
    Ok(SemioImageSnapshot { schema, width, height, colorspace, bit_depth, icc, frames, metadata })
}

impl store::ArtifactPack for SemioImageSnapshot {

    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_image_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_image_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::*;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::image::io::text::snapshot::*;
/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioImageSnapshot`] —
/// the binary half of the same bridge, so a caller outside this crate can check the two codecs
/// against each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_image_pack(bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    <SemioImageSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `store::ArtifactPack::encode_pack` inverse of [`decode_semio_image_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_image_pack(snapshot: &SemioImageSnapshot) -> Vec<u8> {
    <SemioImageSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
