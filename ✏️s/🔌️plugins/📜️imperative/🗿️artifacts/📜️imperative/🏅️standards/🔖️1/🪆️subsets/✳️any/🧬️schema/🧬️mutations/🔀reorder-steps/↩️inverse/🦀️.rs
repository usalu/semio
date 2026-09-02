//! ↩️ Inverse for `ReorderSteps` — `id`'s CURRENT position in BASE's resolved list; a reorder
//! never changes list length, so no extra clamping is needed beyond what's already valid. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReorderSteps, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    match steps.iter().position(|step| step.id == payload.id) {
        Some(orig_index) => vec![crate::artifacts::imperative::mutations::reorder_steps::reorder_steps(payload.path_ref.clone(), payload.id.clone(), orig_index)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
