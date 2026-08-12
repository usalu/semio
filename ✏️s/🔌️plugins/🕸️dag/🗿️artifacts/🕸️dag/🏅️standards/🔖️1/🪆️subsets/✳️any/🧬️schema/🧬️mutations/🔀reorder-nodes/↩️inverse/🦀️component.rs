//! ↩️ Inverse for `ReorderNodes` — reorders back to BASE's own id order.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ReorderNodes, base: &DagSnapshot) -> Vec<DagMutation> {
    vec![super::mutation::reorder_nodes(base.nodes.iter().map(|node| node.id.clone()).collect())]
}
//#endregion 🔖️Inverse
