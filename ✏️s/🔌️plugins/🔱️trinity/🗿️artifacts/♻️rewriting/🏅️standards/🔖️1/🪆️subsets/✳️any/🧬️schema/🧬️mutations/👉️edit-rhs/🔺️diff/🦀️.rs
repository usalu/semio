//! 🔺️ Sparse diff builder for `EditRhs`.
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditRhs, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.rhs_json == payload.new_rhs_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Rhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { rhs_json: Some(payload.new_rhs_json.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
