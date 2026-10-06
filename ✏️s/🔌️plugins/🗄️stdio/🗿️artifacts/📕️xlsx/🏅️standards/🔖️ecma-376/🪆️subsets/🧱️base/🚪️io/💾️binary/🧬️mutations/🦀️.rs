//! xlsx rep for stdio.xlsx 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::*;
use crate::schema::diff::{dec_cell_value, dec_cell_value_bin, dec_sheet, dec_sheet_bin, dec_str, diff_set_snapshot, enc_cell_value, enc_cell_value_bin, enc_sheet, enc_sheet_bin, enc_str, read_str_lp, write_str_lp, XlsxDiff};
#[cfg(test)]
use crate::schema::snapshot::XlsxCell;
#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCellValue, XlsxSheet};
use crate::XlsxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcRelationship;
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcTargetMode;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xlsx_snapshot_bin(s: &XlsxSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &semio_framework_pack_json::to_json_string(s));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xlsx_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSnapshot, String> {
    semio_framework_pack_json::from_json_str(&read_str_lp(reader)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut (confirmed still on that
/// shortcut live by direct read of this file before this wave, not assumed). `tag` is the
/// `XlsxMutation` variant ordinal, in the SAME 0-9 order `print_xlsx_mutation`'s own keyword
/// match uses.
impl OpBinary for XlsxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            XlsxMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
            XlsxMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
            XlsxMutation::InsertSheet(_) => TAG_INSERT_SHEET,
            XlsxMutation::RemoveSheet(_) => TAG_REMOVE_SHEET,
            XlsxMutation::RenameSheet(_) => TAG_RENAME_SHEET,
            XlsxMutation::SetCell(_) => TAG_SET_CELL,
            XlsxMutation::InsertCell(_) => TAG_INSERT_CELL,
            XlsxMutation::RemoveCell(_) => TAG_REMOVE_CELL,
            XlsxMutation::InsertSharedString(_) => TAG_INSERT_SHARED_STRING,
            XlsxMutation::RemoveSharedString(_) => TAG_REMOVE_SHARED_STRING,
            XlsxMutation::SetSharedString(_) => TAG_SET_SHARED_STRING,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_xlsx_snapshot_bin(snapshot, &mut out),
            XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => out.extend(protocol::OpBinary::encode_op(patch)?),
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => enc_sheet_bin(sheet, &mut out),
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => write_str_lp(&mut out, name),
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => {
                write_str_lp(&mut out, name);
                write_str_lp(&mut out, new_name);
            }
            XlsxMutation::SetCell(set_cell::SetCell { address, value }) => {
                write_str_lp(&mut out, &semio_framework_pack_json::to_json_string(address));
                enc_cell_value_bin(value, &mut out);
            }
            XlsxMutation::InsertCell(insert_cell::InsertCell { address, value }) => {
                write_str_lp(&mut out, &semio_framework_pack_json::to_json_string(address));
                enc_cell_value_bin(value, &mut out);
            }
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }) => write_str_lp(&mut out, &semio_framework_pack_json::to_json_string(address)),
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => write_str_lp(&mut out, value),
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                write_str_lp(&mut out, value);
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
            TAG_PATCH_SNAPSHOT => Ok(XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot {
                patch: <semio_s_artifact_stdio_contract::editing::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed {
                    what: "patch-snapshot payload",
                    offset: reader.position() as u64,
                    detail: e.to_string(),
                })?)?,
            })),
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_xlsx_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            TAG_INSERT_SHEET => {
                let sheet = dec_sheet_bin(&mut reader).map_err(|e| malformed("op sheet", reader.position(), e))?;
                Ok(XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }))
            }
            TAG_REMOVE_SHEET => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }))
            }
            TAG_RENAME_SHEET => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let new_name = read_str_lp(&mut reader).map_err(|e| malformed("op new_name", reader.position(), e))?;
                Ok(XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }))
            }
            TAG_SET_CELL => {
                let address = semio_framework_pack_json::from_json_str(&read_str_lp(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| malformed("op address", reader.position(), error.to_string()))?;
                let value = dec_cell_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::SetCell(set_cell::SetCell { address, value }))
            }
            TAG_INSERT_CELL => {
                let address = semio_framework_pack_json::from_json_str(&read_str_lp(&mut reader).map_err(|e| malformed("op vacancy address", reader.position(), e))?, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| malformed("op vacancy address", reader.position(), error.to_string()))?;
                let value = dec_cell_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::InsertCell(insert_cell::InsertCell { address, value }))
            }
            TAG_REMOVE_CELL => {
                let address = semio_framework_pack_json::from_json_str(&read_str_lp(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| malformed("op address", reader.position(), error.to_string()))?;
                Ok(XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }))
            }
            TAG_INSERT_SHARED_STRING => {
                let value = read_str_lp(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }))
            }
            TAG_REMOVE_SHARED_STRING => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }))
            }
            TAG_SET_SHARED_STRING => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = read_str_lp(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }))
            }
            other => Err(malformed("op tag", 1, format!("unknown XlsxMutation tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `XlsxMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_INSERT_SHEET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-sheet");
const TAG_REMOVE_SHEET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-sheet");
const TAG_RENAME_SHEET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-sheet");
const TAG_SET_CELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-cell");
const TAG_INSERT_CELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-cell");
const TAG_REMOVE_CELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-cell");
const TAG_INSERT_SHARED_STRING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-shared-string");
const TAG_REMOVE_SHARED_STRING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-shared-string");
const TAG_SET_SHARED_STRING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-shared-string");
//#endregion 🏷️WireTags
