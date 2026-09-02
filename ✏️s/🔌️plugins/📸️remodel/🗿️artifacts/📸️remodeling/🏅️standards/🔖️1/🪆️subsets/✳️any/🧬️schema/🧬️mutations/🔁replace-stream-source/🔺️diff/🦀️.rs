//! 🔺️ Sparse diff builder for `ReplaceStreamSource`. A missing stream ⇒ Error
//! `mutation.target-missing`.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceStreamSource, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !base.streams.iter().any(|stream| stream.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.source = payload.source.clone();
    }
    protocol::MutationOutcome::new(RemodelingDiff { streams: Some(RemodelingMediaStreamList { values: streams }), ..Default::default() })
}
//#endregion 🔖️Diff
