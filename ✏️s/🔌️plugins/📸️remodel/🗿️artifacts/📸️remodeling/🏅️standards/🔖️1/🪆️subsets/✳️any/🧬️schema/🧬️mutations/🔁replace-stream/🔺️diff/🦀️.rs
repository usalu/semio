//! 🔺️ Sparse diff builder for `ReplaceStreamSource`. A missing stream ⇒ Error
//! `mutation.target-missing`.
use crate::diff::{RemodelingDiff, RemodelingRow, MediaStreamPatch, RemodelingAssigned};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceStreamSource, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if stream.source == payload.source {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Stream \"{}\" already has this source.", payload.id));
    }
    protocol::MutationOutcome::new(RemodelingDiff::stream_rows(vec![RemodelingRow::Patch { key: payload.id.clone(), patch: MediaStreamPatch { source: Some(RemodelingAssigned::new(payload.source.clone())), ..Default::default() } }]))
}
//#endregion 🔖️Diff
