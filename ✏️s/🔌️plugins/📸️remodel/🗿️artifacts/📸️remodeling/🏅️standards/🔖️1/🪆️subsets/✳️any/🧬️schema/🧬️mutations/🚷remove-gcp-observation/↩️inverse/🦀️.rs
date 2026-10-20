//! ↩️ Inverse for `RemoveGcpObservation` — re-`add-gcp-observation`s the captured BASE observation,
//! which lands at its canonical `(stream_id, frame_index)` position, i.e. exactly the one it was
//! removed from. Missing target/index ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::add_gcp_observation::AddGcpObservation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<AddGcpObservation>>(payload: &crate::standards::v1::subsets::any::schema::mutations::remove_gcp_observation::component::RemoveGcpObservation, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return Vec::new();
    };
    let Some(observation) = gcp.observations.get(payload.observation_index as usize) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::add_gcp_observation::add_gcp_observation(payload.id.clone(), observation.clone())]

    })())
}
//#endregion 🔖️Inverse
