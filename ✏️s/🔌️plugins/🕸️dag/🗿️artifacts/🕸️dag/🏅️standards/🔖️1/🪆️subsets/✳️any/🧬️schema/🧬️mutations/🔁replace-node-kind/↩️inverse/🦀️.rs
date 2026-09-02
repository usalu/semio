//! ↩️ Inverse for `ReplaceNodeKind` — the OLD `kind` looked up from BASE. Missing target ⇒
//! `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReplaceNodeKind, base: &DagSnapshot) -> Vec<DagMutation> {
    match dag_working_scene(base).nodes.into_iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::replace_node_kind(payload.id.clone(), node.kind)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
