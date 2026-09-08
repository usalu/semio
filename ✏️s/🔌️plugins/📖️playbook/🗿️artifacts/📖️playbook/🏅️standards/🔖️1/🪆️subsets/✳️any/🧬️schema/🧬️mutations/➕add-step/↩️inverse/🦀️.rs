//! ↩️ Inverse for `AddStep` — always a `remove-step` of the id it created (the payload carries the
//! id, so no BASE lookup is needed to know what to undo).

use crate::mutations::PlaybookMutation;
use crate::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddStep, _base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    vec![crate::mutations::remove_step::remove_step_operation(&payload.step.id)]
}
//#endregion 🔖️Inverse
