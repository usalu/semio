//! binary rep for stdio.stl 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ascii::subsets::any::schema::mutations::*;
use crate::schema::diff::{self, StlDiff};
use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// 🧪️ P2-FG1-FIX: real recursive binary twin of [`enc_snapshot`]/[`dec_snapshot`] above —
/// `StlSnapshot` is genuinely flat (`Vec<StlTriangle>`, no self-recursion), so this is real
/// varint-framed binary all the way down, reusing `diff`'s `pub(crate)` binary value codecs
/// (`enc_triangle_bin`/`dec_triangle_bin`) rather than duplicating them — same intra-artifact
/// reuse pattern this file's own text `enc_snapshot` already establishes over `crate::standards::v_ascii::subsets::any::io::text::diff::enc_triangle`.
/// 🧪️ P2-FG1-FIX: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the prior `print_stl_op(self).into_bytes()` text-as-binary shortcut. `tag` is
/// the `StlMutation` variant's declaration-order ordinal (1=`SetSolidName` .. 5=
/// `SetTriangleVertices`, same order `enum StlMutation` declares them; `0` is retired along with
/// `NoMutation`, not reused). Every variant's payload is real field-by-field binary
/// (`write_varint_u64` for `index: usize`, `write_f64_bin`/`enc_vec3_bin`/`enc_vertices_bin`/
/// `enc_triangle_bin`/`enc_snapshot_bin` for the rest) — `StlMutation`'s payload tree has ZERO
/// self-recursion, so nothing here is opaque at the Rust layer; only the protocol-dialect file
/// still frames the payload as one opaque trailing chain (`InsertTriangle`'s triangle record is a
/// variable-shape record, the same `protocol-array-of-records` `walk_protocol` gap the
/// sibling diff protocol file documents).
impl OpBinary for StlMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, 0u8];
        let tag: u8 = match self {
            StlMutation::SetSolidName(set_solid_name::SetSolidName { name }) => {
                crate::standards::v_ascii::subsets::any::io::binary::diff::write_str_bin(&mut out, name);
                TAG_SET_SOLID_NAME
            }
            StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                crate::standards::v_ascii::subsets::any::io::binary::diff::enc_triangle_bin(triangle, &mut out);
                TAG_INSERT_TRIANGLE
            }
            StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                TAG_REMOVE_TRIANGLE
            }
            StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                crate::standards::v_ascii::subsets::any::io::binary::diff::enc_vec3_bin(normal, &mut out);
                TAG_SET_TRIANGLE_NORMAL
            }
            StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                crate::standards::v_ascii::subsets::any::io::binary::diff::enc_vertices_bin(vertices, &mut out);
                TAG_SET_TRIANGLE_VERTICES
            }
        };
        out[1] = tag;
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let _format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: e.to_string() })?;
        let tag = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: e.to_string() })?;
        match tag {
            TAG_SET_SOLID_NAME => {
                let name = crate::standards::v_ascii::subsets::any::io::binary::diff::read_str_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op name", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::SetSolidName(set_solid_name::SetSolidName { name }))
            }
            TAG_INSERT_TRIANGLE => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                let triangle = crate::standards::v_ascii::subsets::any::io::binary::diff::dec_triangle_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op triangle", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }))
            }
            TAG_REMOVE_TRIANGLE => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                Ok(StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }))
            }
            TAG_SET_TRIANGLE_NORMAL => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                let normal = crate::standards::v_ascii::subsets::any::io::binary::diff::dec_vec3_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op normal", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }))
            }
            TAG_SET_TRIANGLE_VERTICES => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                let vertices = crate::standards::v_ascii::subsets::any::io::binary::diff::dec_vertices_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op vertices", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }))
            }
            other => Err(protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("unknown tag {other}") }),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `StlMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SOLID_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-solid-name");
const TAG_INSERT_TRIANGLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-triangle");
const TAG_REMOVE_TRIANGLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-triangle");
const TAG_SET_TRIANGLE_NORMAL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-triangle-normal");
const TAG_SET_TRIANGLE_VERTICES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-triangle-vertices");
//#endregion 🏷️WireTags
