//! ↩️ Inverse for `ChangeNodeLabel`.

use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ChangeNodeLabel, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::change_node_label::ChangeNodeLabel>,
{
    Ok((|| {
    match base.nodes.iter().find(|n| n.id == payload.id) {
        Some(node) => vec![Op::from(super::ChangeNodeLabel { id: payload.id.clone(), new_label: node.label.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
