//! 🔺️ Sparse diff builder for `EditBeforeFixture`.
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditBeforeFixture, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.before_fixture_json == payload.new_before_fixture_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Before-fixture is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(payload.new_before_fixture_json.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
