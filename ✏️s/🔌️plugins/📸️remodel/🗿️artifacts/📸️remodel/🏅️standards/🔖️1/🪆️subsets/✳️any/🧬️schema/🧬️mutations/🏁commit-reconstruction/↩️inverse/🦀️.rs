//! ↩️ Inverse for the atomic reconstruction commit.
use crate::artifacts::remodel::mutations::{replace_geo_products, replace_job, replace_mesh_result, replace_qc, replace_sparse, replace_trajectory, RemodelMutation};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::CommitReconstruction, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
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
