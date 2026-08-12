//! 🔺️ Sparse diff builder for `EditStepParams` — a direct patched-entry replace (`Dictionary` IS
//! the whole patch value, no wrapper patch-struct); idempotent no-op when the id is absent.
use crate::artifacts::imperative::diff::{ImperativeDiff, ImperativePathDelta, ImperativeStepPatchEntry, ImperativeStepsDelta};
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::EditStepParams, base: &ImperativeSnapshot) -> ImperativeDiff {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref).unwrap_or(&[]);
    if !steps.iter().any(|step| step.id == payload.id) {
        return ImperativeDiff::default();
    }
    ImperativeDiff {
        path: Some(ImperativePathDelta {
            path_ref: payload.path_ref.clone(),
            steps: ImperativeStepsDelta { patched: vec![ImperativeStepPatchEntry { id: payload.id.clone(), patch: payload.new_params.clone() }], ..Default::default() },
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
