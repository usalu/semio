//! 🔗️ target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeEdgeKind` — restores the BASE field value on the addressed edge. Missing
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_kind::component::ChangeEdgeKind;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeEdgeKind>>(payload: &super::ChangeEdgeKind, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_edge_kind::change_edge_kind(edge.id.clone(), edge.edge_kind.clone())]

    })())
}
//#endregion 🔖️Inverse
