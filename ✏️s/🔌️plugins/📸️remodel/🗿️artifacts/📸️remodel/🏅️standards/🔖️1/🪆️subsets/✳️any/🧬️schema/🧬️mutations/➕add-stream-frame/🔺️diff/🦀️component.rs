//! 🔺️ Sparse diff builder for `AddStreamFrame`. Missing target ⇒ `RemodelDiff::default()` — the
//! idempotent-early-return idiom (`📋️forms`'s `➕add-step/🔺️diff` uses the same shape).
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddStreamFrame, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.streams.iter().any(|stream| stream.id == payload.id) {
        return RemodelDiff::default();
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.frames.push(payload.frame.clone());
        stream.kind = payload.kind;
    }
    RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() }
}
//#endregion 🔖️Diff
