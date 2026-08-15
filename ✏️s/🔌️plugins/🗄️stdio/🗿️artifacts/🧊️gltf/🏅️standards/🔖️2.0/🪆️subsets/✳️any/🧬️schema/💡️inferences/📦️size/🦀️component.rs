//! 📦 GLTF size indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfSizeIndicators {
    pub overall_size: GltfMeasure<f64>,
    pub axis_aligned_bounds: GltfMeasure<GltfBounds3>,
    pub oriented_bounds: GltfMeasure<GltfBounds3>,
    pub bounding_box_dimensions: GltfMeasure<GltfVec3>,
    pub characteristic_length: GltfMeasure<f64>,
    pub footprint_area: GltfMeasure<f64>,
    pub projected_area: GltfMeasure<GltfStatistics>,
}
