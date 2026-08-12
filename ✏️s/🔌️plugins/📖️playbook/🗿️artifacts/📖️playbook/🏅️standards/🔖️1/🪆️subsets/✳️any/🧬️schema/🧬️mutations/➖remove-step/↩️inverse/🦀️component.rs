//! ↩️ Inverse for `RemoveStep` — reconstructs an `add-step` of the captured BASE step at its
//! original position. Missing target ⇒ `Vec::new()`.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RemoveStep, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let Some(position) = base.steps.iter().position(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    vec![PlaybookMutation::AddStep(crate::artifacts::playbook::mutations::add_step::mutation::AddStep { step: base.steps[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
