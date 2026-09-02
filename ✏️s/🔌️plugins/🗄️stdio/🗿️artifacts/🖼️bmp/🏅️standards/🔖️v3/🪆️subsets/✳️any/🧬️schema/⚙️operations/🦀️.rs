//! ⚙️ Shared application and inversion of BmpMutation.
use crate::artifacts::bmp::schema::{diff::BmpDiff, mutations::BmpMutation};
use crate::artifacts::bmp::BmpSnapshot;

//#region Operations
pub fn apply_bmp_mutation(snapshot: &mut BmpSnapshot, mutation: &BmpMutation) -> protocol::MutationOutcome<BmpDiff> {
    let outcome = <BmpMutation as protocol::Mutation<BmpSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

pub fn inverse_bmp_mutation(mutation: &BmpMutation, base: &BmpSnapshot) -> Vec<BmpMutation> {
    protocol::Mutation::inverse(mutation, base)
}
//#endregion Operations
