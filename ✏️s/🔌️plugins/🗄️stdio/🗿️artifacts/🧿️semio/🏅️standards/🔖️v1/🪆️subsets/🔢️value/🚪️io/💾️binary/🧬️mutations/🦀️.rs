//! 💾️ Binary representation codec surface for `stdio.semio.value` (mutations).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[path="🧭️path/🦀️.rs"]
mod path_codec;
use path_codec::{enc_semio_path_bin,dec_semio_path_bin};

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::value::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::value::schema::diff::{NamedAdded, SemioValueDiff, SemioValueTreeDiff};
use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value};
use crate::standards::v1::subsets::value::io::text::diff::{enc_semio_value};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_node_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_node_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{dec_semio_value_bin};
use crate::standards::v1::subsets::value::io::binary::diff::{enc_semio_value_bin};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{read_str_lp};
use crate::standards::v1::subsets::drawing::io::binary::snapshot::{write_str_lp};
use crate::standards::v1::subsets::value::io::text::diff::{dec_value_id};
use crate::standards::v1::subsets::value::io::text::diff::{enc_value_id};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, ValueId};
#[cfg(test)]
use protocol::command::DiffAlgebra;
use protocol::{Mutation, OpText};

/// 🧪️ Real binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the `print_op().into_bytes()` text-as-binary shortcut this facet started with.
/// `tag` is the `SemioValueMutation` variant ordinal, in the same 0-7 order
/// `print_value_mutation`'s own keyword match uses. Every variant's own path/key/value/id payload
/// is real LEB128-varint-framed binary (never text-as-bytes) — same treatment json's own
/// `JsonMutation::encode_op`/`decode_op` uses.
impl protocol::OpBinary for SemioValueMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            SemioValueMutation::SetValue(set_value::SetValue { .. }) => TAG_SET_VALUE,
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { .. }) => TAG_SET_MAP_ENTRY,
            SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { .. }) => TAG_REMOVE_MAP_ENTRY,
            SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { .. }) => TAG_INSERT_LIST_ITEM,
            SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { .. }) => TAG_REMOVE_LIST_ITEM,
            SemioValueMutation::SetNode(set_node::SetNode { .. }) => TAG_SET_NODE,
            SemioValueMutation::RemoveNode(remove_node::RemoveNode { .. }) => TAG_REMOVE_NODE,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            SemioValueMutation::SetValue(set_value::SetValue { path, value }) => {
                enc_semio_path_bin(path, &mut out);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, value, at }) => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
                enc_semio_value_bin(value, &mut out);
                store::pack_rt::write_varint_u64(&mut out, at.map_or(0, |at| at as u64 + 1));
            }
            SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }) => {
                enc_semio_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
            }
            SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, value }) => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_semio_value_bin(value, &mut out);
            }
            SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }) => {
                enc_semio_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            SemioValueMutation::SetNode(set_node::SetNode { id, value, at }) => {
                write_str_lp(&mut out, &id.value);
                enc_semio_value_bin(value, &mut out);
                store::pack_rt::write_varint_u64(&mut out, at.map_or(0, |at| at as u64 + 1));
            }
            SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }) => {
                write_str_lp(&mut out, &id.value);
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
            TAG_SET_VALUE => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::SetValue(set_value::SetValue { path, value }))
            }
            TAG_SET_MAP_ENTRY => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                let at = reader.read_varint_u64().map_err(|e| malformed("op at", reader.position(), e.to_string()))?.checked_sub(1).map(|at| at as usize);
                Ok(SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path, key, value, at }))
            }
            TAG_REMOVE_MAP_ENTRY => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                Ok(SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path, key }))
            }
            TAG_INSERT_LIST_ITEM => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path, index, value }))
            }
            TAG_REMOVE_LIST_ITEM => {
                let path = dec_semio_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path, index }))
            }
            TAG_SET_NODE => {
                let id = ValueId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                let value = dec_semio_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                let at = reader.read_varint_u64().map_err(|e| malformed("op at", reader.position(), e.to_string()))?.checked_sub(1).map(|at| at as usize);
                Ok(SemioValueMutation::SetNode(set_node::SetNode { id, value, at }))
            }
            TAG_REMOVE_NODE => {
                let id = ValueId::new(read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?);
                Ok(SemioValueMutation::RemoveNode(remove_node::RemoveNode { id }))
            }
            other => Err(malformed("op tag", 1, format!("unknown op tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioValueMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_VALUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-value");
const TAG_SET_MAP_ENTRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-map-entry");
const TAG_REMOVE_MAP_ENTRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-map-entry");
const TAG_INSERT_LIST_ITEM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-list-item");
const TAG_REMOVE_LIST_ITEM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-list-item");
const TAG_SET_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-node");
const TAG_REMOVE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-node");
//#endregion 🏷️WireTags
