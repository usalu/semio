//! 🔺️ Sparse diff builder for `EditRhs`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditRhs, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.rhs == payload.new_rhs {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Rhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { rhs: Some(payload.new_rhs.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
