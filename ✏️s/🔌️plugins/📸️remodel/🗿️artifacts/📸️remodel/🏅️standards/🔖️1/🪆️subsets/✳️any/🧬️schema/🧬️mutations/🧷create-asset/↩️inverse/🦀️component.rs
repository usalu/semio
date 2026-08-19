//! ↩️ Inverse for `CreateAsset` — an overwrite's inverse is "recreate the OLD value" (same verb);
//! a fresh key's inverse is `delete-asset`. The OLD value is now a composed CHILD handle on
//! `base.assets`, so reconstructing the payload's real `ImageAsset` bytes reads through
//! `remodel_asset`'s working-scene cache (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) —
//! a `thread_local!`, reachable from this pure `base: &RemodelSnapshot` function without threading any
//! session context through the signature. **Staleness gap, documented honestly** (matches every prior
//! exemplar): a cold cache (the old asset was never minted in THIS process — e.g. loaded fresh from a
//! persisted document) makes this inverse honestly `Vec::new()` rather than fabricate replacement
//! bytes, mirroring `💠️lowpoly`'s own `CreateMesh`/`DeleteMesh` inverse precedent for the identical gap.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{remodel_asset, RemodelSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateAsset, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.assets.get(&payload.key) {
        Some(_) => match remodel_asset(&base.assets, &payload.key) {
            Some(old) => vec![super::mutation::create_asset(payload.key.clone(), old)],
            None => Vec::new(),
        },
        None => vec![crate::artifacts::remodel::mutations::delete_asset::mutation::delete_asset(payload.key.clone())],
    }
}
//#endregion 🔖️Inverse
