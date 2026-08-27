//! 🔺️ Sparse diff builder for `EditBeforeFixture`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditBeforeFixture, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
    if base.before_fixture_json == payload.new_before_fixture_json {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Before-fixture is already up to date.");
    }
    protocol::MutationOutcome::new(RewriteDiff { before_fixture_json: Some(payload.new_before_fixture_json.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
