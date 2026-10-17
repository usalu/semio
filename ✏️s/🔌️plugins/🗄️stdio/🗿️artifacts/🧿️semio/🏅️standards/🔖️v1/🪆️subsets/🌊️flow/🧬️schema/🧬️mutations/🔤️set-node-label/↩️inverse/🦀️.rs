//! ↩️ Inverse for `SetNodeLabel`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::node_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_node_label;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetNodeLabel, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_node_label::SetNodeLabel>,
{
    let super::SetNodeLabel { id, .. } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![Op::from(set_node_label::SetNodeLabel { id: id.clone(), label: node.label.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
