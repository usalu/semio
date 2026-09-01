//! ↩️ Inverse for `MoveStep` — moves the step back to its BASE-state position. Missing target ⇒
//! `Vec::new()`.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveStep, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(position) = steps.iter().position(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::playbook::mutations::move_step::move_step_operation(&payload.step_id, position)]
}
//#endregion 🔖️Inverse
