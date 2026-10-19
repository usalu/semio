//! ↩️ Inverse for `RemoveNode`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveNode, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::set_node::SetNode>,
{
    let super::RemoveNode { id } = payload;
    Ok(match base.nodes.iter().position(|n| &n.id == id) {
        Some(at) => vec![Op::from(crate::set_node::SetNode { id: id.clone(), value: base.nodes[at].value.clone(), at: Some(at) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
