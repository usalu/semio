//! ⚙️ ⚙️ Remodeling play app commands command — `set-feature-params`.

use crate::artifacts::remodeling::mutations::update_feature_params;
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::{FeatureDetector, FeatureParams, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "feature-params")]
pub struct SetFeatureParams {
    pub detector: String,
    pub target_count: u32,
    pub octaves: u32,
    pub edge_threshold: f32,
}

pub async fn handle(payload: &SetFeatureParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
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
