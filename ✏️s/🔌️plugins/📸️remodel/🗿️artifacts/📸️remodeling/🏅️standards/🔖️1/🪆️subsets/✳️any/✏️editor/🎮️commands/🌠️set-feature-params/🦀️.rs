//! ⚙️ ⚙️ Remodeling play app commands command — `set-feature-params`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_feature_params::update_feature_params;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FeatureDetector;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FeatureParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "feature-params")]
pub struct SetFeatureParams {
    pub detector: String,
    pub target_count: u32,
    pub octaves: u32,
    pub edge_threshold: f32,
}

pub fn handle(payload: &SetFeatureParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_feature_params(FeatureParams {
        detector: match payload.detector.as_str() {
            "akaze" => FeatureDetector::Akaze,
            "harris" => FeatureDetector::Harris,
            _ => FeatureDetector::Orb,
        },
        target_count: payload.target_count,
        octaves: payload.octaves,
        edge_threshold: payload.edge_threshold,
    })]))
}
