//! 🕳 GLTF concavity indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfConcavityIndicators {
    pub convex_hull_gap: GltfMeasure<f64>,
    pub reentrant_area: GltfMeasure<f64>,
    pub reentrant_volume: GltfMeasure<f64>,
    pub concavity_index: GltfMeasure<f64>,
}
