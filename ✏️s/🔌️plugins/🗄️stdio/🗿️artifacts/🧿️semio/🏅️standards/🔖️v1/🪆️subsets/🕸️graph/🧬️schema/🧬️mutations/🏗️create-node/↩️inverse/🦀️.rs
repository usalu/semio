//! ↩️ Inverse for `CreateNode`.

use crate::schema::mutations::delete_node;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateNode, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_node::DeleteNode>,
{
    Ok((|| {
    if base.nodes.iter().any(|n| n.id == payload.id) {
        return Vec::new();
    }
    vec![Op::from(delete_node::DeleteNode { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
