//! 🧱 GLTF area-volume indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfAreaVolumeIndicators {
    pub surface_area: GltfMeasure<f64>,
    pub total_area: GltfMeasure<f64>,
    pub exposed_area: GltfMeasure<f64>,
    pub contact_area: GltfMeasure<f64>,
    pub volume: GltfMeasure<f64>,
    pub enclosed_volume: GltfMeasure<f64>,
    pub material_volume: GltfMeasure<f64>,
    pub void_volume: GltfMeasure<f64>,
}
