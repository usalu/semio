//! 🔺️ Sparse diff builder for `RemoveParameterBinding` — `None` signals a clear.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveParameterBinding, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
    if !base.parameter_bindings.contains_key(&payload.key) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Parameter binding \"{}\" is already absent.", payload.key));
    }
    let mut bindings = BTreeMap::new();
    bindings.insert(payload.key.clone(), None);
    protocol::MutationOutcome::new(RewriteDiff { parameter_bindings: Some(bindings), ..Default::default() })
}
//#endregion 🔖️Diff
