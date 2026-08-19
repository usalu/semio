//! 🔺️ Sparse diff builder for `EditLhs`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::EditLhs, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
    if base.lhs_json == payload.new_lhs_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Lhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewriteDiff { lhs_json: Some(payload.new_lhs_json.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
