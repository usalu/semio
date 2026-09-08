//! ↩️ Inverse for `ReplaceMeshResult` — the OLD `RemodelingMesh` from BASE, boxed.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ReplaceMeshResult, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::replace_mesh_result(Box::new(base.results.mesh.clone()))]
}
//#endregion 🔖️Inverse
