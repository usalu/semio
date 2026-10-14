//! 🔺️ Sparse diff builder for `RemoveStreamFrame`. A missing stream or an out-of-range index ⇒
//! Error `mutation.target-missing`.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::MediaStreamPatch;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingMembers;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveStreamFrame, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.frame_index as usize >= stream.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" has no frame at index {}.", payload.id, payload.frame_index), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::stream_rows(vec![RemodelingRow::Patch { key: payload.id.clone(), patch: MediaStreamPatch { frames: Some(RemodelingMembers { removed: vec![stream.frames[payload.frame_index as usize].clone()], added: Vec::new() }), ..Default::default() } }]))
}
//#endregion 🔖️Diff
