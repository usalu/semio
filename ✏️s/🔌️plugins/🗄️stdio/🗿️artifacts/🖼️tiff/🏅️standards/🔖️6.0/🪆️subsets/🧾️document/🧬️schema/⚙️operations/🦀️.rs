//! ⚙️ Shared application and inversion of TiffMutation.
use crate::schema::{diff::TiffDiff, mutations::TiffMutation};
use crate::TiffSnapshot;

//#region Operations
pub fn apply_tiff_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffMutation) -> protocol::MutationOutcome<TiffDiff> {
    let outcome = <TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion Operations
