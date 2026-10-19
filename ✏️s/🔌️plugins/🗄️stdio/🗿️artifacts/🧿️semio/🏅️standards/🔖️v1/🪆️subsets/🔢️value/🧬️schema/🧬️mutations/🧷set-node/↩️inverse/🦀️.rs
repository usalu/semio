//! ↩️ Inverse for `SetNode`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetNode, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::remove_node::RemoveNode> + From<crate::set_node::SetNode>,
{
    let super::SetNode { id, .. } = payload;
    Ok(match base.nodes.iter().find(|n| &n.id == id) {
        Some(existing) => vec![Op::from(crate::set_node::SetNode { id: id.clone(), value: existing.value.clone(), at: None })],
        None => vec![Op::from(crate::remove_node::RemoveNode { id: id.clone() })],
    })
}
//#endregion 🔖️Inverse
