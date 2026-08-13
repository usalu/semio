//! ↩️ Inverse for `EditStepParams` — the OLD `params` dictionary looked up from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::EditStepParams, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    match steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![crate::artifacts::imperative::mutations::edit_step_params::mutation::edit_step_params(payload.path_ref.clone(), payload.id.clone(), step.params.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
