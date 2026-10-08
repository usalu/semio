//! ↩️ Inverse for `DisconnectHandles` — reconstructs a `connect-handles` of the captured BASE
//! edge. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DisconnectHandles, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some((index, edge)) = base.edges.iter().enumerate().find(|(_, edge)| edge.id == payload.id) else {
        return Vec::new();
    };
    crate::standards::v1::subsets::any::schema::mutations::connect_handles::restore_edge(edge, index).into_iter().rev().collect()

    })())
}
//#endregion 🔖️Inverse
