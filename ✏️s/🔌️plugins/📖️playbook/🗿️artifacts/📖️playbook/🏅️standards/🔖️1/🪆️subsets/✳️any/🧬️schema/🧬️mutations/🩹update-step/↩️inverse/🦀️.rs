//! ↩️ Inverse for `UpdateStep` — restores the captured BASE title/description. Missing target ⇒
//! `Vec::new()`.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::UpdateStep, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(previous) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::playbook::mutations::update_step::update_step_operation(&payload.step_id, previous.title.clone(), previous.description.clone())]
}
//#endregion 🔖️Inverse
