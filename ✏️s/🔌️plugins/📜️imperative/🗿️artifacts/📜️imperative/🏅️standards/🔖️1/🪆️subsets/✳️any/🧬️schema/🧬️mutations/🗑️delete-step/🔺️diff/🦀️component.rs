//! 🔺️ Sparse diff builder for `DeleteStep` — resolves the step list at the payload's `path_ref`
//! in `base`'s `flow` working scene, removes the id there, then whole-handle-replaces `flow`;
//! idempotent no-op when the id is already absent.
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
/// 🔺️ Error `target-missing` when the id is absent from the payload's `path_ref` target list.
pub fn diff(payload: &super::mutation::DeleteStep, base: &ImperativeSnapshot) -> protocol::MutationOutcome<ImperativeDiff> {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    if !steps.iter().any(|step| step.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut path = crate::artifacts::imperative::imperative_working_scene(base).path;
    if let Some(steps) = crate::artifacts::imperative::mutations::resolve_path_mut(&mut path, &payload.path_ref) {
        steps.retain(|step| step.id != payload.id);
    }
    crate::artifacts::imperative::mutations::prune_empty_slot(&mut path, &payload.path_ref);
    protocol::MutationOutcome::new(crate::artifacts::imperative::diff_replace_flow(&path))
}
//#endregion 🔖️Diff
