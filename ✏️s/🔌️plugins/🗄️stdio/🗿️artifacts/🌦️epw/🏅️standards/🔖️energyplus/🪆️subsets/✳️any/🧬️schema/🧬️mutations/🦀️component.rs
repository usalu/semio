//! 🧬️ EpwMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `EpwDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index/field-aware, reading the pre-state it needs from `base`.

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::{
    dec_data_periods, dec_location, dec_record, dec_str, diff_set_snapshot, enc_data_periods, enc_location, enc_record, enc_str, split_top_level, strip_brackets, EpwDiff, EpwRecordAdded, EpwRecordDiff, EpwRecordModified, EpwRecordsDiff,
};
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.epw`.
/// 🧪️ F6: hand-rolled — `#[derive(dsl::DslOps)]` is not attempted here (the enum embeds
/// `EpwSnapshot`/`EpwLocation`/`EpwDataPeriods`, none of which implement `dsl::DslField`; wiring
/// that up is out of this ticket's scope, matching csv's/gif's own documented hand-roll rationale).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum EpwMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: EpwSnapshot,
    },
    /// 📍️ Replaces the LOCATION header line.
    SetLocation {
        location: EpwLocation,
    },
    /// 🌡️ Replaces the DESIGN CONDITIONS header line (retained verbatim).
    SetDesignConditions {
        value: String,
    },
    /// 📆️ Replaces the TYPICAL/EXTREME PERIODS header line (retained verbatim).
    SetTypicalExtremePeriods {
        value: String,
    },
    /// 🌍️ Replaces the GROUND TEMPERATURES header line (retained verbatim).
    SetGroundTemperatures {
        value: String,
    },
    /// 🎉️ Replaces the HOLIDAYS/DAYLIGHT SAVINGS header line (retained verbatim).
    SetHolidaysDst {
        value: String,
    },
    /// 💬️ Replaces the COMMENTS 1 header line (retained verbatim).
    SetComments1 {
        value: String,
    },
    /// 💬️ Replaces the COMMENTS 2 header line (retained verbatim).
    SetComments2 {
        value: String,
    },
    /// 📅️ Replaces the DATA PERIODS header line.
    SetDataPeriods {
        data_periods: EpwDataPeriods,
    },
    /// ➕️ Inserts a whole record at `index` (clamped to the end on apply).
    InsertRecord {
        index: usize,
        record: EpwRecord,
    },
    /// ➖️ Removes the record at `index`.
    RemoveRecord {
        index: usize,
    },
    /// ✏️ Patches one of a record's 35 columns in place, addressed by its canonical wire index
    /// (see `EpwRecord::field_at`).
    SetRecordField {
        record_index: usize,
        field_index: usize,
        value: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
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
impl Mutation<EpwSnapshot> for EpwMutation {
    type Diff = EpwDiff;

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            EpwMutation::NoMutation => EpwDiff::default(),
            EpwMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            EpwMutation::SetLocation { location } => EpwDiff { location: Some(location.clone()), ..EpwDiff::default() },
            EpwMutation::SetDesignConditions { value } => EpwDiff { design_conditions: Some(value.clone()), ..EpwDiff::default() },
            EpwMutation::SetTypicalExtremePeriods { value } => EpwDiff { typical_extreme_periods: Some(value.clone()), ..EpwDiff::default() },
            EpwMutation::SetGroundTemperatures { value } => EpwDiff { ground_temperatures: Some(value.clone()), ..EpwDiff::default() },
            EpwMutation::SetHolidaysDst { value } => EpwDiff { holidays_dst: Some(value.clone()), ..EpwDiff::default() },
            EpwMutation::SetComments1 { value } => EpwDiff { comments_1: Some(value.clone()), ..EpwDiff::default() },
            EpwMutation::SetComments2 { value } => EpwDiff { comments_2: Some(value.clone()), ..EpwDiff::default() },
            EpwMutation::SetDataPeriods { data_periods } => EpwDiff { data_periods: Some(data_periods.clone()), ..EpwDiff::default() },
            EpwMutation::InsertRecord { index, record } => EpwDiff { records: Some(EpwRecordsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![EpwRecordAdded { index: *index, record: record.clone() }] }), ..EpwDiff::default() },
            EpwMutation::RemoveRecord { index } => EpwDiff { records: Some(EpwRecordsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }), ..EpwDiff::default() },
            EpwMutation::SetRecordField { record_index, field_index, value } => {
                let mut fdiff = EpwRecordDiff::default();
                fdiff.set_at(*field_index, Some(value.clone()));
                EpwDiff { records: Some(EpwRecordsDiff { removed: Vec::new(), modified: vec![EpwRecordModified { index: *record_index, diff: fdiff }], added: Vec::new() }), ..EpwDiff::default() }
            }
        })
    }

    fn inverse(&self, base: &EpwSnapshot) -> Vec<Self> {
        match self {
            EpwMutation::NoMutation => vec![EpwMutation::NoMutation],
            EpwMutation::SetSnapshot { .. } => vec![EpwMutation::SetSnapshot { snapshot: base.clone() }],
            EpwMutation::SetLocation { .. } => vec![EpwMutation::SetLocation { location: base.location.clone() }],
            EpwMutation::SetDesignConditions { .. } => vec![EpwMutation::SetDesignConditions { value: base.design_conditions.clone() }],
            EpwMutation::SetTypicalExtremePeriods { .. } => vec![EpwMutation::SetTypicalExtremePeriods { value: base.typical_extreme_periods.clone() }],
            EpwMutation::SetGroundTemperatures { .. } => vec![EpwMutation::SetGroundTemperatures { value: base.ground_temperatures.clone() }],
            EpwMutation::SetHolidaysDst { .. } => vec![EpwMutation::SetHolidaysDst { value: base.holidays_dst.clone() }],
            EpwMutation::SetComments1 { .. } => vec![EpwMutation::SetComments1 { value: base.comments_1.clone() }],
            EpwMutation::SetComments2 { .. } => vec![EpwMutation::SetComments2 { value: base.comments_2.clone() }],
            EpwMutation::SetDataPeriods { .. } => vec![EpwMutation::SetDataPeriods { data_periods: base.data_periods.clone() }],
            EpwMutation::InsertRecord { index, .. } => vec![EpwMutation::RemoveRecord { index: *index }],
            EpwMutation::RemoveRecord { index } => match base.records.get(*index) {
                Some(record) => vec![EpwMutation::InsertRecord { index: *index, record: record.clone() }],
                None => vec![EpwMutation::NoMutation],
            },
            EpwMutation::SetRecordField { record_index, field_index, .. } => match base.records.get(*record_index).and_then(|r| r.field_at(*field_index)) {
                Some(prior) => vec![EpwMutation::SetRecordField { record_index: *record_index, field_index: *field_index, value: prior.to_string() }],
                None => vec![EpwMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `EpwMutation` — reuses `EpwDiff`'s `pub(crate)`
/// grammar primitives. Grammar: `keyword arg=value ...` (space-separated), same convention csv's/
/// gif89a's/svg's own hand-rolled `OpText` impls use.
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

fn print_epw_mutation(m: &EpwMutation) -> String {
    match m {
        EpwMutation::NoMutation => "no-mutation".to_string(),
        EpwMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_epw_snapshot(snapshot)),
        EpwMutation::SetLocation { location } => format!("set-location location={}", enc_location(location)),
        EpwMutation::SetDesignConditions { value } => format!("set-design-conditions value={}", enc_str(value)),
        EpwMutation::SetTypicalExtremePeriods { value } => format!("set-typical-extreme-periods value={}", enc_str(value)),
        EpwMutation::SetGroundTemperatures { value } => format!("set-ground-temperatures value={}", enc_str(value)),
        EpwMutation::SetHolidaysDst { value } => format!("set-holidays-dst value={}", enc_str(value)),
        EpwMutation::SetComments1 { value } => format!("set-comments-1 value={}", enc_str(value)),
        EpwMutation::SetComments2 { value } => format!("set-comments-2 value={}", enc_str(value)),
        EpwMutation::SetDataPeriods { data_periods } => format!("set-data-periods data-periods={}", enc_data_periods(data_periods)),
        EpwMutation::InsertRecord { index, record } => format!("insert-record index={index} record={}", enc_record(record)),
        EpwMutation::RemoveRecord { index } => format!("remove-record index={index}"),
        EpwMutation::SetRecordField { record_index, field_index, value } => format!("set-record-field record-index={record_index} field-index={field_index} value={}", enc_str(value),),
    }
}
fn parse_epw_mutation(line: &str) -> Result<EpwMutation, String> {
    if line == "no-mutation" {
        return Ok(EpwMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("epw mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("epw mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(EpwMutation::SetSnapshot { snapshot: dec_epw_snapshot(arg("snapshot")?)? }),
        "set-location" => Ok(EpwMutation::SetLocation { location: dec_location(arg("location")?)? }),
        "set-design-conditions" => Ok(EpwMutation::SetDesignConditions { value: dec_str(arg("value")?)? }),
        "set-typical-extreme-periods" => Ok(EpwMutation::SetTypicalExtremePeriods { value: dec_str(arg("value")?)? }),
        "set-ground-temperatures" => Ok(EpwMutation::SetGroundTemperatures { value: dec_str(arg("value")?)? }),
        "set-holidays-dst" => Ok(EpwMutation::SetHolidaysDst { value: dec_str(arg("value")?)? }),
        "set-comments-1" => Ok(EpwMutation::SetComments1 { value: dec_str(arg("value")?)? }),
        "set-comments-2" => Ok(EpwMutation::SetComments2 { value: dec_str(arg("value")?)? }),
        "set-data-periods" => Ok(EpwMutation::SetDataPeriods { data_periods: dec_data_periods(arg("data-periods")?)? }),
        "insert-record" => Ok(EpwMutation::InsertRecord { index: usize_arg("index")?, record: dec_record(arg("record")?)? }),
        "remove-record" => Ok(EpwMutation::RemoveRecord { index: usize_arg("index")? }),
        "set-record-field" => Ok(EpwMutation::SetRecordField { record_index: usize_arg("record-index")?, field_index: usize_arg("field-index")?, value: dec_str(arg("value")?)? }),
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
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    //#region 🔖️Fixtures
    fn location(city: &str) -> EpwLocation {
        EpwLocation { city: city.into(), state_province: "NI".into(), country: "DEU".into(), source: "SRC".into(), wmo: "10238".into(), latitude: "52.37".into(), longitude: "9.74".into(), time_zone: "1.0".into(), elevation: "55.0".into() }
    }
    fn record(hour: &str, temp: &str) -> EpwRecord {
        let mut r = EpwRecord::default();
        r.year = "2026".into();
        r.month = "1".into();
        r.day = "15".into();
        r.hour = hour.into();
        r.minute = "0".into();
        r.dry_bulb_temp = temp.into();
        r.visibility = "20.0".into();
        r
    }
    fn data_periods() -> EpwDataPeriods {
        EpwDataPeriods {
            records_per_hour: 1,
            periods: vec![crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwDataPeriod { name: "Data".into(), start_day_of_week: "Sunday".into(), start_date: " 1/ 1".into(), end_date: " 1/ 1".into() }],
        }
    }
    fn base_snapshot() -> EpwSnapshot {
        EpwSnapshot {
            location: location("Hannover"),
            design_conditions: "DESIGN CONDITIONS,0".into(),
            typical_extreme_periods: "TYPICAL/EXTREME PERIODS,0".into(),
            ground_temperatures: "GROUND TEMPERATURES,0".into(),
            holidays_dst: "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0".into(),
            comments_1: "COMMENTS 1,x".into(),
            comments_2: "COMMENTS 2,y".into(),
            data_periods: data_periods(),
            records: vec![record("1", "-7.8"), record("2", "-7.2"), record("3", "-6.2")],
            ..EpwSnapshot::default()
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ Canonical "differs in every mutable field" snapshot A: 3 records — one removed, one
    /// modified in every one of its 35 columns, one untouched (anchor for the added record's index).
    fn sweep_a() -> EpwSnapshot {
        let mut a = base_snapshot();
        a.records = vec![record("1", "-7.8"), record("2", "-7.2"), record("3", "-6.2")];
        a
    }
    /// 🧬️ Sweep B: every top-level scalar field changes, record 0 is removed, record 1 (now
    /// index 0) is modified in every one of its 35 columns, record 2 (now index 1) is untouched,
    /// and a brand-new record is added at the end.
    fn sweep_b() -> EpwSnapshot {
        let mut modified = EpwRecord::default();
        for i in 0..crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EPW_RECORD_FIELD_COUNT {
            modified.set_field_at(i, format!("swept-{i}"));
        }
        EpwSnapshot {
            location: location("Berlin"),
            design_conditions: "DESIGN CONDITIONS,1,swept".into(),
            typical_extreme_periods: "TYPICAL/EXTREME PERIODS,1,swept".into(),
            ground_temperatures: "GROUND TEMPERATURES,1,swept".into(),
            holidays_dst: "HOLIDAYS/DAYLIGHT SAVINGS,Yes,1,1,1".into(),
            comments_1: "COMMENTS 1,swept".into(),
            comments_2: "COMMENTS 2,swept".into(),
            data_periods: EpwDataPeriods {
                records_per_hour: 2,
                periods: vec![crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwDataPeriod { name: "Swept".into(), start_day_of_week: "Monday".into(), start_date: "1/ 2".into(), end_date: "1/ 2".into() }],
            },
            records: vec![modified, record("3", "-6.2"), record("99", "swept-new")],
            ..EpwSnapshot::default()
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️MutationDiffLaw
    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        let variants = vec![
            EpwMutation::NoMutation,
            EpwMutation::SetSnapshot { snapshot: sweep_b() },
            EpwMutation::SetLocation { location: location("Munich") },
            EpwMutation::SetDesignConditions { value: "DESIGN CONDITIONS,changed".into() },
            EpwMutation::SetDataPeriods { data_periods: data_periods() },
            EpwMutation::InsertRecord { index: 1, record: record("50", "1.0") },
            EpwMutation::RemoveRecord { index: 0 },
            EpwMutation::SetRecordField { record_index: 1, field_index: 6, value: "changed".into() },
        ];
        for m in variants {
            let diff = m.diff(&base);
            let expected = diff.diff().apply(&base).unwrap();

            let mut via_apply = base.clone();
            let returned_diff = apply_epw_mutation(&mut via_apply, &m);

            assert_eq!(via_apply, expected, "apply_epw_mutation mismatch for {m:?}");
            assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            EpwMutation::NoMutation,
            EpwMutation::SetSnapshot { snapshot: sweep_b() },
            EpwMutation::SetLocation { location: location("Munich") },
            EpwMutation::SetDesignConditions { value: "DESIGN CONDITIONS,changed".into() },
            EpwMutation::InsertRecord { index: 1, record: record("50", "1.0") },
            EpwMutation::RemoveRecord { index: 0 },
            EpwMutation::SetRecordField { record_index: 1, field_index: 6, value: "changed".into() },
        ];
        for m in variants {
            let mut forward = base.clone();
            apply_epw_mutation(&mut forward, &m);
            for inv in m.inverse(&base) {
                apply_epw_mutation(&mut forward, &inv);
            }
            assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

            let d = m.diff(&base);
            let mid = d.diff().apply(&base).unwrap();
            let back = d.diff().inverse(&base).apply(&mid).unwrap();
            assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        let d1 = EpwMutation::InsertRecord { index: 2, record: record("40", "ins") }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = EpwMutation::RemoveRecord { index: 0 }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Remove-before absorb mismatch");

        let d1 = EpwMutation::InsertRecord { index: 2, record: record("41", "f") }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = EpwMutation::InsertRecord { index: 2, record: record("42", "g") }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.records.len(), base.records.len() + 2, "both inserts must survive");

        let d1 = EpwMutation::InsertRecord { index: 1, record: record("43", "orig") }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = EpwMutation::SetRecordField { record_index: 1, field_index: 6, value: "patched".into() }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Add+SetRecordField absorb mismatch");
        assert_eq!(after.records[1].dry_bulb_temp, "patched");

        let d1 = EpwMutation::SetRecordField { record_index: 1, field_index: 6, value: "will-vanish".into() }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = EpwMutation::RemoveRecord { index: 1 }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Modify+Remove absorb mismatch");

        let base = base_snapshot();
        let d1 = EpwMutation::InsertRecord { index: 0, record: record("44", "a") }.diff(&base);
        let s1 = d1.diff().apply(&base).unwrap();
        let d2 = EpwMutation::SetRecordField { record_index: 0, field_index: 6, value: "a2".into() }.diff(&s1);
        let s2 = d2.diff().apply(&s1).unwrap();
        let d3 = EpwMutation::RemoveRecord { index: 2 }.diff(&s2);
        let s3 = d3.diff().apply(&s2).unwrap();

        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).unwrap(), s3);
        assert_eq!(right.apply(&base).unwrap(), s3);
        assert_eq!(left.apply(&base).unwrap(), right.apply(&base).unwrap(), "absorb must be associative");
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let b = sweep_b();
        assert_eq!(EpwDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(EpwDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(EpwDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️FieldSweep
    #[test]
    fn field_sweep_every_mutable_field_changes() {
        let a = sweep_a();
        let b = sweep_b();

        let d_ab = EpwDiff::between(&a, &b);
        assert_eq!(d_ab.apply(&a).unwrap(), b, "between(a,b).apply(a) == b");

        let d_ba = EpwDiff::between(&b, &a);
        assert_eq!(d_ba.apply(&b).unwrap(), a, "between(b,a).apply(b) == a");

        assert!(d_ab.location.is_some(), "location must be populated");
        assert!(d_ab.design_conditions.is_some());
        assert!(d_ab.typical_extreme_periods.is_some());
        assert!(d_ab.ground_temperatures.is_some());
        assert!(d_ab.holidays_dst.is_some());
        assert!(d_ab.comments_1.is_some());
        assert!(d_ab.comments_2.is_some());
        assert!(d_ab.data_periods.is_some());
        // 🧭️ `EpwDiff::between` is positional (this file's own doc comment: "EPW rows have no
        // stable identity beyond position") — `min_len` covers only the index range both arrays
        // share, so a single `between()` call can populate `removed` XOR `added` (whichever side
        // is longer), never both at once. `sweep_a`/`sweep_b` are equal-length, so every index is
        // a same-position comparison: `modified` is the one populated triple here; `removed`/
        // `added` are exercised on their own just below via genuinely shorter/longer snapshots.
        let records = d_ab.records.as_ref().expect("records diff must be populated");
        assert!(records.removed.is_empty(), "equal-length record lists: no positional removal");
        assert!(!records.modified.is_empty(), "modified must be non-empty (every record differs positionally)");
        assert!(records.added.is_empty(), "equal-length record lists: no positional addition");
        assert_eq!(records.modified.len(), 3, "all three positions differ between sweep_a and sweep_b");
        let modified = &records.modified[0];
        for i in 0..crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EPW_RECORD_FIELD_COUNT {
            assert!(modified.diff.get_at(i).unwrap().is_some(), "column {i} of the modified record must be patched");
        }

        let mut shorter = a.clone();
        shorter.records.pop();
        let d_shrink = EpwDiff::between(&a, &shorter);
        let shrink_records = d_shrink.records.as_ref().expect("records diff must be populated");
        assert!(!shrink_records.removed.is_empty(), "a shorter record list must produce a removed entry");
        assert_eq!(d_shrink.apply(&a).unwrap(), shorter);

        let mut longer = a.clone();
        longer.records.push(record("4", "-5.0"));
        let d_grow = EpwDiff::between(&a, &longer);
        let grow_records = d_grow.records.as_ref().expect("records diff must be populated");
        assert!(!grow_records.added.is_empty(), "a longer record list must produce an added entry");
        assert_eq!(d_grow.apply(&a).unwrap(), longer);

        assert!(EpwDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    #[test]
    fn op_text_binary_roundtrip_law() {
        let mutations = vec![
            EpwMutation::NoMutation,
            EpwMutation::SetSnapshot { snapshot: sweep_b() },
            EpwMutation::SetLocation { location: location("Tricky, [City]") },
            EpwMutation::SetDesignConditions { value: "DESIGN CONDITIONS,tricky, [value]".into() },
            EpwMutation::SetTypicalExtremePeriods { value: "TYPICAL/EXTREME PERIODS,x".into() },
            EpwMutation::SetGroundTemperatures { value: "GROUND TEMPERATURES,x".into() },
            EpwMutation::SetHolidaysDst { value: "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0".into() },
            EpwMutation::SetComments1 { value: "COMMENTS 1,x".into() },
            EpwMutation::SetComments2 { value: "COMMENTS 2,x".into() },
            EpwMutation::SetDataPeriods { data_periods: data_periods() },
            EpwMutation::InsertRecord { index: 1, record: record("12", "tricky, [value]") },
            EpwMutation::RemoveRecord { index: 0 },
            EpwMutation::SetRecordField { record_index: 1, field_index: 6, value: "with, comma [and] brackets".into() },
        ];
        for m in mutations {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = EpwMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = EpwMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw
}
//#endregion 🧪️Tests
