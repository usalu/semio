//! 🔺️ Sparse diff builder for `ChangeStreamSync`. One guard order for the whole vocabulary —
//! target-missing ⇒ Error, then the invariant (a non-finite offset) ⇒ Fatal, then the identical
//! resubmission ⇒ Warning: a malformed argument is a fault whether or not it happens to match what is
//! already stored.
use crate::diff::{RemodelingDiff, RemodelingRow, MediaStreamPatch};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeStreamSync, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(existing) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_sync_offset_ms.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Stream \"{}\" sync offset must be finite, got {}.", payload.id, payload.new_sync_offset_ms), [payload.id.clone()]);
    }
    if existing.sync_offset_ms == payload.new_sync_offset_ms {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Stream \"{}\" sync offset is already {}ms.", payload.id, payload.new_sync_offset_ms));
    }
    protocol::MutationOutcome::new(RemodelingDiff::stream_rows(vec![RemodelingRow::Patch { key: payload.id.clone(), patch: MediaStreamPatch { sync_offset_ms: Some(payload.new_sync_offset_ms), ..Default::default() } }]))
}
//#endregion 🔖️Diff
