//! 🔺️ Sparse diff builder for `DeleteStream` — removes the stream and the `frames` it OWNS (they
//! travel inside the record). A GCP observation is owned by its GCP, not by the stream it addresses,
//! so a stream any observation still names is REFUSED with `mutation.target-referenced` rather than silently
//! severing another record's data: the document never carries a dangling reference and the delete
//! never destroys what it does not own, which is also what makes `create-stream` its exact inverse.
//! Missing target ⇒ Error.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::delete_stream::component::DeleteStream;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteStream, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !base.streams.iter().any(|stream| stream.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stream \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let referencing: Vec<String> = base.gcps.iter().filter(|gcp| gcp.observations.iter().any(|observation| observation.stream_id == payload.id)).map(|gcp| gcp.id.clone()).collect();
    if !referencing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Stream \"{}\" is still observed by {} ground control point(s); remove those observations first.", payload.id, referencing.len()), referencing);
    }
    protocol::MutationOutcome::new(RemodelingDiff::stream_rows(vec![RemodelingRow::Remove { key: payload.id.clone() }]))
}
//#endregion 🔖️Diff
