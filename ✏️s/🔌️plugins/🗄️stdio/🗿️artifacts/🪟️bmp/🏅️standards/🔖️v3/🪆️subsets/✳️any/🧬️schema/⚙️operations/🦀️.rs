//! ⚙️ Shared application and inversion of BmpMutation.
use crate::schema::{diff::BmpDiff, mutations::BmpMutation};
use crate::BmpSnapshot;

//#region Operations
pub fn apply_bmp_mutation(snapshot: &mut BmpSnapshot, mutation: &BmpMutation) -> protocol::MutationOutcome<BmpDiff> {
    let outcome = <BmpMutation as protocol::Mutation<BmpSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion Operations
