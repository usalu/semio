//! xlsx rep for stdio.xlsx 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxXmlPart};
use crate::XlsxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::diff::XmlDiff;
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use std::collections::BTreeMap;

impl protocol::DiffBinary for XlsxDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut bytes = vec![store::pack_rt::OP_BINARY_FORMAT];
    bytes.extend_from_slice(semio_framework_pack_json::to_json_string(self).as_bytes());
    Ok(bytes)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let payload = bytes.get(1..).ok_or_else(|| protocol::ProtocolError::Malformed { what: "xlsx diff", offset: 0, detail: "missing format byte".into() })?;
    let text = std::str::from_utf8(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "xlsx diff", offset: 1, detail: error.to_string() })?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "xlsx diff", offset: 1, detail: error.to_string() })
}
}
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::diff::*;
#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxXmlPart};
use crate::XlsxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::diff::XmlDiff;
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use std::collections::BTreeMap;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

/// 🔢️ `XlsxCellValue` (data-carrying enum) -- 1-byte kind tag (`0`=Number/`1`=SharedString/
/// `2`=InlineString/`3`=Boolean/`4`=Formula/`5`=Empty), matching `enc_cell_value`'s own
/// `N`/`S`/`I`/`B`/`F`/`E` text-tag numbering.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cell_value_bin(v: &XlsxCellValue, out: &mut Vec<u8>) {
    match v {
        XlsxCellValue::Number(n) => {
            out.push(0);
            out.extend_from_slice(&n.to_le_bytes());
        }
        XlsxCellValue::SharedString(i) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, *i as u64);
        }
        XlsxCellValue::InlineString(s) => {
            out.push(2);
            write_str_lp(out, s);
        }
        XlsxCellValue::Boolean(b) => {
            out.push(3);
            out.push(*b as u8);
        }
        XlsxCellValue::Error(error) => {
            out.push(6);
            write_str_lp(out, error);
        }
        XlsxCellValue::Formula { expr, cached } => {
            out.push(4);
            write_str_lp(out, expr);
            out.push(if cached.is_some() { 1 } else { 0 });
            if let Some(c) = cached {
                enc_cell_value_bin(c, out);
            }
        }
        XlsxCellValue::Empty => out.push(5),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cell_value_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxCellValue, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => {
            let bytes = reader.read_bytes(8).map_err(|e| e.to_string())?;
            let arr: [u8; 8] = bytes.try_into().map_err(|_| "cell value binary: short f64".to_string())?;
            Ok(XlsxCellValue::Number(f64::from_le_bytes(arr)))
        }
        1 => Ok(XlsxCellValue::SharedString(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
        2 => Ok(XlsxCellValue::InlineString(read_str_lp(reader)?)),
        3 => Ok(XlsxCellValue::Boolean(reader.read_u8().map_err(|e| e.to_string())? != 0)),
        4 => {
            let expr = read_str_lp(reader)?;
            let cached = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(Box::new(dec_cell_value_bin(reader)?)) } else { None };
            Ok(XlsxCellValue::Formula { expr, cached })
        }
        5 => Ok(XlsxCellValue::Empty),
        6 => Ok(XlsxCellValue::Error(read_str_lp(reader)?)),
        other => Err(format!("cell value binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cell_bin(c: &XlsxCell, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, c.row as u64);
    store::pack_rt::write_varint_u64(out, c.col as u64);
    enc_cell_value_bin(&c.value, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cell_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxCell, String> {
    let row = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let col = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let value = dec_cell_value_bin(reader)?;
    Ok(XlsxCell { row, col, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sheet_bin(s: &XlsxSheet, out: &mut Vec<u8>) {
    write_str_lp(out, &s.name);
    store::pack_rt::write_varint_u64(out, s.cells.len() as u64);
    for c in &s.cells {
        enc_cell_bin(c, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sheet_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSheet, String> {
    let name = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(count as usize);
    for _ in 0..count {
        cells.push(dec_cell_bin(reader)?);
    }
    Ok(XlsxSheet { name, cells })
}
}
pub use diff_wire_codec::*;
