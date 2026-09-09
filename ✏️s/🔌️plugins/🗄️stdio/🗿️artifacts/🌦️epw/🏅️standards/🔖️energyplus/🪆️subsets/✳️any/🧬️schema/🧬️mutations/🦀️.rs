//! 🧬️ EpwMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `EpwDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index/field-aware, reading the pre-state it needs from `base`.

use crate::standards::energyplus::subsets::any::schema::diff::{
    dec_data_periods, dec_location, dec_record, dec_str, diff_set_snapshot, enc_data_periods, enc_location, enc_record, enc_str, split_top_level, strip_brackets, EpwDiff, EpwRecordAdded, EpwRecordDiff, EpwRecordModified, EpwRecordsDiff,
};
use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

//#region 🔖️Mutations
#[path = "📥insert-record/🦀️.rs"]
pub mod insert_record;
#[path = "📤remove-record/🦀️.rs"]
pub mod remove_record;
#[path = "💬set-comments1/🦀️.rs"]
pub mod set_comments1;
#[path = "🗨️set-comments2/🦀️.rs"]
pub mod set_comments2;
#[path = "📅set-data-periods/🦀️.rs"]
pub mod set_data_periods;
#[path = "🌡️set-design-conditions/🦀️.rs"]
pub mod set_design_conditions;
#[path = "🌍set-ground-temperatures/🦀️.rs"]
pub mod set_ground_temperatures;
#[path = "🎉set-holidays-dst/🦀️.rs"]
pub mod set_holidays_dst;
#[path = "📍set-location/🦀️.rs"]
pub mod set_location;
#[path = "🎚️set-record-field/🦀️.rs"]
pub mod set_record_field;
/// 📐️ Typed content mutation for `stdio.epw`.
/// 🧪️ F6: hand-rolled — `#[derive(dsl::DslOps)]` is not attempted here (the enum embeds
/// `EpwSnapshot`/`EpwLocation`/`EpwDataPeriods`, none of which implement `dsl::DslField`; wiring
/// that up is out of this ticket's scope, matching csv's/gif's own documented hand-roll rationale).
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "📆set-typical-extreme-periods/🦀️.rs"]
pub mod set_typical_extreme_periods;
//#endregion 🔖️Leaves

/// 🧭️ `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly
/// one leaf payload (a unit variant wraps none) and asserts `is_approved_verb(SEMANTICS.verb)`,
/// and `no` is not an approved verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EpwSnapshot, diff = EpwDiff, schema = "EpwMutation")]
pub enum EpwMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 📍️ Replaces the LOCATION header line.
    SetLocation(set_location::SetLocation),
    /// 🌡️ Replaces the DESIGN CONDITIONS header line (retained verbatim).
    SetDesignConditions(set_design_conditions::SetDesignConditions),
    /// 📆️ Replaces the TYPICAL/EXTREME PERIODS header line (retained verbatim).
    SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods),
    /// 🌍️ Replaces the GROUND TEMPERATURES header line (retained verbatim).
    SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures),
    /// 🎉️ Replaces the HOLIDAYS/DAYLIGHT SAVINGS header line (retained verbatim).
    SetHolidaysDst(set_holidays_dst::SetHolidaysDst),
    /// 💬️ Replaces the COMMENTS 1 header line (retained verbatim).
    SetComments1(set_comments1::SetComments1),
    /// 🗨️ Replaces the COMMENTS 2 header line (retained verbatim).
    SetComments2(set_comments2::SetComments2),
    /// 📅️ Replaces the DATA PERIODS header line.
    SetDataPeriods(set_data_periods::SetDataPeriods),
    /// 📥️ Inserts a whole record at `index` (clamped to the end on apply).
    InsertRecord(insert_record::InsertRecord),
    /// 📤️ Removes the record at `index`.
    RemoveRecord(remove_record::RemoveRecord),
    /// 🎚️ Patches one of a record's 35 columns in place, addressed by its canonical wire index
    /// (see `EpwRecord::field_at`).
    SetRecordField(set_record_field::SetRecordField),
}

/// 🧾️ Kebab-case spelling of every `EpwMutation` variant, in declaration order — the exhaustive
/// mutation catalog `epw-energyplus-any` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &[
    "set-snapshot",
    "set-location",
    "set-design-conditions",
    "set-typical-extreme-periods",
    "set-ground-temperatures",
    "set-holidays-dst",
    "set-comments-1",
    "set-comments-2",
    "set-data-periods",
    "insert-record",
    "remove-record",
    "set-record-field",
];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_epw_mutation(snapshot: &mut EpwSnapshot, mutation: &EpwMutation) -> protocol::MutationOutcome<EpwDiff> {
    let outcome = <EpwMutation as Mutation<EpwSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &EpwMutation, base: &EpwSnapshot) -> protocol::MutationOutcome<EpwDiff> {
    protocol::MutationOutcome::new(match this {
        EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        EpwMutation::SetLocation(set_location::SetLocation { location }) => EpwDiff { location: Some(location.clone()), ..EpwDiff::default() },
        EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value }) => EpwDiff { design_conditions: Some(value.clone()), ..EpwDiff::default() },
        EpwMutation::SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods { value }) => EpwDiff { typical_extreme_periods: Some(value.clone()), ..EpwDiff::default() },
        EpwMutation::SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures { value }) => EpwDiff { ground_temperatures: Some(value.clone()), ..EpwDiff::default() },
        EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value }) => EpwDiff { holidays_dst: Some(value.clone()), ..EpwDiff::default() },
        EpwMutation::SetComments1(set_comments1::SetComments1 { value }) => EpwDiff { comments_1: Some(value.clone()), ..EpwDiff::default() },
        EpwMutation::SetComments2(set_comments2::SetComments2 { value }) => EpwDiff { comments_2: Some(value.clone()), ..EpwDiff::default() },
        EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods }) => EpwDiff { data_periods: Some(data_periods.clone()), ..EpwDiff::default() },
        EpwMutation::InsertRecord(insert_record::InsertRecord { index, record }) => {
            EpwDiff { records: Some(EpwRecordsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![EpwRecordAdded { index: *index, record: record.as_ref().clone() }] }), ..EpwDiff::default() }
        }
        EpwMutation::RemoveRecord(remove_record::RemoveRecord { index }) => EpwDiff { records: Some(EpwRecordsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }), ..EpwDiff::default() },
        EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index, field_index, value }) => {
            let mut fdiff = EpwRecordDiff::default();
            fdiff.set_at(*field_index, Some(value.clone()));
            EpwDiff { records: Some(EpwRecordsDiff { removed: Vec::new(), modified: vec![EpwRecordModified { index: *record_index, diff: fdiff }], added: Vec::new() }), ..EpwDiff::default() }
        }
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &EpwMutation, base: &EpwSnapshot) -> Vec<EpwMutation> {
    match this {
        EpwMutation::SetSnapshot(_) => vec![EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        EpwMutation::SetLocation(_) => vec![EpwMutation::SetLocation(set_location::SetLocation { location: base.location.clone() })],
        EpwMutation::SetDesignConditions(_) => vec![EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value: base.design_conditions.clone() })],
        EpwMutation::SetTypicalExtremePeriods(_) => vec![EpwMutation::SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods { value: base.typical_extreme_periods.clone() })],
        EpwMutation::SetGroundTemperatures(_) => vec![EpwMutation::SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures { value: base.ground_temperatures.clone() })],
        EpwMutation::SetHolidaysDst(_) => vec![EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value: base.holidays_dst.clone() })],
        EpwMutation::SetComments1(_) => vec![EpwMutation::SetComments1(set_comments1::SetComments1 { value: base.comments_1.clone() })],
        EpwMutation::SetComments2(_) => vec![EpwMutation::SetComments2(set_comments2::SetComments2 { value: base.comments_2.clone() })],
        EpwMutation::SetDataPeriods(_) => vec![EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods: base.data_periods.clone() })],
        EpwMutation::InsertRecord(insert_record::InsertRecord { index, .. }) => vec![EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: *index })],
        EpwMutation::RemoveRecord(remove_record::RemoveRecord { index }) => match base.records.get(*index) {
            Some(record) => vec![EpwMutation::InsertRecord(insert_record::InsertRecord { index: *index, record: Box::new(record.clone()) })],
            None => Vec::new(),
        },
        EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index, field_index, .. }) => match base.records.get(*record_index).and_then(|r| r.field_at(*field_index)) {
            Some(prior) => vec![EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: *record_index, field_index: *field_index, value: prior.to_string() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `EpwMutation` — reuses `EpwDiff`'s `pub(crate)`
/// grammar primitives. Grammar: `keyword arg=value ...` (space-separated), same convention csv's/
/// gif89a's/svg's own hand-rolled `OpText` impls use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_epw_snapshot(s: &EpwSnapshot) -> String {
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
fn dec_epw_snapshot(s: &str) -> Result<EpwSnapshot, String> {
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
fn print_epw_mutation(m: &EpwMutation) -> String {
    match m {
        EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_epw_snapshot(snapshot)),
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
fn parse_epw_mutation(line: &str) -> Result<EpwMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("epw mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("epw mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_epw_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification as `EpwDiff`'s hand-rolled codec.
impl OpBinary for EpwMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
