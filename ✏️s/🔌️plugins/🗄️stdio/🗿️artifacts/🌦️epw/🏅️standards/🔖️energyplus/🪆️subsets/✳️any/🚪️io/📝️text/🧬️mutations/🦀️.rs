//! 📝️ Text representation codec surface for `stdio.epw` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::energyplus::subsets::any::schema::mutations::*;
use crate::standards::energyplus::subsets::any::schema::diff::{diff_set_snapshot, EpwDiff, EpwRecordAdded, EpwRecordDiff, EpwRecordModified, EpwRecordsDiff};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_record};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_record};
use crate::standards::energyplus::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::energyplus::subsets::any::io::text::diff::{split_top_level};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_data_periods};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_data_periods};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_location};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_location};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_str};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_str};
use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `EpwMutation` — reuses `EpwDiff`'s `pub(crate)`
/// grammar primitives. Grammar: `keyword arg=value ...` (space-separated), same convention csv's/
/// gif89a's/svg's own hand-rolled `OpText` impls use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_epw_snapshot(s: &EpwSnapshot) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},[{}]]",
        enc_str(&s.schema),
        enc_location(&s.location),
        enc_str(&s.design_conditions),
        enc_str(&s.typical_extreme_periods),
        enc_str(&s.ground_temperatures),
        enc_str(&s.holidays_dst),
        enc_str(&s.comments_1),
        enc_str(&s.comments_2),
        enc_data_periods(&s.data_periods),
        s.records.iter().map(enc_record).collect::<Vec<_>>().join(","),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_epw_snapshot(s: &str) -> Result<EpwSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, location, design_conditions, typical_extreme_periods, ground_temperatures, holidays_dst, comments_1, comments_2, data_periods, records] = parts.as_slice() else {
        return Err(format!("epw snapshot: expected 10 fields, got {}", parts.len()));
    };
    let records = split_top_level(strip_brackets(records)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_record).collect::<Result<Vec<_>, String>>()?;
    Ok(EpwSnapshot {
        schema: dec_str(schema)?,
        location: dec_location(location)?,
        design_conditions: dec_str(design_conditions)?,
        typical_extreme_periods: dec_str(typical_extreme_periods)?,
        ground_temperatures: dec_str(ground_temperatures)?,
        holidays_dst: dec_str(holidays_dst)?,
        comments_1: dec_str(comments_1)?,
        comments_2: dec_str(comments_2)?,
        data_periods: dec_data_periods(data_periods)?,
        records,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_epw_mutation(m: &EpwMutation) -> String {
    match m {
        EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_epw_snapshot(snapshot)),
        EpwMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        EpwMutation::SetLocation(set_location::SetLocation { location }) => format!("set-location location={}", enc_location(location)),
        EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value }) => format!("set-design-conditions value={}", enc_str(value)),
        EpwMutation::SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods { value }) => format!("set-typical-extreme-periods value={}", enc_str(value)),
        EpwMutation::SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures { value }) => format!("set-ground-temperatures value={}", enc_str(value)),
        EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value }) => format!("set-holidays-dst value={}", enc_str(value)),
        EpwMutation::SetComments1(set_comments1::SetComments1 { value }) => format!("set-comments-1 value={}", enc_str(value)),
        EpwMutation::SetComments2(set_comments2::SetComments2 { value }) => format!("set-comments-2 value={}", enc_str(value)),
        EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods }) => format!("set-data-periods data-periods={}", enc_data_periods(data_periods)),
        EpwMutation::InsertRecord(insert_record::InsertRecord { index, record }) => format!("insert-record index={index} record={}", enc_record(record)),
        EpwMutation::RemoveRecord(remove_record::RemoveRecord { index }) => format!("remove-record index={index}"),
        EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index, field_index, value }) => format!("set-record-field record-index={record_index} field-index={field_index} value={}", enc_str(value),),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_epw_mutation(line: &str) -> Result<EpwMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("epw mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("epw mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| EpwMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_epw_snapshot(arg("snapshot")?)? })),
        "set-location" => Ok(EpwMutation::SetLocation(set_location::SetLocation { location: dec_location(arg("location")?)? })),
        "set-design-conditions" => Ok(EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value: dec_str(arg("value")?)? })),
        "set-typical-extreme-periods" => Ok(EpwMutation::SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods { value: dec_str(arg("value")?)? })),
        "set-ground-temperatures" => Ok(EpwMutation::SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures { value: dec_str(arg("value")?)? })),
        "set-holidays-dst" => Ok(EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value: dec_str(arg("value")?)? })),
        "set-comments-1" => Ok(EpwMutation::SetComments1(set_comments1::SetComments1 { value: dec_str(arg("value")?)? })),
        "set-comments-2" => Ok(EpwMutation::SetComments2(set_comments2::SetComments2 { value: dec_str(arg("value")?)? })),
        "set-data-periods" => Ok(EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods: dec_data_periods(arg("data-periods")?)? })),
        "insert-record" => Ok(EpwMutation::InsertRecord(insert_record::InsertRecord { index: usize_arg("index")?, record: Box::new(dec_record(arg("record")?)?) })),
        "remove-record" => Ok(EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: usize_arg("index")? })),
        "set-record-field" => Ok(EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: usize_arg("record-index")?, field_index: usize_arg("field-index")?, value: dec_str(arg("value")?)? })),
        other => Err(format!("epw mutation: unknown keyword {other:?}")),
    }
}

impl OpText for EpwMutation {
    fn print_op(&self) -> String {
        print_epw_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_epw_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
