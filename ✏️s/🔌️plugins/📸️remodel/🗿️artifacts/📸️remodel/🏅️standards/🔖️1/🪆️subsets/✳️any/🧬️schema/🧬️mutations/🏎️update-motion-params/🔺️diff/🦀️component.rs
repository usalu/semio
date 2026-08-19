//! 🔺️ Sparse diff builder for `UpdateMotionParams` — always present, no target-missing check
//! possible (a struct field, not an id-keyed collection). Identical resubmission ⇒ Warning;
//! non-finite track quality ⇒ Fatal.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::UpdateMotionParams, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.params == base.params.motion {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Motion params are already up to date.".to_string());
    }
    if !payload.params.min_track_quality.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Motion params have a non-finite minimum track quality.".to_string(), [base.id.clone()]);
    }
    let mut params = base.params.clone();
    params.motion = payload.params.clone();
    protocol::MutationOutcome::new(RemodelDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
