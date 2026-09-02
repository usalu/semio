//! ⚙️ Shared application and inversion of PngMutation.
use crate::artifacts::png::schema::{diff::PngDiff, mutations::PngMutation};
use crate::artifacts::png::PngSnapshot;

//#region Operations
pub fn apply_png_mutation(snapshot: &mut PngSnapshot, mutation: &PngMutation) -> protocol::MutationOutcome<PngDiff> {
    let outcome = <PngMutation as protocol::Mutation<PngSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

pub fn inverse_png_mutation(mutation: &PngMutation, base: &PngSnapshot) -> Vec<PngMutation> {
    protocol::Mutation::inverse(mutation, base)
}
//#endregion Operations
