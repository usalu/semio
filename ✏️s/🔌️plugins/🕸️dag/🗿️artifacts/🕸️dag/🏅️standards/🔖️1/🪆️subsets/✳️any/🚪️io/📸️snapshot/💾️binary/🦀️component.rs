//! 📦️ DAG artifact — native binary codec (`impl store::ArtifactPack for DagSnapshot`), moved here
//! wholesale from the old `🧬️schema/📸️snapshot` codec home (design.md §1 CORRECTION — see the
//! sibling `📝️text` facet's module doc for the full rationale). Distinct from the FRAMEWORK's own
//! separate `infinite_board_port_directed_dag::DagSnapshot` codec. This module carries the encode/
//! decode primitives plus the thin artifact-facing `encode`/`decode` wrappers and the pack↔dsl
//! equivalence law.

use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec, DagSnapshot, DAG_DOCUMENT_SCHEMA};
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
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
    let scene = crate::artifacts::dag::dag_working_scene(s);
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &serde_json::to_string(&scene.nodes).unwrap_or_default());
    write_str_lp(&mut out, &serde_json::to_string(&scene.edges).unwrap_or_default());
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
    let nodes: Vec<DagNodeSpec> = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let edges: Vec<DagFixtureEdge> = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let content = crate::artifacts::dag::dag_content_child_handle_and_cache(nodes, edges);
    Ok(DagSnapshot { schema, content })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactPack
impl store::ArtifactPack for DagSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_dag_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let mut snapshot = decode_dag_snapshot_binary(&inner).map_err(store::PackError::Schema)?;
        snapshot.schema = DAG_DOCUMENT_SCHEMA.into();
        Ok(snapshot)
    }
}
//#endregion 🔖️HandcraftedArtifactPack

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::dag::dsl;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `DagMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing dsl/pack round-trip law (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::dag::op::DagMutation;
        use crate::artifacts::dag::DAG_DOCUMENT_SCHEMA;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let document = DagSnapshot { schema: DAG_DOCUMENT_SCHEMA.into(), content: crate::artifacts::dag::dag_content_child_handle_and_cache(Vec::new(), Vec::new()) };
        let mut store: ArtifactStore<DagSnapshot, DagMutation> = ArtifactStore::new(create_document_envelope(DAG_DOCUMENT_SCHEMA, "dag-demo", document, None)).expect("valid artifact store fixture");
        let node = crate::artifacts::dag::schema::default_node_for_kind("note", "node-1", 0.0, 0.0);
        store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::dag::mutations::create_node(node)], description: None }).expect("apply");
        let edit: &Edit<DagMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<DagSnapshot, DagMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::dag::dsl::parse_dsl(crate::artifacts::dag::dsl::DAG_EXAMPLE_TEXT)
            .expect("parse fixture");
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
}
