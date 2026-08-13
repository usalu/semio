//! ↩️ Inverse for `DeleteAsset` — recreates the captured BASE `ImageAsset` (real bytes, read through
//! `remodel_asset`'s working-scene cache — `base.assets` now holds a composed CHILD handle, not
//! embedded bytes; see `create-asset/↩️inverse`'s doc comment for the identical staleness gap). Missing
//! key OR cold cache ⇒ `Vec::new()`, never fabricated.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{remodel_asset, RemodelSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteAsset, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match remodel_asset(&base.assets, &payload.key) {
        Some(old) => vec![crate::artifacts::remodel::mutations::create_asset::mutation::create_asset(payload.key.clone(), old)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
