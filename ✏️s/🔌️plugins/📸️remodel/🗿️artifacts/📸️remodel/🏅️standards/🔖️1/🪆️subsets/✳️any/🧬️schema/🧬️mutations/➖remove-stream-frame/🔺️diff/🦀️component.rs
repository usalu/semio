//! 🔺️ Sparse diff builder for `RemoveStreamFrame`. Missing target or out-of-range index ⇒
//! `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveStreamFrame, base: &RemodelSnapshot) -> RemodelDiff {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return RemodelDiff::default();
    };
    if payload.frame_index as usize >= stream.frames.len() {
        return RemodelDiff::default();
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.frames.remove(payload.frame_index as usize);
    }
    RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() }
}
//#endregion 🔖️Diff
