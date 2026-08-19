//! 🔺️ Sparse diff builder for `ChangeParameterBinding`.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeParameterBinding, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
    if base.parameter_bindings.get(&payload.key) == Some(&payload.new_value) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Parameter binding \"{}\" is already \"{:?}\".", payload.key, payload.new_value));
    }
    let mut bindings = BTreeMap::new();
    bindings.insert(payload.key.clone(), Some(payload.new_value.clone()));
    protocol::MutationOutcome::new(RewriteDiff { parameter_bindings: Some(bindings), ..Default::default() })
}
//#endregion 🔖️Diff
