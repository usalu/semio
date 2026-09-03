//! ↩️ Inverse for `EditStepParams` — the OLD `params` dictionary looked up from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::ProcedureSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::EditStepParams, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
    let steps = crate::artifacts::procedure::mutations::resolve_steps(base, &payload.path_ref);
    match steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![crate::artifacts::procedure::mutations::edit_step_params::edit_step_params(payload.path_ref.clone(), payload.id.clone(), step.params.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
