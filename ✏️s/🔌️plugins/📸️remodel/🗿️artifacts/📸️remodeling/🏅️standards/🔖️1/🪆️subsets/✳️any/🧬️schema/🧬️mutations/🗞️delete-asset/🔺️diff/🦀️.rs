//! 🔺️ Sparse diff builder for `DeleteAsset`. A missing key ⇒ Error `mutation.target-missing`;
//! stale references left dangling elsewhere (stream frames, mesh texture, geo products) ⇒ Info
//! `mutation.cascade` (reported only — this leaf never rewrites those references, `delete-asset` has
//! no call site that removes an in-use asset today).
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
    let mut assets = base.assets.clone();
    assets.remove(&payload.key);
    let mut stale_refs = base.streams.iter().flat_map(|stream| stream.frames.iter()).filter(|frame| frame.asset_id == payload.key).count();
    if base.results.mesh.texture_asset_id.as_deref() == Some(payload.key.as_str()) {
        stale_refs += 1;
    }
    if let Some(geo) = &base.results.geo {
        stale_refs += [&geo.dsm_asset_id, &geo.dtm_asset_id, &geo.ortho_asset_id].into_iter().filter(|asset_id| asset_id.as_deref() == Some(payload.key.as_str())).count();
    }
    let outcome = protocol::MutationOutcome::new(RemodelingDiff { assets: Some(assets), ..Default::default() });
    if stale_refs == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting asset \"{}\" leaves {stale_refs} stale reference(s) elsewhere in the document.", payload.key))
    }
}
//#endregion 🔖️Diff
