//! 🧬️ TsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `TsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::standards::iana::subsets::any::schema::diff::{dec_row, dec_str, diff_set_snapshot, enc_row, enc_str, split_top_level, strip_brackets, TsvDiff, TsvRowAdded, TsvRowDiff, TsvRowModified, TsvRowsDiff};
use crate::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

//#region 🔖️Mutations
#[path = "➕insert-row/🦀️.rs"]
pub mod insert_row;
#[path = "➖remove-row/🦀️.rs"]
pub mod remove_row;
#[path = "🔲set-cell/🦀️.rs"]
pub mod set_cell;
#[path = "🔀set-line-ending/🦀️.rs"]
pub mod set_line_ending;
/// 📐️ Typed content mutation for `stdio.tsv`.
/// 🧪️ F6: hand-rolled — `#[derive(dsl::DslOps)]` is not attempted (`InsertRow`'s `row: Vec<String>`
/// field would hit the derive's own `DslField for Vec<T>` blanket-impl requirements the same way
/// csv's/gif89a's hand-rolled paths document; hand-rolling below reuses `TsvDiff`'s
/// `pub(crate)` grammar primitives instead).
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔚set-trailing-newline/🦀️.rs"]
pub mod set_trailing_newline;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = TsvSnapshot, diff = TsvDiff, schema = "TsvMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum TsvMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ↩️ Toggles whether the encoded text ends with a line terminator.
    SetTrailingNewline(set_trailing_newline::SetTrailingNewline),
    /// ↩️ Replaces the file's line-ending convention.
    SetLineEnding(set_line_ending::SetLineEnding),
    /// ➕️ Inserts a whole row at `index` (clamped to the end on apply).
    InsertRow(insert_row::InsertRow),
    /// ➖️ Removes the row at `index`.
    RemoveRow(remove_row::RemoveRow),
    /// ✏️ Patches one cell's value in place.
    SetCell(set_cell::SetCell),
}

/// 🧾️ Kebab-case spelling of every `TsvMutation` variant, in declaration order — the exhaustive
/// mutation catalog `tsv-iana-any` (`../../🔮️oracle/🔣️.json`) is measured against this
/// exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "set-trailing-newline", "set-line-ending", "insert-row", "remove-row", "set-cell"];
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
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &TsvMutation, base: &TsvSnapshot) -> protocol::MutationOutcome<TsvDiff> {
    protocol::MutationOutcome::new(match this {
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline }) => TsvDiff { trailing_newline: Some(*trailing_newline), ..TsvDiff::default() },
        TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending }) => TsvDiff { line_ending: Some(*line_ending), ..TsvDiff::default() },
        TsvMutation::InsertRow(insert_row::InsertRow { index, row }) => TsvDiff { records: Some(TsvRowsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![TsvRowAdded { index: *index, row: row.clone() }] }), ..TsvDiff::default() },
        TsvMutation::RemoveRow(remove_row::RemoveRow { index }) => TsvDiff { records: Some(TsvRowsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }), ..TsvDiff::default() },
        TsvMutation::SetCell(set_cell::SetCell { row_index, field_index, value }) => {
            let mut fields = vec![None; field_index + 1];
            fields[*field_index] = Some(value.clone());
            TsvDiff { records: Some(TsvRowsDiff { removed: Vec::new(), modified: vec![TsvRowModified { index: *row_index, diff: TsvRowDiff { fields: Some(fields) } }], added: Vec::new() }), ..TsvDiff::default() }
        }
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &TsvMutation, base: &TsvSnapshot) -> Vec<TsvMutation> {
    match this {
        TsvMutation::SetSnapshot(_) => vec![TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        TsvMutation::SetTrailingNewline(_) => vec![TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: base.trailing_newline })],
        TsvMutation::SetLineEnding(_) => vec![TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: base.line_ending })],
        TsvMutation::InsertRow(insert_row::InsertRow { index, .. }) => vec![TsvMutation::RemoveRow(remove_row::RemoveRow { index: *index })],
        TsvMutation::RemoveRow(remove_row::RemoveRow { index }) => match base.records.get(*index) {
            Some(row) => vec![TsvMutation::InsertRow(insert_row::InsertRow { index: *index, row: row.clone() })],
            None => Vec::new(),
        },
        TsvMutation::SetCell(set_cell::SetCell { row_index, field_index, .. }) => match base.records.get(*row_index).and_then(|r| r.get(*field_index)) {
            Some(cell) => vec![TsvMutation::SetCell(set_cell::SetCell { row_index: *row_index, field_index: *field_index, value: cell.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `TsvMutation` — reuses `TsvDiff`'s `pub(crate)`
/// grammar primitives. Grammar: `keyword arg=value ...` (space-separated), same convention csv's/
/// gif89a's/svg's own hand-rolled `OpText` impls use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_tsv_snapshot(s: &TsvSnapshot) -> String {
    format!("[{},{},{},[{}]]", enc_str(&s.schema), if s.trailing_newline { 1 } else { 0 }, crate::standards::iana::subsets::any::schema::diff::enc_line_ending(s.line_ending), s.records.iter().map(|r| enc_row(r)).collect::<Vec<_>>().join(","),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_tsv_snapshot(s: &str) -> Result<TsvSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, trailing_newline, line_ending, records] = parts.as_slice() else {
        return Err(format!("tsv snapshot: expected 4 fields, got {}", parts.len()));
    };
    let records = split_top_level(strip_brackets(records)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_row).collect::<Result<Vec<_>, String>>()?;
    Ok(TsvSnapshot { schema: dec_str(schema)?, trailing_newline: *trailing_newline == "1", line_ending: crate::standards::iana::subsets::any::schema::diff::dec_line_ending(line_ending)?, records })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_tsv_mutation(m: &TsvMutation) -> String {
    match m {
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_tsv_snapshot(snapshot)),
        TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline }) => format!("set-trailing-newline trailing-newline={}", if *trailing_newline { 1 } else { 0 }),
        TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending }) => format!("set-line-ending line-ending={}", crate::standards::iana::subsets::any::schema::diff::enc_line_ending(*line_ending)),
        TsvMutation::InsertRow(insert_row::InsertRow { index, row }) => format!("insert-row index={index} row={}", enc_row(row)),
        TsvMutation::RemoveRow(remove_row::RemoveRow { index }) => format!("remove-row index={index}"),
        TsvMutation::SetCell(set_cell::SetCell { row_index, field_index, value }) => format!("set-cell row-index={row_index} field-index={field_index} value={}", enc_str(value),),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_tsv_mutation(line: &str) -> Result<TsvMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("tsv mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("tsv mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_tsv_snapshot(arg("snapshot")?)? })),
        "set-trailing-newline" => Ok(TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: arg("trailing-newline")? == "1" })),
        "set-line-ending" => Ok(TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: crate::standards::iana::subsets::any::schema::diff::dec_line_ending(arg("line-ending")?)? })),
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_tsv_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification as `TsvDiff`'s hand-rolled codec.
impl OpBinary for TsvMutation {
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
