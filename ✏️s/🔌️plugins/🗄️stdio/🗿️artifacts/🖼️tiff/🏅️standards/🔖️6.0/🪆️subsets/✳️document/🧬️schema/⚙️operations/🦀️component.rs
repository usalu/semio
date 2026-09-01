//! ⚙️ Shared application and inversion of TiffMutation.
use crate::artifacts::tiff::schema::{diff::TiffDiff, mutations::TiffMutation};
use crate::artifacts::tiff::TiffSnapshot;

//#region Operations
pub fn apply_tiff_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffMutation) -> protocol::MutationOutcome<TiffDiff> {
    let outcome = <TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

pub fn inverse_tiff_mutation(mutation: &TiffMutation, base: &TiffSnapshot) -> Vec<TiffMutation> {
    protocol::Mutation::inverse(mutation, base)
}
//#endregion Operations
