//! 🔺️ Sparse diff builder for `CreateStream` — a real append-only insert (never a whole-snapshot
//! capture). A duplicate `stream.id` (already present in `base`) is a no-op.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateStream, base: &RemodelSnapshot) -> RemodelDiff {
    if base.streams.iter().any(|stream| stream.id == payload.stream.id) {
        return RemodelDiff::default();
    }
    let mut streams = base.streams.clone();
    streams.push(payload.stream.clone());
    RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() }
}
//#endregion 🔖️Diff
