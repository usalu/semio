//! ↩️ Inverse for `ReorderNodes` — reorders back to BASE's own id order.
use crate::mutations::DagMutation;
use crate::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ReorderNodes, base: &DagSnapshot) -> Vec<DagMutation> {
    vec![super::mutation::reorder_nodes(dag_working_scene(base).nodes.into_iter().map(|node| node.id).collect())]
}
//#endregion 🔖️Inverse
