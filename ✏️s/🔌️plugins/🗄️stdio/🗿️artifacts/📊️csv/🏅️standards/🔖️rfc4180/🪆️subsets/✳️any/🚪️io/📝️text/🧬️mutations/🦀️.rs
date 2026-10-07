//! 📝️ Text representation codec surface for `stdio.csv` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::mutations::*;
use crate::schema::diff::{diff_set_snapshot, CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified, CsvRecordsDiff};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{dec_str};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{enc_str};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{dec_record};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{enc_record};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{split_top_level};
use crate::schema::snapshot::{CsvField, CsvRecord};
use crate::CsvSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `CsvMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above — a macro hygiene bug, not §3a/§3b) — reuses `CsvDiff`'s
/// `pub(crate)` grammar primitives (`hex`/`split_top_level`/`encode_option`/`enc_record`/...)
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), same convention gif89a's/svg's own hand-rolled `OpText` impls use, one
/// match arm per variant (no `DslVariants` scaffolding available since nothing here derives it).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_csv_snapshot(s: &CsvSnapshot) -> String {
    format!("[{},{},[{}]]", enc_str(&s.schema), if s.has_header { 1 } else { 0 }, s.records.iter().map(enc_record).collect::<Vec<_>>().join(","),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_csv_snapshot(s: &str) -> Result<CsvSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, has_header, records] = parts.as_slice() else {
        return Err(format!("csv snapshot: expected 3 fields, got {}", parts.len()));
    };
    let records = split_top_level(strip_brackets(records)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_record).collect::<Result<Vec<_>, String>>()?;
    Ok(CsvSnapshot { schema: dec_str(schema)?, has_header: *has_header == "1", records })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_csv_mutation(m: &CsvMutation) -> String {
    match m {
        CsvMutation::PatchSnapshot(_) => super::patch_snapshot::print(m).expect("patch snapshot variant"),
        CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_csv_snapshot(snapshot)),
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header }) => format!("set-has-header has-header={}", if *has_header { 1 } else { 0 }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index, record }) => format!("insert-record index={index} record={}", enc_record(record)),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index }) => format!("remove-record index={index}"),
        CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted }) => format!("set-field record-index={record_index} field-index={field_index} value={} quoted={}", enc_str(value), if *quoted { 1 } else { 0 },),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_csv_mutation(line: &str) -> Result<CsvMutation, String> {
    if line.split_once(' ').map_or(line, |(kind, _)| kind) == super::patch_snapshot::TEXT_OPCODE {
        return super::patch_snapshot::parse(line);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("csv mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("csv mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_csv_snapshot(arg("snapshot")?)? })),
        "set-has-header" => Ok(CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: arg("has-header")? == "1" })),
        "insert-record" => Ok(CsvMutation::InsertRecord(insert_record::InsertRecord { index: usize_arg("index")?, record: dec_record(arg("record")?)? })),
        "remove-record" => Ok(CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: usize_arg("index")? })),
        "set-field" => Ok(CsvMutation::SetField(set_field::SetField { record_index: usize_arg("record-index")?, field_index: usize_arg("field-index")?, value: dec_str(arg("value")?)?, quoted: arg("quoted")? == "1" })),
        other => Err(format!("csv mutation: unknown keyword {other:?}")),
    }
}

impl OpText for CsvMutation {
    fn print_op(&self) -> String {
        print_csv_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_csv_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
