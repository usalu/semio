//! 🔺️ Sparse diff builder for `DeleteAsset`. A missing key ⇒ Error `mutation.target-missing`; an
//! asset any stream frame, the mesh texture or a geo product still names ⇒ Error
//! `mutation.referenced` — the same ownership rule `delete-stream` and `delete-camera-calibration`
//! follow, so the document never keeps a reference to a leaf that is gone. The accepted branch drops
//! the durable leaf the handle owned together with the `assets` entry, which is what makes
//! `create-asset` its exact inverse.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteAsset, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if let Some((_, staging_id, _)) = crate::artifacts::remodeling::remodeling_asset_stage_parts(&payload.key) {
        crate::artifacts::remodeling::discard_staged_remodeling_asset(staging_id);
        return protocol::MutationOutcome::new(RemodelingDiff::default());
    }
    if let Some((staging_id, _)) = crate::artifacts::remodeling::remodeling_mesh_stage_asset_parts(&payload.key) {
        crate::artifacts::remodeling::discard_staged_remodeling_mesh(staging_id);
        return protocol::MutationOutcome::new(RemodelingDiff::default());
    }
    if !base.assets.contains_key(&payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.key), [payload.key.clone()]);
    }
    let mut referencing: Vec<String> = base.streams.iter().filter(|stream| stream.frames.iter().any(|frame| frame.asset_id == payload.key)).map(|stream| stream.id.clone()).collect();
    if base.results.mesh.texture_asset_id.as_deref() == Some(payload.key.as_str()) {
        referencing.push("results.mesh.textureAssetId".to_string());
    }
    if let Some(geo) = &base.results.geo {
        for (lane, asset_id) in [("dsm", &geo.dsm_asset_id), ("dtm", &geo.dtm_asset_id), ("ortho", &geo.ortho_asset_id)] {
            if asset_id.as_deref() == Some(payload.key.as_str()) {
                referencing.push(format!("results.geo.{lane}AssetId"));
            }
        }
    }
    if !referencing.is_empty() {
        return protocol::MutationOutcome::error("mutation.referenced", format!("Asset \"{}\" is still referenced by {} place(s) in the document.", payload.key, referencing.len()), referencing);
    }
    let mut assets = base.assets.clone();
    let mut durable_artifacts = base.durable_artifacts.clone();
    if let Some(handle) = assets.remove(&payload.key) {
        durable_artifacts.remove(&handle.child_id);
    }
    protocol::MutationOutcome::new(RemodelingDiff { assets: Some(assets), durable_artifacts: Some(durable_artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
