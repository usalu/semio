//! 🧬️ TsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `TsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::{dec_row, dec_str, diff_set_snapshot, enc_row, enc_str, split_top_level, strip_brackets, TsvDiff, TsvRowAdded, TsvRowDiff, TsvRowModified, TsvRowsDiff};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.tsv`.
/// 🧪️ F6: hand-rolled — `#[derive(dsl::DslOps)]` is not attempted (`InsertRow`'s `row: Vec<String>`
/// field would hit the derive's own `DslField for Vec<T>` blanket-impl requirements the same way
/// csv's/gif89a's hand-rolled paths document; hand-rolling below reuses `TsvDiff`'s
/// `pub(crate)` grammar primitives instead).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TsvMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: TsvSnapshot,
    },
    /// ↩️ Toggles whether the encoded text ends with a line terminator.
    SetTrailingNewline {
        trailing_newline: bool,
    },
    /// ↩️ Replaces the file's line-ending convention.
    SetLineEnding {
        line_ending: LineEnding,
    },
    /// ➕️ Inserts a whole row at `index` (clamped to the end on apply).
    InsertRow {
        index: usize,
        row: Vec<String>,
    },
    /// ➖️ Removes the row at `index`.
    RemoveRow {
        index: usize,
    },
    /// ✏️ Patches one cell's value in place.
    SetCell {
        row_index: usize,
        field_index: usize,
        value: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_tsv_mutation(snapshot: &mut TsvSnapshot, mutation: &TsvMutation) -> protocol::MutationOutcome<TsvDiff> {
    let outcome = <TsvMutation as Mutation<TsvSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<TsvSnapshot> for TsvMutation {
    type Diff = TsvDiff;

    async fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            TsvMutation::NoMutation => TsvDiff::default(),
            TsvMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            TsvMutation::SetTrailingNewline { trailing_newline } => TsvDiff { trailing_newline: Some(*trailing_newline), ..TsvDiff::default() },
            TsvMutation::SetLineEnding { line_ending } => TsvDiff { line_ending: Some(*line_ending), ..TsvDiff::default() },
            TsvMutation::InsertRow { index, row } => TsvDiff { records: Some(TsvRowsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![TsvRowAdded { index: *index, row: row.clone() }] }), ..TsvDiff::default() },
            TsvMutation::RemoveRow { index } => TsvDiff { records: Some(TsvRowsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }), ..TsvDiff::default() },
            TsvMutation::SetCell { row_index, field_index, value } => {
                let mut fields = vec![None; field_index + 1];
                fields[*field_index] = Some(value.clone());
                TsvDiff { records: Some(TsvRowsDiff { removed: Vec::new(), modified: vec![TsvRowModified { index: *row_index, diff: TsvRowDiff { fields: Some(fields) } }], added: Vec::new() }), ..TsvDiff::default() }
            }
        }).await
    }

    async fn inverse(&self, base: &TsvSnapshot) -> Vec<Self> {
        match self {
            TsvMutation::NoMutation => vec![TsvMutation::NoMutation],
            TsvMutation::SetSnapshot { .. } => vec![TsvMutation::SetSnapshot { snapshot: base.clone() }],
            TsvMutation::SetTrailingNewline { .. } => vec![TsvMutation::SetTrailingNewline { trailing_newline: base.trailing_newline }],
            TsvMutation::SetLineEnding { .. } => vec![TsvMutation::SetLineEnding { line_ending: base.line_ending }],
            TsvMutation::InsertRow { index, .. } => vec![TsvMutation::RemoveRow { index: *index }],
            TsvMutation::RemoveRow { index } => match base.records.get(*index) {
                Some(row) => vec![TsvMutation::InsertRow { index: *index, row: row.clone() }],
                None => vec![TsvMutation::NoMutation],
            },
            TsvMutation::SetCell { row_index, field_index, .. } => match base.records.get(*row_index).and_then(|r| r.get(*field_index)) {
                Some(cell) => vec![TsvMutation::SetCell { row_index: *row_index, field_index: *field_index, value: cell.clone() }],
                None => vec![TsvMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `TsvMutation` — reuses `TsvDiff`'s `pub(crate)`
/// grammar primitives. Grammar: `keyword arg=value ...` (space-separated), same convention csv's/
/// gif89a's/svg's own hand-rolled `OpText` impls use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_tsv_snapshot(s: &TsvSnapshot) -> String {
    format!(
        "[{},{},{},[{}]]",
        enc_str(&s.schema),
        if s.trailing_newline { 1 } else { 0 },
        crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::enc_line_ending(s.line_ending),
        s.records.iter().map(|r| enc_row(r)).collect::<Vec<_>>().join(","),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_tsv_snapshot(s: &str) -> Result<TsvSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, trailing_newline, line_ending, records] = parts.as_slice() else {
        return Err(format!("tsv snapshot: expected 4 fields, got {}", parts.len()));
    };
    let records = split_top_level(strip_brackets(records)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_row).collect::<Result<Vec<_>, String>>()?;
    Ok(TsvSnapshot { schema: dec_str(schema)?, trailing_newline: *trailing_newline == "1", line_ending: crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::dec_line_ending(line_ending)?, records })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_tsv_mutation(m: &TsvMutation) -> String {
    match m {
        TsvMutation::NoMutation => "no-mutation".to_string(),
        TsvMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_tsv_snapshot(snapshot)),
        TsvMutation::SetTrailingNewline { trailing_newline } => format!("set-trailing-newline trailing-newline={}", if *trailing_newline { 1 } else { 0 }),
        TsvMutation::SetLineEnding { line_ending } => format!("set-line-ending line-ending={}", crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::enc_line_ending(*line_ending)),
        TsvMutation::InsertRow { index, row } => format!("insert-row index={index} row={}", enc_row(row)),
        TsvMutation::RemoveRow { index } => format!("remove-row index={index}"),
        TsvMutation::SetCell { row_index, field_index, value } => format!("set-cell row-index={row_index} field-index={field_index} value={}", enc_str(value),),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_tsv_mutation(line: &str) -> Result<TsvMutation, String> {
    if line == "no-mutation" {
        return Ok(TsvMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("tsv mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("tsv mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(TsvMutation::SetSnapshot { snapshot: dec_tsv_snapshot(arg("snapshot")?)? }),
        "set-trailing-newline" => Ok(TsvMutation::SetTrailingNewline { trailing_newline: arg("trailing-newline")? == "1" }),
        "set-line-ending" => Ok(TsvMutation::SetLineEnding { line_ending: crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::dec_line_ending(arg("line-ending")?)? }),
        "insert-row" => Ok(TsvMutation::InsertRow { index: usize_arg("index")?, row: dec_row(arg("row")?)? }),
        "remove-row" => Ok(TsvMutation::RemoveRow { index: usize_arg("index")? }),
        "set-cell" => Ok(TsvMutation::SetCell { row_index: usize_arg("row-index")?, field_index: usize_arg("field-index")?, value: dec_str(arg("value")?)? }),
        other => Err(format!("tsv mutation: unknown keyword {other:?}")),
    }
}

impl OpText for TsvMutation {
    async fn print_op(&self) -> String {
        print_tsv_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_tsv_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification as `TsvDiff`'s hand-rolled codec.
impl OpBinary for TsvMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().await.into_bytes())
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).await.map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
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
            TsvMutation::NoMutation,
            TsvMutation::SetSnapshot { snapshot: sweep_b() },
            TsvMutation::SetTrailingNewline { trailing_newline: false },
            TsvMutation::SetLineEnding { line_ending: LineEnding::Crlf },
            TsvMutation::InsertRow { index: 1, row: row(&["new", "row"]) },
            TsvMutation::RemoveRow { index: 0 },
            TsvMutation::SetCell { row_index: 1, field_index: 0, value: "changed".into() },
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
            TsvMutation::NoMutation,
            TsvMutation::SetSnapshot { snapshot: sweep_b() },
            TsvMutation::SetTrailingNewline { trailing_newline: false },
            TsvMutation::InsertRow { index: 1, row: row(&["new", "row"]) },
            TsvMutation::RemoveRow { index: 0 },
            TsvMutation::SetCell { row_index: 1, field_index: 0, value: "changed".into() },
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

        let d1 = TsvMutation::InsertRow { index: 2, row: row(&["ins", "x"]) }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = TsvMutation::RemoveRow { index: 0 }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Remove-before absorb mismatch");

        let d1 = TsvMutation::InsertRow { index: 2, row: row(&["f", "x"]) }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = TsvMutation::InsertRow { index: 2, row: row(&["g", "y"]) }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.records.len(), base.records.len() + 2, "both inserts must survive");

        let d1 = TsvMutation::InsertRow { index: 1, row: row(&["orig", "x"]) }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = TsvMutation::SetCell { row_index: 1, field_index: 0, value: "patched".into() }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Add+SetCell absorb mismatch");
        assert_eq!(after.records[1][0], "patched");

        let d1 = TsvMutation::SetCell { row_index: 1, field_index: 0, value: "will-vanish".into() }.diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = TsvMutation::RemoveRow { index: 1 }.diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Modify+Remove absorb mismatch");

        let base = base_snapshot();
        let d1 = TsvMutation::InsertRow { index: 0, row: row(&["a", "x"]) }.diff(&base);
        let s1 = d1.diff().apply(&base).unwrap();
        let d2 = TsvMutation::SetCell { row_index: 0, field_index: 0, value: "a2".into() }.diff(&s1);
        let s2 = d2.diff().apply(&s1).unwrap();
        let d3 = TsvMutation::RemoveRow { index: 2 }.diff(&s2);
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
            TsvMutation::NoMutation,
            TsvMutation::SetSnapshot { snapshot: sweep_b() },
            TsvMutation::SetSnapshot { snapshot: TsvSnapshot { records: vec![row(&["a, tricky [value]", "plain"])], trailing_newline: false, line_ending: LineEnding::Crlf, ..TsvSnapshot::default() } },
            TsvMutation::SetTrailingNewline { trailing_newline: true },
            TsvMutation::SetTrailingNewline { trailing_newline: false },
            TsvMutation::SetLineEnding { line_ending: LineEnding::Crlf },
            TsvMutation::InsertRow { index: 1, row: row(&["new, [tricky]"]) },
            TsvMutation::RemoveRow { index: 0 },
            TsvMutation::SetCell { row_index: 1, field_index: 0, value: "changed".into() },
            TsvMutation::SetCell { row_index: 0, field_index: 2, value: "with, comma [and] brackets".into() },
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
}
//#endregion 🧪️Tests
