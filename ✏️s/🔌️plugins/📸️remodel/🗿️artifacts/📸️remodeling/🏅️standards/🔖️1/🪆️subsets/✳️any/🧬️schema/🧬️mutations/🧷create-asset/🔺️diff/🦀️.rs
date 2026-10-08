//! 🔺️ Sparse diff builder for `CreateAsset` — `RemodelingDiff.assets` REPLACES the whole map on apply
//! (see `🔺️diff/📝️text/🦀️.rs`'s `MutationDiff::apply`), so this clones `base.assets` and
//! inserts the one key rather than emitting a single-entry map. `payload.asset` (real `ImageAsset`
//! bytes, the mutation-payload shape — UNCHANGED per ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) is minted into a composed `s.stdio.semio.image`
//! CHILD handle via `store_remodeling_asset` (real content, working-scene cache) — that handle, not the
//! raw asset, is what lands in the document's `assets` map. Deliberately NOT `mutation.duplicate-id`
//! on an existing key: this is the only asset write path in the app and import handlers rely on
//! upsert-on-retry (see the mutation leaf's own docstring) — rejecting an existing key would break a
//! retried import. An upsert also DROPS the durable leaf the overwritten handle owned, so the store
//! never accumulates a leaf nothing addresses and `delete-asset` (which drops the leaf it minted) is
//! this verb's exact inverse in both directions.
use crate::diff::{RemodelingAssetEntry, RemodelingContentDelta, RemodelingContentRow, RemodelingDiff, RemodelingRow, RemodelingRows};
use crate::{durable_remodeling_asset, store_remodeling_asset, RemodelingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateAsset, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let handle = store_remodeling_asset(&payload.key, &payload.asset);
    let Some(artifact) = durable_remodeling_asset(&payload.asset) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The asset payload is malformed or exceeds its exact bounded envelope.", [payload.key.clone()]);
    };
    let previous = base.assets.get(&payload.key);
    let mut content = Vec::new();
    let mut released = None;
    if let Some(previous) = previous.filter(|previous| base.durable_artifacts.contains_key(&previous.child_id)) {
        content.push(RemodelingContentRow::Truncate { id: previous.child_id.clone(), from: 0 });
        released = Some(&previous.child_id);
    }
    match base.durable_artifacts.get(&handle.child_id) {
        Some(held) if released != Some(&handle.child_id) && held == &artifact => {}
        Some(_) if released != Some(&handle.child_id) => {
            content.push(RemodelingContentRow::Truncate { id: handle.child_id.clone(), from: 0 });
            content.push(RemodelingContentDelta::create(&handle.child_id, &artifact));
        }
        _ => content.push(RemodelingContentDelta::create(&handle.child_id, &artifact)),
    }
    let entry = RemodelingAssetEntry { key: payload.key.clone(), child: handle };
    let row = if previous.is_some() { RemodelingRow::Replace { entity: entry } } else { RemodelingRow::Insert { entity: entry } };
    protocol::MutationOutcome::new(RemodelingDiff { assets: Some(RemodelingRows { rows: vec![row] }), durable_artifacts: (!content.is_empty()).then_some(RemodelingContentDelta { rows: content }), ..Default::default() })
}
//#endregion 🔖️Diff
