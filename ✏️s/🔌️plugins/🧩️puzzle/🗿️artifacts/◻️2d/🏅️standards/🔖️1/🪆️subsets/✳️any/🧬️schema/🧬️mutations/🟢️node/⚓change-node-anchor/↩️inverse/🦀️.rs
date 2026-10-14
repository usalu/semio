//! ↩️ Inverse for `ChangeNodeAnchor` — restores the BASE field value on the addressed node. Missing
//! target ⇒ `Vec::new()`.
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeNodeAnchor>>(payload: &super::ChangeNodeAnchor, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_node_anchor::change_node_anchor(node.id.clone(), node.anchor)]

    })())
}
//#endregion 🔖️Inverse
