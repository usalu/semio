//! ↩️ Inverse for `DisconnectHandles` — reconstructs a `connect-handles` of the captured BASE
//! edge. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::ChangeEdgeVisible;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::ChangeEdgeLocked;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::ConnectHandles;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DisconnectHandles> + From<ConnectHandles> + From<ChangeEdgeVisible> + From<ChangeEdgeLocked>>(payload: &super::DisconnectHandles, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some((index, edge)) = base.edges.iter().enumerate().find(|(_, edge)| edge.id == payload.id) else {
        return Vec::new();
    };
    crate::standards::v1::subsets::any::schema::mutations::connect_handles::restore_edge(edge, index).into_iter().rev().collect()

    })())
}
//#endregion 🔖️Inverse
