//! 🔺️ Atomic diff for a reconstruction result. Every durable content handle the payload names must be
//! complete in the BASE document: a `remodeling-content:` sparse buffer ⇒ otherwise
//! `mutation.invalid-reconstruction-sparse`; a `remodeling-mesh-content:` mesh ⇒ otherwise
//! `mutation.invalid-reconstruction-mesh`; a bound asset content id ⇒ otherwise
//! `mutation.invalid-reconstruction-asset`. Inline buffers and constant meshes are plain values. A payload
//! equal to the stored lanes and bindings ⇒ Warning `mutation.no-op`.
use crate::diff::RemodelingDiff;
use crate::{committed_remodeling_asset_handle, remodeling_content_handle_parts, remodeling_content_is_complete, remodeling_mesh_content_handle_parts, RemodelingContentKind, RemodelingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::CommitReconstruction, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if let Some((content_id, chunk_count)) = payload.sparse.as_ref().and_then(|sparse| remodeling_content_handle_parts(&sparse.points.0)) {
        if !remodeling_content_is_complete(&base.durable_artifacts, content_id, RemodelingContentKind::Sparse, chunk_count) {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-sparse", "The sparse cloud names durable content that is not complete.", ["sparse".to_string()]);
        }
    }
    if let Some((content_id, chunk_count)) = payload.mesh.as_ref().and_then(|mesh| remodeling_mesh_content_handle_parts(&mesh.mesh)) {
        if !remodeling_content_is_complete(&base.durable_artifacts, content_id, RemodelingContentKind::Mesh, chunk_count) {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-mesh", "The mesh names durable content that is not complete.", [content_id.to_string()]);
        }
    }
    let mut assets = base.assets.clone();
    for binding in &payload.assets {
        match &binding.content_id {
            Some(content_id) => {
                let complete = base.durable_artifacts.get(content_id).is_some_and(|artifact| artifact.kind == RemodelingContentKind::Image.wire() && remodeling_content_is_complete(&base.durable_artifacts, content_id, RemodelingContentKind::Image, artifact.chunks.len() as u64));
                if !complete {
                    return protocol::MutationOutcome::error("mutation.invalid-reconstruction-asset", "A bound asset names durable image content that is not complete.", [binding.id.clone()]);
                }
                assets.insert(binding.id.clone(), committed_remodeling_asset_handle(&binding.id, content_id));
            }
            None => {
                assets.remove(&binding.id);
            }
        }
    }
    let mut results = base.results.clone();
    results.sparse = payload.sparse.clone();
    results.trajectory = payload.trajectory.clone();
    results.geo = payload.geo.clone();
    results.qc = payload.qc.clone();
    if let Some(mesh) = &payload.mesh {
        results.mesh = (**mesh).clone();
    }
    if results == base.results && assets == base.assets {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The reconstruction result is already committed.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { assets: (assets != base.assets).then_some(assets), results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
