//! 🔺️ Sparse diff builder for `EditRhs`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditRhs, _base: &RewriteSnapshot) -> RewriteDiff {
    RewriteDiff { rhs_json: Some(payload.new_rhs_json.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
