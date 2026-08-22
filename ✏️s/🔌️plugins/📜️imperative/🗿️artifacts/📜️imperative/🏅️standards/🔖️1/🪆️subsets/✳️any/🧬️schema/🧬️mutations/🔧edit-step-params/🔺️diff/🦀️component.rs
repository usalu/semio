//! 🔺️ Sparse diff builder for `EditStepParams` — patches one step's `params` dictionary wholesale
//! within `base`'s `flow` working scene, then whole-handle-replaces `flow`; idempotent no-op when
//! the id is absent.
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
/// 🔺️ Error `target-missing` when the id is absent; Warning `no-op` when `new_params` already
/// equals the step's current `params`.
pub fn diff(payload: &super::mutation::EditStepParams, base: &ImperativeSnapshot) -> protocol::MutationOutcome<ImperativeDiff> {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    let Some(existing) = steps.iter().find(|step| step.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.params == payload.new_params {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" parameters are already identical to the requested replacement.", payload.id));
    }
    let mut path = crate::artifacts::imperative::imperative_working_scene(base).path;
    if let Some(list) = crate::artifacts::imperative::mutations::resolve_path_mut(&mut path, &payload.path_ref) {
        if let Some(step) = list.iter_mut().find(|step| step.id == payload.id) {
            step.params = payload.new_params.clone();
        }
    }
    protocol::MutationOutcome::new(crate::artifacts::imperative::diff_replace_flow(&path))
}
//#endregion 🔖️Diff
