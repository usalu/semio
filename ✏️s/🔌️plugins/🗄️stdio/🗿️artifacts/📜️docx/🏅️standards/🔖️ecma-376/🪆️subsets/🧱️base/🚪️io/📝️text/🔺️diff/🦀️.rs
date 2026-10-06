//! 📝️ Text representation codec surface for `stdio.docx` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DocxDiffText = String;
//#endregion 🚚️Carrier

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

impl protocol::DiffText for DocxDiff {
fn print_diff(&self) -> String {
    semio_framework_pack_json::to_json_string(self)
}
fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
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
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: &bool) -> String {
    if *b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: bad value {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 🌳️ Recursive: `E[name,[attrs],[children]]` / `T[text]` / `D[text]` (CData) / `M[text]`
/// (comment) / `P[target,data]` (processing instruction) -- same tag scheme `📰️xml`/`🎨️svg`'s own
/// hand-rolled codecs use (own copy per the no-shared-helpers-module convention). Needed here
/// because every `extra_*_properties: Vec<XmlNode>` raw-retention field (on `DocxRun`/
/// `DocxParagraph`/`DocxTableCell`/`DocxTableRow`/`DocxTable`) carries this type verbatim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr(a: &XmlAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), enc_str(&a.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr(s: &str) -> Result<XmlAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    Ok(XmlAttr { name: dec_str(name)?, value: dec_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_node(n: &XmlNode) -> String {
    match n {
        XmlNode::Element { name, attrs, children } => {
            let attrs = attrs.iter().map(enc_attr).collect::<Vec<_>>().join(",");
            let children = children.iter().map(enc_xml_node).collect::<Vec<_>>().join(",");
            format!("E[{},[{}],[{}]]", enc_str(name), attrs, children)
        }
        XmlNode::Text { text } => format!("T[{}]", enc_str(text)),
        XmlNode::CData { text } => format!("D[{}]", enc_str(text)),
        XmlNode::Comment { text } => format!("M[{}]", enc_str(text)),
        XmlNode::ProcessingInstruction { target, data } => format!("P[{},{}]", enc_str(target), enc_str(data)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_node(s: &str) -> Result<XmlNode, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "E" => {
            let parts = split_top_level(inner, ',');
            let [name, attrs, children] = parts.as_slice() else { return Err(format!("element: expected 3 fields, got {}", parts.len())) };
            let attrs = split_top_level(strip_brackets(attrs)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_attr).collect::<Result<Vec<_>, String>>()?;
            let children = split_top_level(strip_brackets(children)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_xml_node).collect::<Result<Vec<_>, String>>()?;
            Ok(XmlNode::Element { name: dec_str(name)?, attrs, children })
        }
        "T" => Ok(XmlNode::Text { text: dec_str(inner)? }),
        "D" => Ok(XmlNode::CData { text: dec_str(inner)? }),
        "M" => Ok(XmlNode::Comment { text: dec_str(inner)? }),
        "P" => {
            let parts = split_top_level(inner, ',');
            let [target, data] = parts.as_slice() else { return Err(format!("PI: expected 2 fields, got {}", parts.len())) };
            Ok(XmlNode::ProcessingInstruction { target: dec_str(target)?, data: dec_str(data)? })
        }
        other => Err(format!("xml node: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_run(r: &DocxRun) -> String {
    format!("[{},{},{},{},{}]", enc_str(&r.text), enc_bool(&r.bold), enc_bool(&r.italic), enc_bool(&r.underline), enc_list(&r.extra_run_properties, enc_xml_node))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_run(s: &str) -> Result<DocxRun, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [text, bold, italic, underline, extra] = parts.as_slice() else { return Err(format!("run: expected 5 fields, got {}", parts.len())) };
    Ok(DocxRun { text: dec_str(text)?, bold: dec_bool(bold)?, italic: dec_bool(italic)?, underline: dec_bool(underline)?, extra_run_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_paragraph(p: &DocxParagraph) -> String {
    format!("[{},{},{}]", enc_list(&p.runs, enc_run), encode_option(&p.style, |v| enc_str(v)), enc_list(&p.extra_paragraph_properties, enc_xml_node))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_paragraph(s: &str) -> Result<DocxParagraph, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [runs, style, extra] = parts.as_slice() else { return Err(format!("paragraph: expected 3 fields, got {}", parts.len())) };
    Ok(DocxParagraph { runs: dec_list(runs, dec_run)?, style: decode_option(style, dec_str)?, extra_paragraph_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cell(c: &DocxTableCell) -> String {
    format!("[{},{}]", enc_list(&c.blocks, enc_block), enc_list(&c.extra_cell_properties, enc_xml_node))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cell(s: &str) -> Result<DocxTableCell, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [blocks, extra] = parts.as_slice() else { return Err(format!("cell: expected 2 fields, got {}", parts.len())) };
    Ok(DocxTableCell { blocks: dec_list(blocks, dec_block)?, extra_cell_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row(r: &DocxTableRow) -> String {
    format!("[{},{}]", enc_list(&r.cells, enc_cell), enc_list(&r.extra_row_properties, enc_xml_node))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row(s: &str) -> Result<DocxTableRow, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [cells, extra] = parts.as_slice() else { return Err(format!("row: expected 2 fields, got {}", parts.len())) };
    Ok(DocxTableRow { cells: dec_list(cells, dec_cell)?, extra_row_properties: dec_list(extra, dec_xml_node)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table(t: &DocxTable) -> String {
    format!("[{},{}]", enc_list(&t.rows, enc_row), enc_list(&t.extra_table_properties, enc_xml_node))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table(s: &str) -> Result<DocxTable, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [rows, extra] = parts.as_slice() else { return Err(format!("table: expected 2 fields, got {}", parts.len())) };
    Ok(DocxTable { rows: dec_list(rows, dec_row)?, extra_table_properties: dec_list(extra, dec_xml_node)? })
}

/// 🌳️ `P[paragraph]` / `T[table]` -- `DocxBlock`'s two variants, tag-prefixed like `enc_xml_node`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &DocxBlock) -> String {
    match b {
        DocxBlock::Paragraph(p) => format!("P{}", enc_paragraph(p)),
        DocxBlock::Table(t) => format!("T{}", enc_table(t)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<DocxBlock, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "P" => Ok(DocxBlock::Paragraph(dec_paragraph(rest)?)),
        "T" => Ok(DocxBlock::Table(dec_table(rest)?)),
        other => Err(format!("block: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_style(s: &DocxStyle) -> String {
    format!("[{},{},{}]", enc_str(&s.id), enc_str(&s.name), encode_option(&s.based_on, |v| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_style(s: &str) -> Result<DocxStyle, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, name, based_on] = parts.as_slice() else { return Err(format!("style: expected 3 fields, got {}", parts.len())) };
    Ok(DocxStyle { id: dec_str(id)?, name: dec_str(name)?, based_on: decode_option(based_on, dec_str)? })
}
}
pub use diff_wire_codec::*;

#[allow(unused_imports)]
mod residual_diff_helper {
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
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
}
pub(crate) use residual_diff_helper::*;
