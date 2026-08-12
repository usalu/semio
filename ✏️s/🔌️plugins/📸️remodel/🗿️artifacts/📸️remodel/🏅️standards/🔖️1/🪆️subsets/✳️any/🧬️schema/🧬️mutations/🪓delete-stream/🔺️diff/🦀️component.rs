//! 🔺️ Sparse diff builder for `DeleteStream`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteStream, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.streams.iter().any(|stream| stream.id == payload.id) {
        return RemodelDiff::default();
    }
    let streams: Vec<_> = base.streams.iter().filter(|stream| stream.id != payload.id).cloned().collect();
    RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() }
}
//#endregion 🔖️Diff
