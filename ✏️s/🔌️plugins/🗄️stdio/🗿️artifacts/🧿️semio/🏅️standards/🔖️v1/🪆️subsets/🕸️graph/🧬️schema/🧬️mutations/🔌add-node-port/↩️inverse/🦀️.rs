//! ↩️ Inverse for `AddNodePort`.

use crate::schema::mutations::remove_node_port;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::AddNodePort, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_node_port::RemoveNodePort>,
{
    Ok((|| {
    match base.nodes.iter().find(|n| n.id == payload.node_id) {
        Some(node) => {
            let at = payload.index.min(node.ports.len());
            vec![Op::from(remove_node_port::RemoveNodePort { node_id: payload.node_id.clone(), index: at })]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
