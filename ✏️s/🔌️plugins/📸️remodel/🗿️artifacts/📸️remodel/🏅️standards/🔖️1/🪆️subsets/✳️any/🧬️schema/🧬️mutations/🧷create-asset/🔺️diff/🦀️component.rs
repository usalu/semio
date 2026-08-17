//! 🔺️ Sparse diff builder for `CreateAsset` — `RemodelDiff.assets` REPLACES the whole map on apply
//! (see `🔺️diff/📝️text/🦀️component.rs`'s `MutationDiff::apply`), so this clones `base.assets` and
//! inserts the one key rather than emitting a single-entry map. `payload.asset` (real `ImageAsset`
//! bytes, the mutation-payload shape — UNCHANGED per ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) is minted into a composed `s.stdio.semio.image`
//! CHILD handle via `mint_and_stash_asset` (real content, working-scene cache) — that handle, not the
//! raw asset, is what lands in the document's `assets` map. Deliberately NOT `mutation.duplicate-id`
//! on an existing key: this is the only asset write path in the app and import handlers rely on
//! upsert-on-retry (see the mutation leaf's own docstring) — rejecting an existing key would break a
//! retried import.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::{mint_and_stash_asset, RemodelSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateAsset, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    let mut assets = base.assets.clone();
    assets.insert(payload.key.clone(), mint_and_stash_asset(&payload.key, &payload.asset));
    protocol::MutationOutcome::new(RemodelDiff { assets: Some(assets), ..Default::default() })
}
//#endregion 🔖️Diff
