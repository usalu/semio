//! ↩️ Inverse for `ChangeNodeKind`.

use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ChangeNodeKind, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::change_node_kind::ChangeNodeKind>,
{
    Ok((|| {
    match base.nodes.iter().find(|n| n.id == payload.id) {
        Some(node) => vec![Op::from(super::ChangeNodeKind { id: payload.id.clone(), new_kind: node.kind.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
