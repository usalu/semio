//! 🔺️ Sparse diff builder for `RemoveParameterBinding` — `None` signals a clear.
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::RewritingSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveParameterBinding, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !base.parameter_bindings.contains_key(&payload.key) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Parameter binding \"{}\" is already absent.", payload.key));
    }
    let mut bindings = BTreeMap::new();
    bindings.insert(payload.key.clone(), None);
    protocol::MutationOutcome::new(RewritingDiff { parameter_bindings: Some(bindings), ..Default::default() })
}
//#endregion 🔖️Diff
