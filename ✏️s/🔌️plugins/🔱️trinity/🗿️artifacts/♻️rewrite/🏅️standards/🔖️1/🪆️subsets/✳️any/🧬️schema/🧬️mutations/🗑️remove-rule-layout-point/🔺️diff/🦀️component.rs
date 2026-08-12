//! 🔺️ Sparse diff builder for `RemoveRuleLayoutPoint` — `None` signals a clear.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveRuleLayoutPoint, _base: &RewriteSnapshot) -> RewriteDiff {
    let mut layout = BTreeMap::new();
    layout.insert(payload.key.clone(), None);
    RewriteDiff { rule_layout: Some(layout), ..Default::default() }
}
//#endregion 🔖️Diff
