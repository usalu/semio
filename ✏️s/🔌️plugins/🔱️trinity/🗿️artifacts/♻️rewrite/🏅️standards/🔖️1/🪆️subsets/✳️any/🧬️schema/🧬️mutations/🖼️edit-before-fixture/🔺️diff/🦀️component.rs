//! 🔺️ Sparse diff builder for `EditBeforeFixture`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditBeforeFixture, _base: &RewriteSnapshot) -> RewriteDiff {
    RewriteDiff { before_fixture_json: Some(payload.new_before_fixture_json.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
