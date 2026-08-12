//! 🔺️ Sparse diff builder for `DeleteAsset`. Missing key ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteAsset, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.assets.contains_key(&payload.key) {
        return RemodelDiff::default();
    }
    let mut assets = base.assets.clone();
    assets.remove(&payload.key);
    RemodelDiff { assets: Some(assets), ..Default::default() }
}
//#endregion 🔖️Diff
