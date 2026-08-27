//! 🧬️ Transparent TxtMutation aggregate.
//#region 🔖️Aggregate
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::diff::TxtDiff;
use serde::{Deserialize, Serialize};

pub use super::insert_line::{InsertLineMutation, InsertLinePayload};
pub use super::remove_line::{RemoveLineMutation, RemoveLinePayload};
pub use super::set_line::{SetLineMutation, SetLinePayload};
pub use super::set_line_ending::{SetLineEndingMutation, SetLineEndingPayload};
pub use super::set_trailing_newline::{SetTrailingNewlineMutation, SetTrailingNewlinePayload};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = TxtSnapshot, diff = TxtDiff, schema = "s.stdio.txt")]
pub enum TxtMutation {
    SetTrailingNewline(SetTrailingNewlineMutation),
    SetLineEnding(SetLineEndingMutation),
    InsertLine(InsertLineMutation),
    RemoveLine(RemoveLineMutation),
    SetLine(SetLineMutation),
}
//#endregion 🔖️Aggregate

//#region ⚙️Application
pub fn apply_txt_mutation(snapshot: &mut TxtSnapshot, mutation: &TxtMutation) -> protocol::MutationOutcome<TxtDiff> {
    let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}
//#endregion ⚙️Application

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;

    #[test]
    fn aggregate_roster_is_exact() {
        assert_eq!(TxtMutation::kinds().len(), 5);
    }
}
//#endregion 🧪️Tests
