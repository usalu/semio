//! target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeEdgeVisible` — restores the BASE field value on the addressed edge. Missing
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeEdgeVisible>>(payload: &super::ChangeEdgeVisible, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::change_edge_visible(edge.id.clone(), edge.visible)]

    })())
}
//#endregion 🔖️Inverse
