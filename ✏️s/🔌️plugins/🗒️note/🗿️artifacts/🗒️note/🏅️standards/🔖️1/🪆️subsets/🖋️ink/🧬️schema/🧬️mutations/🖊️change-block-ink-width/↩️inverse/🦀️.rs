//! ↩️ Inverse for `ChangeBlockInkWidth`.
use super::ChangeBlockInkWidth;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeBlockInkWidth, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::ChangeBlockInkWidth>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Ink { stroke_width, .. }) => vec![Op::from(ChangeBlockInkWidth { id: payload.id.clone(), new_stroke_width: *stroke_width })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
