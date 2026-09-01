//! ↩️ Inverse for `MoveBlock` — moves the block back from `to_step_id` to `from_step_id`, at the
//! BASE-state position it held before the move. Missing target ⇒ `Vec::new()`.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveBlock, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let steps = crate::artifacts::playbook::playbook_working_scene(base).steps;
    let Some(step) = steps.iter().find(|step| step.id == payload.from_step_id) else {
        return Vec::new();
    };
    let Some(position) = step.blocks.iter().position(|block| block.id == payload.block_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::playbook::mutations::move_block::move_block_operation(&payload.block_id, &payload.to_step_id, &payload.from_step_id, position)]
}
//#endregion 🔖️Inverse
