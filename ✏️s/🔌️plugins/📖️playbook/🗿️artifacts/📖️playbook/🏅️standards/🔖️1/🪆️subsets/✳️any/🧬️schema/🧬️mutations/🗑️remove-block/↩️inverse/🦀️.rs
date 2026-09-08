//! ↩️ Inverse for `RemoveBlock` — reconstructs an `add-block` of the captured BASE block at its
//! original position within the step. Missing target ⇒ `Vec::new()`.

use crate::mutations::PlaybookMutation;
use crate::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveBlock, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let steps = crate::playbook_working_scene(base).steps;
    let Some(step) = steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    let Some(position) = step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return Vec::new();
    };
    vec![crate::mutations::add_block::add_block_operation(&payload.step_id, step.blocks[position].clone(), Some(position))]
}
//#endregion 🔖️Inverse
