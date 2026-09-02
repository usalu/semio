//! 🔺️ Sparse diff builder for `RemoveStreamFrame`. A missing stream or an out-of-range index ⇒
//! Error `mutation.target-missing`.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveStreamFrame, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.frame_index as usize >= stream.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" has no frame at index {}.", payload.id, payload.frame_index), [payload.id.clone()]);
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.frames.remove(payload.frame_index as usize);
    }
    protocol::MutationOutcome::new(RemodelingDiff { streams: Some(RemodelingMediaStreamList { values: streams }), ..Default::default() })
}
//#endregion 🔖️Diff
