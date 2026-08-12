//! ↩️ Inverse for `RemoveBlock` — reconstructs an `add-block` of the captured BASE block at its
//! original position within the step. Missing target ⇒ `Vec::new()`.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RemoveBlock, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let Some(step) = base.steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    let Some(position) = step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::playbook::mutations::add_block::mutation::add_block_operation(&payload.step_id, step.blocks[position].clone(), Some(position))]
}
//#endregion 🔖️Inverse
