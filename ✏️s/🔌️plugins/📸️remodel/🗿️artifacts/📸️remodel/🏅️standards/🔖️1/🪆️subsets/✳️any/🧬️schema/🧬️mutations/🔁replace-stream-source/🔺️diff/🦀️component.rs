//! 🔺️ Sparse diff builder for `ReplaceStreamSource`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceStreamSource, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.streams.iter().any(|stream| stream.id == payload.id) {
        return RemodelDiff::default();
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.source = payload.source.clone();
    }
    RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() }
}
//#endregion 🔖️Diff
