//! 🔗 GLTF adjacency indicators.

#[path = "🧩️connected-components/🦀️.rs"]
pub mod connected_components;
#[path = "🌳️contact-graph-degree/🦀️.rs"]
pub mod contact_graph_degree;
#[path = "🔢️number-of-contacts/🦀️.rs"]
pub mod number_of_contacts;

use super::super::modules::measurement_contracts::*;
use super::super::modules::mesh_topology::Topology;
use super::geometry_core::{GltfGeometryContext, GltfPairGeometry};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAdjacencyIndicators {
    pub number_of_contacts: GltfMeasure<u64>,
    pub contact_graph_degree: GltfMeasure<u64>,
    pub connected_components: GltfMeasure<u64>,
}

pub struct GltfAdjacencyInference;

impl GltfAdjacencyInference {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<bool> {
        number_of_contacts::infer_pair(pair)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn infer_assembly(indicators: &mut GltfAdjacencyIndicators, part_count: usize, contacts: u64, sample_count: usize, topology: Topology) {
        indicators.number_of_contacts = number_of_contacts::from_assembly(part_count, contacts, sample_count, topology);
        indicators.contact_graph_degree = contact_graph_degree::from_assembly(part_count, contacts, sample_count, topology);
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfAdjacencyInference {
    type Output = GltfAdjacencyIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { number_of_contacts: number_of_contacts::infer(context), contact_graph_degree: contact_graph_degree::infer(context), connected_components: connected_components::infer(context) }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            number_of_contacts: number_of_contacts::unavailable_measure(diagnostic_ids),
            contact_graph_degree: contact_graph_degree::unavailable_measure(diagnostic_ids),
            connected_components: connected_components::unavailable_measure(diagnostic_ids),
        }
    }
}
