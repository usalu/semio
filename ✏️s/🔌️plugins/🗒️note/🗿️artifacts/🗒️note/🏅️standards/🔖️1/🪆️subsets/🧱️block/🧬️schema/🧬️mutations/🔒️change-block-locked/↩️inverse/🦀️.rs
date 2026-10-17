//! ↩️ Inverse for `ChangeBlockLocked`.
use super::ChangeBlockLocked;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeBlockLocked, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::change_block_locked::ChangeBlockLocked>,
{
    Ok((|| {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else { return Vec::new() };
    let old = match block {
        crate::NoteBlockNode::Text { locked, .. }
        | crate::NoteBlockNode::Image { locked, .. }
        | crate::NoteBlockNode::Table { locked, .. }
        | crate::NoteBlockNode::Math { locked, .. }
        | crate::NoteBlockNode::Ink { locked, .. }
        | crate::NoteBlockNode::Group { locked, .. } => *locked,
    };
    vec![Op::from(ChangeBlockLocked { id: payload.id.clone(), new_locked: old })]

    })())
}
//#endregion 🔖️Inverse
