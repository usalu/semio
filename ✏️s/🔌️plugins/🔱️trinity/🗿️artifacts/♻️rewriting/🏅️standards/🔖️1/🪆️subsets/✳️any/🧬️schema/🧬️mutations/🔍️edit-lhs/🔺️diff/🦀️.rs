//! 🔺️ Sparse diff builder for `EditLhs`.
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditLhs, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.lhs_json == payload.new_lhs_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Lhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { lhs_json: Some(payload.new_lhs_json.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
