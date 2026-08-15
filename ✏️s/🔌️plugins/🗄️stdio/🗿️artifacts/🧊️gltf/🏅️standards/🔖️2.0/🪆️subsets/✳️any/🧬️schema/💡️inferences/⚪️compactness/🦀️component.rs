//! ⚪️ GLTF compactness indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfCompactnessIndicators {
    pub compactness: GltfMeasure<f64>,
    pub surface_to_volume_ratio: GltfMeasure<f64>,
    pub sphericity: GltfMeasure<f64>,
    pub compactness_index: GltfMeasure<f64>,
    pub hull_fill_ratio: GltfMeasure<f64>,
}
