//! ↩️ Inverse for `ReplaceNodeProperties` — the OLD `properties` looked up from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReplaceNodeProperties, base: &DagSnapshot) -> Vec<DagMutation> {
    match dag_working_scene(base).nodes.into_iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::replace_node_properties(payload.id.clone(), node.properties)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
