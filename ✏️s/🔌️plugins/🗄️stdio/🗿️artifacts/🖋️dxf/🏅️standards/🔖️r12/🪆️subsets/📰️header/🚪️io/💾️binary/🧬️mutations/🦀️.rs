//! binary rep for stdio.dxf 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_r12::subsets::any::io::binary::diff::dec_block_bin;
use crate::standards::v_r12::subsets::any::schema::mutations::*;
use crate::schema::diff::{block_diff_between, // 🧪️ P2-FG1: real recursive binary twins backing the upgraded `OpBinary` impl below (see
    // `🔺️diff/🦀️.rs`'s `#region 🔖️ItemBinaryCodecs`/`#region 🔖️BinaryPrimitives`).
    diff_insert_block, diff_insert_entity, diff_insert_layer, diff_insert_linetype, diff_insert_style, diff_remove_block, diff_remove_entity, diff_remove_header_var, diff_remove_layer, diff_remove_linetype, diff_remove_style, diff_set_block, diff_set_entity, diff_set_header_var, diff_set_layer, diff_set_linetype, diff_set_snapshot, diff_set_style, entity_diff_between_pub, layer_diff_between, linetype_diff_between, style_diff_between, DxfDiff};
use crate::standards::v_r12::subsets::any::io::binary::snapshot::{dec_dxf_snapshot_bin};
use crate::standards::v_r12::subsets::any::io::binary::snapshot::{enc_dxf_snapshot_bin};
use crate::standards::v_r12::subsets::any::io::text::snapshot::{dec_dxf_snapshot};
use crate::standards::v_r12::subsets::any::io::text::snapshot::{enc_dxf_snapshot};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_linetype};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_linetype};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_style};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_style};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_layer};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_layer};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_str};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_str};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_linetype_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_linetype_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_style_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_style_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_layer_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_layer_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{read_str_lp};
use crate::standards::v_r12::subsets::any::io::binary::diff::{write_str_lp};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_block};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_block};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_header_var};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_header_var};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_dxf_entity};
use crate::standards::v_r12::subsets::any::io::text::diff::{enc_dxf_entity};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_block_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_header_var_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_header_var_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{dec_dxf_entity_bin};
use crate::standards::v_r12::subsets::any::io::binary::diff::{enc_dxf_entity_bin};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle};
use crate::DxfSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `DxfMutation` variant ordinal, in the SAME 0-17 order `parse_dxf_mutation`'s own keyword match
/// uses. Every variant payload reuses `🔺️diff/🦀️.rs`'s real recursive binary item
/// codecs (`enc_dxf_snapshot_bin`/`enc_dxf_entity_bin`/`enc_block_bin`/…) — genuinely structured,
/// varint/length-prefixed binary, never text-as-bytes.
impl OpBinary for DxfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            DxfMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
            DxfMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
            DxfMutation::SetHeaderVar(_) => TAG_SET_HEADER_VAR,
            DxfMutation::RemoveHeaderVar(_) => TAG_REMOVE_HEADER_VAR,
            DxfMutation::InsertLayer(_) => TAG_INSERT_LAYER,
            DxfMutation::RemoveLayer(_) => TAG_REMOVE_LAYER,
            DxfMutation::SetLayer(_) => TAG_SET_LAYER,
            DxfMutation::InsertStyle(_) => TAG_INSERT_STYLE,
            DxfMutation::RemoveStyle(_) => TAG_REMOVE_STYLE,
            DxfMutation::SetStyle(_) => TAG_SET_STYLE,
            DxfMutation::InsertLinetype(_) => TAG_INSERT_LINETYPE,
            DxfMutation::RemoveLinetype(_) => TAG_REMOVE_LINETYPE,
            DxfMutation::SetLinetype(_) => TAG_SET_LINETYPE,
            DxfMutation::InsertEntity(_) => TAG_INSERT_ENTITY,
            DxfMutation::RemoveEntity(_) => TAG_REMOVE_ENTITY,
            DxfMutation::SetEntity(_) => TAG_SET_ENTITY,
            DxfMutation::InsertBlock(_) => TAG_INSERT_BLOCK,
            DxfMutation::RemoveBlock(_) => TAG_REMOVE_BLOCK,
            DxfMutation::SetBlock(_) => TAG_SET_BLOCK,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_dxf_snapshot_bin(snapshot, &mut out),
            DxfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => out.extend(protocol::OpBinary::encode_op(patch)?),
            DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name, header_var }) => {
                write_str_lp(&mut out, name);
                enc_header_var_bin(header_var, &mut out);
            }
            DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name }) => write_str_lp(&mut out, name),
            DxfMutation::InsertLayer(insert_layer::InsertLayer { index, layer }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_layer_bin(layer, &mut out);
            }
            DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => write_str_lp(&mut out, name),
            DxfMutation::SetLayer(set_layer::SetLayer { name, layer }) => {
                write_str_lp(&mut out, name);
                enc_layer_bin(layer, &mut out);
            }
            DxfMutation::InsertStyle(insert_style::InsertStyle { index, style }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_style_bin(style, &mut out);
            }
            DxfMutation::RemoveStyle(remove_style::RemoveStyle { name }) => write_str_lp(&mut out, name),
            DxfMutation::SetStyle(set_style::SetStyle { name, style }) => {
                write_str_lp(&mut out, name);
                enc_style_bin(style, &mut out);
            }
            DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index, linetype }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_linetype_bin(linetype, &mut out);
            }
            DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name }) => write_str_lp(&mut out, name),
            DxfMutation::SetLinetype(set_linetype::SetLinetype { name, linetype }) => {
                write_str_lp(&mut out, name);
                enc_linetype_bin(linetype, &mut out);
            }
            DxfMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_dxf_entity_bin(entity, &mut out);
            }
            DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index }) => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            DxfMutation::SetEntity(set_entity::SetEntity { index, entity }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_dxf_entity_bin(entity, &mut out);
            }
            DxfMutation::InsertBlock(insert_block::InsertBlock { index, block }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
            }
            DxfMutation::RemoveBlock(remove_block::RemoveBlock { index }) => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            DxfMutation::SetBlock(set_block::SetBlock { index, block }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
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
            TAG_PATCH_SNAPSHOT => Ok(DxfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: <semio_s_artifact_stdio_contract::editing::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed { what: "patch-snapshot payload", offset: reader.position() as u64, detail: e.to_string() })?)? })),
            TAG_SET_SNAPSHOT => Ok(DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_dxf_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))? })),
            TAG_SET_HEADER_VAR => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let header_var = dec_header_var_bin(&mut reader).map_err(|e| malformed("op header_var", reader.position(), e))?;
                Ok(DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name, header_var }))
            }
            TAG_REMOVE_HEADER_VAR => Ok(DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? })),
            TAG_INSERT_LAYER => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let layer = dec_layer_bin(&mut reader).map_err(|e| malformed("op layer", reader.position(), e))?;
                Ok(DxfMutation::InsertLayer(insert_layer::InsertLayer { index, layer }))
            }
            TAG_REMOVE_LAYER => Ok(DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? })),
            TAG_SET_LAYER => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let layer = dec_layer_bin(&mut reader).map_err(|e| malformed("op layer", reader.position(), e))?;
                Ok(DxfMutation::SetLayer(set_layer::SetLayer { name, layer }))
            }
            TAG_INSERT_STYLE => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let style = dec_style_bin(&mut reader).map_err(|e| malformed("op style", reader.position(), e))?;
                Ok(DxfMutation::InsertStyle(insert_style::InsertStyle { index, style }))
            }
            TAG_REMOVE_STYLE => Ok(DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? })),
            TAG_SET_STYLE => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let style = dec_style_bin(&mut reader).map_err(|e| malformed("op style", reader.position(), e))?;
                Ok(DxfMutation::SetStyle(set_style::SetStyle { name, style }))
            }
            TAG_INSERT_LINETYPE => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let linetype = dec_linetype_bin(&mut reader).map_err(|e| malformed("op linetype", reader.position(), e))?;
                Ok(DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index, linetype }))
            }
            TAG_REMOVE_LINETYPE => Ok(DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? })),
            TAG_SET_LINETYPE => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let linetype = dec_linetype_bin(&mut reader).map_err(|e| malformed("op linetype", reader.position(), e))?;
                Ok(DxfMutation::SetLinetype(set_linetype::SetLinetype { name, linetype }))
            }
            TAG_INSERT_ENTITY => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_dxf_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(DxfMutation::InsertEntity(insert_entity::InsertEntity { index, entity }))
            }
            TAG_REMOVE_ENTITY => Ok(DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize })),
            TAG_SET_ENTITY => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_dxf_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(DxfMutation::SetEntity(set_entity::SetEntity { index, entity }))
            }
            TAG_INSERT_BLOCK => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DxfMutation::InsertBlock(insert_block::InsertBlock { index, block }))
            }
            TAG_REMOVE_BLOCK => Ok(DxfMutation::RemoveBlock(remove_block::RemoveBlock { index: reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize })),
            TAG_SET_BLOCK => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DxfMutation::SetBlock(set_block::SetBlock { index, block }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `DxfMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_SET_HEADER_VAR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-header-var");
const TAG_REMOVE_HEADER_VAR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-header-var");
const TAG_INSERT_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-layer");
const TAG_REMOVE_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-layer");
const TAG_SET_LAYER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-layer");
const TAG_INSERT_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-style");
const TAG_REMOVE_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-style");
const TAG_SET_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-style");
const TAG_INSERT_LINETYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-linetype");
const TAG_REMOVE_LINETYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-linetype");
const TAG_SET_LINETYPE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-linetype");
const TAG_INSERT_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-entity");
const TAG_REMOVE_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-entity");
const TAG_SET_ENTITY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-entity");
const TAG_INSERT_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-block");
const TAG_REMOVE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-block");
const TAG_SET_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-block");
//#endregion 🏷️WireTags
