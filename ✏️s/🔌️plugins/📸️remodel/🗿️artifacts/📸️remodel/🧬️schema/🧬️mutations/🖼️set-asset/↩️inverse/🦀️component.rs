//! ↩️ Inverse for `SetAsset`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot, key: &str) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetAsset { key: key.to_string(), value: base.assets.get(key).cloned() }]
}
//#endregion 🔖️Inverse
