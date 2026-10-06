//! 💾️ Binary representation codec surface for `stdio.semio.drawing` (snapshot). The real
//! encode/decode lives on `SemioDrawingSnapshot`'s `store::ArtifactPack` impl
//! (📸️snapshot/🦀️.rs) -- this module exposes the protocol source for tooling.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::drawing::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native::{NativeF64,NativeF32};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.semio.flow`'s/`stdio.semio.brep`'s upgraded
/// `ArtifactPack` reuse) backing the real `ArtifactPack` below — replaces the old
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
pub(crate) fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    Ok(SemioPoint2 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point3(out: &mut Vec<u8>, p: &SemioPoint3) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point3(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint3, String> {
    Ok(SemioPoint3 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_quaternion(out: &mut Vec<u8>, q: &SemioQuaternion) {
    out.extend_from_slice(&q.x.to_le_bytes());
    out.extend_from_slice(&q.y.to_le_bytes());
    out.extend_from_slice(&q.z.to_le_bytes());
    out.extend_from_slice(&q.w.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_quaternion(reader: &mut store::ByteReader<'_>) -> Result<SemioQuaternion, String> {
    Ok(SemioQuaternion { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())?, w: reader.read_f64_le().map_err(|e| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_transform(out: &mut Vec<u8>, t: &SemioTransform) {
    write_point3(out, &t.translation);
    write_quaternion(out, &t.rotation);
    write_point3(out, &t.scale);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<SemioTransform, String> {
    Ok(SemioTransform { translation: read_point3(reader)?, rotation: read_quaternion(reader)?, scale: read_point3(reader)? })
}

/// 🩹️ `store::ByteReader` has no native `f32` reader (only `f64_le`) — read 4 raw bytes instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f32_le(reader: &mut store::ByteReader<'_>) -> Result<f32, String> {
    let bytes = reader.read_bytes(4).map_err(|e| e.to_string())?;
    let arr: [u8; 4] = bytes.try_into().map_err(|_| "f32 read: truncated".to_string())?;
    Ok(f32::from_le_bytes(arr))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_rgba(out: &mut Vec<u8>, c: &SemioRgba) {
    out.extend_from_slice(&c.r.to_le_bytes());
    out.extend_from_slice(&c.g.to_le_bytes());
    out.extend_from_slice(&c.b.to_le_bytes());
    out.extend_from_slice(&c.a.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_rgba(reader: &mut store::ByteReader<'_>) -> Result<SemioRgba, String> {
    Ok(SemioRgba { r: read_f32_le(reader)?, g: read_f32_le(reader)?, b: read_f32_le(reader)?, a: read_f32_le(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_option<T>(out: &mut Vec<u8>, opt: &Option<T>, write: impl Fn(&mut Vec<u8>, &T)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            write(out, v);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option<T>(reader: &mut store::ByteReader<'_>, read: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read(reader)?)),
        other => Err(format!("option: bad tag byte {other}")),
    }
}

/// 🏷️ `PathSegment` variant tags — 0=MoveTo, 1=LineTo, 2=CubicTo, 3=QuadTo, 4=ArcTo, 5=Close.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_path_segment(out: &mut Vec<u8>, seg: &PathSegment) {
    match seg {
        PathSegment::MoveTo { to } => {
            out.push(0);
            write_point2(out, to);
        }
        PathSegment::LineTo { to } => {
            out.push(1);
            write_point2(out, to);
        }
        PathSegment::CubicTo { c1, c2, to } => {
            out.push(2);
            write_point2(out, c1);
            write_point2(out, c2);
            write_point2(out, to);
        }
        PathSegment::QuadTo { c, to } => {
            out.push(3);
            write_point2(out, c);
            write_point2(out, to);
        }
        PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
            out.push(4);
            out.extend_from_slice(&rx.to_le_bytes());
            out.extend_from_slice(&ry.to_le_bytes());
            out.extend_from_slice(&x_rotation.to_le_bytes());
            write_bool(out, *large_arc);
            write_bool(out, *sweep);
            write_point2(out, to);
        }
        PathSegment::Close => out.push(5),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_path_segment(reader: &mut store::ByteReader<'_>) -> Result<PathSegment, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(PathSegment::MoveTo { to: read_point2(reader)? }),
        1 => Ok(PathSegment::LineTo { to: read_point2(reader)? }),
        2 => Ok(PathSegment::CubicTo { c1: read_point2(reader)?, c2: read_point2(reader)?, to: read_point2(reader)? }),
        3 => Ok(PathSegment::QuadTo { c: read_point2(reader)?, to: read_point2(reader)? }),
        4 => Ok(PathSegment::ArcTo {
            rx: reader.read_f64_le().map_err(|e| e.to_string())?,
            ry: reader.read_f64_le().map_err(|e| e.to_string())?,
            x_rotation: reader.read_f64_le().map_err(|e| e.to_string())?,
            large_arc: read_bool(reader)?,
            sweep: read_bool(reader)?,
            to: read_point2(reader)?,
        }),
        5 => Ok(PathSegment::Close),
        other => Err(format!("path segment: unknown binary tag {other}")),
    }
}

/// 🏷️ `DrawNode` variant tags — 0=Path, 1=Text, 2=Group (RECURSIVE `children`), 3=Image.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_node(out: &mut Vec<u8>, n: &DrawNode) {
    match n {
        DrawNode::Path { segments, style } => {
            out.push(0);
            store::pack_rt::write_varint_u64(out, segments.len() as u64);
            for seg in segments {
                write_path_segment(out, seg);
            }
            write_option(out, style, |out, s| write_str_lp(out, s));
        }
        DrawNode::Text { value, at, style } => {
            out.push(1);
            write_str_lp(out, value);
            write_point2(out, at);
            write_option(out, style, |out, s| write_str_lp(out, s));
        }
        DrawNode::Group { transform, children } => {
            out.push(2);
            write_transform(out, transform);
            store::pack_rt::write_varint_u64(out, children.len() as u64);
            for child in children {
                write_node(out, child);
            }
        }
        DrawNode::Image { at, width, height, mime, bytes } => {
            out.push(3);
            write_point2(out, at);
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            write_str_lp(out, mime);
            write_bytes_lp(out, bytes);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_node(reader: &mut store::ByteReader<'_>) -> Result<DrawNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut segments = Vec::with_capacity(n as usize);
            for _ in 0..n {
                segments.push(read_path_segment(reader)?);
            }
            let style = read_option(reader, read_str_lp)?;
            Ok(DrawNode::Path { segments, style })
        }
        1 => {
            let value = read_str_lp(reader)?;
            let at = read_point2(reader)?;
            let style = read_option(reader, read_str_lp)?;
            Ok(DrawNode::Text { value, at, style })
        }
        2 => {
            let transform = read_transform(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut children = Vec::with_capacity(n as usize);
            for _ in 0..n {
                children.push(read_node(reader)?);
            }
            Ok(DrawNode::Group { transform, children })
        }
        3 => {
            let at = read_point2(reader)?;
            let width = reader.read_f64_le().map_err(|e| e.to_string())?;
            let height = reader.read_f64_le().map_err(|e| e.to_string())?;
            let mime = read_str_lp(reader)?;
            let bytes = read_bytes_lp(reader)?;
            Ok(DrawNode::Image { at, width, height, mime, bytes })
        }
        other => Err(format!("node: unknown binary tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_style(out: &mut Vec<u8>, s: &DrawStyle) {
    write_str_lp(out, &s.name);
    write_option(out, &s.fill, write_rgba);
    write_option(out, &s.stroke, write_rgba);
    write_option(out, &s.stroke_width, |out, v| out.extend_from_slice(&v.to_le_bytes()));
    write_option(out, &s.opacity, |out, v| out.extend_from_slice(&v.to_le_bytes()));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_style(reader: &mut store::ByteReader<'_>) -> Result<DrawStyle, String> {
    let name = read_str_lp(reader)?;
    let fill = read_option(reader, read_rgba)?;
    let stroke = read_option(reader, read_rgba)?;
    let stroke_width = read_option(reader, |r| r.read_f64_le().map_err(|e| e.to_string()))?;
    let opacity = read_option(reader, read_f32_le)?;
    Ok(DrawStyle { name, fill, stroke, stroke_width, opacity })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_layer(out: &mut Vec<u8>, l: &DrawLayer) {
    write_str_lp(out, &l.id);
    write_str_lp(out, &l.name);
    write_bool(out, l.visible);
    write_node(out, &l.root);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_layer(reader: &mut store::ByteReader<'_>) -> Result<DrawLayer, String> {
    Ok(DrawLayer { id: read_str_lp(reader)?, name: read_str_lp(reader)?, visible: read_bool(reader)?, root: read_node(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_canvas(out: &mut Vec<u8>, c: &DrawCanvas) {
    out.extend_from_slice(&c.width.to_le_bytes());
    out.extend_from_slice(&c.height.to_le_bytes());
    write_option(out, &c.background, write_rgba);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_canvas(reader: &mut store::ByteReader<'_>) -> Result<DrawCanvas, String> {
    let width = reader.read_f64_le().map_err(|e| e.to_string())?;
    let height = reader.read_f64_le().map_err(|e| e.to_string())?;
    let background = read_option(reader, read_rgba)?;
    Ok(DrawCanvas { width, height, background })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_drawing_snapshot_binary(s: &SemioDrawingSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    write_canvas(&mut out, &s.canvas);
    store::pack_rt::write_varint_u64(&mut out, s.styles.len() as u64);
    for style in &s.styles {
        write_style(&mut out, style);
    }
    store::pack_rt::write_varint_u64(&mut out, s.layers.len() as u64);
    for layer in &s.layers {
        write_layer(&mut out, layer);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_drawing_snapshot_binary(bytes: &[u8]) -> Result<SemioDrawingSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let canvas = read_canvas(&mut reader)?;
    let style_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut styles = Vec::with_capacity(style_count as usize);
    for _ in 0..style_count {
        styles.push(read_style(&mut reader)?);
    }
    let layer_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut layers = Vec::with_capacity(layer_count as usize);
    for _ in 0..layer_count {
        layers.push(read_layer(&mut reader)?);
    }
    Ok(SemioDrawingSnapshot { schema, canvas, styles, layers })
}

impl store::ArtifactPack for SemioDrawingSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_drawing_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_drawing_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::drawing::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native::{NativeF64,NativeF32};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::drawing::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioDrawingSnapshot`] as a semio pack envelope — the binary twin of the DSL
/// text, produced by a SEPARATE codec, which is what makes the two committed encodings of one
/// document able to contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_drawing_pack(snapshot: &SemioDrawingSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioDrawingSnapshot`] — the inverse of
/// [`encode_semio_drawing_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_drawing_pack(bytes: &[u8]) -> Result<SemioDrawingSnapshot, String> {
    <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
