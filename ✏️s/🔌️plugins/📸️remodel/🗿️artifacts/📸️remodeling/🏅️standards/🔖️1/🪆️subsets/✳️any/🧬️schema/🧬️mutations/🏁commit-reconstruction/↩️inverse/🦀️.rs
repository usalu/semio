//! ↩️ Inverse for the atomic reconstruction commit — the BASE job and every BASE result lane first, so
//! the restored document no longer references a committed raster, then one asset step per raster the
//! commit published: `create-asset` of the BASE payload where the key was overwritten, `delete-asset`
//! (which drops the durable leaf with the entry) where the commit minted it. The sparse cloud's and
//! the mesh's own durable leaves are the one lane no verb in this vocabulary can drop — they are
//! addressed by content id rather than by an `assets` key — which is why a commit whose `sparse`/`mesh`
//! staged leaves are new is not fully invertible; every committed vector of this kind is a refusal, so
//! nothing pins that gap as if it were the law.
use crate::mutations::{create_asset, delete_asset, replace_geo_products, replace_job, replace_mesh_result, replace_qc, replace_sparse, replace_trajectory, RemodelingMutation};
use crate::{remodeling_asset, RemodelingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::CommitReconstruction, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let mut steps = vec![
        replace_job(base.job.clone()),
        replace_sparse(base.results.sparse.clone()),
        replace_trajectory(base.results.trajectory.clone()),
        replace_mesh_result(Box::new(base.results.mesh.clone())),
        replace_geo_products(base.results.geo.clone()),
        replace_qc(base.results.qc.clone()),
    ];
    for committed in &payload.assets {
        match remodeling_asset(base, &committed.id) {
            Some(old) => steps.push(create_asset(committed.id.clone(), old)),
            None => steps.push(delete_asset(committed.id.clone())),
        }
    }
    steps
}
//#endregion 🔖️Inverse
