//! 🔺️ Sparse diff builder for `UpdateDenseParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). Identical resubmission ⇒ Warning; non-finite
//! confidence threshold ⇒ Fatal.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateDenseParams, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.params == base.params.dense {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Dense params are already up to date.".to_string());
    }
    if !payload.params.confidence_threshold.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Dense params have a non-finite confidence threshold.".to_string(), [base.id.clone()]);
    }
    let mut params = base.params.clone();
    params.dense = payload.params.clone();
    protocol::MutationOutcome::new(RemodelDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
