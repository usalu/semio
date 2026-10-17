//! ↩️ Inverse for `MoveNode`.

use crate::schema::diff::node_origin;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::MoveNode, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::move_node::MoveNode>,
{
    Ok((|| {
    match node_origin(base, &payload.at) {
        Some(old_origin) => vec![Op::from(super::MoveNode { at: payload.at.clone(), new_origin: old_origin })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
