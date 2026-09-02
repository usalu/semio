//! 🔺️ Sparse diff builder for `CreateStream` — a real append-only insert (never a whole-snapshot
//! capture). Duplicate `stream.id` ⇒ Fatal `mutation.duplicate-id`; a `camera_id` referencing an
//! unknown camera ⇒ Fatal `mutation.invariant`.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateStream, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.streams.iter().any(|stream| stream.id == payload.stream.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A stream with id \"{}\" already exists.", payload.stream.id), [payload.stream.id.clone()]);
    }
    if let Some(camera_id) = &payload.stream.camera_id {
        if !base.calibration.cameras.iter().any(|camera| &camera.id == camera_id) {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Stream \"{}\" references unknown camera \"{}\".", payload.stream.id, camera_id), [payload.stream.id.clone()]);
        }
    }
    let mut streams = base.streams.clone();
    streams.push(payload.stream.clone());
    protocol::MutationOutcome::new(RemodelingDiff { streams: Some(RemodelingMediaStreamList { values: streams }), ..Default::default() })
}
//#endregion 🔖️Diff
