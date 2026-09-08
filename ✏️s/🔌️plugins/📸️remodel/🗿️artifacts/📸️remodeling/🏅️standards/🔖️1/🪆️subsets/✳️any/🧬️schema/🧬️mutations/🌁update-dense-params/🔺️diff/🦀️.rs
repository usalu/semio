//! 🔺️ Sparse diff builder for `UpdateDenseParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). The invariant is checked BEFORE the
//! identical-resubmission warning, the one guard order the whole vocabulary follows — non-finite
//! confidence threshold ⇒ Fatal.
use crate::diff::RemodelingDiff;
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateDenseParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.confidence_threshold.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Dense params have a non-finite confidence threshold.".to_string(), [base.id.clone()]);
    }
    if payload.params == base.params.dense {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Dense params are already up to date.".to_string());
    }
    let mut params = base.params.clone();
    params.dense = payload.params.clone();
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
