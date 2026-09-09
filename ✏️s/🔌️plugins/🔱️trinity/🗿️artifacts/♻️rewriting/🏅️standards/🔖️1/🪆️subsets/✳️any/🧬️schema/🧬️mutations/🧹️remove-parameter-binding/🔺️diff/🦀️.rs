//! 🔺️ Sparse diff builder for `RemoveParameterBinding` — `None` signals a clear.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;
use replication::MapDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveParameterBinding, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !base.parameter_bindings.contains_key(&payload.key) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Parameter binding \"{}\" is already absent.", payload.key));
    }
    let bindings = MapDelta::remove(payload.key.clone());
    protocol::MutationOutcome::new(RewritingDiff { parameter_bindings: Some(bindings), ..Default::default() })
}
//#endregion 🔖️Diff
