//! ↩️ Inverse for `MoveBlockToContainer`.
use super::MoveBlockToContainer;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &MoveBlockToContainer, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block_location(&base.blocks, &payload.id) {
        Some((parent_id, index)) => vec![NoteMutation::MoveBlockToContainer(MoveBlockToContainer { id: payload.id.clone(), new_parent_id: parent_id, index })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
