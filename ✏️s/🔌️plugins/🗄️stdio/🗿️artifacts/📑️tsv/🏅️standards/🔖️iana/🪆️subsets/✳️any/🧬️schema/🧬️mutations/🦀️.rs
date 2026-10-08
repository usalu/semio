//! 🧬️ TsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `TsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::standards::iana::subsets::any::schema::diff::{TsvDiff, TsvRowAdded, TsvRowDiff, TsvRowModified, TsvRowsDiff};






use crate::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};

use protocol::{Mutation, MutationDiff};

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
#[path = "🔚set-trailing-newline/🦀️.rs"]
pub mod set_trailing_newline;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = TsvSnapshot, diff = TsvDiff, schema = "TsvMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum TsvMutation {
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
pub const KINDS: &[&str] = &["set-trailing-newline", "set-line-ending", "insert-row", "remove-row", "set-cell"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_tsv_mutation(snapshot: &mut TsvSnapshot, mutation: &TsvMutation) -> protocol::MutationOutcome<TsvDiff> {
    let outcome = <TsvMutation as Mutation<TsvSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the trailing-newline flag and the line ending if they moved, then every row
/// in place (one `set-cell` per differing cell; a row whose column count changed is removed and inserted anew) and the diverging
/// tail (surplus rows removed last first, missing rows inserted).
pub fn net_mutations(base: &TsvSnapshot, next: &TsvSnapshot) -> Vec<TsvMutation> {
    let mut leaves = Vec::new();
    if base.trailing_newline != next.trailing_newline {
        leaves.push(TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: next.trailing_newline }));
    }
    if base.line_ending != next.line_ending {
        leaves.push(TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: next.line_ending }));
    }
    let paired = base.records.len().min(next.records.len());
    for (row_index, (before, after)) in base.records.iter().zip(&next.records).enumerate().filter(|(_, (before, after))| before != after) {
        if before.len() == after.len() {
            leaves.extend(before.iter().zip(after).enumerate().filter(|(_, (old, new))| old != new).map(|(field_index, (_, new))| TsvMutation::SetCell(set_cell::SetCell { row_index, field_index, value: new.clone() })));
        } else {
            leaves.push(TsvMutation::RemoveRow(remove_row::RemoveRow { index: row_index }));
            leaves.push(TsvMutation::InsertRow(insert_row::InsertRow { index: row_index, row: after.clone() }));
        }
    }
    leaves.extend((paired..base.records.len()).rev().map(|index| TsvMutation::RemoveRow(remove_row::RemoveRow { index })));
    leaves.extend(next.records.iter().enumerate().skip(paired).map(|(index, row)| TsvMutation::InsertRow(insert_row::InsertRow { index, row: row.clone() })));
    leaves
}
//#endregion 🔖️Net

//#endregion 🔖️MutationTrait

//#region OpCodecs











//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests



#[cfg(test)]
use protocol::{OpBinary,OpText};
