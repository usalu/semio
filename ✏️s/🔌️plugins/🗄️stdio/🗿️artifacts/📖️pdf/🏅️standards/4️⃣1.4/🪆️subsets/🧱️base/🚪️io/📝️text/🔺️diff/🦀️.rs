//! 📝️ Text representation codec surface for `stdio.pdf` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PdfDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_4::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

/// 🔤️ Hex, so a page's text can carry any byte (including the separators this grammar uses)
/// without an escape layer. Same primitive the sibling 1.7 diff codec uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(text: &str) -> String {
    text.bytes().map(|byte| format!("{byte:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(text: &str) -> Result<String, String> {
    if !text.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {text:?}"));
    }
    let bytes: Result<Vec<u8>, String> = (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).map_err(|error| error.to_string())).collect();
    String::from_utf8(bytes?).map_err(|error| error.to_string())
}

/// 🧭️ Bracket-depth-aware split: a top-level `separator` inside nested brackets is never mistaken
/// for a field separator.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(text: &str, separator: char) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (index, character) in text.char_indices() {
        match character {
            '[' => depth += 1,
            ']' => depth -= 1,
            other if other == separator && depth == 0 => {
                out.push(&text[start..index]);
                start = index + other.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&text[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(text: &str) -> Result<&str, String> {
    text.strip_prefix('[').and_then(|inner| inner.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {text:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(text: &str) -> Result<usize, String> {
    text.parse().map_err(|error: std::num::ParseIntError| error.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(text: &str) -> Result<f64, String> {
    semio_framework_dsl::parse_f64(text)
}

/// 📄️ A whole `PageDoc` literal: `[width,height,hex-text]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page(page: &PageDoc) -> String {
    format!("[{},{},{}]", semio_framework_dsl::format_f64(page.width), semio_framework_dsl::format_f64(page.height), enc_str(&page.text))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page(text: &str) -> Result<PageDoc, String> {
    let parts = split_top_level(strip_brackets(text)?, ',');
    let [width, height, body] = parts.as_slice() else { return Err(format!("page: expected 3 fields, got {}", parts.len())) };
    Ok(PageDoc { width: parse_f64(width)?, height: parse_f64(height)?, text: dec_str(body)? })
}

/// 🏷️ `PdfPageDiff`'s sparse fields as single-letter `tag:value` pairs: `W`=width, `H`=height,
/// `X`=text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page_diff(diff: &PdfPageDiff) -> String {
    let mut parts = Vec::new();
    if let Some(value) = diff.width {
        parts.push(format!("W:{}",semio_framework_dsl::format_f64(value)));
    }
    if let Some(value) = diff.height {
        parts.push(format!("H:{}",semio_framework_dsl::format_f64(value)));
    }
    if let Some(value) = &diff.text {
        parts.push(format!("X:{}", enc_str(value)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page_diff(text: &str) -> Result<PdfPageDiff, String> {
    let mut diff = PdfPageDiff::default();
    for entry in split_top_level(strip_brackets(text)?, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, value) = entry.split_once(':').ok_or_else(|| format!("page diff: bad entry {entry:?}"))?;
        match tag {
            "W" => diff.width = Some(parse_f64(value)?),
            "H" => diff.height = Some(parse_f64(value)?),
            "X" => diff.text = Some(dec_str(value)?),
            other => return Err(format!("page diff: unknown tag {other:?}")),
        }
    }
    Ok(diff)
}

/// 📦️ The triple: `[removed];[modified];[added]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pages_diff(diff: &PdfPagesDiff) -> String {
    let removed = diff.removed.iter().map(|index| index.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|item| format!("{}:{}", item.index, enc_page_diff(&item.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|item| format!("{}:{}", item.index, enc_page(&item.page))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pages_diff(body: &str) -> Result<PdfPagesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_section, modified_section, added_section] = three.as_slice() else { return Err(format!("pages diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_section)?, ',').into_iter().filter(|entry| !entry.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_section)?, ',')
        .into_iter()
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("pages modified: bad entry {entry:?}"))?;
            Ok(PdfPageModified { index: parse_usize(index)?, diff: dec_page_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_section)?, ',')
        .into_iter()
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("pages added: bad entry {entry:?}"))?;
            Ok(PdfPageAdded { index: parse_usize(index)?, page: dec_page(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PdfPagesDiff { removed, modified, added })
}

impl protocol::DiffText for PdfDiff {
/// **Grammar**: `pages=<triple>` when the page lane moved, the empty string when nothing did —
/// one space-separated `name=value` token per changed top-level field, exactly the convention
/// the sibling 1.7 diff prints in.
fn print_diff(&self) -> String {
    match &self.pages {
        Some(pages) => format!("pages={}", enc_pages_diff(pages)),
        None => String::new(),
    }
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    let parse = |line: &str| -> Result<Self, String> {
        let mut diff = PdfDiff::default();
        if line.is_empty() {
            return Ok(diff);
        }
        for token in line.split(' ') {
            match token.strip_prefix("pages=") {
                Some(rest) => diff.pages = Some(dec_pages_diff(rest)?),
                None => return Err(format!("pdf 1.4 diff: unknown token {token:?}")),
            }
        }
        Ok(diff)
    };
    parse(line).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
