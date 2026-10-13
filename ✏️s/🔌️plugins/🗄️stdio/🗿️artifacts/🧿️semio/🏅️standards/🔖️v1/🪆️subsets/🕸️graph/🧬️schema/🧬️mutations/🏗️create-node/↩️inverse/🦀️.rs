//! ↩️ Inverse for `CreateNode`.

use crate::schema::mutations::{delete_node, SemioGraphMutation};
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateNode, base: &SemioGraphSnapshot) -> Result<Vec<SemioGraphMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.nodes.iter().any(|n| n.id == payload.id) {
        return Vec::new();
    }
    vec![SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
