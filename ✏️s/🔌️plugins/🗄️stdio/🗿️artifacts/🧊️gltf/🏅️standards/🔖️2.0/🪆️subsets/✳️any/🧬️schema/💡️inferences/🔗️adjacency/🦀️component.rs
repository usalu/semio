//! 🔗 GLTF adjacency indicators.

#[path = "connected-components/🦀️component.rs"]
pub mod connected_components;
#[path = "contact-graph-degree/🦀️component.rs"]
pub mod contact_graph_degree;
#[path = "number-of-contacts/🦀️component.rs"]
pub mod number_of_contacts;

use super::super::modules::measurement_contracts::*;
use super::super::modules::mesh_topology::Topology;
use super::geometry_core::{GltfGeometryContext, GltfPairGeometry};
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
    pub(crate) async fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<bool> {
        number_of_contacts::infer_pair(pair).await
    }

    pub(crate) async fn infer_assembly(indicators: &mut GltfAdjacencyIndicators, part_count: usize, contacts: u64, sample_count: usize, topology: Topology) {
        indicators.number_of_contacts = number_of_contacts::from_assembly(part_count, contacts, sample_count, topology).await;
        indicators.contact_graph_degree = contact_graph_degree::from_assembly(part_count, contacts, sample_count, topology).await;
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfAdjacencyInference {
    type Output = GltfAdjacencyIndicators;

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { number_of_contacts: number_of_contacts::infer(context).await, contact_graph_degree: contact_graph_degree::infer(context).await, connected_components: connected_components::infer(context).await }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            number_of_contacts: number_of_contacts::unavailable_measure(diagnostic_ids).await,
            contact_graph_degree: contact_graph_degree::unavailable_measure(diagnostic_ids).await,
            connected_components: connected_components::unavailable_measure(diagnostic_ids).await,
        }
    }
}
