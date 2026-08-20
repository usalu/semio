//! 💡️ contact-graph-degree atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, exact, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfContactGraphDegreeInference;

impl GltfInferenceLeaf for GltfContactGraphDegreeInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.contact-graph-degree.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.contact-graph-degree.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfContactGraphDegreeInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<u64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)).await
}

pub(crate) async fn from_assembly(part_count: usize, contacts: u64, sample_count: usize, topology: Topology) -> GltfMeasure<u64> {
    if part_count <= 1 {
        exact(0, GltfUnit::Unitless, sample_count, Some(topology)).await
    } else {
        estimate(2 * contacts / part_count as u64, GltfUnit::Unitless, sample_count, Some(topology)).await
    }
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<u64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None).await
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.adjacency.contact_graph_degree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.contact-graph-degree.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
