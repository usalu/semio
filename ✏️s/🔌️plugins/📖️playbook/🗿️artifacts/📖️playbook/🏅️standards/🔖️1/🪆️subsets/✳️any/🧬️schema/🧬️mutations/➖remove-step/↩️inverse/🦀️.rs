//! ↩️ Inverse for `RemoveStep` — reconstructs an `add-step` of the captured BASE step at its
//! original position. Missing target ⇒ `Vec::new()`.

use crate::mutations::PlaybookMutation;
use crate::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveStep, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let steps = crate::playbook_working_scene(base).steps;
    let Some(position) = steps.iter().position(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    vec![PlaybookMutation::AddStep(crate::mutations::add_step::AddStep { step: steps[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
