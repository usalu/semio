//! 🔺️ Sparse diff builder for `ChangeRuleLayoutPoint`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeRuleLayoutPoint, _base: &RewriteSnapshot) -> RewriteDiff {
    let mut layout = BTreeMap::new();
    layout.insert(payload.key.clone(), Some(payload.new_point.clone()));
    RewriteDiff { rule_layout: Some(layout), ..Default::default() }
}
//#endregion 🔖️Diff
