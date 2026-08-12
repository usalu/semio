//! 🔺️ Sparse diff builder for `DeleteStep` — resolves the step list at the payload's `path_ref`
//! in `base`; idempotent no-op when the id is already absent.
use crate::artifacts::imperative::diff::{ImperativeDiff, ImperativePathDelta, ImperativeStepsDelta};
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteStep, base: &ImperativeSnapshot) -> ImperativeDiff {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref).unwrap_or(&[]);
    if !steps.iter().any(|step| step.id == payload.id) {
        return ImperativeDiff::default();
    }
    ImperativeDiff {
        path: Some(ImperativePathDelta {
            path_ref: payload.path_ref.clone(),
            steps: ImperativeStepsDelta { removed: vec![payload.id.clone()], ..Default::default() },
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
