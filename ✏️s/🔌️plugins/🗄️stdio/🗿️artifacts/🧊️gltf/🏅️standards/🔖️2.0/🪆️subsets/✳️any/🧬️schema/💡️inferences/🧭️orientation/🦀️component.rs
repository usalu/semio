//! 🧭 GLTF orientation indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfOrientationIndicators {
    pub main_axis_direction: GltfMeasure<GltfVec3>,
    pub face_normal_distribution: GltfMeasure<GltfStatistics>,
    pub orientation_consistency: GltfMeasure<f64>,
}
