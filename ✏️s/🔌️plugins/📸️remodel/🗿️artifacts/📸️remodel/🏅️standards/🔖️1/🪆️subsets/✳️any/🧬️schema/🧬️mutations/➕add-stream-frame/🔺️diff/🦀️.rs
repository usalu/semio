//! 🔺️ Sparse diff builder for `AddStreamFrame`. A missing owner stream ⇒ Error
//! `mutation.target-missing`; an already-present exact frame ⇒ Warning `mutation.no-op`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelMediaStreamList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddStreamFrame, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if stream.frames.iter().any(|frame| *frame == payload.frame) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Stream \"{}\" already has frame {}.", payload.id, payload.frame.index));
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.frames.push(payload.frame.clone());
        stream.kind = payload.kind;
    }
    protocol::MutationOutcome::new(RemodelDiff { streams: Some(RemodelMediaStreamList { values: streams }), ..Default::default() })
}
//#endregion 🔖️Diff
