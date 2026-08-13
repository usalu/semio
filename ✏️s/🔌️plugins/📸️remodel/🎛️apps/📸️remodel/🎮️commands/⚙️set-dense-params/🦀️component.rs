//! ⚙️ ⚙️ Remodel play app commands command — `set-dense-params`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::{update_dense_params, update_feature_params, update_geo_params, update_ingest_params, update_match_params, update_mesh_params, update_motion_params, update_sfm_params};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{DenseParams, DenseResolution, FeatureDetector, FeatureParams, GeoParams, IngestParams, MatchParams, MatcherKind, MeshParams, MotionParams, RemodelSnapshot, RobustLossKind, SfmParams};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "dense-params")]
pub struct SetDenseParams {
    pub resolution: String,
    pub window_radius_px: u32,
    pub min_view_consistency: u32,
    pub confidence_threshold: f32,
    pub max_points: u32,
}

pub fn handle(payload: &SetDenseParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
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
