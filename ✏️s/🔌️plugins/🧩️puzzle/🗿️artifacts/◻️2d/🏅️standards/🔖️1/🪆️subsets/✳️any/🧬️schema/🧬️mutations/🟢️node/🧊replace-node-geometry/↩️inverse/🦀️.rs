//! ↩️ Inverse for `ReplaceNodeGeometry` — restores the BASE shape/extent. Missing target ⇒ `Vec::new()`.
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::replace_node_geometry::component::ReplaceNodeGeometry;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceNodeGeometry>>(payload: &super::ReplaceNodeGeometry, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_node_geometry::replace_node_geometry(node.id.clone(), node.shape.clone(), node.radius, node.width, node.height)]

    })())
}
//#endregion 🔖️Inverse
