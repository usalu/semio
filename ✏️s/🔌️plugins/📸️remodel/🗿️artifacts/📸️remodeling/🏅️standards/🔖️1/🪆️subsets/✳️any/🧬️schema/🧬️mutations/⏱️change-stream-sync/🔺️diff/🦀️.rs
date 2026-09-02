//! 🔺️ Sparse diff builder for `ChangeStreamSync`. Missing target ⇒ Error; identical resubmission ⇒
//! Warning; non-finite offset ⇒ Fatal.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeStreamSync, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(existing) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.sync_offset_ms == payload.new_sync_offset_ms {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Stream \"{}\" sync offset is already {}ms.", payload.id, payload.new_sync_offset_ms));
    }
    if !payload.new_sync_offset_ms.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Stream \"{}\" sync offset must be finite, got {}.", payload.id, payload.new_sync_offset_ms), [payload.id.clone()]);
    }
    let mut streams = base.streams.clone();
    if let Some(stream) = streams.iter_mut().find(|stream| stream.id == payload.id) {
        stream.sync_offset_ms = payload.new_sync_offset_ms;
    }
    protocol::MutationOutcome::new(RemodelingDiff { streams: Some(RemodelingMediaStreamList { values: streams }), ..Default::default() })
}
//#endregion 🔖️Diff
