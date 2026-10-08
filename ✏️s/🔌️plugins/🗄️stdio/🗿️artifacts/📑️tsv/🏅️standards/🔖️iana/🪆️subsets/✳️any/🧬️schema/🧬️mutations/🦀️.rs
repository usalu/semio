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



//#endregion 🔖️Apply


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
