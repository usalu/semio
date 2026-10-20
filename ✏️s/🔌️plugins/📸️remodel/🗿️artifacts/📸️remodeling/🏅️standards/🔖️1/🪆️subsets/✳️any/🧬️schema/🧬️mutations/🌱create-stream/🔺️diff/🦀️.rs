//! 🔺️ Sparse diff builder for `CreateStream` — a real single-member insert at the stream's canonical
//! position in `id` order (never a whole-snapshot capture), so `delete-stream` puts it back exactly
//! where it was. Duplicate `stream.id` ⇒ Fatal `mutation.duplicate-id`; a `camera_id` naming a camera
//! this base does not calibrate ⇒ Error `mutation.target-missing`, addressed to that camera.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::create_stream::component::CreateStream;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::create_stream::component::CreateStream, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.streams.iter().any(|stream| stream.id == payload.stream.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A stream with id \"{}\" already exists.", payload.stream.id), [payload.stream.id.clone()]);
    }
    if let Some(camera_id) = &payload.stream.camera_id {
        if !base.calibration.cameras.iter().any(|camera| &camera.id == camera_id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" references unknown camera \"{}\".", payload.stream.id, camera_id), [camera_id.clone()]);
        }
    }
    protocol::MutationOutcome::new(RemodelingDiff::stream_rows(vec![RemodelingRow::Insert { entity: payload.stream.clone() }]))
}
//#endregion 🔖️Diff
