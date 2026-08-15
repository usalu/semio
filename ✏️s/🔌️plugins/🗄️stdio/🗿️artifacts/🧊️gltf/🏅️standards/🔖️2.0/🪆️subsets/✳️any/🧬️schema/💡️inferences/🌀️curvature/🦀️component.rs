//! 🌀 GLTF curvature indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfCurvatureIndicators {
    pub mean_curvature: GltfMeasure<GltfStatistics>,
    pub gaussian_curvature: GltfMeasure<GltfStatistics>,
    pub curvature_histogram: GltfMeasure<GltfStatistics>,
    pub sharp_feature_proportion: GltfMeasure<f64>,
}
