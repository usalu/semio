//! ↩️ Inverse for `EditBlockText`.
use super::EditBlockText;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &EditBlockText, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::text::schema::mutations::edit_block_text::EditBlockText>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Text { content, .. }) => {
            let paragraphs = crate::note_block_text(content);
            vec![Op::from(EditBlockText { id: payload.id.clone(), new_paragraphs: paragraphs })]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
