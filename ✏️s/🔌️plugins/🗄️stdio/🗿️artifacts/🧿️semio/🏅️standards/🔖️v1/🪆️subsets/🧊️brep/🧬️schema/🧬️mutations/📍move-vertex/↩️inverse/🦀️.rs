//! ↩️ Inverse for `MoveVertex`.

use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::MoveVertex, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::move_vertex::MoveVertex>,
{
    Ok((|| {
    match base.vertices.iter().find(|v| v.id == payload.vertex_id) {
        Some(vertex) => vec![Op::from(super::MoveVertex { vertex_id: payload.vertex_id.clone(), new_point: vertex.point })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
