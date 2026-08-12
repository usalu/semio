//! 🔺️ `replace-path` — sparse diff construction; a no-op when `at` does not resolve to a `Path`.

use super::mutation::ReplacePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{diff_at_path, node_at, DrawNodeDiff, DrawPathDiff, SemioDrawingDiff};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePath, base: &SemioDrawingSnapshot) -> SemioDrawingDiff {
    match node_at(base, &payload.at) {
        Some(DrawNode::Path { .. }) => diff_at_path(&payload.at, DrawNodeDiff::Path(DrawPathDiff { segments: Some(payload.new_segments.clone()), style: None })),
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️Diff
