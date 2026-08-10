//! 🔺️ Diff fragment yielded by `SetGeoParams`.
use crate::artifacts::remodel::diff::RemodelDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `SetGeoParams` mutation — a sparse [`RemodelDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetGeoParamsDiff {
    pub diff: RemodelDiff,
}

impl SetGeoParamsDiff {
    pub fn from_diff(diff: RemodelDiff) -> Self {
        Self { diff }
    }

    pub fn into_remodel_diff(self) -> RemodelDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
