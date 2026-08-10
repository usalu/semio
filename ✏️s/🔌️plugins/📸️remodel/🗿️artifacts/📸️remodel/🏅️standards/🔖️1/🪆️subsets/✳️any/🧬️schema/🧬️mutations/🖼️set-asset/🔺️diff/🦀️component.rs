//! 🔺️ Diff fragment yielded by `SetAsset`.
use crate::artifacts::remodel::diff::RemodelDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `SetAsset` mutation — a sparse [`RemodelDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetAssetDiff {
    pub diff: RemodelDiff,
}

impl SetAssetDiff {
    pub fn from_diff(diff: RemodelDiff) -> Self {
        Self { diff }
    }

    pub fn into_remodel_diff(self) -> RemodelDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
