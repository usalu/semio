//! 🔺️ Sparse diff builder for `EditBeforeFixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditBeforeFixture, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.working_graph == payload.new_working_graph {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Before-fixture is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { working_graph: Some(payload.new_working_graph.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
