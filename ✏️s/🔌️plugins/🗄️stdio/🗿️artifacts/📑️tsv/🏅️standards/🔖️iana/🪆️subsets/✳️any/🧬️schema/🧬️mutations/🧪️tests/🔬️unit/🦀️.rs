
use super::*;
use protocol::command::DiffAlgebra;

//#region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn row(fields: &[&str]) -> Vec<String> {
    fields.iter().map(|s| s.to_string()).collect()
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> TsvSnapshot {
    TsvSnapshot { records: vec![row(&["id", "name"]), row(&["1", "Oak"]), row(&["2", "Steel"])], trailing_newline: true, line_ending: LineEnding::Lf, ..TsvSnapshot::default() }
}
//#endregion 🔖️Fixtures

//#region 🔖️FieldSweepFixtures
/// 🧬️ Canonical "differs in every mutable field" snapshot A: 3 rows — one that will be
/// removed, one that will be modified in every column, one untouched.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> TsvSnapshot {
    TsvSnapshot { records: vec![row(&["gone", "also-gone"]), row(&["old-a", "old-b"]), row(&["stable", "x"])], trailing_newline: true, line_ending: LineEnding::Lf, ..TsvSnapshot::default() }
}
/// 🧬️ Sweep B: `trailing_newline`/`line_ending` flip, row 0 is removed, row 1 (now index 0)
/// is modified in every column, row 2 (now index 1) is untouched, and a brand-new row is added.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> TsvSnapshot {
    TsvSnapshot { records: vec![row(&["new-a", "new-b"]), row(&["stable", "x"]), row(&["brand-new", "y"])], trailing_newline: false, line_ending: LineEnding::Crlf, ..TsvSnapshot::default() }
}
//#endregion 🔖️FieldSweepFixtures

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    let variants = vec![
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: false }),
        TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: LineEnding::Crlf }),
        TsvMutation::InsertRow(insert_row::InsertRow { index: 1, row: row(&["new", "row"]) }),
        TsvMutation::RemoveRow(remove_row::RemoveRow { index: 0 }),
        TsvMutation::SetCell(set_cell::SetCell { row_index: 1, field_index: 0, value: "changed".into() }),
    ];
    for m in variants {
        let diff = m.diff(&base);
        let expected = diff.diff().apply(&base).unwrap();

        let mut via_apply = base.clone();
        let returned_diff = apply_tsv_mutation(&mut via_apply, &m);

        assert_eq!(via_apply, expected, "apply_tsv_mutation mismatch for {m:?}");
        assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = base_snapshot();
    let variants = vec![
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: false }),
        TsvMutation::InsertRow(insert_row::InsertRow { index: 1, row: row(&["new", "row"]) }),
        TsvMutation::RemoveRow(remove_row::RemoveRow { index: 0 }),
        TsvMutation::SetCell(set_cell::SetCell { row_index: 1, field_index: 0, value: "changed".into() }),
    ];
    for m in variants {
        let mut forward = base.clone();
        apply_tsv_mutation(&mut forward, &m);
        for inv in m.inverse(&base) {
            apply_tsv_mutation(&mut forward, &inv);
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

    let d1 = TsvMutation::InsertRow(insert_row::InsertRow { index: 2, row: row(&["ins", "x"]) }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = TsvMutation::RemoveRow(remove_row::RemoveRow { index: 0 }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Remove-before absorb mismatch");

    let d1 = TsvMutation::InsertRow(insert_row::InsertRow { index: 2, row: row(&["f", "x"]) }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = TsvMutation::InsertRow(insert_row::InsertRow { index: 2, row: row(&["g", "y"]) }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Insert-same-index absorb mismatch");
    assert_eq!(after.records.len(), base.records.len() + 2, "both inserts must survive");

    let d1 = TsvMutation::InsertRow(insert_row::InsertRow { index: 1, row: row(&["orig", "x"]) }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = TsvMutation::SetCell(set_cell::SetCell { row_index: 1, field_index: 0, value: "patched".into() }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Add+SetCell absorb mismatch");
    assert_eq!(after.records[1][0], "patched");

    let d1 = TsvMutation::SetCell(set_cell::SetCell { row_index: 1, field_index: 0, value: "will-vanish".into() }).diff(&base);
    let mid = d1.diff().apply(&base).unwrap();
    let d2 = TsvMutation::RemoveRow(remove_row::RemoveRow { index: 1 }).diff(&mid);
    let after = d2.diff().apply(&mid).unwrap();
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).unwrap(), after, "Modify+Remove absorb mismatch");

    let base = base_snapshot();
    let d1 = TsvMutation::InsertRow(insert_row::InsertRow { index: 0, row: row(&["a", "x"]) }).diff(&base);
    let s1 = d1.diff().apply(&base).unwrap();
    let d2 = TsvMutation::SetCell(set_cell::SetCell { row_index: 0, field_index: 0, value: "a2".into() }).diff(&s1);
    let s2 = d2.diff().apply(&s1).unwrap();
    let d3 = TsvMutation::RemoveRow(remove_row::RemoveRow { index: 2 }).diff(&s2);
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
    assert_eq!(TsvDiff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(TsvDiff::between(&b, &a).apply(&b).unwrap(), a);

    let mut c = a.clone();
    c.records[0] = row(&["only-one-field"]);
    assert_eq!(TsvDiff::between(&a, &c).apply(&a).unwrap(), c);
    assert_eq!(TsvDiff::between(&c, &a).apply(&c).unwrap(), a);

    assert!(TsvDiff::between(&a, &a).is_empty());
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️FieldSweep
#[semio_framework_async_macros::async_test]
async fn field_sweep_every_mutable_field_changes() {
    let a = sweep_a();
    let b = sweep_b();

    let d_ab = TsvDiff::between(&a, &b);
    assert_eq!(d_ab.apply(&a).unwrap(), b, "between(a,b).apply(a) == b");

    let d_ba = TsvDiff::between(&b, &a);
    assert_eq!(d_ba.apply(&b).unwrap(), a, "between(b,a).apply(b) == a");

    assert!(d_ab.trailing_newline.is_some(), "trailing_newline must be populated");
    assert!(d_ab.line_ending.is_some(), "line_ending must be populated");
    // 🧭️ `TsvDiff::between` is positional (rows have no stable identity beyond position, same
    // as epw's own `EpwDiff::between`) — `min_len` only compares shared index range, so a
    // single `between()` call populates `removed` XOR `added` (whichever side is longer),
    // never both, UNLESS a shared index's row width itself changes (that path emits a
    // matched removed+added pair at the SAME index, see the `b.len() != o.len()` branch
    // above). `sweep_a`/`sweep_b` rows are all 2 columns wide, so every index is a same-width
    // positional comparison: `modified` is what's populated here; `removed`-only/`added`-only
    // are exercised on their own just below via genuinely shorter/longer row lists.
    let records = d_ab.records.as_ref().expect("records diff must be populated");
    assert!(records.removed.is_empty(), "equal-length, equal-width row lists: no positional removal");
    assert!(!records.modified.is_empty(), "modified must be non-empty (every row differs positionally)");
    assert!(records.added.is_empty(), "equal-length, equal-width row lists: no positional addition");
    assert_eq!(records.modified.len(), 3, "all three positions differ between sweep_a and sweep_b");
    let modified = &records.modified[0];
    let field_patches = modified.diff.fields.as_ref().expect("row 1's field patch list must be populated");
    assert!(field_patches.iter().all(|f| f.is_some()), "every column of the modified row must be patched");

    let mut shorter = a.clone();
    shorter.records.pop();
    let d_shrink = TsvDiff::between(&a, &shorter);
    let shrink_records = d_shrink.records.as_ref().expect("records diff must be populated");
    assert!(!shrink_records.removed.is_empty(), "a shorter row list must produce a removed entry");
    assert_eq!(d_shrink.apply(&a).unwrap(), shorter);

    let mut longer = a.clone();
    longer.records.push(row(&["extra", "z"]));
    let d_grow = TsvDiff::between(&a, &longer);
    let grow_records = d_grow.records.as_ref().expect("records diff must be populated");
    assert!(!grow_records.added.is_empty(), "a longer row list must produce an added entry");
    assert_eq!(d_grow.apply(&a).unwrap(), longer);

    assert!(TsvDiff::between(&a, &a).is_empty());
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let mutations = vec![
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: TsvSnapshot { records: vec![row(&["a, tricky [value]", "plain"])], trailing_newline: false, line_ending: LineEnding::Crlf, ..TsvSnapshot::default() } }),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: true }),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: false }),
        TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: LineEnding::Crlf }),
        TsvMutation::InsertRow(insert_row::InsertRow { index: 1, row: row(&["new, [tricky]"]) }),
        TsvMutation::RemoveRow(remove_row::RemoveRow { index: 0 }),
        TsvMutation::SetCell(set_cell::SetCell { row_index: 1, field_index: 0, value: "changed".into() }),
        TsvMutation::SetCell(set_cell::SetCell { row_index: 0, field_index: 2, value: "with, comma [and] brackets".into() }),
    ];
    for m in mutations {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = TsvMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = TsvMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `TsvMutation` without a matching kebab-case spelling here, which is what
/// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
/// `kinds` array as text (the framework never parses Rust, so this is the only side that can
/// prove the manifest matches) and asserts the same list, in the same order.
#[semio_framework_async_macros::async_test]
async fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &TsvMutation) -> &'static str {
        match mutation {
            TsvMutation::SetSnapshot(_) => "set-snapshot",
            TsvMutation::SetTrailingNewline(_) => "set-trailing-newline",
            TsvMutation::SetLineEnding(_) => "set-line-ending",
            TsvMutation::InsertRow(_) => "insert-row",
            TsvMutation::RemoveRow(_) => "remove-row",
            TsvMutation::SetCell(_) => "set-cell",
        }
    }
    let samples = [
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: TsvSnapshot::default() }),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: false }),
        TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: LineEnding::Crlf }),
        TsvMutation::InsertRow(insert_row::InsertRow { index: 0, row: Vec::new() }),
        TsvMutation::RemoveRow(remove_row::RemoveRow { index: 0 }),
        TsvMutation::SetCell(set_cell::SetCell { row_index: 0, field_index: 0, value: String::new() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every TsvMutation variant, in declaration order");
}
//#endregion 🔖️KindsConformanceLaw
