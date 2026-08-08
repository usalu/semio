//! 🔺️ Diff fragment yielded by `SetSnapshot`.
use crate::artifacts::draw::diff::DrawDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff produced by one `SetSnapshot` mutation — a sparse [`DrawDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetSnapshotDiff {
    pub diff: DrawDiff,
}

impl SetSnapshotDiff {
    pub fn from_diff(diff: DrawDiff) -> Self {
        Self { diff }
    }

    pub fn into_draw_diff(self) -> DrawDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
