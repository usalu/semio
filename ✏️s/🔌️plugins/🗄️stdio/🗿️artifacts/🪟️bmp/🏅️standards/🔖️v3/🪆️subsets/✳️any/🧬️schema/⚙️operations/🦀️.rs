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

pub fn inverse_bmp_mutation(mutation: &BmpMutation, base: &BmpSnapshot) -> Vec<BmpMutation> {
    protocol::Mutation::inverse(mutation, base)
}

/// 📥️ Decodes one leaf's wire payload — its `payload_value()` JSON, no aggregate tag, the form every committed
/// `{kind, params}` feature row carries — by semantic kind through the derive-generated `from_payload_value`.
pub fn decode_bmp_mutation_payload(kind: &str, params: &str) -> Result<BmpMutation, String> {
    <BmpMutation as protocol::Mutation<BmpSnapshot>>::from_payload_value(kind, pack::from_json_str(params).map_err(|error| error.to_string())?).map_err(|error| error.to_string())
}
//#endregion Operations
