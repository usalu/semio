//! 🧬️ Transparent TxtMutation aggregate.
//#region 🔖️Aggregate
use crate::schema::diff::TxtDiff;
use crate::TxtSnapshot;

#[path = "📥️insert-line/🦀️.rs"]
pub mod insert_line;
#[path = "🗑️remove-line/🦀️.rs"]
pub mod remove_line;
#[path = "✏️set-line/🦀️.rs"]
pub mod set_line;
#[path = "🔚️set-line-ending/🦀️.rs"]
pub mod set_line_ending;
#[path = "↩️set-trailing-newline/🦀️.rs"]
pub mod set_trailing_newline;
#[path = "✂️splice-text/🦀️.rs"]
pub mod splice_text;

pub use self::insert_line::{InsertLineMutation, InsertLinePayload};
pub use self::remove_line::{RemoveLineMutation, RemoveLinePayload};
pub use self::set_line::{SetLineMutation, SetLinePayload};
pub use self::set_line_ending::{SetLineEndingMutation, SetLineEndingPayload};
pub use self::set_trailing_newline::{SetTrailingNewlineMutation, SetTrailingNewlinePayload};
pub use self::splice_text::{SpliceTextMutation, SpliceTextPayload, TextSplice};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = TxtSnapshot, diff = TxtDiff, schema = "s.stdio.txt")]
pub enum TxtMutation {
    SetTrailingNewline(SetTrailingNewlineMutation),
    SetLineEnding(SetLineEndingMutation),
    InsertLine(InsertLineMutation),
    RemoveLine(RemoveLineMutation),
    SetLine(SetLineMutation),
    SpliceText(SpliceTextMutation),
}
//#endregion 🔖️Aggregate



//#endregion ⚙️Application

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
