//! 💾️ Binary representation codec surface for `stdio.semio.model` (mutations).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::schema::mutations::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use crate::schema::diff::{ModelRelationDiff, SemioModelDiff, SemioModelElementDiff, SpatialNodeDiff};
use semio_s_artifact_stdio_semio_base::io::text::codec::parse_f64;
use crate::io::text::snapshot::dec_relation;
use crate::io::text::snapshot::enc_relation;
use crate::io::text::snapshot::dec_relation_kind;
use crate::io::text::snapshot::enc_relation_kind;
use crate::io::text::snapshot::dec_element;
use crate::io::text::snapshot::enc_element;
use crate::io::text::snapshot::dec_spatial_node;
use crate::io::text::snapshot::enc_spatial_node;
use crate::io::text::snapshot::dec_property_set;
use crate::io::text::snapshot::enc_property_set;
use crate::io::text::snapshot::dec_geometry_ref;
use crate::io::text::snapshot::enc_geometry_ref;
use crate::io::text::snapshot::dec_element_class;
use crate::io::text::snapshot::enc_element_class;
use crate::io::text::snapshot::dec_spatial_kind;
use crate::io::text::snapshot::enc_spatial_kind;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_transform;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_transform;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_list;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_list;
use semio_s_artifact_stdio_semio_base::io::text::codec::decode_option;
use semio_s_artifact_stdio_semio_base::io::text::codec::encode_option;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use crate::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, PropertySet, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioModelMutation` block below
/// calls `Self::parse_op(...)` via trait method syntax, which needs `OpText` in scope in
/// production code too, not merely under `#[cfg(test)]` (same fix `stdio.semio.flow`'s own
/// mutations facet needed).
use protocol::{OpBinary, OpText};
use semio_s_artifact_stdio_contract::deserialize_double_option;
use crate::io::text::mutations::print_semio_model_mutation;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wire_tag(m: &SemioModelMutation) -> u8 {
    match m {
        SemioModelMutation::InsertSpatialNode(_) => TAG_INSERT_SPATIAL_NODE,
        SemioModelMutation::RemoveSpatialNode(_) => TAG_REMOVE_SPATIAL_NODE,
        SemioModelMutation::SetSpatialNode(_) => TAG_SET_SPATIAL_NODE,
        SemioModelMutation::InsertElement(_) => TAG_INSERT_ELEMENT,
        SemioModelMutation::RemoveElement(_) => TAG_REMOVE_ELEMENT,
        SemioModelMutation::SetElement(_) => TAG_SET_ELEMENT,
        SemioModelMutation::InsertRelation(_) => TAG_INSERT_RELATION,
        SemioModelMutation::RemoveRelation(_) => TAG_REMOVE_RELATION,
        SemioModelMutation::SetRelation(_) => TAG_SET_RELATION,
        SemioModelMutation::DragElements(_) => TAG_DRAG_ELEMENTS,
        SemioModelMutation::RotateElements(_) => TAG_ROTATE_ELEMENTS,
        SemioModelMutation::ScaleElements(_) => TAG_SCALE_ELEMENTS,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_semio_model_mutation` — the binary frame's
/// `tag` byte already carries the keyword, so the text keyword itself is redundant in the binary
/// payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_model_mutation_args(m: &SemioModelMutation) -> String {
    match print_semio_model_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ P2 pilot (model): real binary op frame, replacing the old `serde_json::to_vec`/`from_slice`
/// shortcut. `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_semio_model_mutation`/`parse_semio_model_mutation` text codec rather than re-deriving a
/// second independent encoding.
impl OpBinary for SemioModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_semio_model_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
#[cfg(test)] pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioModelMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_INSERT_SPATIAL_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-spatial-node");
const TAG_REMOVE_SPATIAL_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-spatial-node");
const TAG_SET_SPATIAL_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-spatial-node");
const TAG_INSERT_ELEMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-element");
const TAG_REMOVE_ELEMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-element");
const TAG_SET_ELEMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-element");
const TAG_INSERT_RELATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-relation");
const TAG_REMOVE_RELATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-relation");
const TAG_SET_RELATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-relation");
const TAG_DRAG_ELEMENTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "drag-elements");
const TAG_ROTATE_ELEMENTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rotate-elements");
const TAG_SCALE_ELEMENTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "scale-elements");
//#endregion 🏷️WireTags

#[path = "🫳️borrowed/🦀️.rs"]
mod borrowed;

/// 📦️ Reborrows original typed patch or imported element fields for bounded prestage receipts.
pub fn prepared_operation_wire_source(mutation: &crate::schema::mutations::SemioModelMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    use crate::schema::mutations::SemioModelMutation;
    use semio_framework_plugin::plugin_app_close_prelude::store::ArtifactPreparedOperationSource;
    Some(match mutation {
        SemioModelMutation::InsertElement(_) => ArtifactPreparedOperationSource::Text { header: &[1, TAG_INSERT_ELEMENT], body: mutation },
        SemioModelMutation::RemoveElement(_) => ArtifactPreparedOperationSource::Text { header: &[1, TAG_REMOVE_ELEMENT], body: mutation },
        _ => return None,
    })
}
