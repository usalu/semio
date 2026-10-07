//! 💾️ Binary representation codec surface for `stdio.semio.graph` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::graph::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value_entry};
use crate::standards::v1::subsets::value::io::text::diff::{enc_semio_value_entry};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_bin};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
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
pub(crate) fn port_kind_tag(k: SemioGraphPortKind) -> u8 {
    match k {
        SemioGraphPortKind::In => 0,
        SemioGraphPortKind::Out => 1,
        SemioGraphPortKind::InOut => 2,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn port_kind_from_tag(tag: u8) -> Result<SemioGraphPortKind, String> {
    match tag {
        0 => Ok(SemioGraphPortKind::In),
        1 => Ok(SemioGraphPortKind::Out),
        2 => Ok(SemioGraphPortKind::InOut),
        other => Err(format!("unsupported port kind tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_port(out: &mut Vec<u8>, p: &SemioGraphPort) {
    write_str_lp(out, &p.name);
    out.push(port_kind_tag(p.kind));
    write_str_lp(out,&p.category);
    store::pack_rt::write_varint_u64(out,p.properties.len() as u64);
    for property in &p.properties{write_property(out,property);}
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_port(reader: &mut store::ByteReader<'_>) -> Result<SemioGraphPort, String> {
    let name = read_str_lp(reader)?;
    let kind = port_kind_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    let category=read_str_lp(reader)?;
    let count=reader.read_varint_u64().map_err(|error|error.to_string())?;
    let mut properties=Vec::new();for _ in 0..count{properties.push(read_property(reader)?);}
    Ok(SemioGraphPort { name, kind, category, properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_property(out: &mut Vec<u8>, p: &SemioValueEntry) {
    write_str_lp(out, &p.key);
    enc_semio_value_bin(&p.value, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_property(reader: &mut store::ByteReader<'_>) -> Result<SemioValueEntry, String> {
    let key = read_str_lp(reader)?;
    let value = dec_semio_value_bin(reader)?;
    Ok(SemioValueEntry { key, value })
}

/// 🔢 `SemioPoint2`'s `x`/`y` written raw (8+8 bytes, no length prefix needed for a fixed-size
/// float) via `f64::to_le_bytes()`/`f64::from_le_bytes()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    let x = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "point2: short x".to_string())?);
    let y = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "point2: short y".to_string())?);
    Ok(SemioPoint2 { x, y })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_node(out: &mut Vec<u8>, n: &SemioGraphNode) {
    write_str_lp(out, &n.id.value);
    write_str_lp(out, &n.kind);
    write_str_lp(out, &n.label);
    write_point2(out, &n.position);
    out.extend_from_slice(&n.width.to_le_bytes());
    out.extend_from_slice(&n.height.to_le_bytes());
    store::pack_rt::write_varint_u64(out, n.ports.len() as u64);
    for p in &n.ports {
        write_port(out, p);
    }
    store::pack_rt::write_varint_u64(out, n.properties.len() as u64);
    for p in &n.properties {
        write_property(out, p);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_node(reader: &mut store::ByteReader<'_>) -> Result<SemioGraphNode, String> {
    let id = GraphNodeId::new(read_str_lp(reader)?);
    let kind = read_str_lp(reader)?;
    let label = read_str_lp(reader)?;
    let position = read_point2(reader)?;
    let width=reader.read_f64_le().map_err(|error|error.to_string())?;
    let height=reader.read_f64_le().map_err(|error|error.to_string())?;
    let port_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut ports = Vec::with_capacity(port_count as usize);
    for _ in 0..port_count {
        ports.push(read_port(reader)?);
    }
    let property_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut properties = Vec::with_capacity(property_count as usize);
    for _ in 0..property_count {
        properties.push(read_property(reader)?);
    }
    Ok(SemioGraphNode { id, kind, label, position, width, height, ports, properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_edge(out: &mut Vec<u8>, e: &SemioGraphEdge) {
    write_str_lp(out, &e.id.value);
    write_str_lp(out, &e.source.value);
    write_str_lp(out, &e.target.value);
    write_str_lp(out, &e.kind);
    write_str_lp(out, &e.label);
    write_optional(out,e.source_port.as_deref());
    write_optional(out,e.target_port.as_deref());
    store::pack_rt::write_varint_u64(out,e.properties.len() as u64);
    for property in &e.properties{write_property(out,property);}
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_edge(reader: &mut store::ByteReader<'_>) -> Result<SemioGraphEdge, String> {
    let id = GraphEdgeId::new(read_str_lp(reader)?);
    let source = GraphNodeId::new(read_str_lp(reader)?);
    let target = GraphNodeId::new(read_str_lp(reader)?);
    let kind = read_str_lp(reader)?;
    let label = read_str_lp(reader)?;
    let source_port=read_optional(reader)?;
    let target_port=read_optional(reader)?;
    let count=reader.read_varint_u64().map_err(|error|error.to_string())?;
    let mut properties=Vec::new();for _ in 0..count{properties.push(read_property(reader)?);}
    Ok(SemioGraphEdge { id, source, target, kind, label, source_port,target_port,properties })
}

pub(crate) fn write_optional(out:&mut Vec<u8>,value:Option<&str>){match value{Some(value)=>{out.push(1);write_str_lp(out,value)},None=>out.push(0)}}

pub(crate) fn read_optional(reader:&mut store::ByteReader<'_>)->Result<Option<String>,String>{match reader.read_u8().map_err(|error|error.to_string())?{0=>Ok(None),1=>Ok(Some(read_str_lp(reader)?)),_=>Err("invalid Semio optional text tag".into())}}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 — both genuinely, individually
/// protocol-walkable — then `nodes`/`edges` (varint count + per-record fields) as the honest opaque
/// `payload` tail (`protocol-array-of-records` gap — homogeneous, variable-length repeated records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_graph_snapshot_binary(s: &SemioGraphSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.nodes.len() as u64);
    for n in &s.nodes {
        write_node(&mut out, n);
    }
    store::pack_rt::write_varint_u64(&mut out, s.edges.len() as u64);
    for e in &s.edges {
        write_edge(&mut out, e);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_graph_snapshot_binary(bytes: &[u8]) -> Result<SemioGraphSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let node_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut nodes = Vec::with_capacity(node_count as usize);
    for _ in 0..node_count {
        nodes.push(read_node(&mut reader)?);
    }
    let edge_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut edges = Vec::with_capacity(edge_count as usize);
    for _ in 0..edge_count {
        edges.push(read_edge(&mut reader)?);
    }
    Ok(SemioGraphSnapshot { schema, nodes, edges })
}

impl store::ArtifactPack for SemioGraphSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_graph_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_graph_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::graph::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value_entry};
use crate::standards::v1::subsets::value::io::text::diff::{enc_semio_value_entry};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_bin};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::graph::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioGraphSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_graph_pack(snapshot: &SemioGraphSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioGraphSnapshot`] — the inverse of
/// [`encode_semio_graph_pack`], reading `../../🖼️assets/🕸️wires/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_graph_pack(bytes: &[u8]) -> Result<SemioGraphSnapshot, String> {
    <SemioGraphSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
