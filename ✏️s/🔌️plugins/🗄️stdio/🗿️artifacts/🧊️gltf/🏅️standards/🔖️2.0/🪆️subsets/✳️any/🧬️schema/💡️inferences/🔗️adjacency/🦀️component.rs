//! 🔗 GLTF adjacency indicators.

use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfAdjacencyIndicators {
    pub number_of_contacts: GltfMeasure<u64>,
    pub contact_graph_degree: GltfMeasure<u64>,
    pub connected_components: GltfMeasure<u64>,
}
