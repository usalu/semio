//! 🔺️ Sparse diff builder for `UpdateFeatureParams` — the field is always present, so there is no
//! missing-target case. A zero target count or a non-finite/negative edge threshold ⇒ Fatal
//! `mutation.invariant`; identical params ⇒ Warning `mutation.no-op`.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateFeatureParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.params.target_count == 0 || !payload.params.edge_threshold.is_finite() || payload.params.edge_threshold < 0.0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Feature params need a positive target count and a finite non-negative edge threshold (got target_count={}, edge_threshold={}).", payload.params.target_count, payload.params.edge_threshold),
            Vec::<String>::new(),
        );
    }
    if payload.params == base.params.feature {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Feature params are unchanged.");
    }
    let mut params = base.params.clone();
    params.feature = payload.params.clone();
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
