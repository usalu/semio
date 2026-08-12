//! ↩️ Inverse for `DeleteAsset` — recreates the captured BASE `ImageAsset`. Missing key ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteAsset, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.assets.get(&payload.key) {
        Some(old) => vec![crate::artifacts::remodel::mutations::create_asset::mutation::create_asset(payload.key.clone(), old.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
