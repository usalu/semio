//! ↩️ Inverse for `UpdateStep` — restores the captured BASE title/description. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::UpdateStep, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let Some(previous) = base.steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::playbook::mutations::update_step::mutation::update_step_operation(&payload.step_id, previous.title.clone(), previous.description.clone())]
}
//#endregion 🔖️Inverse
