//! ⌨️ target ⇒ `Vec::new()`.
//! ↩️ Inverse for `EditNodeText` — restores the BASE field value on the addressed node. Missing
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::edit_node_text::component::EditNodeText;

//#region 🔖️Inverse
pub fn inverse<Op: From<EditNodeText>>(payload: &super::EditNodeText, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::edit_node_text::edit_node_text(node.id.clone(), node.text.clone())]

    })())
}
//#endregion 🔖️Inverse
