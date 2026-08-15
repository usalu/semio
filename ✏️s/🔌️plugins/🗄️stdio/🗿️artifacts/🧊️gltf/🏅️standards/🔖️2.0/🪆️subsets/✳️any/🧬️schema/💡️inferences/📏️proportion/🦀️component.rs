//! 📏 GLTF proportion indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfProportionIndicators {
    pub aspect_ratios: GltfMeasure<GltfVec3>,
    pub slenderness: GltfMeasure<f64>,
    pub flatness: GltfMeasure<f64>,
    pub elongation: GltfMeasure<f64>,
}
