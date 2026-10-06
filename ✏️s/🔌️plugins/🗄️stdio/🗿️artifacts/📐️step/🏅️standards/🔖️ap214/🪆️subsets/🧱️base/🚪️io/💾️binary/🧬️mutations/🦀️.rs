//! binary rep for stdio.step 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::mutations::*;
use crate::schema::diff::{diff_set_snapshot, StepArgAdded, StepArgModified, StepArgsDiff, StepDiff, StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified};
use crate::standards::v_ap214::subsets::base::io::binary::snapshot::{dec_step_snapshot_bin};
use crate::standards::v_ap214::subsets::base::io::binary::snapshot::{enc_step_snapshot_bin};
use crate::standards::v_ap214::subsets::base::io::text::snapshot::{dec_step_snapshot};
use crate::standards::v_ap214::subsets::base::io::text::snapshot::{enc_step_snapshot};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_value};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_value};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_value_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_value_bin};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_entity};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_entity};
use crate::standards::v_ap214::subsets::base::io::text::diff::{parse_u64};
use crate::standards::v_ap214::subsets::base::io::text::diff::{parse_usize};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_str};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_str};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_entity_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_entity_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{read_str_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{write_str_bin};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_file_schema};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_file_schema};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_file_name};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_file_name};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_file_description};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_file_description};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_file_schema_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_file_schema_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_file_name_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_file_name_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_file_description_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_file_description_bin};
use crate::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::StepSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `StepMutation` variant ordinal, same 0-9 order `print_step_mutation`'s own keyword match uses
/// (shifted down by one from the pre-migration 1-10 numbering: `NoMutation`'s tag `0` had no
/// leaf and is gone with the variant).
/// Reuses `StepDiff`'s `pub(crate)` recursive `enc_value_bin`/`enc_entity_bin`/
/// `enc_step_snapshot_bin`/`write_str_bin` primitives (`../../🔺️diff/🦀️.rs`, imported
/// above) — same intra-artifact-reuse split the TEXT codec above already uses.
impl OpBinary for StepMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            StepMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
            StepMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
            StepMutation::SetFileDescription(_) => TAG_SET_FILE_DESCRIPTION,
            StepMutation::SetFileName(_) => TAG_SET_FILE_NAME,
            StepMutation::SetFileSchema(_) => TAG_SET_FILE_SCHEMA,
            StepMutation::InsertEntity(_) => TAG_INSERT_ENTITY,
            StepMutation::RemoveEntity(_) => TAG_REMOVE_ENTITY,
            StepMutation::SetEntityName(_) => TAG_SET_ENTITY_NAME,
            StepMutation::SetEntityArg(_) => TAG_SET_ENTITY_ARG,
            StepMutation::InsertEntityArg(_) => TAG_INSERT_ENTITY_ARG,
            StepMutation::RemoveEntityArg(_) => TAG_REMOVE_ENTITY_ARG,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_step_snapshot_bin(snapshot, &mut out),
            StepMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => out.extend(protocol::OpBinary::encode_op(patch)?),
            StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description }) => enc_file_description_bin(file_description, &mut out),
            StepMutation::SetFileName(set_file_name::SetFileName { file_name }) => enc_file_name_bin(file_name, &mut out),
            StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema }) => enc_file_schema_bin(file_schema, &mut out),
            StepMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_entity_bin(entity, &mut out);
            }
            StepMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => store::pack_rt::write_varint_u64(&mut out, *id),
            StepMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                write_str_bin(&mut out, name);
            }
            StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, arg_index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *arg_index as u64);
                enc_value_bin(value, &mut out);
            }
            StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, arg_index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *arg_index as u64);
                enc_value_bin(value, &mut out);
            }
            StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, arg_index }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *arg_index as u64);
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
            TAG_PATCH_SNAPSHOT => Ok(StepMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: <semio_s_artifact_stdio_contract::editing::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed { what: "patch-snapshot payload", offset: reader.position() as u64, detail: e.to_string() })?)? })),
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_step_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            TAG_SET_FILE_DESCRIPTION => {
                let file_description = dec_file_description_bin(&mut reader).map_err(|e| malformed("op file_description", reader.position(), e))?;
                Ok(StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description }))
            }
            TAG_SET_FILE_NAME => {
                let file_name = dec_file_name_bin(&mut reader).map_err(|e| malformed("op file_name", reader.position(), e))?;
                Ok(StepMutation::SetFileName(set_file_name::SetFileName { file_name }))
            }
            TAG_SET_FILE_SCHEMA => {
                let file_schema = dec_file_schema_bin(&mut reader).map_err(|e| malformed("op file_schema", reader.position(), e))?;
                Ok(StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema }))
            }
            TAG_INSERT_ENTITY => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(StepMutation::InsertEntity(insert_entity::InsertEntity { index, entity }))
            }
            TAG_REMOVE_ENTITY => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ok(StepMutation::RemoveEntity(remove_entity::RemoveEntity { id }))
            }
            TAG_SET_ENTITY_NAME => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let name = read_str_bin(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(StepMutation::SetEntityName(set_entity_name::SetEntityName { id, name }))
            }
            TAG_SET_ENTITY_ARG => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let arg_index = reader.read_varint_u64().map_err(|e| malformed("op arg_index", reader.position(), e.to_string()))? as usize;
                let value = dec_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, arg_index, value }))
            }
            TAG_INSERT_ENTITY_ARG => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let arg_index = reader.read_varint_u64().map_err(|e| malformed("op arg_index", reader.position(), e.to_string()))? as usize;
                let value = dec_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, arg_index, value }))
            }
            TAG_REMOVE_ENTITY_ARG => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let arg_index = reader.read_varint_u64().map_err(|e| malformed("op arg_index", reader.position(), e.to_string()))? as usize;
                Ok(StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, arg_index }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `StepMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
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
