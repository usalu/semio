//! ↩️ Inverse for `SetAsset`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection, key: &str) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetAsset { key: key.to_string(), value: base.assets.get(key).cloned() }]
}
//#endregion 🔖️Inverse
