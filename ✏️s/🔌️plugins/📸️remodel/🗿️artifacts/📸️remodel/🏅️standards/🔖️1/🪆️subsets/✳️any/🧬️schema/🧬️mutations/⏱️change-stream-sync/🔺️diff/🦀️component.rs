//! 🔺️ Sparse diff builder for `ChangeStreamSync`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeStreamSync, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.streams.iter().any(|stream| stream.id == payload.id) {
        return RemodelDiff::default();
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.sync_offset_ms = payload.new_sync_offset_ms;
    }
    RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() }
}
//#endregion 🔖️Diff
