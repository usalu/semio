//! ↩️ Inverse for `InsertTableColumn`.
use super::InsertTableColumn;
use crate::standards::v1::subsets::table::schema::mutations::remove_table_column::RemoveTableColumn;
use crate::{NoteBlockNode, NoteSnapshot};

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &InsertTableColumn, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::table::schema::mutations::remove_table_column::RemoveTableColumn>,
{
    Ok(match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(NoteBlockNode::Table { rows, .. }) if payload.cells.as_ref().is_none_or(|cells| cells.len() == rows.len()) => vec![Op::from(RemoveTableColumn { id: payload.id.clone() })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
