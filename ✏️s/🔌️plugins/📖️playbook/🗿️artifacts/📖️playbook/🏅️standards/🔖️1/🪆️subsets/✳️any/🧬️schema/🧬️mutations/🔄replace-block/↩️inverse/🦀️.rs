//! ↩️ Inverse for `ReplaceBlock` — restores the captured BASE block. Missing target ⇒ `Vec::new()`.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReplaceBlock, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    let Some(previous) = step.blocks.iter().find(|block| block.id == payload.block.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::playbook::mutations::replace_block::replace_block_operation(&payload.step_id, previous.clone())]
}
//#endregion 🔖️Inverse
