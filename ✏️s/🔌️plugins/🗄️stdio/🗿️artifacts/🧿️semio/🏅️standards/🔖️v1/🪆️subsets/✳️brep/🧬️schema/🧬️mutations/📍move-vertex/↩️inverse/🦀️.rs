//! ↩️ Inverse for `MoveVertex`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::MoveVertex, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    match base.vertices.iter().find(|v| v.id == payload.vertex_id) {
        Some(vertex) => vec![SemioBrepMutation::MoveVertex(super::MoveVertex { vertex_id: payload.vertex_id.clone(), new_point: vertex.point })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
