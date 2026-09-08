//! ↩️ Inverse for `UpdateStep` — restores the captured BASE title/description. Missing target ⇒
//! `Vec::new()`.

use crate::mutations::PlaybookMutation;
use crate::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::UpdateStep, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let steps = crate::playbook_working_scene(base).steps;
    let Some(previous) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    vec![crate::mutations::update_step::update_step_operation(&payload.step_id, previous.title.clone(), previous.description.clone())]
}
//#endregion 🔖️Inverse
