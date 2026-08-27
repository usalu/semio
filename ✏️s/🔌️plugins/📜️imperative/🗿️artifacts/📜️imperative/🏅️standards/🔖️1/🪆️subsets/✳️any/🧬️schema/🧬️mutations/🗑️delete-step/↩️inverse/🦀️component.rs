//! ↩️ Inverse for `DeleteStep` — reconstructs a `create-step` of the FULL captured BASE step
//! (its `bodies` cascade rides along inside the struct, no separate reconnection logic needed).
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteStep, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
    let steps = crate::artifacts::imperative::mutations::resolve_steps(base, &payload.path_ref);
    match steps.iter().find(|step| step.id == payload.id) {
        Some(step) => vec![crate::artifacts::imperative::mutations::create_step::create_step(payload.path_ref.clone(), step.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
