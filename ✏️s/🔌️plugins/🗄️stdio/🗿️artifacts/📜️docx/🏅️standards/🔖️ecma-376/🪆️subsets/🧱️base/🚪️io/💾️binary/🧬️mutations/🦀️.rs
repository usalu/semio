//! docx rep for stdio.docx 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v_ecma_376::subsets::base::io::text::diff::{dec_block, dec_bool, dec_str, dec_style, dec_xml_node, decode_option, enc_block, enc_bool, enc_list, enc_str, enc_style, enc_xml_node, encode_option, hex_decode, hex_encode, split_top_level, strip_brackets};
use crate::standards::v_ecma_376::subsets::base::io::binary::diff::{dec_xml_node_bin, enc_xml_node_bin};
use crate::standards::v_ecma_376::subsets::base::io::text::diff::{parse_usize};
use crate::schema::diff::{diff_set_snapshot, DocxBlockPath, DocxDiff, DocxPathSegment, DocxXmlPartDiff, NamedModified, NamedTripleDiff};
#[cfg(test)]
use crate::schema::snapshot::DocxDocument;
use crate::schema::snapshot::{docx_part_is_xml, DocxBlock, DocxStyle, DocxXmlPart};
#[cfg(test)]
use crate::schema::snapshot::{DocxParagraph, DocxRun, DocxTable, DocxTableCell, DocxTableRow};
use crate::DocxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_xml::schema::diff::{diff_at_path as xml_diff_at_path, XmlChildAdded, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::{OpcTargetMode, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};
use xml_address::{
    docx_block_run_address, docx_top_level_block_address, docx_top_level_block_count, docx_top_level_run_at, docx_top_level_run_count, docx_xml_address, docx_xml_subtree_revision, resolve_docx_xml_address, DocxEditableRun, DocxXmlAddress,
    ResolvedDocxXmlAddress, docx_top_level_text_targets, DocxEditableText, DocxTextTargetKind, docx_run_formatting, DocxRunFormatting,
};
/// 🧪️ FG-wave: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors `📰️xml/…/🧬️mutations/🦀️.rs`'s own `enc_node_path_bin`/`enc_xml_snapshot_bin`
/// shape, reusing `store::pack_rt::write_varint_u64`/`store::ByteReader` plus `DocxDiff`'s own
/// `write_str_lp`/`read_str_lp`/`write_bytes_lp`/`read_bytes_lp`/`enc_block_bin`/`dec_block_bin`/
/// `enc_style_bin`/`dec_style_bin`/`enc_opc_part_bin`/`dec_opc_part_bin`/`enc_rel_bin`/
/// `dec_rel_bin` (`../🔺️diff/🦀️.rs`, `pub(crate)` to this artifact).
use crate::standards::v_ecma_376::subsets::base::io::binary::diff::{dec_block_bin, dec_style_bin, enc_block_bin, enc_style_bin, read_bytes_lp, read_str_lp, write_bytes_lp, write_str_lp};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_segment_bin(seg: &DocxPathSegment, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, seg.block_index as u64);
    store::pack_rt::write_varint_u64(out, seg.row as u64);
    store::pack_rt::write_varint_u64(out, seg.cell as u64);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_segment_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxPathSegment, String> {
    let block_index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let row = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let cell = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(DocxPathSegment { block_index, row, cell })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_path_bin(p: &DocxBlockPath, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, p.segments.len() as u64);
    for seg in &p.segments {
        enc_path_segment_bin(seg, out);
    }
    store::pack_rt::write_varint_u64(out, p.index as u64);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_path_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxBlockPath, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut segments = Vec::with_capacity(count as usize);
    for _ in 0..count {
        segments.push(dec_path_segment_bin(reader)?);
    }
    let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(DocxBlockPath { segments, index })
}

pub(crate) fn enc_xml_address_bin(address: &DocxXmlAddress, output: &mut Vec<u8>) {
    write_str_lp(output, &address.part_path);
    store::pack_rt::write_varint_u64(output, address.node_path.len() as u64);
    for index in &address.node_path {
        store::pack_rt::write_varint_u64(output, *index as u64);
    }
    write_str_lp(output, &address.expected_name);
    write_str_lp(output, &address.revision);
}

pub(crate) fn dec_xml_address_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxXmlAddress, String> {
    let part_path = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut node_path = Vec::with_capacity(count as usize);
    for _ in 0..count {
        node_path.push(reader.read_varint_u64().map_err(|error| error.to_string())? as usize);
    }
    let expected_name = read_str_lp(reader)?;
    let revision = read_str_lp(reader)?;
    Ok(DocxXmlAddress { part_path, node_path, expected_name, revision })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_docx_snapshot_bin(snapshot: &DocxSnapshot, out: &mut Vec<u8>) {
    write_bytes_lp(out, semio_framework_pack_json::to_json_string(snapshot).as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_docx_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxSnapshot, String> {
    let bytes = read_bytes_lp(reader)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `DocxMutation` variant ordinal, in the same 0-11 order `print_docx_mutation`'s own keyword
/// match uses.
impl OpBinary for DocxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => TAG_SET_SNAPSHOT,
            DocxMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
            DocxMutation::InsertBlock(insert_block::InsertBlock { .. }) => TAG_INSERT_BLOCK,
            DocxMutation::RemoveBlock(remove_block::RemoveBlock { .. }) => TAG_REMOVE_BLOCK,
            DocxMutation::SetBlockContent(set_block_content::SetBlockContent { .. }) => TAG_SET_BLOCK_CONTENT,
            DocxMutation::SetRunText(set_run_text::SetRunText { .. }) => TAG_SET_RUN_TEXT,
            DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { .. }) => TAG_REPLACE_XML_NODE,
            DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { .. }) => TAG_SET_RUN_FORMATTING,
            DocxMutation::SetParagraphStyle(_) => TAG_SET_PARAGRAPH_STYLE,
            DocxMutation::InsertTableRow(_) => TAG_INSERT_TABLE_ROW,
            DocxMutation::RemoveTableRow(_) => TAG_REMOVE_TABLE_ROW,
            DocxMutation::InsertXmlNode(_) => TAG_INSERT_XML_NODE,
            DocxMutation::RemoveXmlNode(_) => TAG_REMOVE_XML_NODE,
            DocxMutation::InsertStyle(insert_style::InsertStyle { .. }) => TAG_INSERT_STYLE,
            DocxMutation::RemoveStyle(remove_style::RemoveStyle { .. }) => TAG_REMOVE_STYLE,
            DocxMutation::SetStyleName(set_style_name::SetStyleName { .. }) => TAG_SET_STYLE_NAME,
            DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { .. }) => TAG_SET_STYLE_BASED_ON,
            DocxMutation::SetPart(set_part::SetPart { .. }) => TAG_SET_PART,
            DocxMutation::RemovePart(remove_part::RemovePart { .. }) => TAG_REMOVE_PART,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_docx_snapshot_bin(snapshot, &mut out),
            DocxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => out.extend(protocol::OpBinary::encode_op(patch)?),
            DocxMutation::InsertBlock(insert_block::InsertBlock { path, block }) => {
                enc_block_path_bin(path, &mut out);
                enc_block_bin(block, &mut out);
            }
            DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }) => enc_block_path_bin(path, &mut out),
            DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) => {
                enc_block_path_bin(path, &mut out);
                enc_block_bin(block, &mut out);
            }
            DocxMutation::SetRunText(set_run_text::SetRunText { address, text }) => {
                enc_xml_address_bin(address, &mut out);
                write_str_lp(&mut out, text);
            }
            DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }) => {
                enc_xml_address_bin(address, &mut out);
                enc_xml_node_bin(node, &mut out);
            }
            DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold, italic, underline }) => {
                enc_xml_address_bin(address, &mut out);
                out.push(*bold as u8);
                out.push(*italic as u8);
                out.push(*underline as u8);
            }
            DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id }) => {
                enc_xml_address_bin(address, &mut out);
                out.push(style_id.is_some() as u8);
                if let Some(style_id) = style_id {
                    write_str_lp(&mut out, style_id);
                }
            }
            DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index, cells }) => {
                enc_xml_address_bin(address, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                store::pack_rt::write_varint_u64(&mut out, cells.len() as u64);
                for cell in cells {
                    write_str_lp(&mut out, cell);
                }
            }
            DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address, index }) => {
                enc_xml_address_bin(address, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent, index, node }) => {
                enc_xml_address_bin(parent, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_xml_node_bin(node, &mut out);
            }
            DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent, index, expected_name, revision }) => {
                enc_xml_address_bin(parent, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                write_str_lp(&mut out, expected_name);
                write_str_lp(&mut out, revision);
            }
            DocxMutation::InsertStyle(insert_style::InsertStyle { style }) => enc_style_bin(style, &mut out),
            DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }) => write_str_lp(&mut out, id),
            DocxMutation::SetStyleName(set_style_name::SetStyleName { id, name }) => {
                write_str_lp(&mut out, id);
                write_str_lp(&mut out, name);
            }
            DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }) => {
                write_str_lp(&mut out, id);
                out.push(if based_on.is_some() { 1 } else { 0 });
                if let Some(based_on) = based_on {
                    write_str_lp(&mut out, based_on);
                }
            }
            DocxMutation::SetPart(set_part::SetPart { path, content_type, payload }) => {
                write_str_lp(&mut out, path);
                write_str_lp(&mut out, content_type);
                write_bytes_lp(&mut out, &store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(payload)));
            }
            DocxMutation::RemovePart(remove_part::RemovePart { path }) => write_str_lp(&mut out, path),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_PATCH_SNAPSHOT => Ok(DocxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: <semio_s_artifact_stdio_contract::editing::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed { what: "patch-snapshot payload", offset: reader.position() as u64, detail: e.to_string() })?)? })),
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_docx_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            TAG_INSERT_BLOCK => {
                let path = dec_block_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DocxMutation::InsertBlock(insert_block::InsertBlock { path, block }))
            }
            TAG_REMOVE_BLOCK => {
                let path = dec_block_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                Ok(DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }))
            }
            TAG_SET_BLOCK_CONTENT => {
                let path = dec_block_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }))
            }
            TAG_SET_RUN_TEXT => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let text = read_str_lp(&mut reader).map_err(|e| malformed("op text", reader.position(), e))?;
                Ok(DocxMutation::SetRunText(set_run_text::SetRunText { address, text }))
            }
            TAG_REPLACE_XML_NODE => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let node = dec_xml_node_bin(&mut reader).map_err(|e| malformed("op node", reader.position(), e))?;
                Ok(DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }))
            }
            TAG_SET_RUN_FORMATTING => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let bold = reader.read_u8().map_err(|e| malformed("op bold", reader.position(), e.to_string()))? != 0;
                let italic = reader.read_u8().map_err(|e| malformed("op italic", reader.position(), e.to_string()))? != 0;
                let underline = reader.read_u8().map_err(|e| malformed("op underline", reader.position(), e.to_string()))? != 0;
                Ok(DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold, italic, underline }))
            }
            TAG_SET_PARAGRAPH_STYLE => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let has_style = reader.read_u8().map_err(|e| malformed("op style_id presence", reader.position(), e.to_string()))?;
                let style_id = (has_style != 0).then(|| read_str_lp(&mut reader)).transpose().map_err(|e| malformed("op style_id", reader.position(), e))?;
                Ok(DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id }))
            }
            TAG_INSERT_TABLE_ROW => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let count = reader.read_varint_u64().map_err(|e| malformed("op cells count", reader.position(), e.to_string()))?;
                let mut cells = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    cells.push(read_str_lp(&mut reader).map_err(|e| malformed("op cell", reader.position(), e))?);
                }
                Ok(DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index, cells }))
            }
            TAG_REMOVE_TABLE_ROW => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address, index }))
            }
            TAG_INSERT_XML_NODE => {
                let parent = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op parent", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let node = dec_xml_node_bin(&mut reader).map_err(|e| malformed("op node", reader.position(), e))?;
                Ok(DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent, index, node }))
            }
            TAG_REMOVE_XML_NODE => {
                let parent = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op parent", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let expected_name = read_str_lp(&mut reader).map_err(|e| malformed("op expected_name", reader.position(), e))?;
                let revision = read_str_lp(&mut reader).map_err(|e| malformed("op revision", reader.position(), e))?;
                Ok(DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent, index, expected_name, revision }))
            }
            TAG_INSERT_STYLE => {
                let style = dec_style_bin(&mut reader).map_err(|e| malformed("op style", reader.position(), e))?;
                Ok(DocxMutation::InsertStyle(insert_style::InsertStyle { style }))
            }
            TAG_REMOVE_STYLE => {
                let id = read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                Ok(DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }))
            }
            TAG_SET_STYLE_NAME => {
                let id = read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(DocxMutation::SetStyleName(set_style_name::SetStyleName { id, name }))
            }
            TAG_SET_STYLE_BASED_ON => {
                let id = read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let has = reader.read_u8().map_err(|e| malformed("op based_on presence", reader.position(), e.to_string()))?;
                let based_on = if has != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("op based_on", reader.position(), e))?) } else { None };
                Ok(DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }))
            }
            TAG_SET_PART => {
                let path = read_str_lp(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let content_type = read_str_lp(&mut reader).map_err(|e| malformed("op content_type", reader.position(), e))?;
                let bytes = read_bytes_lp(&mut reader).map_err(|e| malformed("op payload", reader.position(), e))?;
                let value = store::pack_rt::decode_wire_value(&bytes).map_err(|e| malformed("op payload", reader.position(), e.to_string()))?;
                let payload = semio_framework_value::FromValue::from_value(value).map_err(|e| malformed("op payload", reader.position(), e.to_string()))?;
                Ok(DocxMutation::SetPart(set_part::SetPart { path, content_type, payload }))
            }
            TAG_REMOVE_PART => {
                let path = read_str_lp(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                Ok(DocxMutation::RemovePart(remove_part::RemovePart { path }))
            }
            other => Err(malformed("op tag", 1, format!("unknown DocxMutation tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `DocxMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_INSERT_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-block");
const TAG_REMOVE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-block");
const TAG_SET_BLOCK_CONTENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-block-content");
const TAG_SET_RUN_TEXT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-run-text");
const TAG_SET_RUN_FORMATTING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-run-formatting");
const TAG_INSERT_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-style");
const TAG_REMOVE_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-style");
const TAG_SET_STYLE_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-style-name");
const TAG_SET_STYLE_BASED_ON: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-style-based-on");
const TAG_SET_PART: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-part");
const TAG_REMOVE_PART: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-part");
const TAG_REPLACE_XML_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-xml-node");
const TAG_SET_PARAGRAPH_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-paragraph-style");
const TAG_INSERT_TABLE_ROW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-table-row");
const TAG_REMOVE_TABLE_ROW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-table-row");
const TAG_INSERT_XML_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-xml-node");
const TAG_REMOVE_XML_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-xml-node");
//#endregion 🏷️WireTags
