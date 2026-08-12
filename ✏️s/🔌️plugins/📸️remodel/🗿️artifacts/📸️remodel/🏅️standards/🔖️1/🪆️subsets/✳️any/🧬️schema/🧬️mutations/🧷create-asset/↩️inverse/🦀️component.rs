//! ↩️ Inverse for `CreateAsset` — an overwrite's inverse is "recreate the OLD value" (same verb);
//! a fresh key's inverse is `delete-asset`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateAsset, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.assets.get(&payload.key) {
        Some(old) => vec![super::mutation::create_asset(payload.key.clone(), old.clone())],
        None => vec![crate::artifacts::remodel::mutations::delete_asset::mutation::delete_asset(payload.key.clone())],
    }
}
//#endregion 🔖️Inverse
