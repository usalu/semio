//! 🔺️ Sparse diff builder for `ChangeParameterBinding`.
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::RewritingSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeParameterBinding, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if base.parameter_bindings.get(&payload.key) == Some(&payload.new_value) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Parameter binding \"{}\" is already \"{:?}\".", payload.key, payload.new_value));
    }
    let mut bindings = BTreeMap::new();
    bindings.insert(payload.key.clone(), Some(payload.new_value.clone()));
    protocol::MutationOutcome::new(RewritingDiff { parameter_bindings: Some(bindings), ..Default::default() })
}
//#endregion 🔖️Diff
