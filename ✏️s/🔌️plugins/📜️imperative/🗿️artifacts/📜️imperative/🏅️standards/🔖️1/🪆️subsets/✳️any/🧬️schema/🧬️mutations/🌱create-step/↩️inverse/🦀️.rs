//! ↩️ Inverse for `CreateStep` — always a `delete-step` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::ImperativeSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateStep, _base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
    vec![crate::artifacts::imperative::mutations::delete_step::delete_step(payload.path_ref.clone(), payload.step.id.clone())]
}
//#endregion 🔖️Inverse
