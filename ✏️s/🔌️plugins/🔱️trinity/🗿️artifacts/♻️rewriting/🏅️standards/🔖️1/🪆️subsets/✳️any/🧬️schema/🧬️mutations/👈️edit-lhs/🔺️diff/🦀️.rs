//! 🔺️ Sparse diff builder for `EditLhs`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditLhs, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.lhs == payload.new_lhs {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Lhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { lhs: Some(payload.new_lhs.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
