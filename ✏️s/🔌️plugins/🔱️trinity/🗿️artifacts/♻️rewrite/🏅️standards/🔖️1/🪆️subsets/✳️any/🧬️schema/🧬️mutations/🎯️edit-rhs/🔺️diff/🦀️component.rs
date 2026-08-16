//! 🔺️ Sparse diff builder for `EditRhs`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditRhs, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
    if base.rhs_json == payload.new_rhs_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Rhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewriteDiff { rhs_json: Some(payload.new_rhs_json.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
