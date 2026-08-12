//! ↩️ Inverse for `ReplaceBlock` — restores the captured BASE block. Missing target ⇒ `Vec::new()`.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceBlock, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let Some(step) = base.steps.iter().find(|step| step.id == payload.step_id) else {
        return Vec::new();
    };
    let Some(previous) = step.blocks.iter().find(|block| block.id == payload.block.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::playbook::mutations::replace_block::mutation::replace_block_operation(&payload.step_id, previous.clone())]
}
//#endregion 🔖️Inverse
