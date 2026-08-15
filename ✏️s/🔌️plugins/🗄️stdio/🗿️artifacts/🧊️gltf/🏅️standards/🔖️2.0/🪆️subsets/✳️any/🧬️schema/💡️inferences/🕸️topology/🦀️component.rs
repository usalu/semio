//! 🕸 GLTF topology indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfTopologyIndicators {
    pub holes: GltfMeasure<u64>,
    pub handles: GltfMeasure<u64>,
    pub boundary_loops: GltfMeasure<u64>,
    pub euler_characteristic: GltfMeasure<i64>,
    pub genus: GltfMeasure<u64>,
}
