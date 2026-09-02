//! ↩️ Inverse for the atomic reconstruction commit.
use crate::artifacts::remodeling::mutations::{replace_geo_products, replace_job, replace_mesh_result, replace_qc, replace_sparse, replace_trajectory, RemodelingMutation};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::CommitReconstruction, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![
        replace_job(base.job.clone()),
        replace_sparse(base.results.sparse.clone()),
        replace_trajectory(base.results.trajectory.clone()),
        replace_mesh_result(Box::new(base.results.mesh.clone())),
        replace_geo_products(base.results.geo.clone()),
        replace_qc(base.results.qc.clone()),
    ]
}
//#endregion 🔖️Inverse
