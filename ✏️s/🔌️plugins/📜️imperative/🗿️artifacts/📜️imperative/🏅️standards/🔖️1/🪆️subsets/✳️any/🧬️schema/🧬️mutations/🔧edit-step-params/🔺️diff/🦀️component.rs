//! 🔺️ Sparse diff builder for `EditStepParams` — patches one step's `params` dictionary wholesale
//! within `base`'s `flow` working scene, then whole-handle-replaces `flow`; idempotent no-op when
//! the id is absent.
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditStepParams, base: &ImperativeSnapshot) -> ImperativeDiff {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    if !steps.iter().any(|step| step.id == payload.id) {
        return ImperativeDiff::default();
    }
    let mut path = crate::artifacts::imperative::imperative_working_scene(base).path;
    if let Some(list) = crate::artifacts::imperative::mutations::resolve_path_mut(&mut path, &payload.path_ref) {
        if let Some(step) = list.iter_mut().find(|step| step.id == payload.id) {
            step.params = payload.new_params.clone();
        }
    }
    crate::artifacts::imperative::diff_replace_flow(&path)
}
//#endregion 🔖️Diff
