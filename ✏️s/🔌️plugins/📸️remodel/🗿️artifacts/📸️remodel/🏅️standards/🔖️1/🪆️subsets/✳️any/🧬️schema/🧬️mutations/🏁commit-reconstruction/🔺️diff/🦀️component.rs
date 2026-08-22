//! 🔺️ Atomic diff for a completed reconstruction.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::{committed_remodel_asset_handle, durable_staged_remodel_asset, durable_staged_remodel_mesh, RemodelSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CommitReconstruction, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    let mut staged_assets = Vec::with_capacity(payload.assets.len().saturating_add(1));
    if let Some(sparse) = payload.sparse.as_ref().filter(|sparse| !sparse.points.0.is_empty()) {
        let Some((content_id, staging_id, chunk_count)) = crate::artifacts::remodel::remodel_asset_content_handle_parts(&sparse.points.0) else {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-sparse", "The terminal sparse cloud is not a replayable content handle.".into(), ["sparse".to_string()]);
        };
        staged_assets.push((staging_id, content_id, chunk_count, crate::artifacts::remodel::RemodelAssetContentKind::Sparse));
    }
    for committed in &payload.assets {
        let Some((content_id, staging_id, chunk_count)) = crate::artifacts::remodel::remodel_asset_content_handle_parts(&committed.asset.data) else {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-asset", "A terminal asset is not a replayable content handle.".into(), [committed.id.clone()]);
        };
        staged_assets.push((staging_id, content_id, chunk_count, crate::artifacts::remodel::RemodelAssetContentKind::Raster));
    }
    let mut results = base.results.clone();
    results.sparse = payload.sparse.clone();
    results.trajectory = payload.trajectory.clone();
    results.geo = payload.geo.clone();
    results.qc = payload.qc.clone();
    let mut staged_mesh = None;
    if let Some(candidate) = payload.mesh.as_ref() {
        let mut mesh = (**candidate).clone();
        let Some(staging_id) = mesh.mesh.target.artifact_id.strip_prefix("mesh-stage:") else {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-mesh", "The terminal mesh is not a staged replayable handle.".into(), [mesh.mesh.child_id.clone()]);
        };
        let chunk_count = crate::artifacts::remodel::staged_remodel_mesh_chunk_count(staging_id);
        staged_mesh = Some((staging_id.to_string(), mesh.mesh.child_id.clone(), chunk_count));
        mesh.mesh = crate::artifacts::remodel::replayable_remodel_mesh_handle(&mesh.mesh.child_id, staging_id, chunk_count);
        results.mesh = mesh;
    }
    let mut assets = base.assets.clone();
    let mut durable_artifacts = base.durable_artifacts.clone();
    if let Some((staging_id, content_id, _, _)) = staged_assets.iter().find(|(_, _, _, kind)| *kind == crate::artifacts::remodel::RemodelAssetContentKind::Sparse) {
        let Some(artifact) = durable_staged_remodel_asset(staging_id, "sparse", None, 0, 0) else {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-sparse", "The staged sparse cloud could not be copied into document-owned durable leaves.".into(), ["sparse".to_string()]);
        };
        durable_artifacts.insert((*content_id).into(), artifact);
    }
    for committed in &payload.assets {
        let Some((content_id, staging_id, _)) = crate::artifacts::remodel::remodel_asset_content_handle_parts(&committed.asset.data) else {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-asset", "A terminal asset is not a replayable content handle.".into(), [committed.id.clone()]);
        };
        let handle = committed_remodel_asset_handle(&committed.id, content_id);
        let Some(artifact) = durable_staged_remodel_asset(staging_id, "image", Some(committed.asset.mime.clone()), committed.asset.width, committed.asset.height) else {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-asset", "A committed raster could not be copied into document-owned durable leaves.".into(), [committed.id.clone()]);
        };
        durable_artifacts.insert(handle.child_id.clone(), artifact);
        assets.insert(committed.id.clone(), handle);
    }
    if let Some((staging_id, content_id, _)) = staged_mesh.as_ref() {
        let Some(artifact) = durable_staged_remodel_mesh(staging_id) else {
            return protocol::MutationOutcome::error("mutation.invalid-reconstruction-mesh", "The committed mesh could not be copied into document-owned durable leaves.".into(), [content_id.clone()]);
        };
        durable_artifacts.insert(content_id.clone(), artifact);
    }
    if !crate::artifacts::remodel::commit_staged_remodel_reconstruction(&staged_assets, staged_mesh.as_ref().map(|(staging_id, content_id, chunk_count)| (staging_id.as_str(), content_id.as_str(), *chunk_count))) {
        return protocol::MutationOutcome::error("mutation.invalid-reconstruction-staging", "The terminal artifacts failed their aggregate validate-all commit.".into(), [base.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelDiff { assets: Some(assets), durable_artifacts: Some(durable_artifacts), job: Some(payload.job.clone()), results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
