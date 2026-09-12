use super::*;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn location(city: &str) -> EpwLocation {
    EpwLocation { city: city.into(), state_province: "NI".into(), country: "DEU".into(), source: "SRC".into(), wmo: "10238".into(), latitude: "52.37".into(), longitude: "9.74".into(), time_zone: "1.0".into(), elevation: "55.0".into() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn data_periods() -> EpwDataPeriods {
    EpwDataPeriods { records_per_hour: 1, periods: vec![crate::standards::energyplus::subsets::any::schema::snapshot::EpwDataPeriod { name: "Data".into(), start_day_of_week: "Sunday".into(), start_date: " 1/ 1".into(), end_date: " 1/ 1".into() }] }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> EpwSnapshot {
    let mut a = base_snapshot();
    a.records = vec![record("1", "-7.8"), record("2", "-7.2"), record("3", "-6.2")];
    a
}
/// 🧬️ Sweep B: every top-level scalar field changes, record 0 is removed, record 1 (now
/// index 0) is modified in every one of its 35 columns, record 2 (now index 1) is untouched,
/// and a brand-new record is added at the end.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> EpwSnapshot {
    let mut modified = EpwRecord::default();
    for i in 0..crate::standards::energyplus::subsets::any::schema::snapshot::EPW_RECORD_FIELD_COUNT {
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
            periods: vec![crate::standards::energyplus::subsets::any::schema::snapshot::EpwDataPeriod { name: "Swept".into(), start_day_of_week: "Monday".into(), start_date: "1/ 2".into(), end_date: "1/ 2".into() }],
        },
        records: vec![modified, record("3", "-6.2"), record("99", "swept-new")],
        ..EpwSnapshot::default()
    }
}
//#endregion 🔖️FieldSweepFixtures

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    let variants = vec![
        EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        EpwMutation::SetLocation(set_location::SetLocation { location: location("Munich") }),
        EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value: "DESIGN CONDITIONS,changed".into() }),
        EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods: data_periods() }),
        EpwMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: Box::new(record("50", "1.0")) }),
        EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: 1, field_index: 6, value: "changed".into() }),
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
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = base_snapshot();
    let variants = vec![
        EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        EpwMutation::SetLocation(set_location::SetLocation { location: location("Munich") }),
        EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value: "DESIGN CONDITIONS,changed".into() }),
        EpwMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: Box::new(record("50", "1.0")) }),
        EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: 1, field_index: 6, value: "changed".into() }),
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
#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    let base = base_snapshot();

    let d1 = EpwMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: Box::new(record("40", "ins")) }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Remove-before absorb mismatch");

    let d1 = EpwMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: Box::new(record("41", "f")) }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = EpwMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: Box::new(record("42", "g")) }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Insert-same-index absorb mismatch");
    assert_eq!(after.records.len(), base.records.len() + 2, "both inserts must survive");

    let d1 = EpwMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: Box::new(record("43", "orig")) }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: 1, field_index: 6, value: "patched".into() }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Add+SetRecordField absorb mismatch");
    assert_eq!(after.records[1].dry_bulb_temp, "patched");

    let d1 = EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: 1, field_index: 6, value: "will-vanish".into() }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: 1 }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Modify+Remove absorb mismatch");

    let base = base_snapshot();
    let d1 = EpwMutation::InsertRecord(insert_record::InsertRecord { index: 0, record: Box::new(record("44", "a")) }).diff(&base);
    let s1 = d1.diff().apply(&base).unwrap();
    let d2 = EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: 0, field_index: 6, value: "a2".into() }).diff(&s1);
    let s2 = d2.diff().apply(&s1).unwrap();
    let d3 = EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: 2 }).diff(&s2);
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
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = base_snapshot();
    let b = sweep_b();
    assert_eq!(EpwDiff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(EpwDiff::between(&b, &a).apply(&b).unwrap(), a);
    assert!(EpwDiff::between(&a, &a).is_empty());
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️FieldSweep
#[semio_framework_async_macros::async_test]
async fn field_sweep_every_mutable_field_changes() {
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
    for i in 0..crate::standards::energyplus::subsets::any::schema::snapshot::EPW_RECORD_FIELD_COUNT {
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
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let mutations = vec![
        EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        EpwMutation::SetLocation(set_location::SetLocation { location: location("Tricky, [City]") }),
        EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value: "DESIGN CONDITIONS,tricky, [value]".into() }),
        EpwMutation::SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods { value: "TYPICAL/EXTREME PERIODS,x".into() }),
        EpwMutation::SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures { value: "GROUND TEMPERATURES,x".into() }),
        EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value: "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0".into() }),
        EpwMutation::SetComments1(set_comments1::SetComments1 { value: "COMMENTS 1,x".into() }),
        EpwMutation::SetComments2(set_comments2::SetComments2 { value: "COMMENTS 2,x".into() }),
        EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods: data_periods() }),
        EpwMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: Box::new(record("12", "tricky, [value]")) }),
        EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: 1, field_index: 6, value: "with, comma [and] brackets".into() }),
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

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `EpwMutation` without a matching kebab-case spelling here, which is what
/// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
/// `kinds` array as text (the framework never parses Rust, so this is the only side that can
/// prove the manifest matches) and asserts the same list, in the same order.
///
#[semio_framework_async_macros::async_test]
async fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &EpwMutation) -> &'static str {
        match mutation {
            EpwMutation::SetSnapshot(_) => "set-snapshot",
            EpwMutation::SetLocation(_) => "set-location",
            EpwMutation::SetDesignConditions(_) => "set-design-conditions",
            EpwMutation::SetTypicalExtremePeriods(_) => "set-typical-extreme-periods",
            EpwMutation::SetGroundTemperatures(_) => "set-ground-temperatures",
            EpwMutation::SetHolidaysDst(_) => "set-holidays-dst",
            EpwMutation::SetComments1(_) => "set-comments-1",
            EpwMutation::SetComments2(_) => "set-comments-2",
            EpwMutation::SetDataPeriods(_) => "set-data-periods",
            EpwMutation::InsertRecord(_) => "insert-record",
            EpwMutation::RemoveRecord(_) => "remove-record",
            EpwMutation::SetRecordField(_) => "set-record-field",
        }
    }
    let samples = [
        EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: EpwSnapshot::default() }),
        EpwMutation::SetLocation(set_location::SetLocation { location: EpwLocation::default() }),
        EpwMutation::SetDesignConditions(set_design_conditions::SetDesignConditions { value: String::new() }),
        EpwMutation::SetTypicalExtremePeriods(set_typical_extreme_periods::SetTypicalExtremePeriods { value: String::new() }),
        EpwMutation::SetGroundTemperatures(set_ground_temperatures::SetGroundTemperatures { value: String::new() }),
        EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value: String::new() }),
        EpwMutation::SetComments1(set_comments1::SetComments1 { value: String::new() }),
        EpwMutation::SetComments2(set_comments2::SetComments2 { value: String::new() }),
        EpwMutation::SetDataPeriods(set_data_periods::SetDataPeriods { data_periods: EpwDataPeriods::default() }),
        EpwMutation::InsertRecord(insert_record::InsertRecord { index: 0, record: Box::default() }),
        EpwMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: 0, field_index: 0, value: String::new() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every EpwMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match EpwMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw
