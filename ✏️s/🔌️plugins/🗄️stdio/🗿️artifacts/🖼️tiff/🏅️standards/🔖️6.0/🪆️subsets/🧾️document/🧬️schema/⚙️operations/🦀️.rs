//! ⚙️ Shared application and inversion of TiffMutation.
use crate::schema::{diff::TiffDiff, mutations::TiffMutation};
use crate::TiffSnapshot;

//#region Operations
pub fn apply_tiff_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffMutation) -> protocol::MutationOutcome<TiffDiff> {
    let outcome = <TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

pub fn inverse_tiff_mutation(mutation: &TiffMutation, base: &TiffSnapshot) -> Vec<TiffMutation> {
    protocol::Mutation::inverse(mutation, base)
}

/// 📥️ Decodes one leaf's wire payload — its `payload_value()` JSON, no aggregate tag, the form every committed
/// `{kind, params}` feature row carries — by semantic kind through the derive-generated `from_payload_value`.
pub fn decode_tiff_mutation_payload(kind: &str, params: &str) -> Result<TiffMutation, String> {
    <TiffMutation as protocol::Mutation<TiffSnapshot>>::from_payload_value(kind, pack::from_json_str(params).map_err(|error| error.to_string())?).map_err(|error| error.to_string())
}
//#endregion Operations
