//! ⚙️ Shared application and inversion of JpgMutation.
use crate::schema::{diff::JpgDiff, mutations::JpgMutation};
use crate::JpgSnapshot;

//#region Operations
pub fn apply_jpg_mutation(snapshot: &mut JpgSnapshot, mutation: &JpgMutation) -> protocol::MutationOutcome<JpgDiff> {
    let outcome = <JpgMutation as protocol::Mutation<JpgSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion Operations
