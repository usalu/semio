//! ↩️ Inverse for `ChangeBlockLocked`.
use super::ChangeBlockLocked;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeBlockLocked, base: &NoteSnapshot) -> Vec<NoteMutation> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else { return Vec::new() };
    let old = match block {
        crate::artifacts::note::NoteBlockNode::Text { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Image { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Table { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Math { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Ink { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Group { locked, .. } => *locked,
    };
    vec![NoteMutation::ChangeBlockLocked(ChangeBlockLocked { id: payload.id.clone(), new_locked: old })]
}
//#endregion 🔖️Inverse
