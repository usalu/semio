//! ↩️ Inverse for `AddBlock` — always a `remove-block` of the id it created.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddBlock, _base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    vec![crate::artifacts::playbook::mutations::remove_block::mutation::remove_block_operation(&payload.step_id, &payload.block.id)]
}
//#endregion 🔖️Inverse
