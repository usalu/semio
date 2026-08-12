//! 🔺️ Sparse diff builder for `ReorderSteps` — the full reordered id list for the payload's
//! `path_ref`, resolved from `base`; idempotent no-op when the id is absent.
use crate::artifacts::imperative::diff::{ImperativeDiff, ImperativePathDelta, ImperativeStepsDelta};
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReorderSteps, base: &ImperativeSnapshot) -> ImperativeDiff {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref).unwrap_or(&[]);
    let mut ids: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let Some(from) = ids.iter().position(|id| id == &payload.id) else {
        return ImperativeDiff::default();
    };
    let item = ids.remove(from);
    let to = payload.to_index.min(ids.len());
    ids.insert(to, item);
    ImperativeDiff {
        path: Some(ImperativePathDelta { path_ref: payload.path_ref.clone(), steps: ImperativeStepsDelta { reordered: Some(ids), ..Default::default() } }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
