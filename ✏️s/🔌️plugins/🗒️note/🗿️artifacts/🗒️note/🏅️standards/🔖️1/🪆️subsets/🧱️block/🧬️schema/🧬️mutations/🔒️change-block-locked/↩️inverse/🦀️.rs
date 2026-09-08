//! ↩️ Inverse for `ChangeBlockLocked`.
use super::ChangeBlockLocked;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeBlockLocked, base: &NoteSnapshot) -> Vec<NoteMutation> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else { return Vec::new() };
    let old = match block {
        crate::NoteBlockNode::Text { locked, .. }
        | crate::NoteBlockNode::Image { locked, .. }
        | crate::NoteBlockNode::Table { locked, .. }
        | crate::NoteBlockNode::Math { locked, .. }
        | crate::NoteBlockNode::Ink { locked, .. }
        | crate::NoteBlockNode::Group { locked, .. } => *locked,
    };
    vec![NoteMutation::ChangeBlockLocked(ChangeBlockLocked { id: payload.id.clone(), new_locked: old })]
}
//#endregion 🔖️Inverse
