//! 🎯️ 🎯️ Remodeling play app commands command — `place-gcp-observation`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::add_gcp_observation::add_gcp_observation;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GcpObservation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "place-gcp-observation")]
pub struct PlaceGcpObservation {
    pub gcp_id: String,
    pub stream_id: String,
    pub frame_index: u32,
    pub pixel_x: f32,
    pub pixel_y: f32,
}

pub fn handle(payload: &PlaceGcpObservation, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    if !doc.snapshot.gcps.iter().any(|gcp| gcp.id == payload.gcp_id) {
        return Ok(Emit::default());
    }
    let observation = GcpObservation { stream_id: payload.stream_id.clone(), frame_index: payload.frame_index, pixel: [payload.pixel_x, payload.pixel_y] };
    Ok(Emit::mutations(vec![add_gcp_observation(payload.gcp_id.clone(), observation)]))
}
