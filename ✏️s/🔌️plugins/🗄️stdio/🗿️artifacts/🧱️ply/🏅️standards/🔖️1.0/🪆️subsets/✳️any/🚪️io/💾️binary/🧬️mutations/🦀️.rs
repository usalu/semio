//! binary rep for stdio.ply 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::mutations::*;

use crate::standards::v1_0::subsets::any::io::text::diff::{dec_value};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_value};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_value};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_value};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_row};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_row};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_row};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_row};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_element};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_element};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_element};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_element};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_format};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_format};
use crate::standards::v1_0::subsets::any::io::text::diff::{dec_str};
use crate::standards::v1_0::subsets::any::io::text::diff::{enc_str};
use crate::standards::v1_0::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v1_0::subsets::any::io::text::diff::{split_top_level};
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_str};
use crate::standards::v1_0::subsets::any::io::binary::diff::{write_bin_str};
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyRow, PlyValue};
use crate::PlySnapshot;
use protocol::Mutation;
use protocol::OpBinary;
use protocol::OpText;

/// 🧪️ P2-FG3: real binary op frame — upgraded from the F6-era `print_op().into_bytes()`
/// text-as-binary shortcut. Matches `../💾️binary/📡️.protocol.semio`'s `header fixed 2 |
/// field format u8 | field tag u8 | chain payload bytes` shape exactly — `format` is
/// `store::pack_rt::OP_BINARY_FORMAT`, `tag` is the `PlyMutation` variant ordinal in the SAME
/// 0-8 order `print_ply_mutation`'s own match uses, then each variant's own payload real binary
/// (reusing `PlyDiff`'s `pub(crate)` binary primitives — `write_bin_element`/`write_bin_row`/
/// `write_bin_snapshot`/`write_bin_str`/`write_bin_value` — the same way this file's `OpText`
/// already reuses the text-codec primitives).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn op_tag(m: &PlyMutation) -> u8 {
    match m {
        PlyMutation::SetFormat(..) => TAG_SET_FORMAT,
        PlyMutation::InsertComment(..) => TAG_INSERT_COMMENT,
        PlyMutation::RemoveComment(..) => TAG_REMOVE_COMMENT,
        PlyMutation::AddElement(..) => TAG_ADD_ELEMENT,
        PlyMutation::RemoveElement(..) => TAG_REMOVE_ELEMENT,
        PlyMutation::InsertRow(..) => TAG_INSERT_ROW,
        PlyMutation::RemoveRow(..) => TAG_REMOVE_ROW,
        PlyMutation::SetRowProperty(..) => TAG_SET_ROW_PROPERTY,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn op_pack_err(e: &dsl::PackRefusal) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "ply op binary", offset: 0, detail: e.to_string() }
}

impl OpBinary for PlyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        w.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        w.write_u8(op_tag(self));
        match self {
            PlyMutation::SetFormat(set_format::SetFormat { format }) => {
                crate::standards::v1_0::subsets::any::io::binary::diff::write_bin_format(&mut w, *format);
            }
            PlyMutation::InsertComment(insert_comment::InsertComment { index, comment }) => {
                w.write_varint_u64(*index as u64);
                write_bin_str(&mut w, comment);
            }
            PlyMutation::RemoveComment(remove_comment::RemoveComment { index }) => w.write_varint_u64(*index as u64),
            PlyMutation::AddElement(add_element::AddElement { index, element }) => {
                w.write_varint_u64(*index as u64);
                write_bin_element(&mut w, element);
            }
            PlyMutation::RemoveElement(remove_element::RemoveElement { name }) => write_bin_str(&mut w, name),
            PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, row }) => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*index as u64);
                write_bin_row(&mut w, row);
            }
            PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }) => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*index as u64);
            }
            PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, value }) => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*row_index as u64);
                write_bin_str(&mut w, property_name);
                write_bin_value(&mut w, value);
            }
        }
        Ok(w.into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let _format = r.read_u8().map_err(|error| op_pack_err(&error))?;
        let tag = r.read_u8().map_err(|error| op_pack_err(&error))?;
        match tag {
            TAG_SET_FORMAT => Ok(PlyMutation::SetFormat(set_format::SetFormat { format: crate::standards::v1_0::subsets::any::io::binary::diff::read_bin_format(&mut r).map_err(|error| op_pack_err(&error))? })),
            TAG_INSERT_COMMENT => {
                let index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                let comment = read_bin_str(&mut r).map_err(|error| op_pack_err(&error))?;
                Ok(PlyMutation::InsertComment(insert_comment::InsertComment { index, comment }))
            }
            TAG_REMOVE_COMMENT => Ok(PlyMutation::RemoveComment(remove_comment::RemoveComment { index: r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize })),
            TAG_ADD_ELEMENT => {
                let index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                let element = read_bin_element(&mut r).map_err(|error| op_pack_err(&error))?;
                Ok(PlyMutation::AddElement(add_element::AddElement { index, element }))
            }
            TAG_REMOVE_ELEMENT => Ok(PlyMutation::RemoveElement(remove_element::RemoveElement { name: read_bin_str(&mut r).map_err(|error| op_pack_err(&error))? })),
            TAG_INSERT_ROW => {
                let element_name = read_bin_str(&mut r).map_err(|error| op_pack_err(&error))?;
                let index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                let row = read_bin_row(&mut r).map_err(|error| op_pack_err(&error))?;
                Ok(PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, row }))
            }
            TAG_REMOVE_ROW => {
                let element_name = read_bin_str(&mut r).map_err(|error| op_pack_err(&error))?;
                let index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                Ok(PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }))
            }
            TAG_SET_ROW_PROPERTY => {
                let element_name = read_bin_str(&mut r).map_err(|error| op_pack_err(&error))?;
                let row_index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                let property_name = read_bin_str(&mut r).map_err(|error| op_pack_err(&error))?;
                let value = read_bin_value(&mut r).map_err(|error| op_pack_err(&error))?;
                Ok(PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, value }))
            }
            other => Err(protocol::ProtocolError::Malformed { what: "ply op tag", offset: 1, detail: format!("unknown tag {other}") }),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `PlyMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_FORMAT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-format");
const TAG_INSERT_COMMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-comment");
const TAG_REMOVE_COMMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-comment");
const TAG_ADD_ELEMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "add-element");
const TAG_REMOVE_ELEMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-element");
const TAG_INSERT_ROW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-row");
const TAG_REMOVE_ROW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-row");
const TAG_SET_ROW_PROPERTY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-row-property");
//#endregion 🏷️WireTags
