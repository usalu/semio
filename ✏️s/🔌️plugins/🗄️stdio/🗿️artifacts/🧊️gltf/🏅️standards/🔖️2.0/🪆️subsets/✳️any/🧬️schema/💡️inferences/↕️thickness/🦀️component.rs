//! ↕️ GLTF thickness indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfThicknessIndicators {
    pub mean_thickness: GltfMeasure<f64>,
    pub minimum_thickness: GltfMeasure<f64>,
    pub thickness_variability: GltfMeasure<f64>,
    pub thickness_distribution: GltfMeasure<GltfStatistics>,
}
