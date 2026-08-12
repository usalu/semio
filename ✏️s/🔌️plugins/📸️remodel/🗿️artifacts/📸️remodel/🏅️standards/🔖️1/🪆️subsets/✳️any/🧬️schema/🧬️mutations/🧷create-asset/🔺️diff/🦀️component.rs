//! 🔺️ Sparse diff builder for `CreateAsset` — `RemodelDiff.assets` REPLACES the whole map on apply
//! (see `🔺️diff/📝️text/🦀️component.rs`'s `MutationDiff::apply`), so this clones `base.assets` and
//! inserts the one key rather than emitting a single-entry map.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateAsset, base: &RemodelSnapshot) -> RemodelDiff {
    let mut assets = base.assets.clone();
    assets.insert(payload.key.clone(), payload.asset.clone());
    RemodelDiff { assets: Some(assets), ..Default::default() }
}
//#endregion 🔖️Diff
