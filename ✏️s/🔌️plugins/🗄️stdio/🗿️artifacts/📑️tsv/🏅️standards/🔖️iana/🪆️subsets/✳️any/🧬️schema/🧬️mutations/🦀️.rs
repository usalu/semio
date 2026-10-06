//! 🧬️ TsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `TsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::standards::iana::subsets::any::schema::diff::{diff_set_snapshot, TsvDiff, TsvRowAdded, TsvRowDiff, TsvRowModified, TsvRowsDiff};






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
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
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
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
/// mutation catalog `tsv-iana-any` (`../../🔮️oracles/🔣️.json`) is measured against this
/// exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "set-trailing-newline", "set-line-ending", "insert-row", "remove-row", "set-cell"];
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
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &TsvMutation, base: &TsvSnapshot) -> protocol::MutationOutcome<TsvDiff> {
    protocol::MutationOutcome::new(match this {
        TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        TsvMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<TsvSnapshot, TsvMutation>>::diff(patch, base),
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
pub(crate) fn agg_inverse(this: &TsvMutation, base: &TsvSnapshot) -> Result<Vec<TsvMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        TsvMutation::SetSnapshot(_) => vec![TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        TsvMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<TsvSnapshot, TsvMutation>>::inverse(patch, base)?),
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

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs











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
