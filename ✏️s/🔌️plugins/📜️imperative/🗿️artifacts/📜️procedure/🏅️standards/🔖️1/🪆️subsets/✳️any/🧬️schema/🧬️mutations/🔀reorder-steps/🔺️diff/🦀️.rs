//! 🔺️ Sparse diff builder for `ReorderSteps` — computes the full reordered id list for the
//! payload's `path_ref` from `base`'s `flow` working scene, applies it there, then whole-handle-
//! replaces `flow`; idempotent no-op when the id is absent.
use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::ProcedureSnapshot;

//#region 🔖️Diff
/// 🔺️ Error `target-missing` when the id is absent from the payload's `path_ref` target list;
/// Warning `no-op` when the resulting order is unchanged.
pub fn diff(payload: &super::ReorderSteps, base: &ProcedureSnapshot) -> protocol::MutationOutcome<ProcedureDiff> {
    let steps = crate::artifacts::procedure::mutations::resolve_steps(base, &payload.path_ref);
    let original: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let Some(from) = original.iter().position(|id| id == &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Step \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let mut ids = original.clone();
    let item = ids.remove(from);
    let to = payload.to_index.min(ids.len());
    ids.insert(to, item);
    if ids == original {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Step \"{}\" is already at position {}.", payload.id, to));
    }

    let mut path = crate::artifacts::procedure::procedure_working_scene(base).path;
    if let Some(list) = crate::artifacts::procedure::mutations::resolve_path_mut(&mut path, &payload.path_ref) {
        let mut by_id: std::collections::BTreeMap<String, crate::artifacts::procedure::Step> = list.drain(..).map(|step| (step.id.clone(), step)).collect();
        let mut ordered = Vec::with_capacity(ids.len());
        for id in &ids {
            if let Some(step) = by_id.remove(id) {
                ordered.push(step);
            }
        }
        ordered.extend(by_id.into_values());
        *list = ordered;
    }
    protocol::MutationOutcome::new(crate::artifacts::procedure::diff_replace_flow(&path))
}
//#endregion 🔖️Diff
