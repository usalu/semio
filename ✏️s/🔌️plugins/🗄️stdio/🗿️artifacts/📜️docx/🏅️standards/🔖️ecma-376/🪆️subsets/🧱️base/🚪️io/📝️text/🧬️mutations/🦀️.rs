//! 📝️ Text representation codec surface for `stdio.docx` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v_ecma_376::subsets::base::io::text::diff::{dec_block, dec_bool, dec_str, dec_style, dec_xml_node, decode_option, enc_block, enc_bool, enc_list, enc_str, enc_style, enc_xml_node, encode_option, hex_decode, hex_encode, split_top_level, strip_brackets};
use crate::standards::v_ecma_376::subsets::base::io::binary::diff::{dec_xml_node_bin, enc_xml_node_bin};
use crate::standards::v_ecma_376::subsets::base::io::text::diff::{parse_usize};
use crate::schema::diff::{DocxBlockPath, DocxDiff, DocxPathSegment, DocxXmlPartDiff, NamedModified, NamedTripleDiff};
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

/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `DocxMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `DocxDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_block`/`enc_style`/`enc_opc_part`/`split_top_level`/...) rather than
/// duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), same shape the derive's own handcrafted-wrapper convention uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_path_segment(seg: &DocxPathSegment) -> String {
    format!("[{},{},{}]", seg.block_index, seg.row, seg.cell)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_path_segment(s: &str) -> Result<DocxPathSegment, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [block_index, row, cell] = parts.as_slice() else { return Err(format!("path segment: expected 3 fields, got {}", parts.len())) };
    Ok(DocxPathSegment { block_index: parse_usize(block_index)?, row: parse_usize(row)?, cell: parse_usize(cell)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_path(p: &DocxBlockPath) -> String {
    format!("[{},{}]", enc_list(&p.segments, enc_path_segment), p.index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_path(s: &str) -> Result<DocxBlockPath, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [segments, index] = parts.as_slice() else { return Err(format!("block path: expected 2 fields, got {}", parts.len())) };
    Ok(DocxBlockPath { segments: dec_list_segments(segments)?, index: parse_usize(index)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list_segments(s: &str) -> Result<Vec<DocxPathSegment>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_segment).collect()
}

pub(crate) fn enc_xml_address(address: &DocxXmlAddress) -> String {
    hex_encode(semio_framework_pack_json::to_json_string(address).as_bytes())
}

pub(crate) fn dec_xml_address(value: &str) -> Result<DocxXmlAddress, String> {
    let bytes = hex_decode(value)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub(crate) fn dec_string_list(value: &str) -> Result<Vec<String>, String> {
    split_top_level(strip_brackets(value)?, ',').into_iter().filter(|value| !value.is_empty()).map(dec_str).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_docx_mutation(m: &DocxMutation) -> String {
    match m {
        DocxMutation::InsertBlock(insert_block::InsertBlock { path, block }) => format!("insert-block path={} block={}", enc_block_path(path), enc_block(block)),
        DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }) => format!("remove-block path={}", enc_block_path(path)),
        DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) => format!("set-block-content path={} block={}", enc_block_path(path), enc_block(block)),
        DocxMutation::SetRunText(set_run_text::SetRunText { address, text }) => format!("set-run-text address={} text={}", enc_xml_address(address), enc_str(text)),
        DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }) => format!("replace-xml-node address={} node={}", enc_xml_address(address), enc_xml_node(node)),
        DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold, italic, underline }) => {
            format!("set-run-formatting address={} bold={} italic={} underline={}", enc_xml_address(address), enc_bool(bold), enc_bool(italic), enc_bool(underline))
        }
        DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id }) => {
            format!("set-paragraph-style address={} style-id={}", enc_xml_address(address), encode_option(style_id, |value| enc_str(value)))
        }
        DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index, cells }) => {
            format!("insert-table-row address={} index={} cells={}", enc_xml_address(address), index, enc_list(cells, |value| enc_str(value)))
        }
        DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address, index }) => format!("remove-table-row address={} index={}", enc_xml_address(address), index),
        DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent, index, node }) => {
            format!("insert-xml-node parent={} index={} node={}", enc_xml_address(parent), index, enc_xml_node(node))
        }
        DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent, index, expected_name, revision }) => {
            format!("remove-xml-node parent={} index={} expected-name={} revision={}", enc_xml_address(parent), index, enc_str(expected_name), enc_str(revision))
        }
        DocxMutation::InsertStyle(insert_style::InsertStyle { style }) => format!("insert-style style={}", enc_style(style)),
        DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }) => format!("remove-style id={}", enc_str(id)),
        DocxMutation::SetStyleName(set_style_name::SetStyleName { id, name }) => format!("set-style-name id={} name={}", enc_str(id), enc_str(name)),
        DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }) => format!("set-style-based-on id={} based-on={}", enc_str(id), encode_option(based_on, |v| enc_str(v))),
        DocxMutation::SetPart(set_part::SetPart { path, content_type, payload, index }) => {
            format!("set-part path={} content-type={} payload={}{}", enc_str(path), enc_str(content_type), hex_encode(semio_framework_pack_json::to_json_string(payload).as_bytes()), index.map_or(String::new(), |index| format!(" index={index}")))
        }
        DocxMutation::RemovePart(remove_part::RemovePart { path }) => format!("remove-part path={}", enc_str(path)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_docx_mutation(line: &str) -> Result<DocxMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("docx mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("docx mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "insert-block" => Ok(DocxMutation::InsertBlock(insert_block::InsertBlock { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? })),
        "remove-block" => Ok(DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: dec_block_path(arg("path")?)? })),
        "set-block-content" => Ok(DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? })),
        "set-run-text" => Ok(DocxMutation::SetRunText(set_run_text::SetRunText { address: dec_xml_address(arg("address")?)?, text: dec_str(arg("text")?)? })),
        "replace-xml-node" => Ok(DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: dec_xml_address(arg("address")?)?, node: dec_xml_node(arg("node")?)? })),
        "set-run-formatting" => {
            Ok(DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address: dec_xml_address(arg("address")?)?, bold: dec_bool(arg("bold")?)?, italic: dec_bool(arg("italic")?)?, underline: dec_bool(arg("underline")?)? }))
        }
        "set-paragraph-style" => Ok(DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address: dec_xml_address(arg("address")?)?, style_id: decode_option(arg("style-id")?, dec_str)? })),
        "insert-table-row" => Ok(DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address: dec_xml_address(arg("address")?)?, index: usize_arg("index")?, cells: dec_string_list(arg("cells")?)? })),
        "remove-table-row" => Ok(DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address: dec_xml_address(arg("address")?)?, index: usize_arg("index")? })),
        "insert-xml-node" => Ok(DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: dec_xml_address(arg("parent")?)?, index: usize_arg("index")?, node: dec_xml_node(arg("node")?)? })),
        "remove-xml-node" => {
            Ok(DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent: dec_xml_address(arg("parent")?)?, index: usize_arg("index")?, expected_name: dec_str(arg("expected-name")?)?, revision: dec_str(arg("revision")?)? }))
        }
        "insert-style" => Ok(DocxMutation::InsertStyle(insert_style::InsertStyle { style: dec_style(arg("style")?)? })),
        "remove-style" => Ok(DocxMutation::RemoveStyle(remove_style::RemoveStyle { id: dec_str(arg("id")?)? })),
        "set-style-name" => Ok(DocxMutation::SetStyleName(set_style_name::SetStyleName { id: dec_str(arg("id")?)?, name: dec_str(arg("name")?)? })),
        "set-style-based-on" => Ok(DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: dec_str(arg("id")?)?, based_on: decode_option(arg("based-on")?, dec_str)? })),
        "set-part" => Ok(DocxMutation::SetPart(set_part::SetPart { path: dec_str(arg("path")?)?, content_type: dec_str(arg("content-type")?)?, payload: { let bytes = hex_decode(arg("payload")?)?; let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?; semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())? }, index: args.get("index").map(|value| value.parse::<usize>().map_err(|error| error.to_string())).transpose()? })),
        "remove-part" => Ok(DocxMutation::RemovePart(remove_part::RemovePart { path: dec_str(arg("path")?)? })),
        other => Err(format!("docx mutation: unknown keyword {other:?}")),
    }
}

impl OpText for DocxMutation {
    fn print_op(&self) -> String {
        print_docx_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_docx_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
