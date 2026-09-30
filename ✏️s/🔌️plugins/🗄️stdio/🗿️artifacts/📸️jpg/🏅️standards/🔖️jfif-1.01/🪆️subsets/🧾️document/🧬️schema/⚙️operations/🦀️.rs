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

pub fn inverse_jpg_mutation(mutation: &JpgMutation, base: &JpgSnapshot) -> Vec<JpgMutation> {
    protocol::Mutation::inverse(mutation, base)
}

/// 📥️ Decodes one leaf's wire payload — its `payload_value()` JSON, no aggregate tag, the form every committed
/// `{kind, params}` feature row carries — by semantic kind through the derive-generated `from_payload_value`.
pub fn decode_jpg_mutation_payload(kind: &str, params: &str) -> Result<JpgMutation, String> {
    <JpgMutation as protocol::Mutation<JpgSnapshot>>::from_payload_value(kind, pack::from_json_str(params).map_err(|error| error.to_string())?).map_err(|error| error.to_string())
}
//#endregion Operations
