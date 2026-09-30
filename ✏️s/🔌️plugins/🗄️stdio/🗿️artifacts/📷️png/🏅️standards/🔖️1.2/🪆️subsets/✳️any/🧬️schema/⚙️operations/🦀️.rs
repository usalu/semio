//! ⚙️ Shared application and inversion of PngMutation.
use crate::schema::{diff::PngDiff, mutations::PngMutation};
use crate::PngSnapshot;

//#region Operations
pub fn apply_png_mutation(snapshot: &mut PngSnapshot, mutation: &PngMutation) -> protocol::MutationOutcome<PngDiff> {
    let outcome = <PngMutation as protocol::Mutation<PngSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

pub fn inverse_png_mutation(mutation: &PngMutation, base: &PngSnapshot) -> Vec<PngMutation> {
    protocol::Mutation::inverse(mutation, base)
}

/// 📥️ Decodes one leaf's wire payload — its `payload_value()` JSON, no aggregate tag, the form every committed
/// `{kind, params}` feature row carries — by semantic kind through the derive-generated `from_payload_value`.
pub fn decode_png_mutation_payload(kind: &str, params: &str) -> Result<PngMutation, String> {
    <PngMutation as protocol::Mutation<PngSnapshot>>::from_payload_value(kind, pack::from_json_str(params).map_err(|error| error.to_string())?).map_err(|error| error.to_string())
}
//#endregion Operations
