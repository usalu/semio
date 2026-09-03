//! ↩️ Inverse for `ReorderSteps` — `id`'s CURRENT position in BASE's resolved list; a reorder
//! never changes list length, so no extra clamping is needed beyond what's already valid. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::ProcedureSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReorderSteps, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
    let steps = crate::artifacts::procedure::mutations::resolve_steps(base, &payload.path_ref);
    match steps.iter().position(|step| step.id == payload.id) {
        Some(orig_index) => vec![crate::artifacts::procedure::mutations::reorder_steps::reorder_steps(payload.path_ref.clone(), payload.id.clone(), orig_index)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
