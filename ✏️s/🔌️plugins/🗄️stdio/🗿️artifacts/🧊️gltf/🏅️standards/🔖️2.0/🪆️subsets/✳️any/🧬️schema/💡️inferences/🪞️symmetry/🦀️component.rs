//! 🪞 GLTF symmetry indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfSymmetryIndicators {
    pub reflection_symmetry_score: GltfMeasure<f64>,
    pub rotational_symmetry_score: GltfMeasure<f64>,
    pub reflection_symmetries: GltfMeasure<Vec<GltfDirectionScore>>,
    pub rotational_symmetries: GltfMeasure<Vec<GltfDirectionScore>>,
    pub repetition_ratio: GltfMeasure<f64>,
    pub modularity_ratio: GltfMeasure<f64>,
}
