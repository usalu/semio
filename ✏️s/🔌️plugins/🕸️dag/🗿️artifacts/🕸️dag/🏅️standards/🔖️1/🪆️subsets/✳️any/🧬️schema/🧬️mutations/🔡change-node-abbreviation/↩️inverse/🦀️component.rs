//! ↩️ Inverse for `ChangeNodeAbbreviation` — the OLD `abbreviation` looked up from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeAbbreviation, base: &DagSnapshot) -> Vec<DagMutation> {
    match dag_working_scene(base).nodes.into_iter().find(|node| node.id == payload.id) {
        Some(node) => vec![super::mutation::change_node_abbreviation(payload.id.clone(), node.abbreviation)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
