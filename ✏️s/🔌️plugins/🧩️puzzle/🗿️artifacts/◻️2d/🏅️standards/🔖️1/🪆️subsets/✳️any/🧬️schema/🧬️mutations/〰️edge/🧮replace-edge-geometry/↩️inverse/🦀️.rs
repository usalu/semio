//! `Vec::new()`.
//! ↩️ Inverse for `ReplaceEdgeGeometry` — restores the BASE connection pose. Missing target ⇒
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceEdgeGeometry>>(payload: &super::ReplaceEdgeGeometry, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::replace_edge_geometry(edge.id.clone(), edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y)]

    })())
}
//#endregion 🔖️Inverse
