//! 🔺️ Sparse diff builder for `CreateStep` — a real append-only insert at the payload's
//! `path_ref`, applied to a copy of the CURRENT `flow` working scene, then whole-handle-replaced
//! (composed children are opaque; a diff never edits a sub-slice — never a whole-snapshot capture).
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateStep, base: &ImperativeSnapshot) -> ImperativeDiff {
    let mut path = crate::artifacts::imperative::imperative_working_scene(base).path;
    if let Some(steps) = crate::artifacts::imperative::mutations::resolve_path_mut(&mut path, &payload.path_ref) {
        steps.push(payload.step.clone());
    }
    crate::artifacts::imperative::diff_replace_flow(&path)
}
//#endregion 🔖️Diff
