//! ⚖️ GLTF mass-distribution indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMassIndicators {
    pub centroid: GltfMeasure<GltfVec3>,
    pub principal_frame: GltfMeasure<GltfPrincipalFrame>,
    pub principal_axes: GltfMeasure<Vec<GltfDirectionScore>>,
    pub moments_of_inertia: GltfMeasure<GltfVec3>,
    pub inertia_tensor: GltfMeasure<Vec<f64>>,
}
