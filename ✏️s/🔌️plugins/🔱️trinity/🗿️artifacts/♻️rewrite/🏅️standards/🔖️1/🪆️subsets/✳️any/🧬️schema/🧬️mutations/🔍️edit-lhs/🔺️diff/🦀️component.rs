//! 🔺️ Sparse diff builder for `EditLhs`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditLhs, _base: &RewriteSnapshot) -> RewriteDiff {
    RewriteDiff { lhs_json: Some(payload.new_lhs_json.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
