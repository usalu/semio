//! 🔗 GLTF adjacency indicators.

use super::geometry::{estimate, exact, unavailable, GltfGeometryContext, GltfPairGeometry, Topology};
use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfAdjacencyIndicators {
    pub number_of_contacts: GltfMeasure<u64>,
    pub contact_graph_degree: GltfMeasure<u64>,
    pub connected_components: GltfMeasure<u64>,
}

pub struct GltfAdjacencyInference;

impl GltfAdjacencyInference {
    pub(crate) fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<bool> {
        estimate(pair.adjacent, GltfUnit::Unitless, pair.sample_count, None)
    }

    pub(crate) fn infer_assembly(indicators: &mut GltfAdjacencyIndicators, part_count: usize, contacts: u64, sample_count: usize, topology: Topology) {
        if part_count <= 1 {
            indicators.number_of_contacts = exact(0, GltfUnit::Unitless, sample_count, Some(topology));
            indicators.contact_graph_degree = exact(0, GltfUnit::Unitless, sample_count, Some(topology));
        } else {
            indicators.number_of_contacts = estimate(contacts, GltfUnit::Unitless, sample_count, Some(topology));
            indicators.contact_graph_degree = estimate(2 * contacts / part_count as u64, GltfUnit::Unitless, sample_count, Some(topology));
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfAdjacencyInference {
    type Output = GltfAdjacencyIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            number_of_contacts: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            contact_graph_degree: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            connected_components: exact(context.topology.components, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        let measure = || unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None);
        Self::Output { number_of_contacts: measure(), contact_graph_degree: measure(), connected_components: measure() }
    }
}
