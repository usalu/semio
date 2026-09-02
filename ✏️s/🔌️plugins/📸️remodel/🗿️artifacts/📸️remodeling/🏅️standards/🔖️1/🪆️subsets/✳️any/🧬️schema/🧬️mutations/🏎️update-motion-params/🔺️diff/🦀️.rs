//! 🔺️ Sparse diff builder for `UpdateMotionParams` — always present, no target-missing check
//! possible (a struct field, not an id-keyed collection). Identical resubmission ⇒ Warning;
//! non-finite track quality ⇒ Fatal.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateMotionParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.params == base.params.motion {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Motion params are already up to date.".to_string());
    }
    if !payload.params.min_track_quality.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Motion params have a non-finite minimum track quality.".to_string(), [base.id.clone()]);
    }
    let mut params = base.params.clone();
    params.motion = payload.params.clone();
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
