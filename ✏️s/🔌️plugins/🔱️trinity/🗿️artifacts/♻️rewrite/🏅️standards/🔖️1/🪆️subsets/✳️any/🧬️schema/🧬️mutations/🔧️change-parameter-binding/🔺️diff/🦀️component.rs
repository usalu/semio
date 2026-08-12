//! 🔺️ Sparse diff builder for `ChangeParameterBinding`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeParameterBinding, _base: &RewriteSnapshot) -> RewriteDiff {
    let mut bindings = BTreeMap::new();
    bindings.insert(payload.key.clone(), Some(payload.new_value.clone()));
    RewriteDiff { parameter_bindings: Some(bindings), ..Default::default() }
}
//#endregion 🔖️Diff
