use crate::apply_mutation;
use super::*;
use crate::schema::snapshot::CsvField;
use protocol::command::DiffAlgebra;

//#region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn field(value: &str, quoted: bool) -> CsvField {
    CsvField { value: value.into(), quoted }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn record(fields: &[(&str, bool)]) -> CsvRecord {
    CsvRecord { fields: fields.iter().map(|(v, q)| field(v, *q)).collect() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> CsvSnapshot {
    CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("name", false), ("note", true)]), record(&[("a", false), ("b", false)]), record(&[("x", false), ("y", false)])] }
}
//#endregion 🔖️Fixtures

//#region 🔖️FieldSweepFixtures
/// 🧬️ Canonical "differs in every mutable field" snapshot A: 3 records — one that will
/// be removed, one that will be modified in every field, one untouched (so `sweep_b`'s
/// added record has something stable to anchor its own index against).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> CsvSnapshot {
    CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("gone", false), ("also-gone", true)]), record(&[("old-a", false), ("old-b", true)]), record(&[("stable", false)])] }
}
/// 🧬️ Sweep B: `has_header` flips, record 0 is removed, record 1 (now index 0) is
/// modified in every field (value AND quoted), record 2 (now index 1) is untouched, and
/// a brand-new record is added at the end.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> CsvSnapshot {
    CsvSnapshot { schema: "stdio.csv".into(), has_header: false, records: vec![record(&[("new-a", true), ("new-b", false)]), record(&[("stable", false)]), record(&[("brand-new", true)])] }
}
//#endregion 🔖️FieldSweepFixtures

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    let variants = vec![
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new", true)]) }),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
    ];
    for m in variants {
        let diff = m.diff(&base);
        let expected = protocol::apply_diff(diff.diff(), &base).unwrap();

        let mut via_apply = base.clone();
        let returned_diff = apply_mutation(&mut via_apply, &m);

        assert_eq!(via_apply, expected, "apply_mutation mismatch for {m:?}");
        assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = base_snapshot();
    let variants = vec![
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new", true)]) }),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
    ];
    for m in variants {
        // 🔁️ mutation-level round trip
        let mut forward = base.clone();
        apply_mutation(&mut forward, &m);
        for inv in m.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
            apply_mutation(&mut forward, &inv);
        }
        assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

        // 🔁️ diff-level round trip
        let d = m.diff(&base);
        let mid = protocol::apply_diff(d.diff(), &base).unwrap();
        let back = protocol::apply_diff(d.diff().inverse(&base), &mid).unwrap();
        assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    let base = base_snapshot();

    // 🧩 Insert(2) + Remove(0): the two-op sequence base → mid → after.
    let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: record(&[("ins", false)]) }).diff(&base);
    let mid = protocol::apply_diff(d1.diff(), &base).unwrap();
    let d2 = CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }).diff(&mid);
    let after = protocol::apply_diff(d2.diff(), &mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(protocol::apply_diff(&composed, &base).unwrap(), after, "Insert+Remove-before absorb mismatch");

    // 🧩 Insert(2,f) + Insert(2,g): both must survive (fixes the old op-slot LWW bug).
    let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: record(&[("f", false)]) }).diff(&base);
    let mid = protocol::apply_diff(d1.diff(), &base).unwrap();
    let d2 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: record(&[("g", false)]) }).diff(&mid);
    let after = protocol::apply_diff(d2.diff(), &mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(protocol::apply_diff(&composed, &base).unwrap(), after, "Insert+Insert-same-index absorb mismatch");
    assert_eq!(after.records.len(), base.records.len() + 2, "both inserts must survive");

    // 🧩 Add + SetField: patch into the added payload.
    let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("orig", false)]) }).diff(&base);
    let mid = protocol::apply_diff(d1.diff(), &base).unwrap();
    let d2 = CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "patched".into(), quoted: true }).diff(&mid);
    let after = protocol::apply_diff(d2.diff(), &mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(protocol::apply_diff(&composed, &base).unwrap(), after, "Add+SetField absorb mismatch");
    assert_eq!(after.records[1].fields[0].value, "patched");

    // 🧩 Modify + Remove: modifying then removing the same record collapses to a removal.
    let d1 = CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "will-vanish".into(), quoted: false }).diff(&base);
    let mid = protocol::apply_diff(d1.diff(), &base).unwrap();
    let d2 = CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 1 }).diff(&mid);
    let after = protocol::apply_diff(d2.diff(), &mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(protocol::apply_diff(&composed, &base).unwrap(), after, "Modify+Remove absorb mismatch");

    // 🧩 Associativity over a triple.
    let base = base_snapshot();
    let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 0, record: record(&[("a", false)]) }).diff(&base);
    let s1 = protocol::apply_diff(d1.diff(), &base).unwrap();
    let d2 = CsvMutation::SetField(set_field::SetField { record_index: 0, field_index: 0, value: "a2".into(), quoted: true }).diff(&s1);
    let s2 = protocol::apply_diff(d2.diff(), &s1).unwrap();
    let d3 = CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 2 }).diff(&s2);
    let s3 = protocol::apply_diff(d3.diff(), &s2).unwrap();

    let mut left = d1.diff().clone();
    left.absorb(d2.diff().clone());
    left.absorb(d3.diff().clone());

    let mut d23 = d2.diff().clone();
    d23.absorb(d3.diff().clone());
    let mut right = d1.diff().clone();
    right.absorb(d23);

    assert_eq!(protocol::apply_diff(&left, &base).unwrap(), s3);
    assert_eq!(protocol::apply_diff(&right, &base).unwrap(), s3);
    assert_eq!(protocol::apply_diff(&left, &base).unwrap(), protocol::apply_diff(&right, &base).unwrap(), "absorb must be associative");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `CsvMutation` grammar —
/// exercises every variant, incl. a record payload whose fields contain the
/// grammar's own reserved separator characters (`,`/`[`/`]`/space) to prove hex-encoding
/// sidesteps escaping entirely.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let mutations = vec![
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: true }),
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new, [tricky]", true)]) }),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
        CsvMutation::SetField(set_field::SetField { record_index: 0, field_index: 2, value: "with, comma [and] brackets".into(), quoted: false }),
    ];
    for m in mutations {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = CsvMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = CsvMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️OpsGrammarConformanceLaw
/// 🧪️ P2-P1 item 6: `dsl::parse_grammar` + `dsl::Recognizer` recognize REAL `print_op`
/// output for several real mutations (not just one trivial case), incl. the nested positional-tuple
/// `record-value` production.
#[semio_framework_async_macros::async_test]
async fn ops_grammar_conformance_law() {
    let grammar_text = crate::standards::v_rfc4180::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO;
    let grammar = semio_framework_dsl::parse_grammar(grammar_text).expect("parse mutations grammar");
    let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");

    let mutations = vec![
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new", true)]) }),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
    ];
    for m in mutations {
        let printed = m.print_op();
        let ok = recognizer.recognize(&printed).unwrap_or_else(|e| panic!("recognize({printed:?}) errored: {e:?}"));
        assert!(ok, "mutations grammar must recognize real print_op output {printed:?} for {m:?}");
    }
}
//#endregion 🔖️OpsGrammarConformanceLaw

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `CsvMutation` without a matching kebab-case spelling here, which is what
/// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
/// `kinds` array as text (the framework never parses Rust, so this is the only side that can
/// prove the manifest matches) and asserts the same list, in the same order.
#[semio_framework_async_macros::async_test]
async fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &CsvMutation) -> &'static str {
        match mutation {
            CsvMutation::SetHasHeader(_) => "set-has-header",
            CsvMutation::InsertRecord(_) => "insert-record",
            CsvMutation::RemoveRecord(_) => "remove-record",
            CsvMutation::SetField(_) => "set-field",
        }
    }
    let samples = [
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index: 0, record: CsvRecord::default() }),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        CsvMutation::SetField(set_field::SetField { record_index: 0, field_index: 0, value: String::new(), quoted: false }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every CsvMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match CsvMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

/// ⚖️ `csv_mutation_inverse_sum_law`: for every leaf the inverse diffs sum to the negative forward diff.
#[semio_framework_async_macros::async_test]
async fn csv_mutation_inverse_sum_law_holds_for_every_leaf() {
    let base = base_snapshot();
    for mutation in [
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new", true)]) }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index: 3, record: record(&[("tail", false)]) }),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 2 }),
        CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
        CsvMutation::SetField(set_field::SetField { record_index: 2, field_index: 1, value: "y".into(), quoted: false }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
