//! ↩️ Inverse for `ReplaceMeshResult` — the OLD `RemodelMesh` from BASE, boxed.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceMeshResult, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::replace_mesh_result(Box::new(base.results.mesh.clone()))]
}
//#endregion 🔖️Inverse
