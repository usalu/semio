//! ↩️ Inverse for `RemoveNodePort`.

use crate::schema::mutations::add_node_port;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveNodePort, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::add_node_port::AddNodePort>,
{
    Ok((|| {
    match base.nodes.iter().find(|n| n.id == payload.node_id).and_then(|node| node.ports.get(payload.index)) {
        Some(port) => vec![Op::from(add_node_port::AddNodePort { node_id: payload.node_id.clone(), index: payload.index, port: port.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
