//! ↩️ Inverse for `ChangeNodeIcon` — the OLD `icon` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeNodeIcon, base: &DagSnapshot) -> Vec<DagMutation> {
    match dag_working_scene(base).nodes.into_iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::change_node_icon(payload.id.clone(), node.icon)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
