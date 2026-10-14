//! ⚙️ ⚙️ Remodeling play app commands command — `set-ingest-params`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_ingest_params::update_ingest_params;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "ingest-params")]
pub struct SetIngestParams {
    pub frame_sample_stride: u32,
    pub max_frames: u32,
    pub downscale_long_edge_px: u32,
    pub min_sharpness: f32,
}

pub fn handle(payload: &SetIngestParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_ingest_params(IngestParams { frame_sample_stride: payload.frame_sample_stride, max_frames: payload.max_frames, downscale_long_edge_px: payload.downscale_long_edge_px, min_sharpness: payload.min_sharpness })]))
}

//#region 🧪️Tests
//#endregion 🧪️Tests
