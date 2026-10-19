//! 🔗️ target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeEdgeTips` — restores the BASE field value on the addressed edge. Missing
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_tips::component::ChangeEdgeTips;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeEdgeTips>>(payload: &super::ChangeEdgeTips, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_edge_tips::change_edge_tips(edge.id.clone(), edge.source_tip.clone(), edge.target_tip.clone())]

    })())
}
//#endregion 🔖️Inverse
