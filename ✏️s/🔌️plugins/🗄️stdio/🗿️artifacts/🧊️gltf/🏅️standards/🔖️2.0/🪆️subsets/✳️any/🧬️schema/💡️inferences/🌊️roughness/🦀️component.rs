//! 🌊 GLTF roughness indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfRoughnessIndicators {
    pub deviation_from_ideal: GltfMeasure<GltfStatistics>,
    pub deviation_from_smoothed_geometry: GltfMeasure<GltfStatistics>,
    pub normal_variation: GltfMeasure<GltfStatistics>,
    pub surface_waviness: GltfMeasure<GltfStatistics>,
    pub irregularity: GltfMeasure<f64>,
}
