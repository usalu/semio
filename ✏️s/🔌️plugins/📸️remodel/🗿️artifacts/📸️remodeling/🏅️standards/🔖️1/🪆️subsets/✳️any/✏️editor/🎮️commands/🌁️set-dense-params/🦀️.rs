//! ⚙️ ⚙️ Remodeling play app commands command — `set-dense-params`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_dense_params::update_dense_params;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::DenseParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::DenseResolution;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "dense-params")]
pub struct SetDenseParams {
    pub resolution: String,
    pub window_radius_px: u32,
    pub min_view_consistency: u32,
    pub confidence_threshold: f32,
    pub max_points: u32,
}

pub fn handle(payload: &SetDenseParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_dense_params(DenseParams {
        resolution: match payload.resolution.as_str() {
            "low" => DenseResolution::Low,
            "high" => DenseResolution::High,
            _ => DenseResolution::Medium,
        },
        window_radius_px: payload.window_radius_px,
        min_view_consistency: payload.min_view_consistency,
        confidence_threshold: payload.confidence_threshold,
        max_points: payload.max_points,
    })]))
}
