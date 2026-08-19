//! ↩️ Inverse for `AddStep` — always a `remove-step` of the id it created (the payload carries the
//! id, so no BASE lookup is needed to know what to undo).
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::AddStep, _base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    vec![crate::artifacts::playbook::mutations::remove_step::mutation::remove_step_operation(&payload.step.id)]
}
//#endregion 🔖️Inverse
