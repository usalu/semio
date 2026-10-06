//! 📝️ Text representation codec surface for `stdio.xlsx` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type XlsxDiffText = String;
//#endregion 🚚️Carrier

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

impl protocol::DiffText for XlsxDiff {
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
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

/// 🔢️ `f64::to_string`/`str::parse::<f64>()` round-trip exactly (std's shortest-round-trip float
/// formatting) — no manual bit-pattern encoding needed. None of `.`/`-`/`e`/`inf`/`NaN` clash with
/// this grammar's `,`/`;`/`:`/`[`/`]` separators.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_f64(n: f64) -> String {
    n.to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
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

/// 🔢️ `N[f64]`/`S[usize]`/`I[hex]`/`B[0|1]`/`F[expr_hex,cached_option]`/`E[]` — single-uppercase-
/// letter tag prefix immediately followed by the bracketed positional payload, same convention
/// `f6-recon-report.md` §5 and `SvgDiff`'s `enc_xml_node` use for data-carrying enums.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cell_value(v: &XlsxCellValue) -> String {
    match v {
        XlsxCellValue::Number(n) => format!("N[{}]", enc_f64(*n)),
        XlsxCellValue::SharedString(i) => format!("S[{i}]"),
        XlsxCellValue::InlineString(s) => format!("I[{}]", enc_str(s)),
        XlsxCellValue::Boolean(b) => format!("B[{}]", if *b { "1" } else { "0" }),
        XlsxCellValue::Error(error) => format!("R[{}]", enc_str(error)),
        XlsxCellValue::Formula { expr, cached } => format!("F[{},{}]", enc_str(expr), encode_option(cached, |c| enc_cell_value(c))),
        XlsxCellValue::Empty => "E[]".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cell_value(s: &str) -> Result<XlsxCellValue, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "N" => Ok(XlsxCellValue::Number(dec_f64(inner)?)),
        "S" => Ok(XlsxCellValue::SharedString(parse_usize(inner)?)),
        "I" => Ok(XlsxCellValue::InlineString(dec_str(inner)?)),
        "B" => Ok(XlsxCellValue::Boolean(inner == "1")),
        "R" => Ok(XlsxCellValue::Error(dec_str(inner)?)),
        "F" => {
            let parts = split_top_level(inner, ',');
            let [expr, cached] = parts.as_slice() else { return Err(format!("formula: expected 2 fields, got {}", parts.len())) };
            Ok(XlsxCellValue::Formula { expr: dec_str(expr)?, cached: decode_option(cached, |c| dec_cell_value(c).map(Box::new))? })
        }
        "E" => Ok(XlsxCellValue::Empty),
        other => Err(format!("cell value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_cell(c: &XlsxCell) -> String {
    format!("[{},{},{}]", c.row, c.col, enc_cell_value(&c.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_cell(s: &str) -> Result<XlsxCell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [row, col, value] = parts.as_slice() else { return Err(format!("cell: expected 3 fields, got {}", parts.len())) };
    Ok(XlsxCell { row: parse_u32(row)?, col: parse_u32(col)?, value: dec_cell_value(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_sheet(s: &XlsxSheet) -> String {
    let cells = s.cells.iter().map(enc_cell).collect::<Vec<_>>().join(",");
    format!("[{},[{}]]", enc_str(&s.name), cells)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_sheet(s: &str) -> Result<XlsxSheet, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, cells] = parts.as_slice() else { return Err(format!("sheet: expected 2 fields, got {}", parts.len())) };
    let cells = split_top_level(strip_brackets(cells)?, ',').into_iter().map(dec_cell).collect::<Result<Vec<_>, String>>()?;
    Ok(XlsxSheet { name: dec_str(name)?, cells })
}
}
pub use diff_wire_codec::*;
