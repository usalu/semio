//! 🔺️ Sparse diff builder for `CreateStream` — a real single-member insert at the stream's canonical
//! position in `id` order (never a whole-snapshot capture), so `delete-stream` puts it back exactly
//! where it was. Duplicate `stream.id` ⇒ Fatal `mutation.duplicate-id`; a `camera_id` referencing an
//! unknown camera ⇒ Fatal `mutation.invariant`.
use crate::diff::{RemodelingDiff, RemodelingMediaStreamList};
use crate::RemodelingSnapshot;

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
    let at = crate::mutations::ordered_index(&streams, &payload.stream.id, |stream| stream.id.clone());
    streams.insert(at, payload.stream.clone());
    protocol::MutationOutcome::new(RemodelingDiff { streams: Some(RemodelingMediaStreamList { values: streams }), ..Default::default() })
}
//#endregion 🔖️Diff
