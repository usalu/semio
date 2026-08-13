//! 🔺️ Sparse diff builder for `ReorderSteps` — computes the full reordered id list for the
//! payload's `path_ref` from `base`'s `flow` working scene, applies it there, then whole-handle-
//! replaces `flow`; idempotent no-op when the id is absent.
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReorderSteps, base: &ImperativeSnapshot) -> ImperativeDiff {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    let mut ids: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let Some(from) = ids.iter().position(|id| id == &payload.id) else {
        return ImperativeDiff::default();
    };
    let item = ids.remove(from);
    let to = payload.to_index.min(ids.len());
    ids.insert(to, item);

    let mut path = crate::artifacts::imperative::imperative_working_scene(base).path;
    if let Some(list) = crate::artifacts::imperative::mutations::resolve_path_mut(&mut path, &payload.path_ref) {
        let mut by_id: std::collections::BTreeMap<String, crate::artifacts::imperative::Step> = list.drain(..).map(|step| (step.id.clone(), step)).collect();
        let mut ordered = Vec::with_capacity(ids.len());
        for id in &ids {
            if let Some(step) = by_id.remove(id) {
                ordered.push(step);
            }
        }
        ordered.extend(by_id.into_values());
        *list = ordered;
    }
    crate::artifacts::imperative::diff_replace_flow(&path)
}
//#endregion 🔖️Diff
