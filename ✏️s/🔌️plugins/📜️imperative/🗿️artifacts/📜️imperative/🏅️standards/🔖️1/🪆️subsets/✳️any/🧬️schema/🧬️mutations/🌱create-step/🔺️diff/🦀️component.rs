//! 🔺️ Sparse diff builder for `CreateStep` — a real append-only insert at the payload's
//! `path_ref`, applied to a copy of the CURRENT `flow` working scene, then whole-handle-replaced
//! (composed children are opaque; a diff never edits a sub-slice — never a whole-snapshot capture).
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
/// 🔺️ Fatal `invariant` when `path_ref.owner` names a step that doesn't exist in `base` (unknown
/// container); Fatal `duplicate-id` when `step.id` already names a step within the target list.
pub fn diff(payload: &super::CreateStep, base: &ImperativeSnapshot) -> protocol::MutationOutcome<ImperativeDiff> {
    if let Some(owner) = &payload.path_ref.owner {
        let path = crate::artifacts::imperative::imperative_working_scene(base).path;
        if !path.steps.iter().any(|step| &step.id == owner) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Container step \"{}\" does not exist.", owner), [owner.clone()]);
        }
    }
    let existing = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    if existing.iter().any(|step| step.id == payload.step.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A step with id \"{}\" already exists.", payload.step.id), [payload.step.id.clone()]);
    }
    let mut path = crate::artifacts::imperative::imperative_working_scene(base).path;
    if let Some(steps) = crate::artifacts::imperative::mutations::resolve_path_mut(&mut path, &payload.path_ref) {
        steps.push(payload.step.clone());
    }
    protocol::MutationOutcome::new(crate::artifacts::imperative::diff_replace_flow(&path))
}
//#endregion 🔖️Diff
