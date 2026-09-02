//! ↩️ Inverse for `CreateAsset` — an overwrite's inverse is "recreate the OLD value" (same verb);
//! a fresh key's inverse is `delete-asset`. The OLD value is now a composed CHILD handle on
//! `base.assets`, so reconstructing the payload's real `ImageAsset` bytes reads through
//! `remodeling_asset`'s working-scene cache (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) —
//! a `thread_local!`, reachable from this pure `base: &RemodelingSnapshot` function without threading any
//! session context through the signature. **Staleness gap, documented honestly** (matches every prior
//! exemplar): a cold cache (the old asset was never minted in THIS process — e.g. loaded fresh from a
//! persisted document) makes this inverse honestly `Vec::new()` rather than fabricate replacement
//! bytes, mirroring `💠️lowpoly`'s own `CreateMesh`/`DeleteMesh` inverse precedent for the identical gap.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::{remodeling_asset, RemodelingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateAsset, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.assets.get(&payload.key) {
        Some(_) => match remodeling_asset(base, &payload.key) {
            Some(old) => vec![super::create_asset(payload.key.clone(), old)],
            None => Vec::new(),
        },
        None => vec![crate::artifacts::remodeling::mutations::delete_asset::delete_asset(payload.key.clone())],
    }
}
//#endregion 🔖️Inverse
