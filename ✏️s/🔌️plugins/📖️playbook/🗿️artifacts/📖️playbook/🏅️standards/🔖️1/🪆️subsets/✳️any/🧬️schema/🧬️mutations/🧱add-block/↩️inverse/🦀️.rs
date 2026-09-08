//! ↩️ Inverse for `AddBlock` — always a `remove-block` of the id it created.

use crate::mutations::PlaybookMutation;
use crate::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddBlock, _base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    vec![crate::mutations::remove_block::remove_block_operation(&payload.step_id, &payload.block.id)]
}
//#endregion 🔖️Inverse
