//! ↩️ Inverse for `DeleteAsset` — recreates the captured BASE `ImageAsset` (real bytes, read through
//! `remodeling_asset`'s working-scene cache — `base.assets` now holds a composed CHILD handle, not
//! embedded bytes; see `create-asset/↩️inverse`'s doc comment for the identical staleness gap). Missing
//! key OR cold cache ⇒ `Vec::new()`, never fabricated.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::{remodeling_asset, RemodelingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteAsset, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match remodeling_asset(base, &payload.key) {
        Some(old) => vec![crate::artifacts::remodeling::mutations::create_asset::create_asset(payload.key.clone(), old)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
