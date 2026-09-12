//! 📦️ DAG artifact — native binary codec (`impl store::ArtifactPack for DagSnapshot`), moved here
//! wholesale from the old `🧬️schema/📸️snapshot` codec home (design.md §1 CORRECTION — see the
//! sibling `📝️text` facet's module doc for the full rationale). Distinct from the FRAMEWORK's own
//! separate `semio_framework_artifact_infinite_dag::DagSnapshot` codec. This module carries the encode/
//! decode primitives plus the thin artifact-facing `encode`/`decode` wrappers and the pack↔dsl
//! equivalence law.

use crate::{DagFixtureEdge, DagNodeSpec, DagSnapshot};
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `DagSnapshot` to its binary pack form.
pub fn encode(document: &DagSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `DagSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<DagSnapshot, PackError> {
    <DagSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

fn encode_dag_snapshot_binary(s: &DagSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let scene = crate::dag_working_scene(s);
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &dsl::json::to_json_string(&scene.nodes));
    write_str_lp(&mut out, &dsl::json::to_json_string(&scene.edges));
    out
}
fn decode_dag_snapshot_binary(bytes: &[u8]) -> Result<DagSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let nodes: Vec<DagNodeSpec> = dsl::json::from_json_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let edges: Vec<DagFixtureEdge> = dsl::json::from_json_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let content = crate::dag_content_child_with_owner(nodes, edges);
    let snapshot = DagSnapshot { schema, content };
    if reader.read_u8().is_ok() { return Err("trailing DAG snapshot bytes".into()); }
    snapshot.validate()?;
    Ok(snapshot)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactPack
impl store::ArtifactPack for DagSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        self.validate().map_err(PackError::Schema)?;
        let _ = options;
        let raw = encode_dag_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let _ = options;
        decode_dag_snapshot_binary(&inner).map_err(PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactPack

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
