//! 🔺️ Sparse diff builder for `UpdateMatchParams` — the field is always present, so there is no
//! missing-target case. A non-finite or out-of-range `ratio_test` ⇒ Fatal `mutation.invariant`;
//! identical params ⇒ Warning `mutation.no-op`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateMatchParams, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if !payload.params.ratio_test.is_finite() || payload.params.ratio_test <= 0.0 || payload.params.ratio_test > 1.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Match ratio test {} must be finite and within (0, 1].", payload.params.ratio_test), Vec::<String>::new());
    }
    if payload.params == base.params.matching {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Matching params are unchanged.");
    }
    let mut params = base.params.clone();
    params.matching = payload.params.clone();
    protocol::MutationOutcome::new(RemodelDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
