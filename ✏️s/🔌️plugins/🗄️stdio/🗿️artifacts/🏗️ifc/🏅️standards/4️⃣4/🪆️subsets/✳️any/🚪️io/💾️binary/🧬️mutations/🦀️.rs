//! binary rep for stdio.ifc 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v4::subsets::any::schema::mutations::*;
use crate::schema::diff::{self, dec_entity_list_bin, enc_entity_list_bin, IfcDiff};
use crate::standards::v4::subsets::any::io::text::diff::{dec_entity};
use crate::standards::v4::subsets::any::io::text::diff::{enc_entity};
use crate::standards::v4::subsets::any::io::text::diff::{dec_ifc_value};
use crate::standards::v4::subsets::any::io::text::diff::{enc_ifc_value};
use crate::standards::v4::subsets::any::io::binary::diff::{dec_entity_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{enc_entity_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{dec_ifc_value_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{enc_ifc_value_bin};
use crate::standards::v4::subsets::any::io::text::diff::{dec_ifc_value_list};
use crate::standards::v4::subsets::any::io::text::diff::{enc_ifc_value_list};
use crate::standards::v4::subsets::any::io::binary::diff::{dec_ifc_value_list_bin};
use crate::standards::v4::subsets::any::io::binary::diff::{enc_ifc_value_list_bin};
use crate::standards::v2x3::subsets::base::io::text::diff::{strip_brackets};
use crate::standards::v2x3::subsets::base::io::text::diff::{split_top_level};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_str};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_str};
use crate::standards::v2x3::subsets::base::io::binary::diff::{read_str_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{write_str_bin};
use crate::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
use crate::IfcSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};

/// 🧪️ P2-FG1: mutation-specific real binary primitives backing the upgraded `OpBinary` impl below
/// — reuses `IfcDiff`'s `pub(crate)` recursive `enc_entity_bin`/`enc_ifc_value_list_bin`/
/// `write_str_bin` primitives (`../../🔺️diff/🦀️.rs`, imported above) for the SHARED
/// `IfcEntity`/`IfcValue` shape (same intra-artifact-reuse split the TEXT codec above already
/// uses), only `IfcHeader`/`IfcSnapshot`'s own binary shape is genuinely new here.
pub(crate) fn enc_ifc_header_bin(h: &IfcHeader, out: &mut Vec<u8>) {
    enc_ifc_value_list_bin(&h.file_description, out);
    enc_ifc_value_list_bin(&h.file_name, out);
    enc_ifc_value_list_bin(&h.file_schema, out);
}

pub(crate) fn dec_ifc_header_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcHeader, String> {
    let file_description = dec_ifc_value_list_bin(reader)?;
    let file_name = dec_ifc_value_list_bin(reader)?;
    let file_schema = dec_ifc_value_list_bin(reader)?;
    Ok(IfcHeader { file_description, file_name, file_schema })
}

pub(crate) fn enc_ifc_snapshot_bin(s: &IfcSnapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_ifc_header_bin(&s.header, out);
    enc_entity_list_bin(&s.entities, out);
}

pub(crate) fn dec_ifc_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcSnapshot, String> {
    let schema = read_str_bin(reader)?;
    let header = dec_ifc_header_bin(reader)?;
    let entities = dec_entity_list_bin(reader)?;
    Ok(IfcSnapshot { schema, header, entities })
}

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut (`IfcMutation` was one of 4
/// of stdio's 7 FG1 standards still on that shortcut per this wave's own P2-FG1 census). `tag` is
/// the `IfcMutation` variant ordinal, same 1-10 order `parse_ifc_mutation`'s own keyword match
/// uses (tag 0, formerly `NoMutation`, is retired rather than reassigned). Every field is real
/// (`id`/`index` varints, `IfcEntity`/`IfcValue` field-by-field via the reused diff-sibling
/// primitives) — the only place the recursion bottoms out through a fully spec-expressible
/// per-variant tag (`enc_ifc_value_bin`), never an opaque byte-chain fallback.
impl OpBinary for IfcMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            IfcMutation::SetSnapshot(..) => TAG_SET_SNAPSHOT,
            IfcMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
            IfcMutation::SetFileDescription(..) => TAG_SET_FILE_DESCRIPTION,
            IfcMutation::SetFileName(..) => TAG_SET_FILE_NAME,
            IfcMutation::SetFileSchema(..) => TAG_SET_FILE_SCHEMA,
            IfcMutation::InsertEntity(..) => TAG_INSERT_ENTITY,
            IfcMutation::RemoveEntity(..) => TAG_REMOVE_ENTITY,
            IfcMutation::SetEntityName(..) => TAG_SET_ENTITY_NAME,
            IfcMutation::SetEntityArg(..) => TAG_SET_ENTITY_ARG,
            IfcMutation::InsertEntityArg(..) => TAG_INSERT_ENTITY_ARG,
            IfcMutation::RemoveEntityArg(..) => TAG_REMOVE_ENTITY_ARG,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_ifc_snapshot_bin(snapshot, &mut out),
            IfcMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => out.extend(protocol::OpBinary::encode_op(patch)?),
            IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values }) => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::SetFileName(set_file_name::SetFileName { values }) => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values }) => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_entity_bin(entity, &mut out);
            }
            IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => store::pack_rt::write_varint_u64(&mut out, *id),
            IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                write_str_bin(&mut out, name);
            }
            IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_ifc_value_bin(value, &mut out);
            }
            IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_ifc_value_bin(value, &mut out);
            }
            IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_PATCH_SNAPSHOT => Ok(IfcMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: <semio_s_artifact_stdio_contract::editing::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed { what: "patch-snapshot payload", offset: reader.position() as u64, detail: e.to_string() })?)? })),
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_ifc_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            TAG_SET_FILE_DESCRIPTION => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values }))
            }
            TAG_SET_FILE_NAME => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileName(set_file_name::SetFileName { values }))
            }
            TAG_SET_FILE_SCHEMA => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values }))
            }
            TAG_INSERT_ENTITY => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity }))
            }
            TAG_REMOVE_ENTITY => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ok(IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }))
            }
            TAG_SET_ENTITY_NAME => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let name = read_str_bin(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, name }))
            }
            TAG_SET_ENTITY_ARG => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_ifc_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, value }))
            }
            TAG_INSERT_ENTITY_ARG => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_ifc_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, value }))
            }
            TAG_REMOVE_ENTITY_ARG => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `IfcMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_SET_FILE_DESCRIPTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-file-description");
const TAG_SET_FILE_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-file-name");
const TAG_SET_FILE_SCHEMA: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-file-schema");
const TAG_INSERT_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-entity");
const TAG_REMOVE_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-entity");
const TAG_SET_ENTITY_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-entity-name");
const TAG_SET_ENTITY_ARG: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-entity-arg");
const TAG_INSERT_ENTITY_ARG: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-entity-arg");
const TAG_REMOVE_ENTITY_ARG: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-entity-arg");
//#endregion 🏷️WireTags
