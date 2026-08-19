//! ↩️ Inverse for `CreateEdge` — always a `delete-edge` of the id it created.
use crate::artifacts::jack::mutations::{delete_edge, TrinityGraphMutation};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateEdge, _base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    vec![delete_edge(payload.edge.id.clone())]
}
//#endregion 🔖️Inverse
