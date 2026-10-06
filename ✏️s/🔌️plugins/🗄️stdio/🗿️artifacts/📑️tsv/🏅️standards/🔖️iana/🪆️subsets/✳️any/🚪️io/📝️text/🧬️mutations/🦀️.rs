//! 📝️ Text representation codec surface for `stdio.tsv` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::iana::subsets::any::schema::mutations::*;
use crate::standards::iana::subsets::any::schema::diff::{diff_set_snapshot, TsvDiff, TsvRowAdded, TsvRowDiff, TsvRowModified, TsvRowsDiff};
use crate::standards::iana::subsets::any::io::text::diff::{dec_str};
use crate::standards::iana::subsets::any::io::text::diff::{enc_str};
use crate::standards::iana::subsets::any::io::text::diff::{dec_row};
use crate::standards::iana::subsets::any::io::text::diff::{enc_row};
use crate::standards::iana::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::iana::subsets::any::io::text::diff::{split_top_level};
use crate::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `TsvMutation` — reuses `TsvDiff`'s `pub(crate)`
/// grammar primitives. Grammar: `keyword arg=value ...` (space-separated), same convention csv's/
/// gif89a's/svg's own hand-rolled `OpText` impls use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_tsv_snapshot(s: &TsvSnapshot) -> String {
    format!("[{},{},{},[{}]]", enc_str(&s.schema), if s.trailing_newline { 1 } else { 0 }, crate::standards::iana::subsets::any::io::text::diff::enc_line_ending(s.line_ending), s.records.iter().map(|r| enc_row(r)).collect::<Vec<_>>().join(","),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_tsv_snapshot(s: &str) -> Result<TsvSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, trailing_newline, line_ending, records] = parts.as_slice() else {
        return Err(format!("tsv snapshot: expected 4 fields, got {}", parts.len()));
    };
    let records = split_top_level(strip_brackets(records)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_row).collect::<Result<Vec<_>, String>>()?;
    Ok(TsvSnapshot { schema: dec_str(schema)?, trailing_newline: *trailing_newline == "1", line_ending: crate::standards::iana::subsets::any::io::text::diff::dec_line_ending(line_ending)?, records })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_tsv_mutation(m: &TsvMutation) -> String {
    match m {
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_tsv_snapshot(snapshot)),
        TsvMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline }) => format!("set-trailing-newline trailing-newline={}", if *trailing_newline { 1 } else { 0 }),
        TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending }) => format!("set-line-ending line-ending={}", crate::standards::iana::subsets::any::io::text::diff::enc_line_ending(*line_ending)),
        TsvMutation::InsertRow(insert_row::InsertRow { index, row }) => format!("insert-row index={index} row={}", enc_row(row)),
        TsvMutation::RemoveRow(remove_row::RemoveRow { index }) => format!("remove-row index={index}"),
        TsvMutation::SetCell(set_cell::SetCell { row_index, field_index, value }) => format!("set-cell row-index={row_index} field-index={field_index} value={}", enc_str(value),),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_tsv_mutation(line: &str) -> Result<TsvMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("tsv mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("tsv mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| TsvMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_tsv_snapshot(arg("snapshot")?)? })),
        "set-trailing-newline" => Ok(TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: arg("trailing-newline")? == "1" })),
        "set-line-ending" => Ok(TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: crate::standards::iana::subsets::any::io::text::diff::dec_line_ending(arg("line-ending")?)? })),
        "insert-row" => Ok(TsvMutation::InsertRow(insert_row::InsertRow { index: usize_arg("index")?, row: dec_row(arg("row")?)? })),
        "remove-row" => Ok(TsvMutation::RemoveRow(remove_row::RemoveRow { index: usize_arg("index")? })),
        "set-cell" => Ok(TsvMutation::SetCell(set_cell::SetCell { row_index: usize_arg("row-index")?, field_index: usize_arg("field-index")?, value: dec_str(arg("value")?)? })),
        other => Err(format!("tsv mutation: unknown keyword {other:?}")),
    }
}

impl OpText for TsvMutation {
    fn print_op(&self) -> String {
        print_tsv_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_tsv_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
