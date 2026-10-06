//! docx rep for stdio.docx 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
#[cfg(test)]
use crate::schema::snapshot::DocxDocument;
use crate::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow, DocxXmlPart, DocxXmlParts};
use crate::DocxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::deserialize_double_option;
use semio_s_artifact_stdio_xml::schema::diff::{XmlChildrenDiff, XmlDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;
use std::collections::BTreeMap;

impl protocol::DiffBinary for DocxDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut bytes = vec![store::pack_rt::OP_BINARY_FORMAT];
    bytes.extend_from_slice(semio_framework_pack_json::to_json_string(self).as_bytes());
    Ok(bytes)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let payload = bytes.get(1..).ok_or_else(|| protocol::ProtocolError::Malformed { what: "docx diff", offset: 0, detail: "missing format byte".into() })?;
    let text = std::str::from_utf8(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "docx diff", offset: 1, detail: error.to_string() })?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "docx diff", offset: 1, detail: error.to_string() })
}
}

}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::diff::*;
#[cfg(test)]
use crate::schema::snapshot::DocxDocument;
use crate::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow, DocxXmlPart, DocxXmlParts};
use crate::DocxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::deserialize_double_option;
use semio_s_artifact_stdio_xml::schema::diff::{XmlChildrenDiff, XmlDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;
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

/// 🌳️ Binary twin of `enc_xml_node`/`dec_xml_node` -- 1-byte kind tag (`0`=Element/`1`=Text/
/// `2`=CData/`3`=Comment/`4`=ProcessingInstruction, matching xml's own binary tag numbering).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_bin(a: &XmlAttr, out: &mut Vec<u8>) {
    write_str_lp(out, &a.name);
    write_str_lp(out, &a.value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttr, String> {
    let name = read_str_lp(reader)?;
    let value = read_str_lp(reader)?;
    Ok(XmlAttr { name, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_node_bin(node: &XmlNode, out: &mut Vec<u8>) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            out.push(0);
            write_str_lp(out, name);
            store::pack_rt::write_varint_u64(out, attrs.len() as u64);
            for attr in attrs {
                enc_attr_bin(attr, out);
            }
            store::pack_rt::write_varint_u64(out, children.len() as u64);
            for child in children {
                enc_xml_node_bin(child, out);
            }
        }
        XmlNode::Text { text } => {
            out.push(1);
            write_str_lp(out, text);
        }
        XmlNode::CData { text } => {
            out.push(2);
            write_str_lp(out, text);
        }
        XmlNode::Comment { text } => {
            out.push(3);
            write_str_lp(out, text);
        }
        XmlNode::ProcessingInstruction { target, data } => {
            out.push(4);
            write_str_lp(out, target);
            write_str_lp(out, data);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_node_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = read_str_lp(reader)?;
            let attr_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut attrs = Vec::with_capacity(attr_count as usize);
            for _ in 0..attr_count {
                attrs.push(dec_attr_bin(reader)?);
            }
            let child_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut children = Vec::with_capacity(child_count as usize);
            for _ in 0..child_count {
                children.push(dec_xml_node_bin(reader)?);
            }
            Ok(XmlNode::Element { name, attrs, children })
        }
        1 => Ok(XmlNode::Text { text: read_str_lp(reader)? }),
        2 => Ok(XmlNode::CData { text: read_str_lp(reader)? }),
        3 => Ok(XmlNode::Comment { text: read_str_lp(reader)? }),
        4 => {
            let target = read_str_lp(reader)?;
            let data = read_str_lp(reader)?;
            Ok(XmlNode::ProcessingInstruction { target, data })
        }
        other => Err(format!("xml node binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_node_list_bin(nodes: &[XmlNode], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, nodes.len() as u64);
    for n in nodes {
        enc_xml_node_bin(n, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_node_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<XmlNode>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_xml_node_bin(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run_bin(r: &DocxRun, out: &mut Vec<u8>) {
    write_str_lp(out, &r.text);
    out.push(r.bold as u8);
    out.push(r.italic as u8);
    out.push(r.underline as u8);
    enc_xml_node_list_bin(&r.extra_run_properties, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxRun, String> {
    let text = read_str_lp(reader)?;
    let bold = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let italic = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let underline = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let extra_run_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxRun { text, bold, italic, underline, extra_run_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_paragraph_bin(p: &DocxParagraph, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, p.runs.len() as u64);
    for r in &p.runs {
        enc_run_bin(r, out);
    }
    out.push(if p.style.is_some() { 1 } else { 0 });
    if let Some(style) = &p.style {
        write_str_lp(out, style);
    }
    enc_xml_node_list_bin(&p.extra_paragraph_properties, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_paragraph_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxParagraph, String> {
    let run_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut runs = Vec::with_capacity(run_count as usize);
    for _ in 0..run_count {
        runs.push(dec_run_bin(reader)?);
    }
    let style = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let extra_paragraph_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxParagraph { runs, style, extra_paragraph_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cell_bin(c: &DocxTableCell, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, c.blocks.len() as u64);
    for b in &c.blocks {
        enc_block_bin(b, out);
    }
    enc_xml_node_list_bin(&c.extra_cell_properties, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cell_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableCell, String> {
    let block_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut blocks = Vec::with_capacity(block_count as usize);
    for _ in 0..block_count {
        blocks.push(dec_block_bin(reader)?);
    }
    let extra_cell_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxTableCell { blocks, extra_cell_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row_bin(r: &DocxTableRow, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, r.cells.len() as u64);
    for c in &r.cells {
        enc_cell_bin(c, out);
    }
    enc_xml_node_list_bin(&r.extra_row_properties, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTableRow, String> {
    let cell_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(cell_count as usize);
    for _ in 0..cell_count {
        cells.push(dec_cell_bin(reader)?);
    }
    let extra_row_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxTableRow { cells, extra_row_properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_bin(t: &DocxTable, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, t.rows.len() as u64);
    for r in &t.rows {
        enc_row_bin(r, out);
    }
    enc_xml_node_list_bin(&t.extra_table_properties, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxTable, String> {
    let row_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut rows = Vec::with_capacity(row_count as usize);
    for _ in 0..row_count {
        rows.push(dec_row_bin(reader)?);
    }
    let extra_table_properties = dec_xml_node_list_bin(reader)?;
    Ok(DocxTable { rows, extra_table_properties })
}

/// 🌳️ `0`=Paragraph / `1`=Table -- `DocxBlock`'s two variants, tag-prefixed like `enc_xml_node_bin`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_bin(b: &DocxBlock, out: &mut Vec<u8>) {
    match b {
        DocxBlock::Paragraph(p) => {
            out.push(0);
            enc_paragraph_bin(p, out);
        }
        DocxBlock::Table(t) => {
            out.push(1);
            enc_table_bin(t, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxBlock, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(DocxBlock::Paragraph(dec_paragraph_bin(reader)?)),
        1 => Ok(DocxBlock::Table(dec_table_bin(reader)?)),
        other => Err(format!("block binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style_bin(s: &DocxStyle, out: &mut Vec<u8>) {
    write_str_lp(out, &s.id);
    write_str_lp(out, &s.name);
    out.push(if s.based_on.is_some() { 1 } else { 0 });
    if let Some(based_on) = &s.based_on {
        write_str_lp(out, based_on);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxStyle, String> {
    let id = read_str_lp(reader)?;
    let name = read_str_lp(reader)?;
    let based_on = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    Ok(DocxStyle { id, name, based_on })
}
}
pub use diff_wire_codec::*;
