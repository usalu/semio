//! 🔺️ Sparse diff builder for `RemoveParameterBinding` — `None` signals a clear.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveParameterBinding, _base: &RewriteSnapshot) -> RewriteDiff {
    let mut bindings = BTreeMap::new();
    bindings.insert(payload.key.clone(), None);
    RewriteDiff { parameter_bindings: Some(bindings), ..Default::default() }
}
//#endregion 🔖️Diff
