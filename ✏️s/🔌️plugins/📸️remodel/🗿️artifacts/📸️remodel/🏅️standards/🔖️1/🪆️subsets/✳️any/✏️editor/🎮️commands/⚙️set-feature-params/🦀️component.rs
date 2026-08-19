//! ⚙️ ⚙️ Remodel play app commands command — `set-feature-params`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::update_feature_params;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{FeatureDetector, FeatureParams, RemodelSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "feature-params")]
pub struct SetFeatureParams {
    pub detector: String,
    pub target_count: u32,
    pub octaves: u32,
    pub edge_threshold: f32,
}

pub async fn handle(payload: &SetFeatureParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
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
