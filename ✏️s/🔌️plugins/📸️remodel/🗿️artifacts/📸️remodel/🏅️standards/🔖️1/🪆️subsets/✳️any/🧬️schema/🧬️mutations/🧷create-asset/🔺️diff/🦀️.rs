//! 🔺️ Sparse diff builder for `CreateAsset` — `RemodelDiff.assets` REPLACES the whole map on apply
//! (see `🔺️diff/📝️text/🦀️.rs`'s `MutationDiff::apply`), so this clones `base.assets` and
//! inserts the one key rather than emitting a single-entry map. `payload.asset` (real `ImageAsset`
//! bytes, the mutation-payload shape — UNCHANGED per ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) is minted into a composed `s.stdio.semio.image`
//! CHILD handle via `store_remodel_asset` (real content, working-scene cache) — that handle, not the
//! raw asset, is what lands in the document's `assets` map. Deliberately NOT `mutation.duplicate-id`
//! on an existing key: this is the only asset write path in the app and import handlers rely on
//! upsert-on-retry (see the mutation leaf's own docstring) — rejecting an existing key would break a
//! retried import.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::{durable_remodel_asset, store_remodel_asset, RemodelSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateAsset, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if let Some((kind, staging_id, index)) = crate::artifacts::remodel::remodel_asset_stage_parts(&payload.key) {
        match crate::artifacts::remodel::stage_remodel_asset_chunk(staging_id, kind, index, &payload.asset.data) {
            Ok(()) => return protocol::MutationOutcome::new(RemodelDiff::default()),
            Err(crate::artifacts::remodel::RemodelStagingFault::Busy) => {
                return protocol::MutationOutcome::error("mutation.asset-staging-busy", "Replayable asset staging is at its bounded capacity.".into(), [payload.key.clone()]);
            }
            Err(crate::artifacts::remodel::RemodelStagingFault::Invalid) => {}
        }
        return protocol::MutationOutcome::error("mutation.invalid-asset-chunk", "The staged asset chunk is invalid.".into(), [payload.key.clone()]);
    }
    if let Some((staging_id, index)) = crate::artifacts::remodel::remodel_mesh_stage_asset_parts(&payload.key) {
        match crate::artifacts::remodel::stage_remodel_mesh_chunk(staging_id, index, &payload.asset.data) {
            Ok(()) => return protocol::MutationOutcome::new(RemodelDiff::default()),
            Err(crate::artifacts::remodel::RemodelStagingFault::Busy) => {
                return protocol::MutationOutcome::error("mutation.mesh-staging-busy", "Replayable mesh staging is at its bounded capacity.".into(), [payload.key.clone()]);
            }
            Err(crate::artifacts::remodel::RemodelStagingFault::Invalid) => {}
        }
        return protocol::MutationOutcome::error("mutation.invalid-mesh-chunk", "The staged mesh chunk is invalid.".into(), [payload.key.clone()]);
    }
    if crate::artifacts::remodel::remodel_asset_content_handle_parts(&payload.asset.data).is_some() {
        return protocol::MutationOutcome::error("mutation.invalid-asset-payload", "Private reconstruction staging handles are accepted only by CommitReconstruction.".into(), [payload.key.clone()]);
    }
    let mut assets = base.assets.clone();
    let handle = store_remodel_asset(&payload.key, &payload.asset);
    let Some(artifact) = durable_remodel_asset(&payload.asset) else {
        return protocol::MutationOutcome::error("mutation.invalid-asset-payload", "The asset payload is malformed or exceeds its exact bounded envelope.".into(), [payload.key.clone()]);
    };
    let mut durable_artifacts = base.durable_artifacts.clone();
    durable_artifacts.insert(handle.child_id.clone(), artifact);
    assets.insert(payload.key.clone(), handle);
    protocol::MutationOutcome::new(RemodelDiff { assets: Some(assets), durable_artifacts: Some(durable_artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
